use anchor_lang::prelude::*;
use crate::state::game::{Game, GameStatus};
use crate::errors::GameError;
use crate::state::Village;

#[derive(Accounts)]
pub struct RestartGame<'info> {
    #[account(
        mut,
        seeds = [b"game", authority.key().as_ref()],
        bump
    )]
    pub game: Account<'info, Game>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn restart_game_internal(ctx: Context<RestartGame>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    
    if game.status == GameStatus::InProgress {
        return Err(GameError::GameInProgress.into());
    }

    // Reset game state
    game.status = GameStatus::InProgress;
    game.light_forces = Village::default();
    game.dark_forces = Village::default();
    
    Ok(())
} 