use std::ffi::CStr;
use std::fmt;
use std::num::NonZeroU32;

use cupti_sys::*;

macro_rules! error_enum
{
    {
        $( #[$attr:meta] )*
        pub enum $name:ident {
            $(
                $( #[$fattr:meta] )*
                $variant:ident = $value:expr
            ),* $(,)?
        }
    } => {
        $(#[$attr])*
        pub struct $name(NonZeroU32);

        #[allow(non_upper_case_globals)]
        impl $name {
            $(
                $( #[$fattr] )*
                pub const $variant: Self = Self(NonZeroU32::new($value).unwrap());
            )*
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(match *self {
                    $( Self::$variant => stringify!($variant), )*
                    _ => return f.debug_tuple(stringify!($name)).field(&self.0).finish()
                })
            }
        }
    }
}

error_enum! {
    /// Errors that can be returned from CUPTI functions.
    ///
    /// This is meant to be usable like an enum, but it is possible for CUPTI
    /// functions to return error codes not listed here (either due to internal
    /// errors or a new version).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub enum Error {
        /// One or more of the parameters is invalid.
        InvalidParameter = CUPTI_ERROR_INVALID_PARAMETER,

        /// The device does not correspond to a valid CUDA device.
        InvalidDevice = CUPTI_ERROR_INVALID_DEVICE,

        /// The context is NULL or not valid.
        InvalidContext = CUPTI_ERROR_INVALID_CONTEXT,

        /// The event domain id is invalid.
        InvalidEventDomainID = CUPTI_ERROR_INVALID_EVENT_DOMAIN_ID,

        /// The event id is invalid.
        InvalidEventID = CUPTI_ERROR_INVALID_EVENT_ID,

        /// The event name is invalid.
        InvalidEventName = CUPTI_ERROR_INVALID_EVENT_NAME,

        /// The current operation cannot be performed due to a dependency on
        /// other features.
        InvalidOperation = CUPTI_ERROR_INVALID_OPERATION,

        /// Unable to allocate enough memory to perform the requested operation.
        OutOfMemory = CUPTI_ERROR_OUT_OF_MEMORY,

        /// An error occurred on the performance monitoring hardware.
        Hardware = CUPTI_ERROR_HARDWARE,

        /// The output buffer size is not sufficient to return all requested data.
        ParameterSizeNotSufficient = CUPTI_ERROR_PARAMETER_SIZE_NOT_SUFFICIENT,

        /// API is not implemented.
        ApiNotImplemented = CUPTI_ERROR_API_NOT_IMPLEMENTED,

        /// The maximum limit is reached.
        MaxLimitReached = CUPTI_ERROR_MAX_LIMIT_REACHED,

        //// The object is not yet ready to perform the requested operation.
        NotReady = CUPTI_ERROR_NOT_READY,

        /// The current operation is not compatible with the current state of
        /// the object.
        NotCompatible = CUPTI_ERROR_NOT_COMPATIBLE,

        /// CUPTI is unable to initialize its connection to the CUDA driver.
        NotInitialized = CUPTI_ERROR_NOT_INITIALIZED,

        /// The metric ID is invalid.
        InvalidMetricID = CUPTI_ERROR_INVALID_METRIC_ID,

        /// The metric name is invalid.
        InvalidMetricName = CUPTI_ERROR_INVALID_METRIC_NAME,

        /// The queue is empty.
        QueueEmpty = CUPTI_ERROR_QUEUE_EMPTY,

        /// Invalid handle.
        InvalidHandle = CUPTI_ERROR_INVALID_HANDLE,

        /// Invalid stream.
        InvalidStream = CUPTI_ERROR_INVALID_STREAM,

        /// Invalid kind.
        InvalidKind = CUPTI_ERROR_INVALID_KIND,

        /// Invalid event value.
        InvalidEventValue = CUPTI_ERROR_INVALID_EVENT_VALUE,

        /// CUPTI is disabled due to conflicts with other enabled profilers.
        Disabled = CUPTI_ERROR_DISABLED,

        /// Invalid module.
        InvalidModule = CUPTI_ERROR_INVALID_MODULE,

        /// Invalid metric value.
        InvalidMetricValue = CUPTI_ERROR_INVALID_METRIC_VALUE,

        /// The performance monitoring hardware is in use by other client.
        HardwareBusy = CUPTI_ERROR_HARDWARE_BUSY,

        /// The attempted operation is not supported on the current system or
        /// device.
        NotSupported = CUPTI_ERROR_NOT_SUPPORTED,

        /// Unified memory profiling is not supported on the system. Potential
        /// reason could be unsupported OS or architecture.
        UmProfilingNotSupported = CUPTI_ERROR_UM_PROFILING_NOT_SUPPORTED,

        /// Unified memory profiling is not supported on the device.
        UmProfilingNotSupportedOnDevice = CUPTI_ERROR_UM_PROFILING_NOT_SUPPORTED_ON_DEVICE,

        /// Unified memory profiling is not supported on a multi-GPU
        /// configuration without P2P support between any pair of devices.
        UmProfilingNotSupportedOnNonP2PDevices = CUPTI_ERROR_UM_PROFILING_NOT_SUPPORTED_ON_NON_P2P_DEVICES,

        /// Unified memory profiling is not supported under the Multi-Process
        /// Service (MPS) environment. CUDA 7.5 removes this restriction.
        UmProfilingNotSupportedWithMps = CUPTI_ERROR_UM_PROFILING_NOT_SUPPORTED_WITH_MPS,

        /// In CUDA 9.0, devices with compute capability 7.0 don't support CDP
        /// tracing.
        CdpTracingNotSupported = CUPTI_ERROR_CDP_TRACING_NOT_SUPPORTED,

        /// Profiling on virtualized GPU is not supported.
        VirtualizedDeviceNotSupported = CUPTI_ERROR_VIRTUALIZED_DEVICE_NOT_SUPPORTED,

        /// Profiling results might be incorrect for CUDA applications compiled
        /// with nvcc version older than 9.0 for devices with compute capability
        /// 6.0 and 6.1.
        CudaCompilerNotCompatible = CUPTI_ERROR_CUDA_COMPILER_NOT_COMPATIBLE,

        /// User doesn't have sufficient privileges which are required to start
        /// the profiling session.
        InsufficientPrivileges = CUPTI_ERROR_INSUFFICIENT_PRIVILEGES,

        /// Legacy CUPTI Profiling API i.e. event API from the header
        /// cupti_events.h and metric API from the header cupti_metrics.h are
        /// not compatible with the Profiling API in the header
        /// cupti_profiler_target.h and Perfworks metrics API in the headers
        /// nvperf_host.h and nvperf_target.h.
        OldProfilerApiInitialized = CUPTI_ERROR_OLD_PROFILER_API_INITIALIZED,

        /// Missing definition of the OpenACC API routine in the linked OpenACC
        /// library.
        OpenaccUndefinedRoutine = CUPTI_ERROR_OPENACC_UNDEFINED_ROUTINE,

        /// Legacy CUPTI Profiling API i.e. event API from the header
        /// cupti_events.h and metric API from the header cupti_metrics.h are
        /// not supported on devices with compute capability 7.5 and higher.
        LegacyProfilerNotSupported = CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED,

        /// CUPTI doesn't allow multiple callback subscribers. Only a single
        /// subscriber can be registered at a time.
        MultipleSubscribersNotSupported = CUPTI_ERROR_MULTIPLE_SUBSCRIBERS_NOT_SUPPORTED,

        /// Profiling on virtualized GPU is not allowed by hypervisor.
        VirtualizedDeviceInsufficientPrivileges = CUPTI_ERROR_VIRTUALIZED_DEVICE_INSUFFICIENT_PRIVILEGES,

        /// Profiling and tracing are not allowed when confidential computing
        /// mode is enabled.
        ConfidentialComputingNotSupported = CUPTI_ERROR_CONFIDENTIAL_COMPUTING_NOT_SUPPORTED,

        /// CUPTI does not support NVIDIA Crypto Mining Processors (CMP).
        CmpDeviceNotSupported = CUPTI_ERROR_CMP_DEVICE_NOT_SUPPORTED,

        /// Profiling on Multi-instance GPU (MIG) is not supported.
        MigDeviceNotSupported = CUPTI_ERROR_MIG_DEVICE_NOT_SUPPORTED,

        /// Profiling on SLI device is not supported.
        SliDeviceNotSupported = CUPTI_ERROR_SLI_DEVICE_NOT_SUPPORTED,

        /// Profiling on WSL device is not supported.
        WslDeviceNotSupported = CUPTI_ERROR_WSL_DEVICE_NOT_SUPPORTED,

        /// For invalid or unsupported chip name passed to
        /// cuptiProfilerHostInitialize.
        InvalidChipName = CUPTI_ERROR_INVALID_CHIP_NAME,

        /// An unknown internal error has occurred.
        Unknown = CUPTI_ERROR_UNKNOWN,
    }
}

impl Error {
    /// Create a new error object from an error code.
    pub const fn new(code: u32) -> Option<Self> {
        match NonZeroU32::new(code) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Create an error result directly from an error code.
    pub const fn result(code: u32) -> Result<(), Error> {
        match Self::new(code) {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    /// Get the underlying error code for this error.
    pub const fn code(self) -> u32 {
        self.0.get()
    }

    /// Get a string containing CUPTI's name for this error code.
    ///
    /// Returns none if this error code does not have a known name.
    pub fn result_string(self) -> Option<&'static CStr> {
        unsafe {
            let mut s = std::ptr::null();
            if Self::result(cuptiGetResultString(self.code(), &mut s)).is_err() {
                return None;
            }

            if s.is_null() {
                return None;
            }

            Some(CStr::from_ptr(s))
        }
    }

    /// Get a descriptive message for this error.
    ///
    /// Returns `None` if there is no descriptive error message string for this
    /// error.
    pub fn message(self) -> Option<&'static CStr> {
        unsafe {
            let mut s = std::ptr::null();
            if Self::result(cuptiGetErrorMessage(self.code(), &mut s)).is_err() {
                return None;
            }

            if s.is_null() {
                return None;
            }

            Some(CStr::from_ptr(s))
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.message().and_then(|s| s.to_str().ok()) {
            Some(s) => f.write_str(s),
            None => write!(f, "CUPTI error code {}", self.code()),
        }
    }
}

impl std::error::Error for Error {}
