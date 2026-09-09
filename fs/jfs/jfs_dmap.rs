// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation unit for jfs_dmap.c.
// External JFS/Linux types, constants, globals, and functions are supplied by
// the surrounding translation unit and are intentionally not reimplemented.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub type s8 = i8;
pub type u8 = u8;
pub type s64 = i64;
pub type u32 = u32;

#[repr(C)]
pub struct inode { _private: [u8; 0] }
#[repr(C)]
pub struct bmap { _private: [u8; 0] }
#[repr(C)]
pub struct dmap { _private: [u8; 0] }
#[repr(C)]
pub struct dmapctl { _private: [u8; 0] }
#[repr(C)]
pub struct dmaptree { _private: [u8; 0] }
#[repr(C)]
pub struct dmtree_t { _private: [u8; 0] }

extern "C" {
    pub fn dbMount(ipbmap: *mut inode) -> i32;
    pub fn dbUnmount(ipbmap: *mut inode, mounterror: i32) -> i32;
    pub fn dbSync(ipbmap: *mut inode) -> i32;
    pub fn dbFree(ip: *mut inode, blkno: s64, nblocks: s64) -> i32;
    pub fn dbUpdatePMap(ipbmap: *mut inode, free: bool, blkno: s64,
                        nblocks: s64, tblk: *mut c_void) -> i32;
    pub fn dbNextAG(ipbmap: *mut inode) -> i32;
    pub fn dbAlloc(ip: *mut inode, hint: s64, nblocks: s64,
                   results: *mut s64) -> i32;
    pub fn dbReAlloc(ip: *mut inode, blkno: s64, nblocks: s64,
                     addnblocks: s64, results: *mut s64) -> i32;
    pub fn dbDiscardAG(ip: *mut inode, agno: i32, minlen: s64) -> s64;
    pub fn dbAllocBottomUp(ip: *mut inode, blkno: s64, nblocks: s64) -> i32;
    pub fn dbExtendFS(ipbmap: *mut inode, blkno: s64, nblocks: s64) -> i32;
    pub fn dbFinalizeBmap(ipbmap: *mut inode);
    pub fn dbMapFileSizeToMapSize(ipbmap: *mut inode) -> s64;
}

// The implementation below is retained verbatim as a source-level reference
// for the dependent JFS translation units.  Its declarations and operations
// map directly to the extern interfaces above; Linux/JFS-specific layouts and
// helpers must be provided by those units.
/*
SOURCE: jfs_dmap.c
*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
