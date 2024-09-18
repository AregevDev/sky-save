use eframe::egui::widget_text::RichText;
use eframe::egui::{
    containers, vec2, Button, CentralPanel, Context, DragValue, FontFamily, FontId, Id, Key,
    Margin, Response, Sense, Stroke, TextEdit, TextStyle, TopBottomPanel, Ui, Vec2,
    ViewportCommand, WidgetText,
};
use eframe::{egui, App, CreationContext, Frame};
use egui::IconData;
use egui_tiles::{Behavior, TabState, TileId, Tiles, Tree, UiResponse};
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

fn general_ui(state: &mut GeneralTab, ui: &mut Ui, _save: &mut SkySave) {
    ui.heading("General Save Data");
    ui.add_space(16.0);
    ui.horizontal(|ui| {
        ui.label("Team name: ");
        ui.add(
            TextEdit::singleline(&mut state.team_name_buf)
                .char_limit(10)
                .hint_text("Team name"),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Held money: ");
        ui.add(DragValue::new(&mut state.held_money).speed(50.0));
    });
    ui.horizontal(|ui| {
        ui.label("Sp Episode held money: ");
        ui.add(DragValue::new(&mut state.sp_episode_held_money).speed(50.0));
    });
    ui.horizontal(|ui| {
        ui.label("Stored money: ");
        ui.add(DragValue::new(&mut state.stored_money).speed(50.0));
    });
    ui.horizontal(|ui| {
        ui.label("Explorer rank: ");
        ui.add(DragValue::new(&mut state.explorer_rank).speed(25.0));
    });
    ui.horizontal(|ui| {
        ui.label("Number of adventures: ");
        ui.add(DragValue::new(&mut state.number_of_adventures).speed(0.25));
    });
}

fn stored_ui(_state: &mut StoredPokemonTab, ui: &mut Ui, _save: &mut SkySave) {
    ui.label("Display the stored Pokemon here.");
}

fn active_ui(_state: &mut ActivePokemonTab, ui: &mut Ui, _save: &mut SkySave) {
    ui.label("Display the active Pokemon here.");
}

#[derive(Debug)]
enum GuiTabState {
    General(GeneralTab),
    StoredPokemon(StoredPokemonTab),
    ActivePokemon(ActivePokemonTab),
}

#[derive(Debug, Default)]
struct GeneralTab {
    team_name_buf: String,
    held_money: u32,
    sp_episode_held_money: u32,
    stored_money: u32,
    explorer_rank: u32,
    number_of_adventures: i32,
}

impl GeneralTab {
    pub fn new(save: &mut SkySave) -> Self {
        Self {
            team_name_buf: save.team_name().unwrap_or("???".into()),
            held_money: save.held_money(),
            sp_episode_held_money: save.sp_episode_held_money(),
            stored_money: save.stored_money(),
            explorer_rank: save.explorer_rank(),
            number_of_adventures: save.number_of_adventurers(),
        }
    }
}

#[derive(Debug, Default)]
struct StoredPokemonTab;

#[derive(Debug, Default)]
struct ActivePokemonTab;

#[derive(Debug)]
struct TabPane {
    name: &'static str,
    tab_state: GuiTabState,
}

#[derive(Debug)]
struct TabsBehavior<'a> {
    save: &'a mut SkySave,
}

impl<'a> Behavior<TabPane> for TabsBehavior<'a> {
    fn pane_ui(&mut self, ui: &mut Ui, _tile_id: TileId, pane: &mut TabPane) -> UiResponse {
        CentralPanel::default()
            .frame(containers::Frame::default().outer_margin(Margin::symmetric(16.0, 16.0)))
            .show_inside(ui, |ui| match &mut pane.tab_state {
                GuiTabState::General(s) => general_ui(s, ui, self.save),
                GuiTabState::StoredPokemon(s) => stored_ui(s, ui, self.save),
                GuiTabState::ActivePokemon(s) => active_ui(s, ui, self.save),
            });

        UiResponse::None
    }

    fn tab_title_for_pane(&mut self, pane: &TabPane) -> WidgetText {
        pane.name.into()
    }

    // Taken from the default implementation, changed to disable dragging.
    fn tab_ui(
        &mut self,
        tiles: &mut Tiles<TabPane>,
        ui: &mut Ui,
        id: Id,
        tile_id: TileId,
        state: &TabState,
    ) -> Response {
        let text = self.tab_title_for_tile(tiles, tile_id);
        let close_btn_size = Vec2::splat(self.close_button_outer_size());
        let close_btn_left_padding = 4.0;
        let font_id = TextStyle::Button.resolve(ui.style());
        let galley = text.into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, font_id);

        let x_margin = self.tab_title_spacing(ui.visuals());

        let button_width = galley.size().x
            + 2.0 * x_margin
            + f32::from(state.closable) * (close_btn_left_padding + close_btn_size.x);
        let (_, tab_rect) = ui.allocate_space(vec2(button_width, ui.available_height()));

        let tab_response = ui.interact(tab_rect, id, Sense::click());

        // Show a gap when dragged
        if ui.is_rect_visible(tab_rect) && !state.is_being_dragged {
            let bg_color = self.tab_bg_color(ui.visuals(), tiles, tile_id, state);
            let stroke = self.tab_outline_stroke(ui.visuals(), tiles, tile_id, state);
            ui.painter()
                .rect(tab_rect.shrink(0.5), 0.0, bg_color, stroke);

            if state.active {
                // Make the tab name area connect with the tab ui area:
                ui.painter().hline(
                    tab_rect.x_range(),
                    tab_rect.bottom(),
                    Stroke::new(stroke.width + 1.0, bg_color),
                );
            }

            // Prepare title's text for rendering
            let text_color = self.tab_text_color(ui.visuals(), tiles, tile_id, state);
            let text_position = egui::Align2::LEFT_CENTER
                .align_size_within_rect(galley.size(), tab_rect.shrink(x_margin))
                .min;

            // Render the title
            ui.painter().galley(text_position, galley, text_color);
        }

        self.on_tab_button(tiles, tile_id, tab_response)
    }
}

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
    pub message_ch: (Sender<Message>, Receiver<Message>),
    pub tabs: Option<Tree<TabPane>>,
}

impl SkySaveGui {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;

        ctx.set_pixels_per_point(1.2);

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

    pub fn open_save(&mut self, path: PathBuf) {
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
                tab_state: GuiTabState::StoredPokemon(StoredPokemonTab::default()),
            },
            TabPane {
                name: "Active Pokemon",
                tab_state: GuiTabState::ActivePokemon(ActivePokemonTab::default()),
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
        TopBottomPanel::top("Top").show(ctx, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    let tx = self.message_ch.0.clone();
                    self.open_dialog(tx);
                    ui.close_menu();
                }

                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            if let Some(sv) = ctx.input(|st| st.raw.dropped_files.clone()).first() {
                let path = sv.path.clone().unwrap();
                self.open_save(path);
            }

            if let Ok(msg) = self.message_ch.1.try_recv() {
                match msg {
                    Message::SaveFileOpened { filepath } => self.open_save(filepath),
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
