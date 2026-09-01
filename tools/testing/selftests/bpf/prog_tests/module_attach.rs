// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/* Translated from:
 * #include <test_progs.h>
 * #include <stdbool.h>
 * #include "test_module_attach.skel.h"
 * #include "testing_helpers.h"
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_module_attach__bss {
    pub sz: c_int,
    pub retval: c_int,
    pub raw_tp_writable_bare_early_ret: bool,
    pub raw_tp_writable_bare_out_val: c_int,
    pub raw_tp_writable_bare_in_val: c_int,
}

#[repr(C)]
pub struct test_module_attach__progs {
    pub handle_raw_tp_writable_bare: *mut bpf_program,
}

#[repr(C)]
pub struct test_module_attach {
    pub obj: *mut bpf_object,
    pub bss: *mut test_module_attach__bss,
    pub progs: test_module_attach__progs,
}

unsafe extern "C" {
    static BPF_TESTMOD_TEST_FILE: *const c_char;

    fn __errno_location() -> *mut c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;

    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: isize, expected: isize, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;

    fn test_module_attach__open() -> *mut test_module_attach;
    fn test_module_attach__load(skel: *mut test_module_attach) -> c_int;
    fn test_module_attach__attach(skel: *mut test_module_attach) -> c_int;
    fn test_module_attach__destroy(skel: *mut test_module_attach);

    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__set_attach_target(
        prog: *mut bpf_program,
        attach_prog_fd: c_int,
        attach_func_name: *const c_char,
    ) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn trigger_module_test_read(sz: c_int) -> c_int;
    fn trigger_module_test_write(sz: c_int) -> c_int;
    fn try_unload_module(module_name: *const c_char, flags: c_int, wait: bool) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
}

const O_RDONLY: c_int = 0;
const EIO: c_int = 5;

static READ_TESTS: [*const c_char; 5] = [
    b"handle_raw_tp\0".as_ptr() as *const c_char,
    b"handle_tp_btf\0".as_ptr() as *const c_char,
    b"handle_fentry\0".as_ptr() as *const c_char,
    b"handle_fentry_explicit\0".as_ptr() as *const c_char,
    b"handle_fmod_ret\0".as_ptr() as *const c_char,
];

static DETACH_TESTS: [*const c_char; 3] = [
    b"handle_fentry\0".as_ptr() as *const c_char,
    b"handle_fexit\0".as_ptr() as *const c_char,
    b"kprobe_multi\0".as_ptr() as *const c_char,
];

const READ_SZ: c_int = 456;
const WRITE_SZ: c_int = 457;

unsafe fn errno_value() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn trigger_module_test_writable(val: *mut c_int) -> c_int {
    let fd: c_int;
    let mut err: c_int;
    let mut buf: [c_char; 65] = [0; 65];
    let rd: isize;

    fd = unsafe { open(BPF_TESTMOD_TEST_FILE, O_RDONLY) };
    err = -unsafe { errno_value() };
    if !unsafe { ASSERT_GE(fd, 0, b"testmode_file_open\0".as_ptr() as *const c_char) } {
        return err;
    }

    rd = unsafe { read(fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 65]>() - 1) };
    err = -unsafe { errno_value() };
    if !unsafe { ASSERT_GT(rd, 0, b"testmod_file_rd_val\0".as_ptr() as *const c_char) } {
        unsafe {
            close(fd);
        }
        return err;
    }

    buf[rd as usize] = b'\0' as c_char;
    unsafe {
        *val = strtol(buf.as_ptr(), null_mut(), 0) as c_int;
        close(fd);
    }

    0
}

unsafe fn test_module_attach_prog(
    prog_name: *const c_char,
    sz: c_int,
    attach_target: *const c_char,
    ret: c_int,
) {
    let skel: *mut test_module_attach;
    let prog: *mut bpf_program;
    let mut err: c_int;

    skel = unsafe { test_module_attach__open() };
    if !unsafe { ASSERT_OK_PTR(skel as *const c_void, b"module_attach open\0".as_ptr() as *const c_char) } {
        return;
    }

    prog = unsafe { bpf_object__find_program_by_name((*skel).obj, prog_name) };
    if !unsafe {
        ASSERT_OK_PTR(
            prog as *const c_void,
            b"module_attach find_program\0".as_ptr() as *const c_char,
        )
    } {
        goto_cleanup_module_attach_prog(skel);
        return;
    }
    unsafe {
        bpf_program__set_autoload(prog, true);
    }

    if !attach_target.is_null() {
        err = unsafe { bpf_program__set_attach_target(prog, 0, attach_target) };
        if !unsafe { ASSERT_OK(err, attach_target) } {
            goto_cleanup_module_attach_prog(skel);
            return;
        }
    }

    err = unsafe { test_module_attach__load(skel) };
    if !unsafe { ASSERT_OK(err, b"module_attach load\0".as_ptr() as *const c_char) } {
        goto_cleanup_module_attach_prog(skel);
        return;
    }

    err = unsafe { test_module_attach__attach(skel) };
    if !unsafe { ASSERT_OK(err, b"module_attach attach\0".as_ptr() as *const c_char) } {
        goto_cleanup_module_attach_prog(skel);
        return;
    }

    if sz != 0 {
        /* trigger both read and write though each test uses only one */
        unsafe {
            ASSERT_OK(trigger_module_test_read(sz), b"trigger_read\0".as_ptr() as *const c_char);
            ASSERT_OK(trigger_module_test_write(sz), b"trigger_write\0".as_ptr() as *const c_char);

            ASSERT_EQ((*(*skel).bss).sz, sz, prog_name);
        }
    }

    if ret != 0 {
        unsafe {
            ASSERT_EQ((*(*skel).bss).retval, ret, b"ret\0".as_ptr() as *const c_char);
        }
    }

    goto_cleanup_module_attach_prog(skel);
}

unsafe fn goto_cleanup_module_attach_prog(skel: *mut test_module_attach) {
    unsafe {
        test_module_attach__destroy(skel);
    }
}

unsafe fn test_module_attach_writable() {
    let bss: *mut test_module_attach__bss;
    let skel: *mut test_module_attach;
    let mut writable_val: c_int = 0;
    let mut err: c_int;

    skel = unsafe { test_module_attach__open() };
    if !unsafe { ASSERT_OK_PTR(skel as *const c_void, b"module_attach open\0".as_ptr() as *const c_char) } {
        return;
    }

    unsafe {
        bpf_program__set_autoload((*skel).progs.handle_raw_tp_writable_bare, true);
    }

    err = unsafe { test_module_attach__load(skel) };
    if !unsafe { ASSERT_OK(err, b"module_attach load\0".as_ptr() as *const c_char) } {
        goto_cleanup_module_attach_writable(skel);
        return;
    }

    bss = unsafe { (*skel).bss };

    err = unsafe { test_module_attach__attach(skel) };
    if !unsafe { ASSERT_OK(err, b"module_attach attach\0".as_ptr() as *const c_char) } {
        goto_cleanup_module_attach_writable(skel);
        return;
    }

    unsafe {
        (*bss).raw_tp_writable_bare_early_ret = true;
        (*bss).raw_tp_writable_bare_out_val = 0xf1f2f3f4u32 as c_int;
        ASSERT_OK(
            trigger_module_test_writable(&mut writable_val),
            b"trigger_writable\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ(
            (*bss).raw_tp_writable_bare_in_val,
            1024,
            b"writable_test_in\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ(
            (*bss).raw_tp_writable_bare_out_val,
            writable_val,
            b"writable_test_out\0".as_ptr() as *const c_char,
        );
    }

    goto_cleanup_module_attach_writable(skel);
}

unsafe fn goto_cleanup_module_attach_writable(skel: *mut test_module_attach) {
    unsafe {
        test_module_attach__destroy(skel);
    }
}

unsafe fn test_module_attach_detach(prog_name: *const c_char) {
    let skel: *mut test_module_attach;
    let prog: *mut bpf_program;
    let link: *mut bpf_link;
    let mut err: c_int;

    skel = unsafe { test_module_attach__open() };
    if !unsafe { ASSERT_OK_PTR(skel as *const c_void, b"module_attach open\0".as_ptr() as *const c_char) } {
        return;
    }

    prog = unsafe { bpf_object__find_program_by_name((*skel).obj, prog_name) };
    if !unsafe {
        ASSERT_OK_PTR(
            prog as *const c_void,
            b"module_attach find_program\0".as_ptr() as *const c_char,
        )
    } {
        goto_cleanup_module_attach_detach(skel);
        return;
    }
    unsafe {
        bpf_program__set_autoload(prog, true);
    }

    err = unsafe { test_module_attach__load(skel) };
    if !unsafe { ASSERT_OK(err, b"module_attach load\0".as_ptr() as *const c_char) } {
        goto_cleanup_module_attach_detach(skel);
        return;
    }

    /* attach and make sure it gets module reference */
    link = unsafe { bpf_program__attach(prog) };
    if !unsafe { ASSERT_OK_PTR(link as *const c_void, b"module_attach attach\0".as_ptr() as *const c_char) } {
        goto_cleanup_module_attach_detach(skel);
        return;
    }

    unsafe {
        ASSERT_ERR(
            try_unload_module(b"bpf_testmod\0".as_ptr() as *const c_char, 1, false),
            b"try_unload_module\0".as_ptr() as *const c_char,
        );
        bpf_link__destroy(link);
    }

    goto_cleanup_module_attach_detach(skel);
}

unsafe fn goto_cleanup_module_attach_detach(skel: *mut test_module_attach) {
    unsafe {
        test_module_attach__destroy(skel);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_module_attach() {
    let mut i: c_int;

    i = 0;
    while (i as usize) < READ_TESTS.len() {
        if !unsafe { test__start_subtest(READ_TESTS[i as usize]) } {
            i += 1;
            continue;
        }
        unsafe {
            test_module_attach_prog(READ_TESTS[i as usize], READ_SZ, null(), 0);
        }
        i += 1;
    }
    if unsafe { test__start_subtest(b"handle_raw_tp_bare\0".as_ptr() as *const c_char) } {
        unsafe {
            test_module_attach_prog(
                b"handle_raw_tp_bare\0".as_ptr() as *const c_char,
                WRITE_SZ,
                null(),
                0,
            );
        }
    }
    if unsafe { test__start_subtest(b"handle_raw_tp_writable_bare\0".as_ptr() as *const c_char) } {
        unsafe {
            test_module_attach_writable();
        }
    }
    if unsafe { test__start_subtest(b"handle_fentry_manual\0".as_ptr() as *const c_char) } {
        unsafe {
            test_module_attach_prog(
                b"handle_fentry_manual\0".as_ptr() as *const c_char,
                READ_SZ,
                b"bpf_testmod_test_read\0".as_ptr() as *const c_char,
                0,
            );
        }
    }
    if unsafe { test__start_subtest(b"handle_fentry_explicit_manual\0".as_ptr() as *const c_char) } {
        unsafe {
            test_module_attach_prog(
                b"handle_fentry_explicit_manual\0".as_ptr() as *const c_char,
                READ_SZ,
                b"bpf_testmod:bpf_testmod_test_read\0".as_ptr() as *const c_char,
                0,
            );
        }
    }
    if unsafe { test__start_subtest(b"handle_fexit\0".as_ptr() as *const c_char) } {
        unsafe {
            test_module_attach_prog(
                b"handle_fexit\0".as_ptr() as *const c_char,
                READ_SZ,
                null(),
                -EIO,
            );
        }
    }
    if unsafe { test__start_subtest(b"handle_fexit_ret\0".as_ptr() as *const c_char) } {
        unsafe {
            test_module_attach_prog(
                b"handle_fexit_ret\0".as_ptr() as *const c_char,
                0,
                null(),
                0,
            );
        }
    }
    i = 0;
    while (i as usize) < DETACH_TESTS.len() {
        let mut test_name: [c_char; 50] = [0; 50];

        unsafe {
            snprintf(
                test_name.as_mut_ptr(),
                size_of::<[c_char; 50]>(),
                b"%s_detach\0".as_ptr() as *const c_char,
                DETACH_TESTS[i as usize],
            );
        }
        if !unsafe { test__start_subtest(test_name.as_ptr()) } {
            i += 1;
            continue;
        }
        unsafe {
            test_module_attach_detach(DETACH_TESTS[i as usize]);
        }
        i += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
