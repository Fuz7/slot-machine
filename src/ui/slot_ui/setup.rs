use bevy::prelude::*;
use bevy::core_pipeline::bloom::{BloomSettings, BloomCompositeMode};
use super::components::*;
use crate::ui::bet_controls::{BetUpButton, BetDownButton, BetInputField};

pub fn setup_ui(mut commands: Commands) {
    // Camera with enhanced bloom settings for visible effects
    commands.spawn((
        Camera2dBundle::default(),
        BloomSettings {
            intensity: 0.5,
            low_frequency_boost: 0.8,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 1.0,
            prefilter_settings: Default::default(),
            composite_mode: BloomCompositeMode::Additive,
        },
    ));

    // Initialize win bloom state
    commands.insert_resource(WinBloomState {
        is_active: false,
        current_line: 0,
        current_cell: 0,
        flash_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        flash_count: 0,
        max_flashes: 5,
        all_cells_ready: false,
    });

    // Initialize win popup state
    commands.insert_resource(WinPopupState {
        is_active: false,
        win_amount: 0.0,
        multiplier: 1.0,
        popup_timer: Timer::from_seconds(3.0, TimerMode::Once), // 3 second display
        coin_spawn_timer: Timer::from_seconds(0.01, TimerMode::Repeating), // Spawn coins every 0.01s (100 per second)
        coins_spawned: 0,
        max_coins: 150, // Many coins to cover almost entire background
        pending_win: None, // No pending win initially
    });

    // Root UI container
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::srgb(0.1, 0.1, 0.15).into(),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "🎰 SLOT MACHINE 🎰",
                TextStyle {
                    font_size: 48.0,
                    color: Color::srgb(1.0, 0.84, 0.0),
                    ..default()
                },
            ));

            // Player info container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        margin: UiRect::vertical(Val::Px(20.0)),
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.2, 0.2, 0.3, 0.8).into(),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                })
                .with_children(|parent| {
                    create_balance_display(parent);
                    create_bet_controls(parent);
                    create_win_display(parent);
                });

            create_slot_grid(parent);
            create_spin_button(parent);

            // Game instructions
            parent.spawn(TextBundle::from_section(
                "Click SPIN to play! Match 3 symbols in a line to win!",
                TextStyle {
                    font_size: 16.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::vertical(Val::Px(10.0)),
                ..default()
            }));
        });
}

fn create_balance_display(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "BALANCE",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            parent.spawn((
                TextBundle::from_section(
                    "$100.00",
                    TextStyle {
                        font_size: 32.0,
                        color: Color::srgb(0.0, 1.0, 0.0),
                        ..default()
                    },
                ),
                PoolDisplay,
            ));
        });
}

fn create_bet_controls(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "BET",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            
            // Bet control row
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(10.0),
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Decrease bet button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                background_color: Color::srgb(0.6, 0.3, 0.3).into(),
                                border_color: Color::srgb(0.8, 0.8, 0.8).into(),
                                border_radius: BorderRadius::all(Val::Px(5.0)),
                                ..default()
                            },
                            BetDownButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "-",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                        });
                    
                    // Bet amount input field
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(35.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    padding: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::srgb(0.1, 0.1, 0.1).into(),
                                border_color: Color::srgb(0.6, 0.6, 0.6).into(),
                                border_radius: BorderRadius::all(Val::Px(5.0)),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "$5.00",
                                    TextStyle {
                                        font_size: 20.0,
                                        color: Color::srgb(1.0, 1.0, 0.0),
                                        ..default()
                                    },
                                ),
                                BetInputField {
                                    has_focus: false,
                                    is_editing: false,
                                },
                            ));
                        });
                    
                    // Increase bet button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                background_color: Color::srgb(0.3, 0.6, 0.3).into(),
                                border_color: Color::srgb(0.8, 0.8, 0.8).into(),
                                border_radius: BorderRadius::all(Val::Px(5.0)),
                                ..default()
                            },
                            BetUpButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "+",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

fn create_win_display(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "LAST WIN",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            parent.spawn((
                TextBundle::from_section(
                    "$0.00",
                    TextStyle {
                        font_size: 32.0,
                        color: Color::srgb(1.0, 0.27, 0.0),
                        ..default()
                    },
                ),
                WinDisplay,
            ));
        });
}

fn create_slot_grid(parent: &mut ChildBuilder) {
    // Slot machine grid - now organized by columns for animation
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    height: Val::Px(400.0),
                    flex_direction: FlexDirection::Row, // Horizontal layout for columns
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    margin: UiRect::vertical(Val::Px(30.0)),
                    ..default()
                },
                background_color: Color::srgba(0.3, 0.3, 0.4, 0.9).into(),
                border_radius: BorderRadius::all(Val::Px(15.0)),
                ..default()
            },
            SlotGrid,
        ))
        .with_children(|parent| {
            // Create 3 columns
            for col in 0..3 {
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(110.0),
                                height: Val::Px(340.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Center,
                                overflow: Overflow::clip(), // Hide symbols outside visible area
                                ..default()
                            },
                            background_color: Color::srgba(0.2, 0.2, 0.25, 0.8).into(),
                            border_radius: BorderRadius::all(Val::Px(8.0)),
                            ..default()
                        },
                        SlotColumnContainer { column_index: col },
                    ))
                    .with_children(|parent| {
                        // Create 3 visible slots per column
                        for row in 0..3 {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(90.0),
                                        height: Val::Px(90.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    },
                                    background_color: Color::WHITE.into(),
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                },
                                SlotCell { row, col },
                            )).with_children(|parent| {
                                // Add both an image and text as children
                                parent.spawn((
                                    ImageBundle {
                                        style: Style {
                                            width: Val::Px(70.0),
                                            height: Val::Px(70.0),
                                            align_self: AlignSelf::Center,
                                            justify_self: JustifySelf::Center,
                                            margin: UiRect::all(Val::Auto),
                                            ..default()
                                        },
                                        visibility: Visibility::Hidden,
                                        ..default()
                                    },
                                    SlotCellImage,
                                ));
                                
                                parent.spawn(TextBundle {
                                    style: Style {
                                        padding: UiRect::right(Val::Px(65.0)),
                                        ..default()
                                    },
                                    text: Text::from_section(
                                        "?",
                                        TextStyle {
                                            font_size: 50.0,
                                            color: Color::BLACK,
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                });
                            });
                        }
                    });
            }
        });
}

fn create_spin_button(parent: &mut ChildBuilder) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(80.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(20.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                background_color: Color::srgb(0.8, 0.2, 0.2).into(),
                border_color: Color::srgb(0.9, 0.9, 0.9).into(),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            SpinButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "SPIN!",
                TextStyle {
                    font_size: 32.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}