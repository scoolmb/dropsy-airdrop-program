use crate::airdrop::MasterAirdropInitArgs;
use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct AirdropMaster {
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub treasury: Pubkey,

    pub points: u64,
    pub total_claim_count: u64,
    pub monetized_claim_quota: u64,
    pub monetized_claim_count: u64,

    // Airdrop Features fees
    pub airdrop_update_fee: u64,
    pub airdrop_creation_fee: u64,
    pub airdrop_claim_fee: u64,
    pub airdrop_delegate_fee: u64,

    // Bitmap Features fees
    pub bitmap_creation_fee: u64,

    pub bump: u8,
    pub _padding: [u8; 7], // alignement
}

impl AirdropMaster {
    pub const LEN: usize = 8 + 32 * 3 + 8 * 9 + 8; // last 8 bytes includes bump + padding

    pub fn init(
        &mut self,
        creator: Pubkey,
        treasury: Pubkey,
        args: MasterAirdropInitArgs,
        bump: u8,
    ) {
        self.creator = creator;
        self.authority = creator;
        self.treasury = treasury;

        self.airdrop_update_fee = args.airdrop_update_fee.unwrap_or(0);
        self.airdrop_creation_fee = args.airdrop_creation_fee.unwrap_or(0);
        self.airdrop_claim_fee = args.airdrop_claim_fee.unwrap_or(0);
        self.airdrop_delegate_fee = args.airdrop_delegate_fee.unwrap_or(0);
        self.bitmap_creation_fee = args.bitmap_creation_fee.unwrap_or(0);

        // default
        self.points = 0;
        self.total_claim_count = 0;
        self.monetized_claim_quota = 100;
        self.monetized_claim_count = 0;

        self.bump = bump;
        self._padding = [0u8; 7];
    }
}
