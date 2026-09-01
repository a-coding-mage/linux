// SPDX-License-Identifier: GPL-2.0

// C source included: errno.h, fcntl.h, sched.h, sys/mount.h, sys/stat.h,
// unistd.h, and ../../kselftest_harness.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

const CLONE_NEWNS: c_int = 0x0002_0000;
const ENOENT: c_int = 2;
const MS_PRIVATE: c_ulong = 1 << 18;
const MS_REC: c_ulong = 16_384;
const O_CLOEXEC: c_int = 0o2000000;
const O_DIRECTORY: c_int = 0o200000;
const O_PATH: c_int = 0o10000000;
const O_RDONLY: c_int = 0;

unsafe extern "C" {
    fn close(fd: c_int) -> c_int;
    fn geteuid() -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_int) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn unshare(flags: c_int) -> c_int;
}

unsafe extern "C" {
    static mut errno: c_int;
}

#[repr(C)]
pub struct mntns_cleanup {}

pub unsafe fn mntns_cleanup_setup() {
    if unsafe { geteuid() } != 0 {
        SKIP!(return, "test requires CAP_SYS_ADMIN");
    }

    ASSERT_EQ!(unsafe { unshare(CLONE_NEWNS) }, 0);
    ASSERT_EQ!(
        unsafe {
            mount(
                c"".as_ptr(),
                c"/".as_ptr(),
                ptr::null(),
                MS_REC | MS_PRIVATE,
                ptr::null(),
            )
        },
        0
    );

    unsafe {
        rmdir(c"/mnt_dir".as_ptr());
    }
    ASSERT_EQ!(unsafe { mkdir(c"/mnt_dir".as_ptr(), 0o755) }, 0);
    ASSERT_EQ!(
        unsafe {
            mount(
                c"tmpfs".as_ptr(),
                c"/mnt_dir".as_ptr(),
                c"tmpfs".as_ptr(),
                0,
                ptr::null(),
            )
        },
        0
    );
    ASSERT_EQ!(unsafe { mkdir(c"/mnt_dir/hidden".as_ptr(), 0o755) }, 0);
    ASSERT_EQ!(
        unsafe { mkdir(c"/mnt_dir/hidden/secret".as_ptr(), 0o755) },
        0
    );
    ASSERT_EQ!(
        unsafe {
            mount(
                c"tmpfs".as_ptr(),
                c"/mnt_dir/hidden".as_ptr(),
                c"tmpfs".as_ptr(),
                0,
                ptr::null(),
            )
        },
        0
    );
}

pub unsafe fn mntns_cleanup_teardown() {}

/* Mounts must stay connected when a mount namespace is cleaned up. */
pub unsafe fn mntns_cleanup_keeps_mounts_connected() {
    let fd: c_int;
    let sfd: c_int;
    let err: c_int;

    fd = unsafe { open(c"/mnt_dir".as_ptr(), O_PATH | O_DIRECTORY | O_CLOEXEC) };
    ASSERT_GE!(fd, 0);

    /* Destroy the namespace; the fd keeps /mnt_dir alive. */
    ASSERT_EQ!(unsafe { unshare(CLONE_NEWNS) }, 0);

    sfd = unsafe { openat(fd, c"hidden/secret".as_ptr(), O_RDONLY) };
    err = unsafe { errno };
    if sfd >= 0 {
        unsafe {
            close(sfd);
        }
    }
    unsafe {
        close(fd);
    }

    ASSERT_LT!(sfd, 0);
    TH_LOG!("mount namespace teardown revealed what the overmount covered");
    ASSERT_EQ!(err, ENOENT);
}

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
