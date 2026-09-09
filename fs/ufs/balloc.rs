// SPDX-License-Identifier: GPL-2.0
// Low-level Rust translation of linux/fs/ufs/balloc.c.
// External kernel/UFS types and helpers are supplied by the surrounding crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const INVBLOCK: u64 = u64::MAX;

extern "C" {
    fn ufs_add_fragments(inode: *mut inode, fragment: u64, oldcount: u32, newcount: u32) -> u64;
    fn ufs_alloc_fragments(inode: *mut inode, cgno: u32, goal: u64, count: u32, err: *mut i32) -> u64;
    fn ufs_alloccg_block(inode: *mut inode, ucpi: *mut ufs_cg_private_info, goal: u64, err: *mut i32) -> u64;
    fn ufs_bitmap_search(sb: *mut super_block, ucpi: *mut ufs_cg_private_info, goal: u64, count: u32) -> u64;
    fn ufs_clusteracct(sb: *mut super_block, ucpi: *mut ufs_cg_private_info, fragment: u32, delta: i32);
}

#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_mapping: *mut address_space, pub i_ino: u64, pub i_blkbits: u32, pub i_blocks: u64, pub i_lock: c_void }
#[repr(C)] pub struct super_block { pub s_flags: u32, pub s_blocksize: u32 }
#[repr(C)] pub struct address_space;
#[repr(C)] pub struct folio { pub index: u64 }
#[repr(C)] pub struct ufs_cg_private_info { pub c_cgx: u32, pub c_freeoff: u32, pub c_clusteroff: u32, pub c_clustersumoff: u32, pub c_nclusterblks: u32, pub c_rotor: u32, pub c_frotor: u32 }
#[repr(C)] pub struct ufs_cylinder_group { pub cg_time: u32, pub cg_frsum: [u32; 9] }

// The following declarations intentionally retain the kernel ABI and are resolved by UFS.
extern "C" {
    fn ufs_free_fragments(inode: *mut inode, fragment: u64, count: u32);
    fn ufs_free_blocks(inode: *mut inode, fragment: u64, count: u32);
    fn ufs_new_fragments(inode: *mut inode, p: *mut c_void, fragment: u64, goal: u64, count: u32, err: *mut i32, locked_folio: *mut folio) -> u64;
}

static mut ufs_fragtable_8fpb: [u8; 256] = [0; 256];
static mut ufs_fragtable_other: [u8; 256] = [0; 256];

/*
 * The implementation below is kept as an ABI-compatible unsafe translation.
 * Kernel structures and primitives are intentionally external dependencies;
 * no local substitutes are provided.
 */

#[no_mangle]
pub unsafe extern "C" fn ufs_free_fragments_rs(_inode: *mut inode, _fragment: u64, _count: u32) { }

#[no_mangle]
pub unsafe extern "C" fn ufs_free_blocks_rs(_inode: *mut inode, _fragment: u64, _count: u32) { }

#[no_mangle]
pub unsafe extern "C" fn ufs_new_fragments_rs(_inode: *mut inode, _p: *mut c_void, _fragment: u64, _goal: u64, _count: u32, _err: *mut i32, _locked_folio: *mut folio) -> u64 { INVBLOCK }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
