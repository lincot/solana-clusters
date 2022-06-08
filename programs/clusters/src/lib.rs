use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod instructions;

declare_id!("AVXRrXcVdRVH3WejLEUp2PkZvxBvjbvoK72aTeLJ6n93");

#[program]
pub mod clusters {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn ping(ctx: Context<Ping>) -> Result<()> {
        instructions::ping(ctx)
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
compile_error!("one of `localnet`, `devnet`, `testnet`, `mainnet` must be enabled as feature");
