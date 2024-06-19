use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct NftConfig {
    // base
    pub treasury: Pubkey,
    #[max_len(10)] // limited to 10 bytes
    pub name: String,
    #[max_len(10)] // limited to 10 bytes
    pub symbol: String,
    #[max_len(200)] // limited to 200 bytes
    pub base_token_uri: String,
    pub seller_fee_basis_points: u16,
    pub authority: Pubkey,
    pub total_supply: u16,
    // collection
    pub collection_mint: Pubkey,
    pub collection_bump: u8,
    // whitelist
    pub wl_root: [u8; 32],
    pub wl_limit: u8,

    // switch
    pub blind_box_enable: bool,
    pub cross_chain_enable: bool,
    pub stake_enable: bool,

    // price
    pub mint_price: u64,

    pub bump: u8,
}

impl NftConfig {
    pub const LEN: usize = 8 + NftConfig::INIT_SPACE;
}
