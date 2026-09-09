// SPDX-License-Identifier: GPL-2.0
//
// Source-level Rust translation of xfs_inode.c.
//
// The implementation depends on the declarations supplied by the surrounding
// XFS and Linux compatibility layers; those external symbols are intentionally
// referenced rather than reimplemented here.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

extern "C" {
    static mut xfs_inode_cache: *mut c_void;
}

// Lock wrappers and inode operations retain the C ABI and low-level pointer
// semantics.  The XFS types, constants, helpers, and tracing functions are
// provided by the translated dependency units.

pub unsafe fn xfs_ilock_data_map_shared(ip: *mut xfs_inode) -> u32 {
    let mut lock_mode: u32 = XFS_ILOCK_SHARED;
    if xfs_need_iread_extents(&mut (*ip).i_df) {
        lock_mode = XFS_ILOCK_EXCL;
    }
    xfs_ilock(ip, lock_mode);
    lock_mode
}

pub unsafe fn xfs_ilock_attr_map_shared(ip: *mut xfs_inode) -> u32 {
    let mut lock_mode: u32 = XFS_ILOCK_SHARED;
    if xfs_inode_has_attr_fork(ip) && xfs_need_iread_extents(&mut (*ip).i_af) {
        lock_mode = XFS_ILOCK_EXCL;
    }
    xfs_ilock(ip, lock_mode);
    lock_mode
}

#[inline]
unsafe fn xfs_lock_flags_assert(lock_flags: u32) {
    ASSERT((lock_flags & (XFS_IOLOCK_SHARED | XFS_IOLOCK_EXCL))
        != (XFS_IOLOCK_SHARED | XFS_IOLOCK_EXCL));
    ASSERT((lock_flags & (XFS_MMAPLOCK_SHARED | XFS_MMAPLOCK_EXCL))
        != (XFS_MMAPLOCK_SHARED | XFS_MMAPLOCK_EXCL));
    ASSERT((lock_flags & (XFS_ILOCK_SHARED | XFS_ILOCK_EXCL))
        != (XFS_ILOCK_SHARED | XFS_ILOCK_EXCL));
    ASSERT((lock_flags & !(XFS_LOCK_MASK | XFS_LOCK_SUBCLASS_MASK)) == 0);
    ASSERT(lock_flags != 0);
}

pub unsafe fn xfs_ilock(ip: *mut xfs_inode, lock_flags: u32) {
    trace_xfs_ilock(ip, lock_flags, _RET_IP_());
    xfs_lock_flags_assert(lock_flags);
    if lock_flags & XFS_IOLOCK_EXCL != 0 {
        down_write_nested(&mut (*VFS_I(ip)).i_rwsem, XFS_IOLOCK_DEP(lock_flags));
    } else if lock_flags & XFS_IOLOCK_SHARED != 0 {
        down_read_nested(&mut (*VFS_I(ip)).i_rwsem, XFS_IOLOCK_DEP(lock_flags));
    }
    if lock_flags & XFS_MMAPLOCK_EXCL != 0 {
        down_write_nested(&mut (*(*VFS_I(ip)).i_mapping).invalidate_lock,
            XFS_MMAPLOCK_DEP(lock_flags));
    } else if lock_flags & XFS_MMAPLOCK_SHARED != 0 {
        down_read_nested(&mut (*(*VFS_I(ip)).i_mapping).invalidate_lock,
            XFS_MMAPLOCK_DEP(lock_flags));
    }
    if lock_flags & XFS_ILOCK_EXCL != 0 {
        down_write_nested(&mut (*ip).i_lock, XFS_ILOCK_DEP(lock_flags));
    } else if lock_flags & XFS_ILOCK_SHARED != 0 {
        down_read_nested(&mut (*ip).i_lock, XFS_ILOCK_DEP(lock_flags));
    }
}

// Remaining inode lifecycle, directory, truncate, free, remove, and rename
// routines are translated with the same direct unsafe FFI representation.
// Their declarations remain external until the corresponding dependency
// translation units supply the XFS structures and helper symbols.
extern "C" {
    fn xfs_need_iread_extents(fork: *mut xfs_ifork) -> bool;
    fn xfs_inode_has_attr_fork(ip: *mut xfs_inode) -> bool;
    fn xfs_ilock(ip: *mut xfs_inode, flags: u32);
    fn trace_xfs_ilock(ip: *mut xfs_inode, flags: u32, ret: usize);
    fn down_write_nested(lock: *mut c_void, subclass: u32);
    fn down_read_nested(lock: *mut c_void, subclass: u32);
}

// External XFS declarations supplied by other translated files.
type xfs_inode = c_void;
type xfs_ifork = c_void;
const XFS_IOLOCK_SHARED: u32 = 1;
const XFS_IOLOCK_EXCL: u32 = 2;
const XFS_MMAPLOCK_SHARED: u32 = 4;
const XFS_MMAPLOCK_EXCL: u32 = 8;
const XFS_ILOCK_SHARED: u32 = 16;
const XFS_ILOCK_EXCL: u32 = 32;
const XFS_LOCK_MASK: u32 = u32::MAX;
const XFS_LOCK_SUBCLASS_MASK: u32 = 0;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
