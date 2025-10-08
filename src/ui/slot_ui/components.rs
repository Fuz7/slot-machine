use bevy::prelude::*;
use crate::entities::slot_machine::{Symbol, SlotMachine, Reel};

/// Main game state resource
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

impl Default for GameState {
    fn default() -> Self {
        // Create default symbols
        let symbols = vec![
            Symbol::new("🍒", "Cherry", 2.0, 0.0, 50.0),
            Symbol::new("🍋", "Lemon", 3.0, 0.0, 30.0),
            Symbol::new("🔔", "Bell", 5.0, 0.0, 15.0),
            Symbol::new("⭐", "Star", 10.0, 0.0, 4.0),
            Symbol::new("7️⃣", "Seven", 20.0, 0.0, 1.0),
        ];
        
        // Create 3 identical reels
        let reels = vec![
            Reel::new(symbols.clone()),
            Reel::new(symbols.clone()),
            Reel::new(symbols),
        ];
        
        Self {
            slot_machine: SlotMachine::new(reels),
            player_pool: 100.0,
            current_bet: 5.0,
            last_grid: None,
            last_wins: Vec::new(),
            is_spinning: false,
            last_win_amount: 0.0,
            has_recent_win: false,
        }
    }
}

/// Simplified winning line for UI purposes
#[derive(Clone)]
pub struct SimpleWinningLine {
    pub symbols: Vec<Symbol>,
    pub line_type: crate::entities::slot_machine::LineType,
}

/// Bloom animation phases
#[derive(Clone, PartialEq)]
pub enum BloomPhase {
    WaitingToStart,
    SequentialBloom,
    RapidFlashing,
    Finished,
}

/// Win bloom state resource
#[derive(Resource)]
pub struct WinBloomState {
    pub is_active: bool,
    pub current_line: usize,
    pub current_cell: usize,
    pub flash_timer: Timer,
    pub flash_count: u32,
    pub max_flashes: u32,
    pub all_cells_ready: bool,
}

impl Default for WinBloomState {
    fn default() -> Self {
        Self {
            is_active: false,
            current_line: 0,
            current_cell: 0,
            flash_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            flash_count: 0,
            max_flashes: 5,
            all_cells_ready: false,
        }
    }
}

/// Win popup state resource
#[derive(Resource)]
pub struct WinPopupState {
    pub is_active: bool,
    pub win_amount: f32,
    pub multiplier: f32,
    pub popup_timer: Timer,
    pub coin_spawn_timer: Timer,
    pub coins_spawned: u32,
    pub max_coins: u32,
    pub pending_win: Option<(f32, f32)>, // (win_amount, multiplier) - will show popup after bloom finishes
}

impl Default for WinPopupState {
    fn default() -> Self {
        Self {
            is_active: false,
            win_amount: 0.0,
            multiplier: 1.0,
            popup_timer: Timer::from_seconds(3.0, TimerMode::Once),
            coin_spawn_timer: Timer::from_seconds(0.01, TimerMode::Repeating),
            coins_spawned: 0,
            max_coins: 150,
            pending_win: None,
        }
    }
}

// UI Component markers
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

#[derive(Component)]
pub struct WinPopup;

#[derive(Component)]
pub struct CoinAnimation {
    pub start_time: f32,
    pub duration: f32,
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    pub arc_height: f32,
}

#[derive(Component)]
pub struct WinText;

#[derive(Component)]
pub struct MultiplierText;

#[derive(Component)]
pub struct CelebrationOverlay;