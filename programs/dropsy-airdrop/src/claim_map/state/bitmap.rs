use crate::{constants::BITMAP_SIZE, error::ErrorCode};
use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct BitmapAccount {
    pub authority: Pubkey,                 // 32 bytes
    pub airdrop: Pubkey,                   // 32 bytes
    pub total: u32,                        // 4 bytes
    pub claimed_bitmap: [u8; BITMAP_SIZE], // 8000 bytes
    pub id: u16,                           // 2 byte
    pub version: u8,                       // 1 byte
    pub bump: u8,                          // 1 byte
}

impl BitmapAccount {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // airdrop  
        4 +  // total
        BITMAP_SIZE + // claimed_bitmap
        2 + // id
        1 + // version
        1; // bump

    pub fn init(
        &mut self,
        authority: Pubkey,
        airdrop: Pubkey,
        total: u32,
        id: u16,
        bump: u8,
        version: u8,
    ) {
        self.authority = authority;
        self.airdrop = airdrop;
        self.id = id;
        self.bump = bump;
        self.version = version;
        self.total = total;
        self.claimed_bitmap = [0; BITMAP_SIZE];
    }

    /*pub fn set_percentage_claimed(&mut self, index: usize, percentage: u8) -> Result<()> {
        require!(self.version == 1, ErrorCode::InvalidAirdropVersion);
        require!(
            index < self.claimed_bitmap.len(),
            ErrorCode::InvalidBitmapIndex
        );
        require!(percentage <= 100, ErrorCode::InvalidPercentage);

        self.claimed_bitmap[index] = percentage;
        Ok(())
    }

    pub fn get_percentage_claimed(&self, index: usize) -> Result<u8> {
        require!(self.version == 1, ErrorCode::InvalidAirdropVersion);
        require!(
            index < self.claimed_bitmap.len(),
            ErrorCode::InvalidBitmapIndex
        );

        Ok(self.claimed_bitmap[index])
    }*/
}
