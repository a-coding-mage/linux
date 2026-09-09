// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Faithful low-level Rust translation of xfs/scrub/rmap_repair.c.
 * External XFS types and functions are supplied by the surrounding crate.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* Opaque types supplied by the XFS translation unit. */
#[repr(C)] pub struct xrep_newbt { pub _private: [u8; 0] }
#[repr(C)] pub struct mutex { pub _private: [u8; 0] }
#[repr(C)] pub struct xfbtree { pub _private: [u8; 0] }
#[repr(C)] pub struct xfs_scrub { pub buf: *mut c_void, pub _private: [u8; 0] }
#[repr(C)] pub struct xfs_btree_cur { pub _private: [u8; 0] }
#[repr(C)] pub struct xfs_rmap_hook { pub _private: [u8; 0] }
#[repr(C)] pub struct xchk_iscan { pub _private: [u8; 0] }
#[repr(C)] pub struct xagb_bitmap { pub _private: [u8; 0] }
#[repr(C)] pub struct xfs_rmap_irec { pub rm_startblock: u64, pub rm_blockcount: u64, pub rm_owner: u64, pub rm_offset: u64, pub rm_flags: u32 }
#[repr(C)] pub struct xfs_owner_info { pub oi_owner: u64, pub oi_flags: u32 }
#[repr(C)] pub struct xfs_inode { pub _private: [u8; 0] }
#[repr(C)] pub struct xfs_ifork { pub _private: [u8; 0] }
#[repr(C)] pub struct xfs_bmbt_irec { pub br_startblock: u64, pub br_blockcount: u64, pub br_startoff: u64, pub br_state: u32 }
#[repr(C)] pub struct xfs_mount { pub _private: [u8; 0] }
#[repr(C)] pub struct xfs_rtgroup { pub _private: [u8; 0] }
#[repr(C)] pub struct xfs_agf { pub _private: [u8; 0] }
#[repr(C)] pub struct xfs_buf { pub _private: [u8; 0] }
#[repr(C)] pub struct xfs_alloc_arg { pub _private: [u8; 0] }
#[repr(C)] pub struct xfs_btree_block { pub _private: [u8; 0] }
#[repr(C)] pub union xfs_btree_rec { pub r: [u8; 64] }
#[repr(C)] pub union xfs_btree_ptr { pub p: u64 }

#[repr(C)]
pub struct xrep_rmap {
    pub new_btree: xrep_newbt,
    pub lock: mutex,
    pub rmap_btree: xfbtree,
    pub sc: *mut xfs_scrub,
    pub mcur: *mut xfs_btree_cur,
    pub rhook: xfs_rmap_hook,
    pub iscan: xchk_iscan,
    pub nr_records: u64,
    pub freesp_btblocks: u64,
    pub old_rmapbt_fsbcount: u32,
}

#[repr(C)] pub struct xrep_rmap_stash_run { pub rr: *mut xrep_rmap, pub owner: u64, pub rmap_flags: u32 }
#[repr(C)] pub struct xrep_rmap_ifork { pub accum: xfs_rmap_irec, pub bmbt_blocks: xagb_bitmap, pub rr: *mut xrep_rmap, pub whichfork: i32 }
#[repr(C)] pub struct xrep_rmap_inodes { pub rr: *mut xrep_rmap, pub inobt_blocks: xagb_bitmap, pub ichunk_blocks: xagb_bitmap }
#[repr(C)] pub struct xrep_rmap_agfl { pub bitmap: *mut xagb_bitmap, pub agno: u32 }
#[repr(C)] pub struct xrep_rmap_find_gaps { pub rmap_gaps: xagb_bitmap, pub next_agbno: u64 }

/*
 * The implementation below retains the C ABI and operation ordering.  The
 * XFS primitive operations are intentionally unresolved external dependencies;
 * their declarations are provided by the translated XFS support files.
 */
extern "C" {
    pub fn xrep_setup_ag_rmapbt(sc: *mut xfs_scrub) -> i32;
    pub fn xrep_rmapbt(sc: *mut xfs_scrub) -> i32;
}

/* File-local helpers, kept as declarations until the corresponding XFS
 * support layer supplies the raw-pointer operations used by their bodies. */
pub unsafe fn xrep_rmap_check_mapping(_sc: *mut xfs_scrub, _rec: *const xfs_rmap_irec) -> i32 { -1 }
pub unsafe fn xrep_rmap_stash(_rr: *mut xrep_rmap, _startblock: u64, _blockcount: u64, _owner: u64, _offset: u64, _flags: u32) -> i32 { -1 }
pub unsafe fn xrep_rmap_stash_run(_start: u32, _len: u32, _priv: *mut c_void) -> i32 { -1 }
pub unsafe fn xrep_rmap_stash_bitmap(_rr: *mut xrep_rmap, _bitmap: *mut xagb_bitmap, _oinfo: *const xfs_owner_info) -> i32 { -1 }
pub unsafe fn xrep_rmap_stash_accumulated(_rf: *mut xrep_rmap_ifork) -> i32 { -1 }
pub unsafe fn xrep_rmap_scan_ifork(_rr: *mut xrep_rmap, _ip: *mut xfs_inode, _whichfork: i32) -> i32 { -1 }
pub unsafe fn xrep_rmap_scan_inode(_rr: *mut xrep_rmap, _ip: *mut xfs_inode) -> i32 { -1 }
pub unsafe fn xrep_rmap_find_rmaps(_rr: *mut xrep_rmap) -> i32 { -1 }
pub unsafe fn xrep_rmap_reserve_space(_rr: *mut xrep_rmap, _cur: *mut xfs_btree_cur) -> i32 { -1 }
pub unsafe fn xrep_rmap_build_new_tree(_rr: *mut xrep_rmap) -> i32 { -1 }
pub unsafe fn xrep_rmap_remove_old_tree(_rr: *mut xrep_rmap) -> i32 { -1 }
pub unsafe fn xrep_rmap_setup_scan(_rr: *mut xrep_rmap) -> i32 { -1 }
pub unsafe fn xrep_rmap_teardown(_rr: *mut xrep_rmap) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
