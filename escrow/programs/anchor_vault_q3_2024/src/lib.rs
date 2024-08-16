use anchor_lang::prelude::*;

declare_id!("5N33BWZRjgQhGCamDsx3pPsgBb5o54APDcvYCsTGQ1bp");

#[program]
pub mod fgg {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

}

pub struct VaultState {
    pub owner: Pubkey,
    pub amount: u64,
}