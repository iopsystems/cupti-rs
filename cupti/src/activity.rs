use c_enum::c_enum;
use cuda_sys::cuda::CUcontext;
use cupti_sys::*;

use crate::*;

c_enum! {
    /// The kinds of activity objects.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityObjectKind : CUpti_ActivityObjectKind {
        /// The object kind is not known.
        Unknown = 0,
        /// A process.
        Process = 1,
        /// A thread.
        Thread = 2,
        /// A device.
        Device = 3,
        /// A context.
        Context = 4,
        /// A stream.
        Stream = 5,
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
        Invalid = 0,
        /// A host<->host, host<->device, or device<->device memory copy.
        ///
        /// For peer to peer memory copy, use the kind [`Memcpy2`].
        ///
        /// [`Memcpy2`]: Self::Memcpy2
        Memcpy = 1,
        /// A memory set executing on the GPU.
        Memset = 2,
        /// A kernel executing on the GPU.
        ///
        /// This activity kind may significantly change the overall performance characteristics
        /// of the application because all kernel executions are serialized on the GPU. Other
        /// activity kind for kernel [`ConcurrentKernel`] doesn't break kernel concurrency.
        ///
        /// [`ConcurrentKernel`]: Self::ConcurrentKernel
        Kernel = 3,
        /// A CUDA driver API function execution.
        Driver = 4,
        /// A CUDA runtime API function execution.
        Runtime = 5,
        /// A performance counter (aka event) value.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Event API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        Event = 6,
        /// A performance metric value.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Metric API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        Metric = 7,
        /// Information about a CUDA device.
        Device = 8,
        /// Information about a CUDA context.
        Context = 9,
        /// A kernel executing on the GPU.
        ///
        /// This activity kind doesn't break kernel concurrency.
        ConcurrentKernel = 10,
        /// Resource naming done via NVTX APIs for thread, device, context, etc.
        Name = 11,
        /// Instantaneous, start, or end NVTX marker.
        Marker = 12,
        /// Extended, optional, data about a NVTX marker.
        ///
        /// User must enable [`Marker`] as well to get records for marker data.
        ///
        /// [`Marker`]: Self::Marker
        MarkerData = 13,
        /// Source information about source level result.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the SASS Metric APIs from the cupti_sass_metrics.h header.
        SourceLocator = 14,
        /// Results for source-level global access.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the SASS Metric APIs from the cupti_sass_metrics.h header.
        GlobalAccess = 15,
        /// Results for source-level branch.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the SASS Metric APIs from the cupti_sass_metrics.h header.
        Branch = 16,
        /// Overhead added by CUPTI, Compiler, CUDA driver etc.
        Overhead = 17,
        /// A CDP (CUDA Dynamic Parallel) kernel executing on the GPU.
        ///
        /// This activity cannot be directly enabled or disabled. It is enabled and disabled
        /// through concurrent kernel activity i.e. [`ConcurrentKernel`].
        ///
        /// [`ConcurrentKernel`]: Self::ConcurrentKernel
        CdpKernel = 18,
        /// Preemption activity record indicating a preemption of a CDP (CUDA Dynamic Parallel)
        /// kernel executing on the GPU.
        Preemption = 19,
        /// Environment activity records indicating power, clock, thermal, etc. levels of the GPU.
        Environment = 20,
        /// A performance counter value associated with a specific event domain instance.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Event API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        EventInstance = 21,
        /// A peer to peer memory copy.
        Memcpy2 = 22,
        /// A performance metric value associated with a specific metric domain instance.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Metric API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        MetricInstance = 23,
        /// Results for source-level instruction execution.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the SASS Metric APIs from the cupti_sass_metrics.h header.
        InstructionExecution = 24,
        /// Unified Memory counter record.
        UnifiedMemoryCounter = 25,
        /// Device global/function record.
        Function = 26,
        /// CUDA Module record.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the module callback can be stored in the corresponding activity record.
        Module = 27,
        /// A device attribute value.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// attributes `CUpti_DeviceAttribute` or `CUdevice_attribute` can be stored in the
        /// corresponding activity record.
        DeviceAttribute = 28,
        /// Results for source-level shared access.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the SASS Metric APIs from the cupti_sass_metrics.h header.
        SharedAccess = 29,
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
        PcSampling = 30,
        /// Summary information about PC sampling records.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        /// Enabling it will return the error code `CUPTI_ERROR_LEGACY_PROFILER_NOT_SUPPORTED`.
        /// Instead, use the PC Sampling API from the cupti_pcsampling.h header.
        PcSamplingRecordInfo = 31,
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
        InstructionCorrelation = 32,
        /// OpenACC data events.
        OpenaccData = 33,
        /// OpenACC launch events.
        OpenaccLaunch = 34,
        /// OpenACC other events.
        OpenaccOther = 35,
        /// Information about a CUDA event (cudaEvent).
        CudaEvent = 36,
        /// Information about a CUDA stream.
        Stream = 37,
        /// Records for CUDA synchronization primitives.
        Synchronization = 38,
        /// Records for correlation of different programming APIs.
        ExternalCorrelation = 39,
        /// NVLink topology information.
        Nvlink = 40,
        /// Instantaneous Event information.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Event API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        InstantaneousEvent = 41,
        /// Instantaneous Event information for a specific event domain instance.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Event API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        InstantaneousEventInstance = 42,
        /// Instantaneous Metric information.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Metric API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        InstantaneousMetric = 43,
        /// Instantaneous Metric information for a specific metric domain instance.
        ///
        /// This activity cannot be directly enabled or disabled. Information collected using
        /// the Metric API can be stored in the corresponding activity record.
        ///
        /// # Notes
        ///
        /// Starting with the CUDA 13.0 release, this enum is unsupported and should no longer be used.
        InstantaneousMetricInstance = 44,
        /// Memory activity tracking allocation and freeing of the memory.
        Memory = 45,
        /// PCI devices information used for PCI topology.
        Pcie = 46,
        /// OpenMP parallel events.
        Openmp = 47,
        /// A CUDA driver kernel launch occurring outside of any public API function execution.
        ///
        /// Tools can handle these like records for driver API launch functions, although
        /// the cbid field is not used here.
        InternalLaunchApi = 48,
        /// Memory activity tracking allocation and freeing of the memory.
        Memory2 = 49,
        /// Memory pool activity tracking creation, destruction and trimming of the memory pool.
        MemoryPool = 50,
        /// Activity record for graph-level information.
        GraphTrace = 51,
        /// JIT (Just-in-time) operation tracking.
        Jit = 52,
        /// Device graph trace activity.
        ///
        /// This activity cannot be directly enabled or disabled.
        /// It is enabled when [`GraphTrace`] is enabled and device graph trace is enabled
        /// through API `cuptiActivityEnableDeviceGraph()`.
        ///
        /// [`GraphTrace`]: Self::GraphTrace
        DeviceGraphTrace = 53,
        /// Tracing batches of copies that are to be decompressed.
        MemDecompress = 54,
        /// Tracing new overheads introduced on some hardware when confidential computing is enabled.
        ConfidentialComputeRotation = 55,
        /// Count of supported activity kinds.
        Count = 56,
    }
}

c_enum! {
    /// The kinds of activity overhead.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityOverheadKind : CUpti_ActivityOverheadKind {
        /// The overhead kind is not known.
        Unknown = 0,
        /// Compiler overhead.
        DriverCompiler = 1,
        /// Activity buffer flush overhead.
        CuptiBufferFlush = 1<<16,
        /// CUPTI instrumentation overhead.
        CuptiInstrumentation = 2<<16,
        /// CUPTI resource creation and destruction overhead.
        CuptiResource = 3<<16,
        /// CUDA Runtime triggered module loading overhead.
        RuntimeTriggeredModuleLoading = 4<<16,
        /// Lazy function loading overhead.
        LazyFunctionLoading = 5<<16,
        /// Overhead due to lack of command buffer space.
        ///
        /// Refer to `CUpti_ActivityOverheadCommandBufferFullData` for more details.
        CommandBufferFull = 6<<16,
        /// Overhead due to activity buffer request.
        ActivityBufferRequest = 7<<16,
        /// Overhead due to UVM activity initialization.
        UvmActivityInit = 8<<16,
    }
}

c_enum! {
    /// The kind of a compute API.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityComputeApiKind : CUpti_ActivityComputeApiKind {
        /// The compute API is not known.
        Unknown = 0,
        /// The compute APIs are for CUDA.
        Cuda = 1,
        /// The compute APIs are for CUDA running in MPS (Multi-Process Service) environment.
        CudaMps = 2,
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
        const NONE = 0;
        /// The activity represents a device that supports concurrent kernel execution.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_DEVICE`.
        const DEVICE_CONCURRENT_KERNELS = 1 << 0;
        /// If the activity represents a `CUdevice_attribute` value or a `CUpti_DeviceAttribute` value.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_DEVICE_ATTRIBUTE`.
        const DEVICE_ATTRIBUTE_CUDEVICE = 1 << 0;
        /// The activity represents an asynchronous memcpy operation.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MEMCPY`.
        const MEMCPY_ASYNC = 1 << 0;
        /// The activity represents an instantaneous marker.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_INSTANTANEOUS = 1 << 0;
        /// The activity represents a region start marker.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_START = 1 << 1;
        /// The activity represents a region end marker.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_END = 1 << 2;
        /// The activity represents an attempt to acquire a user defined synchronization object.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_SYNC_ACQUIRE = 1 << 3;
        /// The activity represents success in acquiring the user defined synchronization object.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_SYNC_ACQUIRE_SUCCESS = 1 << 4;
        /// The activity represents failure in acquiring the user defined synchronization object.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_SYNC_ACQUIRE_FAILED = 1 << 5;
        /// The activity represents releasing a reservation on user defined synchronization object.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER`.
        const MARKER_SYNC_RELEASE = 1 << 6;
        /// The activity represents a marker that does not specify a color.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER_DATA`.
        const MARKER_COLOR_NONE = 1 << 0;
        /// The activity represents a marker that specifies a color in alpha-red-green-blue format.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MARKER_DATA`.
        const MARKER_COLOR_ARGB = 1 << 1;
        /// The number of bytes requested by each thread.
        ///
        /// Valid for `CUpti_ActivityGlobalAccess3`.
        const GLOBAL_ACCESS_KIND_SIZE_MASK = 0xFF << 0;
        /// If bit in this flag is set, the access was load, else it is a store access.
        ///
        /// Valid for `CUpti_ActivityGlobalAccess3`.
        const GLOBAL_ACCESS_KIND_LOAD = 1 << 8;
        /// If this bit in flag is set, the load access was cached else it is uncached.
        ///
        /// Valid for `CUpti_ActivityGlobalAccess3`.
        const GLOBAL_ACCESS_KIND_CACHED = 1 << 9;
        /// If this bit in flag is set, the metric value overflowed.
        ///
        /// Valid for `CUpti_ActivityMetric` and `CUpti_ActivityMetricInstance`.
        const METRIC_OVERFLOWED = 1 << 0;
        /// If this bit in flag is set, the metric value couldn't be calculated.
        ///
        /// This occurs when a value(s) required to calculate the metric is missing. Valid for
        /// `CUpti_ActivityMetric` and `CUpti_ActivityMetricInstance`.
        const METRIC_VALUE_INVALID = 1 << 1;
        /// If this bit in flag is set, the source level metric value couldn't be calculated.
        ///
        /// This occurs when a value(s) required to calculate the source level metric cannot be
        /// evaluated. Valid for `CUpti_ActivityInstructionExecution`.
        const INSTRUCTION_VALUE_INVALID = 1 << 0;
        /// The mask for the instruction class, `CUpti_ActivityInstructionClass`.
        ///
        /// Valid for `CUpti_ActivityInstructionExecution` and `CUpti_ActivityInstructionCorrelation`.
        const INSTRUCTION_CLASS_MASK = 0xFF << 1;
        /// When calling `cuptiActivityFlushAll`, this flag can be set to force CUPTI to flush all
        /// records in the buffer, whether finished or not.
        const FLUSH_FORCED = 1 << 0;
        /// The number of bytes requested by each thread.
        ///
        /// Valid for `CUpti_ActivitySharedAccess`.
        const SHARED_ACCESS_KIND_SIZE_MASK = 0xFF << 0;
        /// If bit in this flag is set, the access was load, else it is a store access.
        ///
        /// Valid for `CUpti_ActivitySharedAccess`.
        const SHARED_ACCESS_KIND_LOAD = 1 << 8;
        /// The activity represents an asynchronous memset operation.
        ///
        /// Valid for `CUPTI_ACTIVITY_KIND_MEMSET`.
        const MEMSET_ASYNC = 1 << 0;
        /// The activity represents thrashing in CPU.
        ///
        /// Valid for counter of kind `CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_KIND_THRASHING` in
        /// `CUPTI_ACTIVITY_KIND_UNIFIED_MEMORY_COUNTER`.
        const THRASHING_IN_CPU = 1 << 0;
        /// The activity represents page throttling in CPU.
        ///
        /// Valid for counter of kind `CUPTI_ACTIVITY_UNIFIED_MEMORY_COUNTER_KIND_THROTTLING` in
        /// `CUPTI_ACTIVITY_KIND_UNIFIED_MEMORY_COUNTER`.
        const THROTTLING_IN_CPU = 1 << 0;
    }
}

c_enum! {
    /// The stall reason for PC sampling activity.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityPCSamplingStallReason : CUpti_ActivityPCSamplingStallReason {
        /// Invalid reason.
        Invalid = 0,
        /// No stall, instruction is selected for issue.
        None = 1,
        /// Warp is blocked because next instruction is not yet available,
        /// because of instruction cache miss, or because of branching effects.
        InstFetch = 2,
        /// Instruction is waiting on an arithmetic dependency.
        ExecDependency = 3,
        /// Warp is blocked because it is waiting for a memory access to complete.
        MemoryDependency = 4,
        /// Texture sub-system is fully utilized or has too many outstanding requests.
        Texture = 5,
        /// Warp is blocked as it is waiting at `__syncthreads()` or at memory barrier.
        Sync = 6,
        /// Warp is blocked waiting for `__constant__` memory and immediate memory access to complete.
        ConstantMemoryDependency = 7,
        /// Compute operation cannot be performed due to the required resources not
        /// being available.
        PipeBusy = 8,
        /// Warp is blocked because there are too many pending memory operations.
        MemoryThrottle = 9,
        /// Warp was ready to issue, but some other warp issued instead.
        NotSelected = 10,
        /// Miscellaneous reasons.
        Other = 11,
        /// Sleeping.
        Sleeping = 12,
    }
}

c_enum! {
    /// Sampling period for PC sampling method.
    ///
    /// Sampling period can be set using `cuptiActivityConfigurePCSampling`.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityPCSamplingPeriod : CUpti_ActivityPCSamplingPeriod {
        /// The PC sampling period is not set.
        Invalid = 0,
        /// Minimum sampling period available on the device.
        Min = 1,
        /// Sampling period in lower range.
        Low = 2,
        /// Medium sampling period.
        Mid = 3,
        /// Sampling period in higher range.
        High = 4,
        /// Maximum sampling period available on the device.
        Max = 5,
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
        Unknown = 0,
        /// A host to device memory copy.
        Htod = 1,
        /// A device to host memory copy.
        Dtoh = 2,
        /// A host to device array memory copy.
        Htoa = 3,
        /// A device array to host memory copy.
        Atoh = 4,
        /// A device array to device array memory copy.
        Atoa = 5,
        /// A device array to device memory copy.
        Atod = 6,
        /// A device to device array memory copy.
        Dtoa = 7,
        /// A device to device memory copy on the same device.
        Dtod = 8,
        /// A host to host memory copy.
        Htoh = 9,
        /// A peer to peer memory copy across different devices.
        Ptop = 10,
    }
}

c_enum! {
    /// The kinds of memory accessed by a memory operation/copy.
    ///
    /// Each kind represents the type of the memory accessed by a memory operation/copy.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityMemoryKind : CUpti_ActivityMemoryKind {
        /// The memory kind is unknown.
        Unknown = 0,
        /// The memory is pageable.
        Pageable = 1,
        /// The memory is pinned.
        Pinned = 2,
        /// The memory is on the device.
        Device = 3,
        /// The memory is an array.
        Array = 4,
        /// The memory is managed.
        Managed = 5,
        /// The memory is device static.
        DeviceStatic = 6,
        /// The memory is managed static.
        ManagedStatic = 7,
    }
}

c_enum! {
    /// The kind of a preemption activity.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityPreemptionKind : CUpti_ActivityPreemptionKind {
        /// The preemption kind is not known.
        Unknown = 0,
        /// Preemption to save CDP block.
        Save = 1,
        /// Preemption to restore CDP block.
        Restore = 2,
    }
}

c_enum! {
    /// The kind of environment data.
    ///
    /// Used to indicate what type of data is being reported by an environment activity record.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityEnvironmentKind : CUpti_ActivityEnvironmentKind {
        /// Unknown data.
        Unknown = 0,
        /// The environment data is related to speed.
        Speed = 1,
        /// The environment data is related to temperature.
        Temperature = 2,
        /// The environment data is related to power.
        Power = 3,
        /// The environment data is related to cooling.
        Cooling = 4,
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
        Unknown = 0,
        /// Collect unified memory counter for single process on one device.
        ProcessSingleDevice = 1,
        /// Collect unified memory counter for single process across all devices.
        ProcessAllDevices = 2,
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
        Unknown = 0,
        /// Number of bytes transferred from host to device.
        BytesTransferHtoD = 1,
        /// Number of bytes transferred from device to host.
        BytesTransferDtoH = 2,
        /// Number of CPU page faults.
        ///
        /// # Notes
        ///
        /// This is only supported on 64 bit Linux and Mac platforms.
        CpuPageFaultCount = 3,
        /// Number of GPU page faults.
        ///
        /// # Notes
        ///
        /// This is only supported on devices with compute capability 6.0 and higher and 64 bit Linux platforms.
        GpuPageFault = 4,
        /// Thrashing occurs when data is frequently accessed by multiple processors.
        ///
        /// Thrashing happens when data has to be constantly migrated around to achieve data locality.
        /// In this case the overhead of migration may exceed the benefits of locality.
        ///
        /// # Notes
        ///
        /// This is only supported on 64 bit Linux platforms.
        Thrashing = 5,
        /// Throttling is a prevention technique used by the driver to avoid further thrashing.
        ///
        /// Here, the driver doesn't service the fault for one of the contending processors for a
        /// specific period of time, so that the other processor can run at full-speed.
        ///
        /// # Notes
        ///
        /// This is only supported on 64 bit Linux platforms.
        Throttling = 6,
        /// Remote map is used when throttling does not help.
        ///
        /// In case throttling does not help, the driver tries to pin the memory to a processor for
        /// a specific period of time. One of the contending processors will have slow access to the
        /// memory, while the other will have fast access.
        ///
        /// # Notes
        ///
        /// This is only supported on 64 bit Linux platforms.
        RemoteMap = 7,
        /// Number of bytes transferred from one device to another device.
        ///
        /// # Notes
        ///
        /// This is only supported on 64 bit Linux platforms.
        BytesTransferDtoD = 8,
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
        Unknown = 0,
        /// The page fault was triggered by read memory instruction.
        Read = 1,
        /// The page fault was triggered by write memory instruction.
        Write = 2,
        /// The page fault was triggered by atomic memory instruction.
        Atomic = 3,
        /// The page fault was triggered by memory prefetch operation.
        Prefetch = 4,
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
        Unknown = 0,
        /// The unified memory migrated due to an explicit call from the user.
        ///
        /// For example, `cudaMemPrefetchAsync`.
        User = 1,
        /// The unified memory migrated to guarantee data coherence.
        ///
        /// For example, CPU/GPU faults on Pascal+ and kernel launch on pre-Pascal GPUs.
        Coherence = 2,
        /// The unified memory was speculatively migrated by the UVM driver.
        ///
        /// The migration occurs before being accessed by the destination processor to improve performance.
        Prefetch = 3,
        /// The unified memory migrated to the CPU because it was evicted.
        ///
        /// Memory was evicted to make room for another block of memory on the GPU.
        Eviction = 4,
        /// The unified memory migrated to another processor because of access counter notifications.
        ///
        /// Only frequently accessed pages are migrated between CPU and GPU, or between peer GPUs.
        AccessCounters = 5,
    }
}

c_enum! {
    /// Remote memory map cause of the Unified Memory counter.
    ///
    /// This is valid for [`ActivityUnifiedMemoryCounterKind::RemoteMap`].
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityUnifiedMemoryRemoteMapCause : CUpti_ActivityUnifiedMemoryRemoteMapCause {
        /// The cause of mapping to remote memory was unknown.
        Unknown = 0,
        /// Mapping to remote memory was added to maintain data coherence.
        Coherence = 1,
        /// Mapping to remote memory was added to prevent further thrashing.
        Thrashing = 2,
        /// Mapping to remote memory was added to enforce hints.
        ///
        /// The hints are specified by the programmer or by performance heuristics of the UVM driver.
        Policy = 3,
        /// Mapping to remote memory was added because there is no more memory available.
        ///
        /// The processor has no more memory available and eviction was not possible.
        OutOfMemory = 4,
        /// Mapping to remote memory was added after the memory was evicted.
        ///
        /// The memory was evicted to make room for another block of memory on the GPU.
        Eviction = 5,
    }
}

c_enum! {
    /// SASS instruction classification.
    ///
    /// The SASS instructions are broadly divided into different classes. Each enum represents a classification.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityInstructionClass : CUpti_ActivityInstructionClass {
        /// The instruction class is not known.
        Unknown = 0,
        /// Represents a 32 bit floating point operation.
        Fp32 = 1,
        /// Represents a 64 bit floating point operation.
        Fp64 = 2,
        /// Represents an integer operation.
        Integer = 3,
        /// Represents a bit conversion operation.
        BitConversion = 4,
        /// Represents a control flow instruction.
        ControlFlow = 5,
        /// Represents a global load-store instruction.
        Global = 6,
        /// Represents a shared load-store instruction.
        Shared = 7,
        /// Represents a local load-store instruction.
        Local = 8,
        /// Represents a generic load-store instruction.
        Generic = 9,
        /// Represents a surface load-store instruction.
        Surface = 10,
        /// Represents a constant load instruction.
        Constant = 11,
        /// Represents a texture load-store instruction.
        Texture = 12,
        /// Represents a global atomic instruction.
        GlobalAtomic = 13,
        /// Represents a shared atomic instruction.
        SharedAtomic = 14,
        /// Represents a surface atomic instruction.
        SurfaceAtomic = 15,
        /// Represents a inter-thread communication instruction.
        InterThreadCommunication = 16,
        /// Represents a barrier instruction.
        Barrier = 17,
        /// Represents some miscellaneous instructions which do not fit in the above classification.
        Miscellaneous = 18,
        /// Represents a 16 bit floating point operation.
        Fp16 = 19,
        /// Represents uniform instruction.
        Uniform = 20,
    }
}

c_enum! {
    /// Partitioned global caching option.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityPartitionedGlobalCacheConfig : CUpti_ActivityPartitionedGlobalCacheConfig {
        /// Partitioned global cache config unknown.
        Unknown = 0,
        /// Partitioned global cache not supported.
        NotSupported = 1,
        /// Partitioned global cache config off.
        Off = 2,
        /// Partitioned global cache config on.
        On = 3,
    }
}

c_enum! {
    /// Synchronization type.
    ///
    /// The types of synchronization to be used with CUpti_ActivitySynchronization2.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivitySynchronizationType : CUpti_ActivitySynchronizationType {
        /// Unknown data.
        Unknown = 0,
        /// Event synchronize API.
        EventSynchronize = 1,
        /// Stream wait event API.
        StreamWaitEvent = 2,
        /// Stream synchronize API.
        StreamSynchronize = 3,
        /// Context synchronize API.
        ContextSynchronize = 4,
    }
}

c_enum! {
    /// Stream type.
    ///
    /// The types of stream to be used with CUpti_ActivityStream.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityStreamFlag : CUpti_ActivityStreamFlag {
        /// Unknown data.
        Unknown = 0,
        /// Default stream.
        Default = 1,
        /// Non-blocking stream.
        NonBlocking = 2,
        /// Null stream.
        Null = 3,
        /// Stream create Mask
        CreateMask = 0xFFFF,
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
        Invalid = 0,
        /// Memory is allocated.
        Allocation = 1,
        /// Memory is released.
        Release = 2,
    }
}

c_enum! {
    /// Memory pool types.
    ///
    /// Describes the type of memory pool, to be used with `CUpti_ActivityMemory4`.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityMemoryPoolType : CUpti_ActivityMemoryPoolType {
        /// The operation is invalid.
        Invalid = 0,
        /// Memory pool is local to the process.
        Local = 1,
        /// Memory pool is imported by the process.
        Imported = 2,
    }
}

c_enum! {
    /// Memory pool operation types.
    ///
    /// Describes the type of memory pool operation, to be used with `CUpti_ActivityMemoryPool2`.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ActivityMemoryPoolOperationType : CUpti_ActivityMemoryPoolOperationType {
        /// The operation is invalid.
        Invalid = 0,
        /// Memory pool is created.
        Created = 1,
        /// Memory pool is destroyed.
        Destroyed = 2,
        /// Memory pool is trimmed.
        Trimmed = 3,
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
        Regular = 0,
        /// The kernel was launched via API `cudaLaunchCooperativeKernel()` or
        /// `cuLaunchCooperativeKernel()`.
        CooperativeSingleDevice = 1,
        /// The kernel was launched via API `cudaLaunchCooperativeKernelMultiDevice()` or
        /// `cuLaunchCooperativeKernelMultiDevice()`.
        CooperativeMultiDevice = 2,
        /// The kernel was launched as a CBL commandlist.
        CblCommandlist = 3,
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
        Gpu = 0,
        /// PCIE Bridge record.
        Bridge = 1,
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
        Invalid = 0,
        /// This channel has been blocked from accepting new CUDA work so a key rotation can be done.
        KeyRotationChannelBlocked = 1,
        /// This channel remains blocked and all queued CUDA work has completed.
        ///
        /// Other clients or channels may cause delays in starting the key rotation.
        KeyRotationChannelDrained = 2,
        /// Key rotations have completed and this channel is unblocked.
        KeyRotationChannelUnblocked = 3,
    }
}

c_enum! {
    /// The types of JIT entry.
    pub enum ActivityJitEntryType: u32 {
        /// Invalid JIT entry type.
        Invalid = 0,
        /// PTX to CUBIN.
        PtxToCubin = 1,
        /// NVVM-IR to PTX.
        NvvmIrToPtx = 2,
    }
}

c_enum! {
    /// The types of JIT compilation operations.
    pub enum ActivityJitOperationType: u32 {
        /// Invalid JIT operation type.
        Invalid = 0,
        /// Loaded from the compute cache.
        CacheLoad = 1,
        /// Stored in the compute cache.
        CacheStore = 2,
        /// JIT compilation.
        Compile = 3,
    }
}

c_enum! {
    /// The launch mode for device graph execution.
    pub enum DeviceGraphLaunchMode: u32 {
        Invalid = 0,
        FireAndForget = 1,
        Tail = 2,
        FireAndForgetAsSibling = 3,
    }
}

c_enum! {
    /// Activity attributes.
    ///
    /// These attributes are used to control the behavior of the activity API.
    pub enum ActivityAttribute: u32 {
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
        DeviceBufferSize = 0,

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
        DeviceBufferSizeCdp = 1,

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
        DeviceBufferPoolLimit = 2,

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
        ProfilingSemaphorePoolSize = 3,

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
        ProfilingSemaphorePoolLimit = 4,

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
        ZeroedOutActivityBuffer = 5,

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
        DeviceBufferPreAllocateValue = 6,

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
        ProfilingSemaphorePreAllocateValue = 7,

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
        MemAllocationTypeHostPinned = 8,

        /// Request activity buffers per-thread to store CUPTI activity records
        /// in the activity buffer on per-thread basis. The value is a uint8_t.
        ///
        /// The attribute should be set before registering the buffer callbacks using
        /// cuptiActivityRegisterCallbacks API and before any of the CUPTI activity kinds are enabled.
        /// This makes sure that all the records are stored in activity buffers allocated per-thread.
        /// Changing this attribute in the middle of the profiling session will result in undefined behavior.
        ///
        /// The default value is 1.
        PerThreadActivityBuffer = 9,

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
        DeviceBufferSizeDeviceGraphs = 10,
    }
}

c_enum! {
    /// Thread-Id types.
    ///
    /// CUPTI uses different methods to obtain the thread-id depending on the
    /// support and the underlying platform. This enum documents these methods
    /// for each type.
    pub enum ActivityThreadIdType: u32 {
        /// Default type.
        ///
        /// Windows uses API GetCurrentThreadId().
        /// Linux/Mac/Android/QNX use POSIX pthread API pthread_self().
        Default = 0,

        /// This type is based on the system API available on the underlying platform
        /// and thread-id obtained is supposed to be unique for the process lifetime.
        ///
        /// Windows uses API GetCurrentThreadId().
        /// Linux uses syscall SYS_gettid.
        /// Mac uses syscall SYS_thread_selfid.
        /// Android/QNX use gettid().
        System = 1,

        /// Sentinel value representing the count of valid thread ID types.
        Size = 2,
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
