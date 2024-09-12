use eframe::egui::{Context, Key, ViewportCommand};
use eframe::{egui, App, CreationContext, Frame};
use egui::{include_image, IconData, ImageSource};
use sky_save::SkySave;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

pub const ICON_BYTES: &[u8] = include_bytes!("../res/icon.rgba").as_slice();
pub const PNG_BYTES: ImageSource = include_image!("../res/icon.png");

#[derive(Debug, Default)]
struct State {
    pub filepath: Option<PathBuf>,
    pub save: Option<SkySave>,
}

#[derive(Debug, Default)]
struct SkySaveGui {
    pub state: Arc<Mutex<State>>,
}

impl SkySaveGui {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;

        ctx.set_pixels_per_point(1.2);

        SkySaveGui {
            state: Arc::new(Mutex::new(State::default())),
        }
    }
}

impl App for SkySaveGui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("Top").show(ctx, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    let st = Arc::clone(&self.state);
                    thread::spawn(move || {
                        let path = rfd::FileDialog::new()
                            .add_filter("PMD EoS Saves", &["sav", "dsv"])
                            .set_title("Open save file")
                            .pick_file();

                        if let Some(p) = path {
                            match SkySave::open(&p) {
                                Ok(s) => {
                                    let mut st = st.lock().unwrap();
                                    st.filepath = Some(p);
                                    st.save = Some(s);
                                }
                                Err(e) => {
                                    eprintln!("{:?}", e);
                                }
                            }
                        }
                    });

                    ui.close_menu();
                }

                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sky Save GUI");
            let st = Arc::clone(&self.state);
            let mut st = st.lock().unwrap();

            if let Some(sv) = ctx.input(|st| st.raw.dropped_files.clone()).first() {
                let path = sv.path.clone().unwrap();
                match SkySave::open(&path) {
                    Ok(s) => {
                        st.filepath = Some(path);
                        st.save = Some(s);
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                }
            }

            if let Some(s) = &st.save {
                ui.label(format!(
                    "Team name: {}",
                    s.team_name().unwrap_or("???".into())
                ));
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Open a save file with `File -> Open`");
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
            .with_inner_size([640.0, 480.0]),
        window_builder: Some(Box::new(|e| {
            e.with_icon(Arc::new(IconData {
                rgba: ICON_BYTES.to_vec(),
                width: 32,
                height: 32,
            }))
        })),

        ..Default::default()
    };

    eframe::run_native(
        "Sky Save GUI",
        options,
        Box::new(|cc| Ok(Box::new(SkySaveGui::new(cc)))),
    )
}
