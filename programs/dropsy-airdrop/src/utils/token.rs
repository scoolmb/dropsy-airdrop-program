use anchor_lang::prelude::*;
use anchor_spl::token_interface::{BurnChecked, CloseAccount, TransferChecked};

pub fn transfer_tokens_from_vault<'info>(
    vault: &AccountInfo<'info>,
    destination: &AccountInfo<'info>,
    statepda: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    mint: &AccountInfo<'info>, // Added mint parameter
    signer_seeds: &[&[&[u8]]],
    amount: u64,
    decimals: u8, // Added decimals parameter
) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: vault.clone(),
        to: destination.clone(),
        authority: statepda.clone(),
        mint: mint.clone(), // Include mint in CPI accounts
    };

    let cpi_ctx = CpiContext::new_with_signer(token_program.clone(), cpi_accounts, signer_seeds);

    anchor_spl::token_interface::transfer_checked(cpi_ctx, amount, decimals)?;

    msg!("Withdrawn {} tokens from PDA to owner", amount);
    Ok(())
}

pub fn transfer_tokens_to_pda<'info>(
    source: &AccountInfo<'info>,
    destination: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    amount: u64,
    decimals: u8, // Added decimals parameter
) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: source.clone(),
        mint: mint.clone(), // Include mint in CPI accounts
        to: destination.clone(),
        authority: authority.clone(),
    };

    let cpi_ctx = CpiContext::new(token_program.clone(), cpi_accounts);

    // Use transfer_checked which requires decimals
    anchor_spl::token_interface::transfer_checked(cpi_ctx, amount, decimals)?;

    msg!("Transferred {} tokens from source to PDA", amount);
    Ok(())
}

pub fn burn_dropsy_token<'info>(
    source: &AccountInfo<'info>,
    signer: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    amount: u64,
    decimals: u8, // Added decimals parameter
) -> Result<()> {
    let cpi_accounts = BurnChecked {
        mint: mint.clone(),
        from: source.clone(),
        authority: signer.clone(),
    };

    let cpi_ctx = CpiContext::new(token_program.clone(), cpi_accounts);

    // Use transfer_checked which requires decimals
    anchor_spl::token_interface::burn_checked(cpi_ctx, amount, decimals)?;

    msg!("Burned {} tokens from {}", amount, signer.key());
    Ok(())
}

pub fn close_vault_account<'info>(
    vault: AccountInfo<'info>,
    destination: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let cpi_accounts = CloseAccount {
        account: vault,
        destination,
        authority,
    };

    let cpi_ctx = CpiContext::new_with_signer(token_program, cpi_accounts, signer_seeds);

    // Changed to token_interface version
    anchor_spl::token_interface::close_account(cpi_ctx)?;
    msg!("Vault account closed successfully.");

    Ok(())
}
