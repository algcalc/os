#[cfg(feature = "defmt")]
pub(crate) use defmt::{trace, debug, info, warn, error};
#[cfg(feature = "tracing")]
pub(crate) use tracing::{trace, debug, info, warn, error};
