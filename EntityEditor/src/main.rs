mod entity_manager;

use eframe::{egui, App, CreationContext};
use egui::{FontData, FontDefinitions, FontFamily, ScrollArea, RichText};

use entity_manager::schema::{TableSchema, DataType};
use entity_manager::dyn_entity::DynRow;
use entity_manager::app_state::DataSets;

// ===== 동적 폼: 라벨/컨트롤 2열 그리드 =====
fn ui_entity_form(ui: &mut egui::Ui, title: &str, schema: &TableSchema, row: &mut DynRow) {
    use egui::Grid;
    ui.group(|ui| {
        ui.label(RichText::new(title).heading());
        ui.add_space(4.0);
        Grid::new(format!("grid_{}", title))
            .num_columns(2)
            .spacing([12.0, 6.0])
            .striped(true)
            .show(ui, |ui| {
                for col in &schema.columns {
                    let header = &col.label; // CSV 헤더 그대로(표시/키)
                    ui.label(header);
                    match col.dtype {
                        DataType::Int => {
                            let mut v: i64 = row.get(header).unwrap_or("0").parse().unwrap_or(0);
                            let resp = ui.add(egui::DragValue::new(&mut v).speed(1));
                            if resp.changed() {
                                row.set(header, v.to_string());
                            }
                        }
                        DataType::Float => {
                            let mut v: f64 = row.get(header).unwrap_or("0").parse().unwrap_or(0.0);
                            let resp = ui.add(egui::DragValue::new(&mut v).speed(0.1));
                            if resp.changed() {
                                row.set(header, format!("{}", v));
                            }
                        }
                        DataType::Text => {
                            let mut v = row.get(header).unwrap_or("").to_string();
                            if ui.text_edit_singleline(&mut v).changed() {
                                row.set(header, v);
                            }
                        }
                    }
                    ui.end_row();
                }
            });
    });
}

// ===== 상태 키 컬럼 모드 =====
#[derive(Debug, Clone)]
enum StatusKeyMode {
    Auto,               // 자동 추론(권장)
    Unique,             // "Unique"
    CharacterUnique,    // "CharacterUnique"
    Custom(String),     // 임의 헤더명
}
impl StatusKeyMode {
    fn as_hint(&self) -> &str {
        match self {
            StatusKeyMode::Auto => "CharacterUnique", // 기본 힌트(없으면 첫 컬럼 사용)
            StatusKeyMode::Unique => "Unique",
            StatusKeyMode::CharacterUnique => "CharacterUnique",
            StatusKeyMode::Custom(s) => s.as_str(),
        }
    }
    fn label(&self) -> String {
        match self {
            StatusKeyMode::Auto => "Auto (alias-like)".to_string(),
            StatusKeyMode::Unique => "Unique".to_string(),
            StatusKeyMode::CharacterUnique => "CharacterUnique".to_string(),
            StatusKeyMode::Custom(s) => format!("Custom: {}", s),
        }
    }
}

// ===== 앱 상태(동적 스키마 기반) =====
struct EditorApp {
    // 파일 경로
    info_path: String,
    status_path: String,
    attack_path: String,
// New Data file GuideLine Step 8:
    skill_path: String,

    // 상태 키 컬럼 지정
    status_key_mode: StatusKeyMode,
    custom_key_input: String,

    // 동적 데이터셋
    ds: Option<DataSets>,

    // 선택된 키(문자열 키)
    selected_key: Option<String>,

    // 메시지
    last_message: String,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            // 프로젝트 경로 구조에 맞게 조정하세요
            info_path: "src/data/character_info.csv".to_string(),
            status_path: "src/data/character_status_info.csv".to_string(),
            attack_path: "src/data/character_attack_info.txt".to_string(),
            // New Data file GuideLine Step 9:
            skill_path: "src/data/character_skill_info.txt".to_string(),

            status_key_mode: StatusKeyMode::Auto,
            custom_key_input: String::new(),

            ds: None,
            selected_key: None,

            last_message: String::new(),
        }
    }
}

impl EditorApp {
    fn new(cc: &CreationContext) -> Self {
        // 한글 폰트
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "my_korean_font".to_string(),
            FontData::from_static(include_bytes!("fonts/korea_font.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_korean_font".to_string());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "my_korean_font".to_string());
        cc.egui_ctx.set_fonts(fonts);

        Self::default()
    }

    fn gather_sorted_unique_keys(ds: &DataSets) -> Vec<String> {
        use std::collections::BTreeSet;

        // 1) 중복 제거 (빈 키는 제외)
        let mut uniq: BTreeSet<String> = BTreeSet::new();
        for k in 
        ds.info.keys()
        .chain(ds.status.keys())
        .chain(ds.attack.keys())
        .chain(ds.skill.keys()) // New Data file GuideLine Step 15:
         {
            if !k.is_empty() {
                uniq.insert(k.clone());
            }
        }

        // 2) 정렬: 숫자 가능하면 숫자로, 아니면 문자열로
        let mut keys: Vec<String> = uniq.into_iter().collect();
        keys.sort_by(|a, b| {
            let pa = a.parse::<u64>();
            let pb = b.parse::<u64>();
            match (pa, pb) {
                (Ok(na), Ok(nb)) => na.cmp(&nb),
                _ => a.cmp(b),
            }
        });
        keys
    }

    fn try_load(&mut self) {
        // 동적 로드: DataSets::load
        let hint = self.status_key_mode.as_hint().to_string();
        match DataSets::load(
            &self.info_path,
            &self.status_path,
            &self.attack_path,
            // New Data file GuideLine Step 10:            
            &self.skill_path,
            &hint, // status key 힌트
        ) {
            Ok(ds) => {
                // 로드 성공
                // 기본 선택 키: info의 첫 번째 키 or status/attack 중 하나
                let first_key = ds
                    .info
                    .keys()
                    .next()
                    .cloned()
                    .or_else(|| ds.status.keys().next().cloned())
                    .or_else(|| ds.attack.keys().next().cloned())
                    
                    // New Data file GuideLine Step 17:    
                    .or_else(|| ds.skill.keys().next().cloned());

                self.selected_key = first_key;
                self.ds = Some(ds);
                self.last_message = "✅ 데이터 로드 성공".into();
            }
            Err(e) => {
                self.last_message = format!("❌ 데이터 로드 실패: {e}");
            }
        }
    }

    fn try_save(&mut self) {
        if let Some(ds) = &self.ds {
            let res = ds.save_all(
                &self.info_path, 
                &self.status_path, 
                &self.attack_path,
                // New Data file GuideLine Step 11:
                &self.skill_path
                );
            match res {
                Ok(_) => self.last_message = "💾 저장 완료".into(),
                Err(e) => self.last_message = format!("❌ 저장 실패: {e}"),
            }
        } else {
            self.last_message = "⚠️ 저장할 데이터가 없습니다. 먼저 로드하세요.".into();
        }
    }

    fn ui_left_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("📁 데이터 파일");
        ui.label("character_info.csv");
        ui.text_edit_singleline(&mut self.info_path);
        ui.add_space(4.0);

        ui.label("character_status_info.csv");
        ui.text_edit_singleline(&mut self.status_path);
        ui.add_space(4.0);

        ui.label("character_attack_info.txt");
        ui.text_edit_singleline(&mut self.attack_path);
        ui.add_space(8.0);

        // New Data file GuideLine Step 16:
        ui.label("character_skill_info.txt");
        ui.text_edit_singleline(&mut self.skill_path);
        ui.add_space(8.0);

        ui.separator();
        ui.heading("🔑 상태 키컬럼");
        egui::ComboBox::from_id_source("status_key_mode")
            .selected_text(self.status_key_mode.label())
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(matches!(self.status_key_mode, StatusKeyMode::Auto), "Auto")
                    .clicked()
                {
                    self.status_key_mode = StatusKeyMode::Auto;
                }
                if ui
                    .selectable_label(
                        matches!(self.status_key_mode, StatusKeyMode::Unique),
                        "Unique",
                    )
                    .clicked()
                {
                    self.status_key_mode = StatusKeyMode::Unique;
                }
                if ui
                    .selectable_label(
                        matches!(self.status_key_mode, StatusKeyMode::CharacterUnique),
                        "CharacterUnique",
                    )
                    .clicked()
                {
                    self.status_key_mode = StatusKeyMode::CharacterUnique;
                }
                if ui
                    .selectable_label(
                        matches!(self.status_key_mode, StatusKeyMode::Custom(_)),
                        "Custom",
                    )
                    .clicked()
                {
                    let current = match &self.status_key_mode {
                        StatusKeyMode::Custom(s) => s.clone(),
                        _ => self.custom_key_input.clone(),
                    };
                    self.status_key_mode = StatusKeyMode::Custom(current);
                }
            });

        if let StatusKeyMode::Custom(_) = self.status_key_mode {
            ui.horizontal(|ui| {
                ui.label("헤더명:");
                if ui.text_edit_singleline(&mut self.custom_key_input).lost_focus() {
                    self.status_key_mode = StatusKeyMode::Custom(self.custom_key_input.clone());
                }
            });
        }

        ui.add_space(8.0);
        if ui.button("📥 로드").clicked() {
            self.try_load();
        }
        if ui.button("💾 저장").clicked() {
            self.try_save();
        }

        ui.add_space(8.0);
        if !self.last_message.is_empty() {
            ui.label(self.last_message.clone());
        }

        ui.separator();
        ui.heading("📦 엔티티 목록");

        // 좌측 리스트: info/status/attack 모든 키를 합쳐 표시
        egui::ScrollArea::vertical()
        .max_height(320.0)
        .show(ui, |ui| {
            if let Some(ds) = &self.ds {
                // 🔧 중복 제거된 키 목록
                let keys = Self::gather_sorted_unique_keys(ds);

                // 선택 유지(선택 키가 더 이상 존재하지 않으면 해제)
                if let Some(sel) = self.selected_key.clone() {
                    if !keys.iter().any(|k| k == &sel) {
                        self.selected_key = None;
                    }
                }

                for k in keys {
                    let selected = self.selected_key.as_ref().map(|s| s == &k).unwrap_or(false);
                    if ui.selectable_label(selected, format!("Key = {}", k)).clicked() {
                        self.selected_key = Some(k);
                    }
                }
            } else {
                ui.label("먼저 로드하세요.");
            }
        });
    }

    // ===== 우측 상세뷰(동적) =====
    fn ui_entity_detail(&mut self, ui: &mut egui::Ui) {
        if self.ds.is_none() {
            ui.heading("📝 Main View");
            ui.label("좌측에서 파일 경로 설정 후 '로드'를 클릭하세요.");
            return;
        }
        let ds = self.ds.as_mut().unwrap();

        if let Some(selected_key) = self.selected_key.clone() {
            ui.heading(format!("🔧 엔티티 편집: {}", &selected_key));
            ui.separator();

            // 편집 버퍼(복사본) 생성
            let mut info = ds.info.get(&selected_key).cloned();
            let mut status = ds.status.get(&selected_key).cloned();
            let mut attack = ds.attack.get(&selected_key).cloned();
            // New Data file GuideLine Step 12:
            let mut skill = ds.skill.get(&selected_key).cloned();

            ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // 블록을 쪼개서 빌림 충돌(여러 &mut 동시 대출) 피하기
                {
                    if let Some(r) = ds.info.get_mut(&selected_key) {
                        ui_entity_form(ui, "Info", &ds.info_schema, r);
                        ui.add_space(8.0);
                    }
                }
                {
                    if let Some(r) = ds.status.get_mut(&selected_key) {
                        ui_entity_form(ui, "Status", &ds.status_schema, r);
                        ui.add_space(8.0);
                    }
                }
                {
                    if let Some(r) = ds.attack.get_mut(&selected_key) {
                        ui_entity_form(ui, "Attack", &ds.attack_schema, r);
                    }
                }
                // New Data file GuideLine Step 13:
                {
                    if let Some(r) = ds.skill.get_mut(&selected_key) {
                        ui.add_space(8.0);
                        ui_entity_form(ui, "Skill", &ds.skill_schema, r);
                    }
                }
            });

            ui.add_space(8.0);
            if ui.button("✅ 변경사항 적용(메모리)").clicked() {
                // 편집 버퍼를 실제 ds 맵에 반영
                if let Some(r) = info {
                    ds.info.insert(selected_key.clone(), r);
                }
                if let Some(r) = status {
                    ds.status.insert(selected_key.clone(), r);
                }
                if let Some(r) = attack {
                    ds.attack.insert(selected_key.clone(), r);
                }
                // New Data file GuideLine Step 14:
                if let Some(r) = skill {
                    ds.skill.insert(selected_key.clone(), r);
                }
                self.last_message = "🟢 메모리에 적용됨 (저장은 따로)".to_string();
            }
        } else {
            ui.heading("📝 Main View");
            ui.label("좌측에서 엔티티를 선택하세요.");
        }
    }
}

impl App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(300.0)
            .show(ctx, |ui| self.ui_left_panel(ui));

        egui::CentralPanel::default().show(ctx, |ui| self.ui_entity_detail(ui));
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Entity Editor (Dynamic Schema)",
        options,
        Box::new(|cc: &CreationContext| Box::new(EditorApp::new(cc))),
    )
}
