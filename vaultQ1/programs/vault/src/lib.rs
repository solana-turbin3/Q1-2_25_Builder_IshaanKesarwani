use anchor_lang::prelude::*;

declare_id!("4tRngai1omNPqiCePqBnoq94H8ELfpkNcPFzRTwnNtZp");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
