use anchor_lang::prelude::*;

declare_id!("BinYbLe2RXbVWvntQybnye37Lc5k7XT2XCaFLwAWD85h");

#[program]
pub mod anchor_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.program_id;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
