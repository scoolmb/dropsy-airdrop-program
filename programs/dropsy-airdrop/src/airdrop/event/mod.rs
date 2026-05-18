use anchor_lang::prelude::*;

#[event]
pub struct AirdropInitialized {
    pub airdrop: Pubkey,
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub master: Pubkey,
    pub merkle_root: [u8; 32],
    pub start_time: i64,
    pub end_time: i64,
    pub timestamp: i64,
}

#[event]
pub struct TokensDeposited {
    pub airdrop: Pubkey,
    pub amount: u64,
    pub new_supply: u64,
    pub depositor: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TokensRedeemed {
    pub airdrop: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}
