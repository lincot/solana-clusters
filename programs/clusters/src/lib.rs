use anchor_lang::prelude::*;

declare_id!("Gf3bXGS7iA2EUxzXs1xS6qwZBPGS8idyqMNitQ5NKDSA");

#[cfg(any(
    all(
        not(feature = "localnet"),
        not(feature = "devnet"),
        not(feature = "testnet"),
        not(feature = "mainnet"),
    ),
    all(feature = "localnet", feature = "devnet"),
    all(feature = "localnet", feature = "testnet"),
    all(feature = "localnet", feature = "mainnet"),
    all(feature = "devnet", feature = "testnet"),
    all(feature = "devnet", feature = "mainnet"),
    all(feature = "testnet", feature = "mainnet"),
))]
compile_error!("one of `localnet`, `devnet`, `testnet`, `mainnet` must be enabled as feature");

const CLUSTER: &str = {
    #[cfg(feature = "localnet")]
    {
        "localnet"
    }
    #[cfg(feature = "devnet")]
    {
        "devnet"
    }
    #[cfg(feature = "testnet")]
    {
        "testnet"
    }
    #[cfg(feature = "mainnet")]
    {
        "mainnet"
    }
};

#[program]
pub mod clusters {
    use super::*;

    pub fn ping(_ctx: Context<Ping>) -> Result<()> {
        emit!(Pong {
            cluster: CLUSTER.into()
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ping {}

#[event]
struct Pong {
    cluster: String,
}
