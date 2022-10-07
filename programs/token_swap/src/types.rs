use anchor_lang::prelude::*;

pub const POOL_OWNER_SEEDS: &[u8; 10] = b"pool_owner";

#[event]
pub struct SwapCompleted {
    pub token1: u64,
    pub token2: u64,
    pub user: Pubkey,
}

#[event]
pub struct SwapFailed {
    pub user: Pubkey,
}

#[error_code]
pub enum SwapError {
    #[msg("You don't have enough tokens to swap")]
    UserNotHaveEnoughTokens,
    #[msg("not enough tokens in pool to complete the swap")]
    NotEnoughTokensInPool,
}