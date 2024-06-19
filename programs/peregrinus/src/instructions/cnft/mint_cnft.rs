use crate::error::ErrorCode;
use crate::constants::CNFT_CONFIG_SEED;
use crate::state::CnftConfig;
use crate::MplBubblegum;
use crate::Noop;
use crate::SplAccountCompression;

use anchor_lang::prelude::*;
use mpl_bubblegum::{
    instructions::MintV1CpiBuilder,
    types::{Creator, MetadataArgs, TokenProgramVersion, TokenStandard},
};

pub fn mint_cnft(ctx: Context<MintcNFT>, params: MintcNFTParams) -> Result<()> {
    require!(ctx.accounts.config.empty_leaf > 0, ErrorCode::NoEmptyLeaf);

    // TODO: Verify if the signature has permission to do this.
    MintV1CpiBuilder::new(&ctx.accounts.mpl_bubblegum_program.to_account_info())
        .tree_config(&ctx.accounts.tree_config.to_account_info())
        .leaf_owner(&ctx.accounts.user.to_account_info())
        .leaf_delegate(&ctx.accounts.user.to_account_info())
        .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
        .payer(&ctx.accounts.user.to_account_info())
        .tree_creator_or_delegate(&ctx.accounts.config.to_account_info())
        .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
        .compression_program(&ctx.accounts.compression_program.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .metadata(MetadataArgs {
            name: format!("{}_{}", "", ""),
            symbol: "".to_string(),
            uri: params.uri,
            creators: [Creator {
                address: ctx.accounts.user.key(),
                verified: false,
                share: 100,
            }]
            .to_vec(),
            seller_fee_basis_points: 0,
            primary_sale_happened: false,
            is_mutable: false,
            edition_nonce: Some(0),
            uses: None,
            collection: None,
            token_program_version: TokenProgramVersion::Original,
            token_standard: Some(TokenStandard::NonFungible),
        })
        .invoke_signed(&[&[
            CNFT_CONFIG_SEED,
            ctx.accounts.tree_config.key().as_ref(),
            &[ctx.bumps.config],
        ]])?;

    ctx.accounts.config.empty_leaf -= 1;
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MintcNFTParams {
    uri: String,
}

#[derive(Accounts)]
#[instruction(params: MintcNFTParams)]
pub struct MintcNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[
			CNFT_CONFIG_SEED,
			tree_config.key().as_ref(),
		],
        bump
    )]
    pub config: Account<'info, CnftConfig>,

    /// CHECK
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,

    /// CHECK: merkle tree is safe
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub mpl_bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
}
