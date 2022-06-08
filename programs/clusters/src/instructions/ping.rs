use anchor_lang::prelude::*;

const N: u16 = {
    #[cfg(feature = "devnet")]
    {
        1
    }
    #[cfg(feature = "localnet")]
    {
        2
    }
    #[cfg(feature = "mainnet")]
    {
        3
    }
    #[cfg(feature = "testnet")]
    {
        4
    }
};

pub fn ping(_ctx: Context<Ping>) -> Result<()> {
    emit!(Pong { n: N });

    Ok(())
}

#[derive(Accounts)]
pub struct Ping {}

#[event]
struct Pong {
    n: u16,
}
