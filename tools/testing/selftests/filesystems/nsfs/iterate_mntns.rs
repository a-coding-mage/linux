// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024 Christian Brauner <brauner@kernel.org>

// C source dependencies: _GNU_SOURCE, fcntl.h, linux/auto_dev-ioctl.h,
// linux/errno.h, sched.h, stdio.h, string.h, sys/stat.h, sys/mount.h,
// unistd.h, and kselftest_harness.h.

use core::ffi::{c_int, c_ulong, c_void};
use core::mem;
use core::ptr;

const MNT_NS_COUNT: usize = 11;
const MNT_NS_LAST_INDEX: usize = 10;

#[repr(C)]
struct mnt_ns_info {
    size: u32,
    nr_mounts: u32,
    mnt_ns_id: u64,
}

const MNT_NS_INFO_SIZE_VER0: usize = 16; /* size of first published struct */

const IOC_NRBITS: c_ulong = 8;
const IOC_TYPEBITS: c_ulong = 8;
const IOC_SIZEBITS: c_ulong = 14;

const IOC_NRSHIFT: c_ulong = 0;
const IOC_TYPESHIFT: c_ulong = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: c_ulong = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: c_ulong = IOC_SIZESHIFT + IOC_SIZEBITS;

const IOC_READ: c_ulong = 2;

const fn ioc(dir: c_ulong, ty: c_ulong, nr: c_ulong, size: c_ulong) -> c_ulong {
    (dir << IOC_DIRSHIFT) | (ty << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}

const fn ior<T>(ty: c_ulong, nr: c_ulong) -> c_ulong {
    ioc(IOC_READ, ty, nr, mem::size_of::<T>() as c_ulong)
}

/* Get information about namespace. */
const NS_MNT_GET_INFO: c_ulong = ior::<mnt_ns_info>(0xb7, 10);
/* Get next namespace. */
const NS_MNT_GET_NEXT: c_ulong = ior::<mnt_ns_info>(0xb7, 11);
/* Get previous namespace. */
const NS_MNT_GET_PREV: c_ulong = ior::<mnt_ns_info>(0xb7, 12);

const EBADF: c_int = 9;
const ENOENT: c_int = 2;
const ENOTTY: c_int = 25;

const CLONE_NEWNS: c_int = 0x0002_0000;
const F_DUPFD_CLOEXEC: c_int = 1030;
const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;

// From linux/auto_dev-ioctl.h.
const AUTOFS_DEV_IOCTL_VERSION_CMD: c_ulong = 0x71;
const AUTOFS_DEV_IOCTL_OPENMOUNT_CMD: c_ulong = 0x74;
const AUTOFS_DEV_IOCTL_CLOSEMOUNT_CMD: c_ulong = 0x75;
const AUTOFS_DEV_IOCTL_READY_CMD: c_ulong = 0x76;

const AUTOFS_IOCTL: c_ulong = 0x93;

#[repr(C)]
struct autofs_dev_ioctl {
    ver_major: u32,
    ver_minor: u32,
    size: u32,
    ioctlfd: i32,
    arg1: u32,
    arg2: u32,
    path: [u8; 0],
}

const AUTOFS_DEV_IOCTL_OPENMOUNT: c_ulong =
    ioc(IOC_READ, AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_OPENMOUNT_CMD, mem::size_of::<autofs_dev_ioctl>() as c_ulong);
const AUTOFS_DEV_IOCTL_CLOSEMOUNT: c_ulong =
    ioc(IOC_READ, AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_CLOSEMOUNT_CMD, mem::size_of::<autofs_dev_ioctl>() as c_ulong);
const AUTOFS_DEV_IOCTL_READY: c_ulong =
    ioc(IOC_READ, AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_READY_CMD, mem::size_of::<autofs_dev_ioctl>() as c_ulong);

#[repr(C)]
struct iterate_mount_namespaces {
    fd_mnt_ns: [c_int; MNT_NS_COUNT],
    mnt_ns_id: [u64; MNT_NS_COUNT],
}

unsafe extern "C" {
    fn unshare(flags: c_int) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn open(pathname: *const i8, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right)
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

#[inline]
unsafe fn mntns_in_list(mnt_ns_id: *mut u64, info: *mut mnt_ns_info) -> bool {
    for i in 0..MNT_NS_COUNT {
        if *mnt_ns_id.add(i) == (*info).mnt_ns_id {
            return true;
        }
    }
    false
}

unsafe fn iterate_mount_namespaces_setup(self_: *mut iterate_mount_namespaces) {
    for i in 0..MNT_NS_COUNT {
        (*self_).fd_mnt_ns[i] = -EBADF;
    }

    for i in 0..MNT_NS_COUNT {
        let mut info: mnt_ns_info = mem::zeroed();

        ASSERT_EQ!(unshare(CLONE_NEWNS), 0);
        (*self_).fd_mnt_ns[i] = open(c"/proc/self/ns/mnt".as_ptr(), O_RDONLY | O_CLOEXEC);
        ASSERT_GE!((*self_).fd_mnt_ns[i], 0);
        ASSERT_EQ!(ioctl((*self_).fd_mnt_ns[i], NS_MNT_GET_INFO, &mut info), 0);
        (*self_).mnt_ns_id[i] = info.mnt_ns_id;
    }
}

unsafe fn iterate_mount_namespaces_teardown(self_: *mut iterate_mount_namespaces) {
    for i in 0..MNT_NS_COUNT {
        if (*self_).fd_mnt_ns[i] < 0 {
            continue;
        }
        ASSERT_EQ!(close((*self_).fd_mnt_ns[i]), 0);
    }
}

unsafe fn iterate_mount_namespaces_iterate_all_forward(self_: *mut iterate_mount_namespaces) {
    let mut count: c_int = 0;

    let mut fd_mnt_ns_cur = fcntl((*self_).fd_mnt_ns[0], F_DUPFD_CLOEXEC);
    ASSERT_GE!(fd_mnt_ns_cur, 0);

    loop {
        let mut info: mnt_ns_info = mem::zeroed();

        let fd_mnt_ns_next = ioctl(fd_mnt_ns_cur, NS_MNT_GET_NEXT, &mut info);
        if fd_mnt_ns_next < 0 && errno() == ENOENT {
            break;
        }
        if mntns_in_list((*self_).mnt_ns_id.as_mut_ptr(), &mut info) {
            count += 1;
        }
        ASSERT_GE!(fd_mnt_ns_next, 0);
        ASSERT_EQ!(close(fd_mnt_ns_cur), 0);
        fd_mnt_ns_cur = fd_mnt_ns_next;
    }
    ASSERT_EQ!(count, MNT_NS_LAST_INDEX as c_int);
}

unsafe fn iterate_mount_namespaces_iterate_all_backwards(self_: *mut iterate_mount_namespaces) {
    let mut count: c_int = 0;

    let mut fd_mnt_ns_cur = fcntl((*self_).fd_mnt_ns[MNT_NS_LAST_INDEX], F_DUPFD_CLOEXEC);
    ASSERT_GE!(fd_mnt_ns_cur, 0);

    loop {
        let mut info: mnt_ns_info = mem::zeroed();

        let fd_mnt_ns_prev = ioctl(fd_mnt_ns_cur, NS_MNT_GET_PREV, &mut info);
        if fd_mnt_ns_prev < 0 && errno() == ENOENT {
            break;
        }
        if mntns_in_list((*self_).mnt_ns_id.as_mut_ptr(), &mut info) {
            count += 1;
        }
        ASSERT_GE!(fd_mnt_ns_prev, 0);
        ASSERT_EQ!(close(fd_mnt_ns_cur), 0);
        fd_mnt_ns_cur = fd_mnt_ns_prev;
    }
    ASSERT_EQ!(count, MNT_NS_LAST_INDEX as c_int);
}

unsafe fn iterate_mount_namespaces_iterate_forward(self_: *mut iterate_mount_namespaces) {
    ASSERT_EQ!(setns((*self_).fd_mnt_ns[0], CLONE_NEWNS), 0);

    let mut fd_mnt_ns_cur = (*self_).fd_mnt_ns[0];
    for _i in 1..MNT_NS_COUNT {
        let mut info: mnt_ns_info = mem::zeroed();

        let fd_mnt_ns_next = ioctl(fd_mnt_ns_cur, NS_MNT_GET_NEXT, &mut info);
        ASSERT_GE!(fd_mnt_ns_next, 0);
        ASSERT_EQ!(close(fd_mnt_ns_cur), 0);
        fd_mnt_ns_cur = fd_mnt_ns_next;
    }
}

unsafe fn iterate_mount_namespaces_iterate_backward(self_: *mut iterate_mount_namespaces) {
    ASSERT_EQ!(setns((*self_).fd_mnt_ns[MNT_NS_LAST_INDEX], CLONE_NEWNS), 0);

    let mut fd_mnt_ns_cur = (*self_).fd_mnt_ns[MNT_NS_LAST_INDEX];
    for _i in (0..MNT_NS_LAST_INDEX).rev() {
        let mut info: mnt_ns_info = mem::zeroed();

        let fd_mnt_ns_prev = ioctl(fd_mnt_ns_cur, NS_MNT_GET_PREV, &mut info);
        ASSERT_GE!(fd_mnt_ns_prev, 0);
        ASSERT_EQ!(close(fd_mnt_ns_cur), 0);
        fd_mnt_ns_cur = fd_mnt_ns_prev;
    }
}

unsafe fn iterate_mount_namespaces_nfs_valid_ioctl(self_: *mut iterate_mount_namespaces) {
    ASSERT_NE!(
        ioctl(
            (*self_).fd_mnt_ns[0],
            AUTOFS_DEV_IOCTL_OPENMOUNT,
            ptr::null_mut::<c_void>()
        ),
        0
    );
    ASSERT_EQ!(errno(), ENOTTY);

    ASSERT_NE!(
        ioctl(
            (*self_).fd_mnt_ns[0],
            AUTOFS_DEV_IOCTL_CLOSEMOUNT,
            ptr::null_mut::<c_void>()
        ),
        0
    );
    ASSERT_EQ!(errno(), ENOTTY);

    ASSERT_NE!(
        ioctl(
            (*self_).fd_mnt_ns[0],
            AUTOFS_DEV_IOCTL_READY,
            ptr::null_mut::<c_void>()
        ),
        0
    );
    ASSERT_EQ!(errno(), ENOTTY);
}

// TEST_HARNESS_MAIN
