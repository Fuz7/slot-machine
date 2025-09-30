use bevy::prelude::*;
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

pub fn setup_ui(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

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
                                                align_self: AlignSelf::Center,
                                                justify_self: JustifySelf::Center,
                                                margin: UiRect::all(Val::Auto),
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
    mut image_query: Query<(&mut UiImage, &mut Visibility), (With<SlotCellImage>, Without<Text>)>,
    animation_state: Res<SlotAnimationState>,
    symbol_assets: Option<Res<SymbolAssets>>,
) {
    if !animation_state.is_animating {
        return;
    }
    
    for (children, cell, mut bg_color) in &mut cell_query {
        let column_index = cell.col;
        
        // Get the column animation data from SlotAnimationState
        if let Some(column) = animation_state.columns.get(column_index) {
            if column.is_spinning {
                // Calculate which symbols should be visible during animation
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
                    }
                    
                    // Spinning effect - slight blur or different background
                    *bg_color = Color::srgb(0.85, 0.85, 0.95).into(); // Light blue tint while spinning
                }
            } else if !column.is_spinning && *animation_state.completed_reels.get(column_index).unwrap_or(&false) {
                // Column has stopped, show final result from the reel
                let visible_symbols = crate::ui::slot_animation::get_visible_symbols_for_column(
                    column, 
                    3
                );
                
                if let Some(symbol) = visible_symbols.get(cell.row) {
                    let mut image_child = None;
                    let mut text_child = None;
                    
                    for &child in children.iter() {
                        if image_query.get_mut(child).is_ok() {
                            image_child = Some(child);
                        } else if text_query.get_mut(child).is_ok() {
                            text_child = Some(child);
                        }
                    }
                    
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
                    
                    if let Some(text_entity) = text_child {
                        if let Ok(mut text) = text_query.get_mut(text_entity) {
                            if used_image {
                                text.sections[0].value = "".to_string();
                            } else {
                                text.sections[0].value = symbol.icon.clone();
                            }
                        }
                    }
                    
                    *bg_color = Color::WHITE.into();
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