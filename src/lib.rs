#![no_std]

#[cfg(feature = "defmt")]
use defmt::info;
#[cfg(feature = "tracing")]
use tracing::info;

pub fn yeet() {
    info!("yeet");
}
