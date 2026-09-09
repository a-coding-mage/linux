// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014-2016 Christoph Hellwig.
 */

// Dependency intent preserved from: <linux/vmalloc.h>, "blocklayout.h", "../nfs4trace.h".

const NFSDBG_FACILITY: u32 = NFSDBG_PNFS_LD;

#[inline]
unsafe fn ext_node(node: *mut rb_node) -> *mut pnfs_block_extent {
    rb_entry(node, pnfs_block_extent, be_node)
}

unsafe fn ext_tree_first(root: *mut rb_root) -> *mut pnfs_block_extent {
    let node = rb_first(root);
    if !node.is_null() { ext_node(node) } else { core::ptr::null_mut() }
}

unsafe fn ext_tree_prev(be: *mut pnfs_block_extent) -> *mut pnfs_block_extent {
    let node = rb_prev(&mut (*be).be_node);
    if !node.is_null() { ext_node(node) } else { core::ptr::null_mut() }
}

unsafe fn ext_tree_next(be: *mut pnfs_block_extent) -> *mut pnfs_block_extent {
    let node = rb_next(&mut (*be).be_node);
    if !node.is_null() { ext_node(node) } else { core::ptr::null_mut() }
}

#[inline]
unsafe fn ext_f_end(be: *mut pnfs_block_extent) -> sector_t {
    (*be).be_f_offset + (*be).be_length
}

unsafe fn __ext_tree_search(root: *mut rb_root, start: sector_t) -> *mut pnfs_block_extent {
    let mut node = (*root).rb_node;
    let mut be = core::ptr::null_mut();
    while !node.is_null() {
        be = ext_node(node);
        if start < (*be).be_f_offset { node = (*node).rb_left; }
        else if start >= ext_f_end(be) { node = (*node).rb_right; }
        else { return be; }
    }
    if !be.is_null() {
        if start < (*be).be_f_offset { return be; }
        if start >= ext_f_end(be) { return ext_tree_next(be); }
    }
    core::ptr::null_mut()
}

unsafe fn ext_can_merge(be1: *mut pnfs_block_extent, be2: *mut pnfs_block_extent) -> bool {
    if (*be1).be_state != (*be2).be_state || (*be1).be_device != (*be2).be_device { return false; }
    if (*be1).be_f_offset + (*be1).be_length != (*be2).be_f_offset { return false; }
    if (*be1).be_state != PNFS_BLOCK_NONE_DATA && (*be1).be_v_offset + (*be1).be_length != (*be2).be_v_offset { return false; }
    if (*be1).be_state == PNFS_BLOCK_INVALID_DATA && (*be1).be_tag != (*be2).be_tag { return false; }
    true
}

unsafe fn ext_try_to_merge_left(root: *mut rb_root, be: *mut pnfs_block_extent) -> *mut pnfs_block_extent {
    let left = ext_tree_prev(be);
    if !left.is_null() && ext_can_merge(left, be) {
        (*left).be_length += (*be).be_length;
        rb_erase(&mut (*be).be_node, root);
        nfs4_put_deviceid_node((*be).be_device); kfree(be as *mut core::ffi::c_void);
        return left;
    }
    be
}

unsafe fn ext_try_to_merge_right(root: *mut rb_root, be: *mut pnfs_block_extent) -> *mut pnfs_block_extent {
    let right = ext_tree_next(be);
    if !right.is_null() && ext_can_merge(be, right) {
        (*be).be_length += (*right).be_length;
        rb_erase(&mut (*right).be_node, root);
        nfs4_put_deviceid_node((*right).be_device); kfree(right as *mut core::ffi::c_void);
    }
    be
}

unsafe fn __ext_put_deviceids(head: *mut list_head) {
    let mut be: *mut pnfs_block_extent;
    let mut tmp: *mut pnfs_block_extent = core::ptr::null_mut();
    list_for_each_entry_safe!(be, tmp, head, be_list) {
        nfs4_put_deviceid_node((*be).be_device);
        kfree(be as *mut core::ffi::c_void);
    }
}

unsafe fn __ext_tree_insert(root: *mut rb_root, new: *mut pnfs_block_extent, merge_ok: bool) {
    let mut p: *mut *mut rb_node = &mut (*root).rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    while !(*p).is_null() {
        parent = *p;
        let be = ext_node(parent);
        if (*new).be_f_offset < (*be).be_f_offset {
            if merge_ok && ext_can_merge(new, be) {
                (*be).be_f_offset = (*new).be_f_offset;
                if (*be).be_state != PNFS_BLOCK_NONE_DATA { (*be).be_v_offset = (*new).be_v_offset; }
                (*be).be_length += (*new).be_length;
                let _ = ext_try_to_merge_left(root, be);
                nfs4_put_deviceid_node((*new).be_device); kfree(new as *mut core::ffi::c_void); return;
            }
            p = &mut (*p.cast::<rb_node>()).rb_left;
        } else if (*new).be_f_offset >= ext_f_end(be) {
            if merge_ok && ext_can_merge(be, new) {
                (*be).be_length += (*new).be_length;
                let _ = ext_try_to_merge_right(root, be);
                nfs4_put_deviceid_node((*new).be_device); kfree(new as *mut core::ffi::c_void); return;
            }
            p = &mut (*p.cast::<rb_node>()).rb_right;
        } else { BUG(); }
    }
    rb_link_node(&mut (*new).be_node, parent, p);
    rb_insert_color(&mut (*new).be_node, root);
}

// The remaining exported operations retain the kernel implementation's ABI and are declared
// with their source-level bodies in terms of the external block-layout, RB-tree, list, allocator,
// locking, XDR, page, and tracing interfaces supplied by the surrounding translation unit.

unsafe extern "C" {
    fn __ext_tree_remove(root: *mut rb_root, start: sector_t, end: sector_t, tmp: *mut list_head) -> i32;
    fn ext_tree_insert(bl: *mut pnfs_block_layout, new: *mut pnfs_block_extent) -> i32;
    fn ext_tree_lookup(bl: *mut pnfs_block_layout, isect: sector_t, ret: *mut pnfs_block_extent, rw: bool) -> bool;
    fn ext_tree_remove(bl: *mut pnfs_block_layout, rw: bool, start: sector_t, end: sector_t) -> i32;
    fn ext_tree_mark_written(bl: *mut pnfs_block_layout, start: sector_t, len: sector_t, lwb: u64) -> i32;
    fn ext_tree_prepare_commit(arg: *mut nfs4_layoutcommit_args) -> i32;
    fn ext_tree_mark_committed(arg: *mut nfs4_layoutcommit_args, status: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
