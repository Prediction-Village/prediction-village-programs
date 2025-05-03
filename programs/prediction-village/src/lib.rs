use anchor_lang::prelude::*;
mod state;
mod errors;
mod utils;
mod instructions;

use instructions::*;

declare_id!("7DmaWof2zqAwJXnBWyFrpQa4dXUkGVaB5WqSj5QpobaK");

#[program]
pub mod prediction_village {
    use super::*;

    pub fn init_game(ctx: Context<InitGame>) -> anchor_lang::Result<()> {
        instructions::init_game::init_game_internal(ctx)
    }

    pub fn generate_income(ctx: Context<GenerateIncome>) -> anchor_lang::Result<()> {
        instructions::generate_income::generate_income_internal(ctx)
    }

    pub fn restart_game(ctx: Context<RestartGame>) -> anchor_lang::Result<()> {
        instructions::restart_game::restart_game_internal(ctx)
    }
}