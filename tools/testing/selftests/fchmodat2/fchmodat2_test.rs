// SPDX-License-Identifier: GPL-2.0-or-later

// C source dependencies: _GNU_SOURCE, fcntl.h, sys/stat.h, sys/types.h,
// syscall.h, unistd.h, and "kselftest.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type mode_t = c_uint;

const O_PATH: c_int = 0o10000000;
const O_DIRECTORY: c_int = 0o200000;
const O_CREAT: c_int = 0o100;
const O_WRONLY: c_int = 0o1;
const O_TRUNC: c_int = 0o1000;
const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
const __NR_fchmodat2: c_long = 452;

const NUM_TESTS: c_uint = 2;

#[repr(C)]
struct stat {
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

#[repr(C)]
struct testdir {
    dirname: *mut c_char,
    dfd: c_int,
}

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn __errno_location() -> *mut c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn symlinkat(target: *const c_char, newdirfd: c_int, linkpath: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn unlinkat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
    fn fstatat(dirfd: c_int, pathname: *const c_char, statbuf: *mut stat, flags: c_int) -> c_int;

    fn ksft_exit_fail_msg(fmt: *const c_char, ...);
    fn ksft_perror(msg: *const c_char);
    fn ksft_exit_fail() -> !;
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result(condition: bool, fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_finished() -> !;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn sys_fchmodat2(
    dfd: c_int,
    filename: *const c_char,
    mode: mode_t,
    flags: c_int,
) -> c_int {
    let ret = unsafe { syscall(__NR_fchmodat2, dfd, filename, mode, flags) };

    if ret >= 0 {
        ret as c_int
    } else {
        -unsafe { errno() }
    }
}

unsafe fn setup_testdir(testdir: *mut testdir) {
    let mut ret: c_int;
    let dfd: c_int;
    let mut dirname = *b"/tmp/ksft-fchmodat2.XXXXXX\0";

    /* Make the top-level directory. */
    if unsafe { mkdtemp(dirname.as_mut_ptr() as *mut c_char) }.is_null() {
        unsafe {
            ksft_exit_fail_msg(
                c"%s: failed to create tmpdir\n".as_ptr(),
                c"setup_testdir".as_ptr(),
            );
        }
    }

    dfd = unsafe { open(dirname.as_ptr() as *const c_char, O_PATH | O_DIRECTORY) };
    if dfd < 0 {
        unsafe { ksft_perror(c"failed to open tmpdir".as_ptr()) };
        goto_err(testdir, dirname.as_ptr() as *const c_char);
    }

    ret = unsafe { openat(dfd, c"regfile".as_ptr(), O_CREAT | O_WRONLY | O_TRUNC, 0o644 as mode_t) };
    if ret < 0 {
        unsafe { ksft_perror(c"failed to create file in tmpdir".as_ptr()) };
        goto_err(testdir, dirname.as_ptr() as *const c_char);
    }
    unsafe { close(ret) };

    ret = unsafe { symlinkat(c"regfile".as_ptr(), dfd, c"symlink".as_ptr()) };
    if ret < 0 {
        unsafe { ksft_perror(c"symlinkat() failed".as_ptr()) };
        goto_err_regfile(testdir, dfd, dirname.as_ptr() as *const c_char);
    }

    unsafe {
        (*testdir).dirname = strdup(dirname.as_ptr() as *const c_char);
    }
    if unsafe { (*testdir).dirname }.is_null() {
        unsafe { ksft_perror(c"Out of memory".as_ptr()) };
        goto_err_symlink(testdir, dfd, dirname.as_ptr() as *const c_char);
    }
    unsafe {
        (*testdir).dfd = dfd;
    }

    return;

    unsafe fn goto_err_symlink(testdir: *mut testdir, dfd: c_int, dirname: *const c_char) -> ! {
        unsafe {
            (*testdir).dfd = dfd;
            unlinkat((*testdir).dfd, c"symlink".as_ptr(), 0);
        }
        goto_err_regfile(testdir, dfd, dirname);
    }

    unsafe fn goto_err_regfile(testdir: *mut testdir, dfd: c_int, dirname: *const c_char) -> ! {
        unsafe {
            (*testdir).dfd = dfd;
            unlinkat((*testdir).dfd, c"regfile".as_ptr(), 0);
        }
        goto_err(testdir, dirname);
    }

    unsafe fn goto_err(_testdir: *mut testdir, dirname: *const c_char) -> ! {
        unsafe {
            unlink(dirname);
            ksft_exit_fail();
        }
    }
}

unsafe fn cleanup_testdir(testdir: *mut testdir) {
    unsafe {
        unlinkat((*testdir).dfd, c"regfile".as_ptr(), 0);
        unlinkat((*testdir).dfd, c"symlink".as_ptr(), 0);
        rmdir((*testdir).dirname);
        free((*testdir).dirname as *mut c_void);
    }
}

unsafe fn expect_mode(dfd: c_int, filename: *const c_char, expect_mode: mode_t) -> c_int {
    let mut st: stat = unsafe { core::mem::zeroed() };
    let ret = unsafe { fstatat(dfd, filename, &mut st, AT_SYMLINK_NOFOLLOW) };

    if ret != 0 {
        unsafe { ksft_perror(c"fstatat() failed\n".as_ptr()) };
        return 0;
    }

    (st.st_mode == expect_mode) as c_int
}

unsafe fn test_regfile() {
    let mut testdir: testdir = unsafe { core::mem::zeroed() };
    let mut ret: c_int;

    unsafe { setup_testdir(&mut testdir) };

    ret = unsafe { sys_fchmodat2(testdir.dfd, c"regfile".as_ptr(), 0o640, 0) };

    if ret < 0 {
        unsafe { ksft_perror(c"fchmodat2(noflag) failed".as_ptr()) };
        unsafe { ksft_test_result(ret == 0, c"fchmodat2(regfile)\n".as_ptr()) };
        unsafe { cleanup_testdir(&mut testdir) };
        return;
    }

    if unsafe { expect_mode(testdir.dfd, c"regfile".as_ptr(), 0o100640) } == 0 {
        unsafe {
            ksft_print_msg(
                c"%s: wrong file mode bits after fchmodat2\n".as_ptr(),
                c"test_regfile".as_ptr(),
            );
        }
        ret = 1;
        unsafe { ksft_test_result(ret == 0, c"fchmodat2(regfile)\n".as_ptr()) };
        unsafe { cleanup_testdir(&mut testdir) };
        return;
    }

    ret = unsafe {
        sys_fchmodat2(
            testdir.dfd,
            c"regfile".as_ptr(),
            0o600,
            AT_SYMLINK_NOFOLLOW,
        )
    };

    if ret < 0 {
        unsafe { ksft_perror(c"fchmodat2(AT_SYMLINK_NOFOLLOW) failed".as_ptr()) };
        unsafe { ksft_test_result(ret == 0, c"fchmodat2(regfile)\n".as_ptr()) };
        unsafe { cleanup_testdir(&mut testdir) };
        return;
    }

    if unsafe { expect_mode(testdir.dfd, c"regfile".as_ptr(), 0o100600) } == 0 {
        unsafe {
            ksft_print_msg(
                c"%s: wrong file mode bits after fchmodat2 with nofollow\n".as_ptr(),
                c"test_regfile".as_ptr(),
            );
        }
        ret = 1;
    }

    unsafe { ksft_test_result(ret == 0, c"fchmodat2(regfile)\n".as_ptr()) };
    unsafe { cleanup_testdir(&mut testdir) };
}

unsafe fn test_symlink() {
    let mut testdir: testdir = unsafe { core::mem::zeroed() };
    let mut ret: c_int;

    unsafe { setup_testdir(&mut testdir) };

    ret = unsafe { sys_fchmodat2(testdir.dfd, c"symlink".as_ptr(), 0o640, 0) };

    if ret < 0 {
        unsafe { ksft_perror(c"fchmodat2(noflag) failed".as_ptr()) };
        unsafe { ksft_test_result_fail(c"fchmodat2(symlink)\n".as_ptr()) };
        unsafe { cleanup_testdir(&mut testdir) };
        return;
    }

    if unsafe { expect_mode(testdir.dfd, c"regfile".as_ptr(), 0o100640) } == 0 {
        unsafe {
            ksft_print_msg(
                c"%s: wrong file mode bits after fchmodat2\n".as_ptr(),
                c"test_symlink".as_ptr(),
            );
        }
        unsafe { ksft_test_result_fail(c"fchmodat2(symlink)\n".as_ptr()) };
        unsafe { cleanup_testdir(&mut testdir) };
        return;
    }

    if unsafe { expect_mode(testdir.dfd, c"symlink".as_ptr(), 0o120777) } == 0 {
        unsafe {
            ksft_print_msg(
                c"%s: wrong symlink mode bits after fchmodat2\n".as_ptr(),
                c"test_symlink".as_ptr(),
            );
        }
        unsafe { ksft_test_result_fail(c"fchmodat2(symlink)\n".as_ptr()) };
        unsafe { cleanup_testdir(&mut testdir) };
        return;
    }

    ret = unsafe {
        sys_fchmodat2(
            testdir.dfd,
            c"symlink".as_ptr(),
            0o600,
            AT_SYMLINK_NOFOLLOW,
        )
    };

    /*
     * On certain filesystems (xfs or btrfs), chmod operation fails. So we
     * first check the symlink target but if the operation fails we mark the
     * test as skipped.
     *
     * https://sourceware.org/legacy-ml/libc-alpha/2020-02/msg00467.html
     */
    if ret == 0 && unsafe { expect_mode(testdir.dfd, c"symlink".as_ptr(), 0o120600) } == 0 {
        unsafe {
            ksft_print_msg(
                c"%s: wrong symlink mode bits after fchmodat2 with nofollow\n".as_ptr(),
                c"test_symlink".as_ptr(),
            );
        }
        ret = 1;
        unsafe { ksft_test_result_fail(c"fchmodat2(symlink)\n".as_ptr()) };
        unsafe { cleanup_testdir(&mut testdir) };
        return;
    }

    if unsafe { expect_mode(testdir.dfd, c"regfile".as_ptr(), 0o100640) } == 0 {
        unsafe {
            ksft_print_msg(
                c"%s: wrong file mode bits after fchmodat2 with nofollow\n".as_ptr(),
                c"test_symlink".as_ptr(),
            );
        }
    }

    if ret != 0 {
        unsafe { ksft_test_result_skip(c"fchmodat2(symlink)\n".as_ptr()) };
    } else {
        unsafe { ksft_test_result_pass(c"fchmodat2(symlink)\n".as_ptr()) };
    }
    unsafe { cleanup_testdir(&mut testdir) };
}

unsafe fn main_impl(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    unsafe {
        ksft_print_header();
        ksft_set_plan(NUM_TESTS);

        test_regfile();
        test_symlink();

        ksft_finished();
    }
}

fn main() {
    unsafe {
        main_impl(0, core::ptr::null_mut());
    }
}
