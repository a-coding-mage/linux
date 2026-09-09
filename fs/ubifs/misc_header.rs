/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This file is part of UBIFS.
 *
 * Copyright (C) 2006-2008 Nokia Corporation
 *
 * Authors: Artem Bityutskiy (Битюцкий Артём)
 *          Adrian Hunter
 */

/*
 * This file contains miscellaneous helper functions.
 *
 * C header guard __UBIFS_MISC_H__ omitted.
 */

/// ubifs_zn_dirty - check if znode is dirty.
pub unsafe fn ubifs_zn_dirty(znode: *const ubifs_znode) -> libc::c_int {
    (!!test_bit(DIRTY_ZNODE, &(*znode).flags)) as libc::c_int
}

/// ubifs_zn_obsolete - check if znode is obsolete.
pub unsafe fn ubifs_zn_obsolete(znode: *const ubifs_znode) -> libc::c_int {
    (!!test_bit(OBSOLETE_ZNODE, &(*znode).flags)) as libc::c_int
}

/// ubifs_zn_cow - check if znode has to be copied on write.
pub unsafe fn ubifs_zn_cow(znode: *const ubifs_znode) -> libc::c_int {
    (!!test_bit(COW_ZNODE, &(*znode).flags)) as libc::c_int
}

/// ubifs_wake_up_bgt - wake up background thread.
pub unsafe fn ubifs_wake_up_bgt(c: *mut ubifs_info) {
    if !(*c).bgt.is_null() && (*c).need_bgt == 0 {
        (*c).need_bgt = 1;
        wake_up_process((*c).bgt);
    }
}

/// ubifs_tnc_find_child - find next child in znode.
pub unsafe fn ubifs_tnc_find_child(
    znode: *mut ubifs_znode,
    mut start: libc::c_int,
) -> *mut ubifs_znode {
    while start < (*znode).child_cnt {
        if !(*znode).zbranch[start as usize].znode.is_null() {
            return (*znode).zbranch[start as usize].znode;
        }
        start += 1;
    }
    core::ptr::null_mut()
}

/// ubifs_inode - get UBIFS inode information by VFS struct inode object.
pub unsafe fn ubifs_inode(inode: *const inode) -> *mut ubifs_inode {
    container_of(inode, ubifs_inode, vfs_inode)
}

/// ubifs_compr_present - check if compressor was compiled in.
pub unsafe fn ubifs_compr_present(c: *mut ubifs_info, compr_type: libc::c_int) -> libc::c_int {
    ubifs_assert(c, compr_type >= 0 && compr_type < UBIFS_COMPR_TYPES_CNT);
    (!(*ubifs_compressors[compr_type as usize]).capi_name.is_null()) as libc::c_int
}

/// ubifs_compr_name - get compressor name string by its type.
pub unsafe fn ubifs_compr_name(c: *mut ubifs_info, compr_type: libc::c_int) -> *const libc::c_char {
    ubifs_assert(c, compr_type >= 0 && compr_type < UBIFS_COMPR_TYPES_CNT);
    (*ubifs_compressors[compr_type as usize]).name
}

/// ubifs_wbuf_sync - synchronize write-buffer.
pub unsafe fn ubifs_wbuf_sync(wbuf: *mut ubifs_wbuf) -> libc::c_int {
    mutex_lock_nested(&mut (*wbuf).io_mutex, (*wbuf).jhead);
    let err = ubifs_wbuf_sync_nolock(wbuf);
    mutex_unlock(&mut (*wbuf).io_mutex);
    err
}

/// ubifs_encode_dev - encode device node IDs.
pub unsafe fn ubifs_encode_dev(dev: *mut ubifs_dev_desc, rdev: dev_t) -> libc::c_int {
    (*dev).new = cpu_to_le32(new_encode_dev(rdev));
    core::mem::size_of_val(&(*dev).new) as libc::c_int
}

/// ubifs_add_dirt - add dirty space to LEB properties.
pub unsafe fn ubifs_add_dirt(c: *mut ubifs_info, lnum: libc::c_int, dirty: libc::c_int) -> libc::c_int {
    ubifs_update_one_lp(c, lnum, LPROPS_NC, dirty, 0, 0)
}

/// ubifs_return_leb - return LEB to lprops.
pub unsafe fn ubifs_return_leb(c: *mut ubifs_info, lnum: libc::c_int) -> libc::c_int {
    ubifs_change_one_lp(c, lnum, LPROPS_NC, LPROPS_NC, 0, LPROPS_TAKEN, 0)
}

/// ubifs_idx_node_sz - return index node size.
pub unsafe fn ubifs_idx_node_sz(c: *const ubifs_info, child_cnt: libc::c_int) -> libc::c_int {
    UBIFS_IDX_NODE_SZ + (UBIFS_BRANCH_SZ + (*c).key_len + (*c).hash_len) * child_cnt
}

/// ubifs_idx_branch - return pointer to an index branch.
pub unsafe fn ubifs_idx_branch(
    c: *const ubifs_info,
    idx: *const ubifs_idx_node,
    bnum: libc::c_int,
) -> *mut ubifs_branch {
    ((*idx).branches as *mut u8)
        .add(((UBIFS_BRANCH_SZ + (*c).key_len + (*c).hash_len) * bnum) as usize)
        as *mut ubifs_branch
}

/// ubifs_idx_key - return pointer to an index key.
pub unsafe fn ubifs_idx_key(c: *const ubifs_info, idx: *const ubifs_idx_node) -> *mut libc::c_void {
    (*ubifs_idx_branch(c, idx, 0)).key.cast::<libc::c_void>()
}

/// ubifs_tnc_lookup - look up a file-system node.
pub unsafe fn ubifs_tnc_lookup(
    c: *mut ubifs_info,
    key: *const ubifs_key,
    node: *mut libc::c_void,
) -> libc::c_int {
    ubifs_tnc_locate(c, key, node, core::ptr::null_mut(), core::ptr::null_mut())
}

/// ubifs_get_lprops - get reference to LEB properties.
pub unsafe fn ubifs_get_lprops(c: *mut ubifs_info) {
    mutex_lock(&mut (*c).lp_mutex);
}

/// ubifs_release_lprops - release lprops lock.
pub unsafe fn ubifs_release_lprops(c: *mut ubifs_info) {
    ubifs_assert(c, mutex_is_locked(&(*c).lp_mutex));
    ubifs_assert(c, (*c).lst.empty_lebs >= 0 && (*c).lst.empty_lebs <= (*c).main_lebs);
    mutex_unlock(&mut (*c).lp_mutex);
}

/// ubifs_next_log_lnum - switch to the next log LEB.
pub unsafe fn ubifs_next_log_lnum(c: *const ubifs_info, mut lnum: libc::c_int) -> libc::c_int {
    lnum += 1;
    if lnum > (*c).log_last {
        lnum = UBIFS_LOG_LNUM;
    }
    lnum
}

pub unsafe fn ubifs_xattr_max_cnt(c: *mut ubifs_info) -> libc::c_int {
    let max_xattrs = ((*c).leb_size / 2) / UBIFS_INO_NODE_SZ;
    ubifs_assert(c, max_xattrs < (*c).max_orphans);
    max_xattrs
}

extern "C" {
    pub fn ubifs_assert_action_name(c: *mut ubifs_info) -> *const libc::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
