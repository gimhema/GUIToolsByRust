mod entity_manager;

use entity_manager::entity::*;
use entity_manager::handler::*;
use entity_manager::raw_data::*;

use eframe::{egui, App, CreationContext};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Entity Editor",
        options,
        Box::new(|_cc: &CreationContext| Box::new(EditorApp::default())),
    )
}

struct TodoItem {
    text: String,
    done: bool,
}

#[derive(Default)]
struct EditorApp {
    input: String,
    todos: Vec<TodoItem>,

    selected_item: Option<String>,
}

impl App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 좌측 트리 탐색기
        egui::SidePanel::left("navigation_panel").show(ctx, |ui| {
            ui.heading("📁 Modules");

            ui.collapsing("entity", |ui| {
                for name in ["EntityA", "EntityB"] {
                    if ui.selectable_label(
                        self.selected_item.as_deref() == Some(name),
                        name,
                    ).clicked() {
                        self.selected_item = Some(name.to_string());
                    }
                }
            });

            ui.collapsing("handler", |ui| {
                for name in ["HandlerX", "HandlerY"] {
                    if ui.selectable_label(
                        self.selected_item.as_deref() == Some(name),
                        name,
                    ).clicked() {
                        self.selected_item = Some(name.to_string());
                    }
                }
            });

            ui.collapsing("raw_data", |ui| {
                for name in ["Data1", "Data2"] {
                    if ui.selectable_label(
                        self.selected_item.as_deref() == Some(name),
                        name,
                    ).clicked() {
                        self.selected_item = Some(name.to_string());
                    }
                }
            });
        });

        // 우측 중앙 패널
        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.selected_item {
                Some(name) => {
                    ui.heading(format!("🔍 Inspecting: {name}"));
                    ui.separator();
                    ui.label("이곳에 선택한 항목에 대한 정보를 출력하세요.");
                    ui.monospace(format!("선택된 이름: {name}"));
                    // 추후 여기에 실제 구조체의 속성이나 필드 정보 출력 가능
                }
                None => {
                    ui.heading("📝 Resource Editor");

                    ui.horizontal(|ui| {
                        let input = ui.text_edit_singleline(&mut self.input);
                        if ui.button("➕ Add").clicked()
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
