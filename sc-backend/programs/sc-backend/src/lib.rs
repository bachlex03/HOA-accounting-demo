use anchor_lang::prelude::*;

declare_id!("EKM7MoxuobaodaRqpUNQSB8yY54BgVK9Ldo2fpKhXtFA");

#[program]
pub mod sc_backend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
