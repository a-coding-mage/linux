// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/path.c. Original C dependencies:
// "path.h", "cache.h", <linux/kernel.h>, and standard C/POSIX headers.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void, VaList};

type size_t = usize;
type mode_t = c_uint;

const PATH_MAX: usize = 4096;
const S_IFMT: mode_t = 0o170000;
const S_IFREG: mode_t = 0o100000;
const S_IFDIR: mode_t = 0o040000;

#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: mode_t,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad0: c_int,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
    pub __unused: [i64; 3],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

unsafe extern "C" {
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn vsnprintf(s: *mut c_char, n: size_t, format: *const c_char, arg: VaList<'_, '_>) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn fstatat(dirfd: c_int, pathname: *const c_char, statbuf: *mut stat, flags: c_int) -> c_int;
}

#[inline]
fn S_ISREG(mode: mode_t) -> bool {
    (mode & S_IFMT) == S_IFREG
}

#[inline]
fn S_ISDIR(mode: mode_t) -> bool {
    (mode & S_IFMT) == S_IFDIR
}

unsafe fn cleanup_path(mut path: *mut c_char) -> *mut c_char {
    /* Clean it up */
    if memcmp(
        path as *const c_void,
        c"./".as_ptr() as *const c_void,
        2,
    ) == 0
    {
        path = path.add(2);
        while *path == b'/' as c_char {
            path = path.add(1);
        }
    }
    path
}

pub unsafe extern "C" fn mkpath(
    path_buf: *mut c_char,
    sz: size_t,
    fmt: *const c_char,
    mut args: ...
) -> *mut c_char {
    let len: c_uint;

    let args = args.as_va_list();
    len = vsnprintf(path_buf, sz, fmt, args) as c_uint;
    if len >= sz as c_uint {
        strncpy(path_buf, c"/bad-path/".as_ptr(), sz);
    }
    cleanup_path(path_buf)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path__join(
    bf: *mut c_char,
    size: size_t,
    path1: *const c_char,
    path2: *const c_char,
) -> c_int {
    scnprintf(
        bf,
        size,
        c"%s%s%s".as_ptr(),
        path1,
        if *path1 != 0 {
            c"/".as_ptr()
        } else {
            c"".as_ptr()
        },
        path2,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path__join3(
    bf: *mut c_char,
    size: size_t,
    path1: *const c_char,
    path2: *const c_char,
    path3: *const c_char,
) -> c_int {
    scnprintf(
        bf,
        size,
        c"%s%s%s%s%s".as_ptr(),
        path1,
        if *path1 != 0 {
            c"/".as_ptr()
        } else {
            c"".as_ptr()
        },
        path2,
        if *path2 != 0 {
            c"/".as_ptr()
        } else {
            c"".as_ptr()
        },
        path3,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_regular_file(file: *const c_char) -> bool {
    let mut st: stat = core::mem::zeroed();

    if stat(file, &mut st) != 0 {
        return false;
    }

    S_ISREG(st.st_mode)
}

/* Helper function for filesystems that return a dent->d_type DT_UNKNOWN */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_directory(base_path: *const c_char, dent: *const dirent) -> bool {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut st: stat = core::mem::zeroed();

    snprintf(
        path.as_mut_ptr(),
        core::mem::size_of_val(&path),
        c"%s/%s".as_ptr(),
        base_path,
        (*dent).d_name.as_ptr(),
    );
    if stat(path.as_ptr(), &mut st) != 0 {
        return false;
    }

    S_ISDIR(st.st_mode)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_directory_at(dir_fd: c_int, path: *const c_char) -> bool {
    let mut st: stat = core::mem::zeroed();

    if fstatat(dir_fd, path, &mut st, /*flags=*/ 0) != 0 {
        return false;
    }

    S_ISDIR(st.st_mode)
}
