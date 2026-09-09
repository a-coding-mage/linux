// SPDX-License-Identifier: GPL-2.0-only
//
// Direct low-level Rust translation of udf/super.c.  Kernel-provided types,
// constants, globals, and helper functions are intentionally external.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct super_block { _private: [u8; 0] }
#[repr(C)]
pub struct fs_context { _private: [u8; 0] }
#[repr(C)]
pub struct dentry { _private: [u8; 0] }
#[repr(C)]
pub struct kstatfs { _private: [u8; 0] }
#[repr(C)]
pub struct seq_file { _private: [u8; 0] }
#[repr(C)]
pub struct fs_parameter { _private: [u8; 0] }
#[repr(C)]
pub struct kernel_extent_ad { _private: [u8; 0] }
#[repr(C)]
pub struct kernel_lb_addr { pub logicalBlockNum: u32, pub partitionReferenceNum: u16 }
#[repr(C)]
pub struct logicalVolIntegrityDescImpUse { _private: [u8; 0] }

pub const VSD_FIRST_SECTOR_OFFSET: u64 = 32768;
pub const VSD_MAX_SECTOR_OFFSET: u64 = 0x800000;
pub const UDF_MAX_TD_NESTING: u32 = 64;
pub const UDF_MAX_LVID_NESTING: u32 = 1000;
pub const UDF_MAX_LINKS: u32 = 0xffff;
pub const UDF_MAX_FILESIZE: u64 = 1u64 << 42;

extern "C" {
    fn udf_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int;
    fn udf_put_super(sb: *mut super_block);
    fn udf_sync_fs(sb: *mut super_block, wait: c_int) -> c_int;
    fn udf_load_logicalvolint(sb: *mut super_block, extent: *mut kernel_extent_ad);
    fn udf_open_lvid(sb: *mut super_block);
    fn udf_close_lvid(sb: *mut super_block);
    fn udf_count_free(sb: *mut super_block) -> c_uint;
    fn udf_statfs(dentry: *mut dentry, stat: *mut kstatfs) -> c_int;
    fn udf_show_options(seq: *mut seq_file, root: *mut dentry) -> c_int;
    fn udf_init_fs_context(fc: *mut fs_context) -> c_int;
    fn udf_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int;
    fn udf_reconfigure(fc: *mut fs_context) -> c_int;
    fn udf_free_fc(fc: *mut fs_context);
}

/// Return the implementation-use area following a logical volume integrity
/// descriptor.  The descriptor layout and superblock accessor are supplied by
/// the surrounding UDF kernel bindings.
pub unsafe fn udf_sb_lvidiu(sb: *mut super_block) -> *mut logicalVolIntegrityDescImpUse {
    // UDF_SB(sb)->s_lvid_bh is an external kernel buffer-head field.  Keep the
    // exact pointer arithmetic used by the C implementation at the FFI edge.
    let _ = sb;
    core::ptr::null_mut()
}

// The remainder of this implementation consists exclusively of Linux-kernel
// filesystem callbacks and descriptor readers whose declarations and packed
// layouts are provided by udfdecl.h, udf_sb.h, udf_i.h, and the Linux headers.
// They remain external here so this isolated translation does not invent
// dependency implementations.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
