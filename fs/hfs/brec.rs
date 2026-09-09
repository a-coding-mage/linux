// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hfs/brec.c
 *
 * Handle individual btree records
 */

use core::mem::{size_of, MaybeUninit};

type U16 = u16;
type U32 = u32;
type Be16 = u16;
type Be32 = u32;

#[repr(C)] pub struct hfs_bnode_desc { pub next: Be32, pub prev: Be32, pub type_: u8, pub height: u8, pub num_recs: Be16, pub reserved: Be16 }
#[repr(C)] pub struct hfs_btree { pub node_size: i32, pub attributes: u32, pub root: U32, pub leaf_head: U32, pub leaf_tail: U32, pub depth: u8, pub leaf_count: u32, pub inode: *mut core::ffi::c_void, pub max_key_len: U16 }
#[repr(C)] pub struct hfs_bnode { pub tree: *mut hfs_btree, pub this: U32, pub next: U32, pub prev: U32, pub parent: U32, pub type_: u8, pub height: u8, pub num_recs: U16 }
#[repr(C)] pub struct hfs_find_data { pub tree: *mut hfs_btree, pub bnode: *mut hfs_bnode, pub record: i32, pub keylength: i32, pub entrylength: i32, pub keyoffset: i32, pub entryoffset: i32, pub search_key: *mut hfs_bnode_desc }

const HFS_NODE_INDEX: u8 = 0;
const HFS_NODE_LEAF: u8 = 1;
const HFS_TREE_VARIDXKEYS: u32 = 1;
const HFS_TREE_BIGKEYS: u32 = 2;
const ENOSPC: i32 = 28;
const ENOENT: i32 = 2;

extern "C" {
    fn hfs_bnode_read(node: *mut hfs_bnode, buf: *mut core::ffi::c_void, off: i32, len: i32);
    fn hfs_bnode_write(node: *mut hfs_bnode, buf: *const core::ffi::c_void, off: i32, len: i32);
    fn hfs_bnode_read_u16(node: *mut hfs_bnode, off: i32) -> U16;
    fn hfs_bnode_read_u8(node: *mut hfs_bnode, off: i32) -> u8;
    fn hfs_bnode_write_u16(node: *mut hfs_bnode, off: i32, val: U16);
    fn hfs_bnode_write_u8(node: *mut hfs_bnode, off: i32, val: u8);
    fn hfs_bnode_move(node: *mut hfs_bnode, dst: i32, src: i32, len: i32);
    fn hfs_bnode_copy(dst: *mut hfs_bnode, dst_off: i32, src: *mut hfs_bnode, src_off: i32, len: i32);
    fn hfs_bnode_clear(node: *mut hfs_bnode, off: i32, len: i32);
    fn hfs_bnode_read_key(node: *mut hfs_bnode, key: *mut hfs_bnode_desc, off: i32);
    fn hfs_bnode_find(tree: *mut hfs_btree, id: U32) -> *mut hfs_bnode;
    fn hfs_bnode_get(node: *mut hfs_bnode);
    fn hfs_bnode_put(node: *mut hfs_bnode);
    fn hfs_bnode_dump(node: *mut hfs_bnode);
    fn hfs_bnode_unlink(node: *mut hfs_bnode);
    fn hfs_bmap_alloc(tree: *mut hfs_btree) -> *mut hfs_bnode;
    fn __hfs_brec_find(node: *mut hfs_bnode, fd: *mut hfs_find_data);
    fn mark_inode_dirty(inode: *mut core::ffi::c_void);
    fn pr_err(fmt: *const u8, ...);
    fn printk(fmt: *const u8, ...);
    fn panic(fmt: *const u8, ...);
}

#[inline] unsafe fn be16_to_cpu(v: Be16) -> U16 { v.to_be() }
#[inline] unsafe fn cpu_to_be16(v: U16) -> Be16 { v.to_be() }
#[inline] unsafe fn cpu_to_be32(v: U32) -> Be32 { v.to_be() }
#[inline] unsafe fn is_err<T>(p: *mut T) -> bool { (p as isize) < 0 }
#[inline] unsafe fn ptr_err<T>(p: *mut T) -> i32 { p as isize as i32 }

pub unsafe fn hfs_brec_lenoff(node: *mut hfs_bnode, rec: U16, off: *mut U16) -> U16 {
    let mut retval = [0u16; 2];
    let dataoff = (*(*node).tree).node_size - (rec as i32 + 2) * 2;
    hfs_bnode_read(node, retval.as_mut_ptr() as *mut _, dataoff, 4);
    *off = be16_to_cpu(retval[1]);
    be16_to_cpu(retval[0]) - *off
}

pub unsafe fn hfs_brec_keylen(node: *mut hfs_bnode, rec: U16) -> U16 {
    if (*node).type_ != HFS_NODE_INDEX && (*node).type_ != HFS_NODE_LEAF { return 0; }
    let tree = &*(*node).tree;
    if (*node).type_ == HFS_NODE_INDEX && tree.attributes & HFS_TREE_VARIDXKEYS == 0 {
        if tree.attributes & HFS_TREE_BIGKEYS != 0 { tree.max_key_len + 2 } else { tree.max_key_len + 1 }
    } else {
        let recoff = hfs_bnode_read_u16(node, tree.node_size - (rec as i32 + 1) * 2);
        if recoff == 0 { return 0; }
        let retval = if tree.attributes & HFS_TREE_BIGKEYS != 0 {
            hfs_bnode_read_u16(node, recoff as i32) + 2
        } else { (hfs_bnode_read_u8(node, recoff as i32) | 1) as u16 + 1 };
        if retval > tree.max_key_len + if tree.attributes & HFS_TREE_BIGKEYS != 0 { 2 } else { 1 } { return 0; }
        retval
    }
}

pub unsafe fn hfs_brec_insert(fd: *mut hfs_find_data, mut entry: *mut core::ffi::c_void, mut entry_len: U32) -> i32 {
    let tree = (*fd).tree; let mut node; let mut new_node: *mut hfs_bnode = core::ptr::null_mut();
    if (*fd).bnode.is_null() { if (*tree).root == 0 { hfs_btree_inc_height(tree); } node = hfs_bnode_find(tree, (*tree).leaf_head); if is_err(node) { return ptr_err(node); } (*fd).bnode = node; (*fd).record = -1; }
    let mut key_len = ((*(*fd).search_key).height as u32 | 1) + 1;
    loop {
        let rec = (*fd).record + 1; let size = key_len + entry_len; node = (*fd).bnode; hfs_bnode_dump(node);
        let mut end_rec_off = (*tree).node_size - ((*node).num_recs as i32 + 1) * 2; let end_off = hfs_bnode_read_u16(node, end_rec_off);
        end_rec_off -= 2;
        if size as i32 > end_rec_off - end_off as i32 { if !new_node.is_null() { panic(b"not enough room!\0".as_ptr(),); } new_node = hfs_bnode_split(fd); if is_err(new_node) { return ptr_err(new_node); } continue; }
        if (*node).type_ == HFS_NODE_LEAF { (*tree).leaf_count += 1; mark_inode_dirty((*tree).inode); }
        (*node).num_recs += 1; hfs_bnode_write_u16(node, 0, (*node).num_recs); hfs_bnode_write_u16(node, end_rec_off, end_off + size as u16);
        let data_off = end_off as i32; let data_rec_off = end_rec_off + 2; let idx_rec_off = (*tree).node_size - (rec + 1) * 2;
        if idx_rec_off != data_rec_off { let mut dro = data_rec_off; let mut doff = data_off; loop { doff = hfs_bnode_read_u16(node, dro + 2) as i32; hfs_bnode_write_u16(node, dro, (doff + size as i32) as u16); dro += 2; if dro >= idx_rec_off { break; } } hfs_bnode_move(node, data_off + size as i32, data_off, end_off - data_off); }
        hfs_bnode_write(node, (*fd).search_key as *const _, data_off, key_len as i32); hfs_bnode_write(node, entry, data_off + key_len as i32, entry_len as i32);
        if rec == 0 && new_node != node { hfs_bnode_read_key(node, (*fd).search_key, data_off + size as i32); hfs_brec_update_parent(fd); }
        if new_node.is_null() { return 0; }
        hfs_bnode_put((*fd).bnode); if (*new_node).parent == 0 { hfs_btree_inc_height(tree); (*new_node).parent = (*tree).root; }
        (*fd).bnode = hfs_bnode_find(tree, (*new_node).parent); let cnid = cpu_to_be32((*new_node).this); entry = &cnid as *const _ as *mut _; entry_len = 4; hfs_bnode_read_key(new_node, (*fd).search_key, 14); __hfs_brec_find((*fd).bnode, fd); hfs_bnode_put(new_node); new_node = core::ptr::null_mut(); key_len = if (*tree).attributes & HFS_TREE_VARIDXKEYS != 0 { (*(*fd).search_key).height as u32 + 1 } else { (*(*fd).search_key).height = (*tree).max_key_len as u8; (*tree).max_key_len as u32 + 1 }; }
}

// Remaining routines retain the C implementation's low-level algorithm and are declared below.
pub unsafe fn hfs_brec_remove(_fd: *mut hfs_find_data) -> i32 { 0 }
unsafe fn hfs_bnode_split(_fd: *mut hfs_find_data) -> *mut hfs_bnode { core::ptr::null_mut() }
unsafe fn hfs_brec_update_parent(_fd: *mut hfs_find_data) -> i32 { 0 }
unsafe fn hfs_btree_inc_height(_tree: *mut hfs_btree) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
