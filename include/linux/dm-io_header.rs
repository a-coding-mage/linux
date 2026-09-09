/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2003 Sistina Software
 * Copyright (C) 2004 - 2008 Red Hat, Inc. All rights reserved.
 *
 * Device-Mapper low-level I/O.
 *
 * This file is released under the GPL.
 */

// The declarations below are available when building in the kernel.

#[repr(C)]
pub struct dm_io_region {
    pub bdev: *mut block_device,
    pub sector: sector_t,
    pub count: sector_t, // If this is zero the region is ignored.
}

#[repr(C)]
pub struct page_list {
    pub next: *mut page_list,
    pub page: *mut page,
}

pub type io_notify_fn = Option<unsafe extern "C" fn(
    error: ::core::ffi::c_ulong,
    unsup: ::core::ffi::c_ulong,
    context: *mut ::core::ffi::c_void,
)>;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dm_io_mem_type {
    DM_IO_PAGE_LIST, // Page list
    DM_IO_BIO,       // Bio vector
    DM_IO_VMA,       // Virtual memory area
    DM_IO_KMEM,      // Kernel memory
}

#[repr(C)]
pub union dm_io_memory_ptr {
    pub pl: *mut page_list,
    pub bio: *mut bio,
    pub vma: *mut ::core::ffi::c_void,
    pub addr: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct dm_io_memory {
    pub type_: dm_io_mem_type,
    pub offset: ::core::ffi::c_uint,
    pub ptr: dm_io_memory_ptr,
}

#[repr(C)]
pub struct dm_io_notify {
    pub r#fn: io_notify_fn, // Callback for asynchronous requests
    pub context: *mut ::core::ffi::c_void, // Passed to callback
}

/*
 * IO request structure
 */
pub struct dm_io_client;

#[repr(C)]
pub struct dm_io_request {
    pub bi_opf: blk_opf_t,          // Request type and flags
    pub mem: dm_io_memory,          // Memory to use for io
    pub notify: dm_io_notify,       // Synchronous if notify.fn is NULL
    pub client: *mut dm_io_client,  // Client memory handler
}

/*
 * For async io calls, users can alternatively use the dm_io() function below
 * and dm_io_client_create() to create private mempools for the client.
 *
 * Create/destroy may block.
 */
extern "C" {
    pub fn dm_io_client_create() -> *mut dm_io_client;
    pub fn dm_io_client_destroy(client: *mut dm_io_client);
}

/*
 * IO interface using private per-client pools.
 * Each bit in the optional 'sync_error_bits' bitset indicates whether an
 * error occurred doing io to the corresponding region.
 */
extern "C" {
    pub fn dm_io(
        io_req: *mut dm_io_request,
        num_regions: ::core::ffi::c_uint,
        region: *mut dm_io_region,
        sync_error_bits: *mut ::core::ffi::c_ulong,
        sync_unsup_bits: *mut ::core::ffi::c_ulong,
        ioprio: ::core::ffi::c_ushort,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
