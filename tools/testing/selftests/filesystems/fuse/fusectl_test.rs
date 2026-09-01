// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2025 Chen Linxuan <chenlinxuan@uniontech.com>

// C source defined _GNU_SOURCE before including libc/system headers.
// External test harness dependency: kselftest_harness.h

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::mem;
use std::ptr;

const FUSECTL_MOUNTPOINT: &str = "/sys/fs/fuse/connections";
const FUSE_MOUNTPOINT: &str = "/tmp/fuse_mnt_XXXXXX";
const FUSE_DEVICE: &str = "/dev/fuse";
const FUSECTL_TEST_VALUE: &str = "1";

const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const F_OK: c_int = 0;
const SEEK_SET: c_int = 0;
const ENOTCONN: c_int = 107;
const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWUSER: c_int = 0x10000000;
const MS_REC: usize = 16384;
const MS_PRIVATE: usize = 1 << 18;
const MNT_DETACH: c_int = 2;
const PATH_MAX: usize = 4096;

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat {
    st_dev: u64,
    __rest: [u8; 136],
}

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn unshare(flags: c_int) -> c_int;
    fn getuid() -> u32;
    fn getgid() -> u32;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: usize,
        data: *const c_void,
    ) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn fork() -> c_int;
    fn execlp(file: *const c_char, arg: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn waitpid(pid: c_int, wstatus: *mut c_int, options: c_int) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn lseek(fd: c_int, offset: isize, whence: c_int) -> isize;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

macro_rules! assert_eq_c {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! assert_ge_c {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

macro_rules! assert_gt_c {
    ($left:expr, $right:expr) => {
        assert!($left > $right)
    };
}

macro_rules! skip_return {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        return;
    }};
}

unsafe fn cstr(ptr: *const c_char) -> String {
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

static FUSECTL_MOUNTPOINT_C: &[u8] = b"/sys/fs/fuse/connections\0";
static FUSE_MOUNTPOINT_C: &[u8] = b"/tmp/fuse_mnt_XXXXXX\0";
static FUSECTL_TEST_VALUE_C: &[u8] = b"1\0";

static PROC_SELF_UID_MAP: &[u8] = b"/proc/self/uid_map\0";
static PROC_SELF_SETGROUPS: &[u8] = b"/proc/self/setgroups\0";
static PROC_SELF_GID_MAP: &[u8] = b"/proc/self/gid_map\0";
static DENY: &[u8] = b"deny\0";
static ROOT: &[u8] = b"/\0";
static EMPTY: &[u8] = b"\0";

unsafe fn write_file(_metadata: *mut __test_metadata, path: *const c_char, val: *const c_char) {
    let fd: c_int = unsafe { open(path, O_WRONLY) };
    let len: usize = unsafe { strlen(val) };

    assert_ge_c!(fd, 0);
    assert_eq_c!(unsafe { write(fd, val as *const c_void, len) }, len as isize);
    assert_eq_c!(unsafe { close(fd) }, 0);
}

#[repr(C)]
struct fusectl {
    fuse_mountpoint: [c_char; FUSE_MOUNTPOINT.len() + 1],
    connection: c_int,
}

unsafe fn fusectl_setup(_metadata: *mut __test_metadata, self_: *mut fusectl) {
    let fuse_mnt_prog = CString::new("./fuse_mnt").unwrap();
    let mut status: c_int = 0;
    let mut pid: c_int;
    let mut statbuf: stat = unsafe { mem::zeroed() };
    let uid: u32 = unsafe { getuid() };
    let gid: u32 = unsafe { getgid() };
    let mut buf: [c_char; 32] = [0; 32];
    let map_fmt = CString::new("0 %d 1").unwrap();

    /* Setup userns */
    assert_eq_c!(unsafe { unshare(CLONE_NEWNS | CLONE_NEWUSER) }, 0);
    unsafe {
        sprintf(buf.as_mut_ptr(), map_fmt.as_ptr(), uid);
    }
    unsafe { write_file(_metadata, PROC_SELF_UID_MAP.as_ptr() as *const c_char, buf.as_ptr()) };
    unsafe {
        write_file(
            _metadata,
            PROC_SELF_SETGROUPS.as_ptr() as *const c_char,
            DENY.as_ptr() as *const c_char,
        )
    };
    unsafe {
        sprintf(buf.as_mut_ptr(), map_fmt.as_ptr(), gid);
    }
    unsafe { write_file(_metadata, PROC_SELF_GID_MAP.as_ptr() as *const c_char, buf.as_ptr()) };
    assert_eq_c!(
        unsafe {
            mount(
                EMPTY.as_ptr() as *const c_char,
                ROOT.as_ptr() as *const c_char,
                ptr::null(),
                MS_REC | MS_PRIVATE,
                ptr::null(),
            )
        },
        0
    );

    unsafe {
        strcpy(
            (*self_).fuse_mountpoint.as_mut_ptr(),
            FUSE_MOUNTPOINT_C.as_ptr() as *const c_char,
        );
    }

    if unsafe { mkdtemp((*self_).fuse_mountpoint.as_mut_ptr()) }.is_null() {
        skip_return!(
            "Failed to create FUSE mountpoint {}",
            unsafe { cstr(strerror(errno())) }
        );
    }

    if unsafe { access(FUSECTL_MOUNTPOINT_C.as_ptr() as *const c_char, F_OK) } != 0 {
        skip_return!("FUSE control filesystem not mounted");
    }

    pid = unsafe { fork() };
    if pid < 0 {
        skip_return!(
            "Failed to fork FUSE daemon process: {}",
            unsafe { cstr(strerror(errno())) }
        );
    }

    if pid == 0 {
        unsafe {
            execlp(
                fuse_mnt_prog.as_ptr(),
                fuse_mnt_prog.as_ptr(),
                (*self_).fuse_mountpoint.as_ptr(),
                ptr::null::<c_char>(),
            );
            exit(errno());
        }
    }

    unsafe {
        waitpid(pid, &mut status, 0);
    }
    if !wifexited(status) || wexitstatus(status) != 0 {
        skip_return!(
            "Failed to start FUSE daemon {}",
            unsafe { cstr(strerror(wexitstatus(status))) }
        );
    }

    if unsafe { stat((*self_).fuse_mountpoint.as_ptr(), &mut statbuf) } != 0 {
        skip_return!(
            "Failed to stat FUSE mountpoint {}",
            unsafe { cstr(strerror(errno())) }
        );
    }

    unsafe {
        (*self_).connection = statbuf.st_dev as c_int;
    }
}

unsafe fn fusectl_teardown(_metadata: *mut __test_metadata, self_: *mut fusectl) {
    unsafe {
        umount2((*self_).fuse_mountpoint.as_ptr(), MNT_DETACH);
        rmdir((*self_).fuse_mountpoint.as_ptr());
    }
}

unsafe fn fusectl_abort(_metadata: *mut __test_metadata, self_: *mut fusectl) {
    let mut path_buf: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut abort_fd: c_int;
    let mut test_fd: c_int;
    let mut ret: isize;
    let abort_fmt = CString::new("/sys/fs/fuse/connections/%d/abort").unwrap();
    let test_fmt = CString::new("%s/test").unwrap();
    let write_value = CString::new("test").unwrap();

    unsafe {
        sprintf(
            path_buf.as_mut_ptr(),
            abort_fmt.as_ptr(),
            (*self_).connection,
        );
    }

    assert_eq_c!(0, unsafe { access(path_buf.as_ptr(), F_OK) });

    abort_fd = unsafe { open(path_buf.as_ptr(), O_WRONLY) };
    assert_ge_c!(abort_fd, 0);

    unsafe {
        sprintf(
            path_buf.as_mut_ptr(),
            test_fmt.as_ptr(),
            (*self_).fuse_mountpoint.as_ptr(),
        );
    }

    test_fd = unsafe { open(path_buf.as_ptr(), O_RDWR) };
    assert_ge_c!(test_fd, 0);

    ret = unsafe { read(test_fd, path_buf.as_mut_ptr() as *mut c_void, path_buf.len()) };
    assert_eq_c!(ret, 0);

    ret = unsafe {
        write(
            test_fd,
            write_value.as_ptr() as *const c_void,
            b"test\0".len(),
        )
    };
    assert_eq_c!(ret, b"test\0".len() as isize);

    ret = unsafe { lseek(test_fd, 0, SEEK_SET) };
    assert_ge_c!(ret, 0);

    ret = unsafe {
        write(
            abort_fd,
            FUSECTL_TEST_VALUE_C.as_ptr() as *const c_void,
            FUSECTL_TEST_VALUE_C.len(),
        )
    };
    assert_gt_c!(ret, 0);

    unsafe {
        close(abort_fd);
    }

    ret = unsafe { read(test_fd, path_buf.as_mut_ptr() as *mut c_void, path_buf.len()) };
    assert_eq_c!(ret, -1);
    assert_eq_c!(unsafe { errno() }, ENOTCONN);
}

fn main() {
    // TEST_HARNESS_MAIN
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
