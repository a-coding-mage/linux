/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * fence-array: aggregates fence to be waited together
 *
 * Copyright (C) 2016 Collabora Ltd
 * Copyright (C) 2016 Advanced Micro Devices, Inc.
 * Authors:
 *	Gustavo Padovan <gustavo@padovan.org>
 *	Christian König <christian.koenig@amd.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external: dma_fence, dma_fence_cb, irq_work, atomic_t, and helper macros.

/**
 * struct dma_fence_array_cb - callback helper for fence array
 * @cb: fence callback structure for signaling
 * @array: reference to the parent fence array object
 */
#[repr(C)]
pub struct dma_fence_array_cb {
    pub cb: dma_fence_cb,
    pub array: *mut dma_fence_array,
}

/**
 * struct dma_fence_array - fence to represent an array of fences
 * @base: fence base class
 * @lock: spinlock for fence handling
 * @num_fences: number of fences in the array
 * @num_pending: fences in the array still pending
 * @fences: array of the fences
 * @work: internal irq_work function
 * @callbacks: array of callback helpers
 */
#[repr(C)]
pub struct dma_fence_array {
    pub base: dma_fence,
    pub num_fences: ::core::ffi::c_uint,
    pub num_pending: atomic_t,
    pub fences: *mut *mut dma_fence,
    pub work: irq_work,
    pub callbacks: [dma_fence_array_cb; 0],
}

/**
 * to_dma_fence_array - cast a fence to a dma_fence_array
 * @fence: fence to cast to a dma_fence_array
 *
 * Returns NULL if the fence is not a dma_fence_array,
 * or the dma_fence_array otherwise.
 */
#[inline]
pub unsafe fn to_dma_fence_array(fence: *mut dma_fence) -> *mut dma_fence_array {
    if fence.is_null() || !dma_fence_is_array(fence) {
        return core::ptr::null_mut();
    }

    container_of!(fence, dma_fence_array, base)
}

/**
 * dma_fence_array_for_each - iterate over all fences in array
 * @fence: current fence
 * @index: index into the array
 * @head: potential dma_fence_array object
 *
 * Test if @array is a dma_fence_array object and if yes iterate over all fences
 * in the array. If not just iterate over the fence in @array itself.
 *
 * For a deep dive iterator see dma_fence_unwrap_for_each().
 */
#[macro_export]
macro_rules! dma_fence_array_for_each {
    ($fence:ident, $index:ident, $head:expr) => {
        for ($index, $fence) in (0usize..).zip(
            core::iter::successors(
                Some(dma_fence_array_first($head)),
                |current| {
                    if current.is_null() {
                        None
                    } else {
                        $index += 1;
                        Some(dma_fence_array_next($head, $index as ::core::ffi::c_uint))
                    }
                },
            ),
        ) {
            if $fence.is_null() {
                break;
            }
        }
    };
}

unsafe extern "C" {
    pub fn dma_fence_array_alloc(num_fences: ::core::ffi::c_int) -> *mut dma_fence_array;
    pub fn dma_fence_array_init(
        array: *mut dma_fence_array,
        num_fences: ::core::ffi::c_int,
        fences: *mut *mut dma_fence,
        context: u64,
        seqno: ::core::ffi::c_uint,
    );
    pub fn dma_fence_array_create(
        num_fences: ::core::ffi::c_int,
        fences: *mut *mut dma_fence,
        context: u64,
        seqno: ::core::ffi::c_uint,
    ) -> *mut dma_fence_array;
    pub fn dma_fence_match_context(fence: *mut dma_fence, context: u64) -> bool;
    pub fn dma_fence_array_first(head: *mut dma_fence) -> *mut dma_fence;
    pub fn dma_fence_array_next(
        head: *mut dma_fence,
        index: ::core::ffi::c_uint,
    ) -> *mut dma_fence;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
