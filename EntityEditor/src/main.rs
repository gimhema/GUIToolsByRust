mod entity_manager;

use entity_manager::entity::*;
use entity_manager::handler::*;
use entity_manager::raw_data::*;

use eframe::{egui, App, CreationContext};
use egui::{FontData, FontDefinitions, FontFamily};

use std::fs::File;
use csv::ReaderBuilder;

#[derive(Debug, Clone)]
enum StatusKeyMode {
    Auto,               // alias 기반 자동 (Unique/CharacterUnique 둘 다 허용)
    Unique,             // 수동 지정: "Unique"
    CharacterUnique,    // 수동 지정: "CharacterUnique"
    Custom(String),     // 임의 헤더명
}

impl StatusKeyMode {
    fn as_override(&self) -> Option<String> {
        match self {
            StatusKeyMode::Auto => None,
            StatusKeyMode::Unique => Some("Unique".to_string()),
            StatusKeyMode::CharacterUnique => Some("CharacterUnique".to_string()),
            StatusKeyMode::Custom(s) => Some(s.clone()),
        }
    }
    fn label(&self) -> String {
        match self {
            StatusKeyMode::Auto => "Auto (alias)".to_string(),
            StatusKeyMode::Unique => "Unique".to_string(),
            StatusKeyMode::CharacterUnique => "CharacterUnique".to_string(),
            StatusKeyMode::Custom(s) => format!("Custom: {}", s),
        }
    }
}

struct EditorApp {
    // 파일 경로
    info_path: String,
    status_path: String,
    attack_path: String,

    // 상태 키 컬럼 지정
    status_key_mode: StatusKeyMode,
    custom_key_input: String,

    // 데이터 컨테이너
    container: CharacterEntityContainer,

    // 엔티티 ID 목록(좌측 리스트 표시용)
    entity_ids: Vec<u32>,
    selected_entity: Option<u32>,

    // 메시지/알림
    last_message: String,
}

impl Default for EditorApp {
    fn default() -> Self {
        let mut container = CharacterEntityContainer::new();
        // 초기 예시 경로
        let info_path = "character_info.csv".to_string();
        let status_path = "character_status_info.csv".to_string();
        let attack_path = "character_attack_info.txt".to_string();
        container.set_paths(info_path.clone(), status_path.clone(), attack_path.clone());

        Self {
            info_path,
            status_path,
            attack_path,
            status_key_mode: StatusKeyMode::Auto,
            custom_key_input: String::new(),
            container,
            entity_ids: Vec::new(),
            selected_entity: None,
            last_message: String::new(),
        }
    }
}

impl EditorApp {
    fn new(cc: &CreationContext) -> Self {
        // 한글 폰트 설정
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

    fn set_paths_into_container(&mut self) {
        self.container
            .set_paths(self.info_path.clone(), self.status_path.clone(), self.attack_path.clone());
    }

    fn read_ids_from_info_csv(&self) -> Vec<u32> {
    let mut ids = Vec::new();

    if let Ok(file) = File::open(&self.info_path) {
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        for rec in rdr.deserialize::<RawDataCharacterInfo>() {
            if let Ok(record) = rec {
                ids.push(record.unique);
            }
        }
    }

    ids.sort_unstable();
    ids.dedup();
    ids
}

    fn try_load(&mut self) {
        self.set_paths_into_container();

        // 상태 키 컬럼 수동 지정 반영
        if let Some(key) = self.status_key_mode.as_override() {
            self.container.set_status_key_column_override(key);
        } else {
            // Auto 모드면 None 유지
        }

        match self.container.load_data() {
            Ok(_) => {
                self.entity_ids = self.read_ids_from_info_csv();
                self.selected_entity = self.entity_ids.first().cloned();
                self.last_message = "✅ 데이터 로드 성공".to_string();
            }
            Err(e) => {
                self.last_message = format!("❌ 데이터 로드 실패: {}", e);
            }
        }
    }

    fn try_save(&mut self) {
        self.set_paths_into_container();
        match self.container.save_data() {
            Ok(_) => self.last_message = "💾 저장 완료".to_string(),
            Err(e) => self.last_message = format!("❌ 저장 실패: {}", e),
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

        ui.separator();
        ui.heading("🔑 상태 키컬럼");
        egui::ComboBox::from_id_source("status_key_mode")
            .selected_text(self.status_key_mode.label())
            .show_ui(ui, |ui| {
                if ui.selectable_label(matches!(self.status_key_mode, StatusKeyMode::Auto), "Auto (alias)").clicked() {
                    self.status_key_mode = StatusKeyMode::Auto;
                }
                if ui.selectable_label(matches!(self.status_key_mode, StatusKeyMode::Unique), "Unique").clicked() {
                    self.status_key_mode = StatusKeyMode::Unique;
                }
                if ui.selectable_label(matches!(self.status_key_mode, StatusKeyMode::CharacterUnique), "CharacterUnique").clicked() {
                    self.status_key_mode = StatusKeyMode::CharacterUnique;
                }
                if ui.selectable_label(matches!(self.status_key_mode, StatusKeyMode::Custom(_)), "Custom").clicked() {
                    // 기존 입력 보존
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

        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                for id in &self.entity_ids {
                    let selected = self.selected_entity == Some(*id);
                    if ui.selectable_label(selected, format!("CharacterUnique = {}", id)).clicked() {
                        self.selected_entity = Some(*id);
                    }
                }
            });
    }

    fn ui_entity_detail(&mut self, ui: &mut egui::Ui) {
        if let Some(id) = self.selected_entity {
            if let Some(ent) = self.container.get_entity(id).cloned() {
                ui.heading(format!("🔧 엔티티 편집: {}", id));
                ui.separator();

                // 편집 버퍼
                let mut name = ent.character_info.name.clone();
                let mut health = ent.character_status_info.health as i64;
                let mut mana = ent.character_status_info.mana as i64;
                let mut stamina = ent.character_status_info.stamina as i64;
                let mut atk = ent.character_attack_info.attack_power as i64;
                let mut def = ent.character_attack_info.defense_power as i64;

                // 실제 CSV 헤더명 라벨
                let labels = &self.container.labels;

                ui.group(|ui| {
                    ui.label("📄 Character Info");
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", labels.info_name));
                        ui.text_edit_singleline(&mut name);
                    });
                });

                ui.add_space(6.0);

                ui.group(|ui| {
                    ui.label("❤️ Status");
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", labels.status_health));
                        ui.add(egui::DragValue::new(&mut health).clamp_range(0..=1_000_000));
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", labels.status_mana));
                        ui.add(egui::DragValue::new(&mut mana).clamp_range(0..=1_000_000));
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", labels.status_stamina));
                        ui.add(egui::DragValue::new(&mut stamina).clamp_range(0..=1_000_000));
                    });
                });

                ui.add_space(6.0);

                ui.group(|ui| {
                    ui.label("⚔️ Attack/Defense");
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", labels.attack_attack_power));
                        ui.add(egui::DragValue::new(&mut atk).clamp_range(0..=1_000_000));
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", labels.attack_defense_power));
                        ui.add(egui::DragValue::new(&mut def).clamp_range(0..=1_000_000));
                    });
                });

                ui.add_space(8.0);
                if ui.button("✅ 변경사항 적용(메모리)").clicked() {
                    if let Some(ent_mut) = self.container.get_entity_mut(id) {
                        // info
                        ent_mut.character_info.name = name;
                        // status
                        ent_mut.character_status_info.health = health as u32;
                        ent_mut.character_status_info.mana = mana as u32;
                        ent_mut.character_status_info.stamina = stamina as u32;
                        // attack
                        ent_mut.character_attack_info.attack_power = atk as u32;
                        ent_mut.character_attack_info.defense_power = def as u32;

                        self.last_message = "🟢 메모리에 적용됨 (저장은 따로)".to_string();
                    }
                }
            } else {
                ui.label("선택된 엔티티를 찾을 수 없습니다.");
            }
        } else {
            ui.heading("📝 Main View");
            ui.label("좌측에서 엔티티를 선택하거나, 파일 경로를 설정 후 '로드'를 클릭하세요.");
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
        "Entity Editor",
        options,
        Box::new(|cc: &CreationContext| Box::new(EditorApp::new(cc))),
    )
}
