use anchor_lang::prelude::*;

declare_id!("3DUcXWwRwJD9mN271tTtq5XXFiW2C9WpdXtSe1u8odtt");

#[program]
pub mod smart_congrats {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
