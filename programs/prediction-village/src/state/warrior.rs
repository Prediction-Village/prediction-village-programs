use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Warrior {
    pub level: u8,
    pub health: u32,
    pub damage: u32,
    pub position: Position,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
} 