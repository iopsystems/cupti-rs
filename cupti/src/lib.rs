//! CUpti.

pub mod activity;
pub mod callbacks;
pub mod checkpoint;
mod cuda;
mod driver_cbid;
mod error;
mod util;

pub use self::cuda::*;
pub use self::driver_cbid::DriverApiTraceCbid;
pub use self::error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub use self::callbacks::{SubscribeError, Subscriber, SubscriberCallbacks};
