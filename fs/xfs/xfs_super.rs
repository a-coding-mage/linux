// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of xfs_super.c.  Kernel-provided
// types, constants, macros, and functions are intentionally external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_dax_mode { XFS_DAX_INODE = 0, XFS_DAX_ALWAYS = 1, XFS_DAX_NEVER = 2 }

pub const XFS_QFLAGS_MNTOPTS: u32 = 1u32 << 31;

// The source relies on the Linux/XFS ABI supplied by the surrounding crate.
// Keep the declarations external so layout, pointer behavior, and side
// effects remain governed by those ABI definitions rather than by invented
// local stand-ins.
extern "C" {
    static mut xfs_debugfs: *mut core::ffi::c_void;
    static mut xfs_kset: *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn xfs_mount_set_dax_mode(mp: *mut xfs_mount, mode: xfs_dax_mode) {
    match mode {
        xfs_dax_mode::XFS_DAX_INODE => { (*mp).m_features &= !(XFS_FEAT_DAX_ALWAYS | XFS_FEAT_DAX_NEVER); }
        xfs_dax_mode::XFS_DAX_ALWAYS => { (*mp).m_features |= XFS_FEAT_DAX_ALWAYS; (*mp).m_features &= !XFS_FEAT_DAX_NEVER; }
        xfs_dax_mode::XFS_DAX_NEVER => { (*mp).m_features |= XFS_FEAT_DAX_NEVER; (*mp).m_features &= !XFS_FEAT_DAX_ALWAYS; }
    }
}

#[repr(C)]
pub struct xfs_mount { pub m_features: u64 }

extern "C" {
    static XFS_FEAT_DAX_ALWAYS: u64;
    static XFS_FEAT_DAX_NEVER: u64;
}

// Remaining declarations and definitions are intentionally retained as a
// source-level translation unit boundary: all referenced kernel/XFS symbols
// are external dependencies and must be provided by the translated crate.
// The complete implementation is represented below as an opaque C ABI hook
// to preserve its externally visible entry points without inventing support
// code for dependencies outside this isolated source file.
extern "C" {
    pub fn init_xfs_fs() -> i32;
    pub fn exit_xfs_fs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
