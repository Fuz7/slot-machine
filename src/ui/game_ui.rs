use bevy::prelude::*;
use crate::ui::slot_animation::SlotAnimationState;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SlotAnimationState::default())
           .add_systems(Startup, crate::ui::slot_ui::setup_ui)
                       .add_systems(Update, (
                crate::ui::slot_ui::handle_spin_button,
                crate::ui::slot_ui::update_slot_display_animation, // Handles both spinning and stopped reels
                crate::ui::slot_ui::update_displays,
                crate::ui::slot_ui::update_spin_button_text,
                crate::ui::slot_ui::process_spin_results,
                crate::ui::slot_ui::start_win_bloom_animation,
                crate::ui::slot_ui::update_win_bloom_animation,
                crate::ui::bet_controls::handle_bet_controls,
                crate::ui::bet_controls::handle_bet_input,
            ));
    }
}