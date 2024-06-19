use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CnftConfig {
    pub tree_config: Pubkey,
    pub verifier: Pubkey,
    pub max_supply: u64,
    pub empty_leaf: u64,
    pub bump: u8,
}

// https://github.com/solana-labs/solana-program-library/blob/d4bbd51b5167d3f0c8a247b5f304a92e6482cd6f/account-compression/sdk/src/constants/index.ts#L16-L97
impl CnftConfig {
    pub const LEN: usize = 8 + CnftConfig::INIT_SPACE;
    pub const MAX_TREE_DEPTH: u32 = 24;
    pub const MAX_TREE_BUFFER_SIZE: u32 = 64;
}
