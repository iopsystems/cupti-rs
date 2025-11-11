use std::cell::UnsafeCell;

use cuda_sys::cuda::{CUcontext, CUctx_st};
use cupti_sys::*;

use crate::activity::ActivityKind;
use crate::*;

/// A reference to a CUDA context ([`CUcontext`]).
///
/// This is a thin wrapper that only exposes the relevant CUPTI functions.
#[repr(transparent)]
pub struct Context(UnsafeCell<CUctx_st>);

impl Context {
    /// Create a [`Context`] from a [`CUcontext`] reference.
    pub fn from_ref(context: &CUctx_st) -> &Self {
        unsafe { &*(context as *const _ as *const _) }
    }

    /// Create a [`Context`] from a mutable [`CUcontext`] reference.
    pub fn from_mut(context: &mut CUctx_st) -> &mut Self {
        unsafe { &mut *(context as *mut _ as *mut _) }
    }

    /// Create a [`Context`] directly from a pointer.
    ///
    /// # Safety
    /// `ptr` must either be null or a pointer to a valid context.
    pub unsafe fn from_ptr<'a>(ptr: *const CUctx_st) -> Option<&'a Self> {
        if !ptr.is_null() {
            Some(Self::from_ref(unsafe { &*ptr }))
        } else {
            None
        }
    }

    pub fn as_raw(&self) -> CUcontext {
        &self.0 as *const _ as *mut _
    }

    /// Get the ID of this context.
    pub fn id(&self) -> Result<u32> {
        let mut id = 0;
        let code = unsafe { cuptiGetContextId(self.as_raw(), &mut id) };
        Error::result(code).map(|_| id)
    }

    /// Get the id of a stream within this context.
    ///
    /// The returned ID will be unique within this context. Unlike
    /// [`Stream::id`] this will also validate that the stream belongs to this
    /// context.
    ///
    /// # Errors
    /// * [`Error::NotInitialized`]
    /// * [`Error::InvalidStream`] if enable to get the stream ID, or if
    ///   `stream` does not belong to this context.
    pub fn get_stream_id(&self, stream: &Stream, per_thread_stream: bool) -> Result<u32> {
        let mut id = 0;
        let code = unsafe {
            cuptiGetStreamIdEx(
                self.as_raw(),
                stream.as_raw(),
                per_thread_stream.into(),
                &mut id,
            )
        };

        Error::result(code).map(|_| id)
    }

    /// Get the ID of the device that contains this context.
    ///
    /// This works similar to to `cudaGetDevice()` or `cuCtxGetDevice()` but
    /// may be called from within callback functions.
    ///
    /// # Errors
    /// * [`Error::NotInitialized`]
    /// * [`Error::InvalidDevice`] is unable to get the device ID.
    pub fn device_id(&self) -> Result<u32> {
        let mut id = 0;
        let code = unsafe { cuptiGetDeviceId(self.as_raw(), &mut id) };

        Error::result(code).map(|_| id)
    }

    /// Enable collection of a specific kind of activity record for this
    /// context.
    ///
    /// This will supersede the global settings for activity records enabled by
    /// [`activity::enable`]. Multiple kinds can be enabled by calling this
    /// function multiple times with different [`ActivityKind`]s.
    ///
    /// # Parameters
    /// - `kind`: The kind of activity record to collect.
    ///
    /// # Errors
    /// - [`Error::NotInitialized`]
    /// - [`Error::NotCompatible`] if the activity kind cannot be enabled
    /// - [`Error::InvalidKind`] if the activity kind is not supported
    pub fn enable(&self, kind: ActivityKind) -> Result<()> {
        crate::activity::enable_context(self, kind)
    }

    /// Disable collection of a specific kind of activity record for this
    /// context.
    ///
    /// This will supersede the global settings for activity records. Multiple
    /// kinds can be enabled by calling this function multiple times.
    ///
    /// # Parameters
    /// - `kind`: The kind of acivity record to stop collecting.
    ///
    /// # Errors
    /// - [`Error::NotInitialized`]
    /// - [`Error::InvalidKind`] if the activity kind is not supported
    pub fn disable(&self, kind: ActivityKind) -> Result<()> {
        crate::activity::disable_context(self, kind)
    }

    /// Get the number of activity records that were dropped due to insufficient
    /// buffer space.
    ///
    /// The dropped count includes record that could not be recorded because
    /// CUPTI did not have activity buffer space available for the record
    /// (because the buffer request callback did not return an empty buffer of
    /// sufficient size) and also CDB records that could not be recorded because
    /// the device-size buffer was ful (size is controlled by the
    /// [`ActivityAttribute::DeviceBufferSizeCDP`][attr] attribute). The dropped
    /// count maintained for the queue is reset to zero when this function is
    /// called.
    /// 
    /// # Parameters
    ///
    /// [attr]: crate::activity::ActivityAttribute::DeviceBufferSizeCDP
    pub fn get_num_dropped_records(&self, stream_id: u32) -> Result<usize> {
        crate::activity::get_num_dropped_records(Some(self), stream_id)
    }
}
