//! CUpti.

mod callbacks;
mod cuda;
mod error;
mod util;

pub use self::callbacks::*;
pub use self::cuda::*;
pub use self::error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;
