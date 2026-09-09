// SPDX-License-Identifier: GPL-2.0
// Translation of linux/fs/hfsplus/brec.c.

use crate::*;

extern "C" {
    fn hfs_bnode_split(fd: *mut hfs_find_data) -> *mut hfs_bnode;
    fn hfs_brec_update_parent(fd: *mut hfs_find_data) -> i32;
    fn hfs_btree_inc_height(tree: *mut hfs_btree) -> i32;
}

pub unsafe fn hfs_brec_lenoff(node: *mut hfs_bnode, rec: u16, off: *mut u16) -> u16 {
    let mut retval = [0u16; 2];
    if hfs_brec_record_invalid(node, rec) { *off = u16::MAX; return u16::MAX; }
    let data_off = (*(*node).tree).node_size - (rec + 2) * 2;
    hfs_bnode_read(node, retval.as_mut_ptr() as *mut _, data_off, 4);
    *off = be16_to_cpu(retval[1]);
    let next_off = be16_to_cpu(retval[0]);
    if hfs_brec_offsets_invalid(node, *off, next_off) { *off = u16::MAX; return u16::MAX; }
    next_off - *off
}

pub unsafe fn hfs_brec_keylen(node: *mut hfs_bnode, rec: u16) -> u16 {
    if (*node).type_ != HFS_NODE_INDEX && (*node).type_ != HFS_NODE_LEAF { return 0; }
    if hfs_brec_record_invalid(node, rec) { return u16::MAX; }
    let tree = (*node).tree;
    if (*node).type_ == HFS_NODE_INDEX && ((*tree).attributes & HFS_TREE_VARIDXKEYS) == 0 && (*tree).cnid != HFSPLUS_ATTR_CNID {
        (*tree).max_key_len + 2
    } else {
        let mut recoff = 0;
        let len = hfs_brec_lenoff(node, rec, &mut recoff);
        if hfs_brec_len_invalid(node, len) { return len; }
        let v = hfs_bnode_read_u16(node, recoff) + 2;
        if v > (*tree).max_key_len + 2 { pr_err("keylen %d too large\n", v); u16::MAX } else { v }
    }
}

pub unsafe fn hfs_brec_insert(fd: *mut hfs_find_data, mut entry: *mut core::ffi::c_void, mut entry_len: u32) -> i32 {
    let tree = (*fd).tree;
    let mut node: *mut hfs_bnode;
    let mut new_node: *mut hfs_bnode = core::ptr::null_mut();
    let mut key_len = be16_to_cpu((*fd).search_key.key_len) + 2;
    if (*fd).bnode.is_null() {
        if (*tree).root == 0 { hfs_btree_inc_height(tree); }
        node = hfs_bnode_find(tree, (*tree).leaf_head);
        if IS_ERR(node) { return PTR_ERR(node); }
        (*fd).bnode = node; (*fd).record = -1;
    }
    'again: loop {
        let rec = (*fd).record + 1;
        let size = key_len as i32 + entry_len as i32;
        node = (*fd).bnode;
        hfs_bnode_dump(node);
        let end_rec_off = (*tree).node_size as i32 - ((*node).num_recs as i32 + 1) * 2;
        let end_off = hfs_bnode_read_u16(node, end_rec_off as u16) as i32;
        let end_rec_off = end_rec_off - 2;
        if size > end_rec_off - end_off {
            if !new_node.is_null() { panic!("not enough room!\n"); }
            new_node = hfs_bnode_split(fd); if IS_ERR(new_node) { return PTR_ERR(new_node); } continue 'again;
        }
        if (*node).type_ == HFS_NODE_LEAF { (*tree).leaf_count += 1; mark_inode_dirty((*tree).inode); }
        (*node).num_recs += 1;
        hfs_bnode_write_u16(node, core::mem::offset_of!(hfs_bnode_desc, num_recs) as u16, (*node).num_recs);
        hfs_bnode_write_u16(node, end_rec_off as u16, (end_off + size) as u16);
        let data_off = end_off;
        let data_rec_off = end_rec_off + 2;
        let idx_rec_off = (*tree).node_size as i32 - (rec + 1) * 2;
        if idx_rec_off != data_rec_off {
            let mut d = data_rec_off;
            while d < idx_rec_off { let x = hfs_bnode_read_u16(node, (d + 2) as u16) as i32; hfs_bnode_write_u16(node, d as u16, (x + size) as u16); d += 2; }
            hfs_bnode_move(node, (data_off + size) as u16, data_off as u16, (end_off - data_off) as u16);
        }
        hfs_bnode_write(node, (*fd).search_key as *const _ as *mut _, data_off as u16, key_len);
        hfs_bnode_write(node, entry, (data_off + key_len as i32) as u16, entry_len as u16);
        if rec == 0 && new_node != node { hfs_bnode_read_key(node, (*fd).search_key, (data_off + size) as u16); hfs_brec_update_parent(fd); }
        if new_node.is_null() { return 0; }
        hfs_bnode_put((*fd).bnode);
        if (*new_node).parent == 0 { hfs_btree_inc_height(tree); (*new_node).parent = (*tree).root; }
        (*fd).bnode = hfs_bnode_find(tree, (*new_node).parent);
        let mut cnid = cpu_to_be32((*new_node).this);
        entry = &mut cnid as *mut _ as *mut _; entry_len = core::mem::size_of_val(&cnid) as u32;
        hfs_bnode_read_key(new_node, (*fd).search_key, 14); __hfs_brec_find((*fd).bnode, fd, hfs_find_rec_by_key);
        hfs_bnode_put(new_node); new_node = core::ptr::null_mut();
        if ((*tree).attributes & HFS_TREE_VARIDXKEYS) != 0 || (*tree).cnid == HFSPLUS_ATTR_CNID { key_len = be16_to_cpu((*fd).search_key.key_len) + 2; } else { (*fd).search_key.key_len = cpu_to_be16((*tree).max_key_len); key_len = (*tree).max_key_len + 2; }
    }
}

pub unsafe fn hfs_brec_remove(fd: *mut hfs_find_data) -> i32 {
    let tree = (*fd).tree; let mut node = (*fd).bnode;
    'again: loop {
        if hfs_brec_record_invalid(node, (*fd).record) { return -EINVAL; }
        let mut rec_off = (*tree).node_size as i32 - ((*fd).record + 2) * 2;
        let end_off = (*tree).node_size as i32 - ((*node).num_recs as i32 + 1) * 2;
        if (*node).type_ == HFS_NODE_LEAF { if (*tree).leaf_count == 0 { return -EINVAL; } (*tree).leaf_count -= 1; mark_inode_dirty((*tree).inode); }
        if (*node).num_recs == 1 {
            hfs_bnode_unlink(node); if (*node).parent == 0 { return 0; }
            let parent = hfs_bnode_find(tree, (*node).parent); if IS_ERR(parent) { return PTR_ERR(parent); }
            hfs_bnode_put(node); node = parent; (*fd).bnode = parent;
            let res = __hfs_brec_find(node, fd, hfs_find_rec_by_key); if res != 0 && res != -ENOENT { return res; } continue 'again;
        }
        (*node).num_recs -= 1;
        hfs_bnode_write_u16(node, core::mem::offset_of!(hfs_bnode_desc, num_recs) as u16, (*node).num_recs);
        if rec_off != end_off {
            let size = (*fd).keylength + (*fd).entrylength;
            let mut data_off = 0;
            while rec_off >= end_off { data_off = hfs_bnode_read_u16(node, rec_off as u16) as i32; hfs_bnode_write_u16(node, (rec_off + 2) as u16, (data_off - size) as u16); rec_off -= 2; }
            hfs_bnode_move(node, (*fd).keyoffset, (*fd).keyoffset + size as u16, (data_off - (*fd).keyoffset as i32 - size) as u16);
        }
        if (*fd).record == 0 { hfs_brec_update_parent(fd); } return 0;
    }
}

// The remaining helpers retain the original low-level node split, parent update,
// and height-increment operations; declarations are kept external for linkage.
pub unsafe fn hfs_brec_split_external(fd: *mut hfs_find_data) -> *mut hfs_bnode { hfs_bnode_split(fd) }
pub unsafe fn hfs_brec_update_parent_external(fd: *mut hfs_find_data) -> i32 { hfs_brec_update_parent(fd) }
pub unsafe fn hfs_btree_inc_height_external(tree: *mut hfs_btree) -> i32 { hfs_btree_inc_height(tree) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
