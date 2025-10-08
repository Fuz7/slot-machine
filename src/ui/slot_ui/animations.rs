use bevy::prelude::*;
use super::components::*;
use rand::Rng;

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
    mut popup_state: ResMut<WinPopupState>,
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
        
        // Check if there's a pending win popup to show
        if let Some((win_amount, multiplier)) = popup_state.pending_win.take() {
            println!("🎉 Now showing delayed win popup! Amount: ${:.2}, Multiplier: {:.1}x", win_amount, multiplier);
            popup_state.is_active = true;
            popup_state.win_amount = win_amount;
            popup_state.multiplier = multiplier;
            popup_state.popup_timer = Timer::from_seconds(3.0, TimerMode::Once);
            popup_state.coin_spawn_timer = Timer::from_seconds(0.01, TimerMode::Repeating);
            popup_state.coins_spawned = 0;
            popup_state.max_coins = (win_amount / game_state.current_bet * 10.0).min(200.0) as u32; // Many more coins!
        }
    }
}

pub fn show_win_popup(
    mut commands: Commands,
    mut popup_state: ResMut<WinPopupState>,
    existing_popup: Query<Entity, With<WinPopup>>,
) {
    // Debug check if system is running
    if popup_state.is_active {
        println!("🔍 show_win_popup system running, popup_state.is_active = true, existing_popup count = {}", existing_popup.iter().count());
    }
    
    // Only create popup if it's active and doesn't already exist
    if popup_state.is_active && existing_popup.is_empty() {
        println!("🎉 Creating win popup! Amount: ${:.2}, Multiplier: {:.1}x", popup_state.win_amount, popup_state.multiplier);
        // Create the main popup overlay
        let popup_entity = commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(), // Semi-transparent overlay
                z_index: ZIndex::Local(100),
                ..default()
            },
            WinPopup,
        )).id();

        // Create the celebration container
        let celebration_container = commands.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(40.0)),
                    ..default()
                },
                background_color: Color::srgba(1.0, 0.8, 0.0, 0.95).into(), // Golden background
                border_radius: BorderRadius::all(Val::Px(20.0)),
                ..default()
            },
            CelebrationOverlay,
        )).id();

        // "BIG WIN!" text
        let big_win_text = commands.spawn((
            TextBundle::from_section(
                "🎰 BIG WIN! 🎰",
                TextStyle {
                    font_size: 72.0,
                    color: Color::srgb(0.8, 0.0, 0.0), // Red color
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            }),
            WinText,
        )).id();

        // Win amount text
        let win_text = commands.spawn((
            TextBundle::from_section(
                format!("YOU WON: ${:.2}", popup_state.win_amount),
                TextStyle {
                    font_size: 48.0,
                    color: Color::srgb(0.0, 0.6, 0.0), // Green color
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            }),
            WinText,
        )).id();

        // Multiplier text
        let multiplier_text = commands.spawn((
            TextBundle::from_section(
                format!("{}x MULTIPLIER!", popup_state.multiplier as u32),
                TextStyle {
                    font_size: 36.0,
                    color: Color::srgb(0.6, 0.0, 0.8), // Purple color
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            }),
            MultiplierText,
        )).id();

        // Congratulations text
        let congrats_text = commands.spawn((
            TextBundle::from_section(
                "🎉 CONGRATULATIONS! 🎉",
                TextStyle {
                    font_size: 32.0,
                    color: Color::srgb(0.0, 0.0, 0.8), // Blue color
                    ..default()
                },
            ),
            WinText,
        )).id();

        // Build the hierarchy
        commands.entity(celebration_container).push_children(&[
            big_win_text,
            win_text,
            multiplier_text,
            congrats_text,
        ]);
        
        commands.entity(popup_entity).add_child(celebration_container);

        println!("🎰 Created win popup! Amount: ${:.2}, Multiplier: {:.1}x", popup_state.win_amount, popup_state.multiplier);
    }
}

pub fn animate_coin_flood(
    mut commands: Commands,
    time: Res<Time>,
    mut popup_state: ResMut<WinPopupState>,
    mut coin_query: Query<(Entity, &mut Transform, &mut CoinAnimation)>,
    ui_assets: Option<Res<crate::ui::assets::UIAssets>>,
) {
    if !popup_state.is_active {
        return;
    }

    popup_state.coin_spawn_timer.tick(time.delta());

    // Spawn all coins instantly when popup becomes active
    if popup_state.coins_spawned == 0 {
        println!("🪙 Spawning {} coins instantly!", popup_state.max_coins);
        let mut rng = rand::thread_rng();
        
        for _ in 0..popup_state.max_coins {
            // Random position across the entire screen with wider coverage
            let x_pos = rng.gen_range(-450.0..450.0); // Wider X coverage
            let y_pos = rng.gen_range(-350.0..350.0); // Wider Y coverage

            // Create coin entity with appropriate styling
            let _coin_entity = if let Some(assets) = &ui_assets {
                // Use actual coin image if available
                commands.spawn((
                    ImageBundle {
                        image: UiImage::new(assets.coin.clone()),
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Px(200.0),
                            height: Val::Px(200.0),
                            left: Val::Px(x_pos + 400.0), // Offset to center
                            top: Val::Px(y_pos + 300.0),  // Offset to center
                            ..default()
                        },
                        z_index: ZIndex::Local(90),
                        ..default()
                    },
                    CoinAnimation {
                        start_time: time.elapsed_seconds(),
                        duration: 4.0, // Static display duration
                        start_pos: Vec3::new(x_pos, y_pos, 0.0),
                        end_pos: Vec3::new(x_pos, y_pos, 0.0), // No movement, just appear
                        arc_height: 0.0, // No arc needed
                    },
                )).id()
            } else {
                // Fallback to gold circle if coin asset not loaded
                commands.spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
                            left: Val::Px(x_pos + 400.0), // Offset to center
                            top: Val::Px(y_pos + 300.0),  // Offset to center
                            ..default()
                        },
                        background_color: Color::srgb(1.0, 0.8, 0.0).into(), // Gold color
                        border_radius: BorderRadius::all(Val::Percent(50.0)), // Make it circular
                        z_index: ZIndex::Local(90),
                        ..default()
                    },
                    CoinAnimation {
                        start_time: time.elapsed_seconds(),
                        duration: 4.0, // Static display duration
                        start_pos: Vec3::new(x_pos, y_pos, 0.0),
                        end_pos: Vec3::new(x_pos, y_pos, 0.0), // No movement, just appear
                        arc_height: 0.0, // No arc needed
                    },
                )).id()
            };
        }
        
        popup_state.coins_spawned = popup_state.max_coins; // Mark all as spawned
        println!("🪙 All {} coins spawned instantly!", popup_state.max_coins);
    }

    // Update existing coin animations
    for (entity, mut transform, coin_anim) in &mut coin_query {
        let elapsed = time.elapsed_seconds() - coin_anim.start_time;
        let progress = (elapsed / coin_anim.duration).clamp(0.0, 1.0);

        if progress >= 1.0 {
            // Remove coin when animation is complete
            commands.entity(entity).despawn();
        } else {
            // Calculate position with arc trajectory
            let linear_x = coin_anim.start_pos.x + (coin_anim.end_pos.x - coin_anim.start_pos.x) * progress;
            let linear_y = coin_anim.start_pos.y + (coin_anim.end_pos.y - coin_anim.start_pos.y) * progress;
            
            // Add parabolic arc
            let arc_offset = coin_anim.arc_height * (progress * (1.0 - progress)) * 4.0; // Parabolic curve
            let final_y = linear_y + arc_offset;

            // Update transform
            transform.translation.x = linear_x;
            transform.translation.y = final_y;

            // Add some rotation for coin effect
            transform.rotate_z(time.delta_seconds() * 5.0);
        }
    }
}

pub fn update_win_popup(
    mut commands: Commands,
    time: Res<Time>,
    mut popup_state: ResMut<WinPopupState>,
    popup_query: Query<Entity, With<WinPopup>>,
    coin_query: Query<Entity, With<CoinAnimation>>,
) {
    if !popup_state.is_active {
        return;
    }

    popup_state.popup_timer.tick(time.delta());

    // Close popup after timer expires
    if popup_state.popup_timer.finished() {
        popup_state.is_active = false;
        popup_state.coins_spawned = 0;

        // Remove all popup elements
        for entity in &popup_query {
            commands.entity(entity).despawn_recursive();
        }

        // Remove all remaining coins
        for entity in &coin_query {
            commands.entity(entity).despawn();
        }

        println!("🎰 Win popup closed!");
    }
}