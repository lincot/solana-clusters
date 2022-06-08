use anchor_lang::prelude::*;

pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
    msg!("I am on localnet!");
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize {}
