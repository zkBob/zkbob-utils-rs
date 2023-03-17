use libzeropool::{native::params::PoolBN256, fawkes_crypto::{backend::bellman_groth16::engines::Bn256, engines::bn256}};
pub use tracing;

pub mod configuration;
pub mod contracts;
pub mod telemetry;
pub mod relayer;

pub type PoolParams = PoolBN256;
pub type Engine = Bn256;
pub type Fr = bn256::Fr;