use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    exp: u32,
    revive: u32,
    highscore: u32, 
}

impl Player {

    pub fn new(exp: u32, revive: u32, highscore: u32) -> Self {
        Self { exp, revive, highscore }
    }

    pub fn set_exp(&mut self, value: u32) {
        self.exp = value;
    }

    pub fn set_revive(&mut self, value: u32) {
        self.revive = value;
    }

    pub fn set_highscore(&mut self, value: u32) {
        self.highscore = value;
    }

    pub fn exp(&self) -> u32 {
        self.exp
    }

    pub fn revive(&self) -> u32 {
        self.revive
    }

    pub fn highscore(&self) -> u32 {
        self.highscore
    }

}
