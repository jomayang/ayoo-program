pub mod instructions;
pub mod error;
pub mod state;

use {
  anchor_lang::prelude::*,
  instructions::*,
};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod tiktok_market {
    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNft>,
        creator_key: Pubkey,
        title: String,
        uri: String,
        price: u64
    ) -> Result<()> {
        MintNft::handler(ctx, creator_key, title, uri, price)
    }

    pub fn list_nft(
        ctx: Context<ListNft>, 
        price: u64, 
        creator_royalty: u8
    ) -> Result<()> {
        ListNft::handler(ctx, price, creator_royalty)
    }

    pub fn delist_nft(
        ctx: Context<DelistNft>, 
        escrow_nonce: u8
    ) -> Result<()> {
        DelistNft::handler(ctx, escrow_nonce)
    }

    pub fn buy_nft(ctx: Context<BuyNft>, escrow_nonce: u8) -> Result<()> {
        BuyNft::handler(ctx, escrow_nonce)
    }

}
