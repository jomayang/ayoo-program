use {
    anchor_lang::prelude::*,
    anchor_spl::{
        token::{
            Mint,
            TokenAccount,
            Token,
        },
        associated_token::AssociatedToken,
    },
    solana_program::{
        program::{
            invoke
        },
    },
    spl_token::instruction::transfer,
    crate::{
        state::*,
        error::*,
    }
};

impl ListNft<'_> {
    pub fn handler(
        ctx: Context<Self>,
        price: u64,
        creator_royalty: u8
    ) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
    
        if escrow_account.is_active {
            return Err(Error::from(MarketplaceError::NftListed)); 
        }
    
        let ix = transfer(
            &ctx.accounts.token_program.key(),
            &ctx.accounts.seller_nft_token_account.key(),
            &ctx.accounts.escrow_token_account.key(),
            &ctx.accounts.seller.key(),
            &[&ctx.accounts.seller.key()],
            1
        )?;
    
        invoke(
            &ix,
            &[
                ctx.accounts.seller.to_account_info(),
                ctx.accounts.escrow_token_account.to_account_info(),
                ctx.accounts.seller_nft_token_account.to_account_info(),
            ],
        )?;
    
        escrow_account.creator = ctx.accounts.creator.key();
        escrow_account.seller = ctx.accounts.seller.key();
        escrow_account.mint = ctx.accounts.nft_mint.key();
        escrow_account.token_account = ctx.accounts.escrow_token_account.key();
        escrow_account.price = price;
        escrow_account.creator_royalty = creator_royalty;
        escrow_account.is_active = true;
        
        Ok(())
    }
}


#[derive(Accounts)]
pub struct ListNft<'info> {
  #[account(
    init_if_needed,
    payer = seller,
    seeds = [
      b"escrow".as_ref(),
      creator.key().as_ref(),
      nft_mint.key().as_ref(),
      seller.key().as_ref()
    ],
    bump,
    space = 8 + 1 + 32 + 32 + 32 + 32 + 8 + 1
  )]
  pub escrow_account: Account<'info, Escrow>,
  #[account(
    init_if_needed,
    payer = seller,
    token::mint = nft_mint,
    token::authority = escrow_account,
    seeds = [
        b"token-account".as_ref(), 
        escrow_account.key().as_ref()
    ],
    bump,
  )]
  pub escrow_token_account: Account<'info, TokenAccount>,

  pub nft_mint: Account<'info, Mint>,
  
  /// CHECK:
  #[account(
    seeds = [
        b"metadata".as_ref(), 
        mpl_token_metadata::ID.as_ref(), 
        nft_mint.key().as_ref()
    ],
    seeds::program = mpl_token_metadata::ID,
    bump
  )]
  pub nft_metadata_account: UncheckedAccount<'info>,

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
  pub rent: Sysvar<'info, Rent>,
  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
}