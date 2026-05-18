use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{airdrop::{Airdrop}, constants::{ AIRDROP_SEED}, error::ErrorCode};

#[derive(Accounts)]
#[instruction(
    amount: u64,
)]
pub struct DepositTokens<'info> {
    #[account(mut)]
    // validate in handler
    pub source_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = airdrop,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

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

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
