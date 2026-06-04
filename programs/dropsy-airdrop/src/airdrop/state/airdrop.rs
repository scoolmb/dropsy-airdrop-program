use crate::{airdrop::AirdropInitData, constants::VERSION_VESTED};
use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(Default)]
pub struct Airdrop {
    pub master: Pubkey,             // 32
    pub authority: Pubkey,          // 32
    pub mint: Pubkey,               // 32
    pub delegate_authority: Pubkey, // 32
    pub merkle_root: [u8; 32],      // 32
    pub id: u64,                    // 8
    pub supply: u64,                // 8
    pub boost: u64,                 // 8
    pub starts_at: i64,             // 8
    pub ends_at: i64,               // 8
    pub bitmap_count: u16,          // 2
    pub state: u8,                  // 1
    pub version: u8,                // 1
    pub bump: u8,                   // 1
    pub _padding: [u8; 3],          // 3
}

impl Airdrop {
    pub const LEN: usize = 8 + 32 * 5 + 8 * 5 + 8;

    pub fn init(
        &mut self,
        airdrop_master: Pubkey,
        authority: Pubkey,
        mint: Pubkey,
        data: AirdropInitData,
        bump: u8,
    ) -> Result<()> {
        self.master = airdrop_master;
        self.authority = authority;
        self.mint = mint;
        self.delegate_authority = authority;
        self.merkle_root = data.merkle_root;
        self.id = data.id;
        self.supply = 0;
        self.boost = 0;
        self.starts_at = data.starts_at;
        self.ends_at = data.ends_at;
        self.bitmap_count = 0;
        self.state = 0;
        self.version = data.version;
        self.bump = bump;
        self._padding = [0u8; 3];

        Ok(())
    }

    pub fn is_vested(&self) -> bool {
        self.version == VERSION_VESTED
    }
}
