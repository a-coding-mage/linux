/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * NILFS B-tree.
 *
 * Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Koji Sato.
 */

// Dependencies supplied by the corresponding translated kernel/NILFS headers:
// linux/types.h, linux/buffer_head.h, linux/list.h, linux/nilfs2_ondisk.h,
// btnode.h, and bmap.h.

/**
 * A path on which B-tree operations are executed.
 *
 * @bp_bh: buffer head of node block
 * @bp_sib_bh: buffer head of sibling node block
 * @bp_index: index of child node
 * @bp_oldreq: ptr end request for old ptr
 * @bp_newreq: ptr alloc request for new ptr
 * @bp_ctxt: context information for changing the key of a b-tree node block
 * @bp_op: rebalance operation
 */
#[repr(C)]
pub struct nilfs_btree_path {
    pub bp_bh: *mut buffer_head,
    pub bp_sib_bh: *mut buffer_head,
    pub bp_index: core::ffi::c_int,
    pub bp_oldreq: nilfs_bmap_ptr_req,
    pub bp_newreq: nilfs_bmap_ptr_req,
    pub bp_ctxt: nilfs_btnode_chkey_ctxt,
    pub bp_op: Option<unsafe extern "C" fn(
        *mut nilfs_bmap,
        *mut nilfs_btree_path,
        core::ffi::c_int,
        *mut __u64,
        *mut __u64,
    )>,
}

pub const NILFS_BTREE_ROOT_SIZE: usize = NILFS_BMAP_SIZE;
pub const NILFS_BTREE_ROOT_NCHILDREN_MAX: usize =
    (NILFS_BTREE_ROOT_SIZE - core::mem::size_of::<nilfs_btree_node>())
        / (core::mem::size_of::<__le64>() + core::mem::size_of::<__le64>());
pub const NILFS_BTREE_ROOT_NCHILDREN_MIN: usize = 0;
pub const NILFS_BTREE_NODE_EXTRA_PAD_SIZE: usize = core::mem::size_of::<__le64>();

#[inline]
pub const fn NILFS_BTREE_NODE_NCHILDREN_MAX(nodesize: usize) -> usize {
    (nodesize
        - core::mem::size_of::<nilfs_btree_node>()
        - NILFS_BTREE_NODE_EXTRA_PAD_SIZE)
        / (core::mem::size_of::<__le64>() + core::mem::size_of::<__le64>())
}

#[inline]
pub const fn NILFS_BTREE_NODE_NCHILDREN_MIN(nodesize: usize) -> usize {
    (NILFS_BTREE_NODE_NCHILDREN_MAX(nodesize) - 1) / 2 + 1
}

pub const NILFS_BTREE_KEY_MIN: __u64 = 0;
pub const NILFS_BTREE_KEY_MAX: __u64 = !0;

extern "C" {
    pub static mut nilfs_btree_path_cache: *mut kmem_cache;

    pub fn nilfs_btree_init(bmap: *mut nilfs_bmap) -> core::ffi::c_int;
    pub fn nilfs_btree_convert_and_insert(
        bmap: *mut nilfs_bmap,
        key: __u64,
        ptr: __u64,
        keys: *const __u64,
        ptrs: *const __u64,
        n: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn nilfs_btree_init_gc(bmap: *mut nilfs_bmap);
    pub fn nilfs_btree_broken_node_block(bh: *mut buffer_head) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
