// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

/* Translated from:
 * #include <test_progs.h>
 * #include "testing_helpers.h"
 * #include "livepatch_trampoline.skel.h"
 */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const LIVEPATCH_ENABLED_PATH: &[u8] = b"/sys/kernel/livepatch/livepatch_sample/enabled\0";

const PATH_BUF_SIZE: usize = 4096;
const O_RDONLY: c_int = 0;
const F_OK: c_int = 0;
const ENOENT: c_int = 2;
const VERBOSE_NONE: c_int = 0;

#[repr(C)]
pub struct livepatch_trampoline_bss {
    pub my_pid: c_int,
    pub fentry_hit: c_int,
    pub fexit_hit: c_int,
}

#[repr(C)]
pub struct livepatch_trampoline_progs {
    pub fexit_cmdline: *mut bpf_program,
    pub fentry_cmdline: *mut bpf_program,
}

#[repr(C)]
pub struct livepatch_trampoline_links {
    pub fexit_cmdline: *mut bpf_link,
    pub fentry_cmdline: *mut bpf_link,
}

#[repr(C)]
pub struct livepatch_trampoline {
    pub bss: *mut livepatch_trampoline_bss,
    pub progs: livepatch_trampoline_progs,
    pub links: livepatch_trampoline_links,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

unsafe extern "C" {
    static env_verbosity: c_int;

    fn getenv(name: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn load_module(path: *const c_char, verbose: bool) -> c_int;
    fn unload_module(name: *const c_char, verbose: bool);
    fn access(path: *const c_char, amode: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn getpid() -> c_int;

    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(value: isize, expected: isize, name: *const c_char) -> bool;
    fn ASSERT_OK(value: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn livepatch_trampoline__open_and_load() -> *mut livepatch_trampoline;
    fn livepatch_trampoline__attach(skel: *mut livepatch_trampoline) -> c_int;
    fn livepatch_trampoline__destroy(skel: *mut livepatch_trampoline);
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
}

unsafe fn load_livepatch() -> c_int {
    let mut path = [0 as c_char; PATH_BUF_SIZE];

    /* CI will set KBUILD_OUTPUT */
    let kbuild_output = getenv(c"KBUILD_OUTPUT".as_ptr());
    let base = if !kbuild_output.is_null() {
        kbuild_output as *const c_char
    } else {
        c"../../../..".as_ptr()
    };

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        c"%s/samples/livepatch/livepatch-sample.ko".as_ptr(),
        base,
    );

    load_module(path.as_ptr(), env_verbosity > VERBOSE_NONE)
}

unsafe fn unload_livepatch() {
    /* Disable the livepatch before unloading the module */
    if access(LIVEPATCH_ENABLED_PATH.as_ptr() as *const c_char, F_OK) == 0 {
        system(c"echo 0 > /sys/kernel/livepatch/livepatch_sample/enabled".as_ptr());
    }

    unload_module(c"livepatch_sample".as_ptr(), env_verbosity > VERBOSE_NONE);
}

unsafe fn read_proc_cmdline() {
    let mut buf = [0 as c_char; PATH_BUF_SIZE];
    let fd: c_int;
    let ret: isize;

    fd = open(c"/proc/cmdline".as_ptr(), O_RDONLY);
    if !ASSERT_OK_FD(fd, c"open /proc/cmdline".as_ptr()) {
        return;
    }

    ret = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
    if !ASSERT_GT(ret, 0, c"read /proc/cmdline".as_ptr()) {
        close(fd);
        return;
    }

    ASSERT_OK(
        strncmp(
            buf.as_ptr(),
            c"this has been live patched".as_ptr(),
            26,
        ),
        c"strncmp".as_ptr(),
    );

    close(fd);
}

unsafe fn __test_livepatch_trampoline(fexit_first: bool) {
    let mut skel: *mut livepatch_trampoline = ptr::null_mut();
    let err: c_int;

    skel = livepatch_trampoline__open_and_load();
    if !ASSERT_OK_PTR(skel, c"skel_open_and_load".as_ptr()) {
        livepatch_trampoline__destroy(skel);
        return;
    }

    (*(*skel).bss).my_pid = getpid();

    if !fexit_first {
        /* fentry program is loaded first by default */
        err = livepatch_trampoline__attach(skel);
        if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
            livepatch_trampoline__destroy(skel);
            return;
        }
    } else {
        /* Manually load fexit program first. */
        (*skel).links.fexit_cmdline = bpf_program__attach((*skel).progs.fexit_cmdline);
        if !ASSERT_OK_PTR((*skel).links.fexit_cmdline, c"attach_fexit".as_ptr()) {
            livepatch_trampoline__destroy(skel);
            return;
        }

        (*skel).links.fentry_cmdline = bpf_program__attach((*skel).progs.fentry_cmdline);
        if !ASSERT_OK_PTR((*skel).links.fentry_cmdline, c"attach_fentry".as_ptr()) {
            livepatch_trampoline__destroy(skel);
            return;
        }
    }

    read_proc_cmdline();

    ASSERT_EQ((*(*skel).bss).fentry_hit, 1, c"fentry_hit".as_ptr());
    ASSERT_EQ((*(*skel).bss).fexit_hit, 1, c"fexit_hit".as_ptr());

    livepatch_trampoline__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_livepatch_trampoline() {
    let mut retry_cnt: c_int = 0;
    let mut err: c_int;

    /* Skip if kernel was built without CONFIG_LIVEPATCH */
    if access(c"/sys/kernel/livepatch".as_ptr(), F_OK) != 0 {
        test__skip();
        return;
    }

    loop {
        err = load_livepatch();
        if err != 0 {
            if err == -ENOENT {
                test__skip();
                return;
            }

            if retry_cnt != 0 {
                ASSERT_OK(1, c"load_livepatch".as_ptr());
                break;
            }
            /*
             * Something else (previous run of the same test?) loaded
             * the KLP module. Unload the KLP module and retry.
             */
            unload_livepatch();
            retry_cnt += 1;
            continue;
        }

        if test__start_subtest(c"fentry_first".as_ptr()) {
            __test_livepatch_trampoline(false);
        }

        if test__start_subtest(c"fexit_first".as_ptr()) {
            __test_livepatch_trampoline(true);
        }
        break;
    }

    unload_livepatch();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
