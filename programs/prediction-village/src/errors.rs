use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("The game status is not currently set to Init.")]
    StatusIsNotInit,
    #[msg("The game status is not currently set to InProgress.")]
    StatusIsNotInProgress,
    #[msg("The light forces key mismatch.")]
    LightForcesKeyMismatch,
    #[msg("The dark forces key mismatch.")]
    DarkForcesKeyMismatch,
} 