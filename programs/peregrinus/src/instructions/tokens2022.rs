use {
    anchor_lang::prelude::*,
    anchor_spl::{
        // token::Mint,
        token_2022,
        token_2022::Token2022,
        token_2022_extensions,
        token_interface::spl_token_2022::{extension::ExtensionType, state::Mint as Mint2022},
    },
};

pub fn mint_sbt(ctx: Context<Tokens2022>) -> Result<()> {
    let space =
        ExtensionType::try_calculate_account_len::<Mint2022>(&[ExtensionType::NonTransferable])?;
    ctx.accounts.init_account(space)?;

    token_2022_extensions::non_transferable_mint_initialize(CpiContext::new(
        ctx.accounts.token_program_2022.to_account_info(),
        token_2022_extensions::NonTransferableMintInitialize {
            token_program_id: ctx.accounts.token_program_2022.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
        },
    ))?;

    token_2022::initialize_mint2(
        CpiContext::new(
            ctx.accounts.token_program_2022.to_account_info(),
            token_2022::InitializeMint2 {
                mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        0,                              // decimals: u8,
        ctx.accounts.payer.key,         // authority: &Pubkey,
        Some(&ctx.accounts.mint.key()), // freeze_authority: Option<&Pubkey>,
    )?;

    Ok(())
}

impl<'info> Tokens2022<'info> {
    fn init_account(&self, space: usize) -> Result<()> {
        let rent_required = Rent::get()?.minimum_balance(space);

        msg!("Mint account address : {}", self.mint.key());
        anchor_lang::system_program::create_account(
            CpiContext::new(
                self.token_program_2022.to_account_info(),
                anchor_lang::system_program::CreateAccount {
                    from: self.payer.to_account_info(),
                    to: self.mint.to_account_info(),
                },
            ),
            rent_required,
            space as u64,
            self.token_program_2022.key,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Tokens2022<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK:
    // pub mint_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    // pub mint: Account<'info, Mint>,

    /// CHECK:
    pub token_program_2022: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
