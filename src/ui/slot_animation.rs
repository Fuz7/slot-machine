use bevy::prelude::*;
use crate::entities::slot_machine::{Symbol, SlotMachine};
use crate::ui::slot_ui::{GameState, SlotCell, SimpleWinningLine};
use rand::Rng;

#[derive(Resource)]
pub struct SlotAnimationState {
    pub is_animating: bool,
    pub current_spinning_reel: Option<usize>,
    pub columns: Vec<SlotColumn>,
    pub completed_reels: Vec<bool>,
    pub results: Vec<Vec<Symbol>>,
    pub final_symbols: Vec<Vec<Symbol>>,
    pub target_results: Vec<Vec<Symbol>>,
    pub animation_timer: Timer,
    pub deceleration_factor: f32,
}

impl Default for SlotAnimationState {
    fn default() -> Self {
        Self {
            is_animating: false,
            current_spinning_reel: None,
            columns: Vec::new(),
            completed_reels: vec![false; 3],
            results: vec![Vec::new(); 3],
            final_symbols: vec![Vec::new(); 3],
            target_results: vec![Vec::new(); 3],
            animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            deceleration_factor: 0.98,
        }
    }
}

#[derive(Component, Clone)]
pub struct SlotColumn {
    pub reel_symbols: Vec<Symbol>,
    pub current_offset: f32,
    pub target_offset: f32,
    pub spin_speed: f32,
    pub is_spinning: bool,
    pub symbol_height: f32,
    pub column: usize,
}

impl SlotColumn {
    pub fn new(symbols: Vec<Symbol>, column_index: usize) -> Self {
        Self {
            reel_symbols: symbols,
            current_offset: 0.0,
            target_offset: 0.0,
            spin_speed: 200.0,
            is_spinning: false,
            symbol_height: 100.0,
            column: column_index,
        }
    }
}

#[derive(Resource)]
pub struct TargetResults {
    pub grid: Vec<Vec<Symbol>>,
}

impl TargetResults {
    pub fn new(grid: Vec<Vec<Symbol>>) -> Self {
        Self { grid }
    }
    
    pub fn get(&self, column_index: usize) -> Option<&Vec<Symbol>> {
        self.grid.get(column_index)
    }
}

pub fn generate_circular_reel(base_symbols: &[Symbol], reel_length: usize) -> Vec<Symbol> {
    let mut reel = Vec::new();
    
    for i in 0..reel_length {
        let symbol_index = i % base_symbols.len();
        reel.push(base_symbols[symbol_index].clone());
    }
    
    reel
}

pub fn start_slot_animation(
    mut animation_state: ResMut<SlotAnimationState>,
    mut game_state: ResMut<GameState>,
    slot_machine: Res<SlotMachine>,
    mut commands: Commands,
) {
    if animation_state.is_animating {
        return;
    }

    println!("Starting slot animation...");
    
    // Generate final results directly as columns (3 symbols per column)
    let base_symbols = vec![
        Symbol::new("🍒", "Cherry", 2.0, 0.0, 50.0),
        Symbol::new("🍋", "Lemon", 3.0, 0.0, 30.0),
        Symbol::new("🔔", "Bell", 5.0, 0.0, 15.0),
        Symbol::new("⭐", "Star", 10.0, 0.0, 4.0),
        Symbol::new("7️⃣", "Seven", 50.0, 0.0, 1.0),
    ];
    
    // Generate random results ensuring no duplicates in any column (reel)
    let mut target_columns = Vec::new();
    let mut rng = rand::thread_rng();
    
    // Generate column by column to ensure no duplicates in vertical lines (reels)
    for _col_idx in 0..3 {
        let mut available_symbols = base_symbols.clone();
        let mut column_symbols = Vec::new();
        
        for _row in 0..3 {
            // Randomly select from remaining available symbols for this column
            let random_index = rng.gen_range(0..available_symbols.len());
            let selected_symbol = available_symbols.remove(random_index);
            column_symbols.push(selected_symbol);
        }
        
        target_columns.push(column_symbols);
    }

    println!("Generated final column results:");
    for (col_index, column) in target_columns.iter().enumerate() {
        println!("Column {}: {:?}", col_index, column.iter().map(|s| &s.name).collect::<Vec<_>>());
        println!("  Layout - Top: {}, Middle: {}, Bottom: {}", 
            column[0].name, column[1].name, column[2].name);
    }
    
    // Convert to row format for win checking (this matches what the console will show)
    let mut final_grid = vec![Vec::new(); 3];
    for row_idx in 0..3 {
        for col_idx in 0..3 {
            final_grid[row_idx].push(target_columns[col_idx][row_idx].clone());
        }
    }
    
    println!("Final grid in row format (for win checking):");
    for (row_index, row) in final_grid.iter().enumerate() {
        println!("Row {}: {:?}", row_index, row.iter().map(|s| &s.name).collect::<Vec<_>>());
    }
    
    // Store the final grid for game state
    game_state.last_grid = Some(final_grid.clone());

    // Store the target results
    commands.insert_resource(TargetResults::new(target_columns.clone()));
    
    // Initialize animation state
    animation_state.is_animating = true;
    animation_state.current_spinning_reel = Some(0); // Start with first reel
    animation_state.target_results = target_columns.clone();
    animation_state.results = target_columns; // Set results immediately
    animation_state.completed_reels = vec![false; 3];
    
    // Create circular reels that include our target symbols
    // Each reel will be much longer and circular for smooth animation
    let reel_length = 50; // Much longer reel for better circular effect
    
    // Initialize or reset columns
    if animation_state.columns.is_empty() {
        for i in 0..3 {
            // Create a long circular reel with repeating base symbols
            let reel_symbols = generate_circular_reel(&base_symbols, reel_length);
            let mut column = SlotColumn::new(reel_symbols, i);
            
            // Set animation parameters for circular motion
            column.current_offset = 0.0;
            column.target_offset = (column.symbol_height * 3.0) + (i as f32 * 10.0); // Minimal rotation for 1s
            column.spin_speed = 500.0 + (i as f32 * 50.0); // Optimized speeds: 400, 450, 500
            animation_state.columns.push(column);
        }
    } else {
        // Reset existing columns for new animation
        for (i, column) in animation_state.columns.iter_mut().enumerate() {
            // Regenerate circular reel
            column.reel_symbols = generate_circular_reel(&base_symbols, reel_length);
            column.current_offset = 0.0;
            column.target_offset = (column.symbol_height * 3.0) + (i as f32 * 10.0);
            column.spin_speed = 500.0 + (i as f32 * 50.0);
            column.is_spinning = false;
        }
    }
    
    // Start the first reel
    if let Some(first_column) = animation_state.columns.get_mut(0) {
        first_column.is_spinning = true;
    }
    
    game_state.is_spinning = true;
    
    animation_state.animation_timer = Timer::from_seconds(0.1, TimerMode::Repeating);
}

pub fn update_slot_animation(
    time: Res<Time>,
    mut animation_state: ResMut<SlotAnimationState>,
    mut game_state: ResMut<GameState>,
    slot_machine: Option<Res<SlotMachine>>,
    target_results: Option<Res<TargetResults>>,
) {
    if !animation_state.is_animating {
        return;
    }

    let slot_machine = match slot_machine {
        Some(sm) => sm,
        None => return,
    };

    let target_results = match target_results {
        Some(tr) => tr,
        None => return,
    };

    // Update the currently spinning reel
    if let Some(current_reel) = animation_state.current_spinning_reel {
        // First, collect the necessary information without borrowing animation_state mutably
        let (should_stop, current_offset, target_offset) = {
            if let Some(column) = animation_state.columns.get(current_reel) {
                let max_offset = column.reel_symbols.len() as f32 * column.symbol_height;
                
                // Check if we should stop - either reached target or very close with low speed
                let distance_to_target = if column.target_offset > column.current_offset {
                    column.target_offset - column.current_offset
                } else {
                    (max_offset - column.current_offset) + column.target_offset
                };
                
                let should_stop = column.is_spinning && 
                    (distance_to_target < 15.0 || column.spin_speed < 120.0);
                
                (should_stop, column.current_offset, column.target_offset)
            } else {
                (false, 0.0, 0.0)
            }
        };

        // Now update the column
        if let Some(column) = animation_state.columns.get_mut(current_reel) {
            if column.is_spinning {
                if !should_stop {
                    // Update spinning animation with proper circular motion
                    column.current_offset += column.spin_speed * time.delta_seconds();
                    
                    // Keep offset within bounds for circular effect
                    let max_offset = column.reel_symbols.len() as f32 * column.symbol_height;
                    if column.current_offset >= max_offset {
                        column.current_offset -= max_offset; // Wrap around for circular effect
                    }
                    
                    // Check if we should start slowing down
                    let remaining_distance = if column.target_offset > column.current_offset {
                        column.target_offset - column.current_offset
                    } else {
                        // Handle wrap-around case
                        (max_offset - column.current_offset) + column.target_offset
                    };
                    
                    // Quick deceleration for 1s timing
                    if remaining_distance < 100.0 && column.spin_speed > 100.0 {
                        // Fast deceleration for 1s target
                        column.spin_speed *= 0.8;
                        
                        if column.spin_speed < 100.0 {
                            column.spin_speed = 100.0;
                        }
                    }
                } else {
                    // Stop the reel
                    column.current_offset = column.target_offset;
                    column.is_spinning = false;
                    column.spin_speed = 0.0;
                }
            }
        }

        // Handle reel completion (after releasing the mutable borrow of column)
        if should_stop {
            // Position the reel to show our pre-determined target symbols
            if let Some(target_column) = target_results.get(current_reel) {
                if let Some(column) = animation_state.columns.get_mut(current_reel) {
                    position_reel_to_show_symbols(column, target_column);
                }
            }
            
            animation_state.completed_reels[current_reel] = true;
            
            // Get the symbols that are now visible (should match our target)
            let visible_symbols = if let Some(column) = animation_state.columns.get(current_reel) {
                get_visible_symbols_for_column(column, 3)
            } else {
                Vec::new()
            };
            
            // Get our expected target symbols for comparison
            let expected_symbols = if let Some(target_column) = target_results.get(current_reel) {
                target_column.clone()
            } else {
                Vec::new()
            };
            
            println!("Reel {} stopped!", current_reel);
            println!("  Expected: Top={}, Middle={}, Bottom={}", 
                expected_symbols.get(0).map(|s| s.name.as_str()).unwrap_or("None"),
                expected_symbols.get(1).map(|s| s.name.as_str()).unwrap_or("None"),
                expected_symbols.get(2).map(|s| s.name.as_str()).unwrap_or("None"));
            println!("  Actually showing: Top={}, Middle={}, Bottom={}", 
                visible_symbols.get(0).map(|s| s.name.as_str()).unwrap_or("None"),
                visible_symbols.get(1).map(|s| s.name.as_str()).unwrap_or("None"),
                visible_symbols.get(2).map(|s| s.name.as_str()).unwrap_or("None"));
            
            // Check if they match
            let matches = expected_symbols.len() == visible_symbols.len() && 
                expected_symbols.iter().zip(visible_symbols.iter())
                    .all(|(expected, actual)| expected.name == actual.name);
            println!("  Symbols match: {}", matches);
            
            // Start the next reel or finish animation
            if current_reel < 2 {
                // Start next reel
                let next_reel = current_reel + 1;
                animation_state.current_spinning_reel = Some(next_reel);
                if let Some(next_column) = animation_state.columns.get_mut(next_reel) {
                    next_column.is_spinning = true;
                    println!("Starting reel {}...", next_reel);
                }
            } else {
                // All reels completed
                animation_state.current_spinning_reel = None;
                animation_state.is_animating = false;
                game_state.is_spinning = false; // Reset the spinning state!
                
                println!("All reels stopped! Final results:");
                for (i, column_result) in animation_state.results.iter().enumerate() {
                    println!("Column {}: {:?}", i, column_result.iter().map(|s| &s.name).collect::<Vec<_>>());
                    if column_result.len() >= 3 {
                        println!("  Layout - Top: {}, Middle: {}, Bottom: {}", 
                            column_result[0].name, column_result[1].name, column_result[2].name);
                    }
                }
                
                // Convert to grid format for win checking
                let mut final_grid = vec![Vec::new(); 3];
                for row_idx in 0..3 {
                    for col_idx in 0..3 {
                        if let Some(column) = animation_state.results.get(col_idx) {
                            if let Some(symbol) = column.get(row_idx) {
                                final_grid[row_idx].push(symbol.clone());
                            }
                        }
                    }
                }
                
                // Check for wins (horizontal and diagonal only - no vertical for circular reels)
                let wins = slot_machine.as_ref().check_wins(&final_grid);
                
                // Store wins in game state for the process_spin_results function
                game_state.last_wins.clear();
                for win in &wins {
                    let simple_line = SimpleWinningLine {
                        symbols: win.symbols.iter().map(|s| (*s).clone()).collect(),
                        line_type: win.line_type.clone(),
                    };
                    game_state.last_wins.push(simple_line);
                }
                
                if !wins.is_empty() {
                    println!("WINS FOUND: {} winning lines!", wins.len());
                    for win in &wins {
                        println!("Win: {:?} - symbols: {:?}", 
                            win.line_type, 
                            win.symbols.iter().map(|s| &s.name).collect::<Vec<_>>()
                        );
                    }
                    
                    // Calculate and add winnings immediately
                    let total_win: f32 = game_state.last_wins.iter()
                        .map(|line| line.symbols[0].multiplier * game_state.current_bet + line.symbols[0].addition)
                        .sum();
                    
                    println!("🎉 WIN! Bet: ${:.2}, Multiplier calculation: ${:.2}", game_state.current_bet, total_win);
                    println!("🎉 Pool before win: ${:.2}", game_state.player_pool);
                    game_state.player_pool += total_win;
                    println!("🎉 Pool after win: ${:.2}", game_state.player_pool);
                    
                    // Store the win amount and mark as recent win
                    game_state.last_win_amount = total_win;
                    game_state.has_recent_win = true;
                } else {
                    println!("❌ No wins this time. Pool remains: ${:.2}", game_state.player_pool);
                    // Mark that there's no recent win, but keep the last win amount displayed
                    game_state.has_recent_win = false;
                }
            }
        }
    }
}

pub fn get_visible_symbols_for_column(
    column: &SlotColumn,
    visible_count: usize,
) -> Vec<Symbol> {
    // For a truly circular reel, we calculate which symbols are visible based on offset
    let offset_in_symbols = ((column.current_offset / column.symbol_height) as usize) % column.reel_symbols.len();
    let mut visible_symbols = Vec::new();
    
    for i in 0..visible_count {
        let symbol_index = (offset_in_symbols + i) % column.reel_symbols.len();
        visible_symbols.push(column.reel_symbols[symbol_index].clone());
    }
    
    visible_symbols
}

// Helper function to position a reel to show specific target symbols
fn position_reel_to_show_symbols(column: &mut SlotColumn, target_symbols: &[Symbol]) {
    if target_symbols.is_empty() || column.reel_symbols.is_empty() {
        return;
    }
    
    // Find the best position in the existing circular reel to show our target symbols
    let mut best_offset = 0.0;
    let mut best_match_count = 0;
    
    // Search through the circular reel to find where our target symbols appear
    for start_pos in 0..column.reel_symbols.len() {
        let mut match_count = 0;
        
        // Check how many target symbols match at this position
        for (i, target_symbol) in target_symbols.iter().enumerate() {
            if i >= 3 { break; } // Only check first 3 visible symbols
            let reel_index = (start_pos + i) % column.reel_symbols.len();
            if column.reel_symbols[reel_index].name == target_symbol.name {
                match_count += 1;
            }
        }
        
        // If this position has more matches, use it
        if match_count > best_match_count {
            best_match_count = match_count;
            best_offset = start_pos as f32 * column.symbol_height;
        }
        
        // If we found a perfect match, no need to continue
        if match_count == target_symbols.len().min(3) {
            break;
        }
    }
    
    // If we didn't find a good match, modify the reel to include our symbols
    if best_match_count < target_symbols.len().min(3) {
        // Insert our target symbols at a random position in the circular reel
        let mut rng = rand::thread_rng();
        let insert_pos = rng.gen_range(0..column.reel_symbols.len().saturating_sub(3));
        for (i, symbol) in target_symbols.iter().enumerate().take(3) {
            if insert_pos + i < column.reel_symbols.len() {
                column.reel_symbols[insert_pos + i] = symbol.clone();
            }
        }
        best_offset = insert_pos as f32 * column.symbol_height;
    }
    
    // Set the reel position to show our target symbols
    column.current_offset = best_offset;
    column.target_offset = best_offset;
    
    println!("Positioned reel to show target symbols at offset {}. Target symbols: {:?}", 
        best_offset, target_symbols.iter().take(3).map(|s| &s.name).collect::<Vec<_>>());
}