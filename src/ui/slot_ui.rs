use bevy::prelude::*;

// Import all the modular components
mod components;
mod setup;
mod displays;
mod events;
mod animations;

pub use components::*;
pub use setup::*;
pub use displays::*;
pub use events::*;
pub use animations::*;

pub struct SlotUIPlugin;

impl Plugin for SlotUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize game state resource
            .init_resource::<GameState>()
            .init_resource::<WinBloomState>()
            .init_resource::<WinPopupState>()
            // Setup systems
            .add_systems(Startup, setup_ui)
            // Update systems
            .add_systems(Update, (
                handle_spin_button,
                update_displays,
                update_slot_display_animation,
                update_spin_button_text,
                process_spin_results,
                start_win_bloom_animation,
                update_win_bloom_animation,
                show_win_popup,
                animate_coin_flood,
                update_win_popup,
            ));
    }
}