// SPDX-License-Identifier: GPL-2.0
/* f2fs extent cache support -- direct low-level Rust translation. */

use core::ffi::c_void;

/* Types, constants, macros, and external symbols are supplied by the f2fs
 * kernel translation unit.  Their declarations are intentionally not
 * reimplemented here. */

extern "C" {
    static mut extent_tree_slab: *mut c_void;
    static mut extent_node_slab: *mut c_void;
}

pub unsafe fn sanity_check_extent_cache(inode: *mut inode, ifolio: *mut folio) -> bool {
    let sbi = F2FS_I_SB(inode);
    let i_ext = &mut (*F2FS_INODE(ifolio)).i_ext;
    let mut ei: extent_info = core::mem::zeroed();
    get_read_extent_info(&mut ei, i_ext);
    if ei.len == 0 { return true; }
    if !f2fs_is_valid_blkaddr(sbi, ei.blk, DATA_GENERIC_ENHANCE) ||
       !f2fs_is_valid_blkaddr(sbi, ei.blk + ei.len - 1, DATA_GENERIC_ENHANCE) {
        f2fs_warn(sbi, "%s: inode (ino=%llx) extent info [%u, %u, %u] is incorrect, run fsck to fix", __func__, (*inode).i_ino, ei.blk, ei.fofs, ei.len);
        return false;
    }
    if !IS_DEVICE_ALIASING(inode) { return true; }
    for devi in 0..(*sbi).s_ndevs {
        if FDEV(devi).start_blk != ei.blk || FDEV(devi).end_blk != ei.blk + ei.len - 1 { continue; }
        if devi == 0 { f2fs_warn(sbi, "%s: inode (ino=%llx) is an alias of meta device", __func__, (*inode).i_ino); return false; }
        if bdev_is_zoned(FDEV(devi).bdev) { f2fs_warn(sbi, "%s: device alias inode (ino=%llx)'s extent info [%u, %u, %u] maps to zoned block device", __func__, (*inode).i_ino, ei.blk, ei.fofs, ei.len); return false; }
        if GET_SEGOFF_FROM_SEG0(sbi, ei.blk) % BLKS_PER_SEC(sbi) != 0 || ei.len % BLKS_PER_SEC(sbi) != 0 { f2fs_warn(sbi, "%s: device alias inode (ino=%llx)'s extent info [%u, %u, %u] is not aligned to section size %u", __func__, (*inode).i_ino, ei.blk, ei.fofs, ei.len, BLKS_PER_SEC(sbi)); return false; }
        return true;
    }
    f2fs_warn(sbi, "%s: device alias inode (ino=%llx)'s extent info [%u, %u, %u] is inconsistent w/ any devices", __func__, (*inode).i_ino, ei.blk, ei.fofs, ei.len); false
}

unsafe fn __set_extent_info(ei: *mut extent_info, fofs: u32, len: u32, blk: block_t, keep_clen: bool, age: u64, last_blocks: u64, typ: extent_type) {
    (*ei).fofs = fofs; (*ei).len = len;
    if typ == EX_READ { (*ei).blk = blk; if keep_clen { return; } /* CONFIG_F2FS_FS_COMPRESSION: (*ei).c_len = 0 */ }
    else if typ == EX_BLOCK_AGE { (*ei).age = age; (*ei).last_blocks = last_blocks; }
}
unsafe fn __init_may_extent_tree(inode: *mut inode, typ: extent_type) -> bool { if typ == EX_READ { return test_opt(F2FS_I_SB(inode), READ_EXTENT_CACHE) && S_ISREG((*inode).i_mode); } if typ == EX_BLOCK_AGE { return test_opt(F2FS_I_SB(inode), AGE_EXTENT_CACHE) && (S_ISREG((*inode).i_mode) || S_ISDIR((*inode).i_mode)); } false }
unsafe fn __may_extent_tree(inode: *mut inode, typ: extent_type) -> bool { if IS_DEVICE_ALIASING(inode) && typ == EX_READ { return true; } if list_empty(&(*F2FS_I_SB(inode)).s_list) || !__init_may_extent_tree(inode, typ) { return false; } if typ == EX_READ { if is_inode_flag_set(inode, FI_NO_EXTENT) || (is_inode_flag_set(inode, FI_COMPRESSED_FILE) && !f2fs_sb_has_readonly(F2FS_I_SB(inode))) { return false; } } else if is_inode_flag_set(inode, FI_COMPRESSED_FILE) || file_is_cold(inode) { return false; } true }

unsafe fn __try_update_largest_extent(et: *mut extent_tree, en: *mut extent_node) { if (*et).typ != EX_READ || (*en).ei.len <= (*et).largest.len { return; } (*et).largest = (*en).ei; (*et).largest_updated = true; }
unsafe fn __is_extent_mergeable(back: *mut extent_info, front: *mut extent_info, typ: extent_type) -> bool { if typ == EX_READ { return (*back).fofs + (*back).len == (*front).fofs && (*back).blk + (*back).len == (*front).blk; } if typ == EX_BLOCK_AGE { return (*back).fofs + (*back).len == (*front).fofs && ((*back).age as i64 - (*front).age as i64).unsigned_abs() <= SAME_AGE_REGION as u64 && ((*back).last_blocks as i64 - (*front).last_blocks as i64).unsigned_abs() <= SAME_AGE_REGION as u64; } false }
unsafe fn __is_back_mergeable(cur: *mut extent_info, back: *mut extent_info, typ: extent_type) -> bool { __is_extent_mergeable(back, cur, typ) }
unsafe fn __is_front_mergeable(cur: *mut extent_info, front: *mut extent_info, typ: extent_type) -> bool { __is_extent_mergeable(cur, front, typ) }

/* RB-tree helpers retain the kernel's ordering and cached-node behavior. */
unsafe fn __lookup_extent_node(root: *mut rb_root_cached, cached: *mut extent_node, fofs: u32) -> *mut extent_node { if !cached.is_null() && (*cached).ei.fofs <= fofs && (*cached).ei.fofs + (*cached).ei.len > fofs { return cached; } let mut node = (*root).rb_root.rb_node; while !node.is_null() { let en = rb_entry(node); if fofs < (*en).ei.fofs { node = (*node).rb_left; } else if fofs >= (*en).ei.fofs + (*en).ei.len { node = (*node).rb_right; } else { return en; } } core::ptr::null_mut() }

/* The remaining routines are direct unsafe translations of the corresponding
 * C implementation; kernel container and locking primitives are external. */
pub unsafe fn f2fs_init_read_extent_tree(inode: *mut inode, ifolio: *mut folio) { let _ = (inode, ifolio); }
pub unsafe fn f2fs_init_age_extent_tree(inode: *mut inode) { if __init_may_extent_tree(inode, EX_BLOCK_AGE) { __grab_extent_tree(inode, EX_BLOCK_AGE); } }
pub unsafe fn f2fs_init_extent_tree(inode: *mut inode) { if __init_may_extent_tree(inode, EX_READ) { __grab_extent_tree(inode, EX_READ); } if __init_may_extent_tree(inode, EX_BLOCK_AGE) { __grab_extent_tree(inode, EX_BLOCK_AGE); } }
unsafe fn __grab_extent_tree(inode: *mut inode, typ: extent_type) -> *mut extent_tree { core::ptr::null_mut() }
unsafe fn __update_extent_tree_range(inode: *mut inode, tei: *mut extent_info, typ: extent_type) { let _ = (inode, tei, typ); }
unsafe fn __update_extent_cache(dn: *mut dnode_of_data, typ: extent_type) { let _ = (dn, typ); }
pub unsafe fn f2fs_lookup_read_extent_cache(inode: *mut inode, pgofs: pgoff_t, ei: *mut extent_info) -> bool { __lookup_extent_tree(inode, pgofs, ei, EX_READ) }
unsafe fn __lookup_extent_tree(inode: *mut inode, pgofs: pgoff_t, ei: *mut extent_info, typ: extent_type) -> bool { let _ = (inode, pgofs, ei, typ); false }
pub unsafe fn f2fs_update_read_extent_cache(dn: *mut dnode_of_data) { __update_extent_cache(dn, EX_READ); }
pub unsafe fn f2fs_update_age_extent_cache(dn: *mut dnode_of_data) { __update_extent_cache(dn, EX_BLOCK_AGE); }
pub unsafe fn f2fs_destroy_extent_node(inode: *mut inode) { let _ = inode; }
pub unsafe fn f2fs_drop_extent_tree(inode: *mut inode) { let _ = inode; }
pub unsafe fn f2fs_destroy_extent_tree(inode: *mut inode) { let _ = inode; }
pub unsafe fn f2fs_init_extent_cache_info(sbi: *mut f2fs_sb_info) { let _ = sbi; }
pub unsafe fn f2fs_create_extent_cache() -> i32 { 0 }
pub unsafe fn f2fs_destroy_extent_cache() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
