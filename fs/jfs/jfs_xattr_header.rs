/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2002
 */

// Translated from jfs_xattr.h. The Linux xattr dependency and the external
// kernel types and declarations are supplied by other translation units.

/*
 * jfs_ea_list describe the on-disk format of the extended attributes.
 * I know the null-terminator is redundant since namelen is stored, but
 * I am maintaining compatibility with OS/2 where possible.
 */
#[repr(C)]
pub struct jfs_ea {
    pub flag: u8,       /* Unused? */
    pub namelen: u8,    /* Length of name */
    pub valuelen: u16, /* Length of value */
    pub name: [i8; 0],  /* Attribute name (includes null-terminator) */
} /* Value immediately follows name */

#[repr(C)]
pub struct jfs_ea_list {
    pub size: u32,      /* overall size */
    pub ea: [jfs_ea; 0], /* Variable length list */
}

/* Macros for defining maximum number of bytes supported for EAs */
pub const MAXEASIZE: usize = 65535;
pub const MAXEALISTSIZE: usize = MAXEASIZE;

/*
 * Some functions below correspond to the variable-length EA-list macros in
 * the C header. Little-endian conversion is represented by to_le semantics.
 */
#[inline]
pub unsafe fn EA_SIZE(ea: *const jfs_ea) -> usize {
    core::mem::size_of::<jfs_ea>()
        .wrapping_add((*ea).namelen as usize)
        .wrapping_add(1)
        .wrapping_add(u16::from_le((*ea).valuelen) as usize)
}

#[inline]
pub unsafe fn NEXT_EA(ea: *mut jfs_ea) -> *mut jfs_ea {
    (ea as *mut u8).add(EA_SIZE(ea as *const jfs_ea)) as *mut jfs_ea
}

#[inline]
pub unsafe fn FIRST_EA(ealist: *mut jfs_ea_list) -> *mut jfs_ea {
    (*ealist).ea.as_mut_ptr()
}

#[inline]
pub unsafe fn EALIST_SIZE(ealist: *const jfs_ea_list) -> u32 {
    u32::from_le((*ealist).size)
}

#[inline]
pub unsafe fn END_EALIST(ealist: *mut jfs_ea_list) -> *mut jfs_ea {
    (ealist as *mut u8).add(EALIST_SIZE(ealist as *const jfs_ea_list) as usize)
        as *mut jfs_ea
}

extern "C" {
    pub fn __jfs_setxattr(
        tid: tid_t,
        inode: *mut inode,
        name: *const i8,
        value: *const core::ffi::c_void,
        size: usize,
        flags: i32,
    ) -> i32;
    pub fn __jfs_getxattr(
        inode: *mut inode,
        name: *const i8,
        value: *mut core::ffi::c_void,
        size: usize,
    ) -> isize;
    pub fn jfs_listxattr(dentry: *mut dentry, data: *mut i8, size: usize) -> isize;

    pub static jfs_xattr_handlers: *const *const xattr_handler;
}

#[cfg(feature = "CONFIG_JFS_SECURITY")]
extern "C" {
    pub fn jfs_init_security(
        tid: tid_t,
        inode: *mut inode,
        dir: *mut inode,
        qstr: *const qstr,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_JFS_SECURITY"))]
#[inline]
pub unsafe fn jfs_init_security(
    _tid: tid_t,
    _inode: *mut inode,
    _dir: *mut inode,
    _qstr: *const qstr,
) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
