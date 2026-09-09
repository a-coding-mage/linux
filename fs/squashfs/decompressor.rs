// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * decompressor.c
 */

use core::ffi::{c_char, c_int, c_ushort, c_void};

// Declarations supplied by the Squashfs headers and other translation units.
#[repr(C)]
pub struct super_block {
    pub s_fs_info: *mut c_void,
}
#[repr(C)]
pub struct squashfs_sb_info {
    pub thread_ops: *mut squashfs_decompressor,
}
#[repr(C)]
pub struct squashfs_page_actor;

#[repr(C)]
pub struct squashfs_decompressor {
    pub init: Option<unsafe extern "C" fn(*mut squashfs_sb_info, *mut c_void) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub decompress: Option<unsafe extern "C" fn() -> c_int>,
    pub max_decompress: Option<unsafe extern "C" fn() -> c_int>,
    pub id: c_int,
    pub name: *const c_char,
    pub supported: c_int,
}

unsafe extern "C" {
    fn kmalloc(size: usize, flags: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn squashfs_page_actor_init(
        buffer: *mut *mut c_void,
        pages: c_int,
        length: c_int,
    ) -> *mut squashfs_page_actor;
    fn squashfs_read_data(
        sb: *mut super_block,
        index: usize,
        length: c_int,
        block_list: *mut c_void,
        actor: *mut squashfs_page_actor,
    ) -> c_int;
    fn squashfs_comp_opts(
        msblk: *mut squashfs_sb_info,
        buffer: *mut c_void,
        length: c_int,
    ) -> *mut c_void;
    fn create(msblk: *mut squashfs_sb_info, comp_opts: *mut c_void) -> *mut c_void;
    fn is_err(ptr: *mut c_void) -> bool;
}

const PAGE_SIZE: usize = 4096;
const GFP_KERNEL: usize = 0;
const LZMA_COMPRESSION: c_int = 2;
const LZ4_COMPRESSION: c_int = 4;
const LZO_COMPRESSION: c_int = 3;
const XZ_COMPRESSION: c_int = 5;
const ZLIB_COMPRESSION: c_int = 1;
const ZSTD_COMPRESSION: c_int = 6;
const SQUASHFS_COMP_OPTS_MASK: c_ushort = 0x0010;

const fn squashfs_comp_opts_flag(flags: c_ushort) -> bool {
    flags & SQUASHFS_COMP_OPTS_MASK != 0
}

static SQUASHFS_LZMA_UNSUPPORTED_COMP_OPS: squashfs_decompressor = squashfs_decompressor {
    init: None, free: None, decompress: None, max_decompress: None,
    id: LZMA_COMPRESSION, name: b"lzma\0".as_ptr() as *const c_char, supported: 0,
};
static SQUASHFS_LZ4_COMP_OPS: squashfs_decompressor = squashfs_decompressor {
    init: None, free: None, decompress: None, max_decompress: None,
    id: LZ4_COMPRESSION, name: b"lz4\0".as_ptr() as *const c_char, supported: 0,
};
static SQUASHFS_LZO_COMP_OPS: squashfs_decompressor = squashfs_decompressor {
    init: None, free: None, decompress: None, max_decompress: None,
    id: LZO_COMPRESSION, name: b"lzo\0".as_ptr() as *const c_char, supported: 0,
};
static SQUASHFS_XZ_COMP_OPS: squashfs_decompressor = squashfs_decompressor {
    init: None, free: None, decompress: None, max_decompress: None,
    id: XZ_COMPRESSION, name: b"xz\0".as_ptr() as *const c_char, supported: 0,
};
static SQUASHFS_ZLIB_COMP_OPS: squashfs_decompressor = squashfs_decompressor {
    init: None, free: None, decompress: None, max_decompress: None,
    id: ZLIB_COMPRESSION, name: b"zlib\0".as_ptr() as *const c_char, supported: 0,
};
static SQUASHFS_ZSTD_COMP_OPS: squashfs_decompressor = squashfs_decompressor {
    init: None, free: None, decompress: None, max_decompress: None,
    id: ZSTD_COMPRESSION, name: b"zstd\0".as_ptr() as *const c_char, supported: 0,
};
static SQUASHFS_UNKNOWN_COMP_OPS: squashfs_decompressor = squashfs_decompressor {
    init: None, free: None, decompress: None, max_decompress: None,
    id: 0, name: b"unknown\0".as_ptr() as *const c_char, supported: 0,
};

static DECOMPRESSOR: [&squashfs_decompressor; 7] = [
    &SQUASHFS_ZLIB_COMP_OPS, &SQUASHFS_LZ4_COMP_OPS, &SQUASHFS_LZO_COMP_OPS,
    &SQUASHFS_XZ_COMP_OPS, &SQUASHFS_LZMA_UNSUPPORTED_COMP_OPS,
    &SQUASHFS_ZSTD_COMP_OPS, &SQUASHFS_UNKNOWN_COMP_OPS,
];

pub unsafe fn squashfs_lookup_decompressor(id: c_int) -> *const squashfs_decompressor {
    let mut i = 0;
    while DECOMPRESSOR[i].id != 0 {
        if id == DECOMPRESSOR[i].id { break; }
        i += 1;
    }
    DECOMPRESSOR[i]
}

unsafe fn get_comp_opts(sb: *mut super_block, flags: c_ushort) -> *mut c_void {
    let msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    let mut buffer: *mut c_void = core::ptr::null_mut();
    let mut comp_opts: *mut c_void;
    let mut actor: *mut squashfs_page_actor = core::ptr::null_mut();
    let mut length = 0;
    if squashfs_comp_opts_flag(flags) {
        buffer = kmalloc(PAGE_SIZE, GFP_KERNEL);
        if buffer.is_null() { return (-12isize) as *mut c_void; }
        actor = squashfs_page_actor_init(&mut buffer, 1, 0);
        if actor.is_null() { kfree(buffer); return (-12isize) as *mut c_void; }
        length = squashfs_read_data(sb, core::mem::size_of::<super_block>(), 0, core::ptr::null_mut(), actor);
        if length < 0 { kfree(actor as *mut c_void); kfree(buffer); return length as isize as *mut c_void; }
    }
    comp_opts = squashfs_comp_opts(msblk, buffer, length);
    kfree(actor as *mut c_void);
    kfree(buffer);
    comp_opts
}

pub unsafe fn squashfs_decompressor_setup(sb: *mut super_block, flags: c_ushort) -> *mut c_void {
    let msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    let comp_opts = get_comp_opts(sb, flags);
    if is_err(comp_opts) { return comp_opts; }
    let stream = create(msblk, comp_opts);
    if is_err(stream) { kfree(comp_opts); }
    stream
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
