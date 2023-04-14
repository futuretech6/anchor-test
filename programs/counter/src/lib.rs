use anchor_lang::prelude::*;
use std::ops::DerefMut;

declare_id!("Bz3bY4X3oPQsBoUr4B4LWHW2Jf3JYh2UwsAHhmdRN1bq");

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Counter {
    value: i128,
}

impl Counter {
    pub const SIZE: usize = 8 + 16;

    pub fn new(value: i128) -> Self {
        Counter { value }
    }
    pub fn inc(&mut self) {
        self.value += 1;
    }
    pub fn dec(&mut self) {
        self.value -= 1;
    }
    pub fn show(&self) -> i128 {
        self.value
    }
}

#[account]
pub struct Storage {
    counter: Counter,
    pub authority: Pubkey,
    pub bump: u8,
}

impl Storage {
    pub const SIZE: usize = 8 + Counter::SIZE + 32 + 1;

    pub fn new(value: i128, authority: Pubkey, bump: u8) -> Self {
        Storage {
            counter: Counter::new(value),
            authority,
            bump,
        }
    }
}

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, init_value: i128) -> Result<()> {
        let storage = ctx.accounts.storage.deref_mut();
        *storage = Storage::new(
            init_value,
            *ctx.accounts.authority.key,
            *ctx.bumps.get("storage").ok_or(ErrorCode::CannotGetBump)?,
        );

        Ok(())
    }

    pub fn increse(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.storage.counter.inc();
        msg!("counter value: {}", ctx.accounts.storage.counter.show());
        Ok(())
    }
    pub fn decrease(ctx: Context<Decrease>) -> Result<()> {
        ctx.accounts.storage.counter.dec();
        msg!("counter value: {}", ctx.accounts.storage.counter.show());
        Ok(())
    }
    pub fn show(ctx: Context<Show>) -> Result<()> {
        msg!("counter value: {}", ctx.accounts.storage.counter.show());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = Storage::SIZE, seeds = [b"storage"], bump)]
    pub storage: Account<'info, Storage>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    storage: Account<'info, Storage>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Decrease<'info> {
    #[account(mut)]
    storage: Account<'info, Storage>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Show<'info> {
    #[account(mut)]
    storage: Account<'info, Storage>,
    authority: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Cannot get the bump.")]
    CannotGetBump,
}
