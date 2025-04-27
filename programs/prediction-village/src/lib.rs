use anchor_lang::prelude::*;

mod state;
mod errors;
mod utils;
mod instructions;

declare_id!("7DmaWof2zqAwJXnBWyFrpQa4dXUkGVaB5WqSj5QpobaK");

#[program]
pub mod prediction_village {
    use super::*;

    pub fn init_game(ctx: Context<InitGame>) -> Result<()> {
        init_game(ctx)
    }

    pub fn process_income(ctx: Context<ProcessIncome>) -> Result<()> {
        process_income(ctx)
    }
}