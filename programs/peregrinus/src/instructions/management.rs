use crate::constants::NFT_CONFIG_SEED;
use crate::error::ErrorCode;
use crate::state::NftConfig;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Change<'info> {
    #[account(mut)]
    authority: Signer<'info>,

    #[account(
        mut,
        seeds = [NFT_CONFIG_SEED],
        has_one = authority @ ErrorCode::IsNotAuthority,
        bump
    )]
    config: Account<'info, NftConfig>,

    /// CHECK: new authority
    #[account(mut)]
    new_authority: AccountInfo<'info>,
}
