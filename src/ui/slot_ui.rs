use bevy::prelude::*;
use bevy::core_pipeline::bloom::{BloomSettings, BloomCompositeMode};
use crate::entities::slot_machine::{Symbol, SlotMachine};
use crate::ui::assets::{SymbolAssets, get_symbol_texture};
use crate::ui::slot_animation::{SlotAnimationState, start_slot_animation};
use crate::ui::bet_controls::{BetUpButton, BetDownButton, BetInputField};

#[derive(Resource)]
pub struct GameState {
    pub slot_machine: SlotMachine,
    pub player_pool: f32,
    pub current_bet: f32,
    pub last_grid: Option<Vec<Vec<Symbol>>>,
    pub last_wins: Vec<SimpleWinningLine>,
    pub is_spinning: bool,
    pub last_win_amount: f32,
    pub has_recent_win: bool,
}

#[derive(Clone)]
pub struct SimpleWinningLine {
    pub symbols: Vec<Symbol>,
    pub line_type: crate::entities::slot_machine::LineType,
}

#[derive(Component)]
pub struct SpinButton;

#[derive(Component)]
pub struct SlotGrid;

#[derive(Component)]
pub struct SlotCell {
    pub row: usize,
    pub col: usize,
}

#[derive(Component)]
pub struct SlotCellImage;

#[derive(Component)]
pub struct SlotColumnContainer {
    pub column_index: usize,
}

#[derive(Component)]
pub struct PoolDisplay;

#[derive(Component)]
pub struct BetDisplay;

#[derive(Component)]
pub struct WinDisplay;

#[derive(Component)]
pub struct SpinAnimation {
    pub timer: Timer,
    pub target_symbol: Symbol,
}

#[derive(Component)]
pub struct WinningCell {
    pub timer: Timer,
    pub bloom_phase: BloomPhase,
    pub line_index: usize,
    pub cell_index: usize,
}

#[derive(Component)]
pub struct BloomOverlay;

#[derive(Clone, PartialEq)]
pub enum BloomPhase {
    WaitingToStart,
    SequentialBloom,
    RapidFlashing,
    Finished,
}

#[derive(Resource)]
pub struct WinBloomState {
    pub is_active: bool,
    pub current_line: usize,
    pub current_cell: usize,
    pub flash_timer: Timer,
    pub flash_count: u32,
    pub max_flashes: u32,
}

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
        flash_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        flash_count: 0,
        max_flashes: 6,
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
                    // Pool display
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

                    // Bet display with controls
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

                    // Win display
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
                });

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

            // Spin button
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

pub fn handle_spin_button(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<SpinButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut game_state: ResMut<GameState>,
    mut animation_state: ResMut<SlotAnimationState>,
    slot_machine: Res<SlotMachine>,
) {
    let mut should_start_spin = false;
    
    for (interaction, mut color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if !game_state.is_spinning && !animation_state.is_animating && game_state.player_pool >= game_state.current_bet {
                    should_start_spin = true;
                    game_state.is_spinning = true;
                    
                    // Debug output to verify bet deduction
                    println!("💰 Spinning! Bet: ${:.2}, Pool before: ${:.2}", game_state.current_bet, game_state.player_pool);
                    game_state.player_pool -= game_state.current_bet;
                    println!("💰 Pool after bet: ${:.2}", game_state.player_pool);
                    
                    // Update button text
                    if let Some(child) = children.first() {
                        if let Ok(mut text) = text_query.get_mut(*child) {
                            text.sections[0].value = "SPINNING...".to_string();
                        }
                    }
                    
                    *color = Color::srgb(0.6, 0.1, 0.1).into();
                }
            }
            Interaction::Hovered => {
                if !game_state.is_spinning && !animation_state.is_animating {
                    *color = Color::srgb(0.9, 0.3, 0.3).into();
                }
            }
            Interaction::None => {
                if !game_state.is_spinning && !animation_state.is_animating {
                    *color = Color::srgb(0.8, 0.2, 0.2).into();
                }
                // Always ensure button text is correct when not spinning
                if !game_state.is_spinning && !animation_state.is_animating {
                    if let Some(child) = children.first() {
                        if let Ok(mut text) = text_query.get_mut(*child) {
                            if text.sections[0].value != "SPIN!" {
                                text.sections[0].value = "SPIN!".to_string();
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Start animation outside the loop to avoid borrow checker issues
    if should_start_spin {
        start_slot_animation(animation_state, game_state, slot_machine, commands);
    }
}

pub fn update_slot_display_animation(
    mut cell_query: Query<(&Children, &SlotCell, &mut BackgroundColor)>,
    mut text_query: Query<&mut Text, Without<SlotCellImage>>,
    mut style_query: Query<&mut Style>,
    mut image_query: Query<(&mut UiImage, &mut Visibility), (With<SlotCellImage>, Without<Text>)>,
    animation_state: Res<SlotAnimationState>,
    symbol_assets: Option<Res<SymbolAssets>>,
) {
    // Handle both spinning AND stopped reels using the same logic for consistent alignment
    for (children, cell, mut bg_color) in &mut cell_query {
        let column_index = cell.col;
        
        // Get the column animation data from SlotAnimationState
        if let Some(column) = animation_state.columns.get(column_index) {
            // Always use get_visible_symbols_for_column for consistent alignment
            let visible_symbols = crate::ui::slot_animation::get_visible_symbols_for_column(
                column, 
                3
            );
            
            if let Some(symbol) = visible_symbols.get(cell.row) {
                // Update the display
                let mut image_child = None;
                let mut text_child = None;
                
                for &child in children.iter() {
                    if image_query.get_mut(child).is_ok() {
                        image_child = Some(child);
                    } else if text_query.get_mut(child).is_ok() {
                        text_child = Some(child);
                    }
                }
                
                // Try to use image assets first, fall back to emoji
                let mut used_image = false;
                if let (Some(assets), Some(image_entity)) = (&symbol_assets, image_child) {
                    if let Some(texture) = get_symbol_texture(assets, &symbol.name) {
                        if let Ok((mut image, mut visibility)) = image_query.get_mut(image_entity) {
                            image.texture = texture;
                            *visibility = Visibility::Visible;
                            used_image = true;
                        }
                    }
                }
                
                // Update text
                if let Some(text_entity) = text_child {
                    if let Ok(mut text) = text_query.get_mut(text_entity) {
                        if used_image {
                            text.sections[0].value = "".to_string();
                        } else {
                            text.sections[0].value = symbol.icon.clone();
                        }
                    }
                    
                    // Reset padding when animation starts (remove question mark padding)
                    if let Ok(mut style) = style_query.get_mut(text_entity) {
                        style.padding = UiRect::all(Val::Px(0.0));
                    }
                }
                
                // Set background color based on spinning state
                if column.is_spinning {
                    *bg_color = Color::srgb(0.85, 0.85, 0.95).into(); // Light blue tint while spinning
                } else {
                    *bg_color = Color::WHITE.into(); // Normal color when stopped
                }
            }
        }
    }
}

pub fn update_displays(
    mut pool_query: Query<&mut Text, (With<PoolDisplay>, Without<BetDisplay>, Without<WinDisplay>, Without<BetInputField>)>,
    mut bet_query: Query<&mut Text, (With<BetDisplay>, Without<PoolDisplay>, Without<WinDisplay>, Without<BetInputField>)>,
    input_query: Query<(&Text, &BetInputField), (With<BetInputField>, Without<PoolDisplay>, Without<BetDisplay>, Without<WinDisplay>)>,
    mut win_query: Query<&mut Text, (With<WinDisplay>, Without<PoolDisplay>, Without<BetDisplay>, Without<BetInputField>)>,
    game_state: Res<GameState>,
) {
    // Update pool display
    if let Ok(mut text) = pool_query.get_single_mut() {
        text.sections[0].value = format!("${:.2}", game_state.player_pool);
    }

    // Update bet display (if it exists)
    if let Ok(mut text) = bet_query.get_single_mut() {
        text.sections[0].value = format!("${:.2}", game_state.current_bet);
    }

    // Don't automatically update input field - let the input handler manage it
    // The input field will only be updated when not being actively edited

    // Update win display with persistent last win and color change
    if let Ok(mut text) = win_query.get_single_mut() {
        text.sections[0].value = format!("${:.2}", game_state.last_win_amount);
        
        // Change color to green if there's a recent win, otherwise white
        if game_state.has_recent_win && game_state.last_win_amount > 0.0 {
            text.sections[0].style.color = Color::srgb(0.0, 1.0, 0.0); // Green for recent wins
        } else {
            text.sections[0].style.color = Color::srgb(0.8, 0.8, 0.8); // Gray for no recent wins
        }
    }
}

pub fn update_slot_display_final(
    mut cell_query: Query<(&Children, &SlotCell, &mut BackgroundColor)>,
    mut text_query: Query<&mut Text, Without<SlotCellImage>>,
    mut image_query: Query<(&mut UiImage, &mut Visibility), (With<SlotCellImage>, Without<Text>)>,
    animation_state: Res<SlotAnimationState>,
    game_state: Res<GameState>,
    symbol_assets: Option<Res<SymbolAssets>>,
) {
    // Only update final display when animation is NOT running
    // This prevents frame changes after animation completes
    if animation_state.is_animating {
        return; 
    }
    
    for (children, cell, mut bg_color) in &mut cell_query {
        let column_index = cell.col;
        let row_index = cell.row;
        
        // Priority: Use animation results if available and complete, otherwise fallback to game state
        let symbol = if !animation_state.results.is_empty() && 
                       animation_state.results.len() == 3 && // All 3 columns completed
                       column_index < animation_state.results.len() &&
                       row_index < animation_state.results[column_index].len() {
            // Use final animation results (these should match the game state exactly)
            &animation_state.results[column_index][row_index]
        } else if let Some(ref grid) = game_state.last_grid {
            // Fallback to game state grid: [row][column] 
            if row_index < grid.len() && column_index < grid[row_index].len() {
                &grid[row_index][column_index]
            } else {
                continue;
            }
        } else {
            continue;
        };

        // Update the display for this cell
        let mut image_child = None;
        let mut text_child = None;
        
        for &child in children.iter() {
            if image_query.contains(child) {
                image_child = Some(child);
            } else if text_query.contains(child) {
                text_child = Some(child);
            }
        }

        // Check if we have assets available for this symbol
        let has_asset = if let Some(assets) = &symbol_assets {
            matches!(symbol.name.as_str(), "Cherry" | "Lemon" | "Bell" | "Star" | "Seven")
        } else {
            false
        };

        // Update image display if assets are available
        if let (Some(image_entity), Some(assets)) = (image_child, &symbol_assets) {
            if let Ok((mut ui_image, mut visibility)) = image_query.get_mut(image_entity) {
                match symbol.name.as_str() {
                    "Cherry" => {
                        ui_image.texture = assets.cherry.clone();
                        *visibility = Visibility::Visible;
                    }
                    "Lemon" => {
                        ui_image.texture = assets.lemon.clone();
                        *visibility = Visibility::Visible;
                    }
                    "Bell" => {
                        ui_image.texture = assets.bell.clone();
                        *visibility = Visibility::Visible;
                    }
                    "Star" => {
                        ui_image.texture = assets.star.clone();
                        *visibility = Visibility::Visible;
                    }
                    "Seven" => {
                        ui_image.texture = assets.seven.clone();
                        *visibility = Visibility::Visible;
                    }
                    _ => {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }

        // Update text display (only show if no asset is available)
        if let Some(text_entity) = text_child {
            if let Ok(mut text) = text_query.get_mut(text_entity) {
                text.sections[0].value = symbol.icon.clone();
                // Hide text if we're showing an image, otherwise show it
                text.sections[0].style.color = if has_asset { 
                    Color::NONE  // Transparent when image is shown
                } else { 
                    Color::BLACK  // Visible when no image
                };
            }
        }
    }
}

pub fn update_spin_button_text(
    mut button_query: Query<&Children, With<SpinButton>>,
    mut text_query: Query<&mut Text>,
    game_state: Res<GameState>,
    animation_state: Res<SlotAnimationState>,
) {
    if let Ok(children) = button_query.get_single() {
        if let Some(child) = children.first() {
            if let Ok(mut text) = text_query.get_mut(*child) {
                if !game_state.is_spinning && !animation_state.is_animating {
                    if text.sections[0].value != "SPIN!" {
                        text.sections[0].value = "SPIN!".to_string();
                    }
                } else if game_state.is_spinning || animation_state.is_animating {
                    if text.sections[0].value != "SPINNING..." {
                        text.sections[0].value = "SPINNING...".to_string();
                    }
                }
            }
        }
    }
}

pub fn process_spin_results(
    mut game_state: ResMut<GameState>,
    mut button_query: Query<(&mut BackgroundColor, &Children), With<SpinButton>>,
    mut text_query: Query<&mut Text>,
    animation_state: Res<SlotAnimationState>,
) {
    // Check if animation is complete and we need to process results
    if !animation_state.is_animating && game_state.is_spinning {
        game_state.is_spinning = false;
        
        // Reset button appearance
        if let Ok((mut color, children)) = button_query.get_single_mut() {
            *color = Color::srgb(0.8, 0.2, 0.2).into();
            if let Some(child) = children.first() {
                if let Ok(mut text) = text_query.get_mut(*child) {
                    text.sections[0].value = "SPIN!".to_string();
                }
            }
        }
        
        // Check for wins if we have a grid
        if let Some(grid) = game_state.last_grid.clone() {
            let wins = game_state.slot_machine.check_wins(&grid);
            
            // Convert wins to simplified format
            game_state.last_wins.clear();
            for win in wins {
                let simple_line = SimpleWinningLine {
                    symbols: win.symbols.into_iter().cloned().collect(),
                    line_type: win.line_type,
                };
                game_state.last_wins.push(simple_line);
            }
            
            // Add winnings to pool
            let total_win: f32 = game_state.last_wins.iter()
                .map(|line| line.symbols[0].multiplier * game_state.current_bet + line.symbols[0].addition)
                .sum();
            
            if total_win > 0.0 {
                println!("🎉 WIN! Bet: ${:.2}, Multiplier calculation: ${:.2}", game_state.current_bet, total_win);
                println!("🎉 Pool before win: ${:.2}", game_state.player_pool);
                game_state.player_pool += total_win;
                println!("🎉 Pool after win: ${:.2}", game_state.player_pool);
            } else {
                println!("❌ No wins this time. Pool remains: ${:.2}", game_state.player_pool);
            }
        }
    }
}

pub fn start_win_bloom_animation(
    mut commands: Commands,
    mut bloom_state: ResMut<WinBloomState>,
    game_state: Res<GameState>,
    cell_query: Query<(Entity, &SlotCell), With<SlotCell>>,
) {
    // Only start if we have wins and bloom isn't already active
    if !game_state.last_wins.is_empty() && !bloom_state.is_active {
        bloom_state.is_active = true;
        bloom_state.current_line = 0;
        bloom_state.current_cell = 0;
        bloom_state.flash_count = 0;
        
        println!("🌟 Starting win bloom animation for {} lines!", game_state.last_wins.len());
        
        // Add WinningCell components to the winning cells
        for (line_index, win_line) in game_state.last_wins.iter().enumerate() {
            // For horizontal lines, the positions are predictable
            match win_line.line_type {
                crate::entities::slot_machine::LineType::Horizontal(row) => {
                    for col in 0..3 {
                        for (entity, cell) in &cell_query {
                            if cell.row == row && cell.col == col {
                                commands.entity(entity).insert(WinningCell {
                                    timer: Timer::from_seconds(0.3, TimerMode::Once),
                                    bloom_phase: BloomPhase::WaitingToStart,
                                    line_index,
                                    cell_index: col,
                                });
                            }
                        }
                    }
                }
                crate::entities::slot_machine::LineType::Diagonal(diag_type) => {
                    let positions = if diag_type == 0 {
                        vec![(0, 0), (1, 1), (2, 2)] // Top-left to bottom-right
                    } else {
                        vec![(0, 2), (1, 1), (2, 0)] // Top-right to bottom-left
                    };
                    
                    for (cell_index, (row, col)) in positions.iter().enumerate() {
                        for (entity, cell) in &cell_query {
                            if cell.row == *row && cell.col == *col {
                                commands.entity(entity).insert(WinningCell {
                                    timer: Timer::from_seconds(0.3, TimerMode::Once),
                                    bloom_phase: BloomPhase::WaitingToStart,
                                    line_index,
                                    cell_index,
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn update_win_bloom_animation(
    time: Res<Time>,
    mut bloom_state: ResMut<WinBloomState>,
    game_state: Res<GameState>,
    mut winning_cells: Query<(Entity, &mut WinningCell), With<WinningCell>>,
    mut overlay_query: Query<&mut BackgroundColor, With<BloomOverlay>>,
    cell_children: Query<&Children>,
    mut commands: Commands,
) {
    if !bloom_state.is_active {
        return;
    }
    
    bloom_state.flash_timer.tick(time.delta());
    
    // Update winning cell timers and phases
    for (entity, mut winning_cell) in &mut winning_cells {
        winning_cell.timer.tick(time.delta());
        
        match winning_cell.bloom_phase {
            BloomPhase::WaitingToStart => {
                // Check if it's this cell's turn to start blooming
                if winning_cell.line_index == bloom_state.current_line 
                    && winning_cell.cell_index == bloom_state.current_cell {
                    winning_cell.bloom_phase = BloomPhase::SequentialBloom;
                    winning_cell.timer.reset();
                    
                    // Create a bright glowing overlay sprite
                    let overlay = commands.spawn((
                        NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::srgba(1.0, 0.8, 0.0, 0.8).into(), // Bright yellow with transparency
                            z_index: ZIndex::Local(10),
                            ..default()
                        },
                        BloomOverlay,
                    )).id();
                    
                    // Add the overlay as a child of the cell
                    commands.entity(entity).add_child(overlay);
                    
                    println!("🌟 Cell ({}, {}) starting bloom!", winning_cell.cell_index, winning_cell.line_index);
                    
                    // Move to next cell
                    bloom_state.current_cell += 1;
                    if bloom_state.current_cell >= 3 {
                        bloom_state.current_cell = 0;
                        bloom_state.current_line += 1;
                    }
                }
            }
            BloomPhase::SequentialBloom => {
                if winning_cell.timer.finished() {
                    winning_cell.bloom_phase = BloomPhase::RapidFlashing;
                    winning_cell.timer = Timer::from_seconds(0.1, TimerMode::Repeating);
                }
            }
            BloomPhase::RapidFlashing => {
                // Flash the overlay on and off rapidly
                if bloom_state.flash_timer.just_finished() {
                    bloom_state.flash_count += 1;
                    let is_bright = (bloom_state.flash_count % 2) == 1;
                    
                    // Find and update the overlay for this cell
                    if let Ok(children) = cell_children.get(entity) {
                        for &child in children.iter() {
                            if let Ok(mut overlay_color) = overlay_query.get_mut(child) {
                                if is_bright {
                                    *overlay_color = Color::srgba(1.0, 0.8, 0.0, 0.9).into(); // Bright and visible
                                } else {
                                    *overlay_color = Color::srgba(1.0, 0.8, 0.0, 0.2).into(); // Dim
                                }
                            }
                        }
                    }
                    
                    // Check if we've finished flashing
                    if bloom_state.flash_count >= bloom_state.max_flashes {
                        winning_cell.bloom_phase = BloomPhase::Finished;
                    }
                }
            }
            BloomPhase::Finished => {
                // This cell is done
            }
        }
    }
    
    // Check if all animations are complete
    let all_finished = winning_cells.iter().all(|(_, cell)| cell.bloom_phase == BloomPhase::Finished);
    
    if all_finished && bloom_state.current_line >= game_state.last_wins.len() {
        // Clean up
        bloom_state.is_active = false;
        
        // Remove WinningCell components and overlay sprites
        for (entity, _) in &mut winning_cells {
            // Remove overlay children first
            if let Ok(children) = cell_children.get(entity) {
                for &child in children.iter() {
                    if overlay_query.get(child).is_ok() {
                        commands.entity(child).despawn();
                    }
                }
            }
            commands.entity(entity).remove::<WinningCell>();
        }
        
        println!("🌟 Win bloom animation completed!");
    }
}