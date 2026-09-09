// SPDX-License-Identifier: GPL-2.0
/*
 * uncompress.c
 *
 * (C) Copyright 1999 Linus Torvalds
 *
 * cramfs interfaces to the uncompression library. There's really just
 * three entrypoints:
 *
 *  - cramfs_uncompress_init() - called to initialize the thing.
 *  - cramfs_uncompress_exit() - tell me when you're done
 *  - cramfs_uncompress_block() - uncompress a block.
 *
 * NOTE NOTE NOTE! The uncompression is entirely single-threaded. We
 * only have one stream, and we'll initialize it only once even if it
 * then is used by multiple filesystems.
 */

// Dependencies supplied by the surrounding kernel/zlib translation.
use core::ffi::{c_int, c_void};

extern "C" {
    fn zlib_inflate_reset(stream: *mut z_stream) -> c_int;
    fn zlib_inflate_end(stream: *mut z_stream) -> c_int;
    fn zlib_inflate_init(stream: *mut z_stream) -> c_int;
    fn zlib_inflate(stream: *mut z_stream, flush: c_int) -> c_int;
    fn zlib_inflate_workspacesize() -> usize;
    fn vmalloc(size: usize) -> *mut c_void;
    fn vfree(addr: *mut c_void);
}

extern "C" {
    static mut stream: z_stream;
}

// These names and the z_stream layout are provided by the corresponding
// kernel/zlib headers.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_stream {
    pub next_in: *mut u8,
    pub avail_in: u32,
    pub total_in: u64,
    pub next_out: *mut u8,
    pub avail_out: u32,
    pub total_out: u64,
    pub msg: *mut u8,
    pub state: *mut c_void,
    pub zalloc: *mut c_void,
    pub zfree: *mut c_void,
    pub opaque: *mut c_void,
    pub data_type: c_int,
    pub adler: u64,
    pub reserved: u64,
    pub workspace: *mut c_void,
}

static mut initialized: c_int = 0;

const Z_OK: c_int = 0;
const Z_FINISH: c_int = 4;
const Z_STREAM_END: c_int = 1;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;

/* Returns length of decompressed data. */
pub unsafe fn cramfs_uncompress_block(
    dst: *mut c_void,
    dstlen: c_int,
    src: *mut c_void,
    srclen: c_int,
) -> c_int {
    let mut err: c_int;

    stream.next_in = src as *mut u8;
    stream.avail_in = srclen as u32;

    stream.next_out = dst as *mut u8;
    stream.avail_out = dstlen as u32;

    err = zlib_inflate_reset(&mut stream);
    if err != Z_OK {
        pr_err!("zlib_inflateReset error {}\n", err);
        zlib_inflate_end(&mut stream);
        zlib_inflate_init(&mut stream);
    }

    err = zlib_inflate(&mut stream, Z_FINISH);
    if err != Z_STREAM_END {
        pr_err!("Error {} while decompressing!\n", err);
        pr_err!("{:p}({})->{:p}({})\n", src, srclen, dst, dstlen);
        return -EIO;
    }
    stream.total_out as c_int
}

pub unsafe fn cramfs_uncompress_init() -> c_int {
    if initialized == 0 {
        initialized += 1;
        stream.workspace = vmalloc(zlib_inflate_workspacesize());
        if stream.workspace.is_null() {
            initialized = 0;
            return -ENOMEM;
        }
        stream.next_in = core::ptr::null_mut();
        stream.avail_in = 0;
        zlib_inflate_init(&mut stream);
    } else {
        initialized += 1;
    }
    0
}

pub unsafe fn cramfs_uncompress_exit() {
    initialized -= 1;
    if initialized == 0 {
        zlib_inflate_end(&mut stream);
        vfree(stream.workspace);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
