use anchor_lang::prelude::*;

declare_id!("D1NS7CDd3KButkwanYJL1cwHep1ZGeR6r1qEE3LvnZVE");

#[program]
pub mod solana_number_store {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, number: u64) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        data_account.authority = *ctx.accounts.authority.key;
        data_account.number = number;
        Ok(())
    }

    pub fn update_number(ctx: Context<UpdateNumber>, new_number: u64) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        require_keys_eq!(
            data_account.authority,
            ctx.accounts.authority.key(),
            CustomError::Unauthorized
        );
        data_account.number = new_number;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 32)]
    pub data_account: Account<'info, NumberAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateNumber<'info> {
    #[account(mut)]
    pub data_account: Account<'info, NumberAccount>,
    pub authority: Signer<'info>,
}

#[account]
pub struct NumberAccount {
    pub number: u64,
    pub authority: Pubkey,
}

#[error_code]
pub enum CustomError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
