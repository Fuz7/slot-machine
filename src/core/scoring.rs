use crate::entities::slot_machine::{WinningLine, LineType, Symbol};

pub fn line_payout(line: &WinningLine, bet: f32) -> f32 {
    let s: &Symbol = line.symbols[0];
    s.multiplier * bet + s.addition
}

pub fn total_payout(wins: &[WinningLine], bet: f32) -> f32 {
    wins.iter().map(|line| line_payout(line, bet)).sum()
}

pub fn update_pool(current_pool: &mut f32, wins: &[WinningLine], bet: f32) {
    let payout = total_payout(wins, bet);
    *current_pool += payout;
}