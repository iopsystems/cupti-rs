use std::cell::UnsafeCell;

use cupti_sys::*;

use crate::*;

/// A reference to a CUDA graph ([`CUgraph`]).
#[repr(transparent)]
pub struct Graph(UnsafeCell<CUgraph_st>);

impl Graph {
    /// Create a [`Graph`] from a [`CUgraph`] reference.
    pub fn from_ref(context: &CUgraph_st) -> &Self {
        unsafe { &*(context as *const _ as *const _) }
    }

    /// Create a [`Graph`] from a mutable [`CUgraph`] reference.
    pub fn from_mut(context: &mut CUgraph_st) -> &mut Self {
        unsafe { &mut *(context as *mut _ as *mut _) }
    }

    /// Create a [`Graph`] directly from a pointer.
    ///
    /// # Safety
    /// `ptr` must either be null or a pointer to a valid graph.
    pub unsafe fn from_ptr<'a>(ptr: *const CUgraph_st) -> Option<&'a Self> {
        if !ptr.is_null() {
            Some(Self::from_ref(unsafe { &*ptr }))
        } else {
            None
        }
    }

    pub fn as_raw(&self) -> CUgraph {
        &self.0 as *const _ as *mut _
    }

    /// Get the unique ID of this graph.
    ///
    /// # Errors
    /// * [`Error::NotInitialized`]
    pub fn id(&self) -> Result<u32> {
        let mut id = 0;
        let code = unsafe { cuptiGetGraphId(self.as_raw(), &mut id) };

        Error::result(code).map(|_| id)
    }
}

/// A reference to a CUDA graph node ([`CUgraphNode`]).
#[repr(transparent)]
pub struct GraphNode(UnsafeCell<CUgraphNode_st>);

impl GraphNode {
    /// Create a [`GraphNode`] from a [`CUgraphNode`] reference.
    pub fn from_ref(context: &CUgraphNode_st) -> &Self {
        unsafe { &*(context as *const _ as *const _) }
    }

    /// Create a [`GraphNode`] from a mutable [`CUgraphNode`] reference.
    pub fn from_mut(context: &mut CUgraphNode_st) -> &mut Self {
        unsafe { &mut *(context as *mut _ as *mut _) }
    }

    /// Create a [`GraphNode`] directly from a pointer.
    ///
    /// # Safety
    /// `ptr` must either be null or a pointer to a valid graph node.
    pub unsafe fn from_ptr<'a>(ptr: *const CUgraphNode_st) -> Option<&'a Self> {
        if !ptr.is_null() {
            Some(Self::from_ref(unsafe { &*ptr }))
        } else {
            None
        }
    }

    pub fn as_raw(&self) -> CUgraphNode {
        &self.0 as *const _ as *mut _
    }

    /// Get the unique ID of this graph node.
    ///
    /// # Errors
    /// * [`Error::NotInitialized`]
    pub fn id(&self) -> Result<u64> {
        let mut id = 0;
        let code = unsafe { cuptiGetGraphNodeId(self.as_raw(), &mut id) };

        Error::result(code).map(|_| id)
    }
}

/// A reference to a CUDA graph node ([`CUgraphNode`]).
#[repr(transparent)]
pub struct GraphExec(UnsafeCell<CUgraphExec_st>);

impl GraphExec {
    /// Create a [`GraphNode`] from a [`CUgraphNode`] reference.
    pub fn from_ref(context: &CUgraphExec_st) -> &Self {
        unsafe { &*(context as *const _ as *const _) }
    }

    /// Create a [`GraphNode`] from a mutable [`CUgraphNode`] reference.
    pub fn from_mut(context: &mut CUgraphExec) -> &mut Self {
        unsafe { &mut *(context as *mut _ as *mut _) }
    }

    /// Create a [`GraphExec`] directly from a pointer.
    ///
    /// # Safety
    /// `ptr` must either be null or a pointer to a valid execution graph.
    pub unsafe fn from_ptr<'a>(ptr: *const CUgraphExec_st) -> Option<&'a Self> {
        if !ptr.is_null() {
            Some(Self::from_ref(unsafe { &*ptr }))
        } else {
            None
        }
    }

    pub fn as_raw(&self) -> CUgraphExec {
        &self.0 as *const _ as *mut _
    }

    /// Get the unique ID of this graph node.
    ///
    /// # Errors
    /// * [`Error::NotInitialized`]
    pub fn id(&self) -> Result<u32> {
        let mut id = 0;
        let code = unsafe { cuptiGetGraphExecId(self.as_raw(), &mut id) };

        Error::result(code).map(|_| id)
    }
}
