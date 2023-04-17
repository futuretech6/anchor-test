use anchor_lang::prelude::*;

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
