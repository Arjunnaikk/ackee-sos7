//-------------------------------------------------------------------------------
use crate::errors::VaultError;
use crate::events::ToggleLockEvent;
use crate::state::Vault;
///
/// TASK: Implement the toggle lock functionality for the on-chain vault
///
/// Requirements:
/// - Toggle the locked state of the vault (locked becomes unlocked, unlocked becomes locked)
/// - Only the vault authority should be able to toggle the lock
/// - Emit a toggle lock event after successful state change
///
///-------------------------------------------------------------------------------
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ToggleLock<'info> {
    // TODO: Add required accounts and constraints
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    #[account(mut,
    has_one= vault_authority)]
    pub vault: Account<'info, Vault>, 
    pub system_program: Program<'info, System>,
}

pub fn _toggle_lock(ctx: Context<ToggleLock>) -> Result<()> {
    // TODO: Implement toggle lock functionality

    let pointer = &mut ctx.accounts.vault;
    pointer.locked = !pointer.locked;

    emit!(ToggleLockEvent{
        locked: pointer.locked,
        vault: ctx.accounts.vault.key(),
        vault_authority: ctx.accounts.vault_authority.key()
    });

    Ok(())
}
