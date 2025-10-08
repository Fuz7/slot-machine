use bevy::prelude::*;
use super::components::*;
use crate::entities::slot_machine::SlotMachine;
use crate::ui::slot_animation::{SlotAnimationState, start_slot_animation};
use crate::ui::bet_controls::{BetUpButton, BetDownButton, BetInputField};

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

pub fn process_spin_results(
    mut commands: Commands,
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
                
                // Win popup will be handled by the delayed mechanism after bloom completes
                println!("🎯 Win detected, bloom animation will trigger popup after completion");
            } else {
                println!("❌ No wins this time. Pool remains: ${:.2}", game_state.player_pool);
            }
        }
    }
}