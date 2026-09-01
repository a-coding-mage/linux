// SPDX-License-Identifier: GPL-2.0
// Translated from gen_init_cpio.c. C include dependencies are represented as
// local FFI declarations and constants.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::env;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr;

type size_t = usize;
type ssize_t = isize;
type time_t = c_long;
type uid_t = c_uint;
type gid_t = c_uint;
type mode_t = c_uint;

const CPIO_HDR_LEN: usize = 110;
const CPIO_TRAILER: &[u8] = b"TRAILER!!!\0";
const PATH_MAX: usize = 4096;
const STDOUT_FILENO: c_int = 1;
const SEEK_SET: c_int = 0;
const EINVAL: c_int = 22;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const O_LARGEFILE: c_int = 0;
const S_IFLNK: c_uint = 0o120000;
const S_IFDIR: mode_t = 0o040000;
const S_IFIFO: mode_t = 0o010000;
const S_IFSOCK: mode_t = 0o140000;
const S_IFBLK: c_uint = 0o060000;
const S_IFCHR: c_uint = 0o020000;
const S_IFREG: c_uint = 0o100000;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    st_nlink: c_ulong,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: c_ulong,
    st_size: c_long,
    st_blksize: c_long,
    st_blocks: c_long,
    st_atime: time_t,
    st_atime_nsec: c_long,
    st_mtime: time_t,
    st_mtime_nsec: c_long,
    st_ctime: time_t,
    st_ctime_nsec: c_long,
    __glibc_reserved: [c_long; 3],
}

type file_handler_fn = unsafe fn(*const c_char) -> c_int;

#[repr(C)]
struct file_handler {
    type_: *const c_char,
    handler: Option<file_handler_fn>,
}

#[repr(C)]
struct generic_type {
    type_: *const c_char,
    mode: mode_t,
}

#[derive(Copy, Clone)]
enum generic_types {
    GT_DIR = 0,
    GT_PIPE = 1,
    GT_SOCK = 2,
}

unsafe extern "C" {
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    static mut optind: c_int;

    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn dprintf(fd: c_int, format: *const c_char, ...) -> c_int;
    fn fsync(fd: c_int) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn lseek(fd: c_int, offset: c_long, whence: c_int) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn copy_file_range(
        fd_in: c_int,
        off_in: *mut c_long,
        fd_out: c_int,
        off_out: *mut c_long,
        len: size_t,
        flags: c_uint,
    ) -> ssize_t;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn isgraph(c: c_int) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn time(tloc: *mut time_t) -> time_t;
    fn exit(status: c_int) -> !;
}

static mut padding: [c_char; PATH_MAX] = [0; PATH_MAX];
static mut offset: c_uint = 0;
static mut ino: c_uint = 721;
static mut default_mtime: time_t = 0;
static mut do_file_mtime: bool = false;
static mut do_csum: bool = false;
static mut outfd: c_int = STDOUT_FILENO;
static mut dalign: c_uint = 0;

const fn padlen(off: usize, align: usize) -> usize {
    ((align - (off & (align - 1))) % align)
}

unsafe fn push_buf(name: *const c_char, name_len: size_t) -> c_int {
    let len = unsafe { write(outfd, name as *const c_void, name_len) };
    if len != name_len as ssize_t {
        return -1;
    }

    unsafe { offset = offset.wrapping_add(name_len as c_uint) };
    0
}

unsafe fn push_pad(padlen_: size_t) -> c_int {
    let mut len: ssize_t = 0;

    if padlen_ == 0 {
        return 0;
    }

    if padlen_ < PATH_MAX {
        len = unsafe { write(outfd, padding.as_ptr() as *const c_void, padlen_) };
    }
    if len != padlen_ as ssize_t {
        return -1;
    }

    unsafe { offset = offset.wrapping_add(padlen_ as c_uint) };
    0
}

unsafe fn push_rest(name: *const c_char, name_len: size_t) -> c_int {
    let len = unsafe { write(outfd, name as *const c_void, name_len) };
    if len != name_len as ssize_t {
        return -1;
    }

    unsafe { offset = offset.wrapping_add(name_len as c_uint) };

    unsafe { push_pad(padlen(name_len + CPIO_HDR_LEN, 4)) }
}

unsafe fn cpio_trailer() -> c_int {
    let namesize: c_uint = CPIO_TRAILER.len() as c_uint;
    let len = unsafe {
        dprintf(
            outfd,
            c"%s%08X%08X%08lX%08lX%08X%08lX%08X%08X%08X%08X%08X%08X%08X".as_ptr(),
            if do_csum { c"070702".as_ptr() } else { c"070701".as_ptr() },
            0,
            0,
            0 as c_long,
            0 as c_long,
            1,
            0 as c_long,
            0,
            0,
            0,
            0,
            0,
            namesize,
            0,
        )
    };
    unsafe { offset = offset.wrapping_add(len as c_uint) };

    if len != CPIO_HDR_LEN as c_int
        || unsafe { push_rest(CPIO_TRAILER.as_ptr() as *const c_char, namesize as size_t) } < 0
        || unsafe { push_pad(padlen(offset as usize, 512)) } < 0
    {
        return -1;
    }

    if unsafe { fsync(outfd) } < 0 && unsafe { errno } != EINVAL {
        return -1;
    }

    0
}

unsafe fn cpio_mkslink(
    mut name: *const c_char,
    target: *const c_char,
    mut mode: c_uint,
    uid: uid_t,
    gid: gid_t,
) -> c_int {
    let targetsize: c_uint = unsafe { strlen(target).wrapping_add(1) as c_uint };

    if unsafe { *name } == b'/' as c_char {
        name = unsafe { name.add(1) };
    }
    let namesize: c_uint = unsafe { strlen(name).wrapping_add(1) as c_uint };

    mode |= S_IFLNK;
    let len = unsafe {
        dprintf(
            outfd,
            c"%s%08X%08X%08lX%08lX%08X%08lX%08X%08X%08X%08X%08X%08X%08X".as_ptr(),
            if do_csum { c"070702".as_ptr() } else { c"070701".as_ptr() },
            {
                let v = ino;
                ino = ino.wrapping_add(1);
                v
            },
            mode,
            uid as c_long,
            gid as c_long,
            1,
            default_mtime as c_long,
            targetsize,
            3,
            1,
            0,
            0,
            namesize,
            0,
        )
    };
    unsafe { offset = offset.wrapping_add(len as c_uint) };

    if len != CPIO_HDR_LEN as c_int
        || unsafe { push_buf(name, namesize as size_t) } < 0
        || unsafe { push_pad(padlen(offset as usize, 4)) } < 0
        || unsafe { push_buf(target, targetsize as size_t) } < 0
        || unsafe { push_pad(padlen(offset as usize, 4)) } < 0
    {
        return -1;
    }

    0
}

unsafe fn cpio_mkslink_line(line: *const c_char) -> c_int {
    let mut name = [0 as c_char; PATH_MAX + 1];
    let mut target = [0 as c_char; PATH_MAX + 1];
    let mut mode: c_uint = 0;
    let mut uid: c_int = 0;
    let mut gid: c_int = 0;
    let mut rc: c_int = -1;

    if unsafe {
        sscanf(
            line,
            c"%4096s %4096s %o %d %d".as_ptr(),
            name.as_mut_ptr(),
            target.as_mut_ptr(),
            &mut mode,
            &mut uid,
            &mut gid,
        )
    } != 5
    {
        unsafe { fprintf(stderr, c"Unrecognized dir format '%s'".as_ptr(), line) };
    } else {
        rc = unsafe { cpio_mkslink(name.as_ptr(), target.as_ptr(), mode, uid as uid_t, gid as gid_t) };
    }
    rc
}

unsafe fn cpio_mkgeneric(
    mut name: *const c_char,
    mode: c_uint,
    uid: uid_t,
    gid: gid_t,
) -> c_int {
    if unsafe { *name } == b'/' as c_char {
        name = unsafe { name.add(1) };
    }
    let namesize: c_uint = unsafe { strlen(name).wrapping_add(1) as c_uint };

    let len = unsafe {
        dprintf(
            outfd,
            c"%s%08X%08X%08lX%08lX%08X%08lX%08X%08X%08X%08X%08X%08X%08X".as_ptr(),
            if do_csum { c"070702".as_ptr() } else { c"070701".as_ptr() },
            {
                let v = ino;
                ino = ino.wrapping_add(1);
                v
            },
            mode,
            uid as c_long,
            gid as c_long,
            2,
            default_mtime as c_long,
            0,
            3,
            1,
            0,
            0,
            namesize,
            0,
        )
    };
    unsafe { offset = offset.wrapping_add(len as c_uint) };

    if len != CPIO_HDR_LEN as c_int || unsafe { push_rest(name, namesize as size_t) } < 0 {
        return -1;
    }

    0
}

static generic_type_table: [generic_type; 3] = [
    generic_type {
        type_: c"dir".as_ptr(),
        mode: S_IFDIR,
    },
    generic_type {
        type_: c"pipe".as_ptr(),
        mode: S_IFIFO,
    },
    generic_type {
        type_: c"sock".as_ptr(),
        mode: S_IFSOCK,
    },
];

unsafe fn cpio_mkgeneric_line(line: *const c_char, gt: generic_types) -> c_int {
    let mut name = [0 as c_char; PATH_MAX + 1];
    let mut mode: c_uint = 0;
    let mut uid: c_int = 0;
    let mut gid: c_int = 0;
    let mut rc: c_int = -1;
    let idx = gt as usize;

    if unsafe {
        sscanf(
            line,
            c"%4096s %o %d %d".as_ptr(),
            name.as_mut_ptr(),
            &mut mode,
            &mut uid,
            &mut gid,
        )
    } != 4
    {
        unsafe {
            fprintf(
                stderr,
                c"Unrecognized %s format '%s'".as_ptr(),
                line,
                generic_type_table[idx].type_,
            )
        };
    } else {
        mode |= generic_type_table[idx].mode;
        rc = unsafe { cpio_mkgeneric(name.as_ptr(), mode, uid as uid_t, gid as gid_t) };
    }
    rc
}

unsafe fn cpio_mkdir_line(line: *const c_char) -> c_int {
    unsafe { cpio_mkgeneric_line(line, generic_types::GT_DIR) }
}

unsafe fn cpio_mkpipe_line(line: *const c_char) -> c_int {
    unsafe { cpio_mkgeneric_line(line, generic_types::GT_PIPE) }
}

unsafe fn cpio_mksock_line(line: *const c_char) -> c_int {
    unsafe { cpio_mkgeneric_line(line, generic_types::GT_SOCK) }
}

unsafe fn cpio_mknod(
    mut name: *const c_char,
    mut mode: c_uint,
    uid: uid_t,
    gid: gid_t,
    dev_type: c_char,
    maj: c_uint,
    min: c_uint,
) -> c_int {
    if dev_type == b'b' as c_char {
        mode |= S_IFBLK;
    } else {
        mode |= S_IFCHR;
    }

    if unsafe { *name } == b'/' as c_char {
        name = unsafe { name.add(1) };
    }
    let namesize: c_uint = unsafe { strlen(name).wrapping_add(1) as c_uint };

    let len = unsafe {
        dprintf(
            outfd,
            c"%s%08X%08X%08lX%08lX%08X%08lX%08X%08X%08X%08X%08X%08X%08X".as_ptr(),
            if do_csum { c"070702".as_ptr() } else { c"070701".as_ptr() },
            {
                let v = ino;
                ino = ino.wrapping_add(1);
                v
            },
            mode,
            uid as c_long,
            gid as c_long,
            1,
            default_mtime as c_long,
            0,
            3,
            1,
            maj,
            min,
            namesize,
            0,
        )
    };
    unsafe { offset = offset.wrapping_add(len as c_uint) };

    if len != CPIO_HDR_LEN as c_int || unsafe { push_rest(name, namesize as size_t) } < 0 {
        return -1;
    }

    0
}

unsafe fn cpio_mknod_line(line: *const c_char) -> c_int {
    let mut name = [0 as c_char; PATH_MAX + 1];
    let mut mode: c_uint = 0;
    let mut uid: c_int = 0;
    let mut gid: c_int = 0;
    let mut dev_type: c_char = 0;
    let mut maj: c_uint = 0;
    let mut min: c_uint = 0;
    let mut rc: c_int = -1;

    if unsafe {
        sscanf(
            line,
            c"%4096s %o %d %d %c %u %u".as_ptr(),
            name.as_mut_ptr(),
            &mut mode,
            &mut uid,
            &mut gid,
            &mut dev_type,
            &mut maj,
            &mut min,
        )
    } != 7
    {
        unsafe { fprintf(stderr, c"Unrecognized nod format '%s'".as_ptr(), line) };
    } else {
        rc = unsafe { cpio_mknod(name.as_ptr(), mode, uid as uid_t, gid as gid_t, dev_type, maj, min) };
    }
    rc
}

unsafe fn cpio_mkfile_csum(fd: c_int, mut size: c_ulong, csum: *mut u32) -> c_int {
    while size != 0 {
        let mut filebuf = [0u8; 65536];
        let this_size: size_t = if size < filebuf.len() as c_ulong {
            size as size_t
        } else {
            filebuf.len()
        };
        let this_read = unsafe { read(fd, filebuf.as_mut_ptr() as *mut c_void, this_size) };
        if this_read <= 0 || this_read as size_t > this_size {
            return -1;
        }

        for i in 0..this_read as usize {
            unsafe { *csum = (*csum).wrapping_add(filebuf[i] as u32) };
        }

        size = size.wrapping_sub(this_read as c_ulong);
    }
    /* seek back to the start for data segment I/O */
    if unsafe { lseek(fd, 0, SEEK_SET) } < 0 {
        return -1;
    }

    0
}

unsafe fn cpio_mkfile(
    mut name: *const c_char,
    location: *const c_char,
    mut mode: c_uint,
    uid: uid_t,
    gid: gid_t,
    nlinks: c_uint,
) -> c_int {
    let mut buf: stat = unsafe { mem::zeroed() };
    let mut size: c_ulong;
    let mut file: c_int;
    let mut retval: c_int;
    let mut len: c_int;
    let mut rc: c_int = -1;
    let mut mtime: time_t;
    let mut namesize: c_int;
    let mut namepadlen: c_int;
    let mut i: c_uint;
    let mut csum: u32 = 0;
    let mut this_read: ssize_t;

    mode |= S_IFREG;

    file = unsafe { open(location, O_RDONLY) };
    if file < 0 {
        unsafe { fprintf(stderr, c"File %s could not be opened for reading\n".as_ptr(), location) };
        return rc;
    }

    retval = unsafe { fstat(file, &mut buf) };
    if retval != 0 {
        unsafe { fprintf(stderr, c"File %s could not be stat()'ed\n".as_ptr(), location) };
        unsafe { close(file) };
        return rc;
    }

    if unsafe { do_file_mtime } {
        mtime = unsafe { default_mtime };
    } else {
        mtime = buf.st_mtime;
        if mtime > 0xffffffff {
            unsafe {
                fprintf(
                    stderr,
                    c"%s: Timestamp exceeds maximum cpio timestamp, clipping.\n".as_ptr(),
                    location,
                )
            };
            mtime = 0xffffffff;
        }

        if mtime < 0 {
            unsafe {
                fprintf(
                    stderr,
                    c"%s: Timestamp negative, clipping.\n".as_ptr(),
                    location,
                )
            };
            mtime = 0;
        }
    }

    if buf.st_size > 0xffffffff {
        unsafe { fprintf(stderr, c"%s: Size exceeds maximum cpio file size\n".as_ptr(), location) };
        unsafe { close(file) };
        return rc;
    }

    if unsafe { do_csum } && unsafe { cpio_mkfile_csum(file, buf.st_size as c_ulong, &mut csum) } < 0 {
        unsafe { fprintf(stderr, c"Failed to checksum file %s\n".as_ptr(), location) };
        unsafe { close(file) };
        return rc;
    }

    size = 0;
    namepadlen = 0;
    i = 1;
    while i <= nlinks {
        if unsafe { *name } == b'/' as c_char {
            name = unsafe { name.add(1) };
        }
        namesize = unsafe { strlen(name).wrapping_add(1) as c_int };

        /* data goes on last link, after any alignment padding */
        if i == nlinks {
            size = buf.st_size as c_ulong;
        }

        if unsafe { dalign } != 0 && size > unsafe { dalign as c_ulong } {
            namepadlen = padlen(
                unsafe { offset as usize } + CPIO_HDR_LEN + namesize as usize,
                unsafe { dalign as usize },
            ) as c_int;
            if namesize + namepadlen > PATH_MAX as c_int {
                unsafe {
                    fprintf(
                        stderr,
                        c"%s: best-effort alignment %u missed\n".as_ptr(),
                        name,
                        dalign,
                    )
                };
                namepadlen = 0;
            }
        }

        len = unsafe {
            dprintf(
                outfd,
                c"%s%08X%08X%08lX%08lX%08X%08lX%08lX%08X%08X%08X%08X%08X%08X".as_ptr(),
                if do_csum { c"070702".as_ptr() } else { c"070701".as_ptr() },
                ino,
                mode,
                uid as c_long,
                gid as c_long,
                nlinks,
                mtime as c_long,
                size as c_ulong,
                3,
                1,
                0,
                0,
                (namesize + namepadlen) as c_uint,
                if size != 0 { csum } else { 0 },
            )
        };
        unsafe { offset = offset.wrapping_add(len as c_uint) };

        if len != CPIO_HDR_LEN as c_int
            || unsafe { push_buf(name, namesize as size_t) } < 0
            || unsafe {
                push_pad(if namepadlen != 0 {
                    namepadlen as size_t
                } else {
                    padlen(offset as usize, 4)
                })
            } < 0
        {
            unsafe { close(file) };
            return rc;
        }

        if size != 0 {
            this_read = unsafe { copy_file_range(file, ptr::null_mut(), outfd, ptr::null_mut(), size as size_t, 0) };
            if this_read > 0 {
                if this_read as c_ulong > size {
                    unsafe { close(file) };
                    return rc;
                }
                unsafe { offset = offset.wrapping_add(this_read as c_uint) };
                size = size.wrapping_sub(this_read as c_ulong);
            }
            /* short or failed copy falls back to read/write... */
        }

        while size != 0 {
            let mut filebuf = [0u8; 65536];
            let this_size: size_t = if size < filebuf.len() as c_ulong {
                size as size_t
            } else {
                filebuf.len()
            };

            this_read = unsafe { read(file, filebuf.as_mut_ptr() as *mut c_void, this_size) };
            if this_read <= 0 || this_read as size_t > this_size {
                unsafe { fprintf(stderr, c"Can not read %s file\n".as_ptr(), location) };
                unsafe { close(file) };
                return rc;
            }

            if unsafe { write(outfd, filebuf.as_ptr() as *const c_void, this_read as size_t) } != this_read {
                unsafe { fprintf(stderr, c"writing filebuf failed\n".as_ptr()) };
                unsafe { close(file) };
                return rc;
            }
            unsafe { offset = offset.wrapping_add(this_read as c_uint) };
            size = size.wrapping_sub(this_read as c_ulong);
        }
        if unsafe { push_pad(padlen(offset as usize, 4)) } < 0 {
            unsafe { close(file) };
            return rc;
        }

        name = unsafe { name.add(namesize as usize) };
        i = i.wrapping_add(1);
    }
    unsafe { ino = ino.wrapping_add(1) };
    rc = 0;

    if file >= 0 {
        unsafe { close(file) };
    }
    rc
}

unsafe fn cpio_replace_env(new_location: *mut c_char) -> *mut c_char {
    let mut expanded = [0 as c_char; PATH_MAX + 1];
    let mut start: *mut c_char;
    let mut end: *mut c_char;
    let mut var: *mut c_char;

    loop {
        start = unsafe { strstr(new_location, c"${".as_ptr()) };
        if start.is_null() {
            break;
        }
        end = unsafe { strchr(start.add(2), b'}' as c_int) };
        if end.is_null() {
            break;
        }
        unsafe {
            *start = 0;
            *end = 0;
        }
        var = unsafe { getenv(start.add(2)) };
        unsafe {
            snprintf(
                expanded.as_mut_ptr(),
                expanded.len(),
                c"%s%s%s".as_ptr(),
                new_location,
                if !var.is_null() { var } else { c"".as_ptr() as *mut c_char },
                end.add(1),
            );
            strcpy(new_location, expanded.as_ptr());
        }
    }

    new_location
}

unsafe fn cpio_mkfile_line(line: *const c_char) -> c_int {
    let mut name = [0 as c_char; PATH_MAX + 1];
    let mut dname: *mut c_char = ptr::null_mut(); /* malloc'ed buffer for hard links */
    let mut location = [0 as c_char; PATH_MAX + 1];
    let mut mode: c_uint = 0;
    let mut uid: c_int = 0;
    let mut gid: c_int = 0;
    let mut nlinks: c_int = 1;
    let mut end: c_int = 0;
    let mut dname_len: c_int = 0;
    let mut rc: c_int = -1;

    if unsafe {
        sscanf(
            line,
            c"%4096s %4096s %o %d %d %n".as_ptr(),
            name.as_mut_ptr(),
            location.as_mut_ptr(),
            &mut mode,
            &mut uid,
            &mut gid,
            &mut end,
        )
    } > 5
    {
        // This branch is unreachable for the C condition `5 > sscanf(...)`.
    }
    if unsafe {
        sscanf(
            line,
            c"%4096s %4096s %o %d %d %n".as_ptr(),
            name.as_mut_ptr(),
            location.as_mut_ptr(),
            &mut mode,
            &mut uid,
            &mut gid,
            &mut end,
        )
    } < 5
    {
        unsafe { fprintf(stderr, c"Unrecognized file format '%s'".as_ptr(), line) };
    } else {
        if end != 0 && unsafe { isgraph(*line.add(end as usize) as c_int) } != 0 {
            let mut len_: c_int;
            let mut nend: c_int;

            dname = unsafe { malloc(strlen(line)) as *mut c_char };
            if dname.is_null() {
                unsafe { fprintf(stderr, c"out of memory (%d)\n".as_ptr(), dname_len) };
            } else {
                dname_len = unsafe { strlen(name.as_ptr()).wrapping_add(1) as c_int };
                unsafe { memcpy(dname as *mut c_void, name.as_ptr() as *const c_void, dname_len as size_t) };

                loop {
                    nend = 0;
                    if unsafe {
                        sscanf(
                            line.add(end as usize),
                            c"%4096s %n".as_ptr(),
                            name.as_mut_ptr(),
                            &mut nend,
                        )
                    } < 1
                    {
                        break;
                    }
                    len_ = unsafe { strlen(name.as_ptr()).wrapping_add(1) as c_int };
                    unsafe {
                        memcpy(
                            dname.add(dname_len as usize) as *mut c_void,
                            name.as_ptr() as *const c_void,
                            len_ as size_t,
                        )
                    };
                    dname_len += len_;
                    nlinks += 1;
                    end += nend;
                    if unsafe { isgraph(*line.add(end as usize) as c_int) } == 0 {
                        break;
                    }
                }
                rc = unsafe {
                    cpio_mkfile(
                        dname,
                        cpio_replace_env(location.as_mut_ptr()),
                        mode,
                        uid as uid_t,
                        gid as gid_t,
                        nlinks as c_uint,
                    )
                };
            }
        } else {
            dname = name.as_mut_ptr();
            rc = unsafe {
                cpio_mkfile(
                    dname,
                    cpio_replace_env(location.as_mut_ptr()),
                    mode,
                    uid as uid_t,
                    gid as gid_t,
                    nlinks as c_uint,
                )
            };
        }
    }
    if dname_len != 0 {
        unsafe { free(dname as *mut c_void) };
    }
    rc
}

unsafe fn usage(prog: *const c_char) {
    unsafe {
        fprintf(
            stderr,
            c"Usage:\n\t%s [-t <timestamp>] [-c] [-o <output_file>] [-a <data_align>] <cpio_list>\n\n<cpio_list> is a file containing newline separated entries that\ndescribe the files to be included in the initramfs archive:\n\n# a comment\nfile <name> <location> <mode> <uid> <gid> [<hard links>]\ndir <name> <mode> <uid> <gid>\nnod <name> <mode> <uid> <gid> <dev_type> <maj> <min>\nslink <name> <target> <mode> <uid> <gid>\npipe <name> <mode> <uid> <gid>\nsock <name> <mode> <uid> <gid>\n\n<name>       name of the file/dir/nod/etc in the archive\n<location>   location of the file in the current filesystem\n             expands shell variables quoted with ${}\n<target>     link target\n<mode>       mode/permissions of the file\n<uid>        user id (0=root)\n<gid>        group id (0=root)\n<dev_type>   device type (b=block, c=character)\n<maj>        major number of nod\n<min>        minor number of nod\n<hard links> space separated list of other links to file\n\nexample:\n# A simple initramfs\ndir /dev 0755 0 0\nnod /dev/console 0600 0 0 c 5 1\ndir /root 0700 0 0\ndir /sbin 0755 0 0\nfile /sbin/kinit /usr/src/klibc/kinit/kinit 0755 0 0\n\n<timestamp> is time in seconds since Epoch that will be used\nas mtime for symlinks, directories, regular and special files.\nThe default is to use the current time for all files, but\npreserve modification time for regular files.\n-c: calculate and store 32-bit checksums for file data.\n<output_file>: write cpio to this file instead of stdout\n<data_align>: attempt to align file data by zero-padding the\nfilename field up to data_align. Must be a multiple of 4.\nAlignment is best-effort; PATH_MAX limits filename padding.\n".as_ptr(),
            prog,
        )
    };
}

static file_handler_table: [file_handler; 7] = [
    file_handler {
        type_: c"file".as_ptr(),
        handler: Some(cpio_mkfile_line),
    },
    file_handler {
        type_: c"nod".as_ptr(),
        handler: Some(cpio_mknod_line),
    },
    file_handler {
        type_: c"dir".as_ptr(),
        handler: Some(cpio_mkdir_line),
    },
    file_handler {
        type_: c"slink".as_ptr(),
        handler: Some(cpio_mkslink_line),
    },
    file_handler {
        type_: c"pipe".as_ptr(),
        handler: Some(cpio_mkpipe_line),
    },
    file_handler {
        type_: c"sock".as_ptr(),
        handler: Some(cpio_mksock_line),
    },
    file_handler {
        type_: ptr::null(),
        handler: None,
    },
];

const LINE_SIZE: usize = 2 * PATH_MAX + 50;

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut cpio_list: *mut FILE;
    let mut line = [0 as c_char; LINE_SIZE];
    let mut args: *mut c_char;
    let mut type_: *mut c_char;
    let mut ec: c_int = 0;
    let mut line_nr: c_int = 0;
    let filename: *const c_char;

    unsafe { default_mtime = time(ptr::null_mut()) };
    loop {
        let opt = unsafe { getopt(argc, argv, c"t:cho:a:".as_ptr()) };
        let mut invalid: *mut c_char = ptr::null_mut();

        if opt == -1 {
            break;
        }
        match opt {
            x if x == b't' as c_int => {
                unsafe { default_mtime = strtol(optarg, &mut invalid, 10) };
                if unsafe { *optarg == 0 } || unsafe { *invalid != 0 } {
                    unsafe {
                        fprintf(stderr, c"Invalid timestamp: %s\n".as_ptr(), optarg);
                        usage(*argv);
                        exit(1);
                    }
                }
                unsafe { do_file_mtime = true };
            }
            x if x == b'c' as c_int => unsafe {
                do_csum = true;
            },
            x if x == b'o' as c_int => {
                unsafe {
                    outfd = open(optarg, O_WRONLY | O_CREAT | O_LARGEFILE | O_TRUNC, 0o600);
                    if outfd < 0 {
                        fprintf(stderr, c"failed to open %s\n".as_ptr(), optarg);
                        usage(*argv);
                        exit(1);
                    }
                }
            }
            x if x == b'a' as c_int => {
                unsafe { dalign = strtoul(optarg, &mut invalid, 10) as c_uint };
                if unsafe { *optarg == 0 } || unsafe { *invalid != 0 } || unsafe { (dalign & 3) != 0 } {
                    unsafe {
                        fprintf(stderr, c"Invalid data_align: %s\n".as_ptr(), optarg);
                        usage(*argv);
                        exit(1);
                    }
                }
            }
            x if x == b'h' as c_int || x == b'?' as c_int => unsafe {
                usage(*argv);
                exit(if opt == b'h' as c_int { 0 } else { 1 });
            },
            _ => {}
        }
    }

    /*
     * Timestamps after 2106-02-07 06:28:15 UTC have an ascii hex time_t
     * representation that exceeds 8 chars and breaks the cpio header
     * specification. Negative timestamps similarly exceed 8 chars.
     */
    if unsafe { default_mtime > 0xffffffff || default_mtime < 0 } {
        unsafe {
            fprintf(stderr, c"ERROR: Timestamp out of range for cpio format\n".as_ptr());
            exit(1);
        }
    }

    if argc - unsafe { optind } != 1 {
        unsafe {
            usage(*argv);
            exit(1);
        }
    }
    filename = unsafe { *argv.add(optind as usize) };
    if unsafe { strcmp(filename, c"-".as_ptr()) } == 0 {
        cpio_list = unsafe { stdin };
    } else {
        cpio_list = unsafe { fopen(filename, c"r".as_ptr()) };
        if cpio_list.is_null() {
            unsafe {
                fprintf(
                    stderr,
                    c"ERROR: unable to open '%s': %s\n\n".as_ptr(),
                    filename,
                    strerror(errno),
                );
                usage(*argv);
                exit(1);
            }
        }
    }

    while !unsafe { fgets(line.as_mut_ptr(), LINE_SIZE as c_int, cpio_list) }.is_null() {
        let mut type_idx: c_int;
        let slen = unsafe { strlen(line.as_ptr()) };

        line_nr += 1;

        if b'#' as c_char == line[0] {
            /* comment - skip to next line */
            continue;
        }

        type_ = unsafe { strtok(line.as_mut_ptr(), c" \t".as_ptr()) };
        if type_.is_null() {
            unsafe {
                fprintf(
                    stderr,
                    c"ERROR: incorrect format, could not locate file type line %d: '%s'\n".as_ptr(),
                    line_nr,
                    line.as_ptr(),
                )
            };
            ec = -1;
            break;
        }

        if b'\n' as c_char == unsafe { *type_ } {
            /* a blank line */
            continue;
        }

        if slen == unsafe { strlen(type_) } {
            /* must be an empty line */
            continue;
        }

        args = unsafe { strtok(ptr::null_mut(), c"\n".as_ptr()) };
        if args.is_null() {
            unsafe {
                fprintf(
                    stderr,
                    c"ERROR: incorrect format, newline required line %d: '%s'\n".as_ptr(),
                    line_nr,
                    line.as_ptr(),
                )
            };
            ec = -1;
        }

        type_idx = 0;
        while !file_handler_table[type_idx as usize].type_.is_null() {
            let mut rc: c_int;
            if unsafe { strcmp(line.as_ptr(), file_handler_table[type_idx as usize].type_) } == 0 {
                rc = unsafe { (file_handler_table[type_idx as usize].handler.unwrap())(args) };
                if rc != 0 {
                    ec = rc;
                    unsafe { fprintf(stderr, c" line %d\n".as_ptr(), line_nr) };
                }
                break;
            }
            type_idx += 1;
        }

        if file_handler_table[type_idx as usize].type_.is_null() {
            unsafe {
                fprintf(
                    stderr,
                    c"unknown file type line %d: '%s'\n".as_ptr(),
                    line_nr,
                    line.as_ptr(),
                )
            };
        }
    }
    if ec == 0 {
        ec = unsafe { cpio_trailer() };
    }

    unsafe { exit(ec) };
}

fn main() {
    let mut args: Vec<CString> = env::args()
        .map(|arg| CString::new(arg).unwrap_or_else(|_| CString::new("").unwrap()))
        .collect();
    let mut argv: Vec<*mut c_char> = args.iter_mut().map(|arg| arg.as_ptr() as *mut c_char).collect();
    argv.push(ptr::null_mut());
    unsafe {
        c_main((argv.len() - 1) as c_int, argv.as_mut_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
