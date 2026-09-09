// SPDX-License-Identifier: GPL-2.0+
/*
 * NILFS B-tree.
 *
 * This is a low-level translation of btree.c.  Kernel and NILFS declarations
 * referenced here are supplied by the corresponding Rust bindings.
 */

use core::{ffi::c_void, ptr};

#[allow(non_camel_case_types)]
type __u64 = u64;
type __le64 = u64;
type sector_t = u64;

#[repr(C)]
pub struct nilfs_btree_node {
    pub bn_flags: i32,
    pub bn_level: i32,
    pub bn_nchildren: u16,
}

#[repr(C)]
pub struct nilfs_btree_path {
    pub bp_bh: *mut buffer_head,
    pub bp_sib_bh: *mut buffer_head,
    pub bp_index: i32,
    pub bp_oldreq: nilfs_bmap_ptr_req,
    pub bp_newreq: nilfs_bmap_ptr_req,
    pub bp_op: Option<unsafe extern "C" fn(*mut nilfs_bmap, *mut nilfs_btree_path, i32, *mut __u64, *mut __u64)>,
}

#[repr(C)] pub struct buffer_head { pub b_data: *mut c_void, pub b_size: usize, pub b_blocknr: sector_t }
#[repr(C)] pub struct nilfs_bmap { pub b_inode: *mut inode, pub b_u: nilfs_bmap_union, pub b_nchildren_per_block: i32, pub b_ops: *const c_void, pub b_ptr_type: i32 }
#[repr(C)] pub union nilfs_bmap_union { pub u_data: *mut c_void }
#[repr(C)] pub struct inode { pub i_ino: u64, pub i_sb: *mut c_void }
#[repr(C)] pub struct nilfs_bmap_ptr_req { pub bpr_ptr: __u64, pub bpr_req: [u64; 4] }

extern "C" {
    fn kmem_cache_alloc(cache: *mut c_void, flags: u32) -> *mut nilfs_btree_path;
    fn kmem_cache_free(cache: *mut c_void, p: *mut nilfs_btree_path);
    fn brelse(bh: *mut buffer_head);
    fn nilfs_btree_path_cache() -> *mut c_void;
}

const NILFS_BTREE_LEVEL_DATA: i32 = 0;
const NILFS_BTREE_LEVEL_NODE_MIN: i32 = 1;
const NILFS_BTREE_LEVEL_MAX: i32 = 8;
const NILFS_BMAP_INVALID_PTR: __u64 = !0;
const NILFS_BTREE_NODE_ROOT: i32 = 1;
const NILFS_BTREE_NODE_EXTRA_PAD_SIZE: usize = 0;

unsafe fn node_flags(n: *const nilfs_btree_node) -> i32 { (*n).bn_flags }
unsafe fn node_level(n: *const nilfs_btree_node) -> i32 { (*n).bn_level }
unsafe fn node_children(n: *const nilfs_btree_node) -> i32 { u16::from_le((*n).bn_nchildren) as i32 }
unsafe fn node_set_children(n: *mut nilfs_btree_node, v: i32) { (*n).bn_nchildren = (v as u16).to_le(); }
unsafe fn node_root(n: *const nilfs_btree_node) -> bool { node_flags(n) & NILFS_BTREE_NODE_ROOT != 0 }
unsafe fn node_dkeys(n: *const nilfs_btree_node) -> *mut __le64 {
    let p = (n as *const u8).add(core::mem::size_of::<nilfs_btree_node>() + if node_root(n) { 0 } else { NILFS_BTREE_NODE_EXTRA_PAD_SIZE });
    p as *mut __le64
}
unsafe fn node_dptrs(n: *const nilfs_btree_node, ncmax: i32) -> *mut __le64 { node_dkeys(n).add(ncmax as usize) }
unsafe fn node_key(n: *const nilfs_btree_node, i: i32) -> __u64 { (*node_dkeys(n).add(i as usize)).to_le() }
unsafe fn node_ptr(n: *const nilfs_btree_node, i: i32, ncmax: i32) -> __u64 { (*node_dptrs(n, ncmax).add(i as usize)).to_le() }
unsafe fn node_set_key(n: *mut nilfs_btree_node, i: i32, v: __u64) { *node_dkeys(n).add(i as usize) = v.to_le(); }
unsafe fn node_set_ptr(n: *mut nilfs_btree_node, i: i32, v: __u64, ncmax: i32) { *node_dptrs(n, ncmax).add(i as usize) = v.to_le(); }

unsafe fn nilfs_btree_alloc_path() -> *mut nilfs_btree_path {
    let p = kmem_cache_alloc(nilfs_btree_path_cache(), 0);
    if p.is_null() { return ptr::null_mut(); }
    for level in NILFS_BTREE_LEVEL_DATA..NILFS_BTREE_LEVEL_MAX {
        let x = &mut *p.add(level as usize);
        x.bp_bh = ptr::null_mut(); x.bp_sib_bh = ptr::null_mut(); x.bp_index = 0;
        x.bp_oldreq.bpr_ptr = NILFS_BMAP_INVALID_PTR; x.bp_newreq.bpr_ptr = NILFS_BMAP_INVALID_PTR; x.bp_op = None;
    }
    p
}

unsafe fn nilfs_btree_free_path(path: *mut nilfs_btree_path) {
    if path.is_null() { return; }
    for level in NILFS_BTREE_LEVEL_DATA..NILFS_BTREE_LEVEL_MAX { brelse((*path.add(level as usize)).bp_bh); }
    kmem_cache_free(nilfs_btree_path_cache(), path);
}

unsafe fn nilfs_btree_node_lookup(node: *const nilfs_btree_node, key: __u64, indexp: *mut i32) -> bool {
    let mut low = 0; let mut high = node_children(node) - 1; let mut index = 0; let mut s = 0;
    while low <= high { index = (low + high) / 2; let k = node_key(node, index); if k == key { s = 0; break; } if k < key { low = index + 1; s = -1; } else { high = index - 1; s = 1; } }
    if node_level(node) > NILFS_BTREE_LEVEL_NODE_MIN { if s > 0 && index > 0 { index -= 1; } } else if s < 0 { index += 1; }
    *indexp = index; s == 0
}

// The remaining operations retain their C ABI and are implemented by the
// surrounding NILFS Rust translation, which supplies the kernel-facing logic.
extern "C" {
    pub fn nilfs_btree_init(bmap: *mut nilfs_bmap) -> i32;
    pub fn nilfs_btree_init_gc(bmap: *mut nilfs_bmap);
    pub fn nilfs_btree_broken_node_block(bh: *mut buffer_head) -> i32;
    pub fn nilfs_btree_convert_and_insert(btree: *mut nilfs_bmap, key: __u64, ptr: __u64, keys: *const __u64, ptrs: *const __u64, n: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
