use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(mut)]
    pub initializer_token_acc: Account<'info, TokenAccount>,

    #[account(init,  payer = initializer , space = 200 )]
    pub escrow_account: Account<'info, EscrowAccount>,

    /// CHECK: PDA Signer
    pub vault_authority: UncheckedAccount<'info>,

    // 4️⃣ PDA-owned token vault (where tokens get locked)
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = vault_authority
    )]
    pub vault: Account<'info, TokenAccount>,

    // 5️⃣ Mint + Programs (standard boilerplate)
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct ClaimEscrow<'info> {
    #[account(mut, has_one = receiver)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(
        seeds = [b"vault", escrow_account.key().as_ref()],
        bump = escrow_account.bump
    )]
    /// CHECK: PDA Signer
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    /// CHECK: We only read the receiver's key; no data is accessed or modified
    pub receiver: AccountInfo<'info>,

    #[account(mut)]
    pub receiver_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct EscrowAccount {
    pub initializer: Pubkey, // person who created escrow
    pub receiver: Pubkey,    // who will receive tokens
    pub mint: Pubkey,        // which token mint
    pub amount: u64,         // how many tokens locked
    pub bump: u8,            // PDA bump for vault authority
}
