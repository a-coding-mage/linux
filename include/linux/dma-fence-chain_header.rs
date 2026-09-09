/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * fence-chain: chain fences together in a timeline
 *
 * Copyright (C) 2018 Advanced Micro Devices, Inc.
 * Authors:
 *	Christian König <christian.koenig@amd.com>
 */

/* Dependencies supplied by the surrounding Linux/Rust bindings. */

#[repr(C)]
pub union DmaFenceChainCallbackOrWork {
    /// Callback for signaling completion of the fence chain.
    pub cb: dma_fence_cb,
    /// IRQ work item for signaling the fence chain.
    pub work: irq_work,
}

/// Fence to represent a node of a fence chain.
#[repr(C)]
pub struct dma_fence_chain {
    /// Fence base class.
    pub base: dma_fence,
    /// Previous fence of the chain.
    pub prev: *mut dma_fence,
    /// Original previous seqno before garbage collection.
    pub prev_seqno: u64,
    /// Encapsulated fence.
    pub fence: *mut dma_fence,
    /// Callback/work union for fence handling.
    pub u: DmaFenceChainCallbackOrWork,
}

/// Cast a fence to a dma_fence_chain. Returns null if it is not a chain.
#[inline]
pub unsafe fn to_dma_fence_chain(fence: *mut dma_fence) -> *mut dma_fence_chain {
    if fence.is_null() || !dma_fence_is_chain(fence) {
        return core::ptr::null_mut();
    }

    container_of!(fence, dma_fence_chain, base)
}

/// Return the fence contained in a chain, or the fence itself otherwise.
#[inline]
pub unsafe fn dma_fence_chain_contained(fence: *mut dma_fence) -> *mut dma_fence {
    let chain = to_dma_fence_chain(fence);
    if !chain.is_null() {
        (*chain).fence
    } else {
        fence
    }
}

/// Specialized allocator for separately accounted chain allocations.
#[macro_export]
macro_rules! dma_fence_chain_alloc {
    () => {
        kmalloc_obj!(dma_fence_chain)
    };
}

/// Free an allocated but not initialized or published chain object.
#[inline]
pub unsafe fn dma_fence_chain_free(chain: *mut dma_fence_chain) {
    kfree(chain);
}

/// Iterate over all fences in a chain, retaining the current fence.
#[macro_export]
macro_rules! dma_fence_chain_for_each {
    ($iter:ident, $head:expr) => {
        let mut $iter = dma_fence_get($head);
        while !$iter.is_null() {
            {
                /* Loop body supplied by the invocation's surrounding scope. */
            }
            $iter = dma_fence_chain_walk($iter);
        }
    };
}

extern "C" {
    pub fn dma_fence_is_chain(fence: *mut dma_fence) -> bool;
    pub fn dma_fence_get(fence: *mut dma_fence) -> *mut dma_fence;
    pub fn dma_fence_chain_walk(fence: *mut dma_fence) -> *mut dma_fence;
    pub fn dma_fence_chain_find_seqno(pfence: *mut *mut dma_fence, seqno: u64) -> i32;
    pub fn dma_fence_chain_init(
        chain: *mut dma_fence_chain,
        prev: *mut dma_fence,
        fence: *mut dma_fence,
        seqno: u64,
    );
    pub fn kfree(ptr: *mut dma_fence_chain);
}

/* External types and macros are provided by the corresponding dependencies. */
extern "C" {
    pub type dma_fence;
    pub type dma_fence_cb;
    pub type irq_work;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
