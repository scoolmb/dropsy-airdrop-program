use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{airdrop::{Airdrop}, constants::{ AIRDROP_SEED}, error::ErrorCode};

#[derive(Accounts)]
pub struct RedeemedTokens<'info> {
    #[account(
        mut,
        seeds = [
            AIRDROP_SEED, 
            mint.key().as_ref(), 
            authority.key().as_ref(),
        ],
        bump,
        has_one = authority @ ErrorCode::InvalidOwner,
        has_one = mint @ ErrorCode::MintMismatch,
    )]
    pub airdrop: AccountLoader<'info, Airdrop>,

    #[account(
        mut,
        constraint = vault.owner == airdrop.key() @ ErrorCode::InvalidAirdropPda,
        constraint = vault.mint == mint.key() @ ErrorCode::MintMismatch,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
        associated_token::token_program = token_program,
    )]
    pub destination_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
