use anchor_lang::prelude::*;
use anchor_spl::token::transfer;

use account::*;
use helper::*;
use types::*;

mod account;
mod helper;
mod types;

declare_id!("3oJwUQXHhgHrSEm9tnMi2WdEAKRkCAevreuiE6JTKgP6");

#[program]
pub mod swap {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        _seeds_one: [u8; 8],
        _seeds_two: [u8; 8],
        _seeds_three: [u8; 8],
        _seeds_four: [u8; 8],
        rate: u64,
    ) -> Result<()> {
        let pool_account = &mut ctx.accounts.pool;
        pool_account.rate = rate;
        pool_account.pool_creator = *ctx.accounts.payer.key;
        pool_account.token1_mint = ctx.accounts.token1_mint.key().clone();
        pool_account.token1_pool = ctx.accounts.token1_pool.key().clone();
        pool_account.token2_mint = ctx.accounts.token2_mint.key().clone();
        pool_account.token2_pool = ctx.accounts.token2_pool.key().clone();
        pool_account.pool_owner = ctx.accounts.pool_owner.key().clone();
        Ok(())
    }

    pub fn swap_token(
        ctx: Context<SwapToken>,
        // remember #[instruction()] order, very easy make a mistake here
        bumpy: u8,
        swap: Swap,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        match swap {
            Swap::Token1ForToken2 { amount } => {
                // check if the user has tokens to swap, why?
                // because user can enter anything in amount field
                // if they doesn't have enough token to swap, they will get an error
                if ctx.accounts.user_token1.amount < amount {
                    emit!(SwapFailed {
                        user: ctx.accounts.user.key()
                    });
                    return err!(SwapError::UserNotHaveEnoughTokens);
                }
                // get rate and how much token2 user will get
                let token2_to_get = swap.how_much_to_get(pool.rate);
                // check if pool has enough token2 to complete the swap or not
                if ctx.accounts.token2_pool.amount < token2_to_get {
                    emit!(SwapFailed {
                        user: ctx.accounts.user.key()
                    });
                    return err!(SwapError::NotEnoughTokensInPool);
                }
                // now transfer token1 from user to the pool
                transfer(
                    ctx.accounts.transfer_token(
                        ctx.accounts.user_token1.clone(),
                        ctx.accounts.token1_pool.clone(),
                        ctx.accounts.user.to_account_info(),
                    ),
                    amount,
                )?;
                // now transfer the token2 from pool to user
                transfer(
                    ctx.accounts
                        .transfer_token(
                            ctx.accounts.token2_pool.clone(),
                            ctx.accounts.user_token2.clone(),
                            ctx.accounts.pool_owner.to_account_info(),
                        )
                        .with_signer(&[&[
                            types::POOL_OWNER_SEEDS,
                            /* undocumented */ &[bumpy],
                        ]]),
                    token2_to_get,
                )?;
                emit!(SwapCompleted {
                    token1: amount,
                    token2: token2_to_get,
                    user: ctx.accounts.user.key(),
                });
                // congrats! swap complete!
            }
            Swap::Token2ForToken1 { amount } => {
                if ctx.accounts.user_token2.amount < amount {
                    emit!(SwapFailed {
                        user: ctx.accounts.user.key()
                    });
                    return err!(SwapError::UserNotHaveEnoughTokens);
                }
                let token1_to_get = swap.how_much_to_get(pool.rate);
                if ctx.accounts.token1_pool.amount < token1_to_get {
                    emit!(SwapFailed {
                        user: ctx.accounts.user.key()
                    });
                    return err!(SwapError::NotEnoughTokensInPool);
                }
                transfer(
                    ctx.accounts.transfer_token(
                        ctx.accounts.user_token2.clone(),
                        ctx.accounts.token2_pool.clone(),
                        ctx.accounts.user.to_account_info(),
                    ),
                    amount,
                )?;
                // now transfer the token2 from pool to user
                transfer(
                    ctx.accounts
                        .transfer_token(
                            ctx.accounts.token1_pool.clone(),
                            ctx.accounts.user_token1.clone(),
                            ctx.accounts.pool_owner.to_account_info(),
                        )
                        .with_signer(&[&[types::POOL_OWNER_SEEDS, &[bumpy]]]),
                    token1_to_get,
                )?;
                emit!(SwapCompleted {
                    token1: token1_to_get,
                    token2: amount,
                    user: ctx.accounts.user.key(),
                });
            }
        }
        Ok(())
    }
}
