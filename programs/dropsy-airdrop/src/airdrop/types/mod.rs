use anchor_lang::prelude::*;

use crate::error::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct MasterAirdropInitArgs {
    pub airdrop_update_fee: Option<u64>,
    pub airdrop_creation_fee: Option<u64>,
    pub airdrop_claim_fee: Option<u64>,
    pub airdrop_delegate_fee: Option<u64>,
    pub bitmap_creation_fee: Option<u64>,
    pub discount_proof: Option<Vec<[u8; 32]>>,
}

impl MasterAirdropInitArgs {
    pub fn validate(&self, max_claim_fee: u64, max_action_fee: u64) -> Result<()> {
        // Claim fee → special cap
        if let Some(claim_fee) = self.airdrop_claim_fee {
            require!(claim_fee <= max_claim_fee, ErrorCode::ClaimFeeTooHigh);
        }

        // All other fees → action cap
        let action_fees = [
            self.airdrop_update_fee,
            self.airdrop_creation_fee,
            self.airdrop_delegate_fee,
            self.bitmap_creation_fee,
        ];

        for fee in action_fees.iter().flatten() {
            require!(*fee <= max_action_fee, ErrorCode::ActionFeeTooHigh);
        }

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AirdropConfigInitArgs {
    pub wl_root: Option<[u8; 32]>,
    pub airdrop_master_create_fee: Option<u64>,
    pub max_claim_fee: Option<u64>,
    pub max_action_fee: Option<u64>,
    pub min_airdrop_duration: Option<i64>,
    pub default_airdrop_duration: Option<i64>,
    pub max_airdrop_duration: Option<i64>,
    pub update_grace_period: Option<i64>,
    pub protocol_fee: Option<u64>,
    pub master_fee_bps: Option<u16>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AirdropInitArgs {
    pub merkle_root: Option<[u8; 32]>,
    pub starts_at: Option<i64>,
    pub ends_at: Option<i64>,
    pub version: Option<u8>,
    pub mutable: Option<u8>,
    pub delegate_authority: Option<Pubkey>,
    pub delegate_permissions: Option<u8>,
}

pub struct AirdropInitData {
    pub starts_at: i64,
    pub ends_at: i64,
    pub version: u8,
    pub merkle_root: [u8; 32],
    pub mutable: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AirdropClaimArgs {
    pub index: u64,
    pub proof: Vec<[u8; 32]>,
    pub amount: u64,
    pub claim_map_index: u16,
}
