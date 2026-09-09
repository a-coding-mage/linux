/* SPDX-License-Identifier: GPL-2.0 */
/*
 *
 * Copyright (C) 2019-2021 Paragon Software GmbH, All rights reserved.
 *
 * Useful functions for debugging.
 *
 */

// C header guard: _LINUX_NTFS3_DEBUG_H

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn Add2Ptr<P>(p: P, i: usize) -> *mut c_void
where
    P: Into<*mut c_void>,
{
    (p.into() as *mut u8).add(i) as *mut c_void
}

#[inline]
pub unsafe fn PtrOffset<B, O>(b: B, o: O) -> usize
where
    B: Into<usize>,
    O: Into<usize>,
{
    o.into().wrapping_sub(b.into())
}

// CONFIG_PRINTK controls whether these declarations or the empty inline
// implementations are used in the original C header.
#[cfg(CONFIG_PRINTK)]
unsafe extern "C" {
    pub fn ntfs_printk(sb: *const super_block, fmt: *const c_char, ...);
    pub fn ntfs_inode_printk(inode: *mut inode, fmt: *const c_char, ...);
}

#[cfg(not(CONFIG_PRINTK))]
pub unsafe extern "C" fn ntfs_printk(
    _sb: *const super_block,
    _fmt: *const c_char,
    ...
) {
}

#[cfg(not(CONFIG_PRINTK))]
pub unsafe extern "C" fn ntfs_inode_printk(
    _inode: *mut inode,
    _fmt: *const c_char,
    ...
) {
}

/*
 * Logging macros. Thanks Joe Perches <joe@perches.com> for implementation.
 */

#[macro_export]
macro_rules! ntfs_err {
    ($sb:expr, $fmt:expr $(, $args:expr)*) => {
        $crate::ntfs_printk($sb, concat!(KERN_ERR, $fmt) $(, $args)*)
    };
}

#[macro_export]
macro_rules! ntfs_warn {
    ($sb:expr, $fmt:expr $(, $args:expr)*) => {
        $crate::ntfs_printk($sb, concat!(KERN_WARNING, $fmt) $(, $args)*)
    };
}

#[macro_export]
macro_rules! ntfs_info {
    ($sb:expr, $fmt:expr $(, $args:expr)*) => {
        $crate::ntfs_printk($sb, concat!(KERN_INFO, $fmt) $(, $args)*)
    };
}

#[macro_export]
macro_rules! ntfs_notice {
    ($sb:expr, $fmt:expr $(, $args:expr)*) => {
        $crate::ntfs_printk($sb, concat!(KERN_NOTICE, $fmt) $(, $args)*)
    };
}

#[macro_export]
macro_rules! ntfs_inode_err {
    ($inode:expr, $fmt:expr $(, $args:expr)*) => {
        $crate::ntfs_inode_printk($inode, concat!(KERN_ERR, $fmt) $(, $args)*)
    };
}

#[macro_export]
macro_rules! ntfs_inode_warn {
    ($inode:expr, $fmt:expr $(, $args:expr)*) => {
        $crate::ntfs_inode_printk($inode, concat!(KERN_WARNING, $fmt) $(, $args)*)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
