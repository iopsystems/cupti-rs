use std::cell::UnsafeCell;

use cuda_sys::cuda::{CUstream, CUstream_st};
use cupti_sys::*;

use crate::*;

/// A reference to a CUDA stream ([`CUstream`]).
///
/// This is a thin wrapper that only exposes the relevant CUPTI functions.
#[repr(transparent)]
pub struct Stream(UnsafeCell<CUstream_st>);

impl Stream {
    /// Create a [`Stream`] from a [`CUstream`] reference.
    pub fn from_ref(context: &CUstream_st) -> &Self {
        unsafe { &*(context as *const _ as *const _) }
    }

    /// Create a [`Stream`] from a mutable [`CUstream`] reference.
    pub fn from_mut(context: &mut CUstream_st) -> &mut Self {
        unsafe { &mut *(context as *mut _ as *mut _) }
    }

    /// Create a [`Stream`] directly from a pointer.
    ///
    /// # Safety
    /// `ptr` must either be null or a pointer to a valid context.
    pub unsafe fn from_ptr<'a>(ptr: *const CUstream_st) -> Option<&'a Self> {
        if !ptr.is_null() {
            Some(Self::from_ref(unsafe { &*ptr }))
        } else {
            None
        }
    }

    pub fn as_raw(&self) -> CUstream {
        &self.0 as *const _ as *mut _
    }

    /// Get the ID of this stream.
    ///
    /// The returned ID will be unique within the stream's context.
    pub fn id(&self, per_thread_stream: bool) -> Result<u32> {
        let mut id = 0;
        let code = unsafe {
            cuptiGetStreamIdEx(
                std::ptr::null_mut(),
                self.as_raw(),
                per_thread_stream.into(),
                &mut id,
            )
        };

        Error::result(code).map(|_| id)
    }
}
