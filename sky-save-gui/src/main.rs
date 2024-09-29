mod tabs;

use crate::tabs::{
    ActivePokemonTab, GeneralTab, GuiTabState, StoredPokemonTab, TabPane, TabsBehavior,
};
use eframe::egui::widget_text::RichText;
use eframe::egui::{
    containers, Button, CentralPanel, Context, FontFamily, FontId, Key, Margin, TopBottomPanel,
    ViewportCommand, Visuals,
};
use eframe::{egui, App, CreationContext, Frame};
use egui::IconData;
use egui_tiles::{Tiles, Tree};
use sky_save::SkySave;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub const ICON_BYTES: &[u8] = include_bytes!("../res/icon.rgba").as_slice();

#[derive(Debug)]
enum Message {
    SaveFileOpened { filepath: PathBuf },
    SaveFileSavedAs { filepath: PathBuf },
}

#[derive(Debug, Default)]
struct State {
    pub filepath: Option<PathBuf>,
    pub save: Option<SkySave>,
}

#[derive(Debug)]
struct SkySaveGui {
    pub state: State,
    pub message_ch: (Sender<Message>, Receiver<Message>),
    pub tabs: Option<Tree<TabPane>>,
}

impl SkySaveGui {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;

        ctx.set_pixels_per_point(1.2);
        ctx.set_visuals(Visuals::dark());

        SkySaveGui {
            state: State::default(),
            message_ch: mpsc::channel(),
            tabs: None,
        }
    }

    pub fn open_dialog(&mut self, callback_tx: Sender<Message>) {
        thread::spawn(move || {
            let path = rfd::FileDialog::new()
                .add_filter("PMD EoS Saves", &["sav", "dsv"])
                .set_title("Open save file")
                .pick_file();

            if let Some(filepath) = path {
                callback_tx
                    .send(Message::SaveFileOpened { filepath })
                    .unwrap();
            }
        });
    }

    pub fn save_dialog(&mut self, callback_tx: Sender<Message>) {
        thread::spawn(move || {
            let path = rfd::FileDialog::new()
                .add_filter("PMD EoS Saves", &["sav", "dsv"])
                .set_title("Save save file as")
                .save_file();

            if let Some(filepath) = path {
                callback_tx
                    .send(Message::SaveFileSavedAs { filepath })
                    .unwrap();
            }
        });
    }

    pub fn do_open(&mut self, path: PathBuf) {
        match SkySave::open(&path) {
            Ok(mut s) => {
                self.tabs = Some(self.build_tabs(&mut s));
                self.state.filepath = Some(path);
                self.state.save = Some(s);
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }

    pub fn do_save(&mut self, path: PathBuf) {
        if let Some(ref mut save) = self.state.save {
            match save.save(&path) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{:?}", e);
                }
            }
        }
    }

    pub fn build_tabs(&mut self, save: &mut SkySave) -> Tree<TabPane> {
        let mut tiles = Tiles::default();
        let mut ui_tabs = vec![];

        let tabs = vec![
            TabPane {
                name: "General",
                tab_state: GuiTabState::General(GeneralTab::new(save)),
            },
            TabPane {
                name: "Stored Pokemon",
                tab_state: GuiTabState::StoredPokemon(StoredPokemonTab::new(save)),
            },
            TabPane {
                name: "Active Pokemon",
                tab_state: GuiTabState::ActivePokemon(ActivePokemonTab::new(save)),
            },
        ];

        ui_tabs.push({
            let children = tabs
                .into_iter()
                .map(|index| tiles.insert_pane(index))
                .collect();

            tiles.insert_tab_tile(children)
        });

        let root = tiles.insert_tab_tile(ui_tabs);
        Tree::new("tree", root, tiles)
    }
}

impl App for SkySaveGui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top").show(ctx, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    let tx = self.message_ch.0.clone();
                    self.open_dialog(tx);
                    ui.close_menu();
                }

                ui.add_enabled_ui(self.state.filepath.is_some(), |ui| {
                    if ui.button("Save As").clicked() {
                        let tx = self.message_ch.0.clone();
                        self.save_dialog(tx);
                        ui.close_menu();
                    }
                });

                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            if let Some(sv) = ctx.input(|st| st.raw.dropped_files.clone()).first() {
                let path = sv.path.clone().unwrap();
                self.do_open(path);
            }

            if let Ok(msg) = self.message_ch.1.try_recv() {
                match msg {
                    Message::SaveFileOpened { filepath } => self.do_open(filepath),
                    Message::SaveFileSavedAs { filepath } => self.do_save(filepath),
                }
            }

            if let Some(s) = &mut self.state.save {
                let mut be = TabsBehavior { save: s };
                if let Some(t) = &mut self.tabs {
                    t.ui(&mut be, ui);
                }
            } else {
                CentralPanel::default()
                    .frame(containers::Frame::none().outer_margin(Margin::symmetric(64.0, 64.0)))
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading(
                                RichText::new("Sky Save GUI")
                                    .font(FontId::new(32.0, FontFamily::default())),
                            );
                            ui.add_space(48.0);
                            if ui
                                .add_sized([128.0, 48.0], Button::new("Open Save File"))
                                .clicked()
                            {
                                let tx = self.message_ch.0.clone();
                                self.open_dialog(tx);
                            }
                        });
                    });
            }
        });

        TopBottomPanel::bottom("pnl_version").show(ctx, |ui| {
            ui.label(format!(
                "Version: {} (git commit: {})",
                built_info::PKG_VERSION,
                built_info::GIT_COMMIT_HASH_SHORT.unwrap_or("Unknown")
            ));
        });

        if ctx.input(|st| st.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([640.0, 480.0])
            .with_inner_size([640.0, 480.0])
            .with_icon(Arc::new(IconData {
                rgba: ICON_BYTES.to_vec(),
                width: 32,
                height: 32,
            })),

        ..Default::default()
    };

    eframe::run_native(
        "Sky Save GUI",
        options,
        Box::new(|cc| Ok(Box::new(SkySaveGui::new(cc)))),
    )
}
