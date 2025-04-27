use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub light_forces: Option<Pubkey>,
    pub dark_forces: Option<Pubkey>,
    pub status: GameStatus,
}

impl Game {
    pub const LEN: usize = 1 + 32 + 32 + 1; // status + 2 pubkeys + winner id
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameStatus {
    Init,
    InProgress,
    Finished(u8), // Winner id
} 