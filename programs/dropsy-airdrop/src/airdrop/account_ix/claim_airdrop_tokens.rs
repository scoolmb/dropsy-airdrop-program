use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};
use crate::{
    airdrop::{Airdrop, AirdropClaimArgs, AirdropConfig, AirdropMaster}, claim_map::BitmapAccount, 
    constants::{ AIRDROP_CONFIG_SEED, AIRDROP_MASTER_SEED, AIRDROP_SEED, BITMAP_SEED}, 
    error::ErrorCode,
};

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
pub struct ClaimBase<'info> {
   #[account(mut)]
    // validate in handler
    pub source_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = claimer,
        associated_token::mint = mint,
        associated_token::authority = claimer,
        associated_token::token_program = token_program,
    )]
    pub destination_token_account: InterfaceAccount<'info, TokenAccount>,


    #[account(mut, signer)]
    pub claimer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}




#[derive(Accounts)]
#[instruction(args: AirdropClaimArgs)]
pub struct ClaimTokens<'info> {
    pub claim_base: ClaimBase<'info>,
    pub base : BaseStruct<'info>,

    #[account(   
        mut,     
        seeds = [
            AIRDROP_SEED, 
            authority.key().as_ref(),
            claim_base.mint.key().as_ref(), 
        ],
        bump,
        has_one = authority @ ErrorCode::InvalidOwner,
        
    )]
    pub airdrop: AccountLoader<'info, Airdrop>,

    /// CHECK: Validated in airdrop account constraints
    pub authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            BITMAP_SEED, 
            airdrop.key().as_ref(), 
            &args.claim_map_index.to_le_bytes()
        ],
        bump,
        has_one = airdrop @ ErrorCode::InvalidBitmapAccount,
        // validate expected bitmap in handler 
    )]
    pub bitmap: AccountLoader<'info, BitmapAccount>,
}

