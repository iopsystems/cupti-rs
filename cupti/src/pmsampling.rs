//! cupti_pmsampling.h

use c_enum::c_enum;
use cupti_sys::*;

c_enum! {
    /// PM sampling trigger mode.
    ///
    /// Specifies the trigger mode for PM sampling which determines how the sampling interval
    /// is interpreted.
    pub enum PmSamplingTriggerMode : CUpti_PmSampling_TriggerMode {
        /// The trigger is based off of the SYSCLK frequency.
        ///
        /// Note: SYS frequency by default is variable. The sample interval (set in
        /// [`CUpti_PmSampling_SetConfig_Params`]) is in terms of clocks.
        GpuSysclkInterval = CUPTI_PM_SAMPLING_TRIGGER_MODE_GPU_SYSCLK_INTERVAL,

        /// The trigger is based off of a fixed frequency source.
        ///
        /// The sample interval (set in [`CUpti_PmSampling_SetConfig_Params`]) is in terms of
        /// nanoseconds.
        ///
        /// # Notes
        ///
        /// This trigger mode is not supported on Turing GPU architecture and GA100 GPU.
        /// It is supported on Ampere GA10x and later GPU architectures.
        GpuTimeInterval = CUPTI_PM_SAMPLING_TRIGGER_MODE_GPU_TIME_INTERVAL,
    }
}

c_enum! {
    /// PM sampling decode stop reason.
    ///
    /// Indicates why the PM sampling decode operation stopped.
    pub enum PmSamplingDecodeStopReason : CUpti_PmSampling_DecodeStopReason {
        /// Decode stopped for an unspecified reason.
        Other = CUPTI_PM_SAMPLING_DECODE_STOP_REASON_OTHER,

        /// Counter data image is full.
        CounterDataFull = CUPTI_PM_SAMPLING_DECODE_STOP_REASON_COUNTER_DATA_FULL,

        /// All the records in the hardware buffer have been decoded.
        EndOfRecords = CUPTI_PM_SAMPLING_DECODE_STOP_REASON_END_OF_RECORDS,
    }
}

c_enum! {
    /// Hardware buffer append mode for PM sampling.
    ///
    /// Specifies the behavior when the hardware buffer fills up during PM sampling.
    pub enum PmSamplingHardwareBufferAppendMode : CUpti_PmSampling_HardwareBuffer_AppendMode {
        /// Keep the oldest records in the hardware buffer.
        ///
        /// CUPTI will report error for overflow in case hardware buffer is getting filled up.
        KeepOldest = CUPTI_PM_SAMPLING_HARDWARE_BUFFER_APPEND_MODE_KEEP_OLDEST,

        /// Keep the latest records in the hardware buffer.
        ///
        /// # Notes
        ///
        /// This mode is not supported on Turing GPU architecture.
        /// It is supported on Ampere and later GPU architectures.
        KeepLatest = CUPTI_PM_SAMPLING_HARDWARE_BUFFER_APPEND_MODE_KEEP_LATEST,
    }
}

// pub struct
