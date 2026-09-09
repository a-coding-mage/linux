/* SPDX-License-Identifier: MIT */
/******************************************************************************
 * memory.h
 *
 * Memory reservation and information.
 *
 * Copyright (c) 2005, Keir Fraser <keir@xensource.com>
 ******************************************************************************/

// C dependency: <linux/spinlock.h>

/* Increase or decrease the specified domain's memory reservation. */
pub const XENMEM_increase_reservation: u32 = 0;
pub const XENMEM_decrease_reservation: u32 = 1;
pub const XENMEM_populate_physmap: u32 = 6;

#[repr(C)]
pub struct xen_memory_reservation {
    pub extent_start: *mut xen_pfn_t,
    pub nr_extents: xen_ulong_t,
    pub extent_order: ::core::ffi::c_uint,
    pub address_bits: ::core::ffi::c_uint,
    pub domid: domid_t,
}

pub const XENMEM_exchange: u32 = 11;

#[repr(C)]
pub struct xen_memory_exchange {
    pub r#in: xen_memory_reservation,
    pub out: xen_memory_reservation,
    pub nr_exchanged: xen_ulong_t,
}

pub const XENMEM_maximum_ram_page: u32 = 2;
pub const XENMEM_current_reservation: u32 = 3;
pub const XENMEM_maximum_reservation: u32 = 4;

pub const XENMEM_machphys_mfn_list: u32 = 5;

#[repr(C)]
pub struct xen_machphys_mfn_list {
    pub max_extents: ::core::ffi::c_uint,
    pub extent_start: *mut xen_pfn_t,
    pub nr_extents: ::core::ffi::c_uint,
}

pub const XENMEM_machphys_mapping: u32 = 12;

#[repr(C)]
pub struct xen_machphys_mapping {
    pub v_start: xen_ulong_t,
    pub v_end: xen_ulong_t,
    pub max_mfn: xen_ulong_t,
}

pub const XENMAPSPACE_shared_info: u32 = 0;
pub const XENMAPSPACE_grant_table: u32 = 1;
pub const XENMAPSPACE_gmfn: u32 = 2;
pub const XENMAPSPACE_gmfn_range: u32 = 3;
pub const XENMAPSPACE_gmfn_foreign: u32 = 4;
pub const XENMAPSPACE_dev_mmio: u32 = 5;

pub const XENMEM_add_to_physmap: u32 = 7;

#[repr(C)]
pub struct xen_add_to_physmap {
    pub domid: domid_t,
    pub size: u16,
    pub space: ::core::ffi::c_uint,
    pub idx: xen_ulong_t,
    pub gpfn: xen_pfn_t,
}

pub const XENMEM_add_to_physmap_range: u32 = 23;

#[repr(C)]
pub struct xen_add_to_physmap_range {
    pub domid: domid_t,
    pub space: u16,
    pub size: u16,
    pub foreign_domid: domid_t,
    pub idxs: *mut xen_ulong_t,
    pub gpfns: *mut xen_pfn_t,
    pub errs: *mut ::core::ffi::c_int,
}

pub const XENMEM_memory_map: u32 = 9;

#[repr(C)]
pub struct xen_memory_map {
    pub nr_entries: ::core::ffi::c_uint,
    pub buffer: *mut ::core::ffi::c_void,
}

pub const XENMEM_machine_memory_map: u32 = 10;
pub const XENMEM_remove_from_physmap: u32 = 15;

#[repr(C)]
pub struct xen_remove_from_physmap {
    pub domid: domid_t,
    pub gpfn: xen_pfn_t,
}

pub const XENMEM_acquire_resource: u32 = 28;

#[repr(C)]
pub struct xen_mem_acquire_resource {
    pub domid: domid_t,
    pub r#type: u16,
    pub id: u32,
    pub nr_frames: u32,
    pub flags: u32,
    pub frame: u64,
    pub frame_list: *mut xen_pfn_t,
}

pub const XENMEM_resource_ioreq_server: u32 = 0;
pub const XENMEM_resource_grant_table: u32 = 1;
pub const XENMEM_resource_grant_table_id_shared: u32 = 0;
pub const XENMEM_resource_grant_table_id_status: u32 = 1;
pub const _XENMEM_rsrc_acq_caller_owned: u32 = 0;
pub const XENMEM_rsrc_acq_caller_owned: u32 = 1u32 << _XENMEM_rsrc_acq_caller_owned;
pub const XENMEM_resource_ioreq_server_frame_bufioreq: u64 = 0;

#[inline]
pub const fn XENMEM_resource_ioreq_server_frame_ioreq(n: u64) -> u64 {
    1u64.wrapping_add(n)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
