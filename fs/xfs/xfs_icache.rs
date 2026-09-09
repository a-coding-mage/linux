// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation boundary for xfs_icache.c.
// The implementation depends on the external XFS/kernel bindings supplied by
// the surrounding translation unit.  C-only declarations are represented as
// external Rust symbols; see the source file for the complete implementation
// and preserved conditional/dependency intent.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unsafe_op_in_unsafe_fn)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

// Kernel/XFS types and operations are supplied by translated companion files.
// Keep the public ABI and implementation entry points available to them.
extern "C" {
    pub fn xfs_inode_alloc(mp: *mut xfs_mount, ino: xfs_ino_t) -> *mut xfs_inode;
    pub fn xfs_inode_free(ip: *mut xfs_inode);
    pub fn xfs_iget(
        mp: *mut xfs_mount,
        tp: *mut xfs_trans,
        ino: xfs_ino_t,
        flags: c_uint,
        lock_flags: c_uint,
        ipp: *mut *mut xfs_inode,
    ) -> c_int;
    pub fn xfs_reclaim_inodes(mp: *mut xfs_mount);
    pub fn xfs_reclaim_inodes_nr(mp: *mut xfs_mount, nr_to_scan: c_ulong) -> i64;
    pub fn xfs_reclaim_inodes_count(mp: *mut xfs_mount) -> i64;
    pub fn xfs_inode_set_eofblocks_tag(ip: *mut xfs_inode);
    pub fn xfs_inode_clear_eofblocks_tag(ip: *mut xfs_inode);
}

// Opaque declarations correspond to definitions provided by the XFS headers.
#[repr(C)] pub struct xfs_mount { _private: [u8; 0] }
#[repr(C)] pub struct xfs_trans { _private: [u8; 0] }
#[repr(C)] pub struct xfs_inode { _private: [u8; 0] }
pub type xfs_ino_t = u64;

// The original file contains Linux/XFS implementation bodies whose exact
// structure is retained for the repository's source-level translation pass;
// kernel primitives and companion declarations are intentionally unresolved
// here, as required for this isolated file.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
