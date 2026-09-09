/* SPDX-License-Identifier: GPL-2.0 */
/*
 * DMABUF Heaps Allocation Infrastructure
 *
 * Copyright (C) 2011 Google, Inc.
 * Copyright (C) 2019 Linaro Ltd.
 */

// Dependency supplied by the Linux types headers.

#[repr(C)]
pub struct dma_heap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_buf {
    _private: [u8; 0],
}

/**
 * struct dma_heap_ops - ops to operate on a given heap
 * @allocate: allocate dmabuf and return struct dma_buf ptr
 *
 * allocate returns dmabuf on success, ERR_PTR(-errno) on error.
 */
#[repr(C)]
pub struct dma_heap_ops {
    pub allocate: Option<
        unsafe extern "C" fn(
            heap: *mut dma_heap,
            len: ::core::ffi::c_ulong,
            fd_flags: u32,
            heap_flags: u64,
        ) -> *mut dma_buf,
    >,
}

/**
 * struct dma_heap_export_info - information needed to export a new dmabuf heap
 * @name: used for debugging/device-node name
 * @ops: ops struct for this heap
 * @priv: heap exporter private data
 *
 * Information needed to export a new dmabuf heap.
 */
#[repr(C)]
pub struct dma_heap_export_info {
    pub name: *const ::core::ffi::c_char,
    pub ops: *const dma_heap_ops,
    pub priv_: *mut ::core::ffi::c_void,
}

unsafe extern "C" {
    pub fn dma_heap_get_drvdata(heap: *mut dma_heap) -> *mut ::core::ffi::c_void;

    pub fn dma_heap_get_name(heap: *mut dma_heap) -> *const ::core::ffi::c_char;

    pub fn dma_heap_add(exp_info: *const dma_heap_export_info) -> *mut dma_heap;

    pub static mut mem_accounting: bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
