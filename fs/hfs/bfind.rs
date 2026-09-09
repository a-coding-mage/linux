// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hfs/bfind.c
 *
 * Copyright (C) 2001
 * Brad Boyer (flar@allandria.com)
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 *
 * Search routines for btrees
 */

use core::ffi::c_void;

pub unsafe fn hfs_find_init(tree: *mut hfs_btree, fd: *mut hfs_find_data) -> i32 {
    let ptr: *mut c_void;

    if tree.is_null() || fd.is_null() {
        return -EINVAL;
    }

    (*fd).tree = tree;
    (*fd).bnode = core::ptr::null_mut();
    ptr = kzalloc((*tree).max_key_len * 2 + 4, GFP_KERNEL);
    if ptr.is_null() {
        return -ENOMEM;
    }
    (*fd).search_key = ptr;
    (*fd).key = (ptr as *mut u8).add((*tree).max_key_len + 2) as *mut c_void;
    hfs_dbg!("cnid %d, caller %ps\n", (*tree).cnid, core::ptr::null::<c_void>());
    match (*tree).cnid {
        HFS_CAT_CNID => mutex_lock_nested(&mut (*tree).tree_lock, CATALOG_BTREE_MUTEX),
        HFS_EXT_CNID => mutex_lock_nested(&mut (*tree).tree_lock, EXTENTS_BTREE_MUTEX),
        HFS_ATTR_CNID => mutex_lock_nested(&mut (*tree).tree_lock, ATTR_BTREE_MUTEX),
        _ => return -EINVAL,
    }
    0
}

pub unsafe fn hfs_find_exit(fd: *mut hfs_find_data) {
    hfs_bnode_put((*fd).bnode);
    kfree((*fd).search_key);
    hfs_dbg!("cnid %d, caller %ps\n", (*(*fd).tree).cnid, core::ptr::null::<c_void>());
    mutex_unlock(&mut (*(*fd).tree).tree_lock);
    (*fd).tree = core::ptr::null_mut();
}

/* Find the record in bnode that best matches key (not greater than...)*/
pub unsafe fn __hfs_brec_find(bnode: *mut hfs_bnode, fd: *mut hfs_find_data) -> i32 {
    let mut cmpval: i32;
    let mut off: u16 = 0;
    let mut len: u16 = 0;
    let mut keylen: u16 = 0;
    let mut rec: i32 = 0;
    let mut b: i32 = 0;
    let mut e: i32 = (*bnode).num_recs as i32 - 1;
    let mut res: i32 = -ENOENT;
    loop {
        rec = (e + b) / 2;
        len = hfs_brec_lenoff(bnode, rec, &mut off);
        keylen = hfs_brec_keylen(bnode, rec);
        if keylen == 0 {
            res = -EINVAL;
            break;
        }
        hfs_bnode_read(bnode, (*fd).key, off, keylen as u32);
        cmpval = (*(*bnode).tree).keycmp((*fd).key, (*fd).search_key);
        if cmpval == 0 {
            e = rec;
            res = 0;
            break;
        }
        if cmpval < 0 {
            b = rec + 1;
        } else {
            e = rec - 1;
        }
        if b > e {
            if rec != e && e >= 0 {
                len = hfs_brec_lenoff(bnode, e, &mut off);
                keylen = hfs_brec_keylen(bnode, e);
                if keylen == 0 {
                    res = -EINVAL;
                    break;
                }
                hfs_bnode_read(bnode, (*fd).key, off, keylen as u32);
            }
            break;
        }
    }
    (*fd).record = e;
    (*fd).keyoffset = off;
    (*fd).keylength = keylen;
    (*fd).entryoffset = off + keylen;
    (*fd).entrylength = len - keylen;
    res
}

/* Traverse a B*Tree from the root to a leaf finding best fit to key */
/* Return allocated copy of node found, set recnum to best record */
pub unsafe fn hfs_brec_find(fd: *mut hfs_find_data) -> i32 {
    let tree = (*fd).tree;
    let mut bnode: *mut hfs_bnode;
    let mut nidx: u32;
    let mut parent: u32;
    let mut data: __be32 = 0;
    let mut height: i32;
    let mut res: i32 = 0;

    (*fd).record = -1;
    (*fd).keyoffset = -1;
    (*fd).keylength = -1;
    (*fd).entryoffset = -1;
    (*fd).entrylength = -1;
    if !(*fd).bnode.is_null() { hfs_bnode_put((*fd).bnode); }
    (*fd).bnode = core::ptr::null_mut();
    nidx = (*tree).root;
    if nidx == 0 { return -ENOENT; }
    height = (*tree).depth;
    parent = 0;
    loop {
        bnode = hfs_bnode_find(tree, nidx);
        if IS_ERR(bnode) { res = PTR_ERR(bnode); bnode = core::ptr::null_mut(); break; }
        if (*bnode).height != height { res = -EIO; hfs_bnode_put(bnode); break; }
        height -= 1;
        if (*bnode).node_type != if height != 0 { HFS_NODE_INDEX } else { HFS_NODE_LEAF } {
            res = -EIO; hfs_bnode_put(bnode); break;
        }
        (*bnode).parent = parent;
        res = __hfs_brec_find(bnode, fd);
        if height == 0 { (*fd).bnode = bnode; return res; }
        if (*fd).record < 0 { hfs_bnode_put(bnode); break; }
        parent = nidx;
        hfs_bnode_read(bnode, &mut data as *mut __be32 as *mut c_void, (*fd).entryoffset, 4);
        nidx = be32_to_cpu(data);
        hfs_bnode_put(bnode);
    }
    res
}

pub unsafe fn hfs_brec_read(fd: *mut hfs_find_data, rec: *mut c_void, rec_len: u32) -> i32 {
    let res = hfs_brec_find(fd);
    if res != 0 { return res; }
    if (*fd).entrylength > rec_len as u16 { return -EINVAL; }
    hfs_bnode_read((*fd).bnode, rec, (*fd).entryoffset, (*fd).entrylength as u32);
    0
}

pub unsafe fn hfs_brec_goto(fd: *mut hfs_find_data, mut cnt: i32) -> i32 {
    let tree = (*(*fd).bnode).tree;
    let mut bnode = (*fd).bnode;
    let mut idx: i32;
    let mut res = 0;
    let mut off = 0u16;
    let mut len: u16;
    let mut keylen: u16;
    if cnt < 0 {
        cnt = -cnt;
        while cnt > (*fd).record {
            cnt -= (*fd).record + 1;
            (*fd).record = (*bnode).num_recs as i32 - 1;
            idx = (*bnode).prev;
            if idx == 0 { res = -ENOENT; break; }
            hfs_bnode_put(bnode); bnode = hfs_bnode_find(tree, idx as u32);
            if IS_ERR(bnode) { res = PTR_ERR(bnode); bnode = core::ptr::null_mut(); break; }
        }
        if res == 0 { (*fd).record -= cnt; }
    } else {
        while cnt >= (*bnode).num_recs as i32 - (*fd).record {
            cnt -= (*bnode).num_recs as i32 - (*fd).record;
            (*fd).record = 0; idx = (*bnode).next;
            if idx == 0 { res = -ENOENT; break; }
            hfs_bnode_put(bnode); bnode = hfs_bnode_find(tree, idx as u32);
            if IS_ERR(bnode) { res = PTR_ERR(bnode); bnode = core::ptr::null_mut(); break; }
        }
        if res == 0 { (*fd).record += cnt; }
    }
    if res == 0 {
        len = hfs_brec_lenoff(bnode, (*fd).record, &mut off);
        keylen = hfs_brec_keylen(bnode, (*fd).record);
        if keylen == 0 { res = -EINVAL; } else {
            (*fd).keyoffset = off; (*fd).keylength = keylen;
            (*fd).entryoffset = off + keylen; (*fd).entrylength = len - keylen;
            hfs_bnode_read(bnode, (*fd).key, off, keylen as u32);
        }
    }
    (*fd).bnode = bnode;
    res
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
