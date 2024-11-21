use projcore::data_types::{Data, Meta, PubType};
use tokio::runtime::{Builder, Runtime};

use crate::library::{self, Library};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    #[serde(skip)]
    rt: Runtime,
    app_state: AppState,
    selected_category: SelectedCategory,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum AppState {
    Selecting,
    Selected(Meta),
    Reading(Meta, Data)
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum SelectedCategory {
    Books,
    Papers
}

impl Default for App {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            rt: Builder::new_multi_thread().enable_all().build().unwrap(),
            #[cfg(target_arch = "wasm32")]
            rt: Builder::new_current_thread().enable_all().build().unwrap(),
            app_state: AppState::Selecting,
            selected_category: SelectedCategory::Books,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        /*egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
        });*/

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::widgets::global_theme_preference_buttons(ui);
        });

        match self.app_state.clone() {
            AppState::Selecting => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.columns(2, |columns| {
                        columns[0].vertical_centered(|ui| {
                            ui.selectable_value(&mut self.selected_category, SelectedCategory::Books, "Книги");
                        });
                        columns[1].vertical_centered(|ui| {
                            ui.selectable_value(&mut self.selected_category, SelectedCategory::Papers, "Бумаги");
                        });
                    });
                    ui.vertical_centered(|ui| {
                        ui.heading("Библиотеки");
                    });
        
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.group(|ui| {
                                if ui.button("Бессоница").clicked() {
                                    self.app_state = AppState::Selected(self.rt.block_on(library::KomarovLib::new().data_info(4547)).unwrap());
                                }
                            })
                        })
                    });
        
                    if ui.button("Increment").clicked() {
                        /*self.rt.spawn(async {
                            let test = library::KomarovLib::new().data_info(4547).await.unwrap();
                            log::info!(
                                "{} {} {} {:?}",
                                test.date,
                                test.title,
                                test.publisher,
                                test.authors
                            )
                        });*/
                    }
        
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        powered_by_egui_and_eframe(ui);
                        egui::warn_if_debug_build(ui);
                    });
                });
            }
            AppState::Selected(meta) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if ui.button("назаз").clicked() {
                        self.app_state = AppState::Selecting;
                        return;
                    }
                    ui.heading(&meta.title);
                    ui.label(&format!("Описание: {}", meta.desc));
                    ui.label(&format!("Дата произведения: {}", meta.date));
                    ui.label(&format!("Язык: {}", String::from_utf8(meta.lang.to_vec()).unwrap()));
                    ui.label(&format!("ЫДЫДЫ: {}", meta.publisher));
                    ui.label(&format!("Авторы: {:?}", meta.authors));
                    if ui.button("Читать").clicked() {
                        self.app_state = AppState::Reading(meta, self.rt.block_on(library::KomarovLib::new().get_data(4547)).unwrap());
                    }
                });
            }
            AppState::Reading(meta, data) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if ui.button("назаз").clicked() {
                        self.app_state = AppState::Selected(meta);
                        return;
                    }
                    match meta.pub_type {
                        PubType::Book => {
                            match data {
                                Data::TextOnly(text) => {
                                    for paragraph in text {
                                        ui.label(paragraph);
                                    }
                                }
                                _ => ()
                            }
                        },
                        PubType::Poem => {
                            match data {
                                Data::TextOnly(text) => {
                                    ui.vertical_centered(|ui| {
                                        for paragraph in text {
                                            ui.label(paragraph);
                                            ui.label("");
                                        }
                                    });
                                }
                                _ => ()
                            }
                        }
                    }
                });
            }
        }
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
