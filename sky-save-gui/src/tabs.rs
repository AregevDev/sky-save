use eframe::egui;
use eframe::egui::{
    containers, vec2, Align, CentralPanel, CollapsingHeader, Color32, DragValue, Id, Layout,
    Margin, Response, RichText, ScrollArea, Sense, Stroke, TextEdit, TextStyle, Ui, Vec2,
    WidgetText,
};
use egui_tiles::{Behavior, TabState, TileId, Tiles, UiResponse};
use egui_virtual_list::VirtualList;
use sky_save::{ActivePokemon, PmdString, SkySave, StoredPokemon};

#[derive(Debug)]
pub enum GuiTabState {
    General(GeneralTab),
    StoredPokemon(StoredPokemonTab),
    ActivePokemon(ActivePokemonTab),
}

#[derive(Debug, Default)]
pub struct GeneralTab {
    name_buffer: String,
}

impl GeneralTab {
    pub fn new(save: &mut SkySave) -> Self {
        Self {
            name_buffer: save.general.team_name.to_string_until_nul(),
        }
    }
}

pub fn general_ui(state: &mut GeneralTab, ui: &mut Ui, save: &mut SkySave) {
    save.general.team_name = PmdString::from(state.name_buffer.as_bytes());

    ui.heading("General Save Data");
    ui.add_space(16.0);
    ui.horizontal(|ui| {
        ui.label("Team name: ");
        ui.add(
            TextEdit::singleline(&mut state.name_buffer)
                .char_limit(10)
                .hint_text("Team name"),
        );
    });

    ui.horizontal(|ui| {
        ui.label("Held money: ");
        ui.add(DragValue::new(&mut save.general.held_money).speed(50.0));
    });
    ui.horizontal(|ui| {
        ui.label("Sp Episode held money: ");
        ui.add(DragValue::new(&mut save.general.sp_episode_held_money).speed(50.0));
    });
    ui.horizontal(|ui| {
        ui.label("Stored money: ");
        ui.add(DragValue::new(&mut save.general.stored_money).speed(50.0));
    });
    ui.horizontal(|ui| {
        ui.label("Explorer rank: ");
        ui.add(DragValue::new(&mut save.general.explorer_rank).speed(25.0));
    });
    ui.horizontal(|ui| {
        ui.label("Number of adventures: ");
        ui.add(DragValue::new(&mut save.general.number_of_adventures).speed(0.25));
    });
}

#[derive(Debug)]
pub struct StoredPokemonTab {
    list: VirtualList,
    current: usize,
    item_state: StoredPokemon,
    name_buffer: String,
}

impl StoredPokemonTab {
    pub fn new(save: &mut SkySave) -> Self {
        let current = 0;
        let stored = save.stored_pokemon[current].clone();
        let name_buffer = save.stored_pokemon[current].name.to_string_until_nul();

        Self {
            list: VirtualList::new(),
            current,
            item_state: stored,
            name_buffer,
        }
    }
}

pub fn stored_ui(state: &mut StoredPokemonTab, ui: &mut Ui, save: &mut SkySave) {
    ui.heading("Stored Pokemon");
    ui.add_space(16.0);
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ScrollArea::vertical().id_source("scroll1").show(ui, |ui| {
                ui.set_width(128.0);
                state
                    .list
                    .ui_custom_layout(ui, save.stored_pokemon.len(), |ui, index| {
                        egui::Frame::canvas(ui.style())
                            .outer_margin(Margin {
                                right: 16.0,
                                ..Default::default()
                            })
                            .show(ui, |ui| {
                                ui.with_layout(
                                    Layout::top_down(Align::Min).with_cross_justify(true),
                                    |ui| {
                                        let selected = index == state.current;
                                        let text = if save.stored_pokemon[index].valid {
                                            RichText::new(
                                                save.stored_pokemon[index]
                                                    .name
                                                    .to_string_until_nul(),
                                            )
                                        } else {
                                            RichText::new("[Empty]")
                                                .color(Color32::from_hex("#666666").unwrap())
                                        };

                                        if ui.selectable_label(selected, text).clicked() {
                                            state.current = index;
                                            state.item_state = save.stored_pokemon[index].clone();
                                            state.name_buffer =
                                                state.item_state.name.to_string_until_nul()
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
                save.stored_pokemon[state.current].name =
                    PmdString::from(state.name_buffer.as_bytes());

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
                        ui.add(TextEdit::singleline(&mut state.name_buffer).char_limit(10));
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
                                    DragValue::new(&mut state.item_state.evolved_at_1).speed(1.0),
                                );
                                ui.add(
                                    DragValue::new(&mut state.item_state.evolved_at_2).speed(1.0),
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
                            let moves = [
                                &mut state.item_state.move_1,
                                &mut state.item_state.move_2,
                                &mut state.item_state.move_3,
                                &mut state.item_state.move_4,
                            ];
                            for m in moves {
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
pub struct ActivePokemonTab {
    list: VirtualList,
    current: usize,
    item_state: ActivePokemon,
    name_buffer: String,
}

impl ActivePokemonTab {
    pub fn new(save: &mut SkySave) -> Self {
        let current = 0;
        let item_state = save.active_pokemon[current].clone();
        let name_buffer = save.active_pokemon[current].name.to_string_until_nul();

        Self {
            list: VirtualList::new(),
            current,
            item_state,
            name_buffer,
        }
    }
}

pub fn active_ui(state: &mut ActivePokemonTab, ui: &mut Ui, save: &mut SkySave) {
    ui.heading("Active Pokemon");
    ui.add_space(16.0);
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ScrollArea::vertical().id_source("scroll1").show(ui, |ui| {
                ui.set_width(128.0);
                state
                    .list
                    .ui_custom_layout(ui, save.active_pokemon.len(), |ui, index| {
                        egui::Frame::canvas(ui.style())
                            .outer_margin(Margin {
                                right: 16.0,
                                ..Default::default()
                            })
                            .show(ui, |ui| {
                                ui.with_layout(
                                    Layout::top_down(Align::Min).with_cross_justify(true),
                                    |ui| {
                                        let selected = index == state.current;
                                        let text = if save.active_pokemon[index].valid {
                                            RichText::new(
                                                save.active_pokemon[index]
                                                    .name
                                                    .to_string_until_nul(),
                                            )
                                        } else {
                                            RichText::new("[Empty]")
                                                .color(Color32::from_hex("#666666").unwrap())
                                        };

                                        if ui.selectable_label(selected, text).clicked() {
                                            state.current = index;
                                            state.item_state = save.active_pokemon[index].clone();
                                            state.name_buffer =
                                                state.item_state.name.to_string_until_nul()
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
            save.active_pokemon[state.current].name = PmdString::from(state.name_buffer.as_bytes());

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
                        ui.add(TextEdit::singleline(&mut state.name_buffer).char_limit(10));
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
                            let moves = [
                                &mut state.item_state.move_1,
                                &mut state.item_state.move_2,
                                &mut state.item_state.move_3,
                                &mut state.item_state.move_4,
                            ];

                            for m in moves {
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
