use anchor_lang::prelude::*;

declare_id!("BinYbLe2RXbVWvntQybnye37Lc5k7XT2XCaFLwAWD85h");

#[program]
#[allow(clippy::result_large_err)]
pub mod anchor_test {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
