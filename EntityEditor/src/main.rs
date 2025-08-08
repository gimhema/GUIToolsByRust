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

struct TodoItem {
    text: String,
    done: bool,
}

#[derive(Debug, Clone)]
enum ItemData {
    Entity { name: String, is_active: bool },
    Handler { id: u32, enabled: bool },
    RawData { description: String },
}

struct EditorApp {
    input: String,
    todos: Vec<TodoItem>,
    selected_item: Option<ItemData>,

    show_item_list_view: bool,
    item_list: Vec<String>,
    selected_list_index: usize,
    selected_item_result: Option<String>,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            todos: vec![],
            selected_item: None,
            show_item_list_view: false,
            item_list: vec!["사과".to_string(), "바나나".to_string(), "오렌지".to_string()],
            selected_list_index: 0,
            selected_item_result: None,
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
        egui::SidePanel::left("navigation_panel").show(ctx, |ui| {
            ui.heading("📁 모듈 트리");

            if ui.button("🏠 메인 화면으로").clicked() {
                self.selected_item = None;
            }

            ui.separator();

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
                    ui.heading("📝 Main View");

                    if ui.button("항목 선택하기").clicked() {
                        self.show_item_list_view = true;
                    }

                    if let Some(result) = &self.selected_item_result {
                        ui.label(format!("선택한 항목: {}", result));
                    }
                }
            }

            // 팝업 창 띄우기
            if self.show_item_list_view {
                egui::Window::new("ItemListView")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("항목을 선택하세요:");

                        egui::ComboBox::from_id_source("item_select_box")
                            .selected_text(&self.item_list[self.selected_list_index])
                            .show_ui(ui, |ui| {
                                for (i, item) in self.item_list.iter().enumerate() {
                                    ui.selectable_value(&mut self.selected_list_index, i, item);
                                }
                            });

                        if ui.button("선택 완료").clicked() {
                            self.selected_item_result =
                                Some(self.item_list[self.selected_list_index].clone());
                            self.show_item_list_view = false;
                        }

                        if ui.button("닫기").clicked() {
                            self.show_item_list_view = false;
                        }
                    });
            }
        });
    }
}
