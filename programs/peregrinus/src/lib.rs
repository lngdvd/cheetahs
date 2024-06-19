pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("CMY3WwkCxDL1qshDuTQNv4J9twQYwLfHmjQEsm9K7DeP");

#[program]
pub mod peregrinus {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    // ********************* Metaplex NFT *********************
    pub fn stake_nft(ctx: Context<Holders>) -> Result<()> {
        instructions::staking_pool::stake(ctx)
    }

    pub fn redeem(ctx: Context<Holders>) -> Result<()> {
        instructions::staking_pool::redeem(ctx)
    }

    pub fn unstake_nft(ctx: Context<Holders>) -> Result<()> {
        instructions::staking_pool::unstake(ctx)
    }

    pub fn close_staking(ctx: Context<CloseHolders>) -> Result<()> {
        instructions::staking_pool::close_stake(ctx)
    }

    pub fn mint_sbt(ctx: Context<Tokens2022>) -> Result<()> {
        instructions::tokens2022::mint_sbt(ctx)
    }

    // ********************* cNFT *********************
}
