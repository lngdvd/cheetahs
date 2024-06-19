use anchor_lang::prelude::*;

//  each nft has a UserStake account
#[account]
#[derive(InitSpace)]
pub struct Administrator {
    pub manager: Pubkey,
    pub bump: u8,
}

impl Administrator {
    pub const LEN: usize = 8 + Administrator::INIT_SPACE;
}
