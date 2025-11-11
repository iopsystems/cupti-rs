use c_enum::c_enum;
use cupti_sys::*;

use crate::{Context, Error, Result};

c_enum! {
    /// Optimization flags for CUPTI checkpoints.
    #[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
    pub enum CheckpointOptimizations : CUpti_CheckpointOptimizations {
        /// Default behavior.
        None = CUPTI_CHECKPOINT_OPT_NONE as _,

        /// Determine which memory blocks have changed, and only restore those.
        ///
        /// # Notes
        ///
        /// This optimization is cached, which means [`cuptiCheckpointRestore`] must always be called
        /// at the same point in the application when this option is enabled, or the result may be incorrect.
        Transfer = CUPTI_CHECKPOINT_OPT_TRANSFER as _,
    }
}

/// Configuration for a CUPTI checkpoint.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CheckpointOptions {
    /// Restrict the checkpoint from using the last N MB of device memory
    /// (-1 = use no device memory).
    pub reserve_device_mb: usize,

    /// Restrict the checkpoint from using the alst N MB of host memory
    /// (-1 = use no host memory).
    pub reserve_host_mb: usize,

    /// Allow the checkpoint to save over an existing checkpoint.
    pub allow_overwrite: bool,

    /// Mask of [`CheckpointOptimizations`] flags for this checkpoint.
    ///
    /// Note that the underlying field in [`CUpti_Checkpoint`] is a `u8`,
    /// while [`CUpti_CheckpointOptimizations`] is a `u32`. Values larger than
    /// can be stored in a `u8` will be truncated.
    pub optimizations: CheckpointOptimizations,
}

impl Default for CheckpointOptions {
    fn default() -> Self {
        Self {
            reserve_device_mb: 0,
            reserve_host_mb: 0,
            allow_overwrite: false,
            optimizations: CheckpointOptimizations::None,
        }
    }
}

/// Checkpoints allow you to save the state of the CPU and then restore it back
/// to that state at a later point in time.
///
/// This allows you to replay CUDA kernel executions with guaranteed
/// reproducibility.
///
/// Checkpoint data can be stored in device, host, and filesystem space. You can
/// configure [`CheckpointOptions`] to guarantee that a certain amount of memory
/// will remain free for use after the checkpoint is saved. However, falling
/// back to slower levels of memory (host, and then filesystem) to save the
/// checkpoint will be significantly slower.
pub struct Checkpoint {
    raw: CUpti_Checkpoint,
}

impl Checkpoint {
    const CHECKPOINT_SIZE: usize = std::mem::offset_of!(CUpti_Checkpoint, pPriv);

    fn save_impl(context: Option<&Context>, options: CheckpointOptions) -> Result<Self> {
        let mut checkpoint = CUpti_Checkpoint {
            structSize: Self::CHECKPOINT_SIZE,
            ctx: context.map(|c| c.as_raw()).unwrap_or(std::ptr::null_mut()),
            pPriv: std::ptr::null_mut(),

            reserveDeviceMB: options.reserve_device_mb,
            reserveHostMB: options.reserve_host_mb,
            allowOverwrite: options.allow_overwrite.into(),
            optimizations: options.optimizations.0 as _,
        };

        Error::result(unsafe { cuptiCheckpointSave(&mut checkpoint) })?;

        Ok(Self { raw: checkpoint })
    }

    /// Initialize and save a checkpoint of the device state associated with the
    /// current context.
    ///
    /// Uses the provided options to configure and save a checkpoint of the
    /// device state.
    ///
    /// # Parameters
    ///
    /// - `options`: Configuration options for the checkpoint
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if the handle does not appear to refer to
    ///   a valid checkpoint
    /// - [`Error::InvalidContext`]
    /// - [`Error::InvalidDevice`] if device associated with context is not
    ///   compatible with checkpoint API
    /// - [`Error::InvalidOperation`] if save is requested over an existing
    ///   checkpoint, but `allow_overwrite` was not originally specified
    /// - [`Error::OutOfMemory`] if as configured, not enough backing storage
    ///   space to save the checkpoint
    pub fn save(options: CheckpointOptions) -> Result<Self> {
        Self::save_impl(None, options)
    }

    /// Initialize and save a checkpoint of the device state associated with the
    /// specified context.
    ///
    /// Uses the provided options to configure and save a checkpoint of the
    /// device state associated with the specified context.
    ///
    /// # Parameters
    ///
    /// - `context`: The CUDA context to save from
    /// - `options`: Configuration options for the checkpoint
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if the handle does not appear to refer to
    ///   a valid checkpoint
    /// - [`Error::InvalidContext`]
    /// - [`Error::InvalidDevice`] if device associated with context is not
    ///   compatible with checkpoint API
    /// - [`Error::InvalidOperation`] if save is requested over an existing
    ///   checkpoint, but `allow_overwrite` was not originally specified
    /// - [`Error::OutOfMemory`] if as configured, not enough backing storage
    ///   space to save the checkpoint
    pub fn save_context(context: &Context, options: CheckpointOptions) -> Result<Self> {
        Self::save_impl(Some(context), options)
    }

    /// Restore a checkpoint to the device associated with its context.
    ///
    /// Restores device, pinned, and allocated memory to the state when the
    /// checkpoint was saved.
    ///
    /// # Errors
    ///
    /// - [`Error::NotInitialized`] if the checkpoint was not previously
    ///   initialized
    /// - [`Error::InvalidContext`]
    /// - [`Error::InvalidParameter`] if the handle appears invalid
    /// - [`Error::Unknown`] if the restore or optimization operation fails
    pub fn restore(&self) -> Result<()> {
        Error::result(unsafe { cuptiCheckpointRestore(&self.raw as *const _ as *mut _) })
    }
}

impl Drop for Checkpoint {
    fn drop(&mut self) {
        // Any errors we get here mean that the checkpoint is already invalid
        // for one reason or another, so we are safe to ignore them.
        let _ = unsafe { cuptiCheckpointFree(&mut self.raw) };
    }
}
