mod util;
mod entities;
mod core;
mod ui;

use bevy::prelude::*;
use entities::slot_machine::{Symbol, Reel, SlotMachine};
use ui::slot_ui::GameState;
use ui::game_ui::GameUIPlugin;
use ui::assets::AssetsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Slot Machine Game".into(),
                resolution: (800.0, 900.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AssetsPlugin) // Load assets first
        .add_plugins(GameUIPlugin)
        .add_systems(Update, ui::slot_animation::update_slot_animation)
        .add_systems(Startup, setup_game)
        .run();
}

fn setup_game(mut commands: Commands) {
    let symbols = vec![
        Symbol::new("🍒", "Cherry", 2.0, 0.0, 50.0),
        Symbol::new("🍋", "Lemon", 3.0, 0.0, 30.0),
        Symbol::new("🔔", "Bell", 5.0, 0.0, 15.0),
        Symbol::new("⭐", "Star", 10.0, 0.0, 4.0),
        Symbol::new("7️⃣", "Seven", 20.0, 0.0, 1.0),
    ];

    let slot_machine = SlotMachine::new(vec![
        Reel::new(symbols.clone()),
        Reel::new(symbols.clone()),
        Reel::new(symbols.clone()),
    ]);

    // Insert slot machine as separate resource for animation system
    commands.insert_resource(slot_machine.clone());

    let game_state = GameState {
        slot_machine,
        player_pool: 100.0,
        current_bet: 5.0,
        last_grid: None,
        last_wins: Vec::new(),
        is_spinning: false,
        last_win_amount: 0.0,
        has_recent_win: false,
    };

    commands.insert_resource(game_state);
}
