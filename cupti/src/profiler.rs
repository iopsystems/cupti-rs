use std::ffi::CStr;
use std::ptr::NonNull;

use c_enum::c_enum;
use cupti_sys::*;

use crate::pmsampling::CounterDataImage;
use crate::{Context, Error, Result};

c_enum! {
    /// Metric type classification.
    ///
    /// Categorizes metrics by their computational type.
    #[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
    pub enum MetricType : CUpti_MetricType {
        /// Counter metric type.
        Counter = CUPTI_METRIC_TYPE_COUNTER,

        /// Ratio metric type.
        Ratio = CUPTI_METRIC_TYPE_RATIO,

        /// Throughput metric type.
        Throughput = CUPTI_METRIC_TYPE_THROUGHPUT,
    }
}

c_enum! {
    /// Profiler type.
    ///
    /// Specifies the kind of profiler to use.
    #[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
    pub enum ProfilerType : CUpti_ProfilerType {
        /// Range-based profiler.
        RangeProfiler = CUPTI_PROFILER_TYPE_RANGE_PROFILER,

        /// PM sampling profiler.
        PmSampling = CUPTI_PROFILER_TYPE_PM_SAMPLING,

        /// Invalid profiler type.
        Invalid = CUPTI_PROFILER_TYPE_PROFILER_INVALID,
    }
}

pub struct HostProfiler {
    raw: NonNull<CUpti_Profiler_Host_Object>,
}

impl HostProfiler {
    pub fn new(
        ty: ProfilerType,
        chip_name: &CStr,
        counter_availability_image: &CounterAvailabilityImage,
    ) -> Result<Self> {
        let mut params = CUpti_Profiler_Host_Initialize_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.profilerType = ty.into();
        params.pChipName = chip_name.as_ptr();
        params.pCounterAvailabilityImage = counter_availability_image.0.as_ptr();

        Error::result(unsafe { cuptiProfilerHostInitialize(&mut params) })?;

        let raw = match NonNull::new(params.pHostObject) {
            Some(raw) => raw,
            None => panic!("cuptiProfilerHostInitialize succeeded but returned null"),
        };

        Ok(Self { raw })
    }

    pub fn as_raw(&self) -> *const CUpti_Profiler_Host_Object {
        self.raw.as_ptr()
    }

    pub fn as_raw_mut(&mut self) -> *mut CUpti_Profiler_Host_Object {
        self.raw.as_ptr()
    }

    /// Get a list of supported chip names.
    ///
    /// # Errors
    /// - [`Error::Unknown`] for any internal error.
    pub fn supported_chips() -> Result<Vec<&'static CStr>> {
        let mut params = CUpti_Profiler_Host_GetSupportedChips_Params::default();
        params.structSize = std::mem::size_of_val(&params);

        Error::result(unsafe { cuptiProfilerHostGetSupportedChips(&mut params) })?;

        let slice = unsafe { std::slice::from_raw_parts(params.ppChipNames, params.numChips) };
        let names = slice
            .iter()
            .copied()
            .map(|p| unsafe { CStr::from_ptr(p) })
            .collect();

        Ok(names)
    }

    /// Get a list of the supported base metrics for the chip.
    ///
    /// # Params
    /// - `ty` - metric type (counter, ratio, throughput)
    ///
    /// # Errors
    /// - [`Error::InvalidParameter`] if `ty` is not a valid metric type.
    /// - [`Error::Unknown`] for any internal error.
    pub fn get_base_metrics(&self, ty: MetricType) -> Result<Vec<&'static CStr>> {
        let mut params = CUpti_Profiler_Host_GetBaseMetrics_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pHostObject = self.raw.as_ptr();
        params.metricType = ty.into();

        Error::result(unsafe { cuptiProfilerHostGetBaseMetrics(&mut params) })?;

        let slice = unsafe { std::slice::from_raw_parts(params.ppMetricNames, params.numMetrics) };
        let names = slice
            .iter()
            .copied()
            .map(|p| unsafe { CStr::from_ptr(p) })
            .collect();

        Ok(names)
    }

    /// Get the list of supported sub-metrics for the metric.
    ///
    /// # Params
    /// - `ty` - the metric type for the queried metric
    /// - `name` - metric name for which sub-metric will be listed. This can be
    ///   with or without the extension (rollup or submetric).
    ///
    /// # Errors
    /// - [`Error::InvalidParameter`] if `ty` is not a valid metric type.
    /// - [`Error::InvalidMetricName`] if the metric name is not valid or not
    ///   supported for the chip.
    pub fn get_submetrics(&self, ty: MetricType, name: &CStr) -> Result<Vec<&'static CStr>> {
        let mut params = CUpti_Profiler_Host_GetSubMetrics_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pHostObject = self.raw.as_ptr();
        params.metricType = ty.into();
        params.pMetricName = name.as_ptr();

        Error::result(unsafe { cuptiProfilerHostGetSubMetrics(&mut params) })?;

        let slice =
            unsafe { std::slice::from_raw_parts(params.ppSubMetrics, params.numOfSubmetrics) };
        let names = slice
            .iter()
            .copied()
            .map(|p| unsafe { CStr::from_ptr(p) })
            .collect();

        Ok(names)
    }

    /// Get the properties of the metric.
    ///
    /// # Parameters
    /// - `name` - The metric name for which its properties will be listed. The
    ///   name can be with or without extension (rollup or submetric).
    ///
    /// # Errors
    /// - [`Error::InvalidMetricName`] if the metric name is not valid or not
    ///   supported for the chip.
    /// - [`Error::Unknown`] for any internal error.
    pub fn get_metric_properties(&self, name: &CStr) -> Result<MetricProperties> {
        let mut params = CUpti_Profiler_Host_GetMetricProperties_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pHostObject = self.raw.as_ptr();
        params.pMetricName = name.as_ptr();

        Error::result(unsafe { cuptiProfilerHostGetMetricProperties(&mut params) })?;

        unsafe {
            Ok(MetricProperties {
                description: CStr::from_ptr(params.pDescription),
                hw_unit: CStr::from_ptr(params.pHwUnit),
                dim_unit: CStr::from_ptr(params.pDimUnit),
                ty: params.metricType.into(),
            })
        }
    }

    /// Get the config image for hte metrics added to the profiler host object.
    ///
    /// # Errors
    /// - [`Error::Unknown`] for any internal error.
    pub fn get_config_image(&self) -> Result<ConfigImage> {
        let mut params = CUpti_Profiler_Host_GetConfigImageSize_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pHostObject = self.raw.as_ptr();

        Error::result(unsafe { cuptiProfilerHostGetConfigImageSize(&mut params) })?;

        let mut data = Vec::with_capacity(params.configImageSize);

        let mut params = CUpti_Profiler_Host_GetConfigImage_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pHostObject = self.raw.as_ptr();
        params.configImageSize = data.len();
        params.pConfigImage = data.spare_capacity_mut().as_ptr() as *mut u8;

        Error::result(unsafe { cuptiProfilerHostGetConfigImage(&mut params) })?;
        unsafe { data.set_len(params.configImageSize) };

        Ok(ConfigImage(data))
    }

    /// Evaluate the metric values for the range index stored in the counter
    /// data.
    ///
    /// # Params
    /// - `counter_data` - the counter data image where profiling data has been
    ///   decoded.
    /// - `range_index` - the range index for which the range name will be
    ///   queried.
    /// - `metric_names` - the metrics for which GPU values will be evaluated
    ///   for the range.
    ///
    /// # Errors
    /// - [`Error::InvalidParameter`] if any of the parameters is not valid.
    /// - [`Error::InvalidMetricName`] if the metric name is not valid or not
    ///   supported.
    /// - [`Error::Unknown`] for any internal error.
    pub fn evaluate_to_gpu_values(
        &self,
        counter_data: &CounterDataImage,
        range_index: usize,
        metric_names: &[&CStr],
    ) -> Result<Vec<f64>> {
        let mut metric_names = metric_names.iter().map(|c| c.as_ptr()).collect::<Vec<_>>();
        let mut metric_values = Vec::with_capacity(metric_names.len());

        let mut params = CUpti_Profiler_Host_EvaluateToGpuValues_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pHostObject = self.raw.as_ptr();
        params.pCounterDataImage = counter_data.as_bytes().as_ptr();
        params.counterDataImageSize = counter_data.as_bytes().len();
        params.rangeIndex = range_index;
        params.ppMetricNames = metric_names.as_mut_ptr();
        params.numMetrics = metric_names.len();
        params.pMetricValues = metric_values.spare_capacity_mut().as_mut_ptr() as *mut _;

        Error::result(unsafe { cuptiProfilerHostEvaluateToGpuValues(&mut params) })?;
        unsafe { metric_values.set_len(params.numMetrics) };

        Ok(metric_values)
    }
}

impl Drop for HostProfiler {
    fn drop(&mut self) {
        let mut params = CUpti_Profiler_Host_Deinitialize_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pHostObject = self.raw.as_ptr();

        let _ = unsafe { cuptiProfilerHostDeinitialize(&mut params) };
    }
}

#[derive(Clone, Debug)]
pub struct MetricProperties {
    pub description: &'static CStr,
    pub hw_unit: &'static CStr,
    pub dim_unit: &'static CStr,
    pub ty: MetricType,
}

/// A config image containing info about the enabled metrics for the profiler.
#[derive(Clone)]
pub struct ConfigImage(Vec<u8>);

impl ConfigImage {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Get the number of passes required for profilign the scheduled metrics in
    /// this config image.
    ///
    /// # Errors
    /// - [`Error::InvalidParameter`] if this config image is invalid.
    /// - [`Error::Unknown`] for any internal error.
    pub fn get_num_of_passes(&self) -> Result<usize> {
        let mut params = CUpti_Profiler_Host_GetNumOfPasses_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.pConfigImage = self.0.as_ptr() as *mut u8;
        params.configImageSize = self.0.len();

        Error::result(unsafe { cuptiProfilerHostGetNumOfPasses(&mut params) })?;

        Ok(params.numOfPasses)
    }
}

/// Counter availability image.
///
/// This is used by CUPTI to filter out unavailable metrics on the host. For
/// users of the API it is effectivly just an opaque blob of bytes.
#[derive(Clone)]
pub struct CounterAvailabilityImage(pub(crate) Vec<u8>);

impl CounterAvailabilityImage {
    fn get_impl(context: Option<&Context>) -> Result<Self> {
        let mut params = CUpti_Profiler_GetCounterAvailability_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.ctx = context.map(|c| c.as_raw()).unwrap_or(std::ptr::null_mut());

        Error::result(unsafe { cuptiProfilerGetCounterAvailability(&mut params) })?;

        let mut image = Vec::with_capacity(params.counterAvailabilityImageSize);
        params.pCounterAvailabilityImage = image.spare_capacity_mut().as_mut_ptr() as *mut u8;

        Error::result(unsafe { cuptiProfilerGetCounterAvailability(&mut params) })?;
        unsafe { image.set_len(params.counterAvailabilityImageSize) };

        Ok(Self(image))
    }

    /// Query counter availability for the current CUDA context.
    ///
    /// You should use this to query counter availability information into a
    /// buffer that can be then used to filter unavailable metrics on the
    /// host.
    ///
    /// # Errors
    /// This API can fail if any profiling or sampling session is active on the
    /// specified context or its device.
    pub fn get() -> Result<Self> {
        Self::get_impl(None)
    }

    /// Query counter availability.
    ///
    /// You should use this to query counter availability information into a
    /// buffer that can be then used to filter unavailable metrics on the
    /// host.
    ///
    /// # Parameters
    /// - `context` - The context to use.
    ///
    /// # Errors
    /// This API can fail if any profiling or sampling session is active on the
    /// specified context or its device.
    pub fn for_context(context: &Context) -> Result<Self> {
        Self::get_impl(Some(context))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Get the maximum number of hardware metrics (metric names which doesn't
    /// include the `sass` keyword) that can be scheduled in a single pass for a
    /// chip.
    ///
    /// While this represents a theoretical upper limit, practical constraints
    /// may prevent reaching this threshold for a specific set of metrics.
    /// Furthermore, the maximum achievable value is contingent upon the
    /// characteristics and architecture of the chip in question.
    ///
    /// Use [`ConfigImage::get_num_of_passes`] for getting the actual nuber of
    /// passes required for collecting the profiling data for the scheduled
    /// metrics for a config image.
    ///
    /// # Params
    /// - `profiler_type` - the profiler kind.
    /// - `chip_name` - accepted for chips supported at the time of release.
    ///
    /// # Errors
    /// - [`Error::InvalidParameter`] if any parameter is not valid.
    /// - [`Error::Unknown`] for any internal error.
    pub fn get_max_num_hardware_metrics_per_pass(
        &self,
        profiler_type: ProfilerType,
        chip_name: &CStr,
    ) -> Result<usize> {
        let mut params = CUpti_Profiler_Host_GetMaxNumHardwareMetricsPerPass_Params::default();
        params.structSize = std::mem::size_of_val(&params);
        params.profilerType = profiler_type.into();
        params.pChipName = chip_name.as_ptr();
        params.pCounterAvailabilityImage = self.0.as_ptr() as *mut u8;

        Error::result(unsafe { cuptiProfilerHostGetMaxNumHardwareMetricsPerPass(&mut params) })?;

        Ok(params.maxMetricsPerPass)
    }
}
