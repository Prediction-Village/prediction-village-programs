use anchor_lang::prelude::*;

use super::Village;

#[account]
pub struct Game {
    pub light_forces: Village,
    pub dark_forces: Village,
    pub status: GameStatus,
}

impl Game {
    pub const LEN: usize = 1 + 32 + 32 + 1 + Village::LEN + Village::LEN; // status + 2 pubkeys + winner id + 2 villages    
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameStatus {
    InProgress,
    Finished(u8), // Winner id
} 

impl Default for GameStatus {
    fn default() -> Self {
        Self::InProgress
    }
}
