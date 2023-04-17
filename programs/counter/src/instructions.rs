use anchor_lang::prelude::*;

use crate::storage::Storage;

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = Storage::SIZE, seeds = [b"storage"], bump)]
    pub storage: Account<'info, Storage>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct Increase<'info> {
    #[account(mut)]
    pub storage: Account<'info, Storage>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction()]
pub struct Decrease<'info> {
    #[account(mut)]
    pub storage: Account<'info, Storage>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction()]
pub struct Show<'info> {
    #[account(mut)]
    pub storage: Account<'info, Storage>,
    authority: Signer<'info>,
}
