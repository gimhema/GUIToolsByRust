mod entity_manager;

use entity_manager::entity::*;
use entity_manager::handler::*;
use entity_manager::parser::*;

use eframe::{egui, App, CreationContext};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Entity Editor",
        options,
        Box::new(|_cc: &CreationContext| Box::new(TodoApp::default())),
    )
}

struct TodoItem {
    text: String,
    done: bool,
}

#[derive(Default)]
struct TodoApp {
    input: String,
    todos: Vec<TodoItem>,
}

impl App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📝 Todo List");

            ui.horizontal(|ui| {
                let input = ui.text_edit_singleline(&mut self.input);
                if ui.button("➕ Add").clicked() || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
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
        });
    }
}
