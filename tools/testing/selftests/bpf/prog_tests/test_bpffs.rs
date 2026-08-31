// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/* Original C dependencies:
 * #define _GNU_SOURCE
 * #include <stdio.h>
 * #include <sched.h>
 * #include <sys/mount.h>
 * #include <sys/stat.h>
 * #include <sys/types.h>
 * #include <test_progs.h>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

/* TDIR must be in a location we can create a directory in. */
const TDIR: &str = "/tmp/test_bpffs_testdir";

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn unshare(flags: c_int) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: usize,
        data: *const c_void,
    ) -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat_t) -> c_int;
    fn renameat2(
        olddirfd: c_int,
        oldpath: *const c_char,
        newdirfd: c_int,
        newpath: *const c_char,
        flags: c_uint,
    ) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn umount(target: *const c_char) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn __errno_location() -> *mut c_int;

    fn bpf_map_create(
        map_type: c_uint,
        map_name: *const c_char,
        key_size: c_uint,
        value_size: c_uint,
        max_entries: c_uint,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_obj_pin(fd: c_int, pathname: *const c_char) -> c_int;

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(res: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: u64, expected: u64, name: *const c_char) -> bool;
    fn ASSERT_ERR(res: c_int, name: *const c_char) -> bool;
    fn CHECK(condition: bool, tag: *const c_char, fmt: *const c_char, ...) -> bool;
}

#[repr(C)]
struct stat_t {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: u64,
    st_size: i64,
    st_blksize: i64,
    st_blocks: i64,
    st_atime: i64,
    st_atime_nsec: i64,
    st_mtime: i64,
    st_mtime_nsec: i64,
    st_ctime: i64,
    st_ctime_nsec: i64,
    __glibc_reserved: [i64; 3],
}

type pid_t = c_int;

const CLONE_NEWNS: c_int = 0x0002_0000;
const MS_REC: usize = 16_384;
const MS_PRIVATE: usize = 1 << 18;
const EEXIST: c_int = 17;
const EINVAL: c_int = 22;
const ECHILD: c_int = 10;
const BPF_MAP_TYPE_ARRAY: c_uint = 2;
const RENAME_NOREPLACE: c_uint = 1;
const RENAME_EXCHANGE: c_uint = 2;
const F_OK: c_int = 0;

#[inline]
unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

#[inline]
fn c(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

#[inline]
fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn read_iter(file: *mut c_char) -> c_int {
    /* 1024 should be enough to get contiguous 4 "iter" letters at some point */
    let mut buf: [c_char; 1024] = [0; 1024];
    let mut len: c_int;

    let fd = unsafe { open(file, 0) };
    if fd < 0 {
        return -1;
    }
    loop {
        len = unsafe { read(fd, buf.as_mut_ptr() as *mut c_void, buf.len()) as c_int };
        if len <= 0 {
            break;
        }
        buf[buf.len() - 1] = b'\0' as c_char;
        if !unsafe { strstr(buf.as_ptr(), c(b"iter\0")) }.is_null() {
            unsafe {
                close(fd);
            }
            return 0;
        }
    }
    unsafe {
        close(fd);
    }
    -1
}

unsafe fn fn_() {
    let mut a: stat_t = unsafe { core::mem::zeroed() };
    let mut b: stat_t = unsafe { core::mem::zeroed() };
    let mut c_stat: stat_t = unsafe { core::mem::zeroed() };
    let mut err: c_int;
    let map: c_int;

    err = unsafe { unshare(CLONE_NEWNS) };
    if !unsafe { ASSERT_OK(err, c(b"unshare\0")) } {
        goto_out(err);
    }

    err = unsafe { mount(c(b"\0"), c(b"/\0"), c(b"\0"), MS_REC | MS_PRIVATE, core::ptr::null()) };
    if !unsafe { ASSERT_OK(err, c(b"mount /\0")) } {
        goto_out(err);
    }

    err = unsafe { mkdir(c(b"/tmp/test_bpffs_testdir\0"), 0o777) };
    /* If the directory already exists we can carry on. It may be left over
     * from a previous run.
     */
    if (err != 0 && unsafe { errno() } != EEXIST)
        && !unsafe { ASSERT_OK(err, c(b"mkdir /tmp/test_bpffs_testdir\0")) }
    {
        goto_out(err);
    }

    err = unsafe {
        mount(
            c(b"none\0"),
            c(b"/tmp/test_bpffs_testdir\0"),
            c(b"tmpfs\0"),
            0,
            core::ptr::null(),
        )
    };
    if !unsafe { ASSERT_OK(err, c(b"mount tmpfs\0")) } {
        goto_out(err);
    }

    err = unsafe { mkdir(c(b"/tmp/test_bpffs_testdir/fs1\0"), 0o777) };
    if !unsafe { ASSERT_OK(err, c(b"mkdir /tmp/test_bpffs_testdir/fs1\0")) } {
        goto_out(err);
    }
    err = unsafe { mkdir(c(b"/tmp/test_bpffs_testdir/fs2\0"), 0o777) };
    if !unsafe { ASSERT_OK(err, c(b"mkdir /tmp/test_bpffs_testdir/fs2\0")) } {
        goto_out(err);
    }

    err = unsafe {
        mount(
            c(b"bpf\0"),
            c(b"/tmp/test_bpffs_testdir/fs1\0"),
            c(b"bpf\0"),
            0,
            core::ptr::null(),
        )
    };
    if !unsafe { ASSERT_OK(err, c(b"mount bpffs /tmp/test_bpffs_testdir/fs1\0")) } {
        goto_out(err);
    }
    err = unsafe {
        mount(
            c(b"bpf\0"),
            c(b"/tmp/test_bpffs_testdir/fs2\0"),
            c(b"bpf\0"),
            0,
            core::ptr::null(),
        )
    };
    if !unsafe { ASSERT_OK(err, c(b"mount bpffs /tmp/test_bpffs_testdir/fs2\0")) } {
        goto_out(err);
    }

    err = unsafe { read_iter(c(b"/tmp/test_bpffs_testdir/fs1/maps.debug\0") as *mut c_char) };
    if !unsafe { ASSERT_OK(err, c(b"reading /tmp/test_bpffs_testdir/fs1/maps.debug\0")) } {
        goto_out(err);
    }
    err = unsafe { read_iter(c(b"/tmp/test_bpffs_testdir/fs2/progs.debug\0") as *mut c_char) };
    if !unsafe { ASSERT_OK(err, c(b"reading /tmp/test_bpffs_testdir/fs2/progs.debug\0")) } {
        goto_out(err);
    }

    err = unsafe { mkdir(c(b"/tmp/test_bpffs_testdir/fs1/a\0"), 0o777) };
    if !unsafe { ASSERT_OK(err, c(b"creating /tmp/test_bpffs_testdir/fs1/a\0")) } {
        goto_out(err);
    }
    err = unsafe { mkdir(c(b"/tmp/test_bpffs_testdir/fs1/a/1\0"), 0o777) };
    if !unsafe { ASSERT_OK(err, c(b"creating /tmp/test_bpffs_testdir/fs1/a/1\0")) } {
        goto_out(err);
    }
    err = unsafe { mkdir(c(b"/tmp/test_bpffs_testdir/fs1/b\0"), 0o777) };
    if !unsafe { ASSERT_OK(err, c(b"creating /tmp/test_bpffs_testdir/fs1/b\0")) } {
        goto_out(err);
    }

    map = unsafe { bpf_map_create(BPF_MAP_TYPE_ARRAY, core::ptr::null(), 4, 4, 1, core::ptr::null()) };
    if !unsafe { ASSERT_GT(map, 0, c(b"create_map(ARRAY)\0")) } {
        goto_out(err);
    }
    err = unsafe { bpf_obj_pin(map, c(b"/tmp/test_bpffs_testdir/fs1/c\0")) };
    if !unsafe { ASSERT_OK(err, c(b"pin map\0")) } {
        goto_out(err);
    }
    unsafe {
        close(map);
    }

    /* Check that RENAME_EXCHANGE works for directories. */
    err = unsafe { stat(c(b"/tmp/test_bpffs_testdir/fs1/a\0"), &mut a) };
    if !unsafe { ASSERT_OK(err, c(b"stat(/tmp/test_bpffs_testdir/fs1/a)\0")) } {
        goto_out(err);
    }
    err = unsafe {
        renameat2(
            0,
            c(b"/tmp/test_bpffs_testdir/fs1/a\0"),
            0,
            c(b"/tmp/test_bpffs_testdir/fs1/b\0"),
            RENAME_EXCHANGE,
        )
    };
    if !unsafe { ASSERT_OK(err, c(b"renameat2(/fs1/a, /fs1/b, RENAME_EXCHANGE)\0")) } {
        goto_out(err);
    }
    err = unsafe { stat(c(b"/tmp/test_bpffs_testdir/fs1/b\0"), &mut b) };
    if !unsafe { ASSERT_OK(err, c(b"stat(/tmp/test_bpffs_testdir/fs1/b)\0")) } {
        goto_out(err);
    }
    if !unsafe { ASSERT_EQ(a.st_ino, b.st_ino, c(b"b should have a's inode\0")) } {
        goto_out(err);
    }
    err = unsafe { access(c(b"/tmp/test_bpffs_testdir/fs1/b/1\0"), F_OK) };
    if !unsafe { ASSERT_OK(err, c(b"access(/tmp/test_bpffs_testdir/fs1/b/1)\0")) } {
        goto_out(err);
    }

    /* Check that RENAME_EXCHANGE works for mixed file types. */
    err = unsafe { stat(c(b"/tmp/test_bpffs_testdir/fs1/c\0"), &mut c_stat) };
    if !unsafe { ASSERT_OK(err, c(b"stat(/tmp/test_bpffs_testdir/fs1/map)\0")) } {
        goto_out(err);
    }
    err = unsafe {
        renameat2(
            0,
            c(b"/tmp/test_bpffs_testdir/fs1/c\0"),
            0,
            c(b"/tmp/test_bpffs_testdir/fs1/b\0"),
            RENAME_EXCHANGE,
        )
    };
    if !unsafe { ASSERT_OK(err, c(b"renameat2(/fs1/c, /fs1/b, RENAME_EXCHANGE)\0")) } {
        goto_out(err);
    }
    err = unsafe { stat(c(b"/tmp/test_bpffs_testdir/fs1/b\0"), &mut b) };
    if !unsafe { ASSERT_OK(err, c(b"stat(/tmp/test_bpffs_testdir/fs1/b)\0")) } {
        goto_out(err);
    }
    if !unsafe { ASSERT_EQ(c_stat.st_ino, b.st_ino, c(b"b should have c's inode\0")) } {
        goto_out(err);
    }
    err = unsafe { access(c(b"/tmp/test_bpffs_testdir/fs1/c/1\0"), F_OK) };
    if !unsafe { ASSERT_OK(err, c(b"access(/tmp/test_bpffs_testdir/fs1/c/1)\0")) } {
        goto_out(err);
    }

    /* Check that RENAME_NOREPLACE works. */
    err = unsafe {
        renameat2(
            0,
            c(b"/tmp/test_bpffs_testdir/fs1/b\0"),
            0,
            c(b"/tmp/test_bpffs_testdir/fs1/a\0"),
            RENAME_NOREPLACE,
        )
    };
    if !unsafe { ASSERT_ERR(err, c(b"renameat2(RENAME_NOREPLACE)\0")) } {
        err = -EINVAL;
        goto_out(err);
    }
    err = unsafe { access(c(b"/tmp/test_bpffs_testdir/fs1/b\0"), F_OK) };
    if !unsafe { ASSERT_OK(err, c(b"access(/tmp/test_bpffs_testdir/fs1/b)\0")) } {
        goto_out(err);
    }

    goto_out(err);
}

unsafe fn goto_out(err: c_int) -> ! {
    unsafe {
        umount(c(b"/tmp/test_bpffs_testdir/fs1\0"));
        umount(c(b"/tmp/test_bpffs_testdir/fs2\0"));
        rmdir(c(b"/tmp/test_bpffs_testdir/fs1\0"));
        rmdir(c(b"/tmp/test_bpffs_testdir/fs2\0"));
        umount(c(b"/tmp/test_bpffs_testdir\0"));
        rmdir(c(b"/tmp/test_bpffs_testdir\0"));
        exit(err);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_test_bpffs() {
    let mut err: c_int;
    let _duration: c_int = 0;
    let mut status: c_int = 0;
    let pid: pid_t;

    pid = unsafe { fork() };
    if unsafe {
        CHECK(
            pid == -1,
            c(b"clone\0"),
            c(b"clone failed %d\0"),
            errno(),
        )
    } {
        return;
    }
    if pid == 0 {
        unsafe {
            fn_();
        }
    }
    err = unsafe { waitpid(pid, &mut status, 0) };
    if unsafe {
        CHECK(
            err == -1 && errno() != ECHILD,
            c(b"waitpid\0"),
            c(b"failed %d\0"),
            errno(),
        )
    } {
        return;
    }
    if unsafe {
        CHECK(
            wexitstatus(status) != 0,
            c(b"bpffs test \0"),
            c(b"failed %d\0"),
            wexitstatus(status),
        )
    } {
        return;
    }
}
