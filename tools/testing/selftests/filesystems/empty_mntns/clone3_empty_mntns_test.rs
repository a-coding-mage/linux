// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Tests for empty mount namespace creation via clone3() CLONE_EMPTY_MNTNS
 *
 * These tests exercise the clone3() code path for creating empty mount
 * namespaces, which is distinct from the unshare() path tested in
 * empty_mntns_test.c.  With clone3(), CLONE_EMPTY_MNTNS (0x2000000000ULL)
 * is a 64-bit flag that implies CLONE_NEWNS.  The implication happens in
 * kernel_clone() before copy_process(), unlike unshare() where it goes
 * through UNSHARE_EMPTY_MNTNS -> CLONE_EMPTY_MNTNS conversion in
 * unshare_nsproxy_namespaces().
 *
 * Copyright (c) 2024 Christian Brauner <brauner@kernel.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type pid_t = c_int;
type ssize_t = isize;
type uint64_t = u64;
type uintptr_t = usize;

#[repr(C)]
struct __clone_args {
    flags: uint64_t,
    pidfd: uint64_t,
    child_tid: uint64_t,
    parent_tid: uint64_t,
    exit_signal: uint64_t,
    stack: uint64_t,
    stack_size: uint64_t,
    tls: uint64_t,
    set_tid: uint64_t,
    set_tid_size: uint64_t,
    cgroup: uint64_t,
}

#[repr(C)]
struct statmount {
    size: uint32_t,
    __spare1: uint32_t,
    mask: uint64_t,
    sb_dev_major: uint32_t,
    sb_dev_minor: uint32_t,
    sb_magic: uint64_t,
    sb_flags: uint32_t,
    fs_type: uint32_t,
    mnt_id: uint64_t,
    mnt_parent_id: uint64_t,
    mnt_id_old: uint32_t,
    mnt_parent_id_old: uint32_t,
    mnt_attr: uint64_t,
    mnt_propagation: uint64_t,
    mnt_peer_group: uint64_t,
    mnt_master: uint64_t,
    propagate_from: uint64_t,
    mnt_root: uint32_t,
    mnt_point: uint32_t,
    __spare2: [uint64_t; 50],
    str_: [c_char; 0],
}

unsafe extern "C" {
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn _exit(status: c_int) -> !;
    fn getpid() -> pid_t;
    fn getuid() -> c_uint;
    fn close(fd: c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn fchdir(fd: c_int) -> c_int;
    fn chroot(path: *const c_char) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);

    fn sys_clone3(args: *mut __clone_args, size: usize) -> pid_t;
    fn enter_userns() -> c_int;
    fn wait_for_pid(pid: pid_t) -> c_int;
    fn count_mounts() -> ssize_t;
    fn get_unique_mnt_id(path: *const c_char) -> uint64_t;
    fn statmount_alloc(
        mnt_id: uint64_t,
        request_mask: uint64_t,
        mask: uint64_t,
        flags: c_uint,
    ) -> *mut statmount;
    fn listmount(
        mnt_id: uint64_t,
        last_mnt_id: uint64_t,
        request_mask: uint64_t,
        buf: *mut uint64_t,
        bufsize: usize,
        flags: c_uint,
    ) -> ssize_t;
    fn sys_fsopen(fs_name: *const c_char, flags: c_uint) -> c_int;
    fn sys_fsconfig(
        fd: c_int,
        cmd: c_uint,
        key: *const c_char,
        value: *const c_char,
        aux: c_int,
    ) -> c_int;
    fn sys_fsmount(fs_fd: c_int, flags: c_uint, attr_flags: c_uint) -> c_int;
    fn sys_move_mount(
        from_dfd: c_int,
        from_pathname: *const c_char,
        to_dfd: c_int,
        to_pathname: *const c_char,
        flags: c_uint,
    ) -> c_int;
    fn __errno_location() -> *mut c_int;
}

const CLONE_EMPTY_MNTNS: uint64_t = 0x2000000000;
const UNKNOWN_CLONE_FLAG: uint64_t = 0x800000000;

// Constants supplied by Linux and selftest headers in the original C source.
extern "Rust" {
    static CLONE_NEWNS: uint64_t;
    static CLONE_NEWUSER: uint64_t;
    static CLONE_NEWUTS: uint64_t;
    static CLONE_NEWIPC: uint64_t;
    static CLONE_NEWPID: uint64_t;
    static CLONE_FS: uint64_t;
    static CLONE_PIDFD: uint64_t;
    static SIGCHLD: c_int;
    static EINVAL: c_int;
    static EPERM: c_int;
    static EEXIST: c_int;
    static O_CREAT: c_int;
    static O_RDWR: c_int;
    static O_RDONLY: c_int;
    static F_OK: c_int;
    static AT_FDCWD: c_int;
    static MS_REC: c_ulong;
    static MS_PRIVATE: c_ulong;
    static MS_BIND: c_ulong;
    static STATMOUNT_MNT_BASIC: uint64_t;
    static STATMOUNT_MNT_POINT: uint64_t;
    static STATMOUNT_FS_TYPE: uint64_t;
    static LSMT_ROOT: uint64_t;
    static FSCONFIG_SET_STRING: c_uint;
    static FSCONFIG_CMD_CREATE: c_uint;
    static MOVE_MOUNT_F_EMPTY_PATH: c_uint;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn clone3_empty_mntns(extra_flags: uint64_t) -> pid_t {
    let mut args = __clone_args {
        flags: CLONE_EMPTY_MNTNS | extra_flags,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: SIGCHLD as uint64_t,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };

    sys_clone3(&mut args, size_of::<__clone_args>())
}

unsafe fn clone3_empty_mntns_supported() -> bool {
    let mut pid: pid_t;
    let mut status: c_int = 0;

    pid = fork();
    if pid < 0 {
        return false;
    }

    if pid == 0 {
        if enter_userns() != 0 {
            _exit(1);
        }

        pid = clone3_empty_mntns(0);
        if pid < 0 {
            _exit(1);
        }

        if pid == 0 {
            _exit(0);
        }

        _exit((wait_for_pid(pid) != 0) as c_int);
    }

    if waitpid(pid, &mut status, 0) != pid {
        return false;
    }

    if !WIFEXITED(status) {
        return false;
    }

    WEXITSTATUS(status) == 0
}

fixture!(clone3_empty_mntns, {});

fixture_setup!(clone3_empty_mntns, {
    unsafe {
        if !clone3_empty_mntns_supported() {
            skip_return!("CLONE_EMPTY_MNTNS via clone3 not supported");
        }
    }
});

fixture_teardown!(clone3_empty_mntns, {});

/*
 * Basic clone3() with CLONE_EMPTY_MNTNS: child gets empty mount namespace
 * with exactly 1 mount and root == cwd.
 */
test_f!(clone3_empty_mntns, basic, {
    unsafe {
        let mut pid: pid_t;
        let inner: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            if enter_userns() != 0 {
                _exit(1);
            }

            inner = clone3_empty_mntns(0);
            if inner < 0 {
                _exit(2);
            }

            if inner == 0 {
                let root_id: uint64_t;
                let cwd_id: uint64_t;

                if count_mounts() != 1 {
                    _exit(3);
                }

                root_id = get_unique_mnt_id(c"/".as_ptr());
                cwd_id = get_unique_mnt_id(c".".as_ptr());
                if root_id == 0 || cwd_id == 0 {
                    _exit(4);
                }

                if root_id != cwd_id {
                    _exit(5);
                }

                _exit(0);
            }

            _exit(wait_for_pid(inner));
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * CLONE_EMPTY_MNTNS implies CLONE_NEWNS.  Verify that it works without
 * explicitly setting CLONE_NEWNS (tests fork.c:2627-2630).
 */
test_f!(clone3_empty_mntns, implies_newns, {
    unsafe {
        let mut pid: pid_t;
        let inner: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            let parent_mounts: ssize_t;

            if enter_userns() != 0 {
                _exit(1);
            }

            /* Verify we have mounts in our current namespace. */
            parent_mounts = count_mounts();
            if parent_mounts < 1 {
                _exit(2);
            }

            /* Only CLONE_EMPTY_MNTNS, no explicit CLONE_NEWNS. */
            inner = clone3_empty_mntns(0);
            if inner < 0 {
                _exit(3);
            }

            if inner == 0 {
                if count_mounts() != 1 {
                    _exit(4);
                }

                _exit(0);
            }

            /* Parent still has its mounts. */
            if count_mounts() != parent_mounts {
                _exit(5);
            }

            _exit(wait_for_pid(inner));
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * Helper macro: generate a test that clones with CLONE_EMPTY_MNTNS |
 * @extra_flags and verifies the child has exactly one mount.
 */
macro_rules! TEST_CLONE3_FLAGS {
    ($test_name:ident, $extra_flags:expr) => {
        test_f!(clone3_empty_mntns, $test_name, {
            unsafe {
                let mut pid: pid_t;
                let inner: pid_t;

                pid = fork();
                assert_ge!(pid, 0);

                if pid == 0 {
                    if enter_userns() != 0 {
                        _exit(1);
                    }

                    inner = clone3_empty_mntns($extra_flags);
                    if inner < 0 {
                        _exit(2);
                    }

                    if inner == 0 {
                        if count_mounts() != 1 {
                            _exit(3);
                        }
                        _exit(0);
                    }

                    _exit(wait_for_pid(inner));
                }

                assert_eq!(wait_for_pid(pid), 0);
            }
        });
    };
}

/* Redundant CLONE_NEWNS | CLONE_EMPTY_MNTNS should succeed. */
TEST_CLONE3_FLAGS!(with_explicit_newns, CLONE_NEWNS);

/* CLONE_EMPTY_MNTNS combined with CLONE_NEWUSER. */
TEST_CLONE3_FLAGS!(with_newuser, CLONE_NEWUSER);

/* CLONE_EMPTY_MNTNS combined with other namespace flags. */
TEST_CLONE3_FLAGS!(with_other_ns_flags, CLONE_NEWUTS | CLONE_NEWIPC);

/*
 * CLONE_EMPTY_MNTNS combined with CLONE_NEWPID.
 */
test_f!(clone3_empty_mntns, with_newpid, {
    unsafe {
        let mut pid: pid_t;
        let inner: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            if enter_userns() != 0 {
                _exit(1);
            }

            inner = clone3_empty_mntns(CLONE_NEWPID);
            if inner < 0 {
                _exit(2);
            }

            if inner == 0 {
                if count_mounts() != 1 {
                    _exit(3);
                }

                /* In a new PID namespace, getpid() returns 1. */
                if getpid() != 1 {
                    _exit(4);
                }

                _exit(0);
            }

            _exit(wait_for_pid(inner));
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * CLONE_EMPTY_MNTNS | CLONE_FS must fail because the implied CLONE_NEWNS
 * and CLONE_FS are mutually exclusive (fork.c:1981).
 */
test_f!(clone3_empty_mntns, with_clone_fs_fails, {
    unsafe {
        let mut pid: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            let mut args = __clone_args {
                flags: CLONE_EMPTY_MNTNS | CLONE_FS,
                pidfd: 0,
                child_tid: 0,
                parent_tid: 0,
                exit_signal: SIGCHLD as uint64_t,
                stack: 0,
                stack_size: 0,
                tls: 0,
                set_tid: 0,
                set_tid_size: 0,
                cgroup: 0,
            };
            let ret: pid_t;

            if enter_userns() != 0 {
                _exit(1);
            }

            ret = sys_clone3(&mut args, size_of::<__clone_args>());
            if ret >= 0 {
                if ret == 0 {
                    _exit(0);
                }
                wait_for_pid(ret);
                _exit(2);
            }

            if errno() != EINVAL {
                _exit(3);
            }

            _exit(0);
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * CLONE_EMPTY_MNTNS combined with CLONE_PIDFD returns a valid pidfd.
 */
test_f!(clone3_empty_mntns, with_pidfd, {
    unsafe {
        let mut pid: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            let mut args = __clone_args {
                flags: CLONE_EMPTY_MNTNS | CLONE_PIDFD,
                pidfd: 0,
                child_tid: 0,
                parent_tid: 0,
                exit_signal: SIGCHLD as uint64_t,
                stack: 0,
                stack_size: 0,
                tls: 0,
                set_tid: 0,
                set_tid_size: 0,
                cgroup: 0,
            };
            let mut pidfd: c_int = -1;
            let inner: pid_t;

            if enter_userns() != 0 {
                _exit(1);
            }

            args.pidfd = (&mut pidfd as *mut c_int) as uintptr_t as uint64_t;

            inner = sys_clone3(&mut args, size_of::<__clone_args>());
            if inner < 0 {
                _exit(2);
            }

            if inner == 0 {
                if count_mounts() != 1 {
                    _exit(3);
                }

                _exit(0);
            }

            /* Verify we got a valid pidfd. */
            if pidfd < 0 {
                _exit(4);
            }

            close(pidfd);
            _exit(wait_for_pid(inner));
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * clone3 without CAP_SYS_ADMIN must fail with EPERM.
 */
test_f!(clone3_empty_mntns, eperm_without_caps, {
    unsafe {
        let mut pid: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            let ret: pid_t;

            /* Skip if already root. */
            if getuid() == 0 {
                _exit(0);
            }

            ret = clone3_empty_mntns(0);
            if ret >= 0 {
                if ret == 0 {
                    _exit(0);
                }
                wait_for_pid(ret);
                _exit(1);
            }

            if errno() != EPERM {
                _exit(2);
            }

            _exit(0);
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * Parent's mount namespace is unaffected after clone3 with CLONE_EMPTY_MNTNS.
 */
test_f!(clone3_empty_mntns, parent_unchanged, {
    unsafe {
        let mut pid: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            let nr_before: ssize_t;
            let nr_after: ssize_t;
            let inner: pid_t;

            if enter_userns() != 0 {
                _exit(1);
            }

            nr_before = count_mounts();
            if nr_before < 1 {
                _exit(2);
            }

            inner = clone3_empty_mntns(0);
            if inner < 0 {
                _exit(3);
            }

            if inner == 0 {
                _exit(0);
            }

            if wait_for_pid(inner) != 0 {
                _exit(4);
            }

            nr_after = count_mounts();
            if nr_after != nr_before {
                _exit(5);
            }

            _exit(0);
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * Parent with many mounts: child still gets exactly 1 mount.
 */
test_f!(clone3_empty_mntns, many_parent_mounts, {
    unsafe {
        let mut pid: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            let mut tmpdir = *b"/tmp/clone3_mntns_test.XXXXXX\0";
            let inner: pid_t;
            let mut i: c_int;

            if enter_userns() != 0 {
                _exit(1);
            }

            if unshare(CLONE_NEWNS as c_int) != 0 {
                _exit(2);
            }

            if mount(ptr::null(), c"/".as_ptr(), ptr::null(), MS_REC | MS_PRIVATE, ptr::null()) != 0 {
                _exit(3);
            }

            if mkdtemp(tmpdir.as_mut_ptr() as *mut c_char).is_null() {
                _exit(4);
            }

            if mount(c"tmpfs".as_ptr(), tmpdir.as_ptr() as *const c_char, c"tmpfs".as_ptr(), 0, c"size=1M".as_ptr() as *const c_void) != 0 {
                _exit(5);
            }

            i = 0;
            while i < 5 {
                let mut subdir = [0 as c_char; 256];

                snprintf(
                    subdir.as_mut_ptr(),
                    size_of::<[c_char; 256]>(),
                    c"%s/sub%d".as_ptr(),
                    tmpdir.as_ptr() as *const c_char,
                    i,
                );
                if mkdir(subdir.as_ptr(), 0o755) != 0 && errno() != EEXIST {
                    _exit(6);
                }
                if mount(subdir.as_ptr(), subdir.as_ptr(), ptr::null(), MS_BIND, ptr::null()) != 0 {
                    _exit(7);
                }
                i += 1;
            }

            if count_mounts() < 5 {
                _exit(8);
            }

            inner = clone3_empty_mntns(0);
            if inner < 0 {
                _exit(9);
            }

            if inner == 0 {
                if count_mounts() != 1 {
                    _exit(10);
                }

                _exit(0);
            }

            _exit(wait_for_pid(inner));
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * Verify the child's root mount is nullfs with expected statmount properties.
 */
test_f!(clone3_empty_mntns, mount_properties, {
    unsafe {
        let mut pid: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            let inner: pid_t;

            if enter_userns() != 0 {
                _exit(1);
            }

            inner = clone3_empty_mntns(0);
            if inner < 0 {
                _exit(2);
            }

            if inner == 0 {
                let mut sm: *mut statmount;
                let root_id: uint64_t;

                root_id = get_unique_mnt_id(c"/".as_ptr());
                if root_id == 0 {
                    _exit(3);
                }

                sm = statmount_alloc(
                    root_id,
                    0,
                    STATMOUNT_MNT_BASIC | STATMOUNT_MNT_POINT | STATMOUNT_FS_TYPE,
                    0,
                );
                if sm.is_null() {
                    _exit(4);
                }

                /* Root mount point is "/". */
                if ((*sm).mask & STATMOUNT_MNT_POINT) == 0 {
                    _exit(5);
                }
                if strcmp((*sm).str_.as_ptr().add((*sm).mnt_point as usize), c"/".as_ptr()) != 0 {
                    _exit(6);
                }

                /* Filesystem type is nullfs. */
                if ((*sm).mask & STATMOUNT_FS_TYPE) == 0 {
                    _exit(7);
                }
                if strcmp((*sm).str_.as_ptr().add((*sm).fs_type as usize), c"nullfs".as_ptr()) != 0 {
                    _exit(8);
                }

                /* Root mount is its own parent. */
                if ((*sm).mask & STATMOUNT_MNT_BASIC) == 0 {
                    _exit(9);
                }
                if (*sm).mnt_parent_id != (*sm).mnt_id {
                    _exit(10);
                }

                free(sm as *mut c_void);
                _exit(0);
            }

            _exit(wait_for_pid(inner));
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * Listmount returns only the root mount in the child's empty namespace.
 */
test_f!(clone3_empty_mntns, listmount_single_entry, {
    unsafe {
        let mut pid: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            let inner: pid_t;

            if enter_userns() != 0 {
                _exit(1);
            }

            inner = clone3_empty_mntns(0);
            if inner < 0 {
                _exit(2);
            }

            if inner == 0 {
                let mut list: [uint64_t; 16] = [0; 16];
                let nr_mounts: ssize_t;
                let root_id: uint64_t;

                nr_mounts = listmount(LSMT_ROOT, 0, 0, list.as_mut_ptr(), 16, 0);
                if nr_mounts != 1 {
                    _exit(3);
                }

                root_id = get_unique_mnt_id(c"/".as_ptr());
                if root_id == 0 {
                    _exit(4);
                }

                if list[0] != root_id {
                    _exit(5);
                }

                _exit(0);
            }

            _exit(wait_for_pid(inner));
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * Child can mount tmpfs over nullfs root (the primary container use case).
 *
 * Uses the new mount API (fsopen/fsmount/move_mount) because resolving
 * "/" returns the process root directly without following overmounts.
 * The mount fd from fsmount lets us fchdir + chroot into the new tmpfs.
 */
test_f!(clone3_empty_mntns, child_overmount_tmpfs, {
    unsafe {
        let mut pid: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            let inner: pid_t;

            if enter_userns() != 0 {
                _exit(1);
            }

            inner = clone3_empty_mntns(0);
            if inner < 0 {
                _exit(2);
            }

            if inner == 0 {
                let mut sm: *mut statmount;
                let mut root_id: uint64_t;
                let fd: c_int;
                let fsfd: c_int;
                let mntfd: c_int;

                if count_mounts() != 1 {
                    _exit(3);
                }

                /* Verify root is nullfs. */
                root_id = get_unique_mnt_id(c"/".as_ptr());
                if root_id == 0 {
                    _exit(4);
                }

                sm = statmount_alloc(root_id, 0, STATMOUNT_FS_TYPE, 0);
                if sm.is_null() {
                    _exit(5);
                }
                if ((*sm).mask & STATMOUNT_FS_TYPE) == 0 {
                    _exit(6);
                }
                if strcmp((*sm).str_.as_ptr().add((*sm).fs_type as usize), c"nullfs".as_ptr()) != 0 {
                    _exit(7);
                }
                free(sm as *mut c_void);

                /* Create tmpfs via the new mount API. */
                fsfd = sys_fsopen(c"tmpfs".as_ptr(), 0);
                if fsfd < 0 {
                    _exit(8);
                }

                if sys_fsconfig(fsfd, FSCONFIG_SET_STRING, c"size".as_ptr(), c"1M".as_ptr(), 0) != 0 {
                    close(fsfd);
                    _exit(9);
                }

                if sys_fsconfig(fsfd, FSCONFIG_CMD_CREATE, ptr::null(), ptr::null(), 0) != 0 {
                    close(fsfd);
                    _exit(10);
                }

                mntfd = sys_fsmount(fsfd, 0, 0);
                close(fsfd);
                if mntfd < 0 {
                    _exit(11);
                }

                /* Attach tmpfs to "/". */
                if sys_move_mount(mntfd, c"".as_ptr(), AT_FDCWD, c"/".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH) != 0 {
                    close(mntfd);
                    _exit(12);
                }

                if count_mounts() != 2 {
                    close(mntfd);
                    _exit(13);
                }

                /* Enter the tmpfs. */
                if fchdir(mntfd) != 0 {
                    close(mntfd);
                    _exit(14);
                }

                if chroot(c".".as_ptr()) != 0 {
                    close(mntfd);
                    _exit(15);
                }

                close(mntfd);

                /* Verify "/" is now tmpfs. */
                root_id = get_unique_mnt_id(c"/".as_ptr());
                if root_id == 0 {
                    _exit(16);
                }

                sm = statmount_alloc(root_id, 0, STATMOUNT_FS_TYPE, 0);
                if sm.is_null() {
                    _exit(17);
                }
                if ((*sm).mask & STATMOUNT_FS_TYPE) == 0 {
                    _exit(18);
                }
                if strcmp((*sm).str_.as_ptr().add((*sm).fs_type as usize), c"tmpfs".as_ptr()) != 0 {
                    _exit(19);
                }
                free(sm as *mut c_void);

                /* Verify tmpfs is writable. */
                fd = open(c"/testfile".as_ptr(), O_CREAT | O_RDWR, 0o644);
                if fd < 0 {
                    _exit(20);
                }

                if write(fd, c"test".as_ptr() as *const c_void, 4) != 4 {
                    close(fd);
                    _exit(21);
                }
                close(fd);

                if access(c"/testfile".as_ptr(), F_OK) != 0 {
                    _exit(22);
                }

                _exit(0);
            }

            _exit(wait_for_pid(inner));
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * Multiple clone3 calls with CLONE_EMPTY_MNTNS produce children with
 * distinct mount namespace root mount IDs.
 */
test_f!(clone3_empty_mntns, repeated, {
    unsafe {
        let mut pid: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            let mut pipe1: [c_int; 2] = [0; 2];
            let mut pipe2: [c_int; 2] = [0; 2];
            let mut id1: uint64_t = 0;
            let mut id2: uint64_t = 0;
            let inner1: pid_t;
            let inner2: pid_t;

            if enter_userns() != 0 {
                _exit(1);
            }

            if pipe(pipe1.as_mut_ptr()) != 0 || pipe(pipe2.as_mut_ptr()) != 0 {
                _exit(2);
            }

            inner1 = clone3_empty_mntns(0);
            if inner1 < 0 {
                _exit(3);
            }

            if inner1 == 0 {
                let root_id: uint64_t;

                close(pipe1[0]);
                root_id = get_unique_mnt_id(c"/".as_ptr());
                if write(pipe1[1], &root_id as *const uint64_t as *const c_void, size_of::<uint64_t>()) != size_of::<uint64_t>() as ssize_t {
                    _exit(1);
                }
                close(pipe1[1]);
                _exit(0);
            }

            inner2 = clone3_empty_mntns(0);
            if inner2 < 0 {
                _exit(4);
            }

            if inner2 == 0 {
                let root_id: uint64_t;

                close(pipe2[0]);
                root_id = get_unique_mnt_id(c"/".as_ptr());
                if write(pipe2[1], &root_id as *const uint64_t as *const c_void, size_of::<uint64_t>()) != size_of::<uint64_t>() as ssize_t {
                    _exit(1);
                }
                close(pipe2[1]);
                _exit(0);
            }

            close(pipe1[1]);
            close(pipe2[1]);

            if read(pipe1[0], &mut id1 as *mut uint64_t as *mut c_void, size_of::<uint64_t>()) != size_of::<uint64_t>() as ssize_t {
                _exit(5);
            }
            if read(pipe2[0], &mut id2 as *mut uint64_t as *mut c_void, size_of::<uint64_t>()) != size_of::<uint64_t>() as ssize_t {
                _exit(6);
            }

            close(pipe1[0]);
            close(pipe2[0]);

            if wait_for_pid(inner1) != 0 || wait_for_pid(inner2) != 0 {
                _exit(7);
            }

            /* Each child must have a distinct root mount ID. */
            if id1 == 0 || id2 == 0 {
                _exit(8);
            }
            if id1 == id2 {
                _exit(9);
            }

            _exit(0);
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * Verify setns() into a child's empty mount namespace works.
 */
test_f!(clone3_empty_mntns, setns_into_child_mntns, {
    unsafe {
        let mut pid: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            let mut pipe_fd: [c_int; 2] = [0; 2];
            let inner: pid_t;
            let mut c: c_char = 0;

            if enter_userns() != 0 {
                _exit(1);
            }

            if pipe(pipe_fd.as_mut_ptr()) != 0 {
                _exit(2);
            }

            inner = clone3_empty_mntns(0);
            if inner < 0 {
                _exit(3);
            }

            if inner == 0 {
                /* Signal parent we're ready. */
                close(pipe_fd[0]);
                if write(pipe_fd[1], c"r".as_ptr() as *const c_void, 1) != 1 {
                    _exit(1);
                }

                /*
                 * Wait for parent to finish.  Reading from our
                 * write end will block until the parent closes
                 * its read end, giving us an implicit barrier.
                 */
                if read(pipe_fd[1], &mut c as *mut c_char as *mut c_void, 1) < 0 {
                    ;
                }
                close(pipe_fd[1]);
                _exit(0);
            }

            close(pipe_fd[1]);

            /* Wait for child to be ready. */
            if read(pipe_fd[0], &mut c as *mut c_char as *mut c_void, 1) != 1 {
                _exit(4);
            }

            /* Open child's mount namespace. */
            {
                let mut path = [0 as c_char; 64];
                let mntns_fd: c_int;

                snprintf(path.as_mut_ptr(), size_of::<[c_char; 64]>(), c"/proc/%d/ns/mnt".as_ptr(), inner);
                mntns_fd = open(path.as_ptr(), O_RDONLY);
                if mntns_fd < 0 {
                    _exit(5);
                }

                if setns(mntns_fd, CLONE_NEWNS as c_int) != 0 {
                    _exit(6);
                }

                close(mntns_fd);
            }

            /* Now we should be in the child's empty mntns. */
            if count_mounts() != 1 {
                _exit(7);
            }

            close(pipe_fd[0]);
            _exit(wait_for_pid(inner));
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * Tests below do not require CLONE_EMPTY_MNTNS support.
 */

/*
 * Unknown 64-bit flags beyond the known set are rejected.
 */
test!(unknown_flags_rejected, {
    unsafe {
        let mut pid: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            let mut args = __clone_args {
                flags: UNKNOWN_CLONE_FLAG,
                pidfd: 0,
                child_tid: 0,
                parent_tid: 0,
                exit_signal: SIGCHLD as uint64_t,
                stack: 0,
                stack_size: 0,
                tls: 0,
                set_tid: 0,
                set_tid_size: 0,
                cgroup: 0,
            };
            let ret: pid_t;

            ret = sys_clone3(&mut args, size_of::<__clone_args>());
            if ret >= 0 {
                if ret == 0 {
                    _exit(0);
                }
                wait_for_pid(ret);
                _exit(1);
            }

            if errno() != EINVAL {
                _exit(2);
            }

            _exit(0);
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

/*
 * Regular clone3 with CLONE_NEWNS (without CLONE_EMPTY_MNTNS) still
 * copies the full mount tree.
 */
test!(clone3_newns_full_copy, {
    unsafe {
        let mut pid: pid_t;

        pid = fork();
        assert_ge!(pid, 0);

        if pid == 0 {
            let mut args = __clone_args {
                flags: CLONE_NEWNS,
                pidfd: 0,
                child_tid: 0,
                parent_tid: 0,
                exit_signal: SIGCHLD as uint64_t,
                stack: 0,
                stack_size: 0,
                tls: 0,
                set_tid: 0,
                set_tid_size: 0,
                cgroup: 0,
            };
            let parent_mounts: ssize_t;
            let inner: pid_t;

            if enter_userns() != 0 {
                _exit(1);
            }

            parent_mounts = count_mounts();
            if parent_mounts < 1 {
                _exit(2);
            }

            inner = sys_clone3(&mut args, size_of::<__clone_args>());
            if inner < 0 {
                _exit(3);
            }

            if inner == 0 {
                /* Full copy should have at least as many mounts. */
                if count_mounts() < parent_mounts {
                    _exit(1);
                }

                _exit(0);
            }

            _exit(wait_for_pid(inner));
        }

        assert_eq!(wait_for_pid(pid), 0);
    }
});

test_harness_main!();
