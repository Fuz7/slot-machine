use bevy::prelude::*;
use crate::ui::slot_ui::GameState;

pub fn setup_bet_controls(mut commands: Commands, asset_server: Res<AssetServer>) {
    // This is a placeholder for future bet control buttons
}

#[derive(Component)]
pub struct BetUpButton;

#[derive(Component)]
pub struct BetDownButton;

#[derive(Component)]
pub struct BetInputField {
    pub has_focus: bool,
    pub is_editing: bool,
}

pub fn handle_bet_input(
    mut char_input_events: EventReader<ReceivedCharacter>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut input_query: Query<(&mut Text, &mut BetInputField)>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.is_spinning {
        return; // Don't allow input while spinning
    }

    if let Ok((mut text, mut input_field)) = input_query.get_single_mut() {
        // Check if user clicked on input field (simplified focus detection)
        if mouse_button_input.just_pressed(MouseButton::Left) {
            input_field.has_focus = true;
            input_field.is_editing = true;
        }
        
        // Auto-apply bet changes when user starts spinning or using other controls
        // (simplified focus loss detection)
        if input_field.has_focus && input_field.is_editing {
            // If user presses spin or uses other controls, apply the current input
            if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Tab) {
                input_field.has_focus = false;
                input_field.is_editing = false;
                // Ensure the display is properly formatted
                text.sections[0].value = format!("${:.2}", game_state.current_bet);
            }
        }
        
        // Lose focus on escape or when clicking elsewhere (simplified)
        if keys.just_pressed(KeyCode::Escape) {
            input_field.has_focus = false;
            input_field.is_editing = false;
            // Reset to current bet value
            text.sections[0].value = format!("${:.2}", game_state.current_bet);
            return;
        }

        // Only process input if field has focus
        if !input_field.has_focus {
            return;
        }

        // Handle backspace
        if keys.just_pressed(KeyCode::Backspace) {
            input_field.is_editing = true;
            let current_text = &text.sections[0].value;
            if current_text.len() > 1 { // Keep the "$" prefix
                let new_text = format!("${}", &current_text[1..current_text.len()-1]);
                
                // Update game state when backspacing
                let number_part = &new_text[1..];
                if !number_part.is_empty() {
                    if let Ok(bet_amount) = number_part.parse::<f32>() {
                        let clamped_bet = bet_amount.clamp(1.0, game_state.player_pool);
                        if clamped_bet != game_state.current_bet {
                            println!("🎯 Bet updated from backspace: ${:.2} -> ${:.2}", game_state.current_bet, clamped_bet);
                            game_state.current_bet = clamped_bet;
                        }
                    }
                } else {
                    println!("🎯 Bet reset to minimum: ${:.2} -> $1.00", game_state.current_bet);
                    game_state.current_bet = 1.0; // Default to minimum when empty
                }
                
                text.sections[0].value = new_text;
            } else {
                text.sections[0].value = "$1.00".to_string();
                game_state.current_bet = 1.0;
            }
        }

        // Handle character input
        for event in char_input_events.read() {
            input_field.is_editing = true;
            let character_str = event.char.as_str();
            if let Some(character) = character_str.chars().next() {
                if character.is_ascii_digit() || character == '.' {
                    let current_text = &text.sections[0].value;
                    let number_part = &current_text[1..]; // Remove $ prefix
                    
                    // Prevent multiple decimal points
                    if character == '.' && number_part.contains('.') {
                        continue;
                    }
                    
                    let new_text = format!("{}{}", current_text, character);
                    
                    // Update game state in real-time as user types
                    let new_number_part = &new_text[1..];
                    if !new_number_part.is_empty() {
                        if let Ok(bet_amount) = new_number_part.parse::<f32>() {
                            let clamped_bet = bet_amount.clamp(1.0, game_state.player_pool);
                            if clamped_bet != game_state.current_bet {
                                println!("🎯 Bet updated from typing: ${:.2} -> ${:.2}", game_state.current_bet, clamped_bet);
                                game_state.current_bet = clamped_bet;
                            }
                        }
                    }
                    
                    text.sections[0].value = new_text;
                }
            }
        }

        // Handle Enter key to confirm bet
        if keys.just_pressed(KeyCode::Enter) {
            input_field.has_focus = false;
            input_field.is_editing = false;
            
            let current_text = &text.sections[0].value;
            let number_part = &current_text[1..]; // Remove $ prefix
            
            if !number_part.is_empty() {
                if let Ok(bet_amount) = number_part.parse::<f32>() {
                    // Clamp bet amount to valid range (min $1, max = player's balance)
                    game_state.current_bet = bet_amount.clamp(1.0, game_state.player_pool);
                    // Update display to show the clamped value
                    text.sections[0].value = format!("${:.2}", game_state.current_bet);
                } else {
                    // Reset to current bet if invalid input
                    text.sections[0].value = format!("${:.2}", game_state.current_bet);
                }
            } else {
                text.sections[0].value = format!("${:.2}", game_state.current_bet);
            }
        }
    }
}

pub fn handle_bet_controls(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&BetUpButton>, Option<&BetDownButton>),
        Changed<Interaction>,
    >,
    mut input_query: Query<(&mut Text, &mut BetInputField)>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color, bet_up, bet_down) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if !game_state.is_spinning {
                    if bet_up.is_some() {
                        // Increase bet by $1, max = player's balance
                        game_state.current_bet = (game_state.current_bet + 1.0).min(game_state.player_pool);
                        *color = Color::srgb(0.2, 0.8, 0.2).into();
                        println!("🔼 Bet increased to: ${:.2}", game_state.current_bet);
                    } else if bet_down.is_some() {
                        // Decrease bet by $1, min $1
                        game_state.current_bet = (game_state.current_bet - 1.0).max(1.0);
                        *color = Color::srgb(0.8, 0.2, 0.2).into();
                        println!("🔽 Bet decreased to: ${:.2}", game_state.current_bet);
                    }
                    
                    // Always update input field when buttons are pressed
                    if let Ok((mut text, mut input_field)) = input_query.get_single_mut() {
                        text.sections[0].value = format!("${:.2}", game_state.current_bet);
                        input_field.has_focus = false; // Remove focus when button is pressed
                        input_field.is_editing = false; // Stop editing mode
                        println!("📝 Updated input field display to: ${:.2}", game_state.current_bet);
                    }
                }
            }
            Interaction::Hovered => {
                if bet_up.is_some() {
                    *color = Color::srgb(0.4, 0.7, 0.4).into();
                } else if bet_down.is_some() {
                    *color = Color::srgb(0.7, 0.4, 0.4).into();
                }
            }
            Interaction::None => {
                if bet_up.is_some() {
                    *color = Color::srgb(0.3, 0.6, 0.3).into();
                } else if bet_down.is_some() {
                    *color = Color::srgb(0.6, 0.3, 0.3).into();
                }
            }
        }
    }
}