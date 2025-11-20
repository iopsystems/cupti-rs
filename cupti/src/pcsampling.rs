use std::marker::PhantomData;

use c_enum::c_enum;
use cupti_sys::*;

use crate::*;

c_enum! {
    /// PC Sampling collection mode
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum CollectionMode : CUpti_PCSamplingCollectionMode {
        /// Invalid value
        Invalid = CUPTI_PC_SAMPLING_COLLECTION_MODE_INVALID,
        /// Continuous mode. Kernels are not serialized in this mode.
        Continuous = CUPTI_PC_SAMPLING_COLLECTION_MODE_CONTINUOUS,
        /// Serialized mode. Kernels are serialized in this mode.
        KernelSerialized = CUPTI_PC_SAMPLING_COLLECTION_MODE_KERNEL_SERIALIZED,
    }
}

c_enum! {
    /// PC Sampling output data format
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum OutputDataFormat : CUpti_PCSamplingOutputDataFormat {
        Invalid = CUPTI_PC_SAMPLING_OUTPUT_DATA_FORMAT_INVALID,
        /// HW buffer data will be parsed during collection of data
        Parsed = CUPTI_PC_SAMPLING_OUTPUT_DATA_FORMAT_PARSED,
    }
}

c_enum! {
    /// PC Sampling configuration attributes
    ///
    /// PC Sampling configuration attribute types. These attributes can be read
    /// using `cuptiPCSamplingGetConfigurationAttribute` and can be written
    /// using `cuptiPCSamplingSetConfigurationAttribute`. Attributes marked
    /// **r** can only be read using `cuptiPCSamplingGetConfigurationAttribute`,
    /// **w** can only be written using `cuptiPCSamplingSetConfigurationAttribute`,
    /// **rw** can be read using `cuptiPCSamplingGetConfigurationAttribute` and
    /// written using `cuptiPCSamplingSetConfigurationAttribute`
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ConfigurationAttributeType : CUpti_PCSamplingConfigurationAttributeType {
        Invalid = CUPTI_PC_SAMPLING_CONFIGURATION_ATTR_TYPE_INVALID,
        /// **rw** Sampling period for PC Sampling.
        ///
        /// DEFAULT - CUPTI defined value based on number of SMs
        ///
        /// Valid values for the sampling periods are between 5 to 31 both inclusive.
        /// This will set the sampling period to (2^samplingPeriod) cycles.
        /// For e.g. for sampling period = 5 to 31, cycles = 32, 64, 128,..., 2^31
        ///
        /// Value is a `u32`
        SamplingPeriod = CUPTI_PC_SAMPLING_CONFIGURATION_ATTR_TYPE_SAMPLING_PERIOD,
        /// **w** Number of stall reasons to collect.
        ///
        /// DEFAULT - All stall reasons will be collected
        ///
        /// Value is a `size_t`. Stall reasons to collect. Input value should be a pointer
        /// pointing to array of stall reason indexes containing all the stall reason indexes
        /// to collect.
        StallReason = CUPTI_PC_SAMPLING_CONFIGURATION_ATTR_TYPE_STALL_REASON,
        /// **rw** Size of SW buffer for raw PC counter data downloaded from HW buffer
        ///
        /// DEFAULT - 1 MB, which can accommodate approximately 5500 PCs with all stall reasons
        ///
        /// Approximately it takes 16 Bytes (and some fixed size memory)
        /// to accommodate one PC with one stall reason
        /// For e.g. 1 PC with 1 stall reason = 32 Bytes,
        ///          1 PC with 2 stall reason = 48 Bytes,
        ///          1 PC with 4 stall reason = 96 Bytes
        ///
        /// Value is a `size_t`
        ScratchBufferSize = CUPTI_PC_SAMPLING_CONFIGURATION_ATTR_TYPE_SCRATCH_BUFFER_SIZE,
        /// **rw** Size of HW buffer in bytes
        ///
        /// DEFAULT - 512 MB
        ///
        /// If sampling period is too less, HW buffer can overflow and drop PC data
        ///
        /// Value is a `size_t`
        HardwareBufferSize = CUPTI_PC_SAMPLING_CONFIGURATION_ATTR_TYPE_HARDWARE_BUFFER_SIZE,
        /// **rw** PC Sampling collection mode
        ///
        /// DEFAULT - [`CollectionMode::Continuous`]
        ///
        /// Input value should be of type [`CollectionMode`].
        CollectionMode = CUPTI_PC_SAMPLING_CONFIGURATION_ATTR_TYPE_COLLECTION_MODE,
        /// **rw** Control over PC Sampling data collection range
        ///
        /// Default - 0
        ///
        /// 1 - Allows user to start and stop PC Sampling using APIs:
        /// - `cuptiPCSamplingStart()` - Start PC Sampling
        /// - `cuptiPCSamplingStop()` - Stop PC Sampling
        ///
        /// Value is a `u32`
        EnableStartStopControl = CUPTI_PC_SAMPLING_CONFIGURATION_ATTR_TYPE_ENABLE_START_STOP_CONTROL,
        /// **w** Value for output data format
        ///
        /// Default - [`OutputDataFormat::Parsed`]
        ///
        /// Input value should be of type [`OutputDataFormat`].
        OutputDataFormat = CUPTI_PC_SAMPLING_CONFIGURATION_ATTR_TYPE_OUTPUT_DATA_FORMAT,
        /// **w** Data buffer to hold collected PC Sampling data PARSED_DATA
        ///
        /// Default - none.
        ///
        /// Buffer type is `void *` which can point to PARSED_DATA
        /// Refer `CUpti_PCSamplingData` for buffer format for PARSED_DATA
        SamplingDataBuffer = CUPTI_PC_SAMPLING_CONFIGURATION_ATTR_TYPE_SAMPLING_DATA_BUFFER,
        /// **rw** Control sleep time of the worker threads created by CUPTI for various PC sampling operations.
        ///
        /// CUPTI creates multiple worker threads to offload certain operations to these threads.
        /// This includes decoding of HW data to the CUPTI PC sampling data and correlating PC data
        /// to SASS instructions. CUPTI wakes up these threads periodically.
        ///
        /// Default - 100 milliseconds.
        ///
        /// Value is a `u32`
        WorkerThreadPeriodicSleepSpan = CUPTI_PC_SAMPLING_CONFIGURATION_ATTR_TYPE_WORKER_THREAD_PERIODIC_SLEEP_SPAN,
    }
}

fn get_config_attr(ctx: &Context, info: &mut CUpti_PCSamplingConfigurationInfo) -> Result<()> {
    let mut params = CUpti_PCSamplingConfigurationInfoParams {
        size: std::mem::size_of::<CUpti_PCSamplingConfigurationInfoParams>(),
        ctx: ctx.as_raw(),
        numAttributes: 1,
        pPCSamplingConfigurationInfo: info,
        ..Default::default()
    };

    Error::result(unsafe { cuptiPCSamplingGetConfigurationAttribute(&mut params) })?;
    Error::result(info.attributeStatus)?;

    Ok(())
}

fn set_config_attr(ctx: &Context, info: &mut CUpti_PCSamplingConfigurationInfo) -> Result<()> {
    let mut params = CUpti_PCSamplingConfigurationInfoParams {
        size: std::mem::size_of::<CUpti_PCSamplingConfigurationInfoParams>(),
        ctx: ctx.as_raw(),
        numAttributes: 1,
        pPCSamplingConfigurationInfo: info,
        ..Default::default()
    };

    Error::result(unsafe { cuptiPCSamplingSetConfigurationAttribute(&mut params) })?;
    Error::result(info.attributeStatus)?;

    Ok(())
}

pub fn get_sampling_period(ctx: &Context) -> Result<u32> {
    let mut info = CUpti_PCSamplingConfigurationInfo {
        attributeType: ConfigurationAttributeType::SamplingPeriod.into(),
        ..Default::default()
    };

    get_config_attr(ctx, &mut info)?;

    Ok(unsafe { info.attributeData.samplingPeriodData.samplingPeriod })
}

pub fn get_scratch_buffer_size(ctx: &Context) -> Result<usize> {
    let mut info = CUpti_PCSamplingConfigurationInfo {
        attributeType: ConfigurationAttributeType::ScratchBufferSize.into(),
        ..Default::default()
    };

    get_config_attr(ctx, &mut info)?;

    Ok(unsafe { info.attributeData.scratchBufferSizeData.scratchBufferSize })
}

pub fn get_hardware_buffer_size(ctx: &Context) -> Result<usize> {
    let mut info = CUpti_PCSamplingConfigurationInfo {
        attributeType: ConfigurationAttributeType::HardwareBufferSize.into(),
        ..Default::default()
    };

    get_config_attr(ctx, &mut info)?;

    Ok(unsafe { info.attributeData.hardwareBufferSizeData.hardwareBufferSize })
}

pub fn get_collection_mode(ctx: &Context) -> Result<CollectionMode> {
    let mut info = CUpti_PCSamplingConfigurationInfo {
        attributeType: ConfigurationAttributeType::CollectionMode.into(),
        ..Default::default()
    };

    get_config_attr(ctx, &mut info)?;

    Ok(unsafe { info.attributeData.collectionModeData.collectionMode.into() })
}

pub fn get_start_stop_control_enabled(ctx: &Context) -> Result<bool> {
    let mut info = CUpti_PCSamplingConfigurationInfo {
        attributeType: ConfigurationAttributeType::EnableStartStopControl.into(),
        ..Default::default()
    };

    get_config_attr(ctx, &mut info)?;

    Ok(unsafe {
        info.attributeData
            .enableStartStopControlData
            .enableStartStopControl
            != 0
    })
}

pub fn get_worker_thread_periodic_sleep_span(ctx: &Context) -> Result<u32> {
    let mut info = CUpti_PCSamplingConfigurationInfo {
        attributeType: ConfigurationAttributeType::EnableStartStopControl.into(),
        ..Default::default()
    };

    get_config_attr(ctx, &mut info)?;

    Ok(unsafe {
        info.attributeData
            .workerThreadPeriodicSleepSpanData
            .workerThreadPeriodicSleepSpan
    })
}

pub fn set_sampling_period(ctx: &Context, period: u32) -> Result<()> {
    let mut info = CUpti_PCSamplingConfigurationInfo::default();
    info.attributeType = ConfigurationAttributeType::SamplingPeriod.into();
    info.attributeData.samplingPeriodData.samplingPeriod = period;

    set_config_attr(ctx, &mut info)
}

pub fn set_stall_reasons(ctx: &Context, reason_indices: &[u32]) -> Result<()> {
    let mut info = CUpti_PCSamplingConfigurationInfo::default();
    info.attributeType = ConfigurationAttributeType::StallReason.into();
    info.attributeData.stallReasonData.stallReasonCount = reason_indices.len();
    info.attributeData.stallReasonData.pStallReasonIndex = reason_indices.as_ptr() as *mut _;

    set_config_attr(ctx, &mut info)
}

pub fn set_scratch_buffer_size(ctx: &Context, size: usize) -> Result<()> {
    let mut info = CUpti_PCSamplingConfigurationInfo::default();
    info.attributeType = ConfigurationAttributeType::ScratchBufferSize.into();
    info.attributeData.scratchBufferSizeData.scratchBufferSize = size;

    set_config_attr(ctx, &mut info)
}

pub fn set_hardware_buffer_size(ctx: &Context, size: usize) -> Result<()> {
    let mut info = CUpti_PCSamplingConfigurationInfo::default();
    info.attributeType = ConfigurationAttributeType::HardwareBufferSize.into();
    info.attributeData.hardwareBufferSizeData.hardwareBufferSize = size;

    set_config_attr(ctx, &mut info)
}

pub fn set_collection_mode(ctx: &Context, mode: CollectionMode) -> Result<()> {
    let mut info = CUpti_PCSamplingConfigurationInfo::default();
    info.attributeType = ConfigurationAttributeType::CollectionMode.into();
    info.attributeData.collectionModeData.collectionMode = mode.into();

    set_config_attr(ctx, &mut info)
}

pub fn set_start_stop_control_enabled(ctx: &Context, enabled: bool) -> Result<()> {
    let mut info = CUpti_PCSamplingConfigurationInfo::default();
    info.attributeType = ConfigurationAttributeType::EnableStartStopControl.into();
    info.attributeData
        .enableStartStopControlData
        .enableStartStopControl = enabled.into();

    set_config_attr(ctx, &mut info)
}

#[repr(transparent)]
pub struct SamplingData {
    raw: CUpti_PCSamplingData
}


// pub struct PCSampler {
//     ctx: CUcontext,
// }

// impl PCSampler {
//     pub fn new(ctx: &Context) -> Self {}
// }
