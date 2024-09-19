use eframe::egui;
use eframe::egui::{
    containers, vec2, Align, CentralPanel, CollapsingHeader, DragValue, Id, Layout, Margin,
    Response, ScrollArea, Sense, Stroke, TextEdit, TextStyle, Ui, Vec2, WidgetText,
};
use egui_tiles::{Behavior, TabState, TileId, Tiles, UiResponse};
use egui_virtual_list::VirtualList;
use sky_save::{ActiveMove, ActivePokemon, IqMapBits, SkySave, StoredMove, StoredPokemon};

#[derive(Debug)]
pub enum GuiTabState {
    General(GeneralTab),
    StoredPokemon(StoredPokemonTab),
    ActivePokemon(ActivePokemonTab),
}

#[derive(Debug, Default)]
pub struct GeneralTab {
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

pub fn general_ui(state: &mut GeneralTab, ui: &mut Ui, _save: &mut SkySave) {
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

#[derive(Debug)]
pub struct StoredMoveState {
    valid: bool,
    linked: bool,
    switched: bool,
    set: bool,
    id: u16,
    power_boost: u8,
}

impl StoredMoveState {
    pub fn from_stored(stored: &StoredMove) -> Self {
        Self {
            valid: stored.valid(),
            linked: stored.linked(),
            switched: stored.switched(),
            set: stored.set(),
            id: stored.id(),
            power_boost: stored.power_boost(),
        }
    }
}

#[derive(Debug)]
pub struct StoredPokemonState {
    valid: bool,
    level: u8,
    id: u16,
    met_at: u8,
    met_floor: u8,
    evolved_at: (u8, u8),
    iq: u16,
    hp: u16,
    attack: u8,
    sp_attack: u8,
    defense: u8,
    sp_defense: u8,
    exp: u32,
    iq_map: IqMapBits,
    tactic: u8,
    moves: [StoredMoveState; 4],
    name: String,
}

impl StoredPokemonState {
    pub fn from_stored(stored: &StoredPokemon) -> Self {
        let moves = stored.moves();
        Self {
            valid: stored.valid(),
            level: stored.level(),
            id: stored.id(),
            met_at: stored.met_at(),
            met_floor: stored.met_floor(),
            evolved_at: stored.evolved_at(),
            iq: stored.iq(),
            hp: stored.hp(),
            attack: stored.attack(),
            sp_attack: stored.sp_attack(),
            defense: stored.defense(),
            sp_defense: stored.sp_defense(),
            exp: stored.exp(),
            iq_map: stored.iq_map(),
            tactic: stored.tactic(),
            moves: [
                StoredMoveState::from_stored(&moves[0]),
                StoredMoveState::from_stored(&moves[1]),
                StoredMoveState::from_stored(&moves[2]),
                StoredMoveState::from_stored(&moves[3]),
            ],
            name: stored.name().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct StoredPokemonTab {
    list: VirtualList,
    current: usize,
    stored: Box<[StoredPokemon]>,
    item_state: StoredPokemonState,
}

impl StoredPokemonTab {
    pub fn new(save: &mut SkySave) -> Self {
        let stored = save.stored_pokemon();
        let current = 0;
        let item_state = StoredPokemonState::from_stored(&stored[current]);

        Self {
            stored,
            list: VirtualList::new(),
            current,
            item_state,
        }
    }
}

pub fn stored_ui(state: &mut StoredPokemonTab, ui: &mut Ui, _save: &mut SkySave) {
    ui.heading("Stored Pokemon");
    ui.add_space(16.0);
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ScrollArea::vertical().id_source("scroll1").show(ui, |ui| {
                ui.set_width(128.0);
                state
                    .list
                    .ui_custom_layout(ui, state.stored.len(), |ui, index| {
                        egui::Frame::canvas(ui.style())
                            .outer_margin(Margin {
                                right: 16.0,
                                ..Default::default()
                            })
                            .show(ui, |ui| {
                                ui.with_layout(
                                    Layout::top_down(Align::Min).with_cross_justify(true),
                                    |ui| {
                                        if ui
                                            .selectable_label(
                                                index == state.current,
                                                state.stored[index].name().unwrap().to_string(),
                                            )
                                            .clicked()
                                        {
                                            state.current = index;
                                            state.item_state = StoredPokemonState::from_stored(
                                                &state.stored[index],
                                            );
                                        }
                                    },
                                );
                            });
                        1
                    });
            });
        });
        ui.separator();
        ui.vertical(|ui| {
            ScrollArea::vertical().id_source("scroll2").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Valid: ");
                    ui.checkbox(&mut state.item_state.valid, "");
                });
                ui.add_enabled_ui(state.item_state.valid, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("ID: ");
                        ui.add(DragValue::new(&mut state.item_state.id).speed(1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Name: ");
                        ui.add(TextEdit::singleline(&mut state.item_state.name).char_limit(10));
                    });
                    CollapsingHeader::new("Details")
                        .id_source("details")
                        .show_unindented(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Level: ");
                                ui.add(
                                    DragValue::new(&mut state.item_state.level)
                                        .range(0..=100)
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("Met at: ");
                                ui.add(DragValue::new(&mut state.item_state.met_at).speed(1.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Met floor: ");
                                ui.add(DragValue::new(&mut state.item_state.met_floor).speed(1.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Evolved at: ");
                                ui.add(
                                    DragValue::new(&mut state.item_state.evolved_at.0).speed(1.0),
                                );
                                ui.add(
                                    DragValue::new(&mut state.item_state.evolved_at.1).speed(1.0),
                                );
                            });
                        });
                    CollapsingHeader::new("Stats")
                        .id_source("stats")
                        .show_unindented(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("HP: ");
                                ui.add(DragValue::new(&mut state.item_state.hp).speed(1.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Attack: ");
                                ui.add(DragValue::new(&mut state.item_state.attack).speed(1.0));
                                ui.label("Sp. Attack: ");
                                ui.add(DragValue::new(&mut state.item_state.sp_attack).speed(1.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Defense: ");
                                ui.add(DragValue::new(&mut state.item_state.defense).speed(1.0));
                                ui.label("Sp. Defense: ");
                                ui.add(DragValue::new(&mut state.item_state.sp_defense).speed(1.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("EXP: ");
                                ui.add(DragValue::new(&mut state.item_state.exp).speed(1.0));
                                ui.label("IQ: ");
                                ui.add(DragValue::new(&mut state.item_state.iq).speed(1.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Tactic: ");
                                ui.add(DragValue::new(&mut state.item_state.tactic).speed(1.0));
                            });
                        });
                    CollapsingHeader::new("Moves")
                        .id_source("moves")
                        .show_unindented(ui, |ui| {
                            for m in state.item_state.moves.iter_mut() {
                                ui.horizontal(|ui| {
                                    ui.label("ID: ");
                                    ui.add(DragValue::new(&mut m.id).speed(1.0));
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing = vec2(0.25, 0.0);
                                        ui.checkbox(&mut m.valid, "").on_hover_text("Valid");
                                        ui.checkbox(&mut m.linked, "").on_hover_text("Linked");
                                        ui.checkbox(&mut m.switched, "").on_hover_text("Switched");
                                        ui.checkbox(&mut m.set, "").on_hover_text("Set");
                                    });
                                    ui.add(DragValue::new(&mut m.power_boost).speed(1.0))
                                        .on_hover_text("Power boost");
                                });
                            }
                        });
                    CollapsingHeader::new("IQ Map")
                        .id_source("iq_map")
                        .show_unindented(ui, |ui| {
                            ui.label(state.item_state.iq_map.to_string());
                        });
                });
            });
        });
    });
}

#[derive(Debug)]
pub struct ActiveMoveState {
    valid: bool,
    linked: bool,
    switched: bool,
    set: bool,
    sealed: bool,
    id: u16,
    pp: u8,
    power_boost: u8,
}

impl ActiveMoveState {
    pub fn from_active(active: &ActiveMove) -> Self {
        Self {
            valid: active.valid(),
            linked: active.linked(),
            switched: active.switched(),
            set: active.set(),
            sealed: active.sealed(),
            id: active.id(),
            pp: active.pp(),
            power_boost: active.power_boost(),
        }
    }
}

#[derive(Debug)]
pub struct ActivePokemonState {
    valid: bool,
    level: u8,
    met_at: u8,
    met_floor: u8,
    iq: u16,
    roaster_number: u16,
    id: u16,
    current_hp: u16,
    max_hp: u16,
    attack: u8,
    sp_attack: u8,
    defense: u8,
    sp_defense: u8,
    exp: u32,
    moves: [ActiveMoveState; 4],
    iq_map: IqMapBits,
    tactic: u8,
    name: String,
}

impl ActivePokemonState {
    pub fn from_active(active: &ActivePokemon) -> Self {
        Self {
            valid: active.valid(),
            level: active.level(),
            met_at: active.met_at(),
            met_floor: active.met_floor(),
            iq: active.iq(),
            roaster_number: active.roaster_number(),
            id: active.id(),
            current_hp: active.current_hp(),
            max_hp: active.max_hp(),
            attack: active.attack(),
            sp_attack: active.sp_attack(),
            defense: active.defense(),
            sp_defense: active.sp_defense(),
            exp: active.exp(),
            moves: [
                ActiveMoveState::from_active(&active.moves()[0]),
                ActiveMoveState::from_active(&active.moves()[1]),
                ActiveMoveState::from_active(&active.moves()[2]),
                ActiveMoveState::from_active(&active.moves()[3]),
            ],
            iq_map: active.iq_map(),
            tactic: active.tactic(),
            name: active.name().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct ActivePokemonTab {
    list: VirtualList,
    current: usize,
    active: Box<[ActivePokemon]>,
    item_state: ActivePokemonState,
}

impl ActivePokemonTab {
    pub fn new(save: &mut SkySave) -> Self {
        let active = save.active_pokemon();
        let current = 0;
        let item_state = ActivePokemonState::from_active(&active[current]);

        Self {
            active,
            list: VirtualList::new(),
            current,
            item_state,
        }
    }
}

pub fn active_ui(state: &mut ActivePokemonTab, ui: &mut Ui, _save: &mut SkySave) {
    ui.heading("Active Pokemon");
    ui.add_space(16.0);
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ScrollArea::vertical().id_source("scroll1").show(ui, |ui| {
                ui.set_width(128.0);
                state
                    .list
                    .ui_custom_layout(ui, state.active.len(), |ui, index| {
                        egui::Frame::canvas(ui.style())
                            .outer_margin(Margin {
                                right: 16.0,
                                ..Default::default()
                            })
                            .show(ui, |ui| {
                                ui.with_layout(
                                    Layout::top_down(Align::Min).with_cross_justify(true),
                                    |ui| {
                                        if ui
                                            .selectable_label(
                                                index == state.current,
                                                state.active[index].name().unwrap().to_string(),
                                            )
                                            .clicked()
                                        {
                                            state.current = index;
                                            state.item_state = ActivePokemonState::from_active(
                                                &state.active[index],
                                            );
                                        }
                                    },
                                );
                            });
                        1
                    });
            });
        });
        ui.separator();
        ui.vertical(|ui| {
            ScrollArea::vertical().id_source("scroll2").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Valid: ");
                    ui.checkbox(&mut state.item_state.valid, "");
                });
                ui.add_enabled_ui(state.item_state.valid, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("ID: ");
                        ui.add(DragValue::new(&mut state.item_state.id).speed(1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Name: ");
                        ui.add(TextEdit::singleline(&mut state.item_state.name).char_limit(10));
                    });
                    CollapsingHeader::new("Details")
                        .id_source("details")
                        .show_unindented(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Level: ");
                                ui.add(
                                    DragValue::new(&mut state.item_state.level)
                                        .range(0..=100)
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("Met at: ");
                                ui.add(DragValue::new(&mut state.item_state.met_at).speed(1.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Met floor: ");
                                ui.add(DragValue::new(&mut state.item_state.met_floor).speed(1.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Roaster number: ");
                                ui.add(
                                    DragValue::new(&mut state.item_state.roaster_number)
                                        .speed(1.0)
                                        .range(1..=4),
                                );
                            });
                        });
                    CollapsingHeader::new("Stats")
                        .id_source("stats")
                        .show_unindented(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Current HP: ");
                                ui.add(DragValue::new(&mut state.item_state.current_hp).speed(1.0));
                                ui.label("Max HP: ");
                                ui.add(DragValue::new(&mut state.item_state.max_hp).speed(1.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Attack: ");
                                ui.add(DragValue::new(&mut state.item_state.attack).speed(1.0));
                                ui.label("Sp. Attack: ");
                                ui.add(DragValue::new(&mut state.item_state.sp_attack).speed(1.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Defense: ");
                                ui.add(DragValue::new(&mut state.item_state.defense).speed(1.0));
                                ui.label("Sp. Defense: ");
                                ui.add(DragValue::new(&mut state.item_state.sp_defense).speed(1.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("EXP: ");
                                ui.add(DragValue::new(&mut state.item_state.exp).speed(1.0));
                                ui.label("IQ: ");
                                ui.add(DragValue::new(&mut state.item_state.iq).speed(1.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Tactic: ");
                                ui.add(DragValue::new(&mut state.item_state.tactic).speed(1.0));
                            });
                        });
                    CollapsingHeader::new("Moves")
                        .id_source("moves")
                        .show_unindented(ui, |ui| {
                            for m in state.item_state.moves.iter_mut() {
                                ui.horizontal(|ui| {
                                    ui.label("ID: ");
                                    ui.add(DragValue::new(&mut m.id).speed(1.0));
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing = vec2(0.25, 0.0);
                                        ui.checkbox(&mut m.valid, "").on_hover_text("Valid");
                                        ui.checkbox(&mut m.linked, "").on_hover_text("Linked");
                                        ui.checkbox(&mut m.switched, "").on_hover_text("Switched");
                                        ui.checkbox(&mut m.set, "").on_hover_text("Set");
                                        ui.checkbox(&mut m.sealed, "").on_hover_text("Sealed");
                                    });
                                    ui.add(DragValue::new(&mut m.pp).speed(1.0))
                                        .on_hover_text("PP");
                                    ui.add(DragValue::new(&mut m.power_boost).speed(1.0))
                                        .on_hover_text("Power boost");
                                });
                            }
                        });
                    CollapsingHeader::new("IQ Map")
                        .id_source("iq_map")
                        .show_unindented(ui, |ui| {
                            ui.label(state.item_state.iq_map.to_string());
                        });
                });
            });
        });
    });
}

#[derive(Debug)]
pub struct TabPane {
    pub name: &'static str,
    pub tab_state: GuiTabState,
}

#[derive(Debug)]
pub struct TabsBehavior<'a> {
    pub save: &'a mut SkySave,
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
