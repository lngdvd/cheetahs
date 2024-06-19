// use crate::state::config::Config;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        approve_collection_authority, burn_nft, create_master_edition_v3,
        create_metadata_accounts_v3, freeze_delegated_account,
        mpl_token_metadata::{
            accounts::{MasterEdition, Metadata as MetadataAccount},
            types::{Creator, DataV2},
        },
        set_and_verify_collection, set_and_verify_sized_collection_item, sign_metadata,
        thaw_delegated_account, update_metadata_accounts_v2, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata, SignMetadata, ID as METADATA_PROGRAM_ID,
    },
    token::{initialize_mint2, mint_to, Mint, MintTo, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct MintNFT<'info> {
    /// CHECK:
    #[account(mut)]
    pub signer: Signer<'info>,

    // #[account(
    //     mut,
    //     seeds = [Config::SEED_PREFIX],
    //     bump,
    // )]
    // config: Account<'info, Config>,
    #[account(
		init, // => create_account
		payer = signer,
		mint::decimals = 0,
        mint::authority = signer,
        mint::freeze_authority = signer,
	)]
    pub mint_account: Account<'info, Mint>, // Deprecated anchor_spl::token::initialize_mint;

    #[account(
    	init_if_needed,
    	payer = signer,
    	associated_token::mint = mint_account,
        associated_token::authority = signer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    /// CHECK: address
    // address = MetadataAccount::find_pda(&mint_account.key()).0,
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,

    /// CHECK: address
    // address = MasterEdition::find_pda(&mint_account.key()).0,
    #[account(mut)]
    pub master_edition_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_nft(ctx: Context<MintNFT>) -> Result<()> {
    // 1.1 => initialize_mint, using struct MintNFT field constraints to create mint
    // 1.2 => mint_to
    // 2、=> create_metadata_accounts_v3
    // 3、=> create_master_edition_v3
    // 4、verify token metadata => sign_metadata
    // 5、verify token belong this collection => set_and_verify_sized_collection_item

    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        ),
        1, // amount
    )?;

    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                mint_authority: ctx.accounts.signer.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        DataV2 {
            // name: ctx.accounts.config.name.clone(),
            // symbol: ctx.accounts.config.symbol.clone(),
            // uri: ctx.accounts.config.base_token_uri.clone(),
            // seller_fee_basis_points: ctx.accounts.config.seller_fee_basis_points,
            creators: None,
            collection: None,
            uses: None,
            name: String::new(),
            symbol: String::new(),
            uri: String::new(),
            seller_fee_basis_points: 0,
        }, // data,
        false, // is_mutable,
        true,  // update_authority_is_signer,
        None,  // collection_details,
    )?;

    create_master_edition_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                mint_authority: ctx.accounts.signer.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        None, // max_supply,
    )?;

    sign_metadata(CpiContext::new(
        ctx.accounts.token_metadata_program.to_account_info(),
        SignMetadata {
            creator: ctx.accounts.signer.to_account_info(), // creator should be authority not user
            metadata: ctx.accounts.metadata_account.to_account_info(),
        },
    ))?;

    Ok(())
}
