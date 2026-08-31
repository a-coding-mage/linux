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
// Test that /proc/$KERNEL_THREAD/fd/ is empty.

// C dependencies: sys/syscall.h, assert.h, dirent.h, limits.h, stdio.h,
// string.h, sys/types.h, sys/stat.h, fcntl.h, unistd.h, and "proc.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulonglong, c_void};
use core::mem;
use core::ptr;

const PF_KHTREAD: c_uint = 0x00200000;

const O_RDONLY: c_int = 0;
const O_DIRECTORY: c_int = 0o200000;
const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
const DT_DIR: u8 = 4;
const ENOENT: c_int = 2;
const INT_MIN: c_int = c_int::MIN;
const INT_MAX: c_int = c_int::MAX;
const UINT_MAX: c_uint = c_uint::MAX;

// x86_64 Linux syscall number for statx, matching SYS_statx from sys/syscall.h.
const SYS_STATX: c_long = 332;

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: libc::ino_t,
    pub d_off: libc::off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

unsafe extern "C" {
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn fdopendir(fd: c_int) -> *mut DIR;
    fn syscall(num: c_long, ...) -> c_long;
    fn __errno_location() -> *mut c_int;

    fn xstrtoull(s: *const c_char, endptr: *mut *mut c_char) -> c_ulonglong;
    fn xreaddir(dirp: *mut DIR) -> *mut dirent;
    fn streq(s1: *const c_char, s2: *const c_char) -> bool;
}

#[inline]
unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

/*
 * Test for kernel threadness atomically with openat().
 *
 * Return /proc/$PID/fd descriptor if process is kernel thread.
 * Return -1 if a process is userspace process.
 */
unsafe fn kernel_thread_fd(pid: c_uint) -> c_int {
    let mut flags: c_uint = 0;
    let mut buf: [c_char; 4096] = [0; 4096];
    let dir_fd: c_int;
    let mut fd: c_int;
    let rv: isize;

    unsafe {
        snprintf(
            buf.as_mut_ptr(),
            mem::size_of_val(&buf),
            c"/proc/%u".as_ptr(),
            pid,
        );
        dir_fd = open(buf.as_ptr(), O_RDONLY | O_DIRECTORY);
    }
    if dir_fd == -1 {
        return -1;
    }

    /*
     * Believe it or not, struct task_struct::flags is directly exposed
     * to userspace!
     */
    unsafe {
        fd = openat(dir_fd, c"stat".as_ptr(), O_RDONLY);
    }
    if fd == -1 {
        unsafe {
            close(dir_fd);
        }
        return -1;
    }
    unsafe {
        rv = read(fd, buf.as_mut_ptr().cast::<c_void>(), mem::size_of_val(&buf));
        close(fd);
    }
    if 0 < rv && rv <= mem::size_of_val(&buf) as isize {
        let flags_ull: c_ulonglong;
        let mut p: *mut c_char;
        let mut end: *mut c_char = ptr::null_mut();
        let mut i: c_int;

        assert!(buf[(rv - 1) as usize] == b'\n' as c_char);
        buf[(rv - 1) as usize] = b'\0' as c_char;

        /* Search backwards: ->comm can contain whitespace and ')'. */
        i = 0;
        while i < 43 {
            unsafe {
                p = strrchr(buf.as_ptr(), b' ' as c_int);
            }
            assert!(!p.is_null());
            unsafe {
                *p = b'\0' as c_char;
            }
            i += 1;
        }

        unsafe {
            p = strrchr(buf.as_ptr(), b' ' as c_int);
        }
        assert!(!p.is_null());

        unsafe {
            flags_ull = xstrtoull(p.add(1), &mut end);
        }
        unsafe {
            assert!(*end == b'\0' as c_char);
        }
        assert!(flags_ull == flags_ull as c_uint as c_ulonglong);

        flags = flags_ull as c_uint;
    }

    fd = -1;
    if flags & PF_KHTREAD != 0 {
        unsafe {
            fd = openat(dir_fd, c"fd".as_ptr(), O_RDONLY | O_DIRECTORY);
        }
    }
    unsafe {
        close(dir_fd);
    }
    fd
}

unsafe fn test_readdir(fd: c_int) {
    let d: *mut DIR;
    let mut de: *mut dirent;

    unsafe {
        d = fdopendir(fd);
    }
    assert!(!d.is_null());

    unsafe {
        de = xreaddir(d);
    }
    assert!(unsafe { streq((*de).d_name.as_ptr(), c".".as_ptr()) });
    assert!(unsafe { (*de).d_type } == DT_DIR);

    unsafe {
        de = xreaddir(d);
    }
    assert!(unsafe { streq((*de).d_name.as_ptr(), c"..".as_ptr()) });
    assert!(unsafe { (*de).d_type } == DT_DIR);

    unsafe {
        de = xreaddir(d);
    }
    assert!(de.is_null());
}

#[inline]
unsafe fn sys_statx(
    dirfd: c_int,
    pathname: *const c_char,
    flags: c_int,
    mask: c_uint,
    stx: *mut c_void,
) -> c_int {
    unsafe { syscall(SYS_STATX, dirfd, pathname, flags, mask, stx) as c_int }
}

unsafe fn test_lookup_fail(fd: c_int, pathname: *const c_char) {
    #[repr(align(8))]
    struct AlignedStx([c_char; 256]);

    let mut stx = AlignedStx([0; 256]);
    let rv: c_int;

    unsafe {
        rv = sys_statx(
            fd,
            pathname,
            AT_SYMLINK_NOFOLLOW,
            0,
            stx.0.as_mut_ptr().cast::<c_void>(),
        );
    }
    assert!(rv == -1 && unsafe { errno() } == ENOENT);
}

unsafe fn test_lookup(fd: c_int) {
    let mut buf: [c_char; 64] = [0; 64];
    let mut u: c_uint;
    let mut i: c_int;

    i = INT_MIN;
    while i < INT_MIN.wrapping_add(1024) {
        unsafe {
            snprintf(
                buf.as_mut_ptr(),
                mem::size_of_val(&buf),
                c"%d".as_ptr(),
                i,
            );
            test_lookup_fail(fd, buf.as_ptr());
        }
        i = i.wrapping_add(1);
    }
    i = -1024;
    while i < 1024 {
        unsafe {
            snprintf(
                buf.as_mut_ptr(),
                mem::size_of_val(&buf),
                c"%d".as_ptr(),
                i,
            );
            test_lookup_fail(fd, buf.as_ptr());
        }
        i += 1;
    }
    u = (INT_MAX as c_uint).wrapping_sub(1024);
    while u < (INT_MAX as c_uint).wrapping_add(1024) {
        unsafe {
            snprintf(
                buf.as_mut_ptr(),
                mem::size_of_val(&buf),
                c"%u".as_ptr(),
                u,
            );
            test_lookup_fail(fd, buf.as_ptr());
        }
        u = u.wrapping_add(1);
    }
    u = UINT_MAX.wrapping_sub(1024);
    while u != 0 {
        unsafe {
            snprintf(
                buf.as_mut_ptr(),
                mem::size_of_val(&buf),
                c"%u".as_ptr(),
                u,
            );
            test_lookup_fail(fd, buf.as_ptr());
        }
        u = u.wrapping_add(1);
    }
}

fn main() {
    let mut pid: c_uint;
    let mut fd: c_int;

    /*
     * In theory this will loop indefinitely if kernel threads are exiled
     * from /proc.
     *
     * Start with kthreadd.
     */
    pid = 2;
    unsafe {
        fd = kernel_thread_fd(pid);
    }
    while fd == -1 && pid < 1024 {
        pid += 1;
        unsafe {
            fd = kernel_thread_fd(pid);
        }
    }
    /* EACCES if run as non-root. */
    if pid >= 1024 {
        std::process::exit(1);
    }

    unsafe {
        test_readdir(fd);
        test_lookup(fd);
    }
}
