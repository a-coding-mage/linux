/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2022 Advanced Micro Devices, Inc.
 * Authors:
 *	Christian König <christian.koenig@amd.com>
 */

// #include <linux/types.h>

use core::ffi::c_void;

pub type size_t = usize;

#[repr(C)]
pub struct dma_fence {
    _private: [u8; 0],
}

/**
 * struct dma_fence_unwrap - cursor into the container structure
 *
 * Should be used with dma_fence_unwrap_for_each() iterator macro.
 */
#[repr(C)]
pub struct dma_fence_unwrap {
    /**
     * @chain: potential dma_fence_chain, but can be other fence as well
     */
    pub chain: *mut dma_fence,
    /**
     * @array: potential dma_fence_array, but can be other fence as well
     */
    pub array: *mut dma_fence,
    /**
     * @index: last returned index if @array is really a dma_fence_array
     */
    pub index: u32,
}

extern "C" {
    pub fn dma_fence_unwrap_first(
        head: *mut dma_fence,
        cursor: *mut dma_fence_unwrap,
    ) -> *mut dma_fence;
    pub fn dma_fence_unwrap_next(cursor: *mut dma_fence_unwrap) -> *mut dma_fence;

    pub fn __dma_fence_unwrap_merge(
        num_fences: size_t,
        fences: *mut *mut dma_fence,
        cursors: *mut dma_fence_unwrap,
    ) -> *mut dma_fence;

    pub fn dma_fence_dedup_array(array: *mut *mut dma_fence, num_fences: size_t) -> size_t;
}

/**
 * dma_fence_unwrap_for_each - iterate over all fences in containers
 * @fence: current fence
 * @cursor: current position inside the containers
 * @head: starting point for the iterator
 *
 * Unwrap dma_fence_chain and dma_fence_array containers and deep dive into all
 * potential fences in them. If @head is just a normal fence only that one is
 * returned.
 */
#[macro_export]
macro_rules! dma_fence_unwrap_for_each {
    ($fence:ident, $cursor:expr, $head:expr) => {
        for $fence in unsafe {
            let mut __dma_fence_unwrap_fence =
                $crate::dma_fence_unwrap_first($head, $cursor);
            core::iter::from_fn(|| {
                let current = __dma_fence_unwrap_fence;
                if current.is_null() {
                    None
                } else {
                    __dma_fence_unwrap_fence =
                        $crate::dma_fence_unwrap_next($cursor);
                    Some(current)
                }
            })
        }
    };
}

/**
 * dma_fence_unwrap_merge - unwrap and merge fences
 *
 * All fences given as parameters are unwrapped and merged back together as flat
 * dma_fence_array. Useful if multiple containers need to be merged together.
 *
 * Implemented as a macro to allocate the necessary arrays on the stack and
 * account the stack frame size to the caller.
 *
 * Returns NULL on memory allocation failure, a dma_fence object representing
 * all the given fences otherwise.
 */
#[macro_export]
macro_rules! dma_fence_unwrap_merge {
    ($($fence:expr),* $(,)?) => {{
        let mut __f: [*mut $crate::dma_fence; <[()]>::len(&[$( { let _ = &$fence; () }),*])] =
            [$($fence),*];
        let mut __c: [$crate::dma_fence_unwrap; <[()]>::len(&[$( { let _ = &$fence; () }),*])] =
            unsafe { core::mem::zeroed() };
        unsafe {
            $crate::__dma_fence_unwrap_merge(__f.len(), __f.as_mut_ptr(), __c.as_mut_ptr())
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
