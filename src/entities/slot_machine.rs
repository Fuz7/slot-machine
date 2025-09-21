use rand::prelude::*;
use rand::distributions::WeightedIndex;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub icon: &'static str,
    pub name: &'static str,
    pub multiplier: f32,
    pub addition: f32,
    pub chance: f32,
}

impl Symbol {
    pub fn new(icon: &'static str, name: &'static str, multiplier: f32, addition: f32, chance: f32) -> Self {
        Self { icon, name, multiplier, addition, chance }
    }
}

#[derive(Debug)]
pub struct Reel {
    symbols: Vec<Symbol>,
    dist: WeightedIndex<f32>,
}

impl Reel {
    pub fn new(symbols: Vec<Symbol>) -> Self {
        let weights = symbols.iter().map(|s| s.chance).collect::<Vec<_>>();
        let dist = WeightedIndex::new(weights).unwrap();
        Self { symbols, dist }
    }

    pub fn spin(&self) -> &Symbol {
        let mut rng = thread_rng();
        &self.symbols[self.dist.sample(&mut rng)]
    }
}

#[derive(Debug)]
pub struct SlotMachine {
    reels: Vec<Reel>,
}

impl SlotMachine {
    pub fn new(reels: Vec<Reel>) -> Self {
        Self { reels }
    }

    pub fn spin_grid(&self, rows: usize) -> Vec<Vec<&Symbol>> {
        (0..rows)
            .map(|_| self.reels.iter().map(|reel| reel.spin()).collect())
            .collect()
    }

    pub fn check_wins<'a>(&self, grid: &'a [Vec<&'a Symbol>]) -> Vec<WinningLine<'a>> {
        let mut wins = Vec::new();
        let rows = grid.len();
        let cols = grid[0].len();

        // Horizontals
        for row in 0..rows {
            if grid[row].iter().all(|&s| s.name == grid[row][0].name) {
                wins.push(WinningLine {
                    symbols: grid[row].clone(),
                    line_type: LineType::Horizontal(row),
                });
            }
        }

        // Verticals
        for col in 0..cols {
            let column: Vec<&Symbol> = (0..rows).map(|row| grid[row][col]).collect();
            if column.iter().all(|&s| s.name == column[0].name) {
                wins.push(WinningLine {
                    symbols: column,
                    line_type: LineType::Vertical(col),
                });
            }
        }

        // Diagonals
        let diag1: Vec<&Symbol> = (0..rows).map(|i| grid[i][i]).collect();
        if diag1.iter().all(|&s| s.name == diag1[0].name) {
            wins.push(WinningLine {
                symbols: diag1,
                line_type: LineType::Diagonal(0),
            });
        }

        let diag2: Vec<&Symbol> = (0..rows).map(|i| grid[i][cols - 1 - i]).collect();
        if diag2.iter().all(|&s| s.name == diag2[0].name) {
            wins.push(WinningLine {
                symbols: diag2,
                line_type: LineType::Diagonal(1),
            });
        }

        wins
    }
}

#[derive(Debug)]
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