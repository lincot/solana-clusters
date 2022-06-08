use anchor_lang::prelude::*;

const CLUSTER: &str = {
    #[cfg(feature = "devnet")]
    {
        "devnet"
    }
    #[cfg(feature = "localnet")]
    {
        "localnet"
    }
    #[cfg(feature = "mainnet")]
    {
        "mainnet"
    }
    #[cfg(feature = "testnet")]
    {
        "testnet"
    }
};

pub fn ping(_ctx: Context<Ping>) -> Result<()> {
    emit!(Pong {
        cluster: CLUSTER.into()
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Ping {}

#[event]
struct Pong {
    cluster: String,
}
