// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/lzma.c. C includes map to external declarations
// supplied by the surrounding repository/libc/liblzma bindings.

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};

const BUFSIZE: usize = 8192;

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type uint8_t = u8;
type u8_ = u8;
type lzma_ret = c_int;
type lzma_action = c_int;

const LZMA_RUN: lzma_action = 0;
const LZMA_FINISH: lzma_action = 3;

const LZMA_OK: lzma_ret = 0;
const LZMA_STREAM_END: lzma_ret = 1;
const LZMA_MEM_ERROR: lzma_ret = 5;
const LZMA_FORMAT_ERROR: lzma_ret = 7;
const LZMA_OPTIONS_ERROR: lzma_ret = 8;
const LZMA_DATA_ERROR: lzma_ret = 9;
const LZMA_BUF_ERROR: lzma_ret = 10;

const LZMA_CONCATENATED: c_uint = 0x08;
const O_RDONLY: c_int = 0;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lzma_allocator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lzma_internal {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lzma_stream {
    pub next_in: *const u8,
    pub avail_in: size_t,
    pub total_in: u64,
    pub next_out: *mut u8,
    pub avail_out: size_t,
    pub total_out: u64,
    pub allocator: *const lzma_allocator,
    pub internal: *mut lzma_internal,
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
    pub reserved_ptr3: *mut c_void,
    pub reserved_ptr4: *mut c_void,
    pub reserved_int1: u64,
    pub reserved_int2: u64,
    pub reserved_int3: size_t,
    pub reserved_int4: size_t,
    pub reserved_enum1: lzma_ret,
    pub reserved_enum2: lzma_ret,
}

// Equivalent to LZMA_STREAM_INIT for the fields visible in this translation.
const LZMA_STREAM_INIT: lzma_stream = lzma_stream {
    next_in: core::ptr::null(),
    avail_in: 0,
    total_in: 0,
    next_out: core::ptr::null_mut(),
    avail_out: 0,
    total_out: 0,
    allocator: core::ptr::null(),
    internal: core::ptr::null_mut(),
    reserved_ptr1: core::ptr::null_mut(),
    reserved_ptr2: core::ptr::null_mut(),
    reserved_ptr3: core::ptr::null_mut(),
    reserved_ptr4: core::ptr::null_mut(),
    reserved_int1: 0,
    reserved_int2: 0,
    reserved_int3: 0,
    reserved_int4: 0,
    reserved_enum1: LZMA_OK,
    reserved_enum2: LZMA_OK,
};

unsafe extern "C" {
    fn lzma_stream_decoder(strm: *mut lzma_stream, memlimit: u64, flags: u32) -> lzma_ret;
    fn lzma_code(strm: *mut lzma_stream, action: lzma_action) -> lzma_ret;
    fn lzma_end(strm: *mut lzma_stream);

    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn feof(stream: *mut FILE) -> c_int;
    fn ferror(stream: *mut FILE) -> c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;

    fn writen(fd: c_int, buf: *const c_void, n: size_t) -> ssize_t;
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe fn lzma_strerror(ret: lzma_ret) -> *const c_char {
    match ret as c_int {
        LZMA_MEM_ERROR => c"Memory allocation failed".as_ptr(),
        LZMA_OPTIONS_ERROR => c"Unsupported decompressor flags".as_ptr(),
        LZMA_FORMAT_ERROR => c"The input is not in the .xz format".as_ptr(),
        LZMA_DATA_ERROR => c"Compressed file is corrupt".as_ptr(),
        LZMA_BUF_ERROR => c"Compressed file is truncated or otherwise corrupt".as_ptr(),
        _ => c"Unknown error, possibly a bug".as_ptr(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn lzma_decompress_stream_to_file(
    infile: *mut FILE,
    output_fd: c_int,
) -> c_int {
    let mut action: lzma_action = LZMA_RUN;
    let mut strm: lzma_stream = LZMA_STREAM_INIT;
    let mut ret: lzma_ret;
    let mut err: c_int = -1;

    let mut buf_in: [u8_; BUFSIZE] = [0; BUFSIZE];
    let mut buf_out: [u8_; BUFSIZE] = [0; BUFSIZE];

    ret = lzma_stream_decoder(&mut strm, u64::MAX, LZMA_CONCATENATED);
    if ret != LZMA_OK {
        pr_debug(
            c"lzma: lzma_stream_decoder failed %s (%d)\n".as_ptr(),
            lzma_strerror(ret),
            ret,
        );
        return err;
    }

    strm.next_in = core::ptr::null();
    strm.avail_in = 0;
    strm.next_out = buf_out.as_mut_ptr();
    strm.avail_out = core::mem::size_of_val(&buf_out);

    loop {
        if strm.avail_in == 0 && feof(infile) == 0 {
            strm.next_in = buf_in.as_mut_ptr();
            strm.avail_in = fread(
                buf_in.as_mut_ptr() as *mut c_void,
                1,
                core::mem::size_of_val(&buf_in),
                infile,
            );

            if ferror(infile) != 0 {
                pr_debug(c"lzma: read error: %m\n".as_ptr());
                goto_err_lzma_end(&mut strm);
                return err;
            }

            if feof(infile) != 0 {
                action = LZMA_FINISH;
            }
        }

        ret = lzma_code(&mut strm, action);

        if strm.avail_out == 0 || ret == LZMA_STREAM_END {
            let write_size: ssize_t = (core::mem::size_of_val(&buf_out) - strm.avail_out) as ssize_t;

            if writen(output_fd, buf_out.as_ptr() as *const c_void, write_size as size_t) != write_size {
                pr_debug(c"lzma: write error: %m\n".as_ptr());
                goto_err_lzma_end(&mut strm);
                return err;
            }

            strm.next_out = buf_out.as_mut_ptr();
            strm.avail_out = core::mem::size_of_val(&buf_out);
        }

        if ret != LZMA_OK {
            if ret == LZMA_STREAM_END {
                break;
            }

            pr_debug(c"lzma: failed %s\n".as_ptr(), lzma_strerror(ret));
            goto_err_lzma_end(&mut strm);
            return err;
        }
    }

    err = 0;
    lzma_end(&mut strm);
    err
}

unsafe fn goto_err_lzma_end(strm: *mut lzma_stream) {
    lzma_end(strm);
}

#[no_mangle]
pub unsafe extern "C" fn lzma_decompress_to_file(input: *const c_char, output_fd: c_int) -> c_int {
    let infile: *mut FILE;
    let ret: c_int;

    infile = fopen(input, c"rb".as_ptr());
    if infile.is_null() {
        pr_debug(c"lzma: fopen failed on %s: '%m'\n".as_ptr(), input);
        return -1;
    }

    ret = lzma_decompress_stream_to_file(infile, output_fd);
    fclose(infile);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn lzma_is_compressed(input: *const c_char) -> bool_ {
    let fd: c_int = open(input, O_RDONLY);
    let magic: [uint8_t; 6] = [0xFD, b'7', b'z', b'X', b'Z', 0x00];
    let mut buf: [c_char; 6] = [0; 6];
    let rc: ssize_t;

    if fd < 0 {
        return false;
    }

    rc = read(fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf));
    close(fd);
    if rc == core::mem::size_of_val(&buf) as ssize_t {
        memcmp(
            buf.as_ptr() as *const c_void,
            magic.as_ptr() as *const c_void,
            core::mem::size_of_val(&buf),
        ) == 0
    } else {
        false
    }
}
