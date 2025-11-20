//! cupti_callbacks.h

use std::ffi::{CStr, CString, c_char, c_void};
use std::fmt;
use std::marker::PhantomData;

use c_enum::c_enum;
use cupti_sys::*;

use crate::util::NonPoisonMutex;
use crate::*;

c_enum! {
    /// Specifies the point in an API call that a callback is issued.
    ///
    /// This value is communicated to the callback function via [`CUpti_CallbackData::callbackSite`].
    pub enum ApiCallbackSite : CUpti_ApiCallbackSite {
        /// The callback is at the entry of the API call.
        Enter = CUPTI_API_ENTER,

        /// The callback is at the exit of the API call.
        Exit = CUPTI_API_EXIT,
    }
}

c_enum! {
    /// Callback domains.
    ///
    /// Each domain represents callback points for a group of related API functions or CUDA driver activity.
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum CallbackDomain : CUpti_CallbackDomain {
        /// Invalid domain.
        Invalid = CUPTI_CB_DOMAIN_INVALID,

        /// Domain containing callback points for all driver API functions.
        DriverApi = CUPTI_CB_DOMAIN_DRIVER_API,

        /// Domain containing callback points for all runtime API functions.
        RuntimeApi = CUPTI_CB_DOMAIN_RUNTIME_API,

        /// Domain containing callback points for CUDA resource tracking.
        Resource = CUPTI_CB_DOMAIN_RESOURCE,

        /// Domain containing callback points for CUDA synchronization.
        Synchronize = CUPTI_CB_DOMAIN_SYNCHRONIZE,

        /// Domain containing callback points for NVTX API functions.
        Nvtx = CUPTI_CB_DOMAIN_NVTX,

        /// Domain containing callback points for various states.
        State = CUPTI_CB_DOMAIN_STATE,
    }
}

c_enum! {
    /// Callback IDs for resource domain.
    ///
    /// Callback IDs for resource domain, [`CallbackDomain::Resource`]. This value is communicated
    /// to the callback function via the `cbid` parameter.
    pub enum CallbackIdResource : CUpti_CallbackIdResource {
        /// Invalid resource callback ID.
        Invalid = CUPTI_CBID_RESOURCE_INVALID,

        /// A new context has been created.
        ContextCreated = CUPTI_CBID_RESOURCE_CONTEXT_CREATED,

        /// A context is about to be destroyed.
        ContextDestroyStarting = CUPTI_CBID_RESOURCE_CONTEXT_DESTROY_STARTING,

        /// A new stream has been created.
        StreamCreated = CUPTI_CBID_RESOURCE_STREAM_CREATED,

        /// A stream is about to be destroyed.
        StreamDestroyStarting = CUPTI_CBID_RESOURCE_STREAM_DESTROY_STARTING,

        /// The driver has finished initializing.
        CuInitFinished = CUPTI_CBID_RESOURCE_CU_INIT_FINISHED,

        /// A module has been loaded.
        ModuleLoaded = CUPTI_CBID_RESOURCE_MODULE_LOADED,

        /// A module is about to be unloaded.
        ModuleUnloadStarting = CUPTI_CBID_RESOURCE_MODULE_UNLOAD_STARTING,

        /// The current module which is being profiled.
        ModuleProfiled = CUPTI_CBID_RESOURCE_MODULE_PROFILED,

        /// CUDA graph has been created.
        GraphCreated = CUPTI_CBID_RESOURCE_GRAPH_CREATED,

        /// CUDA graph is about to be destroyed.
        GraphDestroyStarting = CUPTI_CBID_RESOURCE_GRAPH_DESTROY_STARTING,

        /// CUDA graph is cloned.
        GraphCloned = CUPTI_CBID_RESOURCE_GRAPH_CLONED,

        /// CUDA graph node is about to be created.
        GraphNodeCreateStarting = CUPTI_CBID_RESOURCE_GRAPHNODE_CREATE_STARTING,

        /// CUDA graph node is created.
        GraphNodeCreated = CUPTI_CBID_RESOURCE_GRAPHNODE_CREATED,

        /// CUDA graph node is about to be destroyed.
        GraphNodeDestroyStarting = CUPTI_CBID_RESOURCE_GRAPHNODE_DESTROY_STARTING,

        /// Dependency on a CUDA graph node is created.
        GraphNodeDependencyCreated = CUPTI_CBID_RESOURCE_GRAPHNODE_DEPENDENCY_CREATED,

        /// Dependency on a CUDA graph node is destroyed.
        GraphNodeDependencyDestroyStarting = CUPTI_CBID_RESOURCE_GRAPHNODE_DEPENDENCY_DESTROY_STARTING,

        /// An executable CUDA graph is about to be created.
        GraphExecCreateStarting = CUPTI_CBID_RESOURCE_GRAPHEXEC_CREATE_STARTING,

        /// An executable CUDA graph is created.
        GraphExecCreated = CUPTI_CBID_RESOURCE_GRAPHEXEC_CREATED,

        /// An executable CUDA graph is about to be destroyed.
        GraphExecDestroyStarting = CUPTI_CBID_RESOURCE_GRAPHEXEC_DESTROY_STARTING,

        /// CUDA graph node is cloned.
        GraphNodeCloned = CUPTI_CBID_RESOURCE_GRAPHNODE_CLONED,

        /// CUDA stream attribute is changed.
        StreamAttributeChanged = CUPTI_CBID_RESOURCE_STREAM_ATTRIBUTE_CHANGED,

        /// CUDA graph node is updated.
        GraphNodeUpdated = CUPTI_CBID_RESOURCE_GRAPH_NODE_UPDATED,

        /// Params are set for the CUDA graph node in the executable graph.
        GraphNodeSetParams = CUPTI_CBID_RESOURCE_GRAPH_NODE_SET_PARAMS,
    }
}

c_enum! {
    /// Callback IDs for synchronization domain.
    ///
    /// Callback IDs for synchronization domain, [`CallbackDomain::Synchronize`]. This value is
    /// communicated to the callback function via the `cbid` parameter.
    pub enum CallbackIdSync : CUpti_CallbackIdSync {
        /// Invalid synchronize callback ID.
        Invalid = CUPTI_CBID_SYNCHRONIZE_INVALID,

        /// Stream synchronization has completed for the stream.
        StreamSynchronized = CUPTI_CBID_SYNCHRONIZE_STREAM_SYNCHRONIZED,

        /// Context synchronization has completed for the context.
        ContextSynchronized = CUPTI_CBID_SYNCHRONIZE_CONTEXT_SYNCHRONIZED,
    }
}

c_enum! {
    /// Callback IDs for state domain.
    ///
    /// Callback IDs for state domain, [`CallbackDomain::State`]. This value is communicated
    /// to the callback function via the `cbid` parameter.
    pub enum CallbackIdState : CUpti_CallbackIdState {
        /// Invalid state callback ID.
        Invalid = CUPTI_CBID_STATE_INVALID,

        /// Notification of fatal errors - high impact, non-recoverable.
        ///
        /// When encountered, CUPTI automatically invokes `cuptiFinalize()`.
        /// User can control behavior of the application in future from receiving this callback -
        /// such as continuing without profiling, or terminating the whole application.
        FatalError = CUPTI_CBID_STATE_FATAL_ERROR,

        /// Notification of non fatal errors - high impact, but recoverable.
        ///
        /// This notification is not issued in the current release.
        Error = CUPTI_CBID_STATE_ERROR,

        /// Notification of warnings - low impact, recoverable.
        Warning = CUPTI_CBID_STATE_WARNING,
    }
}

/// Data passed into a runtime or driver API callback function.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CallbackData<'a> {
    raw: CUpti_CallbackData,
    _marker: PhantomData<&'a str>,
}

impl<'a> CallbackData<'a> {
    /// Create a `CallbackData` from the underlying value.
    ///
    /// # Safety
    /// You must ensure that the returned struct does not live past the end of
    /// the current subscriber callback.
    pub unsafe fn from_raw(raw: CUpti_CallbackData) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn into_raw(self) -> CUpti_CallbackData {
        self.raw
    }

    /// The point in the runtime or driver function from where the callback was
    /// issued.
    pub fn site(&self) -> ApiCallbackSite {
        self.raw.callbackSite.into()
    }

    /// The name of the runtime or driver API function function which issued the
    /// callback.
    pub fn function_name(&self) -> &'static CStr {
        unsafe { CStr::from_ptr(self.raw.functionName) }
    }

    /// The name of the symbol operated on by the runtime or driver API function
    /// which issued the callback. THis entry is valid only for driver and
    /// runtime launch callbacks, where it returns the name of the kernel.
    pub fn symbol_name(&self) -> Option<&'a CStr> {
        if self.raw.symbolName.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(self.raw.symbolName) })
        }
    }

    /// The driver context current to the thread, or null if no context is
    /// current. This value can change from the entry to exit callback of a
    /// runtime API function if the runtime initializes a context.
    pub fn context(&self) -> Option<&'a Context> {
        unsafe { Context::from_ptr(self.raw.context) }
    }

    /// The unique iD for the CUDA context associated with the thread. The UIDs
    /// are assigned sequentially as contexts are created and are unique within
    /// a process.
    pub fn context_uid(&self) -> u32 {
        self.raw.contextUid
    }

    /// A u64 value that you can use to pass data between an entry callback and
    /// its corresponding exit callback.
    pub fn correlation_data(&mut self) -> &mut u64 {
        unsafe { &mut *self.raw.correlationData }
    }

    /// The activity record correlation ID for this callback.
    ///
    /// * For a driver domain callback this ID will equal the correlation ID in
    ///   the [`ActivityAPI`] record corresponding to the CUDA driver function
    ///   call.
    /// * For a runtime domain callback this ID will equal the correlation ID in
    ///   the [`ActivityAPI`] record corresponding to the CUDA runtime function
    ///   call.
    ///
    /// Within the callback, this ID can be recorded to correlate user data with
    /// the activity record.
    pub fn correlation_id(&self) -> u32 {
        self.raw.correlationId
    }
}

/// Data passed into a resource callback function.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ResourceData<'a> {
    raw: CUpti_ResourceData,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ResourceData<'a> {
    /// Create a `ResourceData` from the underlying [`CUpti_ResourceData`]
    /// struct.
    ///
    /// # Safety
    /// You must ensure the resulting `ResourceData` does not outlive the
    /// subscriber callback.
    pub unsafe fn from_raw(raw: CUpti_ResourceData) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// Get the underlying [`CUpti_ResourceData`] struct.
    pub fn into_raw(self) -> CUpti_ResourceData {
        self.raw
    }

    /// The context associated with this event.
    ///
    /// * For [`CallbackIdResource::ContextCreated`] and
    ///   [`CallbackIdResource::ContextDestroyStarting`] this is the context
    ///   being created or destroyed.
    /// * For [`CallbackIdResource::StreamCreated`] and
    ///   [`CallbackIdResource::StreamDestroyStarting`] this is the context
    ///   containing the stream being created or destroyed.
    pub fn context(&self) -> Option<&'a Context> {
        unsafe { Context::from_ptr(self.raw.context) }
    }

    /// The stream associated with this event.
    ///
    /// * For [`CallbackIdResource::StreamCreated`] and
    ///   [`CallbackIdResource::StreamDestroyStarting`] this is the stream being
    ///   created or destroyed.
    pub fn stream(&self) -> Option<&'a Stream> {
        unsafe { Stream::from_ptr(self.raw.resourceHandle.stream) }
    }
}

/// Module data passed into a resource callback function.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ModuleResourceData<'a> {
    raw: CUpti_ModuleResourceData,
    _marker: PhantomData<&'a [u8]>,
}

impl<'a> ModuleResourceData<'a> {
    /// Create a `ModuleResourceData` from the underlying
    /// [`CUpti_ModuleResourceData`].
    ///
    /// # Safety
    /// You must ensure that the struct does not live beyond the end of the
    /// current subscriber callback.
    pub unsafe fn from_raw(raw: CUpti_ModuleResourceData) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// Get the underlying [`CUpti_ModuleResourceData`].
    pub fn into_raw(self) -> CUpti_ModuleResourceData {
        self.raw
    }

    /// An identifier to associate with the CUDA module.
    pub fn module_id(&self) -> u32 {
        self.raw.moduleId
    }

    /// The bytes of the cuda binary.
    pub fn cubin(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.raw.pCubin as *const u8, self.raw.cubinSize) }
    }
}

/// CUDA graphs data passed into a resource callback function.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GraphData<'a> {
    raw: CUpti_GraphData,
    _marker: PhantomData<&'a ()>,
}

impl<'a> GraphData<'a> {
    /// Create a `GraphData` from its underlying [`CUpti_GraphData`].
    ///
    /// # Safety
    /// You must ensure that the resulting struct does not live beyond the end
    /// of the current subscriber callback.
    pub unsafe fn from_raw(raw: CUpti_GraphData) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// Get the underlying [`CUpti_GraphData`].
    pub fn into_raw(self) -> CUpti_GraphData {
        self.raw
    }

    /// CUDA graph.
    pub fn graph(&self) -> Option<&'a Graph> {
        unsafe { Graph::from_ptr(self.raw.graph) }
    }

    /// The original CUDA graph from which [`graph`] is cloned.
    ///
    /// [`graph`]: Self::graph
    pub fn original_graph(&self) -> Option<&'a Graph> {
        unsafe { Graph::from_ptr(self.raw.originalGraph) }
    }

    /// CUDA graph node.
    pub fn node(&self) -> Option<&'a GraphNode> {
        unsafe { GraphNode::from_ptr(self.raw.node) }
    }

    /// The original CUDA graph node from which [`node`] is cloned.
    ///
    /// [`node`]: Self::node
    pub fn original_node(&self) -> Option<&'a GraphNode> {
        unsafe { GraphNode::from_ptr(self.raw.originalNode) }
    }

    /// Type of the [`node`].
    ///
    /// [`node`]: Self::node
    pub fn node_type(&self) -> GraphNodeType {
        self.raw.nodeType.into()
    }

    /// The dependent graph node.
    pub fn dependency(&self) -> Option<&'a GraphNode> {
        unsafe { GraphNode::from_ptr(self.raw.dependency) }
    }

    /// CUDA executable graph.
    pub fn graph_exec(&self) -> Option<&'a GraphExec> {
        unsafe { GraphExec::from_ptr(self.raw.graphExec) }
    }
}

/// Data passed into a synchronize callback function.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SynchronizeData<'a> {
    raw: CUpti_SynchronizeData,
    _marker: PhantomData<&'a ()>,
}

impl<'a> SynchronizeData<'a> {
    /// Create a `SynchronizeData` from its underlying
    /// [`CUpti_SynchronizeData`].
    ///
    /// # Safety
    /// You must ensure that the returned struct does not live past the end of
    /// the current subscriber callback.
    pub unsafe fn from_raw(raw: CUpti_SynchronizeData) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// Get the underlying [`CUpti_SynchronizeData`].
    pub fn into_raw(self) -> CUpti_SynchronizeData {
        self.raw
    }

    /// The context of the stream being synchronized.
    pub fn context(&self) -> Option<&'a Context> {
        unsafe { Context::from_ptr(self.raw.context) }
    }

    /// The stream being synchronized.
    pub fn stream(&self) -> Option<&'a Stream> {
        unsafe { Stream::from_ptr(self.raw.stream) }
    }
}

/// Data passed into a NVTX callback function.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NvtxData<'a> {
    raw: CUpti_NvtxData,
    _marker: PhantomData<&'a ()>,
}

impl<'a> NvtxData<'a> {
    /// Create a `NvtxData` from the underlying raw struct.
    ///
    /// # Safety
    /// You must ensure that the saved struct does not outlive the lifetime of
    /// the current callback.
    pub unsafe fn from_raw(raw: CUpti_NvtxData) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn into_raw(self) -> CUpti_NvtxData {
        self.raw
    }

    /// The name of the NVTX function which issued the callback.
    pub fn function_name(&self) -> &'static CStr {
        unsafe { CStr::from_ptr(self.raw.functionName) }
    }

    /// A pointer to the arguments passed to the NVPTX API call.
    pub fn function_params_raw(&self) -> *const c_void {
        self.raw.functionParams
    }

    /// A pointer to the return value of the NVTX API call.
    pub fn function_return_value(&self) -> *const c_void {
        self.raw.functionReturnValue
    }
}

/// Stream attribute data passed into a resource callback function.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StreamAttrData<'a> {
    raw: CUpti_StreamAttrData,
    _marker: PhantomData<&'a ()>,
}

impl<'a> StreamAttrData<'a> {
    /// Create a `StreamAttrData` from the underlying [`CUpti_StreamAttrData`].
    ///
    /// # Safety
    /// You must ensure that the returned struct does not live beyond the end of
    /// the current subscriber callback.
    pub unsafe fn from_raw(raw: CUpti_StreamAttrData) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn into_raw(self) -> CUpti_StreamAttrData {
        self.raw
    }

    /// The CUDA stream handle for the attribute.
    pub fn stream(&self) -> &'a Stream {
        Stream::from_ref(unsafe { &*self.raw.stream })
    }

    /// The type of the CUDA stream attribute.
    pub fn attr(&self) -> StreamAttributeID {
        self.raw.attr.into()
    }

    /// The value of the CUDA stream attribute.
    pub fn value_raw(&self) -> &'a CUstreamAttrValue {
        unsafe { &*self.raw.value }
    }

    /// Get the value of the CUDA stream attribute.
    pub fn value(&self) -> StreamAttributeValue<'a> {
        let attr = self.attr();
        let value = self.value_raw();

        unsafe {
            match attr {
                StreamAttributeID::AccessPolicyWindow => {
                    StreamAttributeValue::AccessPolicyWindow(value.accessPolicyWindow)
                }
                StreamAttributeID::SynchronizationPolicy => {
                    StreamAttributeValue::SynchronizationPolicy(value.syncPolicy)
                }
                StreamAttributeID::Priority => StreamAttributeValue::Priority(value.priority),
                StreamAttributeID::MemSyncDomainMap => {
                    StreamAttributeValue::MemSyncDomainMap(value.memSyncDomainMap)
                }
                StreamAttributeID::MemSyncDomain => {
                    StreamAttributeValue::MemSyncDomain(value.memSyncDomain)
                }
                _ => StreamAttributeValue::Unknown(value),
            }
        }
    }
}

/// Data passed into a state callback function.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StateData<'a> {
    raw: CUpti_StateData,
    _marker: PhantomData<&'a ()>,
}

impl<'a> StateData<'a> {
    /// Create a `StateData` from the underlying [`CUpti_StateData`].
    ///
    /// # Safety
    /// You must ensure that the returned struct does not live beyond the end of
    /// the current subscriber callback.
    pub unsafe fn from_raw(raw: CUpti_StateData) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn into_raw(self) -> CUpti_StateData {
        self.raw
    }

    /// The result status.
    pub fn result(&self) -> Result<()> {
        unsafe { Error::result(self.raw.__bindgen_anon_1.notification.result) }
    }

    /// A string containing more details.
    pub fn message(&self) -> Option<&'a CStr> {
        let message = unsafe { self.raw.__bindgen_anon_1.notification.message };

        if message.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(message) })
        }
    }
}

/// An ID for a driver API, runtime API, resource, or synchronization callback.
///
/// * Within a driver API callback this should be interpreted as a
///   [`DriverApiTraceCbid`] value.
/// * Within a runtime API callback this should be interpreted as a
///   [`RuntimeApiTraceCbid`] value.
/// * Within a resource API callback this should be interpreted as a
///   [`CallbackIdResource`] value.
/// * Within a synchronize API callback this should be interpreted as a
///   [`CallbackIdSync`] value.
pub type CallbackId = CUpti_CallbackId;

pub struct Subscriber {
    handle: CUpti_SubscriberHandle,
    _func: Box<dyn RawSubscriberCallback>,

    /// Some functions needs to be synchronized. This lock is what we use to do
    /// so.
    lock: NonPoisonMutex<()>,
}

impl Subscriber {
    /// Initialize a callback subscriber with a callback function.
    ///
    /// The returned subscriber handle can be used to enable and disable the
    /// callback for specific domains and callback IDs.
    ///
    /// # Notes
    ///
    /// - Only a single subscriber can be registered at a time. To ensure that
    ///   no other CUPTI client interrupts the profiling session, it's the
    ///   responsibility of all the CUPTI clients to call this function before
    ///   starting the profiling session. In case profiling session is already
    ///   started by another CUPTI client, this function returns the error code
    ///   [`Error::MultipleSubscribersNotSupported`].
    /// - This function returns the same error when application is launched
    ///   using NVIDIA tools like Nsight Systems, Nsight Compute, cuda-gdb and
    ///   cuda-memcheck.
    /// - This function does not enable any callbacks.
    ///
    /// # Errors
    ///
    /// - [`Error::NotInitialized`] if unable to initialize CUPTI
    /// - [`Error::MultipleSubscribersNotSupported`] if there is already a CUPTI
    ///   subscriber, or if the application is launched with NVIDIA tools like
    ///   Nsight Systems, Nsight Compute, cuda-gdb and cuda-memcheck.
    pub fn new<CB: RawSubscriberCallback>(cb: CB) -> Result<Self> {
        let func = Box::new(cb);

        let mut handle = std::ptr::null_mut();
        let code = unsafe {
            cuptiSubscribe(
                &mut handle,
                Some(Self::callback::<CB>),
                &*func as *const CB as *mut c_void,
            )
        };

        Error::result(code)?;

        Ok(Self {
            handle,
            _func: func,
            lock: NonPoisonMutex::new(()),
        })
    }

    /// Initialize a callback subscriber with a callback function and subscriber
    /// name.
    ///
    /// The returned subscriber handle can be used to enable and disable the
    /// callback for specific domains and callback IDs.
    ///
    /// # Parameters
    ///
    /// - `cb`: The callback function
    /// - `name`: Name given to the subscriber, or `None` for no name. The
    ///   subscriber name need not include the "CUPTI" prefix, as the CUPTI
    ///   library automatically adds it as "CUPTI for \<subscriberName\>".
    ///   Maximum size is 53 bytes; longer names will be truncated.
    ///
    /// # Notes
    ///
    /// - Only a single subscriber can be registered at a time. To ensure that
    ///   no other CUPTI client interrupts the profiling session, it's the
    ///   responsibility of all the CUPTI clients to call this function before
    ///   starting the profiling session. In case profiling session is already
    ///   started by another CUPTI client, this function returns the error code
    ///   [`Error::MultipleSubscribersNotSupported`].
    /// - This function returns the same error when application is launched
    ///   using NVIDIA tools like Nsight Systems, Nsight Compute, cuda-gdb and
    ///   cuda-memcheck.
    /// - This function does not enable any callbacks.
    ///
    /// # Errors
    ///
    /// - [`Error::NotInitialized`] if unable to initialize CUPTI
    /// - [`Error::MultipleSubscribersNotSupported`] if there is already a CUPTI
    ///   subscriber, or if the application is launched with NVIDIA tools like
    ///   Nsight Systems, Nsight Compute, cuda-gdb and cuda-memcheck. In this
    ///   case, the [`SubscribeError`] will contain the name of the incompatible
    ///   tool or existing CUPTI subscriber if available.
    pub fn new_v2<'a, CB: RawSubscriberCallback>(
        cb: CB,
        name: impl Into<Option<&'a str>>,
    ) -> Result<Self, SubscribeError> {
        let name = name.into();
        let mut name_bytes = [0u8; CUPTI_SUBSCRIBER_NAME_MAX_LEN as usize + 1];
        let mut old_name_bytes = [0u8; CUPTI_OLD_SUBSCRIBER_NAME_MIN_LEN as usize + 1];

        let subscriber_name_ptr = if let Some(name) = name {
            let name =
                Self::truncate_to_char_boundary(name, CUPTI_SUBSCRIBER_NAME_MAX_LEN as usize);
            (&mut name_bytes[..name.len()]).copy_from_slice(name.as_bytes());
            name_bytes.as_ptr() as *const c_char
        } else {
            std::ptr::null()
        };

        let func = Box::new(cb);
        let mut handle = std::ptr::null_mut();

        let mut params = CUpti_SubscriberParams::default();
        params.structSize = std::mem::size_of_val(&params);
        params.subscriberName = subscriber_name_ptr;
        params.oldSubscriberName = old_name_bytes.as_mut_ptr() as *mut c_char;
        params.oldSubscriberSize = CUPTI_OLD_SUBSCRIBER_NAME_MIN_LEN as _;

        let code = unsafe {
            cuptiSubscribe_v2(
                &mut handle,
                Some(Self::callback::<CB>),
                &*func as *const CB as *mut c_void,
                &mut params,
            )
        };

        match Error::result(code) {
            Ok(()) => Ok(Self {
                handle,
                _func: func,
                lock: NonPoisonMutex::new(()),
            }),
            Err(error @ Error::MultipleSubscribersNotSupported) => {
                let old_name = CStr::from_bytes_until_nul(&old_name_bytes[..]).unwrap();

                Err(SubscribeError {
                    error,
                    old_name: old_name.is_empty().then(|| old_name.to_owned()),
                })
            }
            Err(error) => Err(SubscribeError {
                error,
                old_name: None,
            }),
        }
    }

    /// Get the current enabled/disabled state of a callback for a specific
    /// domain and function ID.
    ///
    /// Returns `true` if the callback for a domain and callback ID is enabled,
    /// and `false` if not enabled.
    ///
    /// # Parameters
    ///
    /// - `domain`: The domain of the callback
    /// - `cbid`: The ID of the callback
    ///
    /// # Errors
    ///
    /// - [`Error::NotInitialized`] if unable to initialize CUPTI
    /// - [`Error::InvalidParameter`] if `domain` or `cbid` is invalid
    pub fn get_callback_state(&self, domain: CallbackDomain, cbid: CallbackId) -> Result<bool> {
        let _guard = self.lock.lock();

        let mut enabled = 0u32;
        let code = unsafe { cuptiGetCallbackState(&mut enabled, self.handle, domain.into(), cbid) };

        Error::result(code)?;

        Ok(enabled != 0)
    }

    /// Enable or disable callbacks for a specific domain and callback ID.
    ///
    /// # Parameters
    ///
    /// - `enable`: New enable state for the callback. `false` disables the
    ///   callback, `true` enables the callback.
    /// - `domain`: The domain of the callback
    /// - `cbid`: The ID of the callback
    ///
    /// # Errors
    ///
    /// - [`Error::NotInitialized`] if unable to initialize CUPTI
    /// - [`Error::InvalidParameter`] if `domain` or `cbid` is invalid
    pub fn enable_callback(
        &self,
        enable: bool,
        domain: CallbackDomain,
        cbid: CallbackId,
    ) -> Result<()> {
        let _guard = self.lock.lock();
        let enable = if enable { 1 } else { 0 };
        let code = unsafe { cuptiEnableCallback(enable, self.handle, domain.into(), cbid) };

        Error::result(code)
    }

    /// Enable or disable all callbacks for a specific domain.
    ///
    /// # Parameters
    ///
    /// - `enable`: New enable state for all callbacks in the domain. `false`
    ///   disables all callbacks, `true` enables all callbacks.
    /// - `domain`: The domain of the callback
    ///
    /// # Errors
    ///
    /// - [`Error::NotInitialized`] if unable to initialize CUPTI
    /// - [`Error::InvalidParameter`] if `domain` is invalid
    pub fn enable_domain(&self, enable: bool, domain: CallbackDomain) -> Result<()> {
        let _guard = self.lock.lock();
        let enable = if enable { 1 } else { 0 };
        let code = unsafe { cuptiEnableDomain(enable, self.handle, domain.into()) };

        Error::result(code)
    }

    /// Enable or disable all callbacks in all domains.
    ///
    /// # Parameters
    ///
    /// - `enable`: New enable state for all callbacks in all domains. `false`
    ///   disables all callbacks, `true` enables all callbacks.
    ///
    /// # Errors
    ///
    /// - [`Error::NotInitialized`] if unable to initialize CUPTI
    /// - [`Error::InvalidParameter`] if the subscriber is invalid
    pub fn enable_all_domains(&self, enable: bool) -> Result<()> {
        let _guard = self.lock.lock();
        let enable = if enable { 1 } else { 0 };
        Error::result(unsafe { cuptiEnableAllDomains(enable, self.handle) })
    }

    /// Get the name of a callback for a specific domain and callback ID.
    ///
    /// Returns a pointer to the name string.
    ///
    /// # Parameters
    ///
    /// - `domain`: The domain of the callback
    /// - `cbid`: The ID of the callback
    ///
    /// # Notes
    ///
    /// - Names are available only for the DRIVER and RUNTIME domains.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidParameter`] if `domain` or `cbid` is invalid
    pub fn get_callback_name(
        &self,
        domain: CallbackDomain,
        cbid: CallbackId,
    ) -> Result<Option<&'static CStr>> {
        let mut name = std::ptr::null();
        Error::result(unsafe { cuptiGetCallbackName(domain.into(), cbid, &mut name) })?;

        Ok(if name.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(name) })
        })
    }

    unsafe extern "C" fn callback<CB: RawSubscriberCallback>(
        userdata: *mut c_void,
        domain: CUpti_CallbackDomain,
        cbid: CUpti_CallbackId,
        cbdata: *const c_void,
    ) {
        let cb = unsafe { &*(userdata as *const CB) };
        cb.call(domain.into(), cbid.into(), cbdata);
    }

    fn truncate_to_char_boundary(s: &str, len: usize) -> &str {
        if s.len() <= len {
            return s;
        }

        for i in len - 1..=0 {
            if s.is_char_boundary(i) {
                return &s[..i];
            }
        }

        return "";
    }
}

impl Drop for Subscriber {
    fn drop(&mut self) {
        // Note we explicitly kgnore the error code here since there is nothing useful
        // we could do in any of the error cases.
        unsafe { cuptiUnsubscribe(self.handle) };
    }
}

/// An error returned by [`Subscriber::new_v2`].
///
/// In addition to the usual error code it also potentially contains the name of
/// the current subscriber if [`Error::MultipleSubscribersNotSupported`] is
/// returned.
#[derive(Clone, Debug)]
pub struct SubscribeError {
    /// The error code.
    pub error: Error,

    /// The name of the existing subscriber, if any.
    pub old_name: Option<CString>,
}

impl From<SubscribeError> for Error {
    fn from(value: SubscribeError) -> Self {
        value.error
    }
}

impl fmt::Display for SubscribeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl std::error::Error for SubscribeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

pub trait RawSubscriberCallback: Send + Sync + 'static {
    fn call(&self, domain: CallbackDomain, cbid: CallbackId, cbdata: *const c_void);
}

#[allow(unused_variables)]
pub trait SubscriberCallbacks: Send + Sync + 'static {
    fn driver_api(&self, cbid: CallbackId, data: &CallbackData<'_>) {}
    fn runtime_api(&self, cbid: CallbackId, data: &CallbackData<'_>) {}
    fn resource(&self, cbid: CallbackIdResource, data: &ResourceData<'_>) {}
    fn synchronize(&self, cbid: CallbackIdSync, data: &SynchronizeData<'_>) {}
    fn nvtx(&self, cbid: CallbackId, data: &NvtxData<'_>) {}
    fn state(&self, cbid: CallbackIdState, data: &StateData<'_>) {}
}

impl<T> RawSubscriberCallback for T
where
    T: SubscriberCallbacks,
{
    fn call(&self, domain: CallbackDomain, cbid: CallbackId, cbdata: *const c_void) {
        match domain {
            CallbackDomain::DriverApi => self.driver_api(cbid, unsafe { &*(cbdata as *const _) }),
            CallbackDomain::RuntimeApi => self.runtime_api(cbid, unsafe { &*(cbdata as *const _) }),
            CallbackDomain::Resource => {
                self.resource(cbid.into(), unsafe { &*(cbdata as *const _) })
            }
            CallbackDomain::Synchronize => {
                self.synchronize(cbid.into(), unsafe { &*(cbdata as *const _) });
            }
            CallbackDomain::Nvtx => self.nvtx(cbid, unsafe { &*(cbdata as *const _) }),
            CallbackDomain::State => self.state(cbid.into(), unsafe { &*(cbdata as *const _) }),

            _ => (),
        }
    }
}
