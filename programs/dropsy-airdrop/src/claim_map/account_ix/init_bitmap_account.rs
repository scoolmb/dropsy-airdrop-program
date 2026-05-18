use anchor_lang::{prelude::*, system_program};
use anchor_spl::token_interface::Mint;

use crate::{airdrop::{Airdrop, AirdropConfig, AirdropMaster}, claim_map::BitmapAccount, constants::{ AIRDROP_CONFIG_SEED, AIRDROP_MASTER_SEED, AIRDROP_SEED, BITMAP_SEED}, error::ErrorCode,};


#[derive(Accounts)]
pub struct BaseStruct<'info> {
   
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
        seeds = [AIRDROP_MASTER_SEED, master_creator.key().as_ref()],
        bump,
        has_one = treasury @ ErrorCode::InvalidTreasuryAccount,
    )]
    pub airdrop_master: AccountLoader<'info, AirdropMaster>,

    #[account(mut)]
    /// CHECK: Treasury
    pub treasury: UncheckedAccount<'info>,

    /// CHECK: creator
    pub master_creator: UncheckedAccount<'info>,

}

#[derive(Accounts)]
#[instruction(
    id: u16, 
    total: u32, 
    //version: u8,
)]
pub struct InitialisedBitmap<'info> {
    pub base : BaseStruct<'info>,
    #[account(
        init,
        payer = authority,
        seeds = [
            BITMAP_SEED, 
            airdrop.key().as_ref(), 
            &id.to_le_bytes()
        ],
        bump,
        space = BitmapAccount::LEN,
    )]
    pub bitmap: AccountLoader<'info, BitmapAccount>,

    #[account(
        mut,
        seeds = [
            AIRDROP_SEED, 
              authority.key().as_ref(),
            mint.key().as_ref(), 
        
        ],
        bump,
        has_one = authority @ ErrorCode::InvalidOwner,
        has_one = mint @ ErrorCode::MintMismatch,
    )]
    pub airdrop: AccountLoader<'info, Airdrop>,

    pub mint: InterfaceAccount<'info, Mint>,
    
    #[account(mut, signer)]
    pub authority: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}