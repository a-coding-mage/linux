/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  linux/fs/hfs/btree.h
 *
 * Copyright (C) 2001
 * Brad Boyer (flar@allandria.com)
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 */

use core::ffi::{c_char, c_void};

pub type btree_keycmp = unsafe extern "C" fn(*const btree_key, *const btree_key) -> i32;

pub const NODE_HASH_SIZE: usize = 256;

/* B-tree mutex nested subclasses */
#[repr(C)]
pub enum hfs_btree_mutex_classes {
    CATALOG_BTREE_MUTEX,
    EXTENTS_BTREE_MUTEX,
    ATTR_BTREE_MUTEX,
}

/* A HFS BTree held in memory */
#[repr(C)]
pub struct hfs_btree {
    pub sb: *mut super_block,
    pub inode: *mut inode,
    pub keycmp: Option<btree_keycmp>,

    pub cnid: u32,
    pub root: u32,
    pub leaf_count: u32,
    pub leaf_head: u32,
    pub leaf_tail: u32,
    pub node_count: u32,
    pub free_nodes: u32,
    pub attributes: u32,

    pub node_size: u32,
    pub node_size_shift: u32,
    pub max_key_len: u32,
    pub depth: u32,

    // unsigned int map1_size, map_size;
    pub tree_lock: mutex,

    pub pages_per_bnode: u32,
    pub hash_lock: spinlock_t,
    pub node_hash: [*mut hfs_bnode; NODE_HASH_SIZE],
    pub node_hash_cnt: i32,
}

/* A HFS BTree node in memory */
#[repr(C)]
pub struct hfs_bnode {
    pub tree: *mut hfs_btree,

    pub prev: u32,
    pub this: u32,
    pub next: u32,
    pub parent: u32,

    pub num_recs: u16,
    pub type_: u8,
    pub height: u8,

    pub next_hash: *mut hfs_bnode,
    pub flags: c_ulong,
    pub lock_wq: wait_queue_head_t,
    pub refcnt: atomic_t,
    pub page_offset: u32,
    pub page: [*mut page; 0],
}

pub const HFS_BNODE_ERROR: i32 = 0;
pub const HFS_BNODE_NEW: i32 = 1;
pub const HFS_BNODE_DELETED: i32 = 2;

#[repr(C)]
pub struct hfs_find_data {
    pub key: *mut btree_key,
    pub search_key: *mut btree_key,
    pub tree: *mut hfs_btree,
    pub bnode: *mut hfs_bnode,
    pub record: i32,
    pub keyoffset: i32,
    pub keylength: i32,
    pub entryoffset: i32,
    pub entrylength: i32,
}

extern "C" {
    pub fn hfs_btree_open(sb: *mut super_block, id: u32, keycmp: Option<btree_keycmp>) -> *mut hfs_btree;
    pub fn hfs_btree_close(tree: *mut hfs_btree);
    pub fn hfs_btree_write(tree: *mut hfs_btree);
    pub fn hfs_bmap_reserve(tree: *mut hfs_btree, rsvd_nodes: u32) -> i32;
    pub fn hfs_bmap_alloc(tree: *mut hfs_btree) -> *mut hfs_bnode;
    pub fn hfs_bmap_free(node: *mut hfs_bnode);

    pub fn hfs_bnode_read(node: *mut hfs_bnode, buf: *mut c_void, off: u32, len: u32);
    pub fn hfs_bnode_read_u16(node: *mut hfs_bnode, off: u32) -> u16;
    pub fn hfs_bnode_read_u8(node: *mut hfs_bnode, off: u32) -> u8;
    pub fn hfs_bnode_read_key(node: *mut hfs_bnode, key: *mut c_void, off: u32);
    pub fn hfs_bnode_write(node: *mut hfs_bnode, buf: *mut c_void, off: u32, len: u32);
    pub fn hfs_bnode_write_u16(node: *mut hfs_bnode, off: u32, data: u16);
    pub fn hfs_bnode_write_u8(node: *mut hfs_bnode, off: u32, data: u8);
    pub fn hfs_bnode_clear(node: *mut hfs_bnode, off: u32, len: u32);
    pub fn hfs_bnode_copy(dst_node: *mut hfs_bnode, dst: u32, src_node: *mut hfs_bnode, src: u32, len: u32);
    pub fn hfs_bnode_move(node: *mut hfs_bnode, dst: u32, src: u32, len: u32);
    pub fn hfs_bnode_dump(node: *mut hfs_bnode);
    pub fn hfs_bnode_unlink(node: *mut hfs_bnode);
    pub fn hfs_bnode_findhash(tree: *mut hfs_btree, cnid: u32) -> *mut hfs_bnode;
    pub fn hfs_bnode_find(tree: *mut hfs_btree, num: u32) -> *mut hfs_bnode;
    pub fn hfs_bnode_unhash(node: *mut hfs_bnode);
    pub fn hfs_bnode_free(node: *mut hfs_bnode);
    pub fn hfs_bnode_create(tree: *mut hfs_btree, num: u32) -> *mut hfs_bnode;
    pub fn hfs_bnode_get(node: *mut hfs_bnode);
    pub fn hfs_bnode_put(node: *mut hfs_bnode);

    pub fn hfs_brec_lenoff(node: *mut hfs_bnode, rec: u16, off: *mut u16) -> u16;
    pub fn hfs_brec_keylen(node: *mut hfs_bnode, rec: u16) -> u16;
    pub fn hfs_brec_insert(fd: *mut hfs_find_data, entry: *mut c_void, entry_len: u32) -> i32;
    pub fn hfs_brec_remove(fd: *mut hfs_find_data) -> i32;

    pub fn hfs_find_init(tree: *mut hfs_btree, fd: *mut hfs_find_data) -> i32;
    pub fn hfs_find_exit(fd: *mut hfs_find_data);
    pub fn __hfs_brec_find(bnode: *mut hfs_bnode, fd: *mut hfs_find_data) -> i32;
    pub fn hfs_brec_find(fd: *mut hfs_find_data) -> i32;
    pub fn hfs_brec_read(fd: *mut hfs_find_data, rec: *mut c_void, rec_len: u32) -> i32;
    pub fn hfs_brec_goto(fd: *mut hfs_find_data, cnt: i32) -> i32;

    fn pr_err(fmt: *const c_char, ...);
}

pub unsafe fn is_bnode_offset_valid(node: *mut hfs_bnode, off: u32) -> bool {
    if node.is_null() || (*node).tree.is_null() {
        return false;
    }
    let is_valid = off < (*(*node).tree).node_size;
    if !is_valid {
        pr_err(c"invalid offset: id %u, type %#x, h %u, sz %u, off %u\n".as_ptr(), (*node).this, (*node).type_, (*node).height, (*(*node).tree).node_size, off);
    }
    is_valid
}

pub unsafe fn check_and_correct_requested_length(node: *mut hfs_bnode, off: u32, len: u32) -> u32 {
    if !is_bnode_offset_valid(node, off) {
        return 0;
    }
    let node_size = (*(*node).tree).node_size;
    if (off as u64) + (len as u64) > node_size as u64 {
        let new_len = node_size - off;
        pr_err(c"corrected len: id %u, type %#x, h %u, sz %u, off %u, len %u->%u\n".as_ptr(), (*node).this, (*node).type_, (*node).height, node_size, off, len, new_len);
        return new_len;
    }
    len
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
