use rand::prelude::*;
use rand::distributions::WeightedIndex;
use bevy::prelude::Resource;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub icon: String,
    pub name: String,
    pub multiplier: f32,
    pub addition: f32,
    pub chance: f32,
}

impl Symbol {
    pub fn new(icon: &str, name: &str, multiplier: f32, addition: f32, chance: f32) -> Self {
        Self { 
            icon: icon.to_string(), 
            name: name.to_string(), 
            multiplier, 
            addition, 
            chance 
        }
    }
}

#[derive(Debug, Clone)]
pub struct Reel {
    symbols: Vec<Symbol>,
    // Note: WeightedIndex doesn't implement Clone, so we'll recreate it when needed
}

impl Reel {
    pub fn new(symbols: Vec<Symbol>) -> Self {
        Self { symbols }
    }

    pub fn spin(&self) -> Symbol {
        let mut rng = thread_rng();
        let weights = self.symbols.iter().map(|s| s.chance).collect::<Vec<_>>();
        let dist = WeightedIndex::new(weights).unwrap();
        self.symbols[dist.sample(&mut rng)].clone()
    }
}

#[derive(Debug, Clone, Resource)]
pub struct SlotMachine {
    reels: Vec<Reel>,
}

impl SlotMachine {
    pub fn new(reels: Vec<Reel>) -> Self {
        Self { reels }
    }

    pub fn spin_grid(&self, rows: usize) -> Vec<Vec<Symbol>> {
        (0..rows)
            .map(|_| self.reels.iter().map(|reel| reel.spin()).collect())
            .collect()
    }

    pub fn check_wins<'a>(&self, grid: &'a [Vec<Symbol>]) -> Vec<WinningLine<'a>> {
        let mut wins = Vec::new();
        let rows = grid.len();
        let cols = grid[0].len();

        // Horizontals
        for row in 0..rows {
            if grid[row].iter().all(|s| s.name == grid[row][0].name) {
                wins.push(WinningLine {
                    symbols: grid[row].iter().collect(),
                    line_type: LineType::Horizontal(row),
                });
            }
        }

        // Skip vertical wins since reels are circular and this won't happen

        // Diagonals
        let diag1: Vec<&Symbol> = (0..rows).map(|i| &grid[i][i]).collect();
        if diag1.iter().all(|&s| s.name == diag1[0].name) {
            wins.push(WinningLine {
                symbols: diag1,
                line_type: LineType::Diagonal(0),
            });
        }

        let diag2: Vec<&Symbol> = (0..rows).map(|i| &grid[i][cols - 1 - i]).collect();
        if diag2.iter().all(|&s| s.name == diag2[0].name) {
            wins.push(WinningLine {
                symbols: diag2,
                line_type: LineType::Diagonal(1),
            });
        }

        wins
    }
}

#[derive(Debug, Clone)]
pub enum LineType {
    Horizontal(usize),
    Vertical(usize),
    Diagonal(usize), // 0 = top-left→bottom-right, 1 = top-right→bottom-left
}

#[derive(Debug)]
pub struct WinningLine<'a> {
    pub symbols: Vec<&'a Symbol>,
    pub line_type: LineType,
}