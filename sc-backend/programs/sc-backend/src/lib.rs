use anchor_lang::prelude::*;

pub mod modules;
pub mod constants;
use crate::modules::*;

declare_id!("EKM7MoxuobaodaRqpUNQSB8yY54BgVK9Ldo2fpKhXtFA");

#[program]
pub mod sc_backend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        user::services::initialize_user();
        Ok(())

    }
}

#[derive(Accounts)]
pub struct Initialize {}
