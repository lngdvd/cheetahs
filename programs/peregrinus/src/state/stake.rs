use anchor_lang::prelude::*;

//  each nft has a UserStake account
#[account]
#[derive(InitSpace)]
pub struct StakeNFT {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub start_time: u64,
    pub last_redeem: u64,
    pub is_staking: bool,
    pub bump: u8,
}

impl StakeNFT {
    pub const LEN: usize = 8 + StakeNFT::INIT_SPACE;
}
