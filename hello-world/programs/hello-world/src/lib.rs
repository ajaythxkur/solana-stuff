use anchor_lang::prelude::*;

declare_id!("Fx9hw1mjUBqUJL2DXbpU5VM5N6AVPvnfv8Ho2WMu2V5Z");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Hello>) -> Result<()> {
        msg!("Hello world {}", ctx.program_id);
        msg!("Program id {}", &id());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}
