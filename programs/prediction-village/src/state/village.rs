use anchor_lang::prelude::*;
use super::warrior::Warrior;

#[account]
pub struct Village {
    pub town_hall: TownHall,
    pub gold_mines: Vec<u8>,
    pub barracks: Vec<u8>,
    pub laboratories: Vec<u8>,
    pub warriors: Vec<Warrior>,
    pub resources: Resources,
}

impl Village {
    pub const LEN: usize = 8 + // discriminator
        1 + 4 + 4 + // TownHall
        4 + (5 * 1) + // gold_mines (max 5)
        4 + (5 * 1) + // barracks (max 5)
        4 + (5 * 1) + // laboratories (max 5)
        4 + (100 * (1 + 4 + 4 + 8)) + // warriors (max 100)
        4 + 4 + 4 + 4; // Resources
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TownHall {
    pub level: u8,
    pub health: u32,
    pub damage: u32,
}

impl Default for TownHall {
    fn default() -> Self {
        Self { level: 1, health: 1000, damage: 10 }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Resources {
    pub gold: u32,
    pub gold_income: u32,
    pub technologies: u32,
    pub technologies_income: u32,
}

impl Default for Resources {
    fn default() -> Self {
        Self { 
            gold: 0, 
            gold_income: 10, 
            technologies: 0, 
            technologies_income: 5, 
        }
    }
} 