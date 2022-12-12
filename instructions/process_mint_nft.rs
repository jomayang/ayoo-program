use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::system_program;
use anchor_spl::associated_token;
use anchor_spl::token;
use mpl_token_metadata::ID as TOKEN_METADATA_ID;
use mpl_token_metadata::instruction as token_instruction;

impl MintNft<'_> {
    pub fn handler(
        ctx: Context<Self>,
        creator_key: Pubkey,
        title: String,
        uri: String,
        price: u64,
    ) -> Result<()> {
        
        let fee_amount = (price * 15)/100;
        let creator_amount = price - fee_amount;
        // USDC PAYMEN HERE!!!
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.user_usdc_account.to_account_info(),
                    to: ctx.accounts.treasury_usdc_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                }
            ),
            creator_amount
        )?;
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.user_usdc_account.to_account_info(),
                    to: ctx.accounts.fee_usdc_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                }
            ),
            fee_amount
        )?;

        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 0,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.treasury.key(),
                verified: false,
                share: 100,
            },
        ];
        // creating mint account
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.mint.to_account_info(),
                },
            ),
            10000000,
            82,
            &ctx.accounts.token_program.key(),
        )?;
    
        // initializing mint account
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            0,
            &ctx.accounts.user.key(),
            Some(&ctx.accounts.user.key()),
        )?;
    
        // creating token account
        associated_token::create(
            CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                associated_token::Create {
                    payer: ctx.accounts.user.to_account_info(),
                    associated_token: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
        )?;
    
        // minting token to token account
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            1,
        )?;
    
        // creating metadata account
        invoke(
            &token_instruction::create_metadata_accounts_v2(
                TOKEN_METADATA_ID, 
                ctx.accounts.metadata.key(), 
                ctx.accounts.mint.key(), 
                ctx.accounts.user.key(), 
                ctx.accounts.user.key(), 
                ctx.accounts.user.key(), 
                title, 
                "".to_string(), 
                uri, 
                Some(creator),
                1,
                true, 
                false, 
                None, 
                None,
            ),
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;
    
        // creating master edition metadata account
        invoke(
            &token_instruction::create_master_edition_v3(
                TOKEN_METADATA_ID, 
                ctx.accounts.master_edition.key(), 
                ctx.accounts.mint.key(), 
                ctx.accounts.user.key(), 
                ctx.accounts.user.key(), 
                ctx.accounts.metadata.key(), 
                ctx.accounts.user.key(), 
                Some(0),
            ),
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;
        
        Ok(())
    }
}
    

#[derive(Accounts)]
pub struct MintNft<'info> {
  #[account(mut)]
  /// CHECK:
  pub metadata: UncheckedAccount<'info>,
  /// CHECK:
  #[account(mut)]
  pub master_edition: UncheckedAccount<'info>,
  #[account(mut)]
  pub mint: Signer<'info>,
  /// CHECK:
  #[account(mut)]
  pub token_account: UncheckedAccount<'info>,
  #[account(
    mut,
    constraint = user_usdc_account.owner == user.key()
  )]
  pub user_usdc_account: Account<'info, token::TokenAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(
    mut,
    constraint = treasury_usdc_account.owner == treasury.key()
  )]
  pub treasury_usdc_account: Account<'info, token::TokenAccount>,
  /// CHECK:
  #[account(mut)]
  pub fee_usdc_account: UncheckedAccount<'info>,
  /// CHECK:
  #[account(mut)]
  pub treasury: UncheckedAccount<'info>,
//   /// CHECK:
//   #[account(mut)]
//   pub fee_acc: UncheckedAccount<'info>,
  pub usdc_mint: Account<'info, token::Mint>,
  pub rent: Sysvar<'info, Rent>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, token::Token>,
  pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
  /// CHECK:
  pub token_metadata_program: UncheckedAccount<'info>,
}

