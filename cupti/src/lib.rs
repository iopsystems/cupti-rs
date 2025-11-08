//! CUpti.

pub mod activity;
pub mod callbacks;
mod cuda;
mod error;
mod util;

pub use self::cuda::*;
pub use self::error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;
