use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub is_active: bool,
    pub seller: Pubkey,
    pub creator: Pubkey,
    pub creator_royalty: u8,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub price: u64,
}