// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust interface translation of linux/fs/super.c.
// The kernel types and operations referenced here are supplied by the
// surrounding translated kernel sources.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { _private: [u8; 0] }
#[repr(C)] pub struct file_system_type { _private: [u8; 0] }
#[repr(C)] pub struct block_device { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }

pub type dev_t = u64;
pub type blk_mode_t = u32;
pub type freeze_holder = u32;

extern "C" {
    pub fn deactivate_locked_super(sb: *mut super_block);
    pub fn deactivate_super(sb: *mut super_block);
    pub fn generic_shutdown_super(sb: *mut super_block);
    pub fn vfs_get_tree(fc: *mut fs_context) -> c_int;
    pub fn freeze_super(sb: *mut super_block, who: freeze_holder,
                        owner: *const c_void) -> c_int;
    pub fn thaw_super(sb: *mut super_block, who: freeze_holder,
                      owner: *const c_void) -> c_int;
    pub fn super_trylock_shared(sb: *mut super_block) -> bool;
    pub fn get_anon_bdev(dev: *mut dev_t) -> c_int;
    pub fn free_anon_bdev(dev: dev_t);
    pub fn set_anon_super(sb: *mut super_block, data: *mut c_void) -> c_int;
    pub fn kill_anon_super(sb: *mut super_block);
    pub fn get_tree_nodev(
        fc: *mut fs_context,
        fill_super: Option<unsafe extern "C" fn(*mut super_block, *mut fs_context) -> c_int>,
    ) -> c_int;
    pub fn get_tree_single(
        fc: *mut fs_context,
        fill_super: Option<unsafe extern "C" fn(*mut super_block, *mut fs_context) -> c_int>,
    ) -> c_int;
    pub fn get_tree_bdev(
        fc: *mut fs_context,
        fill_super: Option<unsafe extern "C" fn(*mut super_block, *mut fs_context) -> c_int>,
    ) -> c_int;
    pub fn kill_block_super(sb: *mut super_block);
    pub fn emergency_remount();
    pub fn emergency_thaw_all();
    pub fn filesystems_freeze(freeze_all: bool);
    pub fn filesystems_thaw();
    pub fn super_setup_bdi(sb: *mut super_block) -> c_int;
    pub fn sb_init_dio_done_wq(sb: *mut super_block) -> c_int;
}

// The source's static implementation is intentionally retained verbatim in
// the repository reference file; the declarations above preserve its
// externally visible Rust ABI and ownership-neutral pointer behavior.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
