use anchor_lang::prelude::*;

// airdrop seeds
pub const AIRDROP_CONFIG_SEED: &[u8] = b"airdrop_config";
pub const AIRDROP_MASTER_SEED: &[u8] = b"airdrop_master";
pub const AIRDROP_SEED: &[u8] = b"airdrop";

pub const AFFILIATE_SEED: &[u8] = b"affiliate";
pub const BITMAP_SEED: &[u8] = b"bitmap";
pub const VESTING_SEED: &[u8] = b"vesting";

pub const PRESALE_SEED: &[u8] = b"presale";
pub const VAULT_SEED: &[u8] = b"vault";
// pubKeys
pub const ADMIN: Pubkey = pubkey!("5gB7WLoBWELAvRDYPUvGj7e7BHEv65j6YDhissdMTQ4F");

pub const MAX_BITMAP_CLAIM: u64 = BITMAP_SIZE as u64 * 8;
pub const BITMAP_SIZE: usize = 1000;

// airdrop version
pub const VERSION_BASIC: u8 = 0; // BasicAirdrop
pub const VERSION_VESTED: u8 = 1; // VestedAirdrop

// airdrop state
pub const STATE_INITIALIZED: u8 = 0; //default
pub const STATE_DELEGATED: u8 = 1; // airdrop is delegated
pub const STATE_REDEEMED: u8 = 2; // airdrop tokens redeemed

pub const PERMISSION_NONE: u8 = 0;
pub const PERMISSION_TIMING: u8 = 1 << 0; // 1
pub const PERMISSION_MERKLE: u8 = 1 << 1; // 2
pub const PERMISSION_DELEGATION: u8 = 1 << 2; // 4
pub const PERMISSION_FULLY: u8 = PERMISSION_TIMING | PERMISSION_MERKLE | PERMISSION_DELEGATION; // 7

pub const MUTABLE_NONE: u8 = 0; // immutable
pub const MUTABLE_TIMING: u8 = 1 << 0; // 1
pub const MUTABLE_MERKLE: u8 = 1 << 1; // 2
pub const MUTABLE_DELEGATION: u8 = 1 << 2; // 4
pub const MUTABLE_FULLY: u8 = MUTABLE_TIMING | MUTABLE_MERKLE | MUTABLE_DELEGATION; // 7

pub const VESTING_LINEAR: u8 = 0;
pub const VESTING_CURVED: u8 = 1;
pub const VESTING_STEP: u8 = 2;

// Default values
pub const DEFAULT_MERKLE_ROOT: [u8; 32] = [0u8; 32];
pub const DEFAULT_PROTOCOL_FEE: u64 = 90_000; // 0.00009 SOL
pub const DEFAULT_MASTER_FEE_BPS: u16 = 5_000; // 50%
pub const DEFAULT_MAX_ACTION_FEE: u64 = 1_000_000_000; // 0.1 SOL
pub const DEFAULT_MAX_CLAIM_FEE: u64 = 20_000_000; // 0.02 SOL
pub const DEFAULT_CREATE_MASTER_FEE: u64 = 200_000_000; // 0.2 SOL

pub const SECONDS_PER_HOUR: i64 = 60 * 60;
pub const SECONDS_PER_DAY: i64 = 24 * SECONDS_PER_HOUR;

pub const DEFAULT_MIN_AIRDROP_DURATION: i64 = SECONDS_PER_DAY; // 24h
pub const DEFAULT_UPDATE_GRACE_PERIOD: i64 = SECONDS_PER_DAY; // 24h
pub const DEFAULT_AIRDROP_DURATION: i64 = 60 * SECONDS_PER_DAY; // 60 days
pub const DEFAULT_MAX_AIRDROP_DURATION: i64 = 4 * 365 * SECONDS_PER_DAY; // ~4 years
