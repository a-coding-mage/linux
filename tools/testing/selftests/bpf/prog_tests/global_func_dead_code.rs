// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies translated as external declarations:
// <test_progs.h>
// "verifier_global_subprogs.skel.h"
// "freplace_dead_global_func.skel.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct verifier_global_subprogs {
    pub progs: verifier_global_subprogs_progs,
}

#[repr(C)]
pub struct verifier_global_subprogs_progs {
    pub chained_global_func_calls_success: *mut bpf_program,
}

#[repr(C)]
pub struct freplace_dead_global_func {
    pub progs: freplace_dead_global_func_progs,
}

#[repr(C)]
pub struct freplace_dead_global_func_progs {
    pub freplace_prog: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn verifier_global_subprogs__open() -> *mut verifier_global_subprogs;
    fn verifier_global_subprogs__load(skel: *mut verifier_global_subprogs) -> c_int;
    fn verifier_global_subprogs__destroy(skel: *mut verifier_global_subprogs);

    fn freplace_dead_global_func__open() -> *mut freplace_dead_global_func;
    fn freplace_dead_global_func__load(skel: *mut freplace_dead_global_func) -> c_int;
    fn freplace_dead_global_func__destroy(skel: *mut freplace_dead_global_func);

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__set_log_buf(prog: *mut bpf_program, log_buf: *mut c_char, log_size: usize);
    fn bpf_program__set_attach_target(
        prog: *mut bpf_program,
        attach_prog_fd: c_int,
        attach_func_name: *const c_char,
    ) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_HAS_SUBSTR(str_: *const c_char, substr: *const c_char, name: *const c_char) -> bool;
}

pub unsafe fn test_global_func_dead_code() {
    let mut tgt_skel: *mut verifier_global_subprogs = core::ptr::null_mut();
    let mut skel: *mut freplace_dead_global_func = core::ptr::null_mut();
    let mut log_buf: [c_char; 4096] = [0; 4096];
    let mut err: c_int;
    let tgt_fd: c_int;

    /* first, try to load target with good global subprog */
    tgt_skel = verifier_global_subprogs__open();
    if !ASSERT_OK_PTR(tgt_skel as *const c_void, c"tgt_skel_good_open".as_ptr()) {
        return;
    }

    bpf_program__set_autoload(
        (*tgt_skel).progs.chained_global_func_calls_success,
        true,
    );

    err = verifier_global_subprogs__load(tgt_skel);
    'out: loop {
        if !ASSERT_OK(err, c"tgt_skel_good_load".as_ptr()) {
            break 'out;
        }

        tgt_fd = bpf_program__fd((*tgt_skel).progs.chained_global_func_calls_success);

        /* Attach to good non-eliminated subprog */
        skel = freplace_dead_global_func__open();
        if !ASSERT_OK_PTR(skel as *const c_void, c"skel_good_open".as_ptr()) {
            break 'out;
        }

        err = bpf_program__set_attach_target(
            (*skel).progs.freplace_prog,
            tgt_fd,
            c"global_good".as_ptr(),
        );
        ASSERT_OK(err, c"attach_target_good".as_ptr());

        err = freplace_dead_global_func__load(skel);
        if !ASSERT_OK(err, c"skel_good_load".as_ptr()) {
            break 'out;
        }

        freplace_dead_global_func__destroy(skel);

        /* Try attaching to dead code-eliminated subprog */
        skel = freplace_dead_global_func__open();
        if !ASSERT_OK_PTR(skel as *const c_void, c"skel_dead_open".as_ptr()) {
            break 'out;
        }

        bpf_program__set_log_buf(
            (*skel).progs.freplace_prog,
            log_buf.as_mut_ptr(),
            core::mem::size_of_val(&log_buf),
        );
        err = bpf_program__set_attach_target(
            (*skel).progs.freplace_prog,
            tgt_fd,
            c"global_dead".as_ptr(),
        );
        ASSERT_OK(err, c"attach_target_dead".as_ptr());

        err = freplace_dead_global_func__load(skel);
        if !ASSERT_ERR(err, c"skel_dead_load".as_ptr()) {
            break 'out;
        }

        ASSERT_HAS_SUBSTR(
            log_buf.as_ptr(),
            c"Subprog global_dead doesn't exist".as_ptr(),
            c"dead_subprog_missing_msg".as_ptr(),
        );

        break 'out;
    }

    verifier_global_subprogs__destroy(tgt_skel);
    freplace_dead_global_func__destroy(skel);
}
