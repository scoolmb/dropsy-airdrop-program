use crate::{
    airdrop::AirdropConfigInitArgs,
    constants::{
        DEFAULT_AIRDROP_DURATION, DEFAULT_CREATE_MASTER_FEE, DEFAULT_MASTER_FEE_BPS,
        DEFAULT_MAX_ACTION_FEE, DEFAULT_MAX_AIRDROP_DURATION, DEFAULT_MAX_CLAIM_FEE,
        DEFAULT_MERKLE_ROOT, DEFAULT_MIN_AIRDROP_DURATION, DEFAULT_PROTOCOL_FEE,
        DEFAULT_UPDATE_GRACE_PERIOD,
    },
};
use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct AirdropConfig {
    pub authority: Pubkey,
    pub protocol_treasury: Pubkey,
    pub wl_root: [u8; 32],
    pub protocol_fee: u64,
    pub airdrop_master_create_fee: u64,
    pub max_claim_fee: u64,
    pub max_action_fee: u64,
    pub min_airdrop_duration: i64,
    pub default_airdrop_duration: i64,
    pub max_airdrop_duration: i64,
    pub update_grace_period: i64,
    pub master_fee_bps: u16,
    pub version: u8,
    pub bump: u8,
    pub _padding: [u8; 4],
    pub reserved: [u8; 256], // 256 bytes of reserved for future upgrades
}

impl AirdropConfig {
    pub const LEN: usize = 8 + 32 * 3 + 8 * 8 + 8 + 256; // last 8 bytes includes version + bump + padding

    pub fn init(
        &mut self,
        authority: Pubkey,
        protocol_treasury: Pubkey,
        args: AirdropConfigInitArgs,
        bump: u8,
    ) {
        self.authority = authority;
        self.protocol_treasury = protocol_treasury;
        self.wl_root = args.wl_root.unwrap_or(DEFAULT_MERKLE_ROOT);
        self.airdrop_master_create_fee = args
            .airdrop_master_create_fee
            .unwrap_or(DEFAULT_CREATE_MASTER_FEE);
        self.max_action_fee = args.max_action_fee.unwrap_or(DEFAULT_MAX_ACTION_FEE);
        self.max_claim_fee = args.max_claim_fee.unwrap_or(DEFAULT_MAX_CLAIM_FEE);
        self.min_airdrop_duration = args
            .min_airdrop_duration
            .unwrap_or(DEFAULT_MIN_AIRDROP_DURATION);
        self.default_airdrop_duration = args
            .default_airdrop_duration
            .unwrap_or(DEFAULT_AIRDROP_DURATION);
        self.max_airdrop_duration = args
            .max_airdrop_duration
            .unwrap_or(DEFAULT_MAX_AIRDROP_DURATION);
        self.update_grace_period = args
            .update_grace_period
            .unwrap_or(DEFAULT_UPDATE_GRACE_PERIOD);
        self.protocol_fee = args.protocol_fee.unwrap_or(DEFAULT_PROTOCOL_FEE);
        self.master_fee_bps = args.master_fee_bps.unwrap_or(DEFAULT_MASTER_FEE_BPS);
        self.version = 0;
        self.bump = bump;
        self._padding = [0u8; 4];
        self.reserved = [0u8; 256]; // Initialize 256 bytes of reserved space
    }
}
