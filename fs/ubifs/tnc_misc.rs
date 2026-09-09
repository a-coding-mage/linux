// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file is part of UBIFS.
 *
 * Copyright (C) 2006-2008 Nokia Corporation.
 *
 * Authors: Adrian Hunter
 *          Artem Bityutskiy (Битюцкий Артём)
 */

/*
 * This file contains miscelanious TNC-related functions shared betweend
 * different files. This file does not form any logically separate TNC sub-system.
 */

use core::ptr;

pub unsafe fn ubifs_tnc_levelorder_next(
    c: *const ubifs_info,
    zr: *mut ubifs_znode,
    mut znode: *mut ubifs_znode,
) -> *mut ubifs_znode {
    let mut level: i32;
    let mut iip: i32;
    let mut level_search = 0;
    let mut zn: *mut ubifs_znode;

    ubifs_assert(c, !zr.is_null());
    if znode.is_null() { return zr; }
    if znode == zr {
        if (*znode).level == 0 { return ptr::null_mut(); }
        return ubifs_tnc_find_child(zr, 0);
    }
    level = (*znode).level;
    iip = (*znode).iip;
    loop {
        ubifs_assert(c, (*znode).level <= (*zr).level);
        while (*znode).parent != zr && iip >= (*(*znode).parent).child_cnt {
            znode = (*znode).parent;
            iip = (*znode).iip;
        }
        if (*znode).parent == zr && iip >= (*(*znode).parent).child_cnt {
            level -= 1;
            if level_search != 0 || level < 0 { return ptr::null_mut(); }
            level_search = 1;
            iip = -1;
            znode = ubifs_tnc_find_child(zr, 0);
            ubifs_assert(c, !znode.is_null());
        }
        zn = ubifs_tnc_find_child((*znode).parent, iip + 1);
        if zn.is_null() {
            iip = (*(*znode).parent).child_cnt;
            continue;
        }
        while (*zn).level != level {
            znode = zn;
            zn = ubifs_tnc_find_child(zn, 0);
            if zn.is_null() { iip = (*znode).iip; break; }
        }
        if !zn.is_null() {
            ubifs_assert(c, (*zn).level >= 0);
            return zn;
        }
    }
}

pub unsafe fn ubifs_search_zbranch(
    c: *const ubifs_info, znode: *const ubifs_znode,
    key: *const ubifs_key, n: *mut i32,
) -> i32 {
    let mut beg = 0;
    let mut end = (*znode).child_cnt;
    let zbr = &(*znode).zbranch[0] as *const ubifs_zbranch;
    ubifs_assert(c, end > beg);
    while end > beg {
        let mid = (beg + end) >> 1;
        let cmp = keys_cmp(c, key, &(*zbr.add(mid as usize)).key);
        if cmp > 0 { beg = mid + 1; }
        else if cmp < 0 { end = mid; }
        else { *n = mid; return 1; }
    }
    *n = end - 1;
    ubifs_assert(c, *n >= -1 && *n < (*znode).child_cnt);
    if *n == -1 { ubifs_assert(c, keys_cmp(c, key, &(*zbr).key) < 0); }
    else { ubifs_assert(c, keys_cmp(c, key, &(*zbr.add(*n as usize)).key) > 0); }
    if *n + 1 < (*znode).child_cnt { ubifs_assert(c, keys_cmp(c, key, &(*zbr.add((*n + 1) as usize)).key) < 0); }
    0
}

pub unsafe fn ubifs_tnc_postorder_first(mut znode: *mut ubifs_znode) -> *mut ubifs_znode {
    if znode.is_null() { return ptr::null_mut(); }
    while (*znode).level > 0 {
        let child = ubifs_tnc_find_child(znode, 0);
        if child.is_null() { return znode; }
        znode = child;
    }
    znode
}

pub unsafe fn ubifs_tnc_postorder_next(c: *const ubifs_info, znode: *mut ubifs_znode) -> *mut ubifs_znode {
    ubifs_assert(c, !znode.is_null());
    if (*znode).parent.is_null() { return ptr::null_mut(); }
    let zn = ubifs_tnc_find_child((*znode).parent, (*znode).iip + 1);
    if zn.is_null() { return (*znode).parent; }
    ubifs_tnc_postorder_first(zn)
}

pub unsafe fn ubifs_destroy_tnc_subtree(c: *const ubifs_info, znode: *mut ubifs_znode) -> i64 {
    let mut zn = ubifs_tnc_postorder_first(znode);
    let mut clean_freed: i64 = 0;
    ubifs_assert(c, !zn.is_null());
    loop {
        for n in 0..(*zn).child_cnt {
            let child = (*zn).zbranch[n as usize].znode;
            if child.is_null() { continue; }
            if (*zn).level > 0 && !ubifs_zn_dirty(child) { clean_freed += 1; }
            cond_resched();
            kfree(child);
        }
        if zn == znode {
            if !ubifs_zn_dirty(zn) { clean_freed += 1; }
            kfree(zn);
            return clean_freed;
        }
        zn = ubifs_tnc_postorder_next(c, zn);
    }
}

pub unsafe fn ubifs_destroy_tnc_tree(c: *mut ubifs_info) {
    if (*c).zroot.znode.is_null() { return; }
    let n = atomic_long_read(&(*c).clean_zn_cnt);
    let freed = ubifs_destroy_tnc_subtree(c, (*c).zroot.znode);
    ubifs_assert(c, freed == n);
    atomic_long_sub(n, &ubifs_clean_zn_cnt);
    (*c).zroot.znode = ptr::null_mut();
}

unsafe fn read_znode(c: *mut ubifs_info, zzbr: *mut ubifs_zbranch, znode: *mut ubifs_znode) -> i32 {
    let lnum = (*zzbr).lnum;
    let offs = (*zzbr).offs;
    let len = (*zzbr).len;
    let idx = kmalloc((*c).max_idx_node_sz, GFP_NOFS);
    if idx.is_null() { return -ENOMEM; }
    let err = ubifs_read_node(c, idx, UBIFS_IDX_NODE, len, lnum, offs);
    if err < 0 { kfree(idx); return err; }
    let err = ubifs_node_check_hash(c, idx, (*zzbr).hash);
    if err != 0 { ubifs_bad_hash(c, idx, (*zzbr).hash, lnum, offs); kfree(idx); return err; }
    (*znode).child_cnt = le16_to_cpu((*idx).child_cnt);
    (*znode).level = le16_to_cpu((*idx).level);
    dbg_tnc("LEB %d:%d, level %d, %d branch", lnum, offs, (*znode).level, (*znode).child_cnt);
    if (*znode).child_cnt > (*c).fanout || (*znode).level > UBIFS_MAX_LEVELS {
        ubifs_err(c, "current fanout %d, branch count %d", (*c).fanout, (*znode).child_cnt);
        ubifs_err(c, "max levels %d, znode level %d", UBIFS_MAX_LEVELS, (*znode).level);
        ubifs_err(c, "bad indexing node at LEB %d:%d", lnum, offs);
        ubifs_dump_node(c, idx, (*c).max_idx_node_sz); kfree(idx); return -EINVAL;
    }
    for i in 0..(*znode).child_cnt {
        let br = ubifs_idx_branch(c, idx, i);
        let zbr = &mut (*znode).zbranch[i as usize];
        key_read(c, &(*br).key, &mut zbr.key);
        zbr.lnum = le32_to_cpu((*br).lnum); zbr.offs = le32_to_cpu((*br).offs); zbr.len = le32_to_cpu((*br).len);
        ubifs_copy_hash(c, ubifs_branch_hash(c, br), zbr.hash); zbr.znode = ptr::null_mut();
        if zbr.lnum < (*c).main_first || zbr.lnum >= (*c).leb_cnt || zbr.offs < 0 || zbr.offs + zbr.len > (*c).leb_size || zbr.offs & 7 != 0 { kfree(idx); return -EINVAL; }
        let typ = key_type(c, &zbr.key);
        if typ != UBIFS_INO_KEY && typ != UBIFS_DATA_KEY && typ != UBIFS_DENT_KEY && typ != UBIFS_XENT_KEY { kfree(idx); return -EINVAL; }
        if (*znode).level == 0 { let r = &(*c).ranges[typ as usize]; if (r.max_len == 0 && zbr.len != r.len) || (r.max_len != 0 && (zbr.len < r.min_len || zbr.len > r.max_len)) { kfree(idx); return -EINVAL; } }
    }
    for i in 0..(*znode).child_cnt - 1 { let a = &(*znode).zbranch[i as usize].key; let b = &(*znode).zbranch[(i + 1) as usize].key; let cmp = keys_cmp(c, a, b); if cmp > 0 || (cmp == 0 && !is_hash_key(c, a)) { kfree(idx); return -EINVAL; } }
    kfree(idx); 0
}

pub unsafe fn ubifs_load_znode(c: *mut ubifs_info, zbr: *mut ubifs_zbranch, parent: *mut ubifs_znode, iip: i32) -> *mut ubifs_znode {
    ubifs_assert(c, (*zbr).znode.is_null());
    let znode = kzalloc((*c).max_znode_sz, GFP_NOFS);
    if znode.is_null() { return ERR_PTR(-ENOMEM); }
    let err = read_znode(c, zbr, znode);
    if err != 0 { kfree(znode); return ERR_PTR(err); }
    atomic_long_inc(&(*c).clean_zn_cnt);
    atomic_long_inc(&ubifs_clean_zn_cnt);
    (*zbr).znode = znode;
    (*znode).parent = parent;
    (*znode).time = ktime_get_seconds();
    (*znode).iip = iip;
    znode
}

pub unsafe fn ubifs_tnc_read_node(c: *mut ubifs_info, zbr: *mut ubifs_zbranch, node: *mut core::ffi::c_void) -> i32 {
    let key = &(*zbr).key;
    let typ = key_type(c, key);
    let wbuf = ubifs_get_wbuf(c, (*zbr).lnum);
    let err = if !wbuf.is_null() { ubifs_read_node_wbuf(wbuf, node, typ, (*zbr).len, (*zbr).lnum, (*zbr).offs) }
              else { ubifs_read_node(c, node, typ, (*zbr).len, (*zbr).lnum, (*zbr).offs) };
    if err != 0 { return err; }
    let mut key1 = core::mem::MaybeUninit::<ubifs_key>::uninit();
    key_read(c, (node as *mut u8).add(UBIFS_KEY_OFFSET as usize), key1.as_mut_ptr());
    if !keys_eq(c, key, key1.as_ptr()) { return -EINVAL; }
    let err = ubifs_node_check_hash(c, node, (*zbr).hash);
    if err != 0 { return err; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
