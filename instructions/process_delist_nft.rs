use {
    anchor_lang::prelude::*,
    anchor_spl::{
        token::{
            Mint,
            TokenAccount,
            Token,
        },
    },
    spl_token::instruction::{
        transfer,
        close_account,
    },
    crate::{
        state::*,
        error::*,
    }
};
use solana_program::program::invoke_signed;

impl DelistNft<'_> {
    pub fn handler(
        ctx: Context<Self>,
        escrow_nonce: u8,
    ) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
    
        if !escrow_account.is_active {
            return Err(Error::from(MarketplaceError::NftUnlisted));
        }
    
        if escrow_account.seller != *ctx.accounts.seller.key {
            return Err(Error::from(MarketplaceError::UnknownSeller));
        }
    
        let transfer_ix = transfer(
            ctx.accounts.token_program.key,
            &ctx.accounts.escrow_token_account.key(),
            &ctx.accounts.seller_nft_token_account.key(),
            &ctx.accounts.escrow_account.key(),
            &[],
            1,
        )?;
    
        invoke_signed(
            &transfer_ix,
            &[
                ctx.accounts.escrow_account.to_account_info(),
                ctx.accounts.escrow_token_account.to_account_info(),
                ctx.accounts.seller_nft_token_account.to_account_info(),
            ],
            &[
                &[
                    b"escrow".as_ref(),
                    ctx.accounts.creator.key().as_ref(),
                    ctx.accounts.nft_mint.key().as_ref(),
                    ctx.accounts.seller.key().as_ref(),
                    &[escrow_nonce],
                ],
            ],
        )?;
    
        let close_ix = close_account(
            ctx.accounts.token_program.key,
            &ctx.accounts.escrow_token_account.key(),
            &ctx.accounts.seller.key(),
            &ctx.accounts.escrow_account.key(),
            &[],
        )?;
    
        invoke_signed(
            &close_ix,
            &[
                ctx.accounts.escrow_account.to_account_info(),
                ctx.accounts.escrow_token_account.to_account_info(),
                ctx.accounts.seller.to_account_info(),
            ],
            &[
                &[
                    b"escrow".as_ref(),
                    ctx.accounts.creator.key().as_ref(),
                    ctx.accounts.nft_mint.key().as_ref(),
                    ctx.accounts.seller.key().as_ref(),
                    &[escrow_nonce],
                ],
            ],
        )?;
        Ok(())    
    }
}



#[derive(Accounts)]
#[instruction(escrow_nonce: u8)]
pub struct DelistNft<'info> {
    #[account(
        mut,
        seeds = [
            b"escrow".as_ref(),
            creator.key().as_ref(),
            nft_mint.key().as_ref(),
            seller.key().as_ref(),
        ],
        bump = escrow_nonce,
        close = seller
    )]
    pub escrow_account: Account<'info, Escrow>,
    
    #[account(
        mut,
        seeds = [
            b"token-account".as_ref(),
            escrow_account.key().as_ref()
        ],
        bump
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    
    pub nft_mint: Account<'info, Mint>,
    
    #[account(
        associated_token::mint = nft_mint,
        associated_token::authority = seller,
    )]
    #[account(mut)]
    pub seller_nft_token_account: Account<'info, TokenAccount>,
   
    #[account(mut)]
    pub seller: Signer<'info>,
    /// CHECK:
    pub creator: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}