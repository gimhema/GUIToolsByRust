mod entity_manager;

use entity_manager::entity::*;
use entity_manager::handler::*;
use entity_manager::raw_data::*;

use eframe::{egui, App, CreationContext};
use egui::{FontData, FontDefinitions, FontFamily};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Entity Editor",
        options,
        Box::new(|cc: &CreationContext| Box::new(EditorApp::new(cc))),
    )
}

// 간단한 ToDo 아이템
struct TodoItem {
    text: String,
    done: bool,
}

// 속성 편집을 위한 enum
#[derive(Debug, Clone)]
enum ItemData {
    Entity { name: String, is_active: bool },
    Handler { id: u32, enabled: bool },
    RawData { description: String },
}

// 메인 앱 구조체
struct EditorApp {
    input: String,
    todos: Vec<TodoItem>,
    selected_item: Option<ItemData>,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            todos: vec![],
            selected_item: None,
        }
    }
}

impl EditorApp {
    fn new(cc: &CreationContext) -> Self {
        // 한글 폰트 설정
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "my_korean_font".to_string(),
            FontData::from_static(include_bytes!("../fonts/xxx.ttf")),
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
}

impl App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 좌측 트리 탐색기
        egui::SidePanel::left("navigation_panel").show(ctx, |ui| {
            ui.heading("📁 모듈 트리");

            // Entity 트리
            ui.collapsing("📦 entity", |ui| {
                for name in ["EntityA", "EntityB"] {
                    let is_selected = matches!(
                        self.selected_item,
                        Some(ItemData::Entity { name: ref n, .. }) if n == name
                    );
                    if ui.selectable_label(is_selected, name).clicked() {
                        self.selected_item = Some(ItemData::Entity {
                            name: name.to_string(),
                            is_active: true,
                        });
                    }
                }
            });

            // Handler 트리
            ui.collapsing("⚙ handler", |ui| {
                for (index, name) in ["HandlerX", "HandlerY"].iter().enumerate() {
                    let is_selected = matches!(
                        self.selected_item,
                        Some(ItemData::Handler { id, .. }) if format!("Handler{}", id) == *name
                    );
                    if ui.selectable_label(is_selected, *name).clicked() {
                        self.selected_item = Some(ItemData::Handler {
                            id: index as u32 + 1,
                            enabled: true,
                        });
                    }
                }
            });

            // RawData 트리
            ui.collapsing("🗄 raw_data", |ui| {
                for name in ["Data1", "Data2"] {
                    let is_selected = matches!(
                        self.selected_item,
                        Some(ItemData::RawData { description: ref d }) if d == name
                    );
                    if ui.selectable_label(is_selected, name).clicked() {
                        self.selected_item = Some(ItemData::RawData {
                            description: name.to_string(),
                        });
                    }
                }
            });
        });

        // 중앙 정보/편집 영역
        egui::CentralPanel::default().show(ctx, |ui| {
            match &mut self.selected_item {
                Some(ItemData::Entity { name, is_active }) => {
                    ui.heading("🔧 Entity 속성 편집");
                    ui.horizontal(|ui| {
                        ui.label("이름:");
                        ui.text_edit_singleline(name);
                    });
                    ui.checkbox(is_active, "활성화 여부");
                }

                Some(ItemData::Handler { id, enabled }) => {
                    ui.heading("🔧 Handler 속성 편집");
                    ui.horizontal(|ui| {
                        ui.label("ID:");
                        ui.add(egui::DragValue::new(id));
                    });
                    ui.checkbox(enabled, "사용 중");
                }

                Some(ItemData::RawData { description }) => {
                    ui.heading("📄 RawData 설명 편집");
                    ui.label("설명:");
                    ui.text_edit_multiline(description);
                }

                None => {
                    ui.heading("📝 기본 리소스 에디터");

                    ui.horizontal(|ui| {
                        let input = ui.text_edit_singleline(&mut self.input);
                        if ui.button("➕ 추가").clicked()
                            || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                        {
                            if !self.input.trim().is_empty() {
                                self.todos.push(TodoItem {
                                    text: self.input.trim().to_string(),
                                    done: false,
                                });
                                self.input.clear();
                            }
                        }
                    });

                    ui.separator();

                    for todo in &mut self.todos {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut todo.done, "");
                            if todo.done {
                                ui.label(egui::RichText::new(&todo.text).strikethrough());
                            } else {
                                ui.label(&todo.text);
                            }
                        });
                    }
                }
            }
        });
    }
}
