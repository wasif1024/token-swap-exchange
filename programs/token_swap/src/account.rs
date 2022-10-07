use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::types::POOL_OWNER_SEEDS;

#[account]
pub struct PoolAccount {
    pub rate: u64,            // 8
    pub token1_mint: Pubkey,  // 32
    pub token1_pool: Pubkey,  // 32
    pub token2_mint: Pubkey,  // 32
    pub token2_pool: Pubkey,  // 32
    pub pool_creator: Pubkey, // 32
    pub pool_owner: Pubkey,   // 32
}

#[derive(Accounts)]
#[instruction(seeds_one: [u8; 8], seeds_two: [u8; 8], seeds_three: [u8; 8], seeds_four: [u8; 8])]
pub struct CreatePool<'info> {
    #[account(init,payer=payer,seeds = [&seeds_one], bump, space=8+8+(6*32))]
    pub pool: Account<'info, PoolAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token1_mint: Account<'info, Mint>,

    #[account(init, payer=payer, seeds = [&seeds_two], bump, token::mint = token1_mint, token::authority = pool_owner)]
    pub token1_pool: Account<'info, TokenAccount>,

    pub token2_mint: Account<'info, Mint>,

    #[account(init, payer=payer, seeds = [&seeds_three], bump, token::mint = token2_mint, token::authority = pool_owner)]
    pub token2_pool: Account<'info, TokenAccount>,

    /// CHECK: none
    /// If I do seeds = [&seeds_four], it throws - A seeds constraint was violated error
    /// but If i do seeds = [b"pool_owner".as_ref()] It runs fine
    #[account(seeds = [POOL_OWNER_SEEDS.as_ref()], bump)]
    pub pool_owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

   
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub pool: Account<'info, PoolAccount>,

    #[account(mut,
        constraint=user_token1.owner==user.key(),
        constraint=user_token1.mint==pool.token1_mint,
    )]
    // this is the token account where they would receive token1 from the token1_pool (If they swap token2 for token1)
    pub user_token1: Account<'info, TokenAccount>,

    #[account(mut,
        constraint=user_token2.owner==user.key(),
        constraint=user_token2.mint==pool.token2_mint,
    )]
    // this is the token account where they would receive token2 from the token2_pool (If they swap token1 for token2)
    pub user_token2: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token1_pool: Account<'info, TokenAccount>,

    #[account[mut]]
    pub token2_pool: Account<'info, TokenAccount>,

    /// CHECK: none
    // for some reason using b"pool_owner".as_ref() for seeds causes
    // Could not create program address with signer seeds: Provided seeds do not result in a valid address
    // later when program is invoked
    // thus have to get the seeds of "pool_owner" from client
    #[account(mut, seeds = [POOL_OWNER_SEEDS.as_ref()], bump, constraint = pool_owner.key() == pool.pool_owner)]
    pub pool_owner: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

impl<'info> SwapToken<'info> {
    pub fn transfer_token(
        &self,
        from: Account<'info, TokenAccount>,
        to: Account<'info, TokenAccount>,
        authority: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: from.to_account_info().clone(),
                to: to.to_account_info().clone(),
                authority: authority.clone(),
            },
        )
    }
}