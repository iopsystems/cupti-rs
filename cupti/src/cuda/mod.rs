//! Bindings for types in cuda.h.

use std::ffi::{c_int, c_uint};

use c_enum::c_enum;
use cuda_sys::cuda::CUevent;
use cupti_sys::*;

mod context;
mod graph;
mod stream;

pub use self::context::Context;
pub use self::stream::Stream;
pub use self::graph::{Graph, GraphExec, GraphNode};

c_enum! {
    /// Launch attribute IDs.
    ///
    /// Kernel launch attributes specify launch properties that differ from defaults.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub enum LaunchAttributeID : CUlaunchAttributeID {
        /// Ignored entry, for convenient composition.
        Ignore = CU_LAUNCH_ATTRIBUTE_IGNORE,

        /// Valid for streams, graph nodes, launches.
        ///
        /// See [`CUlaunchAttributeValue::accessPolicyWindow`].
        AccessPolicyWindow = CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW,

        /// Valid for graph nodes, launches.
        ///
        /// See [`CUlaunchAttributeValue::cooperative`].
        Cooperative = CU_LAUNCH_ATTRIBUTE_COOPERATIVE,

        /// Valid for streams.
        ///
        /// See [`CUlaunchAttributeValue::syncPolicy`].
        SynchronizationPolicy = CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY,

        /// Valid for graph nodes, launches.
        ///
        /// See [`CUlaunchAttributeValue::clusterDim`].
        ClusterDimension = CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION,

        /// Valid for graph nodes, launches.
        ///
        /// See [`CUlaunchAttributeValue::clusterSchedulingPolicyPreference`].
        ClusterSchedulingPolicyPreference = CU_LAUNCH_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE,

        /// Valid for launches.
        ///
        /// Setting [`CUlaunchAttributeValue::programmaticStreamSerializationAllowed`] to non-0
        /// signals that the kernel will use programmatic means to resolve its stream dependency,
        /// so that the CUDA runtime should opportunistically allow the grid's execution to overlap
        /// with the previous kernel in the stream, if that kernel requests the overlap. The
        /// dependent launches can choose to wait on the dependency using the programmatic sync
        /// (cudaGridDependencySynchronize() or equivalent PTX instructions).
        ProgrammaticStreamSerialization = CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_STREAM_SERIALIZATION,

        /// Valid for launches.
        ///
        /// Set [`CUlaunchAttributeValue::programmaticEvent`] to record the event. Event recorded
        /// through this launch attribute is guaranteed to only trigger after all block in the
        /// associated kernel trigger the event. A block can trigger the event through PTX
        /// launchdep.release or CUDA builtin function cudaTriggerProgrammaticLaunchCompletion().
        /// A trigger can also be inserted at the beginning of each block's execution if
        /// triggerAtBlockStart is set to non-0. The dependent launches can choose to wait on the
        /// dependency using the programmatic sync (cudaGridDependencySynchronize() or equivalent
        /// PTX instructions). Note that dependents (including the CPU thread calling
        /// cuEventSynchronize()) are not guaranteed to observe the release precisely when it is
        /// released. For example, cuEventSynchronize() may only observe the event trigger long
        /// after the associated kernel has completed. This recording type is primarily meant for
        /// establishing programmatic dependency between device tasks. Note also this type of
        /// dependency allows, but does not guarantee, concurrent execution of tasks.
        ///
        /// The event supplied must not be an interprocess or interop event. The event must
        /// disable timing (i.e. must be created with the `CU_EVENT_DISABLE_TIMING` flag set).
        ProgrammaticEvent = CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_EVENT,

        /// Valid for streams, graph nodes, launches.
        ///
        /// See [`CUlaunchAttributeValue::priority`].
        Priority = CU_LAUNCH_ATTRIBUTE_PRIORITY,

        /// Valid for streams, graph nodes, launches.
        ///
        /// See [`CUlaunchAttributeValue::memSyncDomainMap`].
        MemSyncDomainMap = CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP,

        /// Valid for streams, graph nodes, launches.
        ///
        /// See [`CUlaunchAttributeValue::memSyncDomain`].
        MemSyncDomain = CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN,

        /// Valid for graph nodes, launches.
        ///
        /// Set [`CUlaunchAttributeValue::preferredClusterDim`] to allow the kernel launch to
        /// specify a preferred substitute cluster dimension. Blocks may be grouped according to
        /// either the dimensions specified with this attribute (grouped into a "preferred
        /// substitute cluster"), or the one specified with [`ClusterDimension`] attribute (grouped
        /// into a "regular cluster"). The cluster dimensions of a "preferred substitute cluster"
        /// shall be an integer multiple greater than zero of the regular cluster dimensions. The
        /// device will attempt - on a best-effort basis - to group thread blocks into preferred
        /// clusters over grouping them into regular clusters. When it deems necessary (primarily
        /// when the device temporarily runs out of physical resources to launch the larger
        /// preferred clusters), the device may switch to launch the regular clusters instead to
        /// attempt to utilize as much of the physical device resources as possible.
        ///
        /// Each type of cluster will have its enumeration / coordinate setup as if the grid
        /// consists solely of its type of cluster. For example, if the preferred substitute
        /// cluster dimensions double the regular cluster dimensions, there might be simultaneously
        /// a regular cluster indexed at (1,0,0), and a preferred cluster indexed at (1,0,0). In
        /// this example, the preferred substitute cluster (1,0,0) replaces regular clusters
        /// (2,0,0) and (3,0,0) and groups their blocks.
        ///
        /// This attribute will only take effect when a regular cluster dimension has been
        /// specified. The preferred substitute cluster dimension must be an integer multiple
        /// greater than zero of the regular cluster dimension and must divide the grid. It must
        /// also be no more than `maxBlocksPerCluster`, if it is set in the kernel's
        /// `__launch_bounds__`. Otherwise it must be less than the maximum value the driver can
        /// support. Otherwise, setting this attribute to a value physically unable to fit on any
        /// particular device is permitted.
        ///
        /// [`ClusterDimension`]: Self::ClusterDimension
        PreferredClusterDimension = CU_LAUNCH_ATTRIBUTE_PREFERRED_CLUSTER_DIMENSION,

        /// Valid for launches.
        ///
        /// Set [`CUlaunchAttributeValue::launchCompletionEvent`] to record the event.
        ///
        /// Nominally, the event is triggered once all blocks of the kernel have begun execution.
        /// Currently this is a best effort. If a kernel B has a launch completion dependency on a
        /// kernel A, B may wait until A is complete. Alternatively, blocks of B may begin before
        /// all blocks of A have begun, for example if B can claim execution resources unavailable
        /// to A (e.g. they run on different GPUs) or if B is a higher priority than A. Exercise
        /// caution if such an ordering inversion could lead to deadlock.
        ///
        /// A launch completion event is nominally similar to a programmatic event with
        /// `triggerAtBlockStart` set except that it is not visible to `cudaGridDependencySynchronize()`
        /// and can be used with compute capability less than 9.0.
        ///
        /// The event supplied must not be an interprocess or interop event. The event must
        /// disable timing (i.e. must be created with the `CU_EVENT_DISABLE_TIMING` flag set).
        LaunchCompletionEvent = CU_LAUNCH_ATTRIBUTE_LAUNCH_COMPLETION_EVENT,

        /// Valid for graph nodes, launches. This attribute is graphs-only.
        ///
        /// Passing this attribute to a launch in a non-capturing stream will result in an error.
        ///
        /// [`CUlaunchAttributeValue::deviceUpdatableKernelNode::deviceUpdatable`] can only be set
        /// to 0 or 1. Setting the field to 1 indicates that the corresponding kernel node should
        /// be device-updatable. On success, a handle will be returned via
        /// [`CUlaunchAttributeValue::deviceUpdatableKernelNode::devNode`] which can be passed to
        /// the various device-side update functions to update the node's kernel parameters from
        /// within another kernel. For more information on the types of device updates that can be
        /// made, as well as the relevant limitations thereof, see `cudaGraphKernelNodeUpdatesApply`.
        ///
        /// Nodes which are device-updatable have additional restrictions compared to regular
        /// kernel nodes. Firstly, device-updatable nodes cannot be removed from their graph via
        /// `cuGraphDestroyNode`. Additionally, once opted-in to this functionality, a node cannot
        /// opt out, and any attempt to set the deviceUpdatable attribute to 0 will result in an
        /// error. Device-updatable kernel nodes also cannot have their attributes copied to/from
        /// another kernel node via `cuGraphKernelNodeCopyAttributes`. Graphs containing one or
        /// more device-updatable nodes also do not allow multiple instantiation, and neither the
        /// graph nor its instantiated version can be passed to `cuGraphExecUpdate`.
        ///
        /// If a graph contains device-updatable nodes and updates those nodes from the device
        /// from within the graph, the graph must be uploaded with `cuGraphUpload` before it is
        /// launched. For such a graph, if host-side executable graph updates are made to the
        /// device-updatable nodes, the graph must be uploaded before it is launched again.
        DeviceUpdatableKernelNode = CU_LAUNCH_ATTRIBUTE_DEVICE_UPDATABLE_KERNEL_NODE,

        /// Valid for launches.
        ///
        /// On devices where the L1 cache and shared memory use the same hardware resources,
        /// setting [`CUlaunchAttributeValue::sharedMemCarveout`] to a percentage between 0-100
        /// signals the CUDA driver to set the shared memory carveout preference, in percent of
        /// the total shared memory for that kernel launch. This attribute takes precedence over
        /// `CU_FUNC_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT`. This is only a hint, and the
        /// CUDA driver can choose a different configuration if required for the launch.
        PreferredSharedMemoryCarveout = CU_LAUNCH_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT,

        /// Valid for streams, graph nodes, launches.
        ///
        /// This attribute is a hint to the CUDA runtime that the launch should attempt to make
        /// the kernel maximize its NVLINK utilization.
        ///
        /// When possible to honor this hint, CUDA will assume each block in the grid launch will
        /// carry out an even amount of NVLINK traffic, and make a best-effort attempt to adjust
        /// the kernel launch based on that assumption.
        ///
        /// This attribute is a hint only. CUDA makes no functional or performance guarantee. Its
        /// applicability can be affected by many different factors, including driver version
        /// (i.e. CUDA doesn't guarantee the performance characteristics will be maintained
        /// between driver versions or a driver update could alter or regress previously observed
        /// perf characteristics.) It also doesn't guarantee a successful result, i.e. applying
        /// the attribute may not improve the performance of either the targeted kernel or the
        /// encapsulating application.
        ///
        /// Valid values for [`CUlaunchAttributeValue::nvlinkUtilCentricScheduling`] are 0
        /// (disabled) and 1 (enabled).
        NvlinkUtilCentricScheduling = CU_LAUNCH_ATTRIBUTE_NVLINK_UTIL_CENTRIC_SCHEDULING,
    }
}

c_enum! {
    /// Stream attribute IDs.
    ///
    /// Stream attributes specify properties that can be set on CUDA streams.
    /// These are a subset of launch attributes that are valid for streams.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub enum StreamAttributeID : CUstreamAttrID {
        /// Valid for streams.
        ///
        /// See [`CUlaunchAttributeValue::accessPolicyWindow`].
        AccessPolicyWindow = CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW,

        /// Valid for streams.
        ///
        /// See [`CUlaunchAttributeValue::syncPolicy`].
        SynchronizationPolicy = CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY,

        /// Valid for streams.
        ///
        /// See [`CUlaunchAttributeValue::priority`].
        Priority = CU_LAUNCH_ATTRIBUTE_PRIORITY,

        /// Valid for streams.
        ///
        /// See [`CUlaunchAttributeValue::memSyncDomainMap`].
        MemSyncDomainMap = CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP,

        /// Valid for streams.
        ///
        /// See [`CUlaunchAttributeValue::memSyncDomain`].
        MemSyncDomain = CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN,
    }
}

c_enum! {
    /// Graph node types.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub enum GraphNodeType : CUgraphNodeType {
        /// GPU kernel node.
        Kernel = CU_GRAPH_NODE_TYPE_KERNEL,

        /// Memcpy node.
        Memcpy = CU_GRAPH_NODE_TYPE_MEMCPY,

        /// Memset node.
        Memset = CU_GRAPH_NODE_TYPE_MEMSET,

        /// Host (executable) node.
        Host = CU_GRAPH_NODE_TYPE_HOST,

        /// Node which executes an embedded graph.
        Graph = CU_GRAPH_NODE_TYPE_GRAPH,

        /// Empty (no-op) node.
        Empty = CU_GRAPH_NODE_TYPE_EMPTY,

        /// External event wait node.
        WaitEvent = CU_GRAPH_NODE_TYPE_WAIT_EVENT,

        /// External event record node.
        EventRecord = CU_GRAPH_NODE_TYPE_EVENT_RECORD,

        /// External semaphore signal node.
        ExtSemasSignal = CU_GRAPH_NODE_TYPE_EXT_SEMAS_SIGNAL,

        /// External semaphore wait node.
        ExtSemasWait = CU_GRAPH_NODE_TYPE_EXT_SEMAS_WAIT,

        /// Memory allocation node.
        MemAlloc = CU_GRAPH_NODE_TYPE_MEM_ALLOC,

        /// Memory free node.
        MemFree = CU_GRAPH_NODE_TYPE_MEM_FREE,

        /// Batch memory operation node.
        BatchMemOp = CU_GRAPH_NODE_TYPE_BATCH_MEM_OP,

        /// Conditional node.
        Conditional = CU_GRAPH_NODE_TYPE_CONDITIONAL,
    }
}

/// Stream attributes.
///
/// This is a subset of the values allowed for a full launch attribute
pub enum StreamAttributeValue<'a> {
    /// Value of launch attribute [`CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW`].
    AccessPolicyWindow(CUaccessPolicyWindow),

    /// Value of launch attribute
    /// [`CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY`].
    ///
    /// `CUsynchronizationPolicy` for work queued up in this stream.
    SynchronizationPolicy(CUsynchronizationPolicy),

    /// Value of launch attribute [`CU_LAUNCH_ATTRIBUTE_PRIORITY`].
    ///
    /// Execution priority of the kernel.
    Priority(c_int),

    /// Value of launch attribute [`CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP`].
    ///
    /// See `CUlaunchMemSyncDomainMap`.
    MemSyncDomainMap(CUlaunchMemSyncDomainMap),

    /// Value of launch attribute [`CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN`].
    ///
    /// See `CUlaunchMemSyncDomain`.
    MemSyncDomain(CUlaunchMemSyncDomain),

    /// A variant known to this library.
    Unknown(&'a CUlaunchAttributeValue),
}

/// Launch attributes union; used as value field of `CUlaunchAttribute`.
pub enum LaunchAttributeValue<'a> {
    /// Value of launch attribute [`CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW`].
    AccessPolicyWindow(CUaccessPolicyWindow),

    /// Value of launch attribute [`CU_LAUNCH_ATTRIBUTE_COOPERATIVE`].
    ///
    /// Nonzero indicates a cooperative kernel (see
    /// `cuLaunchCooperativeKernel`).
    Cooperative(c_int),

    /// Value of launch attribute
    /// [`CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY`].
    ///
    /// `CUsynchronizationPolicy` for work queued up in this stream.
    SynchronizationPolicy(CUsynchronizationPolicy),

    /// Value of launch attribute [`CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION`] that
    /// represents the desired cluster dimensions for the kernel.
    ClusterDim {
        /// The X dimension of the cluster, in blocks. Must be a divisor of the
        /// grid X dimension.
        x: c_uint,
        /// The Y dimension of the cluster, in blocks. Must be a divisor of the
        /// grid Y dimension.
        y: c_uint,
        /// The Z dimension of the cluster, in blocks. Must be a divisor of the
        /// grid Z dimension.
        z: c_uint,
    },

    /// Value of launch attribute
    /// [`CU_LAUNCH_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE`].
    ///
    /// Cluster scheduling policy preference for the kernel.
    ClusterSchedulingPolicyPreference(CUclusterSchedulingPolicy),

    /// Value of launch attribute
    /// [`CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_STREAM_SERIALIZATION`].
    ProgrammaticStreamSerializationAllowed(c_int),

    /// Value of launch attribute [`CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_EVENT`].
    ProgrammaticEvent {
        /// Event to fire when all blocks trigger it.
        event: CUevent,
        /// Event record flags, see `cuEventRecordWithFlags`. Does not accept
        /// `CU_EVENT_RECORD_EXTERNAL`.
        flags: c_int,
        /// If this is set to non-0, each block launch will automatically
        /// trigger the event.
        trigger_at_block_start: c_int,
    },

    /// Value of launch attribute [`CU_LAUNCH_ATTRIBUTE_PRIORITY`].
    ///
    /// Execution priority of the kernel.
    Priority(c_int),

    /// Value of launch attribute [`CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP`].
    ///
    /// See `CUlaunchMemSyncDomainMap`.
    MemSyncDomainMap(CUlaunchMemSyncDomainMap),

    /// Value of launch attribute [`CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN`].
    ///
    /// See `CUlaunchMemSyncDomain`.
    MemSyncDomain(CUlaunchMemSyncDomain),

    /// Value of launch attribute
    /// [`CU_LAUNCH_ATTRIBUTE_PREFERRED_CLUSTER_DIMENSION`] that represents
    /// the desired preferred cluster dimensions for the kernel.
    PreferredClusterDim {
        /// The X dimension of the preferred cluster, in blocks. Must
        /// be a divisor of the grid X dimension, and must be a
        /// multiple of the `x` field of `CUlaunchAttributeValue::clusterDim`.
        x: c_uint,
        /// The Y dimension of the preferred cluster, in blocks. Must
        /// be a divisor of the grid Y dimension, and must be a
        /// multiple of the `y` field of `CUlaunchAttributeValue::clusterDim`.
        y: c_uint,
        /// The Z dimension of the preferred cluster, in blocks. Must be
        /// equal to the `z` field of `CUlaunchAttributeValue::clusterDim`.
        z: c_uint,
    },

    /// Value of launch attribute
    /// [`CU_LAUNCH_ATTRIBUTE_LAUNCH_COMPLETION_EVENT`].
    LaunchCompletionEvent {
        /// Event to fire when the last block launches.
        event: CUevent,
        /// Event record flags, see `cuEventRecordWithFlags`. Does not accept
        /// `CU_EVENT_RECORD_EXTERNAL`.
        flags: c_int,
    },

    /// Value of launch attribute
    /// [`CU_LAUNCH_ATTRIBUTE_DEVICE_UPDATABLE_KERNEL_NODE`].
    DeviceUpdatableKernelNode {
        /// Whether or not the resulting kernel node should be device-updatable.
        device_updatable: c_int,
        /// Returns a handle to pass to the various device-side update
        /// functions.
        dev_node: CUgraphDeviceNode,
    },

    /// Value of launch attribute
    /// [`CU_LAUNCH_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT`].
    SharedMemCarveout(c_uint),

    /// Value of launch attribute
    /// [`CU_LAUNCH_ATTRIBUTE_NVLINK_UTIL_CENTRIC_SCHEDULING`].
    NvlinkUtilCentricScheduling(c_uint),

    /// A variant known to this library.
    Unknown(&'a CUlaunchAttributeValue),
}


