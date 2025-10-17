use anchor_lang::prelude::*;
mod account;
use account::*;

declare_id!("GcDx5UJ9NPhZeANPUJweZrbNAYpvpzaiJ3JLnLPgM3qr");

#[program]
pub mod escrow {
    use anchor_spl::token::{self, transfer, Mint, Token, TokenAccount, Transfer};

    use super::*;
    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        amount: u64,
        reciever: Pubkey,
    ) -> Result<()> {
        // 1️⃣ Derive PDA vault authority
        let (vault_authority, bump) = Pubkey::find_program_address(
            &[b"vault", ctx.accounts.escrow_account.key().as_ref()],
            ctx.program_id,
        );

        // 2️⃣ Move tokens from initializer → vault
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.initializer_token_acc.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.initializer.to_account_info(),
            },
        );

        token::transfer(cpi_ctx, amount)?;

        // writing data in the escrow account
        let escrow = &mut ctx.accounts.escrow_account;
        escrow.initializer = ctx.accounts.initializer.key();
        escrow.receiver = reciever;
        escrow.mint = ctx.accounts.mint.key();
        escrow.amount = amount;
        escrow.bump = bump;

        Ok(())
    }

    pub fn claim_escrow(ctx: Context<ClaimEscrow>) -> Result<()> {
        let escrow_key = ctx.accounts.escrow_account.key();
        let bump = ctx.accounts.escrow_account.bump;
        let seeds = &[b"vault", escrow_key.as_ref(), &[bump]];
        let signer = &[&seeds[..]];


        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.receiver_token_account.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            signer,
        );

        let _ = transfer(cpi_ctx, ctx.accounts.escrow_account.amount);

        Ok(())
    }
}
