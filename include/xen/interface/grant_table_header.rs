/* SPDX-License-Identifier: MIT */
/* Rust translation of xen/interface/grant_table.h. */
/* External types and guest-handle conventions are supplied by xen/interface/xen.h. */

pub type grant_ref_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grant_entry_v1 {
    pub flags: u16,
    pub domid: domid_t,
    pub frame: u32,
}

pub const GNTTAB_NR_RESERVED_ENTRIES: u32 = 8;
pub const GNTTAB_RESERVED_CONSOLE: u32 = 0;
pub const GNTTAB_RESERVED_XENSTORE: u32 = 1;

pub const GTF_invalid: u32 = 0u32 << 0;
pub const GTF_permit_access: u32 = 1u32 << 0;
pub const GTF_accept_transfer: u32 = 2u32 << 0;
pub const GTF_transitive: u32 = 3u32 << 0;
pub const GTF_type_mask: u32 = 3u32 << 0;

pub const _GTF_readonly: u32 = 2;
pub const GTF_readonly: u32 = 1u32 << _GTF_readonly;
pub const _GTF_reading: u32 = 3;
pub const GTF_reading: u32 = 1u32 << _GTF_reading;
pub const _GTF_writing: u32 = 4;
pub const GTF_writing: u32 = 1u32 << _GTF_writing;
pub const _GTF_PWT: u32 = 5;
pub const GTF_PWT: u32 = 1u32 << _GTF_PWT;
pub const _GTF_PCD: u32 = 6;
pub const GTF_PCD: u32 = 1u32 << _GTF_PCD;
pub const _GTF_PAT: u32 = 7;
pub const GTF_PAT: u32 = 1u32 << _GTF_PAT;
pub const _GTF_sub_page: u32 = 8;
pub const GTF_sub_page: u32 = 1u32 << _GTF_sub_page;

pub const _GTF_transfer_committed: u32 = 2;
pub const GTF_transfer_committed: u32 = 1u32 << _GTF_transfer_committed;
pub const _GTF_transfer_completed: u32 = 3;
pub const GTF_transfer_completed: u32 = 1u32 << _GTF_transfer_completed;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grant_entry_header {
    pub flags: u16,
    pub domid: domid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grant_entry_v2_full_page {
    pub hdr: grant_entry_header,
    pub pad0: u32,
    pub frame: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grant_entry_v2_sub_page {
    pub hdr: grant_entry_header,
    pub page_off: u16,
    pub length: u16,
    pub frame: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grant_entry_v2_transitive {
    pub hdr: grant_entry_header,
    pub trans_domid: domid_t,
    pub pad0: u16,
    pub gref: grant_ref_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union grant_entry_v2 {
    pub hdr: grant_entry_header,
    pub full_page: grant_entry_v2_full_page,
    pub sub_page: grant_entry_v2_sub_page,
    pub transitive: grant_entry_v2_transitive,
    pub __spacer: [u32; 4],
}

pub type grant_status_t = u16;

pub const GNTTABOP_map_grant_ref: u32 = 0;
pub const GNTTABOP_unmap_grant_ref: u32 = 1;
pub const GNTTABOP_setup_table: u32 = 2;
pub const GNTTABOP_dump_table: u32 = 3;
pub const GNTTABOP_transfer: u32 = 4;
pub const GNTTABOP_copy: u32 = 5;
pub const GNTTABOP_query_size: u32 = 6;
pub const GNTTABOP_unmap_and_replace: u32 = 7;
pub const GNTTABOP_set_version: u32 = 8;
pub const GNTTABOP_get_status_frames: u32 = 9;
pub const GNTTABOP_get_version: u32 = 10;
pub const GNTTABOP_swap_grant_ref: u32 = 11;
pub const GNTTABOP_cache_flush: u32 = 12;

pub type grant_handle_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_map_grant_ref {
    pub host_addr: u64,
    pub flags: u32,
    pub ref_: grant_ref_t,
    pub dom: domid_t,
    pub status: i16,
    pub handle: grant_handle_t,
    pub dev_bus_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_unmap_grant_ref {
    pub host_addr: u64,
    pub dev_bus_addr: u64,
    pub handle: grant_handle_t,
    pub status: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_setup_table {
    pub dom: domid_t,
    pub nr_frames: u32,
    pub status: i16,
    pub frame_list: *mut xen_pfn_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_dump_table {
    pub dom: domid_t,
    pub status: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_transfer {
    pub mfn: xen_pfn_t,
    pub domid: domid_t,
    pub ref_: grant_ref_t,
    pub status: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union gnttab_copy_ptr_u {
    pub ref_: grant_ref_t,
    pub gmfn: xen_pfn_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_copy_ptr {
    pub u: gnttab_copy_ptr_u,
    pub domid: domid_t,
    pub offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_copy {
    pub source: gnttab_copy_ptr,
    pub dest: gnttab_copy_ptr,
    pub len: u16,
    pub flags: u16,
    pub status: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_query_size {
    pub dom: domid_t,
    pub nr_frames: u32,
    pub max_nr_frames: u32,
    pub status: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_unmap_and_replace {
    pub host_addr: u64,
    pub new_addr: u64,
    pub handle: grant_handle_t,
    pub status: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_set_version {
    pub version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_get_status_frames {
    pub nr_frames: u32,
    pub dom: domid_t,
    pub status: i16,
    pub frame_list: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_get_version {
    pub dom: domid_t,
    pub pad: u16,
    pub version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_swap_grant_ref {
    pub ref_a: grant_ref_t,
    pub ref_b: grant_ref_t,
    pub status: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union gnttab_cache_flush_a {
    pub dev_bus_addr: u64,
    pub ref_: grant_ref_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_cache_flush {
    pub a: gnttab_cache_flush_a,
    pub offset: u16,
    pub length: u16,
    pub op: u32,
}

pub const GNTTAB_CACHE_CLEAN: u32 = 1u32 << 0;
pub const GNTTAB_CACHE_INVAL: u32 = 1u32 << 1;
pub const GNTTAB_CACHE_SOURCE_GREF: u32 = 1u32 << 31;

pub const _GNTMAP_device_map: u32 = 0;
pub const GNTMAP_device_map: u32 = 1u32 << _GNTMAP_device_map;
pub const _GNTMAP_host_map: u32 = 1;
pub const GNTMAP_host_map: u32 = 1u32 << _GNTMAP_host_map;
pub const _GNTMAP_readonly: u32 = 2;
pub const GNTMAP_readonly: u32 = 1u32 << _GNTMAP_readonly;
pub const _GNTMAP_application_map: u32 = 3;
pub const GNTMAP_application_map: u32 = 1u32 << _GNTMAP_application_map;
pub const _GNTMAP_contains_pte: u32 = 4;
pub const GNTMAP_contains_pte: u32 = 1u32 << _GNTMAP_contains_pte;
pub const _GNTMAP_guest_avail0: u32 = 16;
pub const GNTMAP_guest_avail_mask: u32 = !0u32 << _GNTMAP_guest_avail0;

pub const GNTST_okay: i32 = 0;
pub const GNTST_general_error: i32 = -1;
pub const GNTST_bad_domain: i32 = -2;
pub const GNTST_bad_gntref: i32 = -3;
pub const GNTST_bad_handle: i32 = -4;
pub const GNTST_bad_virt_addr: i32 = -5;
pub const GNTST_bad_dev_addr: i32 = -6;
pub const GNTST_no_device_space: i32 = -7;
pub const GNTST_permission_denied: i32 = -8;
pub const GNTST_bad_page: i32 = -9;
pub const GNTST_bad_copy_arg: i32 = -10;
pub const GNTST_address_too_big: i32 = -11;
pub const GNTST_eagain: i32 = -12;
pub const GNTST_no_space: i32 = -13;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
