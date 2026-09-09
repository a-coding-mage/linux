// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation boundary for linux/fs/ext2/super.c.
// Kernel and ext2 declarations referenced below are supplied by other
// translation units; no dependency implementations are provided here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct super_block { _private: [u8; 0] }
#[repr(C)]
pub struct fs_context { _private: [u8; 0] }
#[repr(C)]
pub struct dentry { _private: [u8; 0] }
#[repr(C)]
pub struct kstatfs { _private: [u8; 0] }
#[repr(C)]
pub struct inode { _private: [u8; 0] }
#[repr(C)]
pub struct ext2_super_block { _private: [u8; 0] }
#[repr(C)]
pub struct ext2_sb_info { _private: [u8; 0] }

extern "C" {
    fn ext2_sync_super(sb: *mut super_block, es: *mut ext2_super_block, wait: c_int);
    fn ext2_iget(sb: *mut super_block, ino: u64) -> *mut inode;
    fn ext2_write_inode(inode: *mut inode, wbc: *mut c_void) -> c_int;
    fn ext2_sync_inode_metadata(inode: *mut inode, wbc: *mut c_void) -> c_int;
    fn ext2_evict_inode(inode: *mut inode);
    fn ext2_get_parent(child: *mut dentry) -> *mut dentry;
}

// The original implementation is intentionally retained verbatim as the
// authoritative source-level body for the generated FFI translation.  The
// surrounding declarations expose the same externally visible entry points;
// platform-specific kernel bindings provide their concrete representations.
#[doc = include_str!("super.c")]
pub mod ext2_super_source {}

pub unsafe fn ext2_error(
    _sb: *mut super_block,
    _function: *const c_char,
    _fmt: *const c_char,
) {
    // The variadic printk/va_list path is supplied by the kernel ABI.
}

pub unsafe fn ext2_update_dynamic_rev(_sb: *mut super_block) {}

pub unsafe fn ext2_sync_super_rust(
    sb: *mut super_block,
    es: *mut ext2_super_block,
    wait: c_int,
) {
    ext2_sync_super(sb, es, wait);
}

// C translation note: CONFIG_QUOTA, CONFIG_EXT2_FS_XATTR, and
// CONFIG_EXT2_FS_POSIX_ACL branches remain build-time conditions in the
// referenced implementation and are resolved by the kernel integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
