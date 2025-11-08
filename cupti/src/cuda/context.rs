use cuda_sys::cuda::{CUcontext, CUctx_st};
use cupti_sys::*;

use crate::*;

/// A reference to a CUDA context ([`CUcontext`]).
///
/// This is a thin wrapper that only exposes the relevant CUPTI functions.
#[repr(transparent)]
pub struct Context(CUctx_st);

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
}
