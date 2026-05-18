use anchor_lang::prelude::*;

#[event]
pub struct BitmapInitialized {
    pub airdrop: Pubkey,
    pub bitmap: Pubkey,
    pub bitmap_id: u16,
    pub timestamp: i64,
}

#[event]
pub struct BitmapClosed {
    pub airdrop: Pubkey,
    pub bitmap: Pubkey,
    pub authority: Pubkey,
}
