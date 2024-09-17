use eframe::egui::widget_text::RichText;
use eframe::egui::{containers, Button, Context, FontFamily, FontId, Key, Margin, ViewportCommand};
use eframe::{egui, App, CreationContext, Frame};
use egui::IconData;
use sky_save::SkySave;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;

pub const ICON_BYTES: &[u8] = include_bytes!("../res/icon.rgba").as_slice();

#[derive(Debug)]
enum Message {
    SaveFileOpened { filepath: PathBuf },
}

#[derive(Debug, Default)]
struct State {
    pub filepath: Option<PathBuf>,
    pub save: Option<SkySave>,
}

#[derive(Debug)]
struct SkySaveGui {
    pub state: State,
    pub ch: (Sender<Message>, Receiver<Message>),
}

impl SkySaveGui {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;

        ctx.set_pixels_per_point(1.2);

        SkySaveGui {
            state: State::default(),
            ch: mpsc::channel(),
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
}

impl App for SkySaveGui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("Top").show(ctx, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    let tx = self.ch.0.clone();
                    self.open_dialog(tx);
                    ui.close_menu();
                }

                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(sv) = ctx.input(|st| st.raw.dropped_files.clone()).first() {
                let path = sv.path.clone().unwrap();
                match SkySave::open(&path) {
                    Ok(s) => {
                        self.state.filepath = Some(path);
                        self.state.save = Some(s);
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                }
            }

            if let Ok(msg) = self.ch.1.try_recv() {
                match msg {
                    Message::SaveFileOpened { filepath } => match SkySave::open(&filepath) {
                        Ok(s) => {
                            self.state.filepath = Some(filepath);
                            self.state.save = Some(s);
                        }
                        Err(e) => {
                            eprintln!("{:?}", e);
                        }
                    },
                }
            }

            if let Some(s) = &self.state.save {
                ui.label(format!(
                    "Team name: {}",
                    s.team_name().unwrap_or("???".into())
                ));
            } else {
                egui::CentralPanel::default()
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
                                let tx = self.ch.0.clone();
                                self.open_dialog(tx);
                            }
                        });
                    });
            }
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
