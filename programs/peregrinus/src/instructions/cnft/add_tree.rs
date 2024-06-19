use crate::constants::CNFT_CONFIG_SEED;
use crate::state::CnftConfig;
use crate::MplBubblegum;
use crate::Noop;
use crate::SplAccountCompression;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use mpl_bubblegum::instructions::CreateTreeConfigCpiBuilder;

pub fn add_tree(ctx: Context<AddTree>) -> Result<()> {
    CreateTreeConfigCpiBuilder::new(&ctx.accounts.bubblegum_program.to_account_info())
        .tree_config(&ctx.accounts.tree_config.to_account_info())
        .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
        .payer(&&ctx.accounts.authority.to_account_info())
        .tree_creator(&&ctx.accounts.cnft_config.to_account_info())
        .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
        .compression_program(&ctx.accounts.compression_program.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .max_depth(CnftConfig::MAX_TREE_DEPTH)
        .max_buffer_size(CnftConfig::MAX_TREE_BUFFER_SIZE)
        .invoke_signed(&[&[
            CNFT_CONFIG_SEED,
            ctx.accounts.tree_config.key().as_ref(),
            &[ctx.bumps.cnft_config],
        ]])?;

    Ok(())
}

#[derive(Accounts)]
pub struct AddTree<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
		mut,
		seeds=[
			CNFT_CONFIG_SEED,
			tree_config.key().as_ref(),
		],
        bump
	)]
    pub cnft_config: Account<'info, CnftConfig>,
    /// CHECK: This account must be all zeros
    #[account(zero, signer)]
    pub merkle_tree: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,

    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
}
