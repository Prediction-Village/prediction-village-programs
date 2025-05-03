use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use crate::state::game::{Game, GameStatus};
use crate::errors::GameError;
use crate::utils::generate_random_number;

#[derive(Accounts)]
pub struct GenerateIncome<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
}

pub fn generate_income_internal(ctx: Context<GenerateIncome>) -> Result<()> {
    let game = &mut ctx.accounts.game;

    require!(game.status == GameStatus::InProgress, GameError::StatusIsNotInProgress);

    let slot = Clock::get()?.slot;
    let rnd1 = generate_random_number(slot);
    let rnd2 = generate_random_number(slot + 1);

    game.light_forces.resources.gold = game.light_forces.resources.gold.checked_add(
        (game.light_forces.resources.gold_income as f32 * rnd1) as u32
    ).unwrap();
    game.dark_forces.resources.gold = game.dark_forces.resources.gold.checked_add(
        (game.dark_forces.resources.gold_income as f32 * rnd2) as u32
    ).unwrap();

    game.light_forces.resources.technologies = game.light_forces.resources.technologies.checked_add(
        (game.light_forces.resources.technologies_income as f32 * rnd1) as u32
    ).unwrap();
    game.dark_forces.resources.technologies = game.dark_forces.resources.technologies.checked_add(
        (game.dark_forces.resources.technologies_income as f32 * rnd2) as u32
    ).unwrap();

    Ok(())
} 