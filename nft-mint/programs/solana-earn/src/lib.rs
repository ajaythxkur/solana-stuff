use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{
    use create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
     CreateMetadataAccountsV3, Metadata,
};
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};
use mpl_token_metadata::types::{Collection, Creator, DataV2};

declare_id!("5FiVwy7dFM1QHpgHHffex67Wvg84B3K3T5KvR4SX3RA2");

#[program]
pub mod nft_mint {
    use super::*;

    pub fn create_single_nft(
        ctx: Context<CreateNFT>,
        id: u64,
        name: String,
        uri: String,
        price: f32,
        cant: u64,
    ) -> Result<()> {
        // mint_to()?;
        // ce
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateNFT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
        seeds = ["mint".as_bytes(), id.to_le_bytes().as_ref()],
        bump,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition".as_ref()
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    // CHECK:
    pub master_edition_account: UncheckedAccount<'info>, #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub nft_metadata: UncheckedAccount<'info>,
}
