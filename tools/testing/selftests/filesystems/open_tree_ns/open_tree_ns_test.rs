// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2026 Christian Brauner <brauner@kernel.org>
 *
 * Test for OPEN_TREE_NAMESPACE flag.
 *
 * Test that open_tree() with OPEN_TREE_NAMESPACE creates a new mount
 * namespace containing the specified mount tree.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

type bool_ = bool;
type pid_t = c_int;
type ssize_t = isize;
type uint64_t = u64;

const OPEN_TREE_NAMESPACE: c_uint = 1 << 1;

unsafe extern "C" {
    static mut errno: c_int;

    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn fork() -> pid_t;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn _exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn geteuid() -> c_uint;
    fn unshare(flags: c_int) -> c_int;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;

    fn sys_open_tree(dfd: c_int, filename: *const c_char, flags: c_uint) -> c_int;
    fn statmount(
        mnt_id: uint64_t,
        mnt_ns_id: uint64_t,
        mask: uint64_t,
        flags: c_uint,
        buf: *mut c_void,
        bufsize: usize,
        spare: c_uint,
    ) -> c_int;
    fn listmount(
        mnt_id: uint64_t,
        mnt_ns_id: uint64_t,
        request_mask: uint64_t,
        list: *mut uint64_t,
        num: usize,
        flags: c_uint,
    ) -> ssize_t;
    fn statmount_alloc(
        mnt_id: uint64_t,
        mnt_ns_id: uint64_t,
        mask: uint64_t,
        flags: c_uint,
    ) -> *mut statmount_t;
    fn enter_userns() -> c_int;
    fn caps_down() -> c_int;
    fn sys_mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
}

#[repr(C)]
struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
struct statmount_t {
    size: u32,
    mask: u64,
    sb_dev_major: u32,
    sb_dev_minor: u32,
    sb_magic: u64,
    sb_flags: u32,
    fs_type: u32,
    mnt_id: u64,
    mnt_parent_id: u64,
    mnt_id_old: u32,
    mnt_parent_id_old: u32,
    mnt_attr: u64,
    mnt_propagation: u64,
    mnt_peer_group: u64,
    mnt_master: u64,
    propagate_from: u64,
    mnt_root: u32,
    mnt_point: u32,
    str_: [c_char; 0],
}

const O_RDONLY: c_int = 0;
const ENOSYS: c_int = 38;
const EINVAL: c_int = 22;
const EPERM: c_int = 1;
const AT_FDCWD: c_int = -100;
const AT_RECURSIVE: c_uint = 0x8000;
const OPEN_TREE_CLOEXEC: c_uint = 0x80000;
const CLONE_NEWNS: c_int = 0x00020000;
const MS_SLAVE: c_ulong = 1 << 19;
const MS_REC: c_ulong = 16384;
const MS_UNBINDABLE: c_ulong = 1 << 17;
const MNT_DETACH: c_int = 2;
const PATH_MAX: usize = 4096;
const NS_GET_MNTNS_ID: c_ulong = 0xb704;
const LSMT_ROOT: uint64_t = 0xffff_ffff_ffff_ffff;
const LISTMOUNT_REVERSE: c_uint = 1 << 0;
const STATMOUNT_MNT_BASIC: uint64_t = 0x0000_0001;
const STATMOUNT_FS_TYPE: uint64_t = 0x0000_0002;
const STATMOUNT_MNT_ROOT: uint64_t = 0x0000_0004;
const STATMOUNT_MNT_POINT: uint64_t = 0x0000_0008;

macro_rules! C {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn get_mnt_ns_id(fd: c_int, mnt_ns_id: *mut uint64_t) -> c_int {
    if ioctl(fd, NS_GET_MNTNS_ID, mnt_ns_id) < 0 {
        return -errno;
    }
    0
}

unsafe fn get_mnt_ns_id_from_path(path: *const c_char, mnt_ns_id: *mut uint64_t) -> c_int {
    let fd: c_int;
    let ret: c_int;

    fd = open(path, O_RDONLY);
    if fd < 0 {
        return -errno;
    }

    ret = get_mnt_ns_id(fd, mnt_ns_id);
    close(fd);
    ret
}

unsafe fn log_mount(_metadata: *mut __test_metadata, sm: *mut statmount_t) {
    let mut fs_type: *const c_char = C!("");
    let mut mnt_root: *const c_char = C!("");
    let mut mnt_point: *const c_char = C!("");

    if (*sm).mask & STATMOUNT_FS_TYPE != 0 {
        fs_type = (*sm).str_.as_ptr().add((*sm).fs_type as usize);
    }
    if (*sm).mask & STATMOUNT_MNT_ROOT != 0 {
        mnt_root = (*sm).str_.as_ptr().add((*sm).mnt_root as usize);
    }
    if (*sm).mask & STATMOUNT_MNT_POINT != 0 {
        mnt_point = (*sm).str_.as_ptr().add((*sm).mnt_point as usize);
    }

    TH_LOG!(
        "  mnt_id: %llu, parent_id: %llu, fs_type: %s, root: %s, point: %s",
        (*sm).mnt_id as c_ulong,
        (*sm).mnt_parent_id as c_ulong,
        fs_type,
        mnt_root,
        mnt_point
    );
}

unsafe fn dump_mounts(_metadata: *mut __test_metadata, mnt_ns_id: uint64_t) {
    let mut list: [uint64_t; 256] = [0; 256];
    let nr_mounts: ssize_t;

    nr_mounts = listmount(LSMT_ROOT, mnt_ns_id, 0, list.as_mut_ptr(), 256, 0);
    if nr_mounts < 0 {
        TH_LOG!("listmount failed: %s", strerror(errno));
        return;
    }

    TH_LOG!(
        "Mount namespace %llu contains %zd mount(s):",
        mnt_ns_id as c_ulong,
        nr_mounts
    );

    for i in 0..nr_mounts {
        let sm: *mut statmount_t;

        sm = statmount_alloc(
            list[i as usize],
            mnt_ns_id,
            STATMOUNT_MNT_BASIC | STATMOUNT_FS_TYPE | STATMOUNT_MNT_ROOT | STATMOUNT_MNT_POINT,
            0,
        );
        if sm.is_null() {
            TH_LOG!(
                "  [%zd] mnt_id %llu: statmount failed: %s",
                i,
                list[i as usize] as c_ulong,
                strerror(errno)
            );
            continue;
        }

        log_mount(_metadata, sm);
        free(sm as *mut c_void);
    }
}

#[repr(C)]
struct open_tree_ns {
    fd: c_int,
    current_ns_id: uint64_t,
}

#[repr(C)]
struct open_tree_ns_variant {
    path: *const c_char,
    flags: c_uint,
    expect_success: bool_,
    expect_different_ns: bool_,
    min_mounts: c_int,
}

FIXTURE_VARIANT_ADD!(
    open_tree_ns,
    basic_root,
    open_tree_ns_variant {
        path: C!("/"),
        flags: OPEN_TREE_NAMESPACE | OPEN_TREE_CLOEXEC,
        expect_success: true,
        expect_different_ns: true,
        /*
         * The empty rootfs is hidden from listmount()/mountinfo,
         * so we only see the bind mount on top of it.
         */
        min_mounts: 1,
    }
);

FIXTURE_VARIANT_ADD!(
    open_tree_ns,
    recursive_root,
    open_tree_ns_variant {
        path: C!("/"),
        flags: OPEN_TREE_NAMESPACE | AT_RECURSIVE | OPEN_TREE_CLOEXEC,
        expect_success: true,
        expect_different_ns: true,
        min_mounts: 1,
    }
);

FIXTURE_VARIANT_ADD!(
    open_tree_ns,
    subdir_tmp,
    open_tree_ns_variant {
        path: C!("/tmp"),
        flags: OPEN_TREE_NAMESPACE | OPEN_TREE_CLOEXEC,
        expect_success: true,
        expect_different_ns: true,
        min_mounts: 1,
    }
);

FIXTURE_VARIANT_ADD!(
    open_tree_ns,
    subdir_proc,
    open_tree_ns_variant {
        path: C!("/proc"),
        flags: OPEN_TREE_NAMESPACE | OPEN_TREE_CLOEXEC,
        expect_success: true,
        expect_different_ns: true,
        min_mounts: 1,
    }
);

FIXTURE_VARIANT_ADD!(
    open_tree_ns,
    recursive_tmp,
    open_tree_ns_variant {
        path: C!("/tmp"),
        flags: OPEN_TREE_NAMESPACE | AT_RECURSIVE | OPEN_TREE_CLOEXEC,
        expect_success: true,
        expect_different_ns: true,
        min_mounts: 1,
    }
);

FIXTURE_VARIANT_ADD!(
    open_tree_ns,
    recursive_run,
    open_tree_ns_variant {
        path: C!("/run"),
        flags: OPEN_TREE_NAMESPACE | AT_RECURSIVE | OPEN_TREE_CLOEXEC,
        expect_success: true,
        expect_different_ns: true,
        min_mounts: 1,
    }
);

FIXTURE_VARIANT_ADD!(
    open_tree_ns,
    invalid_recursive_alone,
    open_tree_ns_variant {
        path: C!("/"),
        flags: AT_RECURSIVE | OPEN_TREE_CLOEXEC,
        expect_success: false,
        expect_different_ns: false,
        min_mounts: 0,
    }
);

unsafe fn open_tree_ns_setup(self_: *mut open_tree_ns) {
    let ret: c_int;

    (*self_).fd = -1;

    /* Check if open_tree syscall is supported */
    ret = sys_open_tree(-1, core::ptr::null(), 0);
    if ret == -1 && errno == ENOSYS {
        SKIP!(return, "open_tree() syscall not supported");
    }

    /* Check if statmount/listmount are supported */
    ret = statmount(0, 0, 0, 0, core::ptr::null_mut(), 0, 0);
    if ret == -1 && errno == ENOSYS {
        SKIP!(return, "statmount() syscall not supported");
    }

    /* Get current mount namespace ID for comparison */
    ret = get_mnt_ns_id_from_path(C!("/proc/self/ns/mnt"), &mut (*self_).current_ns_id);
    if ret < 0 {
        SKIP!(return, "Failed to get current mount namespace ID");
    }
}

unsafe fn open_tree_ns_teardown(self_: *mut open_tree_ns) {
    if (*self_).fd >= 0 {
        close((*self_).fd);
    }
}

unsafe fn open_tree_ns_create_namespace(
    _metadata: *mut __test_metadata,
    self_: *mut open_tree_ns,
    variant: *const open_tree_ns_variant,
) {
    let mut new_ns_id: uint64_t = 0;
    let mut list: [uint64_t; 256] = [0; 256];
    let nr_mounts: ssize_t;
    let ret: c_int;

    (*self_).fd = sys_open_tree(AT_FDCWD, (*variant).path, (*variant).flags);

    if !(*variant).expect_success {
        ASSERT_LT!((*self_).fd, 0);
        ASSERT_EQ!(errno, EINVAL);
        return;
    }

    if (*self_).fd < 0 && errno == EINVAL {
        SKIP!(return, "OPEN_TREE_NAMESPACE not supported");
    }

    ASSERT_GE!((*self_).fd, 0);

    /* Verify we can get the namespace ID */
    ret = get_mnt_ns_id((*self_).fd, &mut new_ns_id);
    ASSERT_EQ!(ret, 0);

    /* Verify it's a different namespace */
    if (*variant).expect_different_ns {
        ASSERT_NE!(new_ns_id, (*self_).current_ns_id);
    }

    /* List mounts in the new namespace */
    nr_mounts = listmount(LSMT_ROOT, new_ns_id, 0, list.as_mut_ptr(), 256, 0);
    ASSERT_GE!(nr_mounts, 0, {
        TH_LOG!("%m - listmount failed");
    });

    /* Verify minimum expected mounts */
    ASSERT_GE!(nr_mounts, (*variant).min_mounts as ssize_t);
    TH_LOG!("Namespace contains %zd mounts", nr_mounts);
}

unsafe fn open_tree_ns_setns_into_namespace(
    _metadata: *mut __test_metadata,
    self_: *mut open_tree_ns,
    variant: *const open_tree_ns_variant,
) {
    let mut new_ns_id: uint64_t = 0;
    let pid: pid_t;
    let mut status: c_int = 0;
    let ret: c_int;

    /* Only test with basic flags */
    if (*variant).flags & OPEN_TREE_NAMESPACE == 0 {
        SKIP!(return, "setns test only for basic / case");
    }

    (*self_).fd = sys_open_tree(AT_FDCWD, (*variant).path, (*variant).flags);
    if (*self_).fd < 0 && errno == EINVAL {
        SKIP!(return, "OPEN_TREE_NAMESPACE not supported");
    }

    ASSERT_GE!((*self_).fd, 0);

    /* Get namespace ID and dump all mounts */
    ret = get_mnt_ns_id((*self_).fd, &mut new_ns_id);
    ASSERT_EQ!(ret, 0);

    dump_mounts(_metadata, new_ns_id);

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        /* Child: try to enter the namespace */
        if setns((*self_).fd, CLONE_NEWNS) < 0 {
            _exit(1);
        }
        _exit(0);
    }

    ASSERT_EQ!(waitpid(pid, &mut status, 0), pid);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);
}

unsafe fn open_tree_ns_verify_mount_properties(
    _metadata: *mut __test_metadata,
    self_: *mut open_tree_ns,
    variant: *const open_tree_ns_variant,
) {
    let mut sm: statmount_t = core::mem::zeroed();
    let mut new_ns_id: uint64_t = 0;
    let mut list: [uint64_t; 256] = [0; 256];
    let nr_mounts: ssize_t;
    let ret: c_int;

    /* Only test with basic flags on root */
    if (*variant).flags != (OPEN_TREE_NAMESPACE | OPEN_TREE_CLOEXEC) || strcmp((*variant).path, C!("/")) != 0 {
        SKIP!(return, "mount properties test only for basic / case");
    }

    (*self_).fd = sys_open_tree(AT_FDCWD, C!("/"), OPEN_TREE_NAMESPACE | OPEN_TREE_CLOEXEC);
    if (*self_).fd < 0 && errno == EINVAL {
        SKIP!(return, "OPEN_TREE_NAMESPACE not supported");
    }

    ASSERT_GE!((*self_).fd, 0);

    ret = get_mnt_ns_id((*self_).fd, &mut new_ns_id);
    ASSERT_EQ!(ret, 0);

    nr_mounts = listmount(LSMT_ROOT, new_ns_id, 0, list.as_mut_ptr(), 256, 0);
    ASSERT_GE!(nr_mounts, 1);

    /* Get info about the root mount (the bind mount, rootfs is hidden) */
    ret = statmount(
        list[0],
        new_ns_id,
        0,
        STATMOUNT_MNT_BASIC as c_uint,
        &mut sm as *mut _ as *mut c_void,
        core::mem::size_of::<statmount_t>(),
        0,
    );
    ASSERT_EQ!(ret, 0);

    ASSERT_NE!(sm.mnt_id, sm.mnt_parent_id);

    TH_LOG!(
        "Root mount id: %llu, parent: %llu",
        sm.mnt_id as c_ulong,
        sm.mnt_parent_id as c_ulong
    );
}

#[repr(C)]
struct open_tree_ns_caps {
    has_caps: bool_,
}

unsafe fn open_tree_ns_caps_setup(self_: *mut open_tree_ns_caps) {
    let ret: c_int;

    /* Check if open_tree syscall is supported */
    ret = sys_open_tree(-1, core::ptr::null(), 0);
    if ret == -1 && errno == ENOSYS {
        SKIP!(return, "open_tree() syscall not supported");
    }

    (*self_).has_caps = geteuid() == 0;
}

unsafe fn open_tree_ns_caps_teardown(_self: *mut open_tree_ns_caps) {}

unsafe fn open_tree_ns_caps_requires_cap_sys_admin(
    _metadata: *mut __test_metadata,
    _self: *mut open_tree_ns_caps,
) {
    let pid: pid_t;
    let mut status: c_int = 0;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let fd: c_int;

        /* Child: drop privileges using utils.h helper */
        if enter_userns() != 0 {
            _exit(2);
        }

        /* Drop all caps using utils.h helper */
        if caps_down() == 0 {
            _exit(3);
        }

        fd = sys_open_tree(AT_FDCWD, C!("/"), OPEN_TREE_NAMESPACE | OPEN_TREE_CLOEXEC);
        if fd >= 0 {
            close(fd);
            /* Should have failed without caps */
            _exit(1);
        }

        if errno == EPERM {
            _exit(0);
        }

        /* EINVAL means OPEN_TREE_NAMESPACE not supported */
        if errno == EINVAL {
            _exit(4);
        }

        /* Unexpected error */
        _exit(5);
    }

    ASSERT_EQ!(waitpid(pid, &mut status, 0), pid);
    ASSERT_TRUE!(WIFEXITED(status));

    match WEXITSTATUS(status) {
        0 => {
            /* Expected: EPERM without caps */
        }
        1 => {
            ASSERT_FALSE!(true);
            TH_LOG!("OPEN_TREE_NAMESPACE succeeded without caps");
        }
        2 => {
            SKIP!(return, "setup_userns failed");
        }
        3 => {
            SKIP!(return, "caps_down failed");
        }
        4 => {
            SKIP!(return, "OPEN_TREE_NAMESPACE not supported");
        }
        _ => {
            ASSERT_FALSE!(true);
            TH_LOG!("Unexpected error in child (exit %d)", WEXITSTATUS(status));
        }
    }
}

#[repr(C)]
struct open_tree_ns_userns {
    fd: c_int,
}

unsafe fn open_tree_ns_userns_setup(self_: *mut open_tree_ns_userns) {
    let ret: c_int;

    (*self_).fd = -1;

    /* Check if open_tree syscall is supported */
    ret = sys_open_tree(-1, core::ptr::null(), 0);
    if ret == -1 && errno == ENOSYS {
        SKIP!(return, "open_tree() syscall not supported");
    }

    /* Check if statmount/listmount are supported */
    ret = statmount(0, 0, 0, 0, core::ptr::null_mut(), 0, 0);
    if ret == -1 && errno == ENOSYS {
        SKIP!(return, "statmount() syscall not supported");
    }
}

unsafe fn open_tree_ns_userns_teardown(self_: *mut open_tree_ns_userns) {
    if (*self_).fd >= 0 {
        close((*self_).fd);
    }
}

unsafe fn open_tree_ns_userns_create_in_userns(
    _metadata: *mut __test_metadata,
    _self: *mut open_tree_ns_userns,
) {
    let pid: pid_t;
    let mut status: c_int = 0;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut new_ns_id: uint64_t = 0;
        let mut list: [uint64_t; 256] = [0; 256];
        let nr_mounts: ssize_t;
        let fd: c_int;

        /* Create new user namespace (also creates mount namespace) */
        if enter_userns() != 0 {
            _exit(2);
        }

        /* Now we have CAP_SYS_ADMIN in the user namespace */
        fd = sys_open_tree(AT_FDCWD, C!("/"), OPEN_TREE_NAMESPACE | OPEN_TREE_CLOEXEC);
        if fd < 0 {
            if errno == EINVAL {
                _exit(4); /* OPEN_TREE_NAMESPACE not supported */
            }
            _exit(1);
        }

        /* Verify we can get the namespace ID */
        if get_mnt_ns_id(fd, &mut new_ns_id) != 0 {
            _exit(5);
        }

        /* Verify we can list mounts in the new namespace */
        nr_mounts = listmount(LSMT_ROOT, new_ns_id, 0, list.as_mut_ptr(), 256, 0);
        if nr_mounts < 0 {
            _exit(6);
        }

        /* Should have at least 1 mount */
        if nr_mounts < 1 {
            _exit(7);
        }

        close(fd);
        _exit(0);
    }

    ASSERT_EQ!(waitpid(pid, &mut status, 0), pid);
    ASSERT_TRUE!(WIFEXITED(status));

    match WEXITSTATUS(status) {
        0 => {
            /* Success */
        }
        1 => {
            ASSERT_FALSE!(true);
            TH_LOG!("open_tree(OPEN_TREE_NAMESPACE) failed in userns");
        }
        2 => {
            SKIP!(return, "setup_userns failed");
        }
        4 => {
            SKIP!(return, "OPEN_TREE_NAMESPACE not supported");
        }
        5 => {
            ASSERT_FALSE!(true);
            TH_LOG!("Failed to get mount namespace ID");
        }
        6 => {
            ASSERT_FALSE!(true);
            TH_LOG!("listmount failed in new namespace");
        }
        7 => {
            ASSERT_FALSE!(true);
            TH_LOG!("New namespace has no mounts");
        }
        _ => {
            ASSERT_FALSE!(true);
            TH_LOG!("Unexpected error in child (exit %d)", WEXITSTATUS(status));
        }
    }
}

unsafe fn open_tree_ns_userns_setns_in_userns(
    _metadata: *mut __test_metadata,
    _self: *mut open_tree_ns_userns,
) {
    let pid: pid_t;
    let mut status: c_int = 0;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut new_ns_id: uint64_t = 0;
        let fd: c_int;
        let inner_pid: pid_t;
        let mut inner_status: c_int = 0;

        /* Create new user namespace */
        if enter_userns() != 0 {
            _exit(2);
        }

        fd = sys_open_tree(AT_FDCWD, C!("/"), OPEN_TREE_NAMESPACE | OPEN_TREE_CLOEXEC);
        if fd < 0 {
            if errno == EINVAL {
                _exit(4);
            }
            _exit(1);
        }

        if get_mnt_ns_id(fd, &mut new_ns_id) != 0 {
            _exit(5);
        }

        /* Fork again to test setns into the new namespace */
        inner_pid = fork();
        if inner_pid < 0 {
            _exit(8);
        }

        if inner_pid == 0 {
            /* Inner child: enter the new namespace */
            if setns(fd, CLONE_NEWNS) < 0 {
                _exit(1);
            }
            _exit(0);
        }

        if waitpid(inner_pid, &mut inner_status, 0) != inner_pid {
            _exit(9);
        }

        if !WIFEXITED(inner_status) || WEXITSTATUS(inner_status) != 0 {
            _exit(10);
        }

        close(fd);
        _exit(0);
    }

    ASSERT_EQ!(waitpid(pid, &mut status, 0), pid);
    ASSERT_TRUE!(WIFEXITED(status));

    match WEXITSTATUS(status) {
        0 => {
            /* Success */
        }
        1 => {
            ASSERT_FALSE!(true);
            TH_LOG!("open_tree or setns failed in userns");
        }
        2 => {
            SKIP!(return, "setup_userns failed");
        }
        4 => {
            SKIP!(return, "OPEN_TREE_NAMESPACE not supported");
        }
        5 => {
            ASSERT_FALSE!(true);
            TH_LOG!("Failed to get mount namespace ID");
        }
        8 => {
            ASSERT_FALSE!(true);
            TH_LOG!("Inner fork failed");
        }
        9 => {
            ASSERT_FALSE!(true);
            TH_LOG!("Inner waitpid failed");
        }
        10 => {
            ASSERT_FALSE!(true);
            TH_LOG!("setns into new namespace failed");
        }
        _ => {
            ASSERT_FALSE!(true);
            TH_LOG!("Unexpected error in child (exit %d)", WEXITSTATUS(status));
        }
    }
}

unsafe fn open_tree_ns_userns_recursive_in_userns(
    _metadata: *mut __test_metadata,
    _self: *mut open_tree_ns_userns,
) {
    let pid: pid_t;
    let mut status: c_int = 0;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut new_ns_id: uint64_t = 0;
        let mut list: [uint64_t; 256] = [0; 256];
        let nr_mounts: ssize_t;
        let fd: c_int;

        /* Create new user namespace */
        if enter_userns() != 0 {
            _exit(2);
        }

        /* Test recursive flag in userns */
        fd = sys_open_tree(
            AT_FDCWD,
            C!("/"),
            OPEN_TREE_NAMESPACE | AT_RECURSIVE | OPEN_TREE_CLOEXEC,
        );
        if fd < 0 {
            if errno == EINVAL {
                _exit(4);
            }
            _exit(1);
        }

        if get_mnt_ns_id(fd, &mut new_ns_id) != 0 {
            _exit(5);
        }

        nr_mounts = listmount(LSMT_ROOT, new_ns_id, 0, list.as_mut_ptr(), 256, 0);
        if nr_mounts < 0 {
            _exit(6);
        }

        /* Recursive should copy submounts too */
        if nr_mounts < 1 {
            _exit(7);
        }

        close(fd);
        _exit(0);
    }

    ASSERT_EQ!(waitpid(pid, &mut status, 0), pid);
    ASSERT_TRUE!(WIFEXITED(status));

    match WEXITSTATUS(status) {
        0 => {
            /* Success */
        }
        1 => {
            ASSERT_FALSE!(true);
            TH_LOG!("open_tree(OPEN_TREE_NAMESPACE|AT_RECURSIVE) failed in userns");
        }
        2 => {
            SKIP!(return, "setup_userns failed");
        }
        4 => {
            SKIP!(return, "OPEN_TREE_NAMESPACE not supported");
        }
        5 => {
            ASSERT_FALSE!(true);
            TH_LOG!("Failed to get mount namespace ID");
        }
        6 => {
            ASSERT_FALSE!(true);
            TH_LOG!("listmount failed in new namespace");
        }
        7 => {
            ASSERT_FALSE!(true);
            TH_LOG!("New namespace has no mounts");
        }
        _ => {
            ASSERT_FALSE!(true);
            TH_LOG!("Unexpected error in child (exit %d)", WEXITSTATUS(status));
        }
    }
}

unsafe fn open_tree_ns_userns_umount_fails_einval(
    _metadata: *mut __test_metadata,
    _self: *mut open_tree_ns_userns,
) {
    let pid: pid_t;
    let mut status: c_int = 0;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut new_ns_id: uint64_t = 0;
        let mut list: [uint64_t; 256] = [0; 256];
        let nr_mounts: ssize_t;
        let fd: c_int;
        let mut i: ssize_t;

        /* Create new user namespace */
        if enter_userns() != 0 {
            _exit(2);
        }

        fd = sys_open_tree(
            AT_FDCWD,
            C!("/"),
            OPEN_TREE_NAMESPACE | AT_RECURSIVE | OPEN_TREE_CLOEXEC,
        );
        if fd < 0 {
            if errno == EINVAL {
                _exit(4);
            }
            _exit(1);
        }

        if get_mnt_ns_id(fd, &mut new_ns_id) != 0 {
            _exit(5);
        }

        /* Get all mounts in the new namespace */
        nr_mounts = listmount(
            LSMT_ROOT,
            new_ns_id,
            0,
            list.as_mut_ptr(),
            256,
            LISTMOUNT_REVERSE,
        );
        if nr_mounts < 0 {
            _exit(9);
        }

        if nr_mounts < 1 {
            _exit(10);
        }

        /* Enter the new namespace */
        if setns(fd, CLONE_NEWNS) < 0 {
            _exit(6);
        }

        i = 0;
        while i < nr_mounts {
            let sm: *mut statmount_t;
            let mnt_point: *const c_char;

            sm = statmount_alloc(list[i as usize], new_ns_id, STATMOUNT_MNT_POINT, 0);
            if sm.is_null() {
                _exit(11);
            }

            mnt_point = (*sm).str_.as_ptr().add((*sm).mnt_point as usize);

            TH_LOG!("Trying to umount %s", mnt_point);
            if umount2(mnt_point, MNT_DETACH) == 0 {
                free(sm as *mut c_void);
                _exit(7);
            }

            if errno != EINVAL {
                /* Wrong error */
                free(sm as *mut c_void);
                _exit(8);
            }

            free(sm as *mut c_void);
            i += 1;
        }

        close(fd);
        _exit(0);
    }

    ASSERT_EQ!(waitpid(pid, &mut status, 0), pid);
    ASSERT_TRUE!(WIFEXITED(status));

    match WEXITSTATUS(status) {
        0 => {}
        1 => {
            ASSERT_FALSE!(true);
            TH_LOG!("open_tree(OPEN_TREE_NAMESPACE) failed");
        }
        2 => {
            SKIP!(return, "setup_userns failed");
        }
        4 => {
            SKIP!(return, "OPEN_TREE_NAMESPACE not supported");
        }
        5 => {
            ASSERT_FALSE!(true);
            TH_LOG!("Failed to get mount namespace ID");
        }
        6 => {
            ASSERT_FALSE!(true);
            TH_LOG!("setns into new namespace failed");
        }
        7 => {
            ASSERT_FALSE!(true);
            TH_LOG!("umount succeeded but should have failed with EINVAL");
        }
        8 => {
            ASSERT_FALSE!(true);
            TH_LOG!("umount failed with wrong error (expected EINVAL)");
        }
        9 => {
            ASSERT_FALSE!(true);
            TH_LOG!("listmount failed");
        }
        10 => {
            ASSERT_FALSE!(true);
            TH_LOG!("No mounts in new namespace");
        }
        11 => {
            ASSERT_FALSE!(true);
            TH_LOG!("statmount_alloc failed");
        }
        _ => {
            ASSERT_FALSE!(true);
            TH_LOG!("Unexpected error in child (exit %d)", WEXITSTATUS(status));
        }
    }
}

unsafe fn open_tree_ns_userns_umount_succeeds(
    _metadata: *mut __test_metadata,
    _self: *mut open_tree_ns_userns,
) {
    let pid: pid_t;
    let mut status: c_int = 0;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut new_ns_id: uint64_t = 0;
        let mut list: [uint64_t; 256] = [0; 256];
        let nr_mounts: ssize_t;
        let fd: c_int;
        let mut i: ssize_t;

        if unshare(CLONE_NEWNS) != 0 {
            _exit(1);
        }

        if sys_mount(
            core::ptr::null(),
            C!("/"),
            core::ptr::null(),
            MS_SLAVE | MS_REC,
            core::ptr::null(),
        ) != 0
        {
            _exit(1);
        }

        fd = sys_open_tree(
            AT_FDCWD,
            C!("/"),
            OPEN_TREE_NAMESPACE | AT_RECURSIVE | OPEN_TREE_CLOEXEC,
        );
        if fd < 0 {
            if errno == EINVAL {
                _exit(4);
            }
            _exit(1);
        }

        if get_mnt_ns_id(fd, &mut new_ns_id) != 0 {
            _exit(5);
        }

        /* Get all mounts in the new namespace */
        nr_mounts = listmount(
            LSMT_ROOT,
            new_ns_id,
            0,
            list.as_mut_ptr(),
            256,
            LISTMOUNT_REVERSE,
        );
        if nr_mounts < 0 {
            _exit(9);
        }

        if nr_mounts < 1 {
            _exit(10);
        }

        /* Enter the new namespace */
        if setns(fd, CLONE_NEWNS) < 0 {
            _exit(6);
        }

        i = 0;
        while i < nr_mounts {
            let sm: *mut statmount_t;
            let mnt_point: *const c_char;

            sm = statmount_alloc(list[i as usize], new_ns_id, STATMOUNT_MNT_POINT, 0);
            if sm.is_null() {
                _exit(11);
            }

            mnt_point = (*sm).str_.as_ptr().add((*sm).mnt_point as usize);

            TH_LOG!("Trying to umount %s", mnt_point);
            if umount2(mnt_point, MNT_DETACH) != 0 {
                free(sm as *mut c_void);
                _exit(7);
            }

            free(sm as *mut c_void);
            i += 1;
        }

        close(fd);
        _exit(0);
    }

    ASSERT_EQ!(waitpid(pid, &mut status, 0), pid);
    ASSERT_TRUE!(WIFEXITED(status));

    match WEXITSTATUS(status) {
        0 => {}
        1 => {
            ASSERT_FALSE!(true);
            TH_LOG!("open_tree(OPEN_TREE_NAMESPACE) failed");
        }
        2 => {
            SKIP!(return, "setup_userns failed");
        }
        4 => {
            SKIP!(return, "OPEN_TREE_NAMESPACE not supported");
        }
        5 => {
            ASSERT_FALSE!(true);
            TH_LOG!("Failed to get mount namespace ID");
        }
        6 => {
            ASSERT_FALSE!(true);
            TH_LOG!("setns into new namespace failed");
        }
        7 => {
            ASSERT_FALSE!(true);
            TH_LOG!("umount failed but should have succeeded");
        }
        9 => {
            ASSERT_FALSE!(true);
            TH_LOG!("listmount failed");
        }
        10 => {
            ASSERT_FALSE!(true);
            TH_LOG!("No mounts in new namespace");
        }
        11 => {
            ASSERT_FALSE!(true);
            TH_LOG!("statmount_alloc failed");
        }
        _ => {
            ASSERT_FALSE!(true);
            TH_LOG!("Unexpected error in child (exit %d)", WEXITSTATUS(status));
        }
    }
}

#[repr(C)]
struct open_tree_ns_unbindable {
    tmpdir: [c_char; PATH_MAX],
    mounted: bool_,
}

unsafe fn open_tree_ns_unbindable_setup(self_: *mut open_tree_ns_unbindable) {
    let ret: c_int;

    (*self_).mounted = false;

    /* Check if open_tree syscall is supported */
    ret = sys_open_tree(-1, core::ptr::null(), 0);
    if ret == -1 && errno == ENOSYS {
        SKIP!(return, "open_tree() syscall not supported");
    }

    /* Create a temporary directory for the test mount */
    snprintf(
        (*self_).tmpdir.as_mut_ptr(),
        core::mem::size_of_val(&(*self_).tmpdir),
        C!("/tmp/open_tree_ns_test.XXXXXX"),
    );
    ASSERT_NE!(mkdtemp((*self_).tmpdir.as_mut_ptr()), core::ptr::null_mut());

    /* Mount tmpfs there */
    ret = mount(
        C!("tmpfs"),
        (*self_).tmpdir.as_ptr(),
        C!("tmpfs"),
        0,
        core::ptr::null(),
    );
    if ret < 0 {
        rmdir((*self_).tmpdir.as_ptr());
        SKIP!(return, "Failed to mount tmpfs");
    }
    (*self_).mounted = true;

    ret = mount(
        core::ptr::null(),
        (*self_).tmpdir.as_ptr(),
        core::ptr::null(),
        MS_UNBINDABLE,
        core::ptr::null(),
    );
    if ret < 0 {
        rmdir((*self_).tmpdir.as_ptr());
        SKIP!(return, "Failed to make tmpfs unbindable");
    }
}

unsafe fn open_tree_ns_unbindable_teardown(self_: *mut open_tree_ns_unbindable) {
    if (*self_).mounted {
        umount2((*self_).tmpdir.as_ptr(), MNT_DETACH);
    }
    rmdir((*self_).tmpdir.as_ptr());
}

unsafe fn open_tree_ns_unbindable_fails_on_unbindable(
    _metadata: *mut __test_metadata,
    self_: *mut open_tree_ns_unbindable,
) {
    let fd: c_int;

    fd = sys_open_tree(
        AT_FDCWD,
        (*self_).tmpdir.as_ptr(),
        OPEN_TREE_NAMESPACE | OPEN_TREE_CLOEXEC,
    );
    ASSERT_LT!(fd, 0);
}

unsafe fn open_tree_ns_unbindable_recursive_skips_on_unbindable(
    _metadata: *mut __test_metadata,
    self_: *mut open_tree_ns_unbindable,
) {
    let mut new_ns_id: uint64_t = 0;
    let mut list: [uint64_t; 256] = [0; 256];
    let nr_mounts: ssize_t;
    let fd: c_int;
    let mut i: ssize_t;
    let mut found_unbindable: bool_ = false;

    fd = sys_open_tree(
        AT_FDCWD,
        C!("/"),
        OPEN_TREE_NAMESPACE | AT_RECURSIVE | OPEN_TREE_CLOEXEC,
    );
    ASSERT_GT!(fd, 0);

    ASSERT_EQ!(get_mnt_ns_id(fd, &mut new_ns_id), 0);

    nr_mounts = listmount(LSMT_ROOT, new_ns_id, 0, list.as_mut_ptr(), 256, 0);
    ASSERT_GE!(nr_mounts, 0, {
        TH_LOG!("listmount failed: %m");
    });

    /*
     * Iterate through all mounts in the new namespace and verify
     * the unbindable tmpfs mount was silently dropped.
     */
    i = 0;
    while i < nr_mounts {
        let sm: *mut statmount_t;
        let mnt_point: *const c_char;

        sm = statmount_alloc(list[i as usize], new_ns_id, STATMOUNT_MNT_POINT, 0);
        ASSERT_NE!(sm, core::ptr::null_mut(), {
            TH_LOG!(
                "statmount_alloc failed for mnt_id %llu",
                list[i as usize] as c_ulong
            );
        });

        mnt_point = (*sm).str_.as_ptr().add((*sm).mnt_point as usize);

        if strcmp(mnt_point, (*self_).tmpdir.as_ptr()) == 0 {
            TH_LOG!(
                "Found unbindable mount at %s (should have been dropped)",
                mnt_point
            );
            found_unbindable = true;
        }

        free(sm as *mut c_void);
        i += 1;
    }

    ASSERT_FALSE!(found_unbindable, {
        TH_LOG!(
            "Unbindable mount at %s was not dropped",
            (*self_).tmpdir.as_ptr()
        );
    });

    close(fd);
}

TEST_HARNESS_MAIN!();
