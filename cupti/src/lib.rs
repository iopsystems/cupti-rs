//! CUpti.

#[macro_use]
mod macros;

pub mod activity;
pub mod callbacks;
pub mod checkpoint;
pub mod pmsampling;
pub mod profiler;
pub mod rangeprofiling;
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
pub use self::util::{CStringList, CStringSlice};

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

/// Get the chip name for a CUDA device.
///
/// Returns the chip name (e.g., "ga100", "gv100") for the device at the given index.
///
/// # Parameters
///
/// - `device_index`: The index of the CUDA device
///
/// # Errors
///
/// - [`Error::InvalidParameter`] if `device_index` is invalid
pub fn get_device_chip_name(device_index: usize) -> Result<&'static str> {
    use cupti_sys::{CUpti_Device_GetChipName_Params, cuptiDeviceGetChipName};
    use std::ffi::CStr;

    let mut params = CUpti_Device_GetChipName_Params::default();
    params.deviceIndex = device_index;

    Error::result(unsafe { cuptiDeviceGetChipName(&mut params) })?;

    let chip_name = unsafe { CStr::from_ptr(params.pChipName) };
    Ok(chip_name.to_str().expect("chip name should be valid UTF-8"))
}
