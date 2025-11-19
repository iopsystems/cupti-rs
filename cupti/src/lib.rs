//! CUpti.

#[macro_use]
mod macros;

pub mod activity;
pub mod callbacks;
pub mod checkpoint;
pub mod pmsampling;
pub mod profiler;
// pub mod pcsampling;
mod cuda;
mod driver_cbid;
mod error;
mod nvtx_cbid;
mod util;

pub use self::cuda::*;
pub use self::driver_cbid::DriverApiTraceCbid;
pub use self::error::Error;
pub use self::nvtx_cbid::NvtxApiTraceCbid;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub use self::callbacks::{SubscribeError, Subscriber, SubscriberCallbacks};

/// Initialize the profiler interface.
///
/// Loads the required libraries in the process address space and sets up the
/// hooks with the CUDA driver.
///
/// If you do not call this then most CUPTI methods will return
/// [`Error::NotInitialized`].
///
/// Returns an [`InitializeGuard`] that will deinitialize the profiler when it
/// goes out of scope.
pub fn initialize() -> Result<InitializeGuard> {
    use cupti_sys::{CUpti_Profiler_Initialize_Params, cuptiProfilerInitialize};

    let mut params = CUpti_Profiler_Initialize_Params::default();
    Error::result(unsafe { cuptiProfilerInitialize(&mut params) }).map(InitializeGuard)
}

/// Deinitialize the profiler interface.
///
/// Normally dropping [`InitializeGuard`] will take care of this for you.
pub fn deinitialize() -> Result<()> {
    use cupti_sys::*;

    let mut params = CUpti_Profiler_DeInitialize_Params::default();
    Error::result(unsafe { cuptiProfilerDeInitialize(&mut params) })
}

/// A owned wrapper around [`initialize`] and [`deinitialize`] that calls
/// [`deinitialize`] when it is dropped.
///
/// If you would like to managed the lifetime of the profiler yourself (or just
/// leave it initialized) then you can use [`std::mem::forget`] to prevent it
/// from being automatically deinitialized when this guard goes out of scope.
pub struct InitializeGuard(());

impl InitializeGuard {
    /// Explicitly deinitialize the profiler interface so you can get a result.
    pub fn deinitialize(self) -> Result<()> {
        std::mem::forget(self);
        deinitialize()
    }
}

impl Drop for InitializeGuard {
    fn drop(&mut self) {
        let _ = deinitialize();
    }
}
