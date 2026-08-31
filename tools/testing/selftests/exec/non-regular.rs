// SPDX-License-Identifier: GPL-2.0+
// Translated from testing/selftests/exec/non-regular.c.
// C dependencies: errno.h, fcntl.h, stdio.h, string.h, unistd.h,
// sys/socket.h, sys/stat.h, sys/sysmacros.h, sys/types.h,
// and "kselftest_harness.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type mode_t = c_uint;
type dev_t = c_ulong;

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn execv(path: *const c_char, argv: *const *const c_char) -> c_int;
    fn fexecve(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int;
    fn mkdir(pathname: *const c_char, mode: mode_t) -> c_int;
    fn mkfifo(pathname: *const c_char, mode: mode_t) -> c_int;
    fn mknod(pathname: *const c_char, mode: mode_t, dev: dev_t) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn symlink(target: *const c_char, linkpath: *const c_char) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;

    fn __errno_location() -> *mut c_int;
}

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const X_OK: c_int = 1;

const EPERM: c_int = 1;
const ENOENT: c_int = 2;
const EACCES: c_int = 13;
const ELOOP: c_int = 40;

const S_IFIFO: c_int = 0o010000;
const S_IFCHR: c_int = 0o020000;
const S_IFDIR: c_int = 0o040000;
const S_IFBLK: c_int = 0o060000;
const S_IFLNK: c_int = 0o120000;

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

const fn makedev(major: c_int, minor: c_int) -> dev_t {
    (((major as dev_t) & 0x00000fff) << 8)
        | (((major as dev_t) & 0xfffff000) << 32)
        | ((minor as dev_t) & 0x000000ff)
        | (((minor as dev_t) & 0xffffff00) << 12)
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_GT {
    ($left:expr, $right:expr) => {
        assert!($left > $right)
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

macro_rules! EXPECT_LT {
    ($left:expr, $right:expr) => {
        assert!($left < $right)
    };
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! TH_LOG {
    ($($arg:tt)*) => {
        eprintln!($($arg)*)
    };
}

macro_rules! SKIP {
    (return, $($arg:tt)*) => {{
        eprintln!($($arg)*);
        return;
    }};
}

/* Remove a file, ignoring the result if it didn't exist. */
pub unsafe fn rm(
    _metadata: *mut __test_metadata,
    pathname: *const c_char,
    is_dir: c_int,
) {
    let rc: c_int;

    if is_dir != 0 {
        rc = unsafe { rmdir(pathname) };
    } else {
        rc = unsafe { unlink(pathname) };
    }

    if rc < 0 {
        if unsafe { errno() } != ENOENT {
            TH_LOG!("Not ENOENT: {:?}", pathname);
        }
        ASSERT_EQ!(unsafe { errno() }, ENOENT);
    } else {
        if rc != 0 {
            TH_LOG!("Failed to remove: {:?}", pathname);
        }
        ASSERT_EQ!(rc, 0);
    }
}

#[repr(C)]
pub struct file {
    pathname: *mut c_char,
    is_dir: c_int,
}

#[repr(C)]
pub struct file_variant {
    name: *const c_char,
    expected: c_int,
    is_dir: c_int,
    setup: unsafe fn(*mut __test_metadata, *mut file, *const file_variant),
    major: c_int,
    minor: c_int,
    mode: c_int, /* for mknod() */
}

pub unsafe fn setup_link(
    _metadata: *mut __test_metadata,
    self_: *mut file,
    _variant: *const file_variant,
) {
    let paths: [*const c_char; 2] = [
        c"/bin/true".as_ptr(),
        c"/usr/bin/true".as_ptr(),
    ];
    let mut i: usize;

    i = 0;
    while i < paths.len() {
        if unsafe { access(paths[i], X_OK) } == 0 {
            ASSERT_EQ!(unsafe { symlink(paths[i], (*self_).pathname) }, 0);
            return;
        }
        i += 1;
    }
    TH_LOG!("Could not find viable 'true' binary");
    ASSERT_EQ!(1, 0);
}

pub static mut file_S_IFLNK: file_variant = file_variant {
    name: c"S_IFLNK".as_ptr(),
    expected: ELOOP,
    is_dir: 0,
    setup: setup_link,
    major: 0,
    minor: 0,
    mode: 0,
};

pub unsafe fn setup_dir(
    _metadata: *mut __test_metadata,
    self_: *mut file,
    _variant: *const file_variant,
) {
    ASSERT_EQ!(unsafe { mkdir((*self_).pathname, 0o755) }, 0);
}

pub static mut file_S_IFDIR: file_variant = file_variant {
    name: c"S_IFDIR".as_ptr(),
    is_dir: 1,
    expected: EACCES,
    setup: setup_dir,
    major: 0,
    minor: 0,
    mode: 0,
};

pub unsafe fn setup_node(
    _metadata: *mut __test_metadata,
    self_: *mut file,
    variant: *const file_variant,
) {
    let dev: dev_t;
    let rc: c_int;

    dev = makedev(unsafe { (*variant).major }, unsafe { (*variant).minor });
    rc = unsafe {
        mknod(
            (*self_).pathname,
            (0o755 | (*variant).mode) as mode_t,
            dev,
        )
    };
    if rc != 0 {
        if unsafe { errno() } == EPERM {
            SKIP!(
                return,
                "Please run as root; cannot mknod({:?})",
                unsafe { (*variant).name }
            );
        }
    }
    ASSERT_EQ!(rc, 0);
}

pub static mut file_S_IFBLK: file_variant = file_variant {
    name: c"S_IFBLK".as_ptr(),
    expected: EACCES,
    is_dir: 0,
    setup: setup_node,
    /* /dev/loop0 */
    major: 7,
    minor: 0,
    mode: S_IFBLK,
};

pub static mut file_S_IFCHR: file_variant = file_variant {
    name: c"S_IFCHR".as_ptr(),
    expected: EACCES,
    is_dir: 0,
    setup: setup_node,
    /* /dev/zero */
    major: 1,
    minor: 5,
    mode: S_IFCHR,
};

pub unsafe fn setup_fifo(
    _metadata: *mut __test_metadata,
    self_: *mut file,
    _variant: *const file_variant,
) {
    ASSERT_EQ!(unsafe { mkfifo((*self_).pathname, 0o755) }, 0);
}

pub static mut file_S_IFIFO: file_variant = file_variant {
    name: c"S_IFIFO".as_ptr(),
    expected: EACCES,
    is_dir: 0,
    setup: setup_fifo,
    major: 0,
    minor: 0,
    mode: 0,
};

pub unsafe fn file_setup(
    _metadata: *mut __test_metadata,
    self_: *mut file,
    variant: *const file_variant,
) {
    ASSERT_GT!(
        unsafe {
            asprintf(
                &mut (*self_).pathname,
                c"%s.test".as_ptr(),
                (*variant).name,
            )
        },
        6
    );
    unsafe {
        (*self_).is_dir = (*variant).is_dir;
    }

    unsafe { rm(_metadata, (*self_).pathname, (*variant).is_dir) };
    unsafe { ((*variant).setup)(_metadata, self_, variant) };
}

pub unsafe fn file_teardown(_metadata: *mut __test_metadata, self_: *mut file) {
    unsafe { rm(_metadata, (*self_).pathname, (*self_).is_dir) };
}

pub unsafe fn file_exec_errno(
    _metadata: *mut __test_metadata,
    self_: *mut file,
    variant: *const file_variant,
) {
    let argv: [*const c_char; 2] = [unsafe { (*self_).pathname }, ptr::null()];

    EXPECT_LT!(unsafe { execv(argv[0], argv.as_ptr()) }, 0);
    EXPECT_EQ!(unsafe { errno() }, unsafe { (*variant).expected });
}

/* S_IFSOCK */
#[repr(C)]
pub struct sock {
    fd: c_int,
}

pub unsafe fn sock_setup(_metadata: *mut __test_metadata, self_: *mut sock) {
    unsafe {
        (*self_).fd = socket(AF_INET, SOCK_STREAM, 0);
        ASSERT_GE!((*self_).fd, 0);
    }
}

pub unsafe fn sock_teardown(_metadata: *mut __test_metadata, self_: *mut sock) {
    unsafe {
        if (*self_).fd >= 0 {
            ASSERT_EQ!(close((*self_).fd), 0);
        }
    }
}

pub unsafe fn sock_exec_errno(_metadata: *mut __test_metadata, self_: *mut sock) {
    let argv: [*const c_char; 2] = [c" magic socket ".as_ptr(), ptr::null()];
    let envp: [*const c_char; 1] = [ptr::null()];

    EXPECT_LT!(unsafe { fexecve((*self_).fd, argv.as_ptr(), envp.as_ptr()) }, 0);
    EXPECT_EQ!(unsafe { errno() }, EACCES);
}

// TEST_HARNESS_MAIN
