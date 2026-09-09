/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * decompressor.h
 */

use core::ffi::{c_char, c_int, c_void};

/* Supplied by the corresponding translated dependencies. */
#[repr(C)]
pub struct squashfs_sb_info {
    pub decompressor: *mut squashfs_decompressor,
}

#[repr(C)]
pub struct bio {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct squashfs_page_actor {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct squashfs_decompressor {
    pub init: Option<unsafe extern "C" fn(*mut squashfs_sb_info, *mut c_void) -> *mut c_void>,
    pub comp_opts:
        Option<unsafe extern "C" fn(*mut squashfs_sb_info, *mut c_void, c_int) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub decompress: Option<unsafe extern "C" fn(
        *mut squashfs_sb_info,
        *mut c_void,
        *mut bio,
        c_int,
        c_int,
        *mut squashfs_page_actor,
    ) -> c_int>,
    pub id: c_int,
    pub name: *mut c_char,
    pub alloc_buffer: c_int,
    pub supported: c_int,
}

pub unsafe fn squashfs_comp_opts(
    msblk: *mut squashfs_sb_info,
    buff: *mut c_void,
    length: c_int,
) -> *mut c_void {
    let decompressor = (*msblk).decompressor;
    match (*decompressor).comp_opts {
        Some(comp_opts) => comp_opts(msblk, buff, length),
        None => core::ptr::null_mut(),
    }
}

/* CONFIG_SQUASHFS_XZ controls this declaration. */
#[cfg(feature = "CONFIG_SQUASHFS_XZ")]
extern "C" {
    pub static squashfs_xz_comp_ops: squashfs_decompressor;
}

/* CONFIG_SQUASHFS_LZ4 controls this declaration. */
#[cfg(feature = "CONFIG_SQUASHFS_LZ4")]
extern "C" {
    pub static squashfs_lz4_comp_ops: squashfs_decompressor;
}

/* CONFIG_SQUASHFS_LZO controls this declaration. */
#[cfg(feature = "CONFIG_SQUASHFS_LZO")]
extern "C" {
    pub static squashfs_lzo_comp_ops: squashfs_decompressor;
}

/* CONFIG_SQUASHFS_ZLIB controls this declaration. */
#[cfg(feature = "CONFIG_SQUASHFS_ZLIB")]
extern "C" {
    pub static squashfs_zlib_comp_ops: squashfs_decompressor;
}

/* CONFIG_SQUASHFS_ZSTD controls this declaration. */
#[cfg(feature = "CONFIG_SQUASHFS_ZSTD")]
extern "C" {
    pub static squashfs_zstd_comp_ops: squashfs_decompressor;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
