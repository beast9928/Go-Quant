use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("47ZVKmD5b2c5XRfrAtnenNirfgq8NYrnLy8bFpJqxm45");

#[program]
pub mod collateral_vault {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = ctx.accounts.user.key();
        vault.token_account = ctx.accounts.vault_token_account.key();
        vault.total_balance = 0;
        vault.locked_balance = 0;
        vault.available_balance = 0;
        vault.total_deposited = 0;
        vault.total_withdrawn = 0;
        vault.created_at = Clock::get()?.unix_timestamp;
        vault.bump = *ctx.bumps.get("vault").unwrap();
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user_token_account.to_account_info(),
                    to: ctx.accounts.vault_token_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            amount,
        )?;

        let vault = &mut ctx.accounts.vault;
        vault.total_balance += amount;
        vault.available_balance += amount;
        vault.total_deposited += amount;

        emit!(VaultEvent::Deposit {
            user: vault.owner,
            amount,
            balance: vault.total_balance,
        });

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.available_balance >= amount, ErrorCode::InsufficientBalance);

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault_token_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.vault.to_account_info(),
                },
                &[&[
                    b"vault",
                    vault.owner.as_ref(),
                    &[vault.bump],
                ]],
            ),
            amount,
        )?;

        vault.total_balance -= amount;
        vault.available_balance -= amount;
        vault.total_withdrawn += amount;

        emit!(VaultEvent::Withdraw {
            user: vault.owner,
            amount,
            balance: vault.total_balance,
        });

        Ok(())
    }

    pub fn lock_collateral(ctx: Context<LockUnlock>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.available_balance >= amount, ErrorCode::InsufficientBalance);

        vault.locked_balance += amount;
        vault.available_balance -= amount;

        emit!(VaultEvent::Lock { amount });
        Ok(())
    }

    pub fn unlock_collateral(ctx: Context<LockUnlock>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.locked_balance >= amount, ErrorCode::InsufficientBalance);

        vault.locked_balance -= amount;
        vault.available_balance += amount;

        emit!(VaultEvent::Unlock { amount });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"vault", user.key().as_ref()],
        bump,
        space = 8 + 128
    )]
    pub vault: Account<'info, CollateralVault>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub vault: Account<'info, CollateralVault>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub user: Signer<'info>,

    #[account(mut, has_one = owner)]
    pub vault: Account<'info, CollateralVault>,

    pub owner: Pubkey,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct LockUnlock<'info> {
    #[account(mut)]
    pub vault: Account<'info, CollateralVault>,
}

#[account]
pub struct CollateralVault {
    pub owner: Pubkey,
    pub token_account: Pubkey,
    pub total_balance: u64,
    pub locked_balance: u64,
    pub available_balance: u64,
    pub total_deposited: u64,
    pub total_withdrawn: u64,
    pub created_at: i64,
    pub bump: u8,
}

#[event]
pub enum VaultEvent {
    Deposit { user: Pubkey, amount: u64, balance: u64 },
    Withdraw { user: Pubkey, amount: u64, balance: u64 },
    Lock { amount: u64 },
    Unlock { amount: u64 },
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Insufficient balance")]
    InsufficientBalance,
}
