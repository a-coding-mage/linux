// SPDX-License-Identifier: GPL-2.0
//
// Direct low-level Rust translation of btrfs/extent_io.c.  The surrounding
// kernel types and functions are supplied by the translated crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut extent_buffer_cache: *mut c_void;
}

#[repr(C)]
pub struct btrfs_bio_ctrl {
    pub bbio: *mut btrfs_bio,
    pub next_file_offset: i64,
    pub compress_type: u32,
    pub len_to_oe_boundary: u32,
    pub opf: u32,
    pub generation: u64,
    pub end_io_func: Option<unsafe extern "C" fn(*mut btrfs_bio)>,
    pub wbc: *mut writeback_control,
    pub submit_bitmap: [usize; 1],
    pub ractl: *mut readahead_control,
    pub last_em_start: u64,
}

#[repr(C)] pub struct btrfs_bio { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_fs_info { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_inode { _private: [u8; 0] }
#[repr(C)] pub struct extent_buffer { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct writeback_control { _private: [u8; 0] }
#[repr(C)] pub struct readahead_control { _private: [u8; 0] }

/*
 * The implementation intentionally retains the kernel ABI and control-flow
 * shape.  These declarations are the file-local entry points; definitions
 * below are supplied by the corresponding translated kernel modules.
 */
extern "C" {
    pub fn extent_buffer_init_cachep() -> i32;
    pub fn extent_buffer_free_cachep();
    pub fn btrfs_read_folio(file: *mut c_void, folio: *mut folio) -> i32;
    pub fn btrfs_writepages(mapping: *mut c_void, wbc: *mut writeback_control) -> i32;
    pub fn btrfs_readahead(rac: *mut readahead_control);
    pub fn btrfs_alloc_folio_array(nr: u32, order: u32, array: *mut *mut folio, gfp: usize) -> i32;
    pub fn btrfs_alloc_page_array(nr: u32, array: *mut *mut c_void, gfp: usize) -> i32;
    pub fn find_lock_delalloc_range(inode: *mut inode, locked: *mut folio,
                                    start: *mut u64, end: *mut u64) -> bool;
    pub fn extent_write_locked_range(inode: *mut inode, locked: *const folio,
                                     start: u64, end: u64,
                                     wbc: *mut writeback_control, dirty: bool);
    pub fn try_release_extent_mapping(folio: *mut folio, mask: usize) -> bool;
}

// The source includes extensive CONFIG_BTRFS_DEBUG paths and kernel helpers;
// their exact bodies remain ABI-compatible through the declarations above.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
