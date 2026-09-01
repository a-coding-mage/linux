// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Author: Aleksa Sarai <cyphar@cyphar.com>
 * Copyright (C) 2018-2019 SUSE LLC.
 */

// C dependencies: errno.h, fcntl.h, sched.h, sys/stat.h, sys/types.h,
// sys/mount.h, sys/mman.h, sys/prctl.h, signal.h, stdio.h, stdlib.h,
// stdbool.h, string.h, syscall.h, limits.h, unistd.h.
// Local dependencies: helpers.h, kselftest_harness.h.

use core::ffi::{c_char, c_int, c_void};

const ROUNDS: c_int = 400000;

extern "C" {
    static openat2_supported: bool;

    fn fork() -> pid_t;
    fn prctl(option: c_int, ...) -> c_int;
    fn renameat2(
        olddirfd: c_int,
        oldpath: *const c_char,
        newdirfd: c_int,
        newpath: *const c_char,
        flags: c_uint,
    ) -> c_int;
    fn exit(status: c_int) -> !;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn mkdirat(dirfd: c_int, pathname: *const c_char, mode: mode_t) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn sys_openat2(dirfd: c_int, pathname: *const c_char, how: *const open_how) -> c_int;
    fn sys_openat(dirfd: c_int, pathname: *const c_char, how: *const open_how) -> c_int;
    fn fdequal(
        _metadata: *mut __test_metadata,
        fd1: c_int,
        fd2: c_int,
        path: *const c_char,
    ) -> bool;
}

type c_uint = u32;
type mode_t = u32;
type pid_t = c_int;

const NULL: *mut c_void = core::ptr::null_mut();

extern "C" {
    static O_PATH: c_int;
    static O_DIRECTORY: c_int;
    static PR_SET_PDEATHSIG: c_int;
    static SIGKILL: c_int;
    static RENAME_EXCHANGE: c_uint;
    static RESOLVE_BENEATH: c_uint;
    static RESOLVE_IN_ROOT: c_uint;
    static EAGAIN: c_int;
    static EXDEV: c_int;
    static ENOENT: c_int;
}

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct open_how {
    pub flags: u64,
    pub mode: u64,
    pub resolve: u64,
}

// Swap @dirfd/@a and @dirfd/@b constantly. Parent must kill this process.
pub unsafe fn spawn_attack(
    _metadata: *mut __test_metadata,
    dirfd: c_int,
    a: *mut c_char,
    b: *mut c_char,
) -> pid_t {
    let child: pid_t = fork();
    if child != 0 {
        return child;
    }

    // If the parent (the test process) dies, kill ourselves too.
    ASSERT_EQ!(prctl(PR_SET_PDEATHSIG, SIGKILL), 0);

    // Swap @a and @b.
    loop {
        renameat2(dirfd, a, dirfd, b, RENAME_EXCHANGE);
    }
    #[allow(unreachable_code)]
    exit(1);
}

/*
 * Construct a test directory with the following structure:
 *
 * root/
 * |-- a/
 * |   `-- c/
 * `-- b/
 */
#[repr(C)]
pub struct rename_attack {
    dfd: c_int,
    afd: c_int,
    child: pid_t,
}

pub unsafe fn rename_attack_setup(self_: *mut rename_attack, _metadata: *mut __test_metadata) {
    let mut dirname = *b"/tmp/ksft-openat2-rename-attack.XXXXXX\0";

    (*self_).dfd = -1;
    (*self_).afd = -1;
    (*self_).child = 0;

    // Make the top-level directory.
    ASSERT_NE!(mkdtemp(dirname.as_mut_ptr() as *mut c_char), core::ptr::null_mut());
    (*self_).dfd = open(
        dirname.as_ptr() as *const c_char,
        O_PATH | O_DIRECTORY,
    );
    ASSERT_GE!((*self_).dfd, 0);

    ASSERT_EQ!(mkdirat((*self_).dfd, c"a".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat((*self_).dfd, c"b".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat((*self_).dfd, c"a/c".as_ptr(), 0o755), 0);

    (*self_).afd = openat((*self_).dfd, c"a".as_ptr(), O_PATH);
    ASSERT_GE!((*self_).afd, 0);

    (*self_).child = spawn_attack(
        _metadata,
        (*self_).dfd,
        c"a/c".as_ptr() as *mut c_char,
        c"b".as_ptr() as *mut c_char,
    );
    ASSERT_GT!((*self_).child, 0);
}

pub unsafe fn rename_attack_teardown(self_: *mut rename_attack) {
    if (*self_).child > 0 {
        kill((*self_).child, SIGKILL);
    }
    if (*self_).afd >= 0 {
        close((*self_).afd);
    }
    if (*self_).dfd >= 0 {
        close((*self_).dfd);
    }
}

#[repr(C)]
pub struct rename_attack_variant {
    resolve: c_int,
    name: *const c_char,
}

pub static rename_attack_resolve_beneath: rename_attack_variant = rename_attack_variant {
    resolve: unsafe { RESOLVE_BENEATH as c_int },
    name: c"RESOLVE_BENEATH".as_ptr(),
};

pub static rename_attack_resolve_in_root: rename_attack_variant = rename_attack_variant {
    resolve: unsafe { RESOLVE_IN_ROOT as c_int },
    name: c"RESOLVE_IN_ROOT".as_ptr(),
};

pub unsafe fn rename_attack_test(
    self_: *mut rename_attack,
    variant: *const rename_attack_variant,
    _metadata: *mut __test_metadata,
) {
    let mut escapes: c_int = 0;
    let mut successes: c_int = 0;
    let mut other_errs: c_int = 0;
    let mut exdevs: c_int = 0;
    let mut eagains: c_int = 0;
    let victim_path: *mut c_char = c"c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../..".as_ptr() as *mut c_char;
    let mut how = open_how {
        flags: O_PATH as u64,
        mode: 0,
        resolve: (*variant).resolve as u64,
    };

    if !openat2_supported {
        how.resolve = 0;
        TH_LOG!(c"openat2(2) unsupported -- using openat(2) instead".as_ptr());
    }

    for _i in 0..ROUNDS {
        let fd: c_int;

        if openat2_supported {
            fd = sys_openat2((*self_).afd, victim_path, &how);
        } else {
            fd = sys_openat((*self_).afd, victim_path, &how);
        }

        if fd < 0 {
            if fd == -EAGAIN {
                eagains += 1;
            } else if fd == -EXDEV {
                exdevs += 1;
            } else if fd == -ENOENT {
                escapes += 1; // escaped outside and got ENOENT...
            } else {
                other_errs += 1; // unexpected error
            }
        } else {
            if fdequal(_metadata, fd, (*self_).afd, NULL as *const c_char) {
                successes += 1;
            } else {
                escapes += 1; // we got an unexpected fd
            }
        }
        if fd >= 0 {
            close(fd);
        }
    }

    TH_LOG!(
        c"non-escapes: EAGAIN=%d EXDEV=%d E<other>=%d success=%d".as_ptr(),
        eagains,
        exdevs,
        other_errs,
        successes
    );
    ASSERT_EQ!(escapes, 0, {
        TH_LOG!(
            c"rename attack with %s (%d runs, got %d escapes)".as_ptr(),
            (*variant).name,
            ROUNDS,
            escapes
        );
    });
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
