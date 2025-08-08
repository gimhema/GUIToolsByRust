use chrono::{Datelike, Local, NaiveDate};
use eframe::{egui, App};

struct MyApp {
    selected_date: Option<NaiveDate>,
    view_year: i32,
    view_month: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        let today = Local::now().date_naive();
        Self {
            selected_date: None,
            view_year: today.year(),
            view_month: today.month(),
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📆 Planner");
            ui.separator();

            // 연도 및 월 선택 UI
            ui.horizontal(|ui| {
                if ui.button("◀ Year").clicked() {
                    self.view_year -= 1;
                }
                ui.label(format!("{}Year", self.view_year));
                if ui.button("Year ▶").clicked() {
                    self.view_year += 1;
                }

                ui.add_space(16.0);

                if ui.button("◀ Month").clicked() {
                    if self.view_month == 1 {
                        self.view_month = 12;
                        self.view_year -= 1;
                    } else {
                        self.view_month -= 1;
                    }
                }
                ui.label(format!("{}Month", self.view_month));
                if ui.button("Month ▶").clicked() {
                    if self.view_month == 12 {
                        self.view_month = 1;
                        self.view_year += 1;
                    } else {
                        self.view_month += 1;
                    }
                }
            });

            ui.separator();

            // 달력 그리기
            show_calendar(ui, self.view_year, self.view_month, &mut self.selected_date);

            // 선택된 날짜 표시
            if let Some(sel) = self.selected_date {
                ui.separator();
                ui.label(format!("Selected : {}", sel));
            }
        });
    }
}

fn show_calendar(ui: &mut egui::Ui, year: i32, month: u32, selected: &mut Option<NaiveDate>) {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let days_in_month = match month {
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };

    let start_weekday = first_day.weekday().number_from_monday(); // 1 = Monday

    // 요일 헤더
    ui.horizontal(|ui| {
        for day in ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"] {
            ui.add_sized([32.0, 20.0], egui::Label::new(day));
        }
    });

    let mut day = 1;
    for _ in 0..6 {
        let mut finished = false;

        ui.horizontal(|ui| {
            for i in 1..=7 {
                if day == 1 && i < start_weekday {
                    ui.add_sized([32.0, 32.0], egui::Label::new(""));
                } else if day <= days_in_month {
                    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
                    if ui
                        .add_sized([32.0, 32.0], egui::Button::new(format!("{:2}", day)))
                        .clicked()
                    {
                        *selected = Some(date);
                    }
                    day += 1;
                } else {
                    finished = true;
                    ui.add_sized([32.0, 32.0], egui::Label::new(""));
                }
            }
        });

        if finished || day > days_in_month {
            break;
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "📅 Calendar App",
        native_options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
