//! Range profiling API.
//!
//! This module is a wrapper around the functions in `cupti_range_profiler.h`. It
//! allows you to collect GPU performance metrics for specific code regions (ranges)
//! that you define in your application.
//!
//! Range profiling is the recommended approach for GPU performance profiling as of
//! CUDA 12.6, replacing the deprecated profiler target API.
//!
//! These APIs are supported on Turing and later GPU architectures.

use std::ffi::CStr;
use std::ptr::NonNull;

use c_enum::c_enum;
use cupti_sys::*;

use crate::profiler::*;
use crate::util::CStringSlice;
use crate::{Context, Error, Result};

c_enum! {
    /// Profiler range mode.
    ///
    /// Specifies how profiling ranges are defined in a profiling session.
    #[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
    pub enum ProfilerRange : CUpti_ProfilerRange {
        /// Invalid value.
        Invalid = CUPTI_Range_INVALID,

        /// Ranges are automatically defined around each kernel in a profiling session.
        ///
        /// Use this mode when you want to collect metrics for individual kernel launches
        /// without manually marking ranges.
        Auto = CUPTI_AutoRange,

        /// Ranges are defined by the user with [`RangeProfiler::push_range`] and
        /// [`RangeProfiler::pop_range`].
        ///
        /// Use this mode for fine-grained control over which code regions are profiled.
        User = CUPTI_UserRange,
    }
}

c_enum! {
    /// Profiler replay mode.
    ///
    /// For metrics which require multi-pass collection, a replay of the GPU kernel(s)
    /// is required. This attribute specifies how the replay is performed.
    #[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
    pub enum ProfilerReplayMode : CUpti_ProfilerReplayMode {
        /// Invalid value.
        Invalid = CUPTI_Replay_INVALID,

        /// Replay is done by the CUPTI user around the entire application.
        ///
        /// The user is responsible for re-running the entire workload for each pass.
        Application = CUPTI_ApplicationReplay,

        /// Replay is done implicitly by CUPTI around each kernel.
        ///
        /// CUPTI automatically replays each kernel as many times as needed.
        Kernel = CUPTI_KernelReplay,

        /// Replay is done by the CUPTI user within a process.
        ///
        /// The user controls replay at a finer granularity than application replay.
        User = CUPTI_UserReplay,
    }
}

/// Builder for creating a range profiler.
pub struct RangeProfilerBuilder {
    host: HostProfiler,
}

impl RangeProfilerBuilder {
    /// Create and initialize a new builder for range profiling.
    ///
    /// # Parameters
    ///
    /// - `chip_name`: The chip name (accepted for chips supported at the time-of-release)
    /// - `counter_availability_image`: Buffer with counter availability image
    ///   (required for future chip support)
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any parameter is not valid
    /// - [`Error::Unknown`] for any internal error
    pub fn new(
        chip_name: &CStr,
        counter_availability_image: &CounterAvailabilityImage,
    ) -> Result<Self> {
        let host = HostProfiler::new(
            ProfilerType::RangeProfiler,
            chip_name,
            counter_availability_image,
        )?;
        Ok(Self { host })
    }

    /// Construct a builder from a pre-configured host profiler.
    pub fn from_host_profiler(host: HostProfiler) -> Self {
        Self { host }
    }

    /// Get a list of the supported base metrics for the chip.
    ///
    /// # Parameters
    ///
    /// - `ty`: Metric type (counter, ratio, throughput)
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if `ty` is not a valid metric type.
    /// - [`Error::Unknown`] for any internal error.
    pub fn get_base_metrics(&self, ty: MetricType) -> Result<&'static CStringSlice> {
        self.host.get_base_metrics(ty)
    }

    /// Get the list of supported sub-metrics for the metric.
    ///
    /// # Parameters
    ///
    /// - `ty`: The metric type for the queried metric
    /// - `name`: Metric name for which sub-metrics will be listed. This can be
    ///   with or without the extension (rollup or submetric).
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if `ty` is not a valid metric type.
    /// - [`Error::InvalidMetricName`] if the metric name is not valid or not
    ///   supported for the chip.
    pub fn get_submetrics(&self, ty: MetricType, name: &CStr) -> Result<&'static CStringSlice> {
        self.host.get_submetrics(ty, name)
    }

    /// Get the properties of a metric.
    ///
    /// # Parameters
    ///
    /// - `name`: The metric name for which properties will be listed. The name
    ///   can be with or without extension (rollup or submetric).
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidMetricName`] if the metric name is not valid or not
    ///   supported for the chip.
    /// - [`Error::Unknown`] for any internal error.
    pub fn get_metric_properties(&self, name: &CStr) -> Result<MetricProperties> {
        self.host.get_metric_properties(name)
    }

    /// Add metrics to the profiler host object for generating the config image.
    ///
    /// The config image will have the required information to schedule the
    /// metrics for collecting the profiling data.
    ///
    /// # Parameters
    ///
    /// - `metric_names`: Metric names for which config image will be generated
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any parameter is not valid
    /// - [`Error::InvalidMetricName`] if the metric name is not valid or not
    ///   supported for the chip
    /// - [`Error::Unknown`] for any internal error
    pub fn add_metrics(&mut self, metric_names: &CStringSlice) -> Result<()> {
        self.host.add_metrics(metric_names)
    }

    /// Create a range profiler object and enable range profiling on the CUDA context.
    ///
    /// # Parameters
    ///
    /// - `ctx`: CUDA context to be used for profiling. Use `None` for the current context.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any parameter is not valid
    /// - [`Error::OutOfMemory`] if memory allocation fails while creating the
    ///   range profiler object
    /// - [`Error::InsufficientPrivileges`] if the user does not have sufficient
    ///   privileges to perform the operation
    /// - [`Error::Unknown`] for any internal error
    pub fn build(self, ctx: Option<&Context>) -> Result<RangeProfiler> {
        let config_image = self.host.get_config_image()?;

        let mut params = CUpti_RangeProfiler_Enable_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.ctx = ctx.map(|c| c.as_raw()).unwrap_or(std::ptr::null_mut());

        Error::result(unsafe { cuptiRangeProfilerEnable(&mut params) })?;

        let raw = NonNull::new(params.pRangeProfilerObject).unwrap_or_else(|| {
            panic!("cuptiRangeProfilerEnable succeeded but returned a null pointer")
        });

        Ok(RangeProfiler {
            raw,
            host: self.host,
            config_image,
        })
    }
}

/// Collect GPU performance metrics for specific code regions.
///
/// The range profiler allows you to define named regions of code and collect
/// metrics specifically for those regions. This is useful for understanding
/// performance characteristics of specific parts of your application.
pub struct RangeProfiler {
    raw: NonNull<CUpti_RangeProfiler_Object>,
    host: HostProfiler,
    config_image: ConfigImage,
}

impl RangeProfiler {
    /// Create and initialize a new builder for range profiling.
    ///
    /// # Parameters
    ///
    /// - `chip_name`: The chip name (accepted for chips supported at the time-of-release)
    /// - `counter_availability_image`: Buffer with counter availability image
    ///   (required for future chip support)
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any parameter is not valid
    /// - [`Error::Unknown`] for any internal error
    pub fn builder(
        chip_name: &CStr,
        counter_availability_image: &CounterAvailabilityImage,
    ) -> Result<RangeProfilerBuilder> {
        RangeProfilerBuilder::new(chip_name, counter_availability_image)
    }

    /// Get a reference to the underlying host profiler.
    pub fn host(&self) -> &HostProfiler {
        &self.host
    }

    /// Set the configuration for range profiling.
    ///
    /// This sets up the profiler with the range mode, replay mode, and other
    /// configuration parameters needed for metric collection.
    ///
    /// # Parameters
    ///
    /// - `counter_data`: Counter data image where profiling results will be stored
    /// - `config`: Configuration for the range profiler
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any parameter is not valid
    pub fn set_config(
        &mut self,
        counter_data: &mut RangeCounterDataImage,
        config: &RangeProfilerConfig,
    ) -> Result<()> {
        let mut params = CUpti_RangeProfiler_SetConfig_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pRangeProfilerObject = self.raw.as_ptr();
        params.configSize = self.config_image.as_bytes().len();
        params.pConfig = self.config_image.as_bytes().as_ptr();
        params.counterDataImageSize = counter_data.0.len();
        params.pCounterDataImage = counter_data.0.as_mut_ptr();
        params.range = config.range.into();
        params.replayMode = config.replay_mode.into();
        params.maxRangesPerPass = config.max_ranges_per_pass;
        params.numNestingLevels = config.num_nesting_levels;
        params.minNestingLevel = config.min_nesting_level;
        params.passIndex = config.pass_index;
        params.targetNestingLevel = config.target_nesting_level;

        Error::result(unsafe { cuptiRangeProfilerSetConfig(&mut params) })
    }

    /// Start the range profiler.
    ///
    /// After calling this, the profiler will begin collecting metrics for
    /// ranges defined with [`push_range`] and [`pop_range`] (in User mode) or
    /// automatically around kernels (in Auto mode).
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any parameter is not valid
    /// - [`Error::InvalidOperation`] if range profiler is not enabled or is
    ///   already started
    /// - [`Error::Unknown`] for any internal error
    ///
    /// [`push_range`]: Self::push_range
    /// [`pop_range`]: Self::pop_range
    pub fn start(&mut self) -> Result<()> {
        let mut params = CUpti_RangeProfiler_Start_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pRangeProfilerObject = self.raw.as_ptr();

        Error::result(unsafe { cuptiRangeProfilerStart(&mut params) })
    }

    /// Stop the range profiler.
    ///
    /// Returns information about the completed pass, including whether more
    /// passes are needed for multi-pass metric collection.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any parameter is not valid
    /// - [`Error::InvalidOperation`] if range profiler is not enabled or is
    ///   not started
    /// - [`Error::Unknown`] for any internal error
    pub fn stop(&mut self) -> Result<StopStatus> {
        let mut params = CUpti_RangeProfiler_Stop_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pRangeProfilerObject = self.raw.as_ptr();

        Error::result(unsafe { cuptiRangeProfilerStop(&mut params) })?;

        Ok(StopStatus {
            pass_index: params.passIndex,
            target_nesting_level: params.targetNestingLevel,
            is_all_pass_submitted: params.isAllPassSubmitted != 0,
        })
    }

    /// Push a new range onto the profiler stack.
    ///
    /// This marks the beginning of a named code region to profile. Use
    /// [`pop_range`] to mark the end of the region.
    ///
    /// For nested ranges, call this again for the innermost range. To profile
    /// nested ranges, set the `min_nesting_level` and `num_nesting_levels` in
    /// the [`RangeProfilerConfig`].
    ///
    /// # Parameters
    ///
    /// - `range_name`: Name of the range to be profiled (only valid for User range mode)
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any parameter is not valid
    /// - [`Error::InvalidOperation`] if range profiler is not enabled
    /// - [`Error::Unknown`] for any internal error
    ///
    /// [`pop_range`]: Self::pop_range
    pub fn push_range(&mut self, range_name: &CStr) -> Result<()> {
        let mut params = CUpti_RangeProfiler_PushRange_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pRangeProfilerObject = self.raw.as_ptr();
        params.pRangeName = range_name.as_ptr();

        Error::result(unsafe { cuptiRangeProfilerPushRange(&mut params) })
    }

    /// Pop the current range from the profiler stack.
    ///
    /// This marks the end of the current code region. The number of pop calls
    /// must match the number of push calls in the same order.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any parameter is not valid
    /// - [`Error::InvalidOperation`] if range profiler is not enabled or no
    ///   range is active
    /// - [`Error::Unknown`] for any internal error
    pub fn pop_range(&mut self) -> Result<()> {
        let mut params = CUpti_RangeProfiler_PopRange_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pRangeProfilerObject = self.raw.as_ptr();

        Error::result(unsafe { cuptiRangeProfilerPopRange(&mut params) })
    }

    /// Decode the profiling data from hardware into the counter data image.
    ///
    /// This should be called after [`stop`]. The counter data image passed
    /// to [`set_config`] will be updated with the profiling data.
    ///
    /// Returns the number of ranges that were dropped if the counter data
    /// image didn't have enough capacity to store all profiled ranges.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any parameter is not valid
    /// - [`Error::InvalidOperation`] if range profiler is not enabled
    /// - [`Error::Unknown`] for any internal error
    ///
    /// [`stop`]: Self::stop
    /// [`set_config`]: Self::set_config
    pub fn decode_data(&mut self) -> Result<usize> {
        let mut params = CUpti_RangeProfiler_DecodeData_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pRangeProfilerObject = self.raw.as_ptr();

        Error::result(unsafe { cuptiRangeProfilerDecodeData(&mut params) })?;

        Ok(params.numOfRangeDropped)
    }

}

impl Drop for RangeProfiler {
    fn drop(&mut self) {
        let mut params = CUpti_RangeProfiler_Disable_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pRangeProfilerObject = self.raw.as_ptr();

        let _ = unsafe { cuptiRangeProfilerDisable(&mut params) };
    }
}

/// Configuration for the range profiler.
#[derive(Clone, Debug)]
pub struct RangeProfilerConfig {
    /// Profiling range mode.
    pub range: ProfilerRange,
    /// Replay mode for multi-pass collection.
    pub replay_mode: ProfilerReplayMode,
    /// Maximum number of ranges that can be profiled in a pass.
    pub max_ranges_per_pass: usize,
    /// Number of nesting levels to be profiled. For Auto range mode, this should be 1.
    pub num_nesting_levels: u16,
    /// Minimum nesting level to be profiled.
    pub min_nesting_level: u16,
    /// Pass index for the replay session.
    pub pass_index: usize,
    /// Target nesting level for the replay session.
    pub target_nesting_level: u16,
}

impl Default for RangeProfilerConfig {
    fn default() -> Self {
        Self {
            range: ProfilerRange::Auto,
            replay_mode: ProfilerReplayMode::Kernel,
            max_ranges_per_pass: 64,
            num_nesting_levels: 1,
            min_nesting_level: 1,
            pass_index: 0,
            target_nesting_level: 1,
        }
    }
}

/// Status information returned from stopping the range profiler.
#[derive(Copy, Clone, Debug)]
pub struct StopStatus {
    /// Pass index for the next replay session.
    pub pass_index: usize,
    /// Target nesting level for the next replay session.
    pub target_nesting_level: usize,
    /// `true` if all passes have been submitted to GPU for collection.
    pub is_all_pass_submitted: bool,
}

/// A buffer storing counter data for range profiling.
pub struct RangeCounterDataImage(Vec<u8>);

impl RangeCounterDataImage {
    /// Create and initialize a counter data image buffer for storing metric data.
    ///
    /// This method combines getting the required buffer size and initializing the buffer
    /// in a single operation.
    ///
    /// # Parameters
    ///
    /// - `profiler`: Range profiler object
    /// - `metric_names`: Names of the metrics to be collected
    /// - `max_num_of_ranges`: Maximum number of ranges to be stored
    /// - `max_num_range_tree_nodes`: Maximum number of range tree nodes (must be >= max_num_of_ranges)
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any parameter is not valid
    /// - [`Error::InvalidOperation`] if called without enabling range profiler
    /// - [`Error::Unknown`] for any internal error
    pub fn new(
        profiler: &RangeProfiler,
        metric_names: &[&CStr],
        max_num_of_ranges: usize,
        max_num_range_tree_nodes: u32,
    ) -> Result<Self> {
        // Get the required size for the counter data image
        let mut metric_names_ptrs: Vec<*const _> =
            metric_names.iter().map(|c| c.as_ptr()).collect();

        let mut size_params = CUpti_RangeProfiler_GetCounterDataSize_Params::default();
        size_params.structSize = std::mem::size_of_val(&size_params);
        size_params.pRangeProfilerObject = profiler.raw.as_ptr() as *mut _;
        size_params.pMetricNames = metric_names_ptrs.as_mut_ptr();
        size_params.numMetrics = metric_names_ptrs.len();
        size_params.maxNumOfRanges = max_num_of_ranges;
        size_params.maxNumRangeTreeNodes = max_num_range_tree_nodes;

        Error::result(unsafe { cuptiRangeProfilerGetCounterDataSize(&mut size_params) })?;

        // Allocate the buffer
        let image = vec![0u8; size_params.counterDataSize];
        let mut this = Self(image);

        // Initialize the buffer
        let mut init_params = CUpti_RangeProfiler_CounterDataImage_Initialize_Params::default();
        init_params.structSize = std::mem::size_of_val(&init_params);
        init_params.pRangeProfilerObject = profiler.raw.as_ptr() as *mut _;
        init_params.counterDataSize = this.0.len();
        init_params.pCounterData = this.0.as_mut_ptr();

        Error::result(unsafe { cuptiRangeProfilerCounterDataImageInitialize(&mut init_params) })?;

        Ok(this)
    }

    /// Get the raw bytes of the counter data image.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Get the raw bytes of the counter data image as a mutable slice.
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }

    /// Get the number of ranges stored in the counter data image.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any parameter is not valid
    /// - [`Error::Unknown`] for any internal error
    pub fn get_data_info(&self) -> Result<RangeCounterDataInfo> {
        let mut params = CUpti_RangeProfiler_GetCounterDataInfo_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pCounterDataImage = self.0.as_ptr();
        params.counterDataImageSize = self.0.len();

        Error::result(unsafe { cuptiRangeProfilerGetCounterDataInfo(&mut params) })?;

        Ok(RangeCounterDataInfo {
            num_total_ranges: params.numTotalRanges,
        })
    }

    /// Get the range name for a given range index.
    ///
    /// # Parameters
    ///
    /// - `range_index`: Index of the range
    /// - `delimiter`: Delimiter for nested range names (e.g., "/")
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any parameter is not valid
    /// - [`Error::Unknown`] for any internal error
    pub fn get_range_info(
        &self,
        range_index: usize,
        delimiter: &CStr,
    ) -> Result<&'static CStr> {
        let mut params = CUpti_RangeProfiler_CounterData_GetRangeInfo_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pCounterDataImage = self.0.as_ptr();
        params.counterDataImageSize = self.0.len();
        params.rangeIndex = range_index;
        params.rangeDelimiter = delimiter.as_ptr();

        Error::result(unsafe { cuptiRangeProfilerCounterDataGetRangeInfo(&mut params) })?;

        Ok(unsafe { CStr::from_ptr(params.rangeName) })
    }

    /// Evaluate the metric values for the range index stored in the counter data.
    ///
    /// # Parameters
    ///
    /// - `profiler`: The range profiler
    /// - `range_index`: The range index for which metrics will be evaluated
    /// - `metric_names`: The metrics for which GPU values will be evaluated
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if any of the parameters is not valid
    /// - [`Error::InvalidMetricName`] if the metric name is not valid or not supported
    /// - [`Error::Unknown`] for any internal error
    pub fn evaluate(
        &self,
        profiler: &RangeProfiler,
        range_index: usize,
        metric_names: &CStringSlice,
    ) -> Result<Vec<f64>> {
        let mut metric_values = Vec::with_capacity(metric_names.len());

        let mut params = CUpti_Profiler_Host_EvaluateToGpuValues_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pHostObject = profiler.host.as_raw() as *mut _;
        params.pCounterDataImage = self.0.as_ptr();
        params.counterDataImageSize = self.0.len();
        params.rangeIndex = range_index;
        params.ppMetricNames = metric_names.as_raw_slice().as_ptr() as *mut _;
        params.numMetrics = metric_names.as_raw_slice().len();
        params.pMetricValues = metric_values.spare_capacity_mut().as_mut_ptr() as *mut _;

        Error::result(unsafe { cuptiProfilerHostEvaluateToGpuValues(&mut params) })?;
        unsafe { metric_values.set_len(params.numMetrics) };

        Ok(metric_values)
    }
}

/// Information about ranges in a counter data image.
#[derive(Copy, Clone, Debug)]
pub struct RangeCounterDataInfo {
    /// Number of ranges in the counter data image.
    pub num_total_ranges: usize,
}
