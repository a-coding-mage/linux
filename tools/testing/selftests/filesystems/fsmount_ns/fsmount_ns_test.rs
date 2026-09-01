// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026 Christian Brauner <brauner@kernel.org>
 *
 * Test for FSMOUNT_NAMESPACE flag.
 *
 * Test that fsmount() with FSMOUNT_NAMESPACE creates a new mount
 * namespace containing the specified mount.
 */

// C dependencies translated as external expectations:
// errno.h, fcntl.h, limits.h, linux/nsfs.h, sched.h, stdio.h, stdlib.h,
// string.h, sys/ioctl.h, sys/mount.h, sys/stat.h, sys/wait.h, unistd.h,
// ../wrappers.h, ../statmount/statmount.h, ../utils.h,
// ../../kselftest_harness.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

type uint64_t = u64;
type ssize_t = isize;
type pid_t = c_int;

const FSMOUNT_NAMESPACE: c_uint = 0x00000002;
const FSMOUNT_CLOEXEC: c_uint = 0x00000001;
const FSCONFIG_CMD_CREATE: c_uint = 6;

unsafe fn get_mnt_ns_id(fd: c_int, mnt_ns_id: *mut uint64_t) -> c_int {
    if ioctl(fd, NS_GET_MNTNS_ID as c_ulong, mnt_ns_id) < 0 {
        return -errno();
    }
    0
}

unsafe fn get_mnt_ns_id_from_path(path: *const c_char, mnt_ns_id: *mut uint64_t) -> c_int {
    let fd: c_int;
    let ret: c_int;

    fd = open(path, O_RDONLY);
    if fd < 0 {
        return -errno();
    }

    ret = get_mnt_ns_id(fd, mnt_ns_id);
    close(fd);
    ret
}

unsafe fn log_mount(_metadata: *mut __test_metadata, sm: *mut statmount) {
    let mut fs_type: *const c_char = c"".as_ptr();
    let mut mnt_root: *const c_char = c"".as_ptr();
    let mut mnt_point: *const c_char = c"".as_ptr();

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
        c"  mnt_id: %llu, parent_id: %llu, fs_type: %s, root: %s, point: %s",
        (*sm).mnt_id as c_ulonglong,
        (*sm).mnt_parent_id as c_ulonglong,
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
        TH_LOG!(c"listmount failed: %s", strerror(errno()));
        return;
    }

    TH_LOG!(
        c"Mount namespace %llu contains %zd mount(s):",
        mnt_ns_id as c_ulonglong,
        nr_mounts
    );

    let mut i: ssize_t = 0;
    while i < nr_mounts {
        let sm: *mut statmount;

        sm = statmount_alloc(
            list[i as usize],
            mnt_ns_id,
            STATMOUNT_MNT_BASIC | STATMOUNT_FS_TYPE | STATMOUNT_MNT_ROOT | STATMOUNT_MNT_POINT,
            0,
        );
        if sm.is_null() {
            TH_LOG!(
                c"  [%zd] mnt_id %llu: statmount failed: %s",
                i,
                list[i as usize] as c_ulonglong,
                strerror(errno())
            );
            i += 1;
            continue;
        }

        log_mount(_metadata, sm);
        free(sm as *mut c_void);
        i += 1;
    }
}

unsafe fn create_tmpfs_fd() -> c_int {
    let fs_fd: c_int;
    let ret: c_int;

    fs_fd = sys_fsopen(c"tmpfs".as_ptr(), FSOPEN_CLOEXEC);
    if fs_fd < 0 {
        return -errno();
    }

    ret = sys_fsconfig(fs_fd, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0);
    if ret < 0 {
        close(fs_fd);
        return -errno();
    }

    fs_fd
}

FIXTURE!(fsmount_ns {
    fd: c_int,
    fs_fd: c_int,
    current_ns_id: uint64_t,
});

FIXTURE_VARIANT!(fsmount_ns {
    fstype: *const c_char,
    flags: c_uint,
    expect_success: bool,
    expect_different_ns: bool,
    min_mounts: c_int,
});

FIXTURE_VARIANT_ADD!(fsmount_ns, basic_tmpfs {
    fstype: c"tmpfs".as_ptr(),
    flags: FSMOUNT_NAMESPACE | FSMOUNT_CLOEXEC,
    expect_success: true,
    expect_different_ns: true,
    min_mounts: 1,
});

FIXTURE_VARIANT_ADD!(fsmount_ns, cloexec_only {
    fstype: c"tmpfs".as_ptr(),
    flags: FSMOUNT_CLOEXEC,
    expect_success: true,
    expect_different_ns: false,
    min_mounts: 1,
});

FIXTURE_VARIANT_ADD!(fsmount_ns, namespace_only {
    fstype: c"tmpfs".as_ptr(),
    flags: FSMOUNT_NAMESPACE,
    expect_success: true,
    expect_different_ns: true,
    min_mounts: 1,
});

FIXTURE_SETUP!(fsmount_ns, |self_: *mut fsmount_ns, _metadata: *mut __test_metadata| unsafe {
    let ret: c_int;

    (*self_).fd = -1;
    (*self_).fs_fd = -1;

    /* Check if fsopen syscall is supported */
    ret = sys_fsopen(c"tmpfs".as_ptr(), 0);
    if ret == -1 && errno() == ENOSYS {
        SKIP!(return, c"fsopen() syscall not supported");
    }
    if ret >= 0 {
        close(ret);
    }

    /* Check if statmount/listmount are supported */
    ret = statmount(0, 0, 0, 0, core::ptr::null_mut(), 0, 0);
    if ret == -1 && errno() == ENOSYS {
        SKIP!(return, c"statmount() syscall not supported");
    }

    /* Get current mount namespace ID for comparison */
    ret = get_mnt_ns_id_from_path(c"/proc/self/ns/mnt".as_ptr(), &mut (*self_).current_ns_id);
    if ret < 0 {
        SKIP!(return, c"Failed to get current mount namespace ID");
    }
});

FIXTURE_TEARDOWN!(fsmount_ns, |self_: *mut fsmount_ns, _metadata: *mut __test_metadata| unsafe {
    if (*self_).fd >= 0 {
        close((*self_).fd);
    }
    if (*self_).fs_fd >= 0 {
        close((*self_).fs_fd);
    }
});

TEST_F!(fsmount_ns, create_namespace, |self_: *mut fsmount_ns, variant: *mut fsmount_ns_variant, _metadata: *mut __test_metadata| unsafe {
    let mut new_ns_id: uint64_t = 0;
    let mut list: [uint64_t; 256] = [0; 256];
    let nr_mounts: ssize_t;
    let ret: c_int;

    (*self_).fs_fd = create_tmpfs_fd();
    ASSERT_GE!((*self_).fs_fd, 0);

    (*self_).fd = sys_fsmount((*self_).fs_fd, (*variant).flags, 0);

    if !(*variant).expect_success {
        ASSERT_LT!((*self_).fd, 0);
        return;
    }

    if (*self_).fd < 0 && errno() == EINVAL {
        SKIP!(return, c"FSMOUNT_NAMESPACE not supported");
    }

    ASSERT_GE!((*self_).fd, 0);

    if (*variant).expect_different_ns {
        /* Verify we can get the namespace ID from the fd */
        ret = get_mnt_ns_id((*self_).fd, &mut new_ns_id);
        ASSERT_EQ!(ret, 0);

        /* Verify it's a different namespace */
        ASSERT_NE!(new_ns_id, (*self_).current_ns_id);

        /* List mounts in the new namespace */
        nr_mounts = listmount(LSMT_ROOT, new_ns_id, 0, list.as_mut_ptr(), 256, 0);
        ASSERT_GE!(nr_mounts, 0, {
            TH_LOG!(c"%m - listmount failed");
        });

        /* Verify minimum expected mounts */
        ASSERT_GE!(nr_mounts, (*variant).min_mounts as ssize_t);
        TH_LOG!(c"Namespace contains %zd mounts", nr_mounts);
    }
});

TEST_F!(fsmount_ns, setns_into_namespace, |self_: *mut fsmount_ns, variant: *mut fsmount_ns_variant, _metadata: *mut __test_metadata| unsafe {
    let mut new_ns_id: uint64_t = 0;
    let pid: pid_t;
    let mut status: c_int = 0;
    let ret: c_int;

    /* Only test with FSMOUNT_NAMESPACE flag */
    if (*variant).flags & FSMOUNT_NAMESPACE == 0 {
        SKIP!(return, c"setns test only for FSMOUNT_NAMESPACE case");
    }

    (*self_).fs_fd = create_tmpfs_fd();
    ASSERT_GE!((*self_).fs_fd, 0);

    (*self_).fd = sys_fsmount((*self_).fs_fd, (*variant).flags, 0);
    if (*self_).fd < 0 && errno() == EINVAL {
        SKIP!(return, c"FSMOUNT_NAMESPACE not supported");
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
});

TEST_F!(fsmount_ns, verify_mount_properties, |self_: *mut fsmount_ns, variant: *mut fsmount_ns_variant, _metadata: *mut __test_metadata| unsafe {
    let mut sm: statmount = core::mem::zeroed();
    let mut new_ns_id: uint64_t = 0;
    let mut list: [uint64_t; 256] = [0; 256];
    let nr_mounts: ssize_t;
    let ret: c_int;

    /* Only test with basic FSMOUNT_NAMESPACE flags */
    if (*variant).flags != (FSMOUNT_NAMESPACE | FSMOUNT_CLOEXEC) {
        SKIP!(return, c"mount properties test only for basic case");
    }

    (*self_).fs_fd = create_tmpfs_fd();
    ASSERT_GE!((*self_).fs_fd, 0);

    (*self_).fd = sys_fsmount((*self_).fs_fd, FSMOUNT_NAMESPACE | FSMOUNT_CLOEXEC, 0);
    if (*self_).fd < 0 && errno() == EINVAL {
        SKIP!(return, c"FSMOUNT_NAMESPACE not supported");
    }

    ASSERT_GE!((*self_).fd, 0);

    ret = get_mnt_ns_id((*self_).fd, &mut new_ns_id);
    ASSERT_EQ!(ret, 0);

    nr_mounts = listmount(LSMT_ROOT, new_ns_id, 0, list.as_mut_ptr(), 256, 0);
    ASSERT_GE!(nr_mounts, 1);

    /* Get info about the root mount */
    ret = statmount(list[0], new_ns_id, 0, STATMOUNT_MNT_BASIC, &mut sm, core::mem::size_of_val(&sm), 0);
    ASSERT_EQ!(ret, 0);

    TH_LOG!(
        c"Root mount id: %llu, parent: %llu",
        sm.mnt_id as c_ulonglong,
        sm.mnt_parent_id as c_ulonglong
    );
});

TEST_F!(fsmount_ns, verify_tmpfs_type, |self_: *mut fsmount_ns, variant: *mut fsmount_ns_variant, _metadata: *mut __test_metadata| unsafe {
    let sm: *mut statmount;
    let mut new_ns_id: uint64_t = 0;
    let mut list: [uint64_t; 256] = [0; 256];
    let nr_mounts: ssize_t;
    let fs_type: *const c_char;
    let ret: c_int;

    /* Only test with basic FSMOUNT_NAMESPACE flags */
    if (*variant).flags != (FSMOUNT_NAMESPACE | FSMOUNT_CLOEXEC) {
        SKIP!(return, c"fs type test only for basic case");
    }

    (*self_).fs_fd = create_tmpfs_fd();
    ASSERT_GE!((*self_).fs_fd, 0);

    (*self_).fd = sys_fsmount((*self_).fs_fd, FSMOUNT_NAMESPACE | FSMOUNT_CLOEXEC, 0);
    if (*self_).fd < 0 && errno() == EINVAL {
        SKIP!(return, c"FSMOUNT_NAMESPACE not supported");
    }

    ASSERT_GE!((*self_).fd, 0);

    ret = get_mnt_ns_id((*self_).fd, &mut new_ns_id);
    ASSERT_EQ!(ret, 0);

    nr_mounts = listmount(LSMT_ROOT, new_ns_id, 0, list.as_mut_ptr(), 256, 0);
    ASSERT_GE!(nr_mounts, 1);

    sm = statmount_alloc(list[0], new_ns_id, STATMOUNT_FS_TYPE, 0);
    ASSERT_NE!(sm, core::ptr::null_mut());

    fs_type = (*sm).str_.as_ptr().add((*sm).fs_type as usize);
    ASSERT_STREQ!(fs_type, c"tmpfs".as_ptr());

    free(sm as *mut c_void);
});

FIXTURE!(fsmount_ns_caps {
    has_caps: bool,
});

FIXTURE_SETUP!(fsmount_ns_caps, |self_: *mut fsmount_ns_caps, _metadata: *mut __test_metadata| unsafe {
    let ret: c_int;

    /* Check if fsopen syscall is supported */
    ret = sys_fsopen(c"tmpfs".as_ptr(), 0);
    if ret == -1 && errno() == ENOSYS {
        SKIP!(return, c"fsopen() syscall not supported");
    }
    if ret >= 0 {
        close(ret);
    }

    (*self_).has_caps = geteuid() == 0;
});

FIXTURE_TEARDOWN!(fsmount_ns_caps, |_self_: *mut fsmount_ns_caps, _metadata: *mut __test_metadata| unsafe {
});

TEST_F!(fsmount_ns_caps, requires_cap_sys_admin, |self_: *mut fsmount_ns_caps, _variant: *mut c_void, _metadata: *mut __test_metadata| unsafe {
    let pid: pid_t;
    let mut status: c_int = 0;
    let fs_fd: c_int;

    /*
     * Prepare the configured filesystem fd as root before forking.
     * fsopen() requires CAP_SYS_ADMIN in the mount namespace's
     * user_ns, which won't be available after enter_userns().
     */
    fs_fd = sys_fsopen(c"tmpfs".as_ptr(), FSOPEN_CLOEXEC);
    ASSERT_GE!(fs_fd, 0);

    ASSERT_EQ!(sys_fsconfig(fs_fd, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);

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

        fd = sys_fsmount(fs_fd, FSMOUNT_NAMESPACE | FSMOUNT_CLOEXEC, 0);
        close(fs_fd);

        if fd >= 0 {
            close(fd);
            /* Should have failed without caps */
            _exit(1);
        }

        if errno() == EPERM {
            _exit(0);
        }

        /* EINVAL means FSMOUNT_NAMESPACE not supported */
        if errno() == EINVAL {
            _exit(6);
        }

        /* Unexpected error */
        _exit(7);
    }

    close(fs_fd);
    ASSERT_EQ!(waitpid(pid, &mut status, 0), pid);
    ASSERT_TRUE!(WIFEXITED(status));

    match WEXITSTATUS(status) {
        0 => {
            /* Expected: EPERM without caps */
        }
        1 => {
            ASSERT_FALSE!(true);
            TH_LOG!(c"FSMOUNT_NAMESPACE succeeded without caps");
        }
        2 => SKIP!(return, c"enter_userns failed"),
        3 => SKIP!(return, c"caps_down failed"),
        6 => SKIP!(return, c"FSMOUNT_NAMESPACE not supported"),
        _ => {
            ASSERT_FALSE!(true);
            TH_LOG!(c"Unexpected error in child (exit %d)", WEXITSTATUS(status));
        }
    }
});

FIXTURE!(fsmount_ns_userns {
    fd: c_int,
    fs_fd: c_int,
});

FIXTURE_SETUP!(fsmount_ns_userns, |self_: *mut fsmount_ns_userns, _metadata: *mut __test_metadata| unsafe {
    let ret: c_int;

    (*self_).fd = -1;
    (*self_).fs_fd = -1;

    /* Check if fsopen syscall is supported */
    ret = sys_fsopen(c"tmpfs".as_ptr(), 0);
    if ret == -1 && errno() == ENOSYS {
        SKIP!(return, c"fsopen() syscall not supported");
    }
    if ret >= 0 {
        close(ret);
    }

    /* Check if statmount/listmount are supported */
    ret = statmount(0, 0, 0, 0, core::ptr::null_mut(), 0, 0);
    if ret == -1 && errno() == ENOSYS {
        SKIP!(return, c"statmount() syscall not supported");
    }
});

FIXTURE_TEARDOWN!(fsmount_ns_userns, |self_: *mut fsmount_ns_userns, _metadata: *mut __test_metadata| unsafe {
    if (*self_).fd >= 0 {
        close((*self_).fd);
    }
    if (*self_).fs_fd >= 0 {
        close((*self_).fs_fd);
    }
});

TEST_F!(fsmount_ns_userns, create_in_userns, |self_: *mut fsmount_ns_userns, _variant: *mut c_void, _metadata: *mut __test_metadata| unsafe {
    let pid: pid_t;
    let mut status: c_int = 0;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut new_ns_id: uint64_t = 0;
        let mut list: [uint64_t; 256] = [0; 256];
        let nr_mounts: ssize_t;
        let fs_fd: c_int;
        let fd: c_int;

        /* Create new user namespace (also creates mount namespace) */
        if setup_userns() != 0 {
            _exit(2);
        }

        /* Now we have CAP_SYS_ADMIN in the user namespace */
        fs_fd = sys_fsopen(c"tmpfs".as_ptr(), FSOPEN_CLOEXEC);
        if fs_fd < 0 {
            _exit(3);
        }

        if sys_fsconfig(fs_fd, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0) < 0 {
            close(fs_fd);
            _exit(4);
        }

        fd = sys_fsmount(fs_fd, FSMOUNT_NAMESPACE | FSMOUNT_CLOEXEC, 0);
        close(fs_fd);

        if fd < 0 {
            if errno() == EINVAL {
                _exit(6); /* FSMOUNT_NAMESPACE not supported */
            }
            _exit(1);
        }

        /* Verify we can get the namespace ID */
        if get_mnt_ns_id(fd, &mut new_ns_id) != 0 {
            _exit(7);
        }

        /* Verify we can list mounts in the new namespace */
        nr_mounts = listmount(LSMT_ROOT, new_ns_id, 0, list.as_mut_ptr(), 256, 0);
        if nr_mounts < 0 {
            _exit(8);
        }

        /* Should have at least 1 mount (the tmpfs) */
        if nr_mounts < 1 {
            _exit(9);
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
            TH_LOG!(c"fsmount(FSMOUNT_NAMESPACE) failed in userns");
        }
        2 => SKIP!(return, c"setup_userns failed"),
        3 => SKIP!(return, c"fsopen failed in userns"),
        4 => SKIP!(return, c"fsconfig CMD_CREATE failed in userns"),
        6 => SKIP!(return, c"FSMOUNT_NAMESPACE not supported"),
        7 => {
            ASSERT_FALSE!(true);
            TH_LOG!(c"Failed to get mount namespace ID");
        }
        8 => {
            ASSERT_FALSE!(true);
            TH_LOG!(c"listmount failed in new namespace");
        }
        9 => {
            ASSERT_FALSE!(true);
            TH_LOG!(c"New namespace has no mounts");
        }
        _ => {
            ASSERT_FALSE!(true);
            TH_LOG!(c"Unexpected error in child (exit %d)", WEXITSTATUS(status));
        }
    }
});

TEST_F!(fsmount_ns_userns, setns_in_userns, |self_: *mut fsmount_ns_userns, _variant: *mut c_void, _metadata: *mut __test_metadata| unsafe {
    let pid: pid_t;
    let mut status: c_int = 0;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut new_ns_id: uint64_t = 0;
        let fs_fd: c_int;
        let fd: c_int;
        let inner_pid: pid_t;
        let mut inner_status: c_int = 0;

        /* Create new user namespace */
        if setup_userns() != 0 {
            _exit(2);
        }

        fs_fd = sys_fsopen(c"tmpfs".as_ptr(), FSOPEN_CLOEXEC);
        if fs_fd < 0 {
            _exit(3);
        }

        if sys_fsconfig(fs_fd, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0) < 0 {
            close(fs_fd);
            _exit(4);
        }

        fd = sys_fsmount(fs_fd, FSMOUNT_NAMESPACE | FSMOUNT_CLOEXEC, 0);
        close(fs_fd);

        if fd < 0 {
            if errno() == EINVAL {
                _exit(6);
            }
            _exit(1);
        }

        if get_mnt_ns_id(fd, &mut new_ns_id) != 0 {
            _exit(7);
        }

        /* Fork again to test setns into the new namespace */
        inner_pid = fork();
        if inner_pid < 0 {
            _exit(10);
        }

        if inner_pid == 0 {
            /* Inner child: enter the new namespace */
            if setns(fd, CLONE_NEWNS) < 0 {
                _exit(1);
            }
            _exit(0);
        }

        if waitpid(inner_pid, &mut inner_status, 0) != inner_pid {
            _exit(11);
        }

        if !WIFEXITED(inner_status) || WEXITSTATUS(inner_status) != 0 {
            _exit(12);
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
            TH_LOG!(c"fsmount or setns failed in userns");
        }
        2 => SKIP!(return, c"setup_userns failed"),
        3 => SKIP!(return, c"fsopen failed in userns"),
        4 => SKIP!(return, c"fsconfig CMD_CREATE failed in userns"),
        6 => SKIP!(return, c"FSMOUNT_NAMESPACE not supported"),
        7 => {
            ASSERT_FALSE!(true);
            TH_LOG!(c"Failed to get mount namespace ID");
        }
        10 => {
            ASSERT_FALSE!(true);
            TH_LOG!(c"Inner fork failed");
        }
        11 => {
            ASSERT_FALSE!(true);
            TH_LOG!(c"Inner waitpid failed");
        }
        12 => {
            ASSERT_FALSE!(true);
            TH_LOG!(c"setns into new namespace failed");
        }
        _ => {
            ASSERT_FALSE!(true);
            TH_LOG!(c"Unexpected error in child (exit %d)", WEXITSTATUS(status));
        }
    }
});

macro_rules! test_umount_case {
    ($name:ident, $prepare:expr, $expect_umount_success:expr, $failure_message:expr) => {
        TEST_F!(fsmount_ns_userns, $name, |self_: *mut fsmount_ns_userns, _variant: *mut c_void, _metadata: *mut __test_metadata| unsafe {
            let pid: pid_t;
            let mut status: c_int = 0;

            pid = fork();
            ASSERT_GE!(pid, 0);

            if pid == 0 {
                let mut new_ns_id: uint64_t = 0;
                let mut list: [uint64_t; 256] = [0; 256];
                let nr_mounts: ssize_t;
                let fs_fd: c_int;
                let fd: c_int;
                let mut i: ssize_t;

                $prepare;

                fs_fd = sys_fsopen(c"tmpfs".as_ptr(), FSOPEN_CLOEXEC);
                if fs_fd < 0 {
                    _exit(3);
                }

                if sys_fsconfig(fs_fd, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0) < 0 {
                    close(fs_fd);
                    _exit(4);
                }

                fd = sys_fsmount(fs_fd, FSMOUNT_NAMESPACE | FSMOUNT_CLOEXEC, 0);
                close(fs_fd);

                if fd < 0 {
                    if errno() == EINVAL {
                        _exit(6);
                    }
                    _exit(1);
                }

                if get_mnt_ns_id(fd, &mut new_ns_id) != 0 {
                    _exit(7);
                }

                /* Get all mounts in the new namespace */
                nr_mounts = listmount(LSMT_ROOT, new_ns_id, 0, list.as_mut_ptr(), 256, LISTMOUNT_REVERSE);
                if nr_mounts < 0 {
                    _exit(13);
                }

                if nr_mounts < 1 {
                    _exit(14);
                }

                /* Enter the new namespace */
                if setns(fd, CLONE_NEWNS) < 0 {
                    _exit(8);
                }

                i = 0;
                while i < nr_mounts {
                    let sm: *mut statmount;
                    let mnt_point: *const c_char;

                    sm = statmount_alloc(list[i as usize], new_ns_id, STATMOUNT_MNT_POINT, 0);
                    if sm.is_null() {
                        _exit(15);
                    }

                    mnt_point = (*sm).str_.as_ptr().add((*sm).mnt_point as usize);

                    if $expect_umount_success {
                        if umount2(mnt_point, MNT_DETACH) != 0 {
                            free(sm as *mut c_void);
                            _exit(9);
                        }
                    } else {
                        if umount2(mnt_point, MNT_DETACH) == 0 {
                            free(sm as *mut c_void);
                            _exit(9);
                        }

                        if errno() != EINVAL {
                            /* Wrong error */
                            free(sm as *mut c_void);
                            _exit(10);
                        }
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
                    TH_LOG!($failure_message);
                }
                2 => SKIP!(return, c"setup_userns failed"),
                3 => SKIP!(return, c"fsopen failed in userns"),
                4 => SKIP!(return, c"fsconfig CMD_CREATE failed in userns"),
                6 => SKIP!(return, c"FSMOUNT_NAMESPACE not supported"),
                7 => {
                    ASSERT_FALSE!(true);
                    TH_LOG!(c"Failed to get mount namespace ID");
                }
                8 => {
                    ASSERT_FALSE!(true);
                    TH_LOG!(c"setns into new namespace failed");
                }
                9 => {
                    ASSERT_FALSE!(true);
                    if $expect_umount_success {
                        TH_LOG!(c"umount failed but should have succeeded");
                    } else {
                        TH_LOG!(c"umount succeeded but should have failed with EINVAL");
                    }
                }
                10 => {
                    ASSERT_FALSE!(true);
                    TH_LOG!(c"umount failed with wrong error (expected EINVAL)");
                }
                13 => {
                    ASSERT_FALSE!(true);
                    TH_LOG!(c"listmount failed");
                }
                14 => {
                    ASSERT_FALSE!(true);
                    TH_LOG!(c"No mounts in new namespace");
                }
                15 => {
                    ASSERT_FALSE!(true);
                    TH_LOG!(c"statmount_alloc failed");
                }
                _ => {
                    ASSERT_FALSE!(true);
                    TH_LOG!(c"Unexpected error in child (exit %d)", WEXITSTATUS(status));
                }
            }
        });
    };
}

test_umount_case!(
    umount_fails_einval,
    {
        /* Create new user namespace */
        if setup_userns() != 0 {
            _exit(2);
        }
    },
    false,
    c"fsmount(FSMOUNT_NAMESPACE) failed"
);

test_umount_case!(
    umount_succeeds,
    {
        if unshare(CLONE_NEWNS) != 0 {
            _exit(1);
        }

        if sys_mount(core::ptr::null(), c"/".as_ptr(), core::ptr::null(), MS_SLAVE | MS_REC, core::ptr::null()) != 0 {
            _exit(1);
        }
    },
    true,
    c"fsmount(FSMOUNT_NAMESPACE) failed or unshare failed"
);

FIXTURE!(fsmount_ns_mount_attrs {
    fd: c_int,
    fs_fd: c_int,
});

FIXTURE_SETUP!(fsmount_ns_mount_attrs, |self_: *mut fsmount_ns_mount_attrs, _metadata: *mut __test_metadata| unsafe {
    let ret: c_int;

    (*self_).fd = -1;
    (*self_).fs_fd = -1;

    /* Check if fsopen syscall is supported */
    ret = sys_fsopen(c"tmpfs".as_ptr(), 0);
    if ret == -1 && errno() == ENOSYS {
        SKIP!(return, c"fsopen() syscall not supported");
    }
    if ret >= 0 {
        close(ret);
    }

    /* Check if statmount/listmount are supported */
    ret = statmount(0, 0, 0, 0, core::ptr::null_mut(), 0, 0);
    if ret == -1 && errno() == ENOSYS {
        SKIP!(return, c"statmount() syscall not supported");
    }
});

FIXTURE_TEARDOWN!(fsmount_ns_mount_attrs, |self_: *mut fsmount_ns_mount_attrs, _metadata: *mut __test_metadata| unsafe {
    if (*self_).fd >= 0 {
        close((*self_).fd);
    }
    if (*self_).fs_fd >= 0 {
        close((*self_).fs_fd);
    }
});

macro_rules! test_mount_attr {
    ($name:ident, $attr:expr, $comment:literal) => {
        TEST_F!(fsmount_ns_mount_attrs, $name, |self_: *mut fsmount_ns_mount_attrs, _variant: *mut c_void, _metadata: *mut __test_metadata| unsafe {
            let mut sm: statmount = core::mem::zeroed();
            let mut new_ns_id: uint64_t = 0;
            let mut list: [uint64_t; 256] = [0; 256];
            let nr_mounts: ssize_t;
            let ret: c_int;

            (*self_).fs_fd = create_tmpfs_fd();
            ASSERT_GE!((*self_).fs_fd, 0);

            (*self_).fd = sys_fsmount((*self_).fs_fd, FSMOUNT_NAMESPACE | FSMOUNT_CLOEXEC, $attr);
            if (*self_).fd < 0 && errno() == EINVAL {
                SKIP!(return, c"FSMOUNT_NAMESPACE not supported");
            }

            ASSERT_GE!((*self_).fd, 0);

            ret = get_mnt_ns_id((*self_).fd, &mut new_ns_id);
            ASSERT_EQ!(ret, 0);

            nr_mounts = listmount(LSMT_ROOT, new_ns_id, 0, list.as_mut_ptr(), 256, 0);
            ASSERT_GE!(nr_mounts, 1);

            ret = statmount(list[0], new_ns_id, 0, STATMOUNT_MNT_BASIC, &mut sm, core::mem::size_of_val(&sm), 0);
            ASSERT_EQ!(ret, 0);

            /* Verify the mount is $comment */
            ASSERT_TRUE!(sm.mnt_attr & $attr != 0);
        });
    };
}

test_mount_attr!(readonly, MOUNT_ATTR_RDONLY, "read-only");
test_mount_attr!(noexec, MOUNT_ATTR_NOEXEC, "noexec");
test_mount_attr!(nosuid, MOUNT_ATTR_NOSUID, "nosuid");
test_mount_attr!(noatime, MOUNT_ATTR_NOATIME, "noatime");

TEST_F!(fsmount_ns_mount_attrs, combined, |self_: *mut fsmount_ns_mount_attrs, _variant: *mut c_void, _metadata: *mut __test_metadata| unsafe {
    let mut sm: statmount = core::mem::zeroed();
    let mut new_ns_id: uint64_t = 0;
    let mut list: [uint64_t; 256] = [0; 256];
    let nr_mounts: ssize_t;
    let ret: c_int;

    (*self_).fs_fd = create_tmpfs_fd();
    ASSERT_GE!((*self_).fs_fd, 0);

    (*self_).fd = sys_fsmount(
        (*self_).fs_fd,
        FSMOUNT_NAMESPACE | FSMOUNT_CLOEXEC,
        MOUNT_ATTR_RDONLY | MOUNT_ATTR_NOEXEC | MOUNT_ATTR_NOSUID | MOUNT_ATTR_NOATIME,
    );
    if (*self_).fd < 0 && errno() == EINVAL {
        SKIP!(return, c"FSMOUNT_NAMESPACE not supported");
    }

    ASSERT_GE!((*self_).fd, 0);

    ret = get_mnt_ns_id((*self_).fd, &mut new_ns_id);
    ASSERT_EQ!(ret, 0);

    nr_mounts = listmount(LSMT_ROOT, new_ns_id, 0, list.as_mut_ptr(), 256, 0);
    ASSERT_GE!(nr_mounts, 1);

    ret = statmount(list[0], new_ns_id, 0, STATMOUNT_MNT_BASIC, &mut sm, core::mem::size_of_val(&sm), 0);
    ASSERT_EQ!(ret, 0);

    /* Verify all attributes are set */
    ASSERT_TRUE!(sm.mnt_attr & MOUNT_ATTR_RDONLY != 0);
    ASSERT_TRUE!(sm.mnt_attr & MOUNT_ATTR_NOEXEC != 0);
    ASSERT_TRUE!(sm.mnt_attr & MOUNT_ATTR_NOSUID != 0);
    ASSERT_TRUE!(sm.mnt_attr & MOUNT_ATTR_NOATIME != 0);
});

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
