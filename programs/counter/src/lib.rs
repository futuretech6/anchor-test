use anchor_lang::prelude::*;
use std::ops::DerefMut;

mod inner_counter;
mod instructions;
mod storage;

use crate::instructions::*;
use crate::storage::Storage;

declare_id!("Bz3bY4X3oPQsBoUr4B4LWHW2Jf3JYh2UwsAHhmdRN1bq");

#[program]
#[allow(clippy::result_large_err)]
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

    pub fn increase(ctx: Context<Increase>) -> Result<()> {
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

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Cannot get the bump.")]
    CannotGetBump,
}
