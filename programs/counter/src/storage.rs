use anchor_lang::prelude::*;

use crate::inner_counter::Counter;

#[account]
pub struct Storage {
    pub counter: Counter,
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
