use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::constants::USER_STAKE_SEED;
use crate::error::ErrorCode;
use crate::state::StakeNFT;

pub fn stake(ctx: Context<Holders>) -> Result<()> {
    token::transfer(ctx.accounts.nft_to_pda_ctx(), 1)?;
    let clock: Clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp as u64;

    ctx.accounts.user_stake_info.start_time = current_timestamp;
    Ok(())
}

pub fn redeem(ctx: Context<Holders>) -> Result<()> {
    let clock: Clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp as u64;
    ctx.accounts.user_stake_info.last_redeem = current_timestamp;
    Ok(())
}

pub fn unstake(ctx: Context<Holders>) -> Result<()> {
    let owner_key = ctx.accounts.owner.key();
    let seeds = [
        USER_STAKE_SEED,
        owner_key.as_ref(),
        &[ctx.accounts.user_stake_info.bump],
    ];
    let signer_seed: &[&[&[u8]]; 1] = &[&seeds[..]];
    token::transfer(ctx.accounts.nft_to_user_ctx(signer_seed), 1)?;
    Ok(())
}

pub fn close_stake(_ctx: Context<CloseHolders>) -> Result<()> {
    Ok(())
}

impl<'info> Holders<'info> {
    fn nft_to_pda_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let cpi_accounts: Transfer = Transfer {
            from: self.user_nft_token_account.to_account_info(),
            to: self.pda_nft_token_account.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(program, cpi_accounts)
    }
    fn nft_to_user_ctx<'a, 'b, 'c>(
        &self,
        signer_seeds: &'a [&'b [&'c [u8]]],
    ) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let cpi_accounts: Transfer = Transfer {
            from: self.pda_nft_token_account.to_account_info(),
            to: self.user_nft_token_account.to_account_info(),
            authority: self.user_stake_info.to_account_info(),
        };
        CpiContext::new_with_signer(program, cpi_accounts, signer_seeds)
    }
}

#[derive(Accounts)]
pub struct Holders<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        seeds=[USER_STAKE_SEED, owner.key().as_ref(), mint.key().as_ref()],
        bump,
        payer = owner,
        space = StakeNFT::LEN
    )]
    pub user_stake_info: Account<'info, StakeNFT>,

    #[account(mut, constraint = mint.supply == 1)]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = user_nft_token_account.mint == mint.key(),
        constraint = user_nft_token_account.amount == 1
    )]
    pub user_nft_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = user_stake_info
    )]
    pub pda_nft_token_account: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseHolders<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        close = owner,
        constraint = user_stake.is_staking == false @ ErrorCode::StillStaking
    )]
    pub user_stake: Account<'info, StakeNFT>,
}
