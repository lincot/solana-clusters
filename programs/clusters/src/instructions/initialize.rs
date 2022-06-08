#[cfg(feature = "devnet")]
pub use crate::instructions::initialize::devnet::*;
#[cfg(feature = "localnet")]
pub use crate::instructions::initialize::localnet::*;
#[cfg(feature = "mainnet")]
pub use crate::instructions::initialize::mainnet::*;
#[cfg(feature = "testnet")]
pub use crate::instructions::initialize::testnet::*;

pub mod devnet;
pub mod localnet;
pub mod mainnet;
pub mod testnet;
