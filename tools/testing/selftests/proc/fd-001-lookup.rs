/*
 * Copyright © 2018 Alexey Dobriyan <adobriyan@gmail.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
// Test /proc/*/fd lookup.

// Original C dependencies: assert.h, dirent.h, errno.h, limits.h, sched.h,
// stdio.h, unistd.h, sys/types.h, sys/stat.h, fcntl.h, and "proc.h".

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

const ENOENT: c_int = 2;
const INT_MIN: c_int = -2147483648;
const INT_MAX: c_int = 2147483647;
const UINT_MAX: c_uint = c_uint::MAX;
const CLONE_FILES: c_int = 0x00000400;
const O_DIRECTORY: c_int = 0o200000;
const O_PATH: c_int = 0o10000000;
const DT_DIR: u8 = 4;
const DT_LNK: u8 = 10;
const S_IFMT: u32 = 0o170000;
const S_IFLNK: u32 = 0o120000;

type SizeT = usize;
type SSizeT = isize;
type ModeT = u32;
type DevT = u64;
type InoT = u64;
type NlinkT = u64;
type UidT = u32;
type GidT = u32;
type OffT = i64;
type BlkcntT = i64;
type BlksizeT = i64;
type TimeT = i64;

#[repr(C)]
struct timespec {
    tv_sec: TimeT,
    tv_nsec: isize,
}

#[repr(C)]
struct stat {
    st_dev: DevT,
    st_ino: InoT,
    st_nlink: NlinkT,
    st_mode: ModeT,
    st_uid: UidT,
    st_gid: GidT,
    __pad0: c_int,
    st_rdev: DevT,
    st_size: OffT,
    st_blksize: BlksizeT,
    st_blocks: BlkcntT,
    st_atim: timespec,
    st_mtim: timespec,
    st_ctim: timespec,
    __glibc_reserved: [isize; 3],
}

#[repr(C)]
struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
struct dirent {
    d_ino: InoT,
    d_off: OffT,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 256],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn memset(s: *mut c_void, c: c_int, n: SizeT) -> *mut c_void;
    fn lstat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn snprintf(str: *mut c_char, size: SizeT, format: *const c_char, ...) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn dirfd(dirp: *mut DIR) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;

    fn xreaddir(dirp: *mut DIR) -> *mut dirent;
    fn xstrtoull(nptr: *const c_char, endptr: *mut *mut c_char) -> c_ulonglong;
    fn streq(s1: *const c_char, s2: *const c_char) -> bool;
}

fn S_ISLNK(m: ModeT) -> bool {
    (m & S_IFMT) == S_IFLNK
}

/* lstat(2) has more "coverage" in case non-symlink pops up somehow. */
unsafe fn test_lookup_pass(pathname: *const c_char) {
    let mut st: stat = core::mem::zeroed();
    let rv: SSizeT;

    memset(
        &mut st as *mut stat as *mut c_void,
        0,
        size_of::<stat>(),
    );
    rv = lstat(pathname, &mut st) as SSizeT;
    assert!(rv == 0);
    assert!(S_ISLNK(st.st_mode));
}

unsafe fn test_lookup_fail(pathname: *const c_char) {
    let mut st: stat = core::mem::zeroed();
    let rv: SSizeT;

    rv = lstat(pathname, &mut st) as SSizeT;
    assert!(rv == -1 && errno == ENOENT);
}

unsafe fn test_lookup(fd: c_uint) {
    let mut buf: [c_char; 64] = [0; 64];
    let mut c: c_uint;
    let mut u: c_uint;
    let mut i: c_int;

    snprintf(
        buf.as_mut_ptr(),
        size_of::<[c_char; 64]>(),
        c"/proc/self/fd/%u".as_ptr(),
        fd,
    );
    test_lookup_pass(buf.as_ptr());

    /* leading junk */
    c = 1;
    while c <= 255 {
        if c == b'/' as c_uint {
            c += 1;
            continue;
        }
        snprintf(
            buf.as_mut_ptr(),
            size_of::<[c_char; 64]>(),
            c"/proc/self/fd/%c%u".as_ptr(),
            c,
            fd,
        );
        test_lookup_fail(buf.as_ptr());
        c += 1;
    }

    /* trailing junk */
    c = 1;
    while c <= 255 {
        if c == b'/' as c_uint {
            c += 1;
            continue;
        }
        snprintf(
            buf.as_mut_ptr(),
            size_of::<[c_char; 64]>(),
            c"/proc/self/fd/%u%c".as_ptr(),
            fd,
            c,
        );
        test_lookup_fail(buf.as_ptr());
        c += 1;
    }

    i = INT_MIN;
    while i < INT_MIN + 1024 {
        snprintf(
            buf.as_mut_ptr(),
            size_of::<[c_char; 64]>(),
            c"/proc/self/fd/%d".as_ptr(),
            i,
        );
        test_lookup_fail(buf.as_ptr());
        i += 1;
    }
    i = -1024;
    while i < 0 {
        snprintf(
            buf.as_mut_ptr(),
            size_of::<[c_char; 64]>(),
            c"/proc/self/fd/%d".as_ptr(),
            i,
        );
        test_lookup_fail(buf.as_ptr());
        i += 1;
    }
    u = (INT_MAX as c_uint).wrapping_sub(1024);
    while u <= (INT_MAX as c_uint).wrapping_add(1024) {
        snprintf(
            buf.as_mut_ptr(),
            size_of::<[c_char; 64]>(),
            c"/proc/self/fd/%u".as_ptr(),
            u,
        );
        test_lookup_fail(buf.as_ptr());
        u = u.wrapping_add(1);
    }
    u = UINT_MAX.wrapping_sub(1024);
    while u != 0 {
        snprintf(
            buf.as_mut_ptr(),
            size_of::<[c_char; 64]>(),
            c"/proc/self/fd/%u".as_ptr(),
            u,
        );
        test_lookup_fail(buf.as_ptr());
        u = u.wrapping_add(1);
    }
}

unsafe fn c_main() -> c_int {
    let mut de: *mut dirent;
    let mut fd: c_uint;
    let mut target_fd: c_uint;

    if unshare(CLONE_FILES) == -1 {
        return 1;
    }

    /* Wipe fdtable. */
    loop {
        let d: *mut DIR;

        d = opendir(c"/proc/self/fd".as_ptr());
        if d.is_null() {
            return 1;
        }

        de = xreaddir(d);
        assert!((*de).d_type == DT_DIR);
        assert!(streq((*de).d_name.as_ptr(), c".".as_ptr()));

        de = xreaddir(d);
        assert!((*de).d_type == DT_DIR);
        assert!(streq((*de).d_name.as_ptr(), c"..".as_ptr()));

        loop {
            de = xreaddir(d);
            if !de.is_null() {
                let fd_ull: c_ulonglong;
                let fd: c_uint;
                let mut end: *mut c_char = ptr::null_mut();

                assert!((*de).d_type == DT_LNK);

                fd_ull = xstrtoull((*de).d_name.as_ptr(), &mut end);
                assert!(*end == b'\0' as c_char);
                assert!(fd_ull == fd_ull as c_uint as c_ulonglong);

                fd = fd_ull as c_uint;
                if fd == dirfd(d) as c_uint {
                    continue;
                }
                close(fd as c_int);
            }
            break;
        }

        closedir(d);
        if de.is_null() {
            break;
        }
    }

    /* Now fdtable is clean. */

    fd = open(c"/".as_ptr(), O_PATH | O_DIRECTORY) as c_uint;
    assert!(fd == 0);
    test_lookup(fd);
    close(fd as c_int);

    /* Clean again! */

    fd = open(c"/".as_ptr(), O_PATH | O_DIRECTORY) as c_uint;
    assert!(fd == 0);
    /* Default RLIMIT_NOFILE-1 */
    target_fd = 1023;
    while target_fd > 0 {
        if dup2(fd as c_int, target_fd as c_int) == target_fd as c_int {
            break;
        }
        target_fd /= 2;
    }
    assert!(target_fd > 0);
    close(fd as c_int);
    test_lookup(target_fd);
    close(target_fd as c_int);

    return 0;
}

fn main() {
    unsafe {
        std::process::exit(c_main());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
