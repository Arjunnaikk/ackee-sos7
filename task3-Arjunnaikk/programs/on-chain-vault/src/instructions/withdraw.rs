//-------------------------------------------------------------------------------
use crate::errors::VaultError;
use crate::events::WithdrawEvent;
use crate::state::Vault;
///
/// TASK: Implement the withdraw functionality for the on-chain vault
///
/// Requirements:
/// - Verify that the vault is not locked
/// - Verify that the vault has enough balance to withdraw
/// - Transfer lamports from vault to vault authority
/// - Emit a withdraw event after successful transfer
///
///-------------------------------------------------------------------------------
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut,
    has_one = vault_authority)]
    pub vault: Account<'info, Vault>, // Vault account storing balance info
    #[account(mut)]
    pub vault_authority: Signer<'info>, // User making the deposit
    pub system_program: Program<'info, System>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // TODO: Implement withdraw functionality
    if ctx.accounts.vault.locked {
        return err!(VaultError::VaultLocked);
    }

    let vault_balance = ctx.accounts.vault.get_lamports();
    if vault_balance < amount {
        return err!(VaultError::InsufficientBalance);
    }

    ctx.accounts.vault.sub_lamports(amount)?;
    ctx.accounts.vault_authority.add_lamports(amount)?;

    emit!(WithdrawEvent{
        amount,
        vault: ctx.accounts.vault.key(),
        vault_authority: ctx.accounts.vault_authority.key()
    });

    Ok(())
}
