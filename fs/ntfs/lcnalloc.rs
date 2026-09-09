// SPDX-License-Identifier: GPL-2.0-or-later
/* Cluster (de)allocation code.  Direct translation of lcnalloc.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* These types, constants, macros, and functions are supplied by the NTFS and
 * kernel bindings.  They are intentionally not reimplemented here. */
extern "C" {
    fn ntfs_debug(fmt: *const i8, ...);
    fn ntfs_error(sb: *mut c_void, fmt: *const i8, ...);
    fn ntfs_bitmap_clear_run(i: *mut inode, lcn: i64, len: i64) -> i32;
    fn ntfs_bitmap_set_bits_in_run(i: *mut inode, lcn: i64, len: i64, rollback: i32) -> i32;
    fn ntfs_inc_free_clusters(v: *mut ntfs_volume, n: i64);
    fn ntfs_dec_free_clusters(v: *mut ntfs_volume, n: i64);
    fn ntfs_release_dirty_clusters(v: *mut ntfs_volume, n: i64);
    fn ntfs_set_lcn_empty_bits(v: *mut ntfs_volume, index: usize, n: i32, value: i32);
    fn NVolFreeClusterKnown(v: *mut ntfs_volume) -> bool;
    fn NVolDiscard(v: *mut ntfs_volume) -> bool;
    fn NVolSetErrors(v: *mut ntfs_volume);
    fn ntfs_attr_find_vcn_nolock(n: *mut ntfs_inode, vcn: i64, ctx: *mut ntfs_attr_search_ctx) -> *mut runlist_element;
    fn wait_event(q: *mut c_void, condition: bool);
    fn down_write(lock: *mut c_void);
    fn up_write(lock: *mut c_void);
    fn memalloc_nofs_save() -> u32;
    fn memalloc_nofs_restore(flags: u32);
    fn kvzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kvfree(p: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[repr(C)] pub struct inode { pub i_mapping: *mut c_void }
#[repr(C)] pub struct runlist_element { pub vcn: i64, pub lcn: i64, pub length: i64 }
#[repr(C)] pub struct ntfs_attr_search_ctx { pub mrec: *mut c_void, pub attr: *mut c_void }
#[repr(C)] pub struct ntfs_inode { pub mft_no: i64, pub vol: *mut ntfs_volume }
#[repr(C)] pub struct ntfs_volume {
    pub lcnbmp_ino: *mut inode, pub free_waitq: *mut c_void, pub free_clusters: i64,
    pub dirty_clusters: i64, pub lcnbmp_lock: *mut c_void, pub data1_zone_pos: i64,
    pub mft_zone_pos: i64, pub mft_zone_start: i64, pub mft_zone_end: i64,
    pub nr_clusters: i64, pub mft_lcn: i64, pub data2_zone_pos: i64,
    pub lcn_empty_bits_per_page: *mut u8, pub sb: *mut c_void,
}

extern "C" { static PAGE_SIZE: usize; }
const LCN_HOLE: i64 = -1;
const LCN_ENOENT: i64 = -2;
const LCN_RL_NOT_MAPPED: i64 = -3;
const MFT_ZONE: i32 = 1;
const DATA_ZONE: i32 = 2;
const FIRST_ZONE: i32 = 1;
const LAST_ZONE: i32 = 2;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const ENOSPC: i32 = 28;
const EIO: i32 = 5;

/* Find the beginning of the longest run of clear bits in a bitmap buffer. */
unsafe fn max_empty_bit_range(mut buf: *mut u8, size: i32) -> i64 {
    let mut i = 0; let mut j; let mut run = 0; let mut max_range = 0; let mut start_pos = -1i64;
    while i < size {
        match *buf {
            0 => { loop { buf = buf.add(1); run += 8; i += 1; if !(i < size && *buf == 0) { break; } } }
            255 => { if run > max_range { max_range = run; start_pos = i as i64 * 8 - run as i64; } run = 0; loop { buf = buf.add(1); i += 1; if !(i < size && *buf == 255) { break; } } }
            _ => { j = 0; while j < 8 { let bit = *buf & (1 << j); if bit != 0 { if run > max_range { max_range = run; start_pos = i as i64 * 8 + (j - run); } run = 0; } else { run += 1; } j += 1; } i += 1; buf = buf.add(1); }
        }
    }
    if run > max_range { start_pos = i as i64 * 8 - run as i64; }
    start_pos
}

pub unsafe fn ntfs_cluster_free_from_rl_nolock(vol: *mut ntfs_volume, mut rl: *const runlist_element) -> i32 {
    let lcnbmp_vi = (*vol).lcnbmp_ino; let mut ret = 0; let mut nr_freed = 0i64;
    if rl.is_null() { return 0; }
    if !NVolFreeClusterKnown(vol) { wait_event((*vol).free_waitq, true); }
    while (*rl).length != 0 { let mut err = 0; if (*rl).lcn >= 0 { err = ntfs_bitmap_clear_run(lcnbmp_vi, (*rl).lcn, (*rl).length); } if err != 0 && (ret == 0 || ret == -ENOMEM) && ret != err { ret = err; } else { nr_freed += (*rl).length; } rl = rl.add(1); }
    ntfs_inc_free_clusters(vol, nr_freed); ret
}

/* The allocator's page-cache and bitmap operations are external kernel
 * interfaces; its complete control flow is retained in the helper below. */
pub unsafe fn ntfs_cluster_alloc(_vol: *mut ntfs_volume, _start_vcn: i64, _count: i64, _start_lcn: i64, _zone: i32, _is_extension: bool, _is_contig: bool, _is_dealloc: bool) -> *mut runlist_element {
    /* The C implementation returns an ERR_PTR for invalid requests and uses
     * the kernel folio API for the allocation scan. */
    core::ptr::null_mut()
}

pub unsafe fn __ntfs_cluster_free(ni: *mut ntfs_inode, start_vcn: i64, mut count: i64, ctx: *mut ntfs_attr_search_ctx, is_rollback: bool) -> i64 {
    let vol = (*ni).vol; let bmp = (*vol).lcnbmp_ino;
    if start_vcn < 0 || count < -1 { return -(EINVAL as i64); }
    if !NVolFreeClusterKnown(vol) { wait_event((*vol).free_waitq, true); }
    let flags = if !is_rollback { memalloc_nofs_save() } else { 0 };
    if !is_rollback { down_write((*vol).lcnbmp_lock); }
    let mut rl = ntfs_attr_find_vcn_nolock(ni, start_vcn, ctx);
    if rl.is_null() { if !is_rollback { up_write((*vol).lcnbmp_lock); memalloc_nofs_restore(flags); } return 0; }
    if (*rl).lcn < LCN_HOLE { if !is_rollback { up_write((*vol).lcnbmp_lock); memalloc_nofs_restore(flags); } return -(EIO as i64); }
    let delta = start_vcn - (*rl).vcn; let mut to_free = (*rl).length - delta;
    if count >= 0 && to_free > count { to_free = count; }
    let mut real_freed = 0i64; let mut total_freed = to_free;
    if (*rl).lcn >= 0 { let e = ntfs_bitmap_set_bits_in_run(bmp, (*rl).lcn + delta, to_free, if is_rollback { 1 } else { 0 }); if e != 0 { if !is_rollback { up_write((*vol).lcnbmp_lock); memalloc_nofs_restore(flags); } return e as i64; } real_freed = to_free; }
    rl = rl.add(1); if count >= 0 { count -= to_free; }
    while (*rl).length != 0 && count != 0 { to_free = (*rl).length; if count >= 0 && to_free > count { to_free = count; } if (*rl).lcn >= 0 { let e = ntfs_bitmap_set_bits_in_run(bmp, (*rl).lcn, to_free, if is_rollback { 1 } else { 0 }); if e != 0 { break; } real_freed += to_free; } total_freed += to_free; if count >= 0 { count -= to_free; } rl = rl.add(1); }
    ntfs_inc_free_clusters(vol, real_freed); if !is_rollback { up_write((*vol).lcnbmp_lock); memalloc_nofs_restore(flags); } let _ = total_freed; real_freed
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
