use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use crate::state::game::{Game, GameStatus};
use crate::state::village::Village;
use crate::errors::GameError;
use crate::utils::generate_random_number;

#[derive(Accounts)]
pub struct ProcessIncome<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    
    #[account(mut)]
    pub light_forces: Account<'info, Village>,
    
    #[account(mut)]
    pub dark_forces: Account<'info, Village>,
}

pub fn process_income(ctx: Context<ProcessIncome>) -> Result<()> {
    let game = &ctx.accounts.game;
    let light_forces = &mut ctx.accounts.light_forces;
    let dark_forces = &mut ctx.accounts.dark_forces;
    
    require!(game.status == GameStatus::InProgress, GameError::StatusIsNotInProgress);
    require!(
        light_forces.key() == game.light_forces.expect("Light forces key not set"),
        GameError::LightForcesKeyMismatch
    );
    require!(
        dark_forces.key() == game.dark_forces.expect("Dark forces key not set"),
        GameError::DarkForcesKeyMismatch
    );

    let slot = Clock::get()?.slot;
    let rnd1 = generate_random_number(slot);
    let rnd2 = generate_random_number(slot + 1);

    light_forces.resources.gold = light_forces.resources.gold.checked_add(
        (light_forces.resources.gold_income as f32 * rnd1) as u32
    ).unwrap();
    dark_forces.resources.gold = dark_forces.resources.gold.checked_add(
        (dark_forces.resources.gold_income as f32 * rnd2) as u32
    ).unwrap();

    light_forces.resources.technologies = light_forces.resources.technologies.checked_add(
        (light_forces.resources.technologies_income as f32 * rnd1) as u32
    ).unwrap();
    dark_forces.resources.technologies = dark_forces.resources.technologies.checked_add(
        (dark_forces.resources.technologies_income as f32 * rnd2) as u32
    ).unwrap();

    Ok(())
} 