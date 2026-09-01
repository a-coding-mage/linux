// SPDX-License-Identifier: GPL-2.0

// Translated from C implementation source.
// Original includes:
//   <test_progs.h>
//   "bad_struct_ops.skel.h"
//   "bad_struct_ops2.skel.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bad_struct_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bad_struct_ops2__progs {
    pub foo: *mut bpf_program,
}

#[repr(C)]
pub struct bad_struct_ops2 {
    pub progs: bad_struct_ops2__progs,
}

unsafe extern "C" {
    fn bad_struct_ops__open() -> *mut bad_struct_ops;
    fn bad_struct_ops__load(skel: *mut bad_struct_ops) -> c_int;
    fn bad_struct_ops__destroy(skel: *mut bad_struct_ops);

    fn bad_struct_ops2__open() -> *mut bad_struct_ops2;
    fn bad_struct_ops2__load(skel: *mut bad_struct_ops2) -> c_int;
    fn bad_struct_ops2__destroy(skel: *mut bad_struct_ops2);

    fn start_libbpf_log_capture() -> c_int;
    fn stop_libbpf_log_capture() -> *mut c_char;

    fn bpf_program__autoload(prog: *mut bpf_program) -> bool;

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_HAS_SUBSTR(str_: *const c_char, substr: *const c_char, name: *const c_char) -> bool;
    fn ASSERT_TRUE(actual: bool, name: *const c_char) -> bool;

    fn free(ptr: *mut c_void);
}

unsafe fn invalid_prog_reuse() {
    let mut skel: *mut bad_struct_ops;
    let mut log: *mut c_char = core::ptr::null_mut();
    let err: c_int;

    skel = bad_struct_ops__open();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        b"bad_struct_ops__open\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    if start_libbpf_log_capture() != 0 {
        goto_cleanup_invalid_prog_reuse(log, skel);
        return;
    }

    err = bad_struct_ops__load(skel);
    log = stop_libbpf_log_capture();
    ASSERT_ERR(
        err,
        b"bad_struct_ops__load should fail\0".as_ptr() as *const c_char,
    );
    ASSERT_HAS_SUBSTR(
        log,
        b"struct_ops init_kern testmod_2 func ptr test_1: invalid reuse of prog test_1\0".as_ptr()
            as *const c_char,
        b"expected init_kern message\0".as_ptr() as *const c_char,
    );

    goto_cleanup_invalid_prog_reuse(log, skel);
}

unsafe fn goto_cleanup_invalid_prog_reuse(log: *mut c_char, skel: *mut bad_struct_ops) {
    free(log as *mut c_void);
    bad_struct_ops__destroy(skel);
}

unsafe fn unused_program() {
    let mut skel: *mut bad_struct_ops2;
    let mut log: *mut c_char = core::ptr::null_mut();
    let err: c_int;

    skel = bad_struct_ops2__open();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        b"bad_struct_ops2__open\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    /* struct_ops programs not referenced from any maps are open
     * with autoload set to true.
     */
    ASSERT_TRUE(
        bpf_program__autoload((*skel).progs.foo),
        b"foo autoload == true\0".as_ptr() as *const c_char,
    );

    if start_libbpf_log_capture() != 0 {
        goto_cleanup_unused_program(log, skel);
        return;
    }

    err = bad_struct_ops2__load(skel);
    ASSERT_ERR(
        err,
        b"bad_struct_ops2__load should fail\0".as_ptr() as *const c_char,
    );
    log = stop_libbpf_log_capture();
    ASSERT_HAS_SUBSTR(
        log,
        b"prog 'foo': failed to load\0".as_ptr() as *const c_char,
        b"message about 'foo' failing to load\0".as_ptr() as *const c_char,
    );

    goto_cleanup_unused_program(log, skel);
}

unsafe fn goto_cleanup_unused_program(log: *mut c_char, skel: *mut bad_struct_ops2) {
    free(log as *mut c_void);
    bad_struct_ops2__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_bad_struct_ops() {
    if test__start_subtest(b"invalid_prog_reuse\0".as_ptr() as *const c_char) {
        invalid_prog_reuse();
    }
    if test__start_subtest(b"unused_program\0".as_ptr() as *const c_char) {
        unused_program();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
