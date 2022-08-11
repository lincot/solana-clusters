#[cfg(feature = "localnet")]
use crate::state_localnet::*;
#[cfg(not(feature = "localnet"))]
use crate::state_othernet::*;
use anchor_lang::prelude::*;

pub fn different_state(ctx: Context<DifferentState>) -> Result<()> {
    msg!("{:?}", ctx.accounts.my_acc);

    Ok(())
}

#[derive(Accounts)]
pub struct DifferentState<'info> {
    #[account(init, payer = payer, space = 8 + MyAcc::LEN)]
    my_acc: Account<'info, MyAcc>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}
