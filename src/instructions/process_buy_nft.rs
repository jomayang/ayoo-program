use {
    anchor_lang::prelude::*,
    anchor_spl::associated_token,
    anchor_spl::token,
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

impl BuyNft<'_> {
    pub fn handler(ctx: Context<Self>, escrow_nonce: u8) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        let price = escrow_account.price;
        
        if !escrow_account.is_active {
            return Err(Error::from(MarketplaceError::NftUnlisted));
        }

        msg!("price: {}", escrow_account.price);

        let royalty_amount = (price * escrow_account.creator_royalty as u64) / 100;
        let seller_amount = price - royalty_amount; 
        let fee_amount = (royalty_amount * 15)/100;
        let creator_amount = royalty_amount - fee_amount;
        // msg!("creator amount: {}", creator_amount);
        // msg!("seller amount: {}", seller_amount);

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.buyer_usdc_account.to_account_info(),
                    to: ctx.accounts.seller_usdc_account.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info(),
                }
            ),
            seller_amount,
        )?;
        
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.buyer_usdc_account.to_account_info(),
                    to: ctx.accounts.creator_usdc_account.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info(),
                }
            ),
            creator_amount,
        )?;

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.buyer_usdc_account.to_account_info(),
                    to: ctx.accounts.fee_usdc_account.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info(),
                }
            ),
            fee_amount,
        )?;

        let transfer_ix = transfer(
            ctx.accounts.token_program.key,
            &ctx.accounts.escrow_token_account.key(),
            &ctx.accounts.buyer_nft_token_account.key(),
            &ctx.accounts.escrow_account.key(),
            &[],
            1,
        )?;
    
        invoke_signed(
            &transfer_ix,
            &[
                ctx.accounts.escrow_account.to_account_info(),
                ctx.accounts.escrow_token_account.to_account_info(),
                ctx.accounts.buyer_nft_token_account.to_account_info(),
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
pub struct BuyNft<'info> {
    #[account(
        mut,
        seeds = [
            b"escrow".as_ref(),
            creator.key().as_ref(),
            nft_mint.key().as_ref(),
            seller.key().as_ref(),
        ],
        bump = escrow_nonce,
        close = buyer
    )]
    pub escrow_account: Box<Account<'info, Escrow>>,
    
    #[account(
        mut,
        seeds = [
            b"token-account".as_ref(),
            escrow_account.key().as_ref()
        ],
        bump
    )]
    pub escrow_token_account: Box<Account<'info, token::TokenAccount>>,
    
    pub nft_mint: Box<Account<'info, token::Mint>>,
    
    #[account(
        init_if_needed, 
        payer = buyer,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub buyer_nft_token_account: Box<Account<'info, token::TokenAccount>>,
    /// CHECK: 
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    #[account(
       mut,
       constraint = seller_usdc_account.owner == seller.key()
    )]
    pub seller_usdc_account: Box<Account<'info, token::TokenAccount>>,

    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        mut,
        constraint = buyer_usdc_account.owner == buyer.key()
    )]
    pub buyer_usdc_account: Box<Account<'info, token::TokenAccount>>,
    /// CHECK:
    pub creator: UncheckedAccount<'info>,
    #[account(
        mut,
        constraint = creator_usdc_account.owner == creator.key()
    )]
    pub creator_usdc_account: Box<Account<'info, token::TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub fee_usdc_account: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}