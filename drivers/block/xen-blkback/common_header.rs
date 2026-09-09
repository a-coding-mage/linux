/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License version 2
 * as published by the Free Software Foundation; either version 2 of the
 * License, or (at your option) any later version.
 */

// C dependencies supplied by the surrounding kernel/Xen translation.

pub const MAX_INDIRECT_SEGMENTS: usize = 256;
pub const XEN_PAGES_PER_SEGMENT: usize = PAGE_SIZE / XEN_PAGE_SIZE;
pub const XEN_PAGES_PER_INDIRECT_FRAME: usize =
    XEN_PAGE_SIZE / core::mem::size_of::<blkif_request_segment>();
pub const SEGS_PER_INDIRECT_FRAME: usize =
    XEN_PAGES_PER_INDIRECT_FRAME / XEN_PAGES_PER_SEGMENT;
pub const MAX_INDIRECT_PAGES: usize =
    (MAX_INDIRECT_SEGMENTS + SEGS_PER_INDIRECT_FRAME - 1) / SEGS_PER_INDIRECT_FRAME;

#[inline]
pub const fn indirect_pages(segs: usize) -> usize {
    (segs + XEN_PAGES_PER_INDIRECT_FRAME - 1) / XEN_PAGES_PER_INDIRECT_FRAME
}

#[repr(C)]
pub struct blkif_common_request {
    pub dummy: core::ffi::c_char,
}

#[repr(C, packed)]
pub struct blkif_x86_32_request_rw {
    pub nr_segments: u8,
    pub handle: blkif_vdev_t,
    pub id: u64,
    pub sector_number: blkif_sector_t,
    pub seg: [blkif_request_segment; BLKIF_MAX_SEGMENTS_PER_REQUEST],
}

#[repr(C, packed)]
pub struct blkif_x86_32_request_discard {
    pub flag: u8,
    pub _pad1: blkif_vdev_t,
    pub id: u64,
    pub sector_number: blkif_sector_t,
    pub nr_sectors: u64,
}

#[repr(C, packed)]
pub struct blkif_x86_32_request_other {
    pub _pad1: u8,
    pub _pad2: blkif_vdev_t,
    pub id: u64,
}

#[repr(C, packed)]
pub struct blkif_x86_32_request_indirect {
    pub indirect_op: u8,
    pub nr_segments: u16,
    pub id: u64,
    pub sector_number: blkif_sector_t,
    pub handle: blkif_vdev_t,
    pub _pad1: u16,
    pub indirect_grefs: [grant_ref_t; BLKIF_MAX_INDIRECT_PAGES_PER_REQUEST],
    pub _pad2: u64,
}

#[repr(C, packed)]
pub union blkif_x86_32_request_union {
    pub rw: core::mem::ManuallyDrop<blkif_x86_32_request_rw>,
    pub discard: core::mem::ManuallyDrop<blkif_x86_32_request_discard>,
    pub other: core::mem::ManuallyDrop<blkif_x86_32_request_other>,
    pub indirect: core::mem::ManuallyDrop<blkif_x86_32_request_indirect>,
}

#[repr(C, packed)]
pub struct blkif_x86_32_request {
    pub operation: u8,
    pub u: blkif_x86_32_request_union,
}

#[repr(C, packed)]
pub struct blkif_x86_64_request_rw {
    pub nr_segments: u8,
    pub handle: blkif_vdev_t,
    pub _pad1: u32,
    pub id: u64,
    pub sector_number: blkif_sector_t,
    pub seg: [blkif_request_segment; BLKIF_MAX_SEGMENTS_PER_REQUEST],
}

#[repr(C, packed)]
pub struct blkif_x86_64_request_discard {
    pub flag: u8,
    pub _pad1: blkif_vdev_t,
    pub _pad2: u32,
    pub id: u64,
    pub sector_number: blkif_sector_t,
    pub nr_sectors: u64,
}

#[repr(C, packed)]
pub struct blkif_x86_64_request_other {
    pub _pad1: u8,
    pub _pad2: blkif_vdev_t,
    pub _pad3: u32,
    pub id: u64,
}

#[repr(C, packed)]
pub struct blkif_x86_64_request_indirect {
    pub indirect_op: u8,
    pub nr_segments: u16,
    pub _pad1: u32,
    pub id: u64,
    pub sector_number: blkif_sector_t,
    pub handle: blkif_vdev_t,
    pub _pad2: u16,
    pub indirect_grefs: [grant_ref_t; BLKIF_MAX_INDIRECT_PAGES_PER_REQUEST],
    pub _pad3: u32,
}

#[repr(C, packed)]
pub union blkif_x86_64_request_union {
    pub rw: core::mem::ManuallyDrop<blkif_x86_64_request_rw>,
    pub discard: core::mem::ManuallyDrop<blkif_x86_64_request_discard>,
    pub other: core::mem::ManuallyDrop<blkif_x86_64_request_other>,
    pub indirect: core::mem::ManuallyDrop<blkif_x86_64_request_indirect>,
}

#[repr(C, packed)]
pub struct blkif_x86_64_request {
    pub operation: u8,
    pub u: blkif_x86_64_request_union,
}

// DEFINE_RING_TYPES(...) supplies these protocol ring types from Xen's ring.h.

#[repr(C)]
pub union blkif_back_rings {
    pub native: blkif_back_ring,
    pub common: blkif_common_back_ring,
    pub x86_32: blkif_x86_32_back_ring,
    pub x86_64: blkif_x86_64_back_ring,
}

#[repr(C)]
pub enum blkif_protocol {
    BLKIF_PROTOCOL_NATIVE = 1,
    BLKIF_PROTOCOL_X86_32 = 2,
    BLKIF_PROTOCOL_X86_64 = 3,
}

// CONFIG_X86 selects BLKIF_PROTOCOL_X86_32; other builds select NATIVE.
#[cfg(target_arch = "x86")]
pub const BLKIF_PROTOCOL_DEFAULT: blkif_protocol = blkif_protocol::BLKIF_PROTOCOL_X86_32;
#[cfg(not(target_arch = "x86"))]
pub const BLKIF_PROTOCOL_DEFAULT: blkif_protocol = blkif_protocol::BLKIF_PROTOCOL_NATIVE;

#[repr(C)]
pub struct xen_vbd {
    pub handle: blkif_vdev_t,
    pub readonly: u8,
    pub type_: u8,
    pub pdevice: u32,
    pub bdev_file: *mut file,
    pub size: sector_t,
    // C bitfields: flush_support:1, discard_secure:1,
    // feature_gnt_persistent_parm:1, feature_gnt_persistent:1, overflow_max_grants:1.
    pub flags: u32,
}

pub struct backend_info;

pub const XEN_BLKIF_REQS_PER_PAGE: usize = 32;

#[repr(C)]
pub struct persistent_gnt {
    pub page: *mut page,
    pub gnt: grant_ref_t,
    pub handle: grant_handle_t,
    pub last_used: usize,
    pub active: bool,
    pub node: rb_node,
    pub remove_node: list_head,
}

#[repr(C)]
pub struct xen_blkif_ring {
    pub irq: u32,
    pub blk_rings: blkif_back_rings,
    pub blk_ring: *mut core::ffi::c_void,
    pub blk_ring_lock: spinlock_t,
    pub wq: wait_queue_head_t,
    pub inflight: atomic_t,
    pub active: bool,
    pub xenblkd: *mut task_struct,
    pub waiting_reqs: u32,
    pub pending_free: list_head,
    pub pending_free_lock: spinlock_t,
    pub pending_free_wq: wait_queue_head_t,
    pub persistent_gnts: rb_root,
    pub persistent_gnt_c: u32,
    pub persistent_gnt_in_use: atomic_t,
    pub next_lru: usize,
    pub st_print: usize,
    pub st_rd_req: u64,
    pub st_wr_req: u64,
    pub st_oo_req: u64,
    pub st_f_req: u64,
    pub st_ds_req: u64,
    pub st_rd_sect: u64,
    pub st_wr_sect: u64,
    pub persistent_purge_list: list_head,
    pub persistent_purge_work: work_struct,
    pub free_pages: gnttab_page_cache,
    pub free_work: work_struct,
    pub shutdown_wq: wait_queue_head_t,
    pub blkif: *mut xen_blkif,
}

#[repr(C)]
pub struct xen_blkif {
    pub domid: domid_t,
    pub handle: u32,
    pub blk_protocol: blkif_protocol,
    pub vbd: xen_vbd,
    pub be: *mut backend_info,
    pub refcnt: atomic_t,
    pub drain_complete: completion,
    pub drain: atomic_t,
    pub free_work: work_struct,
    pub nr_ring_pages: u32,
    pub multi_ref: bool,
    pub rings: *mut xen_blkif_ring,
    pub nr_rings: u32,
    pub buffer_squeeze_end: usize,
}

#[repr(C)]
pub struct seg_buf { pub offset: usize, pub nsec: u32 }

#[repr(C)]
pub struct grant_page {
    pub page: *mut page,
    pub persistent_gnt: *mut persistent_gnt,
    pub handle: grant_handle_t,
    pub gref: grant_ref_t,
}

#[repr(C)]
pub struct pending_req {
    pub ring: *mut xen_blkif_ring,
    pub id: u64,
    pub nr_segs: i32,
    pub pendcnt: atomic_t,
    pub operation: u16,
    pub status: i32,
    pub free_list: list_head,
    pub segments: [*mut grant_page; MAX_INDIRECT_SEGMENTS],
    pub indirect_pages: [*mut grant_page; MAX_INDIRECT_PAGES],
    pub seg: [seg_buf; MAX_INDIRECT_SEGMENTS],
    pub biolist: [*mut bio; MAX_INDIRECT_SEGMENTS],
    pub unmap: [gnttab_unmap_grant_ref; MAX_INDIRECT_SEGMENTS],
    pub unmap_pages: [*mut page; MAX_INDIRECT_SEGMENTS],
    pub gnttab_unmap_data: gntab_unmap_queue_data,
}

#[inline]
pub unsafe fn vbd_sz(v: *mut xen_vbd) -> sector_t {
    bdev_nr_sectors(file_bdev((*v).bdev_file))
}

#[inline]
pub unsafe fn xen_blkif_get(b: *mut xen_blkif) {
    atomic_inc(&mut (*b).refcnt);
}

#[inline]
pub unsafe fn xen_blkif_put(b: *mut xen_blkif) {
    if atomic_dec_and_test(&mut (*b).refcnt) {
        schedule_work(&mut (*b).free_work);
    }
}

#[repr(C)]
pub struct phys_req {
    pub dev: u16,
    pub nr_sects: blkif_sector_t,
    pub bdev: *mut block_device,
    pub sector_number: blkif_sector_t,
}

extern "C" {
    pub fn xen_blkif_interface_init() -> i32;
    pub fn xen_blkif_interface_fini();
    pub fn xen_blkif_xenbus_init() -> i32;
    pub fn xen_blkif_xenbus_fini();
    pub fn xen_blkif_be_int(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn xen_blkif_schedule(arg: *mut core::ffi::c_void) -> i32;
    pub fn xen_blkbk_free_caches(ring: *mut xen_blkif_ring);
    pub fn xen_blkbk_flush_diskcache(xbt: xenbus_transaction, be: *mut backend_info, state: i32) -> i32;
    pub fn xen_blkbk_barrier(xbt: xenbus_transaction, be: *mut backend_info, state: i32) -> i32;
    pub fn xen_blkbk_xenbus(be: *mut backend_info) -> *mut xenbus_device;
    pub fn xen_blkbk_unmap_purged_grants(work: *mut work_struct);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
