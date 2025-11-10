use c_enum::c_enum;
use cuda_sys::cuda::CUcontext;
use cupti_sys::*;

use crate::*;

c_enum! {
    /// The kinds of activity objects.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityObjectKind : CUpti_ActivityObjectKind {
        /// The object kind is not known.
        Unknown = CUPTI_ACTIVITY_OBJECT_UNKNOWN,
        /// A process.
        Process = CUPTI_ACTIVITY_OBJECT_PROCESS,
        /// A thread.
        Thread = CUPTI_ACTIVITY_OBJECT_THREAD,
        /// A device.
        Device = CUPTI_ACTIVITY_OBJECT_DEVICE,
        /// A context.
        Context = CUPTI_ACTIVITY_OBJECT_CONTEXT,
        /// A stream.
        Stream = CUPTI_ACTIVITY_OBJECT_STREAM,
    }
}

c_enum! {
    /// The kinds of activity records.
    ///
    /// Each activity record kind represents information about a GPU or an
    /// activity occurring on a CPU or GPU. Each kind is associated with an
    /// activity record structure that holds the information associated
    /// with the kind.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityKind : CUpti_ActivityKind {
        /// The activity record is invalid.
        Invalid = CUPTI_ACTIVITY_KIND_INVALID,
        /// A host<->host, host<->device, or device<->device memory copy.
        ///
        /// For peer to peer memory copy, use the kind [`Memcpy2`].
        ///
        /// [`Memcpy2`]: Self::Memcpy2
        Memcpy = CUPTI_ACTIVITY_KIND_MEMCPY,
        /// A memory set executing on the GPU.
        Memset = CUPTI_ACTIVITY_KIND_MEMSET,
        /// A kernel executing on the GPU.
        ///
        /// This activity kind may significantly change the overall performance characteristics
        /// of the application because all kernel executions are serialized on the GPU. Other
        /// activity kind for kernel [`ConcurrentKernel`] doesn't break kernel concurrency.
        ///
        /// [`ConcurrentKernel`]: Self::ConcurrentKernel
        Kernel = CUPTI_ACTIVITY_KIND_KERNEL,
        /// A CUDA driver API function execution.
        Driver = CUPTI_ACTIVITY_KIND_DRIVER,
        /// A CUDA runtime API function execution.
        Runtime = CUPTI_ACTIVITY_KIND_RUNTIME,
        /// A performance counter (aka event) value.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Event API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        Event = CUPTI_ACTIVITY_KIND_EVENT,
        /// A performance metric value.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Metric API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        Metric = CUPTI_ACTIVITY_KIND_METRIC,
        /// Information about a CUDA device.
        Device = CUPTI_ACTIVITY_KIND_DEVICE,
        /// Information about a CUDA context.
        Context = CUPTI_ACTIVITY_KIND_CONTEXT,
        /// A kernel executing on the GPU.
        ///
        /// This activity kind doesn't break kernel concurrency.
        ConcurrentKernel = CUPTI_ACTIVITY_KIND_CONCURRENT_KERNEL,
        /// Resource naming done via NVTX APIs for thread, device, context, etc.
        Name = CUPTI_ACTIVITY_KIND_NAME,
        /// Instantaneous, start, or end NVTX marker.
        Marker = CUPTI_ACTIVITY_KIND_MARKER,
        /// Extended, optional, data about a NVTX marker.
        ///
        /// User must enable [`Marker`] as well to get records for marker data.
        ///
        /// [`Marker`]: Self::Marker
        MarkerData = CUPTI_ACTIVITY_KIND_MARKER_DATA,
        /// Source information about source level result.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the SASS Metric APIs from the cupti_sass_metrics.h header.
        SourceLocator = CUPTI_ACTIVITY_KIND_SOURCE_LOCATOR,
        /// Results for source-level global access.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the SASS Metric APIs from the cupti_sass_metrics.h header.
        GlobalAccess = CUPTI_ACTIVITY_KIND_GLOBAL_ACCESS,
        /// Results for source-level branch.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the SASS Metric APIs from the cupti_sass_metrics.h header.
        Branch = CUPTI_ACTIVITY_KIND_BRANCH,
        /// Overhead added by CUPTI, Compiler, CUDA driver etc.
        Overhead = CUPTI_ACTIVITY_KIND_OVERHEAD,
        /// A CDP (CUDA Dynamic Parallel) kernel executing on the GPU.
        ///
        /// This activity cannot be directly enabled or disabled. It is enabled and disabled
        /// through concurrent kernel activity i.e. [`ConcurrentKernel`].
        ///
        /// [`ConcurrentKernel`]: Self::ConcurrentKernel
        CdpKernel = CUPTI_ACTIVITY_KIND_CDP_KERNEL,
        /// Preemption activity record indicating a preemption of a CDP (CUDA Dynamic Parallel)
        /// kernel executing on the GPU.
        Preemption = CUPTI_ACTIVITY_KIND_PREEMPTION,
        /// Environment activity records indicating power, clock, thermal, etc. levels of the GPU.
        Environment = CUPTI_ACTIVITY_KIND_ENVIRONMENT,
        /// A performance counter value associated with a specific event domain instance.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Event API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        EventInstance = CUPTI_ACTIVITY_KIND_EVENT_INSTANCE,
        /// A peer to peer memory copy.
        Memcpy2 = CUPTI_ACTIVITY_KIND_MEMCPY2,
        /// A performance metric value associated with a specific metric domain instance.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Metric API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        MetricInstance = CUPTI_ACTIVITY_KIND_METRIC_INSTANCE,
        /// Results for source-level instruction execution.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the SASS Metric APIs from the cupti_sass_metrics.h header.
        InstructionExecution = CUPTI_ACTIVITY_KIND_INSTRUCTION_EXECUTION,
        /// Unified Memory counter record.
        UnifiedMemoryCounter = CUPTI_ACTIVITY_KIND_UNIFIED_MEMORY_COUNTER,
        /// Device global/function record.
        Function = CUPTI_ACTIVITY_KIND_FUNCTION,
        /// CUDA Module record.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the module callback can be stored in the corresponding activity record.
        Module = CUPTI_ACTIVITY_KIND_MODULE,
        /// A device attribute value.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// attributes `CUpti_DeviceAttribute` or `CUdevice_attribute` can be stored in the
        /// corresponding activity record.
        DeviceAttribute = CUPTI_ACTIVITY_KIND_DEVICE_ATTRIBUTE,
        /// Results for source-level shared access.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the SASS Metric APIs from the cupti_sass_metrics.h header.
        SharedAccess = CUPTI_ACTIVITY_KIND_SHARED_ACCESS,
        /// PC sampling information for kernels.
        ///
        /// This will serialize kernels.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the PC Sampling API from the cupti_pcsampling.h header, which
        /// allows concurrent kernel execution.
        PcSampling = CUPTI_ACTIVITY_KIND_PC_SAMPLING,
        /// Summary information about PC sampling records.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the PC Sampling API from the cupti_pcsampling.h header.
        PcSamplingRecordInfo = CUPTI_ACTIVITY_KIND_PC_SAMPLING_RECORD_INFO,
        /// SASS/Source line-by-line correlation record.
        ///
        /// This will generate sass/source correlation for functions that have source
        /// level analysis or pc sampling results. The records will be generated only
        /// when either of source level analysis or pc sampling activity is enabled.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the SASS Metric APIs from the cupti_sass_metrics.h header.
        InstructionCorrelation = CUPTI_ACTIVITY_KIND_INSTRUCTION_CORRELATION,
        /// OpenACC data events.
        OpenaccData = CUPTI_ACTIVITY_KIND_OPENACC_DATA,
        /// OpenACC launch events.
        OpenaccLaunch = CUPTI_ACTIVITY_KIND_OPENACC_LAUNCH,
        /// OpenACC other events.
        OpenaccOther = CUPTI_ACTIVITY_KIND_OPENACC_OTHER,
        /// Information about a CUDA event (cudaEvent).
        CudaEvent = CUPTI_ACTIVITY_KIND_CUDA_EVENT,
        /// Information about a CUDA stream.
        Stream = CUPTI_ACTIVITY_KIND_STREAM,
        /// Records for CUDA synchronization primitives.
        Synchronization = CUPTI_ACTIVITY_KIND_SYNCHRONIZATION,
        /// Records for correlation of different programming APIs.
        ExternalCorrelation = CUPTI_ACTIVITY_KIND_EXTERNAL_CORRELATION,
        /// NVLink topology information.
        Nvlink = CUPTI_ACTIVITY_KIND_NVLINK,
        /// Instantaneous Event information.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Event API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        InstantaneousEvent = CUPTI_ACTIVITY_KIND_INSTANTANEOUS_EVENT,
        /// Instantaneous Event information for a specific event domain instance.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Event API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        InstantaneousEventInstance = CUPTI_ACTIVITY_KIND_INSTANTANEOUS_EVENT_INSTANCE,
        /// Instantaneous Metric information.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Metric API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        InstantaneousMetric = CUPTI_ACTIVITY_KIND_INSTANTANEOUS_METRIC,
        /// Instantaneous Metric information for a specific metric domain instance.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Metric API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        InstantaneousMetricInstance = CUPTI_ACTIVITY_KIND_INSTANTANEOUS_METRIC_INSTANCE,
        /// Memory activity tracking allocation and freeing of the memory.
        Memory = CUPTI_ACTIVITY_KIND_MEMORY,
        /// PCI devices information used for PCI topology.
        Pcie = CUPTI_ACTIVITY_KIND_PCIE,
        /// OpenMP parallel events.
        Openmp = CUPTI_ACTIVITY_KIND_OPENMP,
        /// A CUDA driver kernel launch occurring outside of any public API function execution.
        ///
        /// Tools can handle these like records for driver API launch functions, although
        /// the cbid field is not used here.
        InternalLaunchApi = CUPTI_ACTIVITY_KIND_INTERNAL_LAUNCH_API,
        /// Memory activity tracking allocation and freeing of the memory.
        Memory2 = CUPTI_ACTIVITY_KIND_MEMORY2,
        /// Memory pool activity tracking creation, destruction and trimming of the memory pool.
        MemoryPool = CUPTI_ACTIVITY_KIND_MEMORY_POOL,
        /// Activity record for graph-level information.
        GraphTrace = CUPTI_ACTIVITY_KIND_GRAPH_TRACE,
        /// JIT (Just-in-time) operation tracking.
        Jit = CUPTI_ACTIVITY_KIND_JIT,
        /// Device graph trace activity.
        ///
        /// This activity cannot be directly enabled or disabled.
        /// It is enabled when [`GraphTrace`] is enabled and device graph trace is enabled
        /// through API `cuptiActivityEnableDeviceGraph()`.
        ///
        /// [`GraphTrace`]: Self::GraphTrace
        DeviceGraphTrace = CUPTI_ACTIVITY_KIND_DEVICE_GRAPH_TRACE,
        /// Tracing batches of copies that are to be decompressed.
        MemDecompress = CUPTI_ACTIVITY_KIND_MEM_DECOMPRESS,
        /// Tracing new overheads introduced on some hardware when confidential computing is enabled.
        ConfidentialComputeRotation = CUPTI_ACTIVITY_KIND_CONFIDENTIAL_COMPUTE_ROTATION,
        /// Count of supported activity kinds.
        Count = CUPTI_ACTIVITY_KIND_COUNT,
    }
}

c_enum! {
    /// The kinds of activity overhead.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityOverheadKind : CUpti_ActivityOverheadKind {
        /// The overhead kind is not known.
        Unknown = CUPTI_ACTIVITY_OVERHEAD_UNKNOWN,
        /// Compiler overhead.
        DriverCompiler = CUPTI_ACTIVITY_OVERHEAD_DRIVER_COMPILER,
        /// Activity buffer flush overhead.
        CuptiBufferFlush = CUPTI_ACTIVITY_OVERHEAD_CUPTI_BUFFER_FLUSH,
        /// CUPTI instrumentation overhead.
        CuptiInstrumentation = CUPTI_ACTIVITY_OVERHEAD_CUPTI_INSTRUMENTATION,
        /// CUPTI resource creation and destruction overhead.
        CuptiResource = CUPTI_ACTIVITY_OVERHEAD_CUPTI_RESOURCE,
        /// CUDA Runtime triggered module loading overhead.
        RuntimeTriggeredModuleLoading = CUPTI_ACTIVITY_OVERHEAD_RUNTIME_TRIGGERED_MODULE_LOADING,
        /// Lazy function loading overhead.
        LazyFunctionLoading = CUPTI_ACTIVITY_OVERHEAD_LAZY_FUNCTION_LOADING,
        /// Overhead due to lack of command buffer space.
        ///
        /// Refer to `CUpti_ActivityOverheadCommandBufferFullData` for more details.
        CommandBufferFull = CUPTI_ACTIVITY_OVERHEAD_COMMAND_BUFFER_FULL,
        /// Overhead due to activity buffer request.
        ActivityBufferRequest = CUPTI_ACTIVITY_OVERHEAD_ACTIVITY_BUFFER_REQUEST,
        /// Overhead due to UVM activity initialization.
        UvmActivityInit = CUPTI_ACTIVITY_OVERHEAD_UVM_ACTIVITY_INIT,
    }
}

c_enum! {
    /// The kind of a compute API.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityComputeApiKind : CUpti_ActivityComputeApiKind {
        /// The compute API is not known.
        Unknown = CUPTI_ACTIVITY_COMPUTE_API_UNKNOWN,
        /// The compute APIs are for CUDA.
        Cuda = CUPTI_ACTIVITY_COMPUTE_API_CUDA,
        /// The compute APIs are for CUDA running in MPS (Multi-Process Service) environment.
        CudaMps = CUPTI_ACTIVITY_COMPUTE_API_CUDA_MPS,
    }
}

bitflags::bitflags! {
    /// Flags associated with activity records.
    ///
    /// Activity record flags. Flags can be combined by bitwise OR to associate multiple flags with
    /// an activity record. Each flag is specific to a certain activity kind.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ActivityFlag : CUpti_ActivityFlag {
        /// The activity record has no flags.
        const NONE = CUPTI_ACTIVITY_FLAG_NONE;
        /// The activity represents a device that supports concurrent kernel execution.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_DEVICE`.
        const DEVICE_CONCURRENT_KERNELS = CUPTI_ACTIVITY_FLAG_DEVICE_CONCURRENT_KERNELS;
        /// If the activity represents a `CUdevice_attribute` value or a `CUpti_DeviceAttribute` value.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_DEVICE_ATTRIBUTE`.
        const DEVICE_ATTRIBUTE_CUDEVICE = CUPTI_ACTIVITY_FLAG_DEVICE_ATTRIBUTE_CUDEVICE;
        /// The activity represents an asynchronous memcpy operation.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MEMCPY`.
        const MEMCPY_ASYNC = CUPTI_ACTIVITY_FLAG_MEMCPY_ASYNC;
        /// The activity represents an instantaneous marker.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_INSTANTANEOUS = CUPTI_ACTIVITY_FLAG_MARKER_INSTANTANEOUS;
        /// The activity represents a region start marker.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_START = CUPTI_ACTIVITY_FLAG_MARKER_START;
        /// The activity represents a region end marker.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_END = CUPTI_ACTIVITY_FLAG_MARKER_END;
        /// The activity represents an attempt to acquire a user defined synchronization object.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_SYNC_ACQUIRE = CUPTI_ACTIVITY_FLAG_MARKER_SYNC_ACQUIRE;
        /// The activity represents success in acquiring the user defined synchronization object.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_SYNC_ACQUIRE_SUCCESS = CUPTI_ACTIVITY_FLAG_MARKER_SYNC_ACQUIRE_SUCCESS;
        /// The activity represents failure in acquiring the user defined synchronization object.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_SYNC_ACQUIRE_FAILED = CUPTI_ACTIVITY_FLAG_MARKER_SYNC_ACQUIRE_FAILED;
        /// The activity represents releasing a reservation on user defined synchronization object.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_SYNC_RELEASE = CUPTI_ACTIVITY_FLAG_MARKER_SYNC_RELEASE;
        /// The activity represents a marker that does not specify a color.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER_DATA`.
        const MARKER_COLOR_NONE = CUPTI_ACTIVITY_FLAG_MARKER_COLOR_NONE;
        /// The activity represents a marker that specifies a color in alpha-red-green-blue format.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER_DATA`.
        const MARKER_COLOR_ARGB = CUPTI_ACTIVITY_FLAG_MARKER_COLOR_ARGB;
        /// The number of bytes requested by each thread.
        ///
        /// Valid for `CUpti_ActivityGlobalAccess3`.
        const GLOBAL_ACCESS_KIND_SIZE_MASK = CUPTI_ACTIVITY_FLAG_GLOBAL_ACCESS_KIND_SIZE_MASK;
        /// If bit in this flag is set, the access was load, else it is a store access.
        ///
        /// Valid for `CUpti_ActivityGlobalAccess3`.
        const GLOBAL_ACCESS_KIND_LOAD = CUPTI_ACTIVITY_FLAG_GLOBAL_ACCESS_KIND_LOAD;
        /// If this bit in flag is set, the load access was cached else it is uncached.
        ///
        /// Valid for `CUpti_ActivityGlobalAccess3`.
        const GLOBAL_ACCESS_KIND_CACHED = CUPTI_ACTIVITY_FLAG_GLOBAL_ACCESS_KIND_CACHED;
        /// If this bit in flag is set, the metric value overflowed.
        ///
        /// Valid for `CUpti_ActivityMetric` and `CUpti_ActivityMetricInstance`.
        const METRIC_OVERFLOWED = CUPTI_ACTIVITY_FLAG_METRIC_OVERFLOWED;
        /// If this bit in flag is set, the metric value couldn't be calculated.
        ///
        /// This occurs when a value(s) required to calculate the metric is missing. Valid for
        /// `CUpti_ActivityMetric` and `CUpti_ActivityMetricInstance`.
        const METRIC_VALUE_INVALID = CUPTI_ACTIVITY_FLAG_METRIC_VALUE_INVALID;
        /// If this bit in flag is set, the source level metric value couldn't be calculated.
        ///
        /// This occurs when a value(s) required to calculate the source level metric cannot be
        /// evaluated. Valid for `CUpti_ActivityInstructionExecution`.
        const INSTRUCTION_VALUE_INVALID = CUPTI_ACTIVITY_FLAG_INSTRUCTION_VALUE_INVALID;
        /// The mask for the instruction class, `CUpti_ActivityInstructionClass`.
        ///
        /// Valid for `CUpti_ActivityInstructionExecution` and `CUpti_ActivityInstructionCorrelation`.
        const INSTRUCTION_CLASS_MASK = CUPTI_ACTIVITY_FLAG_INSTRUCTION_CLASS_MASK;
        /// When calling `cuptiActivityFlushAll`, this flag can be set to force CUPTI to flush all
        /// records in the buffer, whether finished or not.
        const FLUSH_FORCED = CUPTI_ACTIVITY_FLAG_FLUSH_FORCED;
        /// The number of bytes requested by each thread.
        ///
        /// Valid for `CUpti_ActivitySharedAccess`.
        const SHARED_ACCESS_KIND_SIZE_MASK = CUPTI_ACTIVITY_FLAG_SHARED_ACCESS_KIND_SIZE_MASK;
        /// If bit in this flag is set, the access was load, else it is a store access.
        ///
        /// Valid for `CUpti_ActivitySharedAccess`.
        const SHARED_ACCESS_KIND_LOAD = CUPTI_ACTIVITY_FLAG_SHARED_ACCESS_KIND_LOAD;
        /// The activity represents an asynchronous memset operation.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MEMSET`.
        const MEMSET_ASYNC = CUPTI_ACTIVITY_FLAG_MEMSET_ASYNC;
        /// The activity represents thrashing in CPU.
        ///
        /// Valid for counter of kind `CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_KIND_THRASHING` in
        /// `CUPTI_ACTIVITY_KIND_UNIFIED_MEMORY_COUNTER`.
        const THRASHING_IN_CPU = CUPTI_ACTIVITY_FLAG_THRASHING_IN_CPU;
        /// The activity represents page throttling in CPU.
        ///
        /// Valid for counter of kind `CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_KIND_THROTTLING` in
        /// `CUPTI_ACTIVITY_KIND_UNIFIED_MEMORY_COUNTER`.
        const THROTTLING_IN_CPU = CUPTI_ACTIVITY_FLAG_THROTTLING_IN_CPU;
    }
}

c_enum! {
    /// The stall reason for PC sampling activity.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityPCSamplingStallReason : CUpti_ActivityPCSamplingStallReason {
        /// Invalid reason.
        Invalid = CUPTI_ACTIVITY_PC_SAMPLING_STALL_INVALID,
        /// No stall, instruction is selected for issue.
        None = CUPTI_ACTIVITY_PC_SAMPLING_STALL_NONE,
        /// Warp is blocked because next instruction is not yet available,
        /// because of instruction cache miss, or because of branching effects.
        InstFetch = CUPTI_ACTIVITY_PC_SAMPLING_STALL_INST_FETCH,
        /// Instruction is waiting on an arithmetic dependency.
        ExecDependency = CUPTI_ACTIVITY_PC_SAMPLING_STALL_EXEC_DEPENDENCY,
        /// Warp is blocked because it is waiting for a memory access to complete.
        MemoryDependency = CUPTI_ACTIVITY_PC_SAMPLING_STALL_MEMORY_DEPENDENCY,
        /// Texture sub-system is fully utilized or has too many outstanding requests.
        Texture = CUPTI_ACTIVITY_PC_SAMPLING_STALL_TEXTURE,
        /// Warp is blocked as it is waiting at `__syncthreads()` or at memory barrier.
        Sync = CUPTI_ACTIVITY_PC_SAMPLING_STALL_SYNC,
        /// Warp is blocked waiting for `__constant__` memory and immediate memory access to complete.
        ConstantMemoryDependency = CUPTI_ACTIVITY_PC_SAMPLING_STALL_CONSTANT_MEMORY_DEPENDENCY,
        /// Compute operation cannot be performed due to the required resources not
        /// being available.
        PipeBusy = CUPTI_ACTIVITY_PC_SAMPLING_STALL_PIPE_BUSY,
        /// Warp is blocked because there are too many pending memory operations.
        MemoryThrottle = CUPTI_ACTIVITY_PC_SAMPLING_STALL_MEMORY_THROTTLE,
        /// Warp was ready to issue, but some other warp issued instead.
        NotSelected = CUPTI_ACTIVITY_PC_SAMPLING_STALL_NOT_SELECTED,
        /// Miscellaneous reasons.
        Other = CUPTI_ACTIVITY_PC_SAMPLING_STALL_OTHER,
        /// Sleeping.
        Sleeping = CUPTI_ACTIVITY_PC_SAMPLING_STALL_SLEEPING,
    }
}

c_enum! {
    /// Sampling period for PC sampling method.
    ///
    /// Sampling period can be set using `cuptiActivityConfigurePCSampling`.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityPCSamplingPeriod : CUpti_ActivityPCSamplingPeriod {
        /// The PC sampling period is not set.
        Invalid = CUPTI_ACTIVITY_PC_SAMPLING_PERIOD_INVALID,
        /// Minimum sampling period available on the device.
        Min = CUPTI_ACTIVITY_PC_SAMPLING_PERIOD_MIN,
        /// Sampling period in lower range.
        Low = CUPTI_ACTIVITY_PC_SAMPLING_PERIOD_LOW,
        /// Medium sampling period.
        Mid = CUPTI_ACTIVITY_PC_SAMPLING_PERIOD_MID,
        /// Sampling period in higher range.
        High = CUPTI_ACTIVITY_PC_SAMPLING_PERIOD_HIGH,
        /// Maximum sampling period available on the device.
        Max = CUPTI_ACTIVITY_PC_SAMPLING_PERIOD_MAX,
    }
}

c_enum! {
    /// The kind of a memory copy, indicating the source and destination targets of the copy.
    ///
    /// Each kind represents the source and destination targets of a memory copy. Targets are
    /// host, device, and array.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityMemcpyKind : CUpti_ActivityMemcpyKind {
        /// The memory copy kind is not known.
        Unknown = CUPTI_ACTIVITY_MEMCPY_KIND_UNKNOWN,
        /// A host to device memory copy.
        Htod = CUPTI_ACTIVITY_MEMCPY_KIND_HTOD,
        /// A device to host memory copy.
        Dtoh = CUPTI_ACTIVITY_MEMCPY_KIND_DTOH,
        /// A host to device array memory copy.
        Htoa = CUPTI_ACTIVITY_MEMCPY_KIND_HTOA,
        /// A device array to host memory copy.
        Atoh = CUPTI_ACTIVITY_MEMCPY_KIND_ATOH,
        /// A device array to device array memory copy.
        Atoa = CUPTI_ACTIVITY_MEMCPY_KIND_ATOA,
        /// A device array to device memory copy.
        Atod = CUPTI_ACTIVITY_MEMCPY_KIND_ATOD,
        /// A device to device array memory copy.
        Dtoa = CUPTI_ACTIVITY_MEMCPY_KIND_DTOA,
        /// A device to device memory copy on the same device.
        Dtod = CUPTI_ACTIVITY_MEMCPY_KIND_DTOD,
        /// A host to host memory copy.
        Htoh = CUPTI_ACTIVITY_MEMCPY_KIND_HTOH,
        /// A peer to peer memory copy across different devices.
        Ptop = CUPTI_ACTIVITY_MEMCPY_KIND_PTOP,
    }
}

c_enum! {
    /// The kinds of memory accessed by a memory operation/copy.
    ///
    /// Each kind represents the type of the memory accessed by a memory operation/copy.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityMemoryKind : CUpti_ActivityMemoryKind {
        /// The memory kind is unknown.
        Unknown = CUPTI_ACTIVITY_MEMORY_KIND_UNKNOWN,
        /// The memory is pageable.
        Pageable = CUPTI_ACTIVITY_MEMORY_KIND_PAGEABLE,
        /// The memory is pinned.
        Pinned = CUPTI_ACTIVITY_MEMORY_KIND_PINNED,
        /// The memory is on the device.
        Device = CUPTI_ACTIVITY_MEMORY_KIND_DEVICE,
        /// The memory is an array.
        Array = CUPTI_ACTIVITY_MEMORY_KIND_ARRAY,
        /// The memory is managed.
        Managed = CUPTI_ACTIVITY_MEMORY_KIND_MANAGED,
        /// The memory is device static.
        DeviceStatic = CUPTI_ACTIVITY_MEMORY_KIND_DEVICE_STATIC,
        /// The memory is managed static.
        ManagedStatic = CUPTI_ACTIVITY_MEMORY_KIND_MANAGED_STATIC,
    }
}

c_enum! {
    /// The kind of a preemption activity.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityPreemptionKind : CUpti_ActivityPreemptionKind {
        /// The preemption kind is not known.
        Unknown = CUPTI_ACTIVITY_PREEMPTION_KIND_UNKNOWN,
        /// Preemption to save CDP block.
        Save = CUPTI_ACTIVITY_PREEMPTION_KIND_SAVE,
        /// Preemption to restore CDP block.
        Restore = CUPTI_ACTIVITY_PREEMPTION_KIND_RESTORE,
    }
}

c_enum! {
    /// The kind of environment data.
    ///
    /// Used to indicate what type of data is being reported by an environment activity record.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityEnvironmentKind : CUpti_ActivityEnvironmentKind {
        /// Unknown data.
        Unknown = CUPTI_ACTIVITY_ENVIRONMENT_UNKNOWN,
        /// The environment data is related to speed.
        Speed = CUPTI_ACTIVITY_ENVIRONMENT_SPEED,
        /// The environment data is related to temperature.
        Temperature = CUPTI_ACTIVITY_ENVIRONMENT_TEMPERATURE,
        /// The environment data is related to power.
        Power = CUPTI_ACTIVITY_ENVIRONMENT_POWER,
        /// The environment data is related to cooling.
        Cooling = CUPTI_ACTIVITY_ENVIRONMENT_COOLING,
    }
}

bitflags::bitflags! {
    /// Reasons for clock throttling.
    ///
    /// There could be more than one reason that is clock is being throttled so this is a bitfield.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EnvironmentClocksThrottleReason : CUpti_EnvironmentClocksThrottleReason {
        /// Nothing is running on the GPU and the clocks are dropping to idle state.
        const GPU_IDLE = CUPTI_CLOCKS_THROTTLE_REASON_GPU_IDLE;

        /// The GPU clocks are limited by a user specified limit.
        const USER_DEFINED_CLOCKS = CUPTI_CLOCKS_THROTTLE_REASON_USER_DEFINED_CLOCKS;

        /// A software power scaling algorithm is reducing the clocks below the
        /// requested clocks.
        const SW_POWER_CAP = CUPTI_CLOCKS_THROTTLE_REASON_SW_POWER_CAP;

        /// Hardware slowdown to reduce theclock by a factor of two or more is
        /// engaged.
        ///
        /// This is an indicator of one of the following:
        /// 1. temperature is too high,
        /// 2. external power brake assertion is being triggered (e.g. by power
        ///    system supply),
        /// 3. change in power state.
        const HW_SLOWDOWN = CUPTI_CLOCKS_THROTTLE_REASON_HW_SLOWDOWN;

        /// Some unspecified factor is reducing the clocks.
        const UNKNOWN = CUPTI_CLOCKS_THROTTLE_REASON_UNKNOWN;

        /// Throttle reason is not supported by this GPU.
        const UNSUPPORTED = CUPTI_CLOCKS_THROTTLE_REASON_UNSUPPORTED;
    }
}

c_enum! {
    /// Scope of the unified memory counter (deprecated in CUDA 7.0).
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityUnifiedMemoryCounterScope : CUpti_ActivityUnifiedMemoryCounterScope {
        /// The unified memory counter scope is not known.
        Unknown = CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_SCOPE_UNKNOWN,
        /// Collect unified memory counter for single process on one device.
        ProcessSingleDevice = CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_SCOPE_PROCESS_SINGLE_DEVICE,
        /// Collect unified memory counter for single process across all devices.
        ProcessAllDevices = CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_SCOPE_PROCESS_ALL_DEVICES,
    }
}

c_enum! {
    /// Kind of the Unified Memory counter.
    ///
    /// Many activities are associated with Unified Memory mechanism; among them
    /// are transfers from host to device, device to host, page fault at
    /// host side.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityUnifiedMemoryCounterKind : CUpti_ActivityUnifiedMemoryCounterKind {
        /// The unified memory counter kind is not known.
        Unknown = CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_KIND_UNKNOWN,
        /// Number of bytes transferred from host to device.
        BytesTransferHtoD = CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_KIND_BYTES_TRANSFER_HTOD,
        /// Number of bytes transferred from device to host.
        BytesTransferDtoH = CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_KIND_BYTES_TRANSFER_DTOH,
        /// Number of CPU page faults.
        ///
        /// # Notes
        ///
        /// This is only supported on 64 bit Linux and Mac platforms.
        CpuPageFaultCount = CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_KIND_CPU_PAGE_FAULT_COUNT,
        /// Number of GPU page faults.
        ///
        /// # Notes
        ///
        /// This is only supported on devices with compute capability 6.0 and higher and 64 bit Linux platforms.
        GpuPageFault = CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_KIND_GPU_PAGE_FAULT,
        /// Thrashing occurs when data is frequently accessed by multiple processors.
        ///
        /// Thrashing happens when data has to be constantly migrated around to achieve data locality.
        /// In this case the overhead of migration may exceed the benefits of locality.
        ///
        /// # Notes
        ///
        /// This is only supported on 64 bit Linux platforms.
        Thrashing = CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_KIND_THRASHING,
        /// Throttling is a prevention technique used by the driver to avoid further thrashing.
        ///
        /// Here, the driver doesn't service the fault for one of the contending processors for a
        /// specific period of time, so that the other processor can run at full-speed.
        ///
        /// # Notes
        ///
        /// This is only supported on 64 bit Linux platforms.
        Throttling = CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_KIND_THROTTLING,
        /// Remote map is used when throttling does not help.
        ///
        /// In case throttling does not help, the driver tries to pin the memory to a processor for
        /// a specific period of time. One of the contending processors will have slow access to the
        /// memory, while the other will have fast access.
        ///
        /// # Notes
        ///
        /// This is only supported on 64 bit Linux platforms.
        RemoteMap = CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_KIND_REMOTE_MAP,
        /// Number of bytes transferred from one device to another device.
        ///
        /// # Notes
        ///
        /// This is only supported on 64 bit Linux platforms.
        BytesTransferDtoD = CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_KIND_BYTES_TRANSFER_DTOD,
    }
}

c_enum! {
    /// Memory access type for unified memory page faults.
    ///
    /// This is valid for [`ActivityUnifiedMemoryCounterKind::GpuPageFault`] and
    /// [`ActivityUnifiedMemoryCounterKind::CpuPageFaultCount`].
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityUnifiedMemoryAccessType : CUpti_ActivityUnifiedMemoryAccessType {
        /// The unified memory access type is not known.
        Unknown = CUPTI_ACTIVITY_UNIFIED_MEMORY_ACCESS_TYPE_UNKNOWN,
        /// The page fault was triggered by read memory instruction.
        Read = CUPTI_ACTIVITY_UNIFIED_MEMORY_ACCESS_TYPE_READ,
        /// The page fault was triggered by write memory instruction.
        Write = CUPTI_ACTIVITY_UNIFIED_MEMORY_ACCESS_TYPE_WRITE,
        /// The page fault was triggered by atomic memory instruction.
        Atomic = CUPTI_ACTIVITY_UNIFIED_MEMORY_ACCESS_TYPE_ATOMIC,
        /// The page fault was triggered by memory prefetch operation.
        Prefetch = CUPTI_ACTIVITY_UNIFIED_MEMORY_ACCESS_TYPE_PREFETCH,
    }
}

c_enum! {
    /// Migration cause of the Unified Memory counter.
    ///
    /// This is valid for [`ActivityUnifiedMemoryCounterKind::BytesTransferHtoD`] and
    /// [`ActivityUnifiedMemoryCounterKind::BytesTransferDtoH`].
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityUnifiedMemoryMigrationCause : CUpti_ActivityUnifiedMemoryMigrationCause {
        /// The unified memory migration cause is not known.
        Unknown = CUPTI_ACTIVITY_UNIFIED_MEMORY_MIGRATION_CAUSE_UNKNOWN,
        /// The unified memory migrated due to an explicit call from the user.
        ///
        /// For example, `cudaMemPrefetchAsync`.
        User = CUPTI_ACTIVITY_UNIFIED_MEMORY_MIGRATION_CAUSE_USER,
        /// The unified memory migrated to guarantee data coherence.
        ///
        /// For example, CPU/GPU faults on Pascal+ and kernel launch on pre-Pascal GPUs.
        Coherence = CUPTI_ACTIVITY_UNIFIED_MEMORY_MIGRATION_CAUSE_COHERENCE,
        /// The unified memory was speculatively migrated by the UVM driver.
        ///
        /// The migration occurs before being accessed by the destination processor to improve performance.
        Prefetch = CUPTI_ACTIVITY_UNIFIED_MEMORY_MIGRATION_CAUSE_PREFETCH,
        /// The unified memory migrated to the CPU because it was evicted.
        ///
        /// Memory was evicted to make room for another block of memory on the GPU.
        Eviction = CUPTI_ACTIVITY_UNIFIED_MEMORY_MIGRATION_CAUSE_EVICTION,
        /// The unified memory migrated to another processor because of access counter notifications.
        ///
        /// Only frequently accessed pages are migrated between CPU and GPU, or between peer GPUs.
        AccessCounters = CUPTI_ACTIVITY_UNIFIED_MEMORY_MIGRATION_CAUSE_ACCESS_COUNTERS,
    }
}

c_enum! {
    /// Remote memory map cause of the Unified Memory counter.
    ///
    /// This is valid for [`ActivityUnifiedMemoryCounterKind::RemoteMap`].
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityUnifiedMemoryRemoteMapCause : CUpti_ActivityUnifiedMemoryRemoteMapCause {
        /// The cause of mapping to remote memory was unknown.
        Unknown = CUPTI_ACTIVITY_UNIFIED_MEMORY_REMOTE_MAP_CAUSE_UNKNOWN,
        /// Mapping to remote memory was added to maintain data coherence.
        Coherence = CUPTI_ACTIVITY_UNIFIED_MEMORY_REMOTE_MAP_CAUSE_COHERENCE,
        /// Mapping to remote memory was added to prevent further thrashing.
        Thrashing = CUPTI_ACTIVITY_UNIFIED_MEMORY_REMOTE_MAP_CAUSE_THRASHING,
        /// Mapping to remote memory was added to enforce hints.
        ///
        /// The hints are specified by the programmer or by performance heuristics of the UVM driver.
        Policy = CUPTI_ACTIVITY_UNIFIED_MEMORY_REMOTE_MAP_CAUSE_POLICY,
        /// Mapping to remote memory was added because there is no more memory available.
        ///
        /// The processor has no more memory available and eviction was not possible.
        OutOfMemory = CUPTI_ACTIVITY_UNIFIED_MEMORY_REMOTE_MAP_CAUSE_OUT_OF_MEMORY,
        /// Mapping to remote memory was added after the memory was evicted.
        ///
        /// The memory was evicted to make room for another block of memory on the GPU.
        Eviction = CUPTI_ACTIVITY_UNIFIED_MEMORY_REMOTE_MAP_CAUSE_EVICTION,
    }
}

c_enum! {
    /// SASS instruction classification.
    ///
    /// The SASS instructions are broadly divided into different classes. Each enum represents a classification.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityInstructionClass : CUpti_ActivityInstructionClass {
        /// The instruction class is not known.
        Unknown = CUPTI_ACTIVITY_INSTRUCTION_CLASS_UNKNOWN,
        /// Represents a 32 bit floating point operation.
        Fp32 = CUPTI_ACTIVITY_INSTRUCTION_CLASS_FP_32,
        /// Represents a 64 bit floating point operation.
        Fp64 = CUPTI_ACTIVITY_INSTRUCTION_CLASS_FP_64,
        /// Represents an integer operation.
        Integer = CUPTI_ACTIVITY_INSTRUCTION_CLASS_INTEGER,
        /// Represents a bit conversion operation.
        BitConversion = CUPTI_ACTIVITY_INSTRUCTION_CLASS_BIT_CONVERSION,
        /// Represents a control flow instruction.
        ControlFlow = CUPTI_ACTIVITY_INSTRUCTION_CLASS_CONTROL_FLOW,
        /// Represents a global load-store instruction.
        Global = CUPTI_ACTIVITY_INSTRUCTION_CLASS_GLOBAL,
        /// Represents a shared load-store instruction.
        Shared = CUPTI_ACTIVITY_INSTRUCTION_CLASS_SHARED,
        /// Represents a local load-store instruction.
        Local = CUPTI_ACTIVITY_INSTRUCTION_CLASS_LOCAL,
        /// Represents a generic load-store instruction.
        Generic = CUPTI_ACTIVITY_INSTRUCTION_CLASS_GENERIC,
        /// Represents a surface load-store instruction.
        Surface = CUPTI_ACTIVITY_INSTRUCTION_CLASS_SURFACE,
        /// Represents a constant load instruction.
        Constant = CUPTI_ACTIVITY_INSTRUCTION_CLASS_CONSTANT,
        /// Represents a texture load-store instruction.
        Texture = CUPTI_ACTIVITY_INSTRUCTION_CLASS_TEXTURE,
        /// Represents a global atomic instruction.
        GlobalAtomic = CUPTI_ACTIVITY_INSTRUCTION_CLASS_GLOBAL_ATOMIC,
        /// Represents a shared atomic instruction.
        SharedAtomic = CUPTI_ACTIVITY_INSTRUCTION_CLASS_SHARED_ATOMIC,
        /// Represents a surface atomic instruction.
        SurfaceAtomic = CUPTI_ACTIVITY_INSTRUCTION_CLASS_SURFACE_ATOMIC,
        /// Represents a inter-thread communication instruction.
        InterThreadCommunication = CUPTI_ACTIVITY_INSTRUCTION_CLASS_INTER_THREAD_COMMUNICATION,
        /// Represents a barrier instruction.
        Barrier = CUPTI_ACTIVITY_INSTRUCTION_CLASS_BARRIER,
        /// Represents some miscellaneous instructions which do not fit in the above classification.
        Miscellaneous = CUPTI_ACTIVITY_INSTRUCTION_CLASS_MISCELLANEOUS,
        /// Represents a 16 bit floating point operation.
        Fp16 = CUPTI_ACTIVITY_INSTRUCTION_CLASS_FP_16,
        /// Represents uniform instruction.
        Uniform = CUPTI_ACTIVITY_INSTRUCTION_CLASS_UNIFORM,
    }
}

c_enum! {
    /// Partitioned global caching option.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityPartitionedGlobalCacheConfig : CUpti_ActivityPartitionedGlobalCacheConfig {
        /// Partitioned global cache config unknown.
        Unknown = CUPTI_ACTIVITY_PARTITIONED_GLOBAL_CACHE_CONFIG_UNKNOWN,
        /// Partitioned global cache not supported.
        NotSupported = CUPTI_ACTIVITY_PARTITIONED_GLOBAL_CACHE_CONFIG_NOT_SUPPORTED,
        /// Partitioned global cache config off.
        Off = CUPTI_ACTIVITY_PARTITIONED_GLOBAL_CACHE_CONFIG_OFF,
        /// Partitioned global cache config on.
        On = CUPTI_ACTIVITY_PARTITIONED_GLOBAL_CACHE_CONFIG_ON,
    }
}

c_enum! {
    /// Synchronization type.
    ///
    /// The types of synchronization to be used with CUpti_ActivitySynchronization2.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivitySynchronizationType : CUpti_ActivitySynchronizationType {
        /// Unknown data.
        Unknown = CUPTI_ACTIVITY_SYNCHRONIZATION_TYPE_UNKNOWN,
        /// Event synchronize API.
        EventSynchronize = CUPTI_ACTIVITY_SYNCHRONIZATION_TYPE_EVENT_SYNCHRONIZE,
        /// Stream wait event API.
        StreamWaitEvent = CUPTI_ACTIVITY_SYNCHRONIZATION_TYPE_STREAM_WAIT_EVENT,
        /// Stream synchronize API.
        StreamSynchronize = CUPTI_ACTIVITY_SYNCHRONIZATION_TYPE_STREAM_SYNCHRONIZE,
        /// Context synchronize API.
        ContextSynchronize = CUPTI_ACTIVITY_SYNCHRONIZATION_TYPE_CONTEXT_SYNCHRONIZE,
    }
}

c_enum! {
    /// Stream type.
    ///
    /// The types of stream to be used with CUpti_ActivityStream.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityStreamFlag : CUpti_ActivityStreamFlag {
        /// Unknown data.
        Unknown = CUPTI_ACTIVITY_STREAM_CREATE_FLAG_UNKNOWN,
        /// Default stream.
        Default = CUPTI_ACTIVITY_STREAM_CREATE_FLAG_DEFAULT,
        /// Non-blocking stream.
        NonBlocking = CUPTI_ACTIVITY_STREAM_CREATE_FLAG_NON_BLOCKING,
        /// Null stream.
        Null = CUPTI_ACTIVITY_STREAM_CREATE_FLAG_NULL,
        /// Stream create Mask
        CreateMask = CUPTI_ACTIVITY_STREAM_CREATE_MASK,
    }
}

bitflags::bitflags! {
    /// Link flags.
    ///
    /// Describes link properties, to be used with NvLink activities.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LinkFlag : CUpti_LinkFlag {
        /// The flag is invalid.
        const INVALID = CUPTI_LINK_FLAG_INVALID;
        /// Is peer to peer access supported by this link.
        const PEER_ACCESS = CUPTI_LINK_FLAG_PEER_ACCESS;
        /// Is system memory access supported by this link.
        const SYSMEM_ACCESS = CUPTI_LINK_FLAG_SYSMEM_ACCESS;
        /// Is peer atomic access supported by this link.
        const PEER_ATOMICS = CUPTI_LINK_FLAG_PEER_ATOMICS;
        /// Is system memory atomic access supported by this link.
        const SYSMEM_ATOMICS = CUPTI_LINK_FLAG_SYSMEM_ATOMICS;
    }
}

c_enum! {
    /// Memory operation types.
    ///
    /// Describes the type of memory operation, to be used with `CUpti_ActivityMemory4`.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityMemoryOperationType : CUpti_ActivityMemoryOperationType {
        /// The operation is invalid.
        Invalid = CUPTI_ACTIVITY_MEMORY_OPERATION_TYPE_INVALID,
        /// Memory is allocated.
        Allocation = CUPTI_ACTIVITY_MEMORY_OPERATION_TYPE_ALLOCATION,
        /// Memory is released.
        Release = CUPTI_ACTIVITY_MEMORY_OPERATION_TYPE_RELEASE,
    }
}

c_enum! {
    /// Memory pool types.
    ///
    /// Describes the type of memory pool, to be used with `CUpti_ActivityMemory4`.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityMemoryPoolType : CUpti_ActivityMemoryPoolType {
        /// The operation is invalid.
        Invalid = CUPTI_ACTIVITY_MEMORY_POOL_TYPE_INVALID,
        /// Memory pool is local to the process.
        Local = CUPTI_ACTIVITY_MEMORY_POOL_TYPE_LOCAL,
        /// Memory pool is imported by the process.
        Imported = CUPTI_ACTIVITY_MEMORY_POOL_TYPE_IMPORTED,
    }
}

c_enum! {
    /// Memory pool operation types.
    ///
    /// Describes the type of memory pool operation, to be used with `CUpti_ActivityMemoryPool2`.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityMemoryPoolOperationType : CUpti_ActivityMemoryPoolOperationType {
        /// The operation is invalid.
        Invalid = CUPTI_ACTIVITY_MEMORY_POOL_OPERATION_TYPE_INVALID,
        /// Memory pool is created.
        Created = CUPTI_ACTIVITY_MEMORY_POOL_OPERATION_TYPE_CREATED,
        /// Memory pool is destroyed.
        Destroyed = CUPTI_ACTIVITY_MEMORY_POOL_OPERATION_TYPE_DESTROYED,
        /// Memory pool is trimmed.
        Trimmed = CUPTI_ACTIVITY_MEMORY_POOL_OPERATION_TYPE_TRIMMED,
    }
}

c_enum! {
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ChannelType : CUpti_ChannelType {
        Invalid = CUPTI_CHANNEL_TYPE_INVALID,

        /// Channel is used for standard work launch and tracking.
        Compute = CUPTI_CHANNEL_TYPE_COMPUTE,

        /// Channel is used by an asynchronous copy engine. For confidential
        /// compute configurations, work launch and completion are done using
        /// the copy engines.
        AsyncMemcpy = CUPTI_CHANNEL_TYPE_ASYNC_MEMCPY,

        /// Channel is used for memory decompression operations.
        Decomp = CUPTI_CHANNEL_TYPE_DECOMP,
    }
}

c_enum! {
    /// CIG (CUDA in Graphics) modes.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ContextCigMode : CUpti_ContextCigMode {
        /// Regular (non-CIG) mode.
        None = CUPTI_CONTEXT_CIG_MODE_NONE,

        /// CIG mode.
        Cig = CUPTI_CONTEXT_CIG_MODE_CIG,

        /// CIG fallback mode.
        CigFallback = CUPTI_CONTEXT_CIG_MODE_CIG_FALLBACK,
    }
}

c_enum! {
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum NvtxExtPayloadType : CUpti_NvtxExtPayloadType {
        /// The payload type is not known.
        Unknown = CUPTI_NVTX_EXT_PAYLOAD_TYPE_UNKNOWN,

        /// The payload type is a schema.
        Schema = CUPTI_NVTX_EXT_PAYLOAD_TYPE_SCHEMA,

        /// The payload type is an enum.
        Enum = CUPTI_NVTX_EXT_PAYLOAD_TYPE_ENUM,
    }
}

c_enum! {
    /// The type of the CUDA kernel launch.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityLaunchType : CUpti_ActivityLaunchType {
        /// The kernel was launched via a regular kernel call.
        Regular = CUPTI_ACTIVITY_LAUNCH_TYPE_REGULAR,
        /// The kernel was launched via API `cudaLaunchCooperativeKernel()` or
        /// `cuLaunchCooperativeKernel()`.
        CooperativeSingleDevice = CUPTI_ACTIVITY_LAUNCH_TYPE_COOPERATIVE_SINGLE_DEVICE,
        /// The kernel was launched via API `cudaLaunchCooperativeKernelMultiDevice()` or
        /// `cuLaunchCooperativeKernelMultiDevice()`.
        CooperativeMultiDevice = CUPTI_ACTIVITY_LAUNCH_TYPE_COOPERATIVE_MULTI_DEVICE,
        /// The kernel was launched as a CBL commandlist.
        CblCommandlist = CUPTI_ACTIVITY_LAUNCH_TYPE_CBL_COMMANDLIST,
    }
}

c_enum! {
    /// The shared memory limit per block config for a kernel.
    ///
    /// This should be used to set the `cudaOccFuncShmemConfig` field in the occupancy
    /// calculator API.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum FuncShmemLimitConfig : CUpti_FuncShmemLimitConfig {
        /// The shared memory limit config is the default.
        Default = CUPTI_FUNC_SHMEM_LIMIT_DEFAULT,

        /// The user has opted for a higher dynamic shared memory limit using function
        /// attribute `cudaFuncAttributeMaxDynamicSharedMemorySize` for the runtime API
        /// or `CU_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES` for the driver API.
        Optin = CUPTI_FUNC_SHMEM_LIMIT_OPTIN,
    }
}

c_enum! {
    /// The kind of external APIs supported for correlation.
    ///
    /// Custom correlation kinds are reserved for usage in external tools.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ExternalCorrelationKind : u32 {
        /// Invalid external correlation kind.
        Invalid = 0,
        /// The external API is unknown to CUPTI.
        Unknown = 1,
        /// The external API is OpenACC.
        Openacc = 2,
        /// The external API is custom0.
        Custom0 = 3,
        /// The external API is custom1.
        Custom1 = 4,
        /// The external API is custom2.
        Custom2 = 5,
    }
}

c_enum! {
    /// Field to differentiate whether PCIE Activity record is of a GPU or a PCI Bridge.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum PcieDeviceType: CUpti_PcieDeviceType {
        /// PCIE GPU record.
        Gpu = CUPTI_PCIE_DEVICE_TYPE_GPU,
        /// PCIE Bridge record.
        Bridge = CUPTI_PCIE_DEVICE_TYPE_BRIDGE,
    }
}

c_enum! {
    /// PCIE Generation.
    ///
    /// Enumeration of PCIE Generation for pcie activity attribute pcieGeneration.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum PcieGen: CUpti_PcieGen {
        /// PCIE Generation 1.
        Gen1 = 1,
        /// PCIE Generation 2.
        Gen2 = 2,
        /// PCIE Generation 3.
        Gen3 = 3,
        /// PCIE Generation 4.
        Gen4 = 4,
        /// PCIE Generation 5.
        Gen5 = 5,
        /// PCIE Generation 6.
        Gen6 = 6,
    }
}

c_enum! {
    /// Confidential Computing Rotation Events.
    ///
    /// Event types for confidential compute tracing.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ConfidentialComputeRotation: CUpti_ConfidentialComputeRotationEventType {
        /// Invalid rotation event.
        Invalid = CUPTI_CONFIDENTIAL_COMPUTE_INVALID_ROTATION_EVENT,
        /// This channel has been blocked from accepting new CUDA work so a key rotation can be done.
        KeyRotationChannelBlocked = CUPTI_CONFIDENTIAL_COMPUTE_KEY_ROTATION_CHANNEL_BLOCKED,
        /// This channel remains blocked and all queued CUDA work has completed.
        ///
        /// Other clients or channels may cause delays in starting the key rotation.
        KeyRotationChannelDrained = CUPTI_CONFIDENTIAL_COMPUTE_KEY_ROTATION_CHANNEL_DRAINED,
        /// Key rotations have completed and this channel is unblocked.
        KeyRotationChannelUnblocked = CUPTI_CONFIDENTIAL_COMPUTE_KEY_ROTATION_CHANNEL_UNBLOCKED,
    }
}

c_enum! {
    /// The types of JIT entry.
    pub enum ActivityJitEntryType: CUpti_ActivityJitEntryType {
        /// Invalid JIT entry type.
        Invalid = CUPTI_ACTIVITY_JIT_ENTRY_INVALID,
        /// PTX to CUBIN.
        PtxToCubin = CUPTI_ACTIVITY_JIT_ENTRY_PTX_TO_CUBIN,
        /// NVVM-IR to PTX.
        NvvmIrToPtx = CUPTI_ACTIVITY_JIT_ENTRY_NVVM_IR_TO_PTX,
    }
}

c_enum! {
    /// The types of JIT compilation operations.
    pub enum ActivityJitOperationType: CUpti_ActivityJitOperationType {
        /// Invalid JIT operation type.
        Invalid = CUPTI_ACTIVITY_JIT_OPERATION_INVALID,
        /// Loaded from the compute cache.
        CacheLoad = CUPTI_ACTIVITY_JIT_OPERATION_CACHE_LOAD,
        /// Stored in the compute cache.
        CacheStore = CUPTI_ACTIVITY_JIT_OPERATION_CACHE_STORE,
        /// JIT compilation.
        Compile = CUPTI_ACTIVITY_JIT_OPERATION_COMPILE,
    }
}

c_enum! {
    /// The launch mode for device graph execution.
    pub enum DeviceGraphLaunchMode: CUpti_DeviceGraphLaunchMode {
        Invalid = CUPTI_DEVICE_GRAPH_LAUNCH_MODE_INVALID,
        FireAndForget = CUPTI_DEVICE_GRAPH_LAUNCH_MODE_FIRE_AND_FORGET,
        Tail = CUPTI_DEVICE_GRAPH_LAUNCH_MODE_TAIL,
        FireAndForgetAsSibling = CUPTI_DEVICE_GRAPH_LAUNCH_MODE_FIRE_AND_FORGET_AS_SIBLING,
    }
}

c_enum! {
    /// Activity attributes.
    ///
    /// These attributes are used to control the behavior of the activity API.
    pub enum ActivityAttribute: CUpti_ActivityAttribute {
        /// The device memory size (in bytes) reserved for storing profiling data for concurrent
        /// kernels (activity kind CUPTI_ACTIVITY_KIND_CONCURRENT_KERNEL), memcopies and memsets
        /// for each buffer on a context. The value is a size_t.
        ///
        /// There is a limit on how many device buffers can be allocated per context. User
        /// can query and set this limit using the attribute
        /// `DEVICE_BUFFER_POOL_LIMIT`.
        /// CUPTI doesn't pre-allocate all the buffers, it pre-allocates only those many
        /// buffers as set by the attribute `DEVICE_BUFFER_PRE_ALLOCATE_VALUE`.
        /// When all of the data in a buffer is consumed, it is added in the reuse pool, and
        /// CUPTI picks a buffer from this pool when a new buffer is needed. Thus memory
        /// footprint does not scale with the kernel count. Applications with the high density
        /// of kernels, memcopies and memsets might result in having CUPTI to allocate more device buffers.
        /// CUPTI allocates another buffer only when it runs out of the buffers in the
        /// reuse pool.
        ///
        /// Since buffer allocation happens in the main application thread, this might result
        /// in stalls in the critical path. CUPTI pre-allocates 3 buffers of the same size to
        /// mitigate this issue. User can query and set the pre-allocation limit using the
        /// attribute `DEVICE_BUFFER_PRE_ALLOCATE_VALUE`.
        ///
        /// Having larger buffer size leaves less device memory for the application.
        /// Having smaller buffer size increases the risk of dropping timestamps for
        /// records if too many kernels or memcopies or memsets are launched at one time.
        ///
        /// This value only applies to new buffer allocations. Set this value before initializing
        /// CUDA or before creating a context to ensure it is considered for the following allocations.
        ///
        /// The default value is 3200000 (~3MB) which can accommodate profiling data
        /// up to 100,000 kernels, memcopies and memsets combined.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 12.0 Update 1 release, CUPTI allocates the profiling buffer
        /// in the device memory by default, which may improve the performance of the tracing run.
        /// To change the preferred location to page-locked host memory, refer to the attribute
        /// `MEM_ALLOCATION_TYPE_HOST_PINNED`.
        /// The size of the memory and maximum number of pools are still controlled by the attributes
        /// `DEVICE_BUFFER_SIZE` and `DEVICE_BUFFER_POOL_LIMIT`.
        ///
        /// The actual amount of device memory per buffer reserved by CUPTI might be larger.
        DeviceBufferSize = CUPTI_ACTIVITY_ATTR_DEVICE_BUFFER_SIZE,

        /// The device memory size (in bytes) reserved for storing profiling
        /// data for CDP operations for each buffer on a context. The
        /// value is a size_t.
        ///
        /// Having larger buffer size means less flush operations but
        /// consumes more device memory. This value only applies to new
        /// allocations.
        ///
        /// Set this value before initializing CUDA or before creating a
        /// context to ensure it is considered for the following allocations.
        ///
        /// The default value is 8388608 (8MB).
        ///
        /// # Notes
        ///
        /// The actual amount of device memory per context reserved by
        /// CUPTI might be larger.
        DeviceBufferSizeCdp = CUPTI_ACTIVITY_ATTR_DEVICE_BUFFER_SIZE_CDP,

        /// The maximum number of device memory buffers per context. The value is a size_t.
        ///
        /// For an application with high rate of kernel launches, memcopies and memsets having a bigger pool
        /// limit helps in timestamp collection for all these activities at the expense of a larger memory footprint.
        /// Refer to the description of the attribute `DEVICE_BUFFER_SIZE`
        /// for more details.
        ///
        /// Setting this value will not modify the number of memory buffers
        /// currently stored.
        ///
        /// Set this value before initializing CUDA to ensure the limit is
        /// not exceeded.
        ///
        /// The default value is 250.
        DeviceBufferPoolLimit = CUPTI_ACTIVITY_ATTR_DEVICE_BUFFER_POOL_LIMIT,

        /// This attribute is not supported starting with CUDA 12.3.
        /// CUPTI no longer uses profiling semaphore pool to store profiling data.
        ///
        /// There is a limit on how many semaphore pools can be allocated per context. User
        /// can query and set this limit using the attribute
        /// `PROFILING_SEMAPHORE_POOL_LIMIT`.
        /// CUPTI doesn't pre-allocate all the semaphore pools, it pre-allocates only those many
        /// semaphore pools as set by the attribute `PROFILING_SEMAPHORE_PRE_ALLOCATE_VALUE`.
        /// When all of the data in a semaphore pool is consumed, it is added in the reuse pool, and
        /// CUPTI picks a semaphore pool from the reuse pool when a new semaphore pool is needed. Thus memory
        /// footprint does not scale with the kernel count. Applications with the high density
        /// of kernels might result in having CUPTI to allocate more semaphore pools.
        /// CUPTI allocates another semaphore pool only when it runs out of the semaphore pools in the
        /// reuse pool.
        ///
        /// Since semaphore pool allocation happens in the main application thread, this might result
        /// in stalls in the critical path. CUPTI pre-allocates 3 semaphore pools of the same size to
        /// mitigate this issue. User can query and set the pre-allocation limit using the
        /// attribute `PROFILING_SEMAPHORE_PRE_ALLOCATE_VALUE`.
        ///
        /// Having larger semaphore pool size leaves less device memory for the application.
        /// Having smaller semaphore pool size increases the risk of dropping timestamps for
        /// kernel records if too many kernels are issued/launched at one time.
        ///
        /// This value only applies to new semaphore pool allocations. Set this value before initializing
        /// CUDA or before creating a context to ensure it is considered for the following allocations.
        ///
        /// The default value is 25000 which can accommodate profiling data for upto 25,000 kernels.
        ProfilingSemaphorePoolSize = CUPTI_ACTIVITY_ATTR_PROFILING_SEMAPHORE_POOL_SIZE,

        /// This attribute is not supported starting with CUDA 12.3.
        /// CUPTI no longer uses profiling semaphore pool to store profiling data.
        ///
        /// The maximum number of profiling semaphore pools per context. The value is a size_t.
        ///
        /// Refer to the description of the attribute `PROFILING_SEMAPHORE_POOL_SIZE`
        /// for more details.
        ///
        /// Set this value before initializing CUDA to ensure the limit is not exceeded.
        ///
        /// The default value is 250.
        ProfilingSemaphorePoolLimit = CUPTI_ACTIVITY_ATTR_PROFILING_SEMAPHORE_POOL_LIMIT,

        /// The flag to indicate whether user should provide activity buffer of zero value.
        /// The value is a uint8_t.
        ///
        /// If the value of this attribute is non-zero, user should provide
        /// a zero value buffer in the buffer request callback.
        /// If the user does not provide a zero value buffer after setting this to non-zero,
        /// the activity buffer may contain some uninitialized values when CUPTI returns it in
        /// the buffer complete callback.
        ///
        /// If the value of this attribute is zero, CUPTI will initialize the user buffer
        /// received in the buffer request callback to zero before filling it.
        /// If the user sets this to zero, a few stalls may appear in critical path because CUPTI
        /// will zero out the buffer in the main thread.
        /// Set this value before returning from the buffer request callback to
        /// ensure it is considered for all the subsequent user buffers.
        ///
        /// The default value is 0.
        ZeroedOutActivityBuffer = CUPTI_ACTIVITY_ATTR_ZEROED_OUT_ACTIVITY_BUFFER,

        /// Number of device buffers to pre-allocate for a context during the initialization phase.
        /// The value is a size_t.
        ///
        /// Refer to the description of the attribute `DEVICE_BUFFER_SIZE`
        /// for details.
        ///
        /// This value must be less than the maximum number of device buffers set using
        /// the attribute `DEVICE_BUFFER_POOL_LIMIT`.
        ///
        /// Set this value before initializing CUDA or before creating a context to ensure it
        /// is considered by the CUPTI.
        ///
        /// The default value is set to 3 to ping pong between these buffers (if possible).
        DeviceBufferPreAllocateValue = CUPTI_ACTIVITY_ATTR_DEVICE_BUFFER_PRE_ALLOCATE_VALUE,

        /// This attribute is not supported starting with CUDA 12.3.
        /// CUPTI no longer uses profiling semaphore pool to store profiling data.
        ///
        /// Number of profiling semaphore pools to pre-allocate for a context during the
        /// initialization phase. The value is a size_t.
        ///
        /// Refer to the description of the attribute `PROFILING_SEMAPHORE_POOL_SIZE`
        /// for details.
        ///
        /// This value must be less than the maximum number of profiling semaphore pools set
        /// using the attribute `PROFILING_SEMAPHORE_POOL_LIMIT`.
        ///
        /// Set this value before initializing CUDA or before creating a context to ensure it
        /// is considered by the CUPTI.
        ///
        /// The default value is set to 3 to ping pong between these pools (if possible).
        ProfilingSemaphorePreAllocateValue = CUPTI_ACTIVITY_ATTR_PROFILING_SEMAPHORE_PRE_ALLOCATE_VALUE,

        /// Allocate page-locked (pinned) host memory for storing profiling data for concurrent
        /// kernels, memcopies and memsets for each buffer on a context. The value is a uint8_t.
        ///
        /// From CUDA 11.2 through CUDA 12.0 GA releases, CUPTI allocated the profiling buffer
        /// in pinned host memory by default.
        /// Allocating excessive amounts of pinned memory may degrade system performance, as it
        /// reduces the amount of memory available to the system for paging. For this reason user
        /// might want to change the location from pinned host memory to device memory by setting
        /// value of this attribute to 0.
        ///
        /// # Notes
        ///
        /// Using page-locked (pinned) host memory buffers is not supported on confidential computing
        /// devices. If this attribute is set to 1, CUPTI will return error CUPTI_ERROR_NOT_SUPPORTED.
        ///
        /// The default value is 0.
        MemAllocationTypeHostPinned = CUPTI_ACTIVITY_ATTR_MEM_ALLOCATION_TYPE_HOST_PINNED,

        /// Request activity buffers per-thread to store CUPTI activity records
        /// in the activity buffer on per-thread basis. The value is a uint8_t.
        ///
        /// The attribute should be set before registering the buffer callbacks using
        /// cuptiActivityRegisterCallbacks API and before any of the CUPTI activity kinds are enabled.
        /// This makes sure that all the records are stored in activity buffers allocated per-thread.
        /// Changing this attribute in the middle of the profiling session will result in undefined behavior.
        ///
        /// The default value is 1.
        PerThreadActivityBuffer = CUPTI_ACTIVITY_ATTR_PER_THREAD_ACTIVITY_BUFFER,

        /// The device memory size (in bytes) reserved for storing profiling
        /// data for device graph operations for each buffer on a context. The
        /// value is a size_t.
        ///
        /// Having larger buffer size means less flush operations but
        /// consumes more device memory. This value only applies to new
        /// allocations.
        ///
        /// Set this value before initializing CUDA or before creating a
        /// context to ensure it is considered for the following allocations.
        ///
        /// The default value is 16777216 (16MB).
        ///
        /// # Notes
        ///
        /// The actual amount of device memory per context reserved by
        /// CUPTI might be larger.
        DeviceBufferSizeDeviceGraphs = CUPTI_ACTIVITY_ATTR_DEVICE_BUFFER_SIZE_DEVICE_GRAPHS,
    }
}

c_enum! {
    /// Thread-Id types.
    ///
    /// CUPTI uses different methods to obtain the thread-id depending on the
    /// support and the underlying platform. This enum documents these methods
    /// for each type.
    pub enum ActivityThreadIdType: CUpti_ActivityThreadIdType {
        /// Default type.
        ///
        /// Windows uses API GetCurrentThreadId().
        /// Linux/Mac/Android/QNX use POSIX pthread API pthread_self().
        Default = CUPTI_ACTIVITY_THREAD_ID_TYPE_DEFAULT,

        /// This type is based on the system API available on the underlying platform
        /// and thread-id obtained is supposed to be unique for the process lifetime.
        ///
        /// Windows uses API GetCurrentThreadId().
        /// Linux uses syscall SYS_gettid.
        /// Mac uses syscall SYS_thread_selfid.
        /// Android/QNX use gettid().
        System = CUPTI_ACTIVITY_THREAD_ID_TYPE_SYSTEM,
    }
}

/// Get the CUPTI timestamp.
///
/// Returns a timestamp normalized to correspond with the start and end
/// timestamps reported in the CUPTI activity records. The timestamp is reported
/// in nanoseconds.
pub fn get_timestamp() -> u64 {
    let mut timestamp = 0u64;
    let code = unsafe { cuptiGetTimestamp(&mut timestamp) };

    // The only documented error is if the timestamp parameter is null.
    // We panic here just in case, but there is no need to actually return
    // a result here.
    Error::result(code).expect("cuptiGetTimestamp returned an unexpected error");

    timestamp
}

pub fn enable(kind: ActivityKind) -> Result<()> {
    Error::result(unsafe { cuptiActivityEnable(kind.0) })
}

pub fn enable_and_dump(kind: ActivityKind) -> Result<()> {
    Error::result(unsafe { cuptiActivityEnableAndDump(kind.0) })
}

pub fn disable(kind: ActivityKind) -> Result<()> {
    Error::result(unsafe { cuptiActivityDisable(kind.0) })
}

pub fn enable_context(context: &Context, kind: ActivityKind) -> Result<()> {
    Error::result(unsafe { cuptiActivityEnableContext(context.as_raw(), kind.0) })
}

pub fn disable_context(context: &Context, kind: ActivityKind) -> Result<()> {
    Error::result(unsafe { cuptiActivityDisableContext(context.as_raw(), kind.0) })
}

pub fn get_num_dropped_records(context: &Context, stream_id: u32) -> Result<usize> {
    let mut num = 0;
    let code = unsafe { cuptiActivityGetNumDroppedRecords(context.as_raw(), stream_id, &mut num) };

    Error::result(code).map(|_| num)
}
