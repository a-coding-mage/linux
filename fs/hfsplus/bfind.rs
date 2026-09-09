// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hfsplus/bfind.c
 *
 * Search routines for btrees
 *
 * Translated from C. Types, constants, globals, and helper functions supplied
 * by hfsplus_fs.h and the kernel remain external dependencies.
 */

unsafe extern "C" {
    fn hfs_find_result_init(fd: *mut hfs_find_data);
    fn hfs_bnode_put(node: *mut hfs_bnode);
    fn hfs_bnode_find(tree: *mut hfs_btree, idx: u32) -> *mut hfs_bnode;
    fn hfs_bnode_read(node: *mut hfs_bnode, dst: *mut core::ffi::c_void, off: u16, len: u16);
    fn hfs_brec_lenoff(node: *mut hfs_bnode, rec: i32, off: *mut u16) -> u16;
    fn hfs_brec_keylen(node: *mut hfs_bnode, rec: i32) -> u16;
    fn hfs_brec_len_invalid(node: *mut hfs_bnode, len: u16) -> bool;
    fn hfs_bnode_num_recs_invalid(node: *mut hfs_bnode) -> bool;
    fn hfsplus_btree_lock_class(tree: *mut hfs_btree) -> *mut core::ffi::c_void;
    fn mutex_lock_nested(lock: *mut core::ffi::c_void, subclass: *mut core::ffi::c_void);
    fn mutex_unlock(lock: *mut core::ffi::c_void);
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn hfs_keycmp(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> i32;
}

extern "C" {
    static HFSPLUS_EXT_CNID: u32;
    static HFSPLUS_CAT_CNID: u32;
    static HFSPLUS_ATTR_CNID: u32;
}

pub unsafe fn hfs_find_init(tree: *mut hfs_btree, fd: *mut hfs_find_data) -> i32 {
    (*fd).tree = tree;
    (*fd).bnode = core::ptr::null_mut();
    hfs_find_result_init(fd);
    let ptr = kzalloc((*tree).max_key_len as usize * 2 + 4, GFP_KERNEL);
    if ptr.is_null() { return -ENOMEM; }
    (*fd).search_key = ptr;
    (*fd).key = (ptr as *mut u8).add((*tree).max_key_len as usize + 2) as *mut _;
    mutex_lock_nested(&mut (*tree).tree_lock, hfsplus_btree_lock_class(tree));
    0
}

pub unsafe fn hfs_find_exit(fd: *mut hfs_find_data) {
    hfs_bnode_put((*fd).bnode);
    kfree((*fd).search_key);
    mutex_unlock(&mut (*(*fd).tree).tree_lock);
    (*fd).tree = core::ptr::null_mut();
}

pub unsafe fn hfs_find_1st_rec_by_cnid(bnode: *mut hfs_bnode, fd: *mut hfs_find_data,
    begin: *mut i32, end: *mut i32, cur_rec: *mut i32) -> i32 {
    let (cur_cnid, search_cnid) = if (*(*bnode).tree).cnid == HFSPLUS_EXT_CNID {
        ((*(*fd).key).ext.cnid, (*(*fd).search_key).ext.cnid)
    } else if (*(*bnode).tree).cnid == HFSPLUS_CAT_CNID {
        ((*(*fd).key).cat.parent, (*(*fd).search_key).cat.parent)
    } else if (*(*bnode).tree).cnid == HFSPLUS_ATTR_CNID {
        ((*(*fd).key).attr.cnid, (*(*fd).search_key).attr.cnid)
    } else { BUG!(); (0, 0) };
    if cur_cnid == search_cnid {
        *end = *cur_rec;
        if *begin == *end { return 1; }
    } else if be32_to_cpu(cur_cnid) < be32_to_cpu(search_cnid) { *begin = *cur_rec + 1; }
    else { *end = *cur_rec - 1; }
    0
}

pub unsafe fn hfs_find_rec_by_key(bnode: *mut hfs_bnode, fd: *mut hfs_find_data,
    begin: *mut i32, end: *mut i32, cur_rec: *mut i32) -> i32 {
    let cmpval = ((*(*bnode).tree).keycmp)((*fd).key, (*fd).search_key);
    if cmpval == 0 { *end = *cur_rec; return 1; }
    if cmpval < 0 { *begin = *cur_rec + 1; } else { *end = *cur_rec - 1; }
    0
}

pub unsafe fn __hfs_brec_find(bnode: *mut hfs_bnode, fd: *mut hfs_find_data,
    rec_found: search_strategy_t) -> i32 {
    BUG_ON!(rec_found.is_none());
    hfs_find_result_init(fd);
    if hfs_bnode_num_recs_invalid(bnode) { return -ENOENT; }
    let mut b = 0i32; let mut e = (*bnode).num_recs as i32 - 1;
    let mut off = 0u16; let mut len = 0u16; let mut keylen = 0u16; let mut rec;
    loop {
        rec = (e + b) / 2;
        len = hfs_brec_lenoff(bnode, rec, &mut off); keylen = hfs_brec_keylen(bnode, rec);
        if hfs_brec_len_invalid(bnode, len) || hfs_brec_len_invalid(bnode, keylen) { return -EINVAL; }
        hfs_bnode_read(bnode, (*fd).key, off, keylen);
        if rec_found(bnode, fd, &mut b, &mut e, &mut rec) != 0 { break; }
        if b > e { break; }
    }
    if rec != e && e >= 0 {
        len = hfs_brec_lenoff(bnode, e, &mut off); keylen = hfs_brec_keylen(bnode, e);
        if hfs_brec_len_invalid(bnode, keylen) || hfs_brec_len_invalid(bnode, len) { return -EINVAL; }
        hfs_bnode_read(bnode, (*fd).key, off, keylen);
    }
    (*fd).record = e; (*fd).keyoffset = off; (*fd).keylength = keylen;
    (*fd).entryoffset = off + keylen; (*fd).entrylength = len - keylen; 0
}

pub unsafe fn hfs_brec_find(fd: *mut hfs_find_data, cmp: search_strategy_t) -> i32 {
    hfs_find_result_init(fd);
    let tree = (*fd).tree;
    if !(*fd).bnode.is_null() { hfs_bnode_put((*fd).bnode); }
    (*fd).bnode = core::ptr::null_mut(); let mut nidx = (*tree).root;
    if nidx == 0 { return -ENOENT; }
    let mut height = (*tree).depth; let mut parent = 0; let mut res = 0;
    loop {
        let bnode = hfs_bnode_find(tree, nidx);
        if bnode.is_null() { return -EIO; }
        if (*bnode).height != height || (*bnode).node_type != (if { height -= 1; height != 0 } { HFS_NODE_INDEX } else { HFS_NODE_LEAF }) { hfs_bnode_put(bnode); return -EIO; }
        (*bnode).parent = parent; res = __hfs_brec_find(bnode, fd, cmp);
        if height == 0 { (*fd).bnode = bnode; return res; }
        if (*fd).record < 0 { hfs_bnode_put(bnode); return res; }
        parent = nidx; let mut data = 0u32; hfs_bnode_read(bnode, &mut data as *mut _ as *mut _, (*fd).entryoffset, 4);
        nidx = be32_to_cpu(data); hfs_bnode_put(bnode);
    }
}

pub unsafe fn hfs_brec_read(fd: *mut hfs_find_data, rec: *mut core::ffi::c_void, rec_len: u32) -> i32 {
    let res = hfs_brec_find(fd, Some(hfs_find_rec_by_key)); if res != 0 { return res; }
    if (*fd).entrylength as u32 > rec_len { return -EINVAL; }
    hfs_bnode_read((*fd).bnode, rec, (*fd).entryoffset, (*fd).entrylength); 0
}

pub unsafe fn hfs_brec_goto(fd: *mut hfs_find_data, mut cnt: i32) -> i32 {
    let tree = (*fd).tree; let mut bnode = (*fd).bnode; let mut res = 0;
    if cnt < 0 { cnt = -cnt; while cnt > (*fd).record { cnt -= (*fd).record + 1; (*fd).record = (*bnode).num_recs as i32 - 1; let idx = (*bnode).prev; if idx == 0 { return -ENOENT; } hfs_bnode_put(bnode); bnode = hfs_bnode_find(tree, idx); } (*fd).record -= cnt; }
    else { while cnt >= (*bnode).num_recs as i32 - (*fd).record { cnt -= (*bnode).num_recs as i32 - (*fd).record; (*fd).record = 0; let idx = (*bnode).next; if idx == 0 { return -ENOENT; } hfs_bnode_put(bnode); bnode = hfs_bnode_find(tree, idx); } (*fd).record += cnt; }
    let mut off = 0; let len = hfs_brec_lenoff(bnode, (*fd).record, &mut off); let keylen = hfs_brec_keylen(bnode, (*fd).record);
    if hfs_brec_len_invalid(bnode, len) || hfs_brec_len_invalid(bnode, keylen) { res = -EINVAL; } else { (*fd).keyoffset = off; (*fd).keylength = keylen; (*fd).entryoffset = off + keylen; (*fd).entrylength = len - keylen; hfs_bnode_read(bnode, (*fd).key, off, keylen); }
    (*fd).bnode = bnode; res
}

pub unsafe fn hfsplus_brec_read_cat(fd: *mut hfs_find_data, entry: *mut hfsplus_cat_entry) -> i32 {
    let res = hfs_brec_read(fd, entry as *mut _, core::mem::size_of::<hfsplus_cat_entry>() as u32);
    if res != 0 { return res; }
    let expected_size = match be16_to_cpu((*entry).type_) {
        HFSPLUS_FOLDER => core::mem::size_of::<hfsplus_cat_folder>() as u32,
        HFSPLUS_FILE => core::mem::size_of::<hfsplus_cat_file>() as u32,
        HFSPLUS_FOLDER_THREAD | HFSPLUS_FILE_THREAD => {
            if (*fd).entrylength as u32 < HFSPLUS_MIN_THREAD_SZ { return -EIO; }
            hfsplus_cat_thread_size(&mut (*entry).thread) as u32
        }
        _ => return -EIO,
    };
    if (*fd).entrylength as u32 != expected_size { return -EIO; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
