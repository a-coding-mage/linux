// SPDX-License-Identifier: GPL-2.0
// C dependencies: fcntl.h, stdio.h, string.h, unistd.h, sys/stat.h,
// sys/mman.h, zlib.h, linux/compiler.h, internal/lib.h, util/compress.h

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};

const CHUNK_SIZE: usize = 16384;

const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 0x1;
const MAP_PRIVATE: c_int = 0x02;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

const Z_NO_FLUSH: c_int = 0;
const Z_OK: c_int = 0;
const Z_STREAM_END: c_int = 1;
const Z_NEED_DICT: c_int = 2;
const Z_STREAM_ERROR: c_int = -2;
const Z_DATA_ERROR: c_int = -3;
const Z_MEM_ERROR: c_int = -4;
const MAX_WBITS: c_int = 15;

type ssize_t = isize;
type size_t = usize;
type off_t = i64;

#[repr(C)]
pub struct stat {
    pub st_dev: c_ulong,
    pub st_ino: c_ulong,
    pub st_nlink: c_ulong,
    pub st_mode: c_uint,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad0: c_int,
    pub st_rdev: c_ulong,
    pub st_size: off_t,
    pub st_blksize: c_ulong,
    pub st_blocks: c_ulong,
    pub st_atime: c_ulong,
    pub st_atime_nsec: c_ulong,
    pub st_mtime: c_ulong,
    pub st_mtime_nsec: c_ulong,
    pub st_ctime: c_ulong,
    pub st_ctime_nsec: c_ulong,
    pub __unused: [c_long; 3],
}

type c_long = i64;

#[repr(C)]
pub struct z_stream {
    pub next_in: *mut c_uchar,
    pub avail_in: c_uint,
    pub total_in: c_ulong,
    pub next_out: *mut c_uchar,
    pub avail_out: c_uint,
    pub total_out: c_ulong,
    pub msg: *mut c_char,
    pub state: *mut c_void,
    pub zalloc: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> *mut c_void>,
    pub zfree: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    pub opaque: *mut c_void,
    pub data_type: c_int,
    pub adler: c_ulong,
    pub reserved: c_ulong,
}

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn inflateInit2_(strm: *mut z_stream, windowBits: c_int, version: *const c_char, stream_size: c_int)
        -> c_int;
    fn inflate(strm: *mut z_stream, flush: c_int) -> c_int;
    fn inflateEnd(strm: *mut z_stream) -> c_int;
    fn zlibVersion() -> *const c_char;

    fn writen(fd: c_int, buf: *const c_void, n: size_t) -> ssize_t;
}

unsafe fn inflateInit2(strm: *mut z_stream, windowBits: c_int) -> c_int {
    unsafe {
        inflateInit2_(
            strm,
            windowBits,
            zlibVersion(),
            core::mem::size_of::<z_stream>() as c_int,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gzip_decompress_to_file(input: *const c_char, output_fd: c_int) -> c_int {
    let mut ret: c_int = Z_STREAM_ERROR;
    let input_fd: c_int;
    let ptr: *mut c_void;
    let mut len: c_int;
    let mut stbuf: stat = unsafe { core::mem::zeroed() };
    let mut buf: [c_uchar; CHUNK_SIZE] = [0; CHUNK_SIZE];
    let mut zs: z_stream = unsafe { core::mem::zeroed() };

    zs.zalloc = None;
    zs.zfree = None;
    zs.opaque = core::ptr::null_mut();
    zs.avail_in = 0;
    zs.next_in = core::ptr::null_mut();

    input_fd = unsafe { open(input, O_RDONLY) };
    if input_fd < 0 {
        return -1;
    }

    if unsafe { fstat(input_fd, &mut stbuf) } < 0 {
        unsafe {
            close(input_fd);
        }
        return if ret == Z_STREAM_END { 0 } else { -1 };
    }

    ptr = unsafe {
        mmap(
            core::ptr::null_mut(),
            stbuf.st_size as size_t,
            PROT_READ,
            MAP_PRIVATE,
            input_fd,
            0,
        )
    };
    if ptr == MAP_FAILED {
        unsafe {
            close(input_fd);
        }
        return if ret == Z_STREAM_END { 0 } else { -1 };
    }

    if unsafe { inflateInit2(&mut zs, 16 + MAX_WBITS) } != Z_OK {
        unsafe {
            munmap(ptr, stbuf.st_size as size_t);
            close(input_fd);
        }
        return if ret == Z_STREAM_END { 0 } else { -1 };
    }

    zs.next_in = ptr as *mut c_uchar;
    zs.avail_in = stbuf.st_size as c_uint;

    loop {
        zs.next_out = buf.as_mut_ptr();
        zs.avail_out = CHUNK_SIZE as c_uint;

        ret = unsafe { inflate(&mut zs, Z_NO_FLUSH) };
        match ret {
            Z_NEED_DICT => {
                ret = Z_DATA_ERROR;
                unsafe {
                    inflateEnd(&mut zs);
                    munmap(ptr, stbuf.st_size as size_t);
                    close(input_fd);
                }
                return if ret == Z_STREAM_END { 0 } else { -1 };
            }
            Z_DATA_ERROR | Z_MEM_ERROR => {
                unsafe {
                    inflateEnd(&mut zs);
                    munmap(ptr, stbuf.st_size as size_t);
                    close(input_fd);
                }
                return if ret == Z_STREAM_END { 0 } else { -1 };
            }
            _ => {}
        }

        len = (CHUNK_SIZE as c_uint).wrapping_sub(zs.avail_out) as c_int;
        if unsafe { writen(output_fd, buf.as_ptr() as *const c_void, len as size_t) } != len as ssize_t {
            ret = Z_DATA_ERROR;
            unsafe {
                inflateEnd(&mut zs);
                munmap(ptr, stbuf.st_size as size_t);
                close(input_fd);
            }
            return if ret == Z_STREAM_END { 0 } else { -1 };
        }

        if ret == Z_STREAM_END {
            break;
        }
    }

    unsafe {
        inflateEnd(&mut zs);
        munmap(ptr, stbuf.st_size as size_t);
        close(input_fd);
    }

    if ret == Z_STREAM_END { 0 } else { -1 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gzip_is_compressed(input: *const c_char) -> bool {
    let fd: c_int = unsafe { open(input, O_RDONLY) };
    let magic: [u8; 2] = [0x1f, 0x8b];
    let mut buf: [c_char; 2] = [0; 2];
    let rc: ssize_t;

    if fd < 0 {
        return false;
    }

    rc = unsafe { read(fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) };
    unsafe {
        close(fd);
    }
    if rc == core::mem::size_of_val(&buf) as ssize_t {
        unsafe {
            memcmp(
                buf.as_ptr() as *const c_void,
                magic.as_ptr() as *const c_void,
                core::mem::size_of_val(&buf),
            ) == 0
        }
    } else {
        false
    }
}
