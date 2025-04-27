use anchor_lang::prelude::*;
use crate::state::game::{Game, GameStatus};
use crate::state::village::Village;
use crate::errors::GameError;

#[derive(Accounts)]
pub struct InitGame<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Game::LEN
    )]
    pub game: Account<'info, Game>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + Village::LEN
    )]
    pub light_forces: Account<'info, Village>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + Village::LEN
    )]
    pub dark_forces: Account<'info, Village>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_game(ctx: Context<InitGame>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    
    require!(game.status == GameStatus::Init, GameError::StatusIsNotInit);
    
    game.status = GameStatus::InProgress;
    game.dark_forces = Some(ctx.accounts.dark_forces.key());
    game.light_forces = Some(ctx.accounts.light_forces.key());
    
    Ok(())
} 