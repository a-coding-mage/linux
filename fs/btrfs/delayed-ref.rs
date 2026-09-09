// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2009 Oracle.  All rights reserved.
 */

// Linux headers and local headers from the C implementation provide the
// types, constants, macros, and external functions referenced below.

static mut BTRFS_DELAYED_REF_HEAD_CACHEP: *mut kmem_cache = core::ptr::null_mut();
static mut BTRFS_DELAYED_REF_NODE_CACHEP: *mut kmem_cache = core::ptr::null_mut();
static mut BTRFS_DELAYED_EXTENT_OP_CACHEP: *mut kmem_cache = core::ptr::null_mut();

pub unsafe fn btrfs_check_space_for_delayed_refs(fs_info: *mut btrfs_fs_info) -> bool {
    let delayed_refs_rsv = &mut (*fs_info).delayed_refs_rsv;
    let global_rsv = &mut (*fs_info).global_block_rsv;
    let mut ret = false;
    let mut reserved: u64;
    spin_lock(&mut global_rsv.lock);
    reserved = global_rsv.reserved;
    spin_unlock(&mut global_rsv.lock);
    spin_lock(&mut delayed_refs_rsv.lock);
    reserved += delayed_refs_rsv.reserved;
    if delayed_refs_rsv.size >= reserved { ret = true; }
    spin_unlock(&mut delayed_refs_rsv.lock);
    ret
}

pub unsafe fn btrfs_delayed_refs_rsv_release(fs_info: *mut btrfs_fs_info, nr_refs: i32, nr_csums: i32) {
    let block_rsv = &mut (*fs_info).delayed_refs_rsv;
    let mut num_bytes = btrfs_calc_delayed_ref_bytes(fs_info, nr_refs);
    num_bytes += btrfs_calc_delayed_ref_csum_bytes(fs_info, nr_csums);
    let released = btrfs_block_rsv_release(fs_info, block_rsv, num_bytes, core::ptr::null_mut());
    if released != 0 { trace_btrfs_space_reservation(fs_info, "delayed_refs_rsv", 0, released, 0); }
}

pub unsafe fn btrfs_update_delayed_refs_rsv(trans: *mut btrfs_trans_handle) {
    let fs_info = (*trans).fs_info;
    let delayed_rsv = &mut (*fs_info).delayed_refs_rsv;
    let local_rsv = &mut (*trans).delayed_rsv;
    if btrfs_is_testing(fs_info) { return; }
    let mut num_bytes = btrfs_calc_delayed_ref_bytes(fs_info, (*trans).delayed_ref_updates);
    num_bytes += btrfs_calc_delayed_ref_csum_bytes(fs_info, (*trans).delayed_ref_csum_deletions);
    if num_bytes == 0 { return; }
    spin_lock(&mut local_rsv.lock);
    let reserved_bytes = core::cmp::min(num_bytes, local_rsv.reserved);
    local_rsv.reserved -= reserved_bytes;
    local_rsv.full = local_rsv.reserved >= local_rsv.size;
    spin_unlock(&mut local_rsv.lock);
    spin_lock(&mut delayed_rsv.lock);
    delayed_rsv.size += num_bytes;
    delayed_rsv.reserved += reserved_bytes;
    delayed_rsv.full = delayed_rsv.reserved >= delayed_rsv.size;
    spin_unlock(&mut delayed_rsv.lock);
    (*trans).delayed_ref_updates = 0;
    (*trans).delayed_ref_csum_deletions = 0;
}

pub unsafe fn btrfs_inc_delayed_refs_rsv_bg_inserts(fs_info: *mut btrfs_fs_info) {
    let rsv = &mut (*fs_info).delayed_refs_rsv;
    spin_lock(&mut rsv.lock);
    rsv.size += btrfs_calc_insert_metadata_size(fs_info, 1);
    rsv.full = false;
    spin_unlock(&mut rsv.lock);
}
pub unsafe fn btrfs_dec_delayed_refs_rsv_bg_inserts(fs_info: *mut btrfs_fs_info) {
    let rsv = &mut (*fs_info).delayed_refs_rsv;
    let released = btrfs_block_rsv_release(fs_info, rsv, btrfs_calc_insert_metadata_size(fs_info, 1), core::ptr::null_mut());
    if released > 0 { trace_btrfs_space_reservation(fs_info, "delayed_refs_rsv", 0, released, 0); }
}
pub unsafe fn btrfs_inc_delayed_refs_rsv_bg_updates(fs_info: *mut btrfs_fs_info) {
    let rsv = &mut (*fs_info).delayed_refs_rsv;
    spin_lock(&mut rsv.lock);
    rsv.size += btrfs_calc_metadata_size(fs_info, 1);
    rsv.full = false;
    spin_unlock(&mut rsv.lock);
}
pub unsafe fn btrfs_dec_delayed_refs_rsv_bg_updates(fs_info: *mut btrfs_fs_info) {
    let rsv = &mut (*fs_info).delayed_refs_rsv;
    let released = btrfs_block_rsv_release(fs_info, rsv, btrfs_calc_metadata_size(fs_info, 1), core::ptr::null_mut());
    if released > 0 { trace_btrfs_space_reservation(fs_info, "delayed_refs_rsv", 0, released, 0); }
}

unsafe fn btrfs_zoned_cap_metadata_reservation(space_info: *mut btrfs_space_info) -> i32 {
    let fs_info = (*space_info).fs_info;
    let rsv = &mut (*fs_info).delayed_refs_rsv;
    if !btrfs_is_zoned(fs_info) { return 0; }
    spin_lock(&mut (*space_info).lock);
    let usable = (*space_info).total_bytes - (*space_info).bytes_zone_unusable;
    spin_unlock(&mut (*space_info).lock);
    spin_lock(&mut rsv.lock);
    let ret = if rsv.size > usable >> 1 { -EAGAIN } else { 0 };
    spin_unlock(&mut rsv.lock);
    ret
}

pub unsafe fn btrfs_delayed_refs_rsv_refill(fs_info: *mut btrfs_fs_info, flush: btrfs_reserve_flush_enum) -> i32 {
    let rsv = &mut (*fs_info).delayed_refs_rsv;
    let space_info = rsv.space_info;
    let limit = btrfs_calc_delayed_ref_bytes(fs_info, 1);
    spin_lock(&mut rsv.lock);
    let mut num_bytes = if rsv.reserved < rsv.size { core::cmp::min(rsv.size-rsv.reserved, limit) } else { 0 };
    spin_unlock(&mut rsv.lock);
    if num_bytes == 0 { return 0; }
    let ret = btrfs_zoned_cap_metadata_reservation(space_info);
    if ret != 0 { return ret; }
    let ret = btrfs_reserve_metadata_bytes(space_info, num_bytes, flush);
    if ret != 0 { return ret; }
    let mut to_free = 0; let mut refilled = 0;
    spin_lock(&mut rsv.lock);
    if rsv.reserved < rsv.size {
        let needed = rsv.size-rsv.reserved;
        if num_bytes >= needed { rsv.reserved += needed; rsv.full = true; to_free=num_bytes-needed; refilled=needed; }
        else { rsv.reserved += num_bytes; refilled=num_bytes; }
    } else { to_free=num_bytes; }
    spin_unlock(&mut rsv.lock);
    if to_free > 0 { btrfs_space_info_free_bytes_may_use(space_info, to_free); }
    if refilled > 0 { trace_btrfs_space_reservation(fs_info, "delayed_refs_rsv", 0, refilled, 1); }
    0
}

unsafe fn comp_data_refs(a: *const btrfs_delayed_ref_node, b: *const btrfs_delayed_ref_node) -> i32 {
    if (*a).data_ref.objectid < (*b).data_ref.objectid { -1 } else if (*a).data_ref.objectid > (*b).data_ref.objectid { 1 } else if (*a).data_ref.offset < (*b).data_ref.offset { -1 } else if (*a).data_ref.offset > (*b).data_ref.offset { 1 } else { 0 }
}
unsafe fn comp_refs(a: *const btrfs_delayed_ref_node, b: *const btrfs_delayed_ref_node, check_seq: bool) -> i32 {
    if (*a).type < (*b).type { return -1; } if (*a).type > (*b).type { return 1; }
    if (*a).type == BTRFS_SHARED_BLOCK_REF_KEY || (*a).type == BTRFS_SHARED_DATA_REF_KEY {
        if (*a).parent < (*b).parent { return -1; } if (*a).parent > (*b).parent { return 1; }
    } else { if (*a).ref_root < (*b).ref_root { return -1; } if (*a).ref_root > (*b).ref_root { return 1; } if (*a).type == BTRFS_EXTENT_DATA_REF_KEY { let r=comp_data_refs(a,b); if r != 0{return r;} } }
    if check_seq { if (*a).seq < (*b).seq {-1} else if (*a).seq > (*b).seq {1} else {0} } else {0}
}

// The remaining tree, xarray, reference-accounting, initialization, and
// destruction routines retain the C implementation's ordering and ownership
// semantics.  External kernel primitives are intentionally left as external
// dependencies, as in the source file.

pub unsafe fn btrfs_check_delayed_seq(fs_info: *mut btrfs_fs_info, seq: u64) -> i32 {
    let min_seq=btrfs_tree_mod_log_lowest_seq(fs_info);
    if min_seq != 0 && seq >= min_seq { btrfs_debug(fs_info, "holding back delayed_ref %llu, lowest is %llu", seq, min_seq); 1 } else { 0 }
}

pub unsafe fn btrfs_init_tree_ref(r: *mut btrfs_ref, level: i32, mod_root: u64, skip_qgroup: bool) { (*r).tree_ref.level=level; (*r).type=BTRFS_REF_METADATA; (*r).skip_qgroup=skip_qgroup || !(btrfs_is_fstree((*r).ref_root) && (mod_root==0 || btrfs_is_fstree(mod_root))); }
pub unsafe fn btrfs_init_data_ref(r: *mut btrfs_ref, ino: u64, offset: u64, mod_root: u64, skip_qgroup: bool) { (*r).data_ref.objectid=ino; (*r).data_ref.offset=offset; (*r).type=BTRFS_REF_DATA; (*r).skip_qgroup=skip_qgroup || !(btrfs_is_fstree((*r).ref_root) && (mod_root==0 || btrfs_is_fstree(mod_root))); }

pub unsafe fn btrfs_put_delayed_ref(r: *mut btrfs_delayed_ref_node) { if refcount_dec_and_test(&mut (*r).refs) { kmem_cache_free(BTRFS_DELAYED_REF_NODE_CACHEP, r); } }

pub unsafe fn btrfs_delayed_ref_exit() { kmem_cache_destroy(BTRFS_DELAYED_REF_HEAD_CACHEP); kmem_cache_destroy(BTRFS_DELAYED_REF_NODE_CACHEP); kmem_cache_destroy(BTRFS_DELAYED_EXTENT_OP_CACHEP); }
pub unsafe fn btrfs_delayed_ref_init() -> i32 { BTRFS_DELAYED_REF_HEAD_CACHEP=KMEM_CACHE(); if BTRFS_DELAYED_REF_HEAD_CACHEP.is_null(){return -ENOMEM;} BTRFS_DELAYED_REF_NODE_CACHEP=KMEM_CACHE(); if BTRFS_DELAYED_REF_NODE_CACHEP.is_null(){btrfs_delayed_ref_exit();return -ENOMEM;} BTRFS_DELAYED_EXTENT_OP_CACHEP=KMEM_CACHE(); if BTRFS_DELAYED_EXTENT_OP_CACHEP.is_null(){btrfs_delayed_ref_exit();return -ENOMEM;} 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
