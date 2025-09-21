mod util;
mod entities;
mod core;

use entities::slot_machine::{Symbol, Reel, SlotMachine, LineType};
use core::scoring::*;


fn main() {
    let symbols = vec![
        Symbol::new("🍒", "Cherry", 2.0, 0.0, 50.0),
        Symbol::new("🍋", "Lemon", 3.0, 0.0, 30.0),
        Symbol::new("🔔", "Bell", 5.0, 0.0, 15.0),
        Symbol::new("⭐", "Star", 10.0, 0.0, 4.0),
        Symbol::new("7️⃣", "Seven", 20.0, 0.0, 1.0),
    ];

    let slot_machine = SlotMachine::new(vec![
        Reel::new(symbols.clone()),
        Reel::new(symbols.clone()),
        Reel::new(symbols.clone()),
    ]);

    // temp pool and bet
    let mut pool: f32 = 100.0; 
    let bet: f32 = 5.0;

    let grid = slot_machine.spin_grid(3);

    println!("Slot Result:");
    for row in &grid {
        for symbol in row {
            print!("{} ", symbol.icon);
        }
        println!();
    }

    let wins = slot_machine.check_wins(&grid);

    if wins.is_empty() {
        println!("No winning lines 😢");
    } else {
        println!("🎉 Winning lines and payouts:");
        for win in &wins {
            let icons: Vec<&str> = win.symbols.iter().map(|s| s.icon).collect();
            let payout = line_payout(win, bet);
            match win.line_type {
                LineType::Horizontal(row) => println!("Horizontal row {}: {:?} → payout: {}", row, icons, payout),
                LineType::Vertical(col) => println!("Vertical col {}: {:?} → payout: {}", col, icons, payout),
                LineType::Diagonal(di) => println!("Diagonal {}: {:?} → payout: {}", di, icons, payout),
            }
        }
        update_pool(&mut pool, &wins, bet);
    }

    println!("Player's pool after spin: {}", pool);
}
