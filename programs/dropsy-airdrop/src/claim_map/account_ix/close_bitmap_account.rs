use anchor_lang::{prelude::*, system_program};
use anchor_spl::token_interface::Mint;

use crate::{airdrop::{Airdrop, AirdropConfig}, claim_map::BitmapAccount, constants::{ AIRDROP_CONFIG_SEED, AIRDROP_SEED, BITMAP_SEED}, error::ErrorCode};



#[derive(Accounts)]
#[instruction(
    airdrop_id: u64,
    id: u16,  
)]
pub struct ClosedBitmap<'info> {
    #[account(
        seeds = [AIRDROP_CONFIG_SEED],
        bump,
        has_one = protocol_treasury @ ErrorCode::InvalidTreasuryAccount,
    )]
    pub airdrop_config: AccountLoader<'info, AirdropConfig>,

    #[account(mut)]
    /// CHECK: Treasury
    pub protocol_treasury: UncheckedAccount<'info>,
    
    // airdrop pda
    #[account(
        mut,
        seeds = [
            AIRDROP_SEED, 
            authority.key().as_ref(),
            mint.key().as_ref(), 
            &airdrop_id.to_le_bytes(),
        ],
        bump,
        has_one = authority @ ErrorCode::InvalidOwner,
        has_one = mint @ ErrorCode::MintMismatch,
    )]
    pub airdrop: AccountLoader<'info, Airdrop>,

       // airdrop mint 
    pub mint: InterfaceAccount<'info, Mint>,

    // bitmap pda to be closed 
    #[account(
        mut,
        close = authority,
        has_one = airdrop @ ErrorCode::InvalidAirdropPda,
        has_one = authority @ ErrorCode::InvalidOwner,
        seeds = [
            BITMAP_SEED, 
            airdrop.key().as_ref(), 
            &id.to_le_bytes()
        ],
        bump,
    )]
    pub bitmap: AccountLoader<'info, BitmapAccount>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,

    #[account(address = system_program::ID)] 
    pub system_program: Program<'info, System>,
}