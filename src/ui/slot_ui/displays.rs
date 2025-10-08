use bevy::prelude::*;
use super::components::*;
use crate::entities::slot_machine::Symbol;
use crate::ui::assets::{SymbolAssets, get_symbol_texture};
use crate::ui::slot_animation::SlotAnimationState;
use crate::ui::bet_controls::BetInputField;

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