use anchor_lang::prelude::*;
use crate::state::game::{Game, GameStatus};
use crate::state::Village;

#[derive(Accounts)]
pub struct InitGame<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Game::LEN,
        seeds = [b"game", payer.key().as_ref()],
        bump
    )]
    pub game: Account<'info, Game>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_game_internal(ctx: Context<InitGame>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    
    game.light_forces = Village::default();
    game.dark_forces = Village::default();
    game.status = GameStatus::InProgress;
    
    Ok(())
} 