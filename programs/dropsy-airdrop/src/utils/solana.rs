use crate::error::ErrorCode;
use crate::utils::calculate_fees;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke, invoke_signed},
    system_instruction,
};

pub struct FeeRecipient<'info> {
    pub account: AccountInfo<'info>,
    pub allocation: u64, // exact lamports to send
}

pub fn check_user_deposit(owner: &AccountInfo, deposit_amount: u64) -> Result<()> {
    require!(
        owner.lamports() >= deposit_amount,
        ErrorCode::InsufficientDeposit
    );
    Ok(())
}

pub fn transfer_sol<'info>(
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    signer_seeds: Option<&[&[&[u8]]]>,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }

    let ix = system_instruction::transfer(from.key, to.key, amount);

    match signer_seeds {
        Some(seeds) => {
            invoke_signed(
                &ix,
                &[from.clone(), to.clone(), system_program.clone()],
                seeds,
            )?;
        }
        None => {
            invoke(&ix, &[from.clone(), to.clone(), system_program.clone()])?;
        }
    }

    Ok(())
}

pub fn charge_protocol_fee<'info>(
    payer: &AccountInfo<'info>,
    treasury: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    fee: u64,
) -> Result<()> {
    transfer_sol(payer, treasury, system_program, None, fee)?;

    msg!("Protocol fee charged: {} lamports", fee);
    Ok(())
}

pub fn process_fees<'info>(
    authority_info: &AccountInfo<'info>,
    affiliate_info: Option<&AccountInfo<'info>>,
    treasury_info: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    total_fee: u64,
    affiliate_percentage: u8,
) -> Result<()> {
    let (affiliate_fee, protocol_fee) = if affiliate_info.is_some() {
        calculate_fees(total_fee, affiliate_percentage)?
    } else {
        // No affiliate → treasury gets 100%
        (0, total_fee)
    };

    if let Some(affiliate_info) = affiliate_info {
        if affiliate_fee > 0 {
            transfer_sol(
                authority_info,
                affiliate_info,
                system_program,
                None,
                affiliate_fee,
            )?;
        }
    }

    if protocol_fee > 0 {
        transfer_sol(
            authority_info,
            treasury_info,
            system_program,
            None,
            protocol_fee,
        )?;
    }

    Ok(())
}

pub fn process_fee_recipients<'info>(
    payer: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    _total_fee: u64,
    recipients: Vec<FeeRecipient<'info>>,
) -> Result<()> {
    let mut total_allocated: u64 = 0;

    for recipient in recipients {
        if recipient.allocation == 0 {
            continue;
        }

        transfer_sol(
            payer,
            &recipient.account,
            system_program,
            None,
            recipient.allocation,
        )?;

        total_allocated = total_allocated
            .checked_add(recipient.allocation)
            .ok_or(ErrorCode::Overflow)?;
    }

    // Strict validation — everything must match exactly
    //require!(total_allocated == total_fee, ErrorCode::InvalidAllocation);

    Ok(())
}
