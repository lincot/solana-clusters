use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod instructions;
mod state_localnet;
mod state_othernet;

declare_id!("EwsSbcPDTpuoLaf5zSwHWTdY4CJmc7zodDMpKqV29R9D");

#[program]
pub mod clusters {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn ping(ctx: Context<Ping>) -> Result<()> {
        instructions::ping(ctx)
    }

    pub fn different_state(ctx: Context<DifferentState>) -> Result<()> {
        instructions::different_state(ctx)
    }
}

#[cfg(any(
    all(
        not(feature = "devnet"),
        not(feature = "localnet"),
        not(feature = "mainnet"),
        not(feature = "testnet"),
    ),
    all(feature = "devnet", feature = "localnet"),
    all(feature = "devnet", feature = "mainnet"),
    all(feature = "devnet", feature = "testnet"),
    all(feature = "localnet", feature = "mainnet"),
    all(feature = "localnet", feature = "testnet"),
    all(feature = "mainnet", feature = "testnet"),
))]
compile_error!("one of `devnet`, `localnet`, `mainnet`, `testnet` must be enabled as feature");
