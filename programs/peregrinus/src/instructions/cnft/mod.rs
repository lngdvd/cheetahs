pub mod add_tree;
pub use add_tree::*;

pub mod set_and_verify_collection;
pub use set_and_verify_collection::*;

pub mod mint_cnft;
pub use mint_cnft::*;

// using for compressed nft
use anchor_lang::prelude::*;

#[derive(Clone)]
pub struct MplBubblegum;
impl Id for MplBubblegum {
    fn id() -> Pubkey {
        mpl_bubblegum::ID
    }
}

#[derive(Clone)]
pub struct SplAccountCompression;
impl Id for SplAccountCompression {
    fn id() -> Pubkey {
        spl_account_compression::id()
    }
}

#[derive(Clone)]
pub struct Noop;
impl Id for Noop {
    fn id() -> Pubkey {
        spl_noop::id()
    }
}

// using for stake native sol
// #[derive(Clone)]
// pub struct NativeStake;
// impl Id for NativeStake {
//     fn id() -> Pubkey {
//         anchor_lang::solana_program::stake::program::id()
//     }
// }
