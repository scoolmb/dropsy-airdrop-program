use anchor_lang::{prelude::*, system_program};
use anchor_spl::token_interface::Mint;

use crate::{airdrop::{Airdrop, AirdropConfig}, constants::{ AIRDROP_CONFIG_SEED, AIRDROP_SEED}, error::ErrorCode,};

#[derive(Accounts)]
#[instruction(
    airdrop_id: u64,
)]
pub struct ClosedAirdrop<'info> {
    #[account(
        seeds = [AIRDROP_CONFIG_SEED],
        bump,
        has_one = protocol_treasury @ ErrorCode::InvalidTreasuryAccount,
    )]
    pub airdrop_config: AccountLoader<'info, AirdropConfig>,

    #[account(mut)]
    /// CHECK: Treasury
    pub protocol_treasury: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            AIRDROP_SEED, 
            authority.key().as_ref(),
            mint.key().as_ref(), 
            &airdrop_id.to_le_bytes() 
        ],
        bump,
   
        has_one = authority @ ErrorCode::InvalidOwner,
        has_one = mint @ ErrorCode::MintMismatch,
    )]
    pub airdrop: AccountLoader<'info, Airdrop>,
    
    // airdrop mint 
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}