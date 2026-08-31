// SPDX-License-Identifier: GPL-2.0
// C dependency intent:
// #include <test_progs.h>
// #include <testing_helpers.h>
// #include "kfunc_module_order.skel.h"

use core::ffi::{c_char, c_int, c_void};

const EINVAL: c_int = 22;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *mut c_void,
    pub data_size_in: usize,
    pub retval: u32,
}

#[repr(C)]
pub struct kfunc_module_order_progs {
    pub call_kfunc_xy: *const bpf_program,
    pub call_kfunc_yx: *const bpf_program,
}

#[repr(C)]
pub struct kfunc_module_order {
    pub progs: kfunc_module_order_progs,
}

unsafe extern "C" {
    static env_verbosity: c_int;
    static VERBOSE_NONE: c_int;

    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *const bpf_program) -> c_int;
    fn bpf_program__name(prog: *const bpf_program) -> *const c_char;

    fn load_module(module: *const c_char, verbose: bool) -> c_int;
    fn unload_module(module: *const c_char, verbose: bool);

    fn kfunc_module_order__open_and_load() -> *mut kfunc_module_order;
    fn kfunc_module_order__destroy(skel: *mut kfunc_module_order);

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

unsafe fn test_run_prog(prog: *const bpf_program, opts: *mut bpf_test_run_opts) -> c_int {
    let mut err: c_int;

    err = bpf_prog_test_run_opts(bpf_program__fd(prog), opts);
    if !ASSERT_OK(err, b"bpf_prog_test_run_opts\0".as_ptr() as *const c_char) {
        return err;
    }

    if !ASSERT_EQ((*opts).retval as c_int, 0, bpf_program__name(prog)) {
        return -EINVAL;
    }

    0
}

pub unsafe fn test_kfunc_module_order() {
    let mut skel: *mut kfunc_module_order;
    let mut pkt_data = [0u8; 64];
    let mut err: c_int = 0;

    let mut test_opts = bpf_test_run_opts {
        data_in: pkt_data.as_mut_ptr() as *mut c_void,
        data_size_in: core::mem::size_of_val(&pkt_data),
        retval: 0,
    };

    err = load_module(
        b"bpf_test_modorder_x.ko\0".as_ptr() as *const c_char,
        env_verbosity > VERBOSE_NONE,
    );
    if !ASSERT_OK(err, b"load bpf_test_modorder_x.ko\0".as_ptr() as *const c_char) {
        return;
    }

    err = load_module(
        b"bpf_test_modorder_y.ko\0".as_ptr() as *const c_char,
        env_verbosity > VERBOSE_NONE,
    );
    if !ASSERT_OK(err, b"load bpf_test_modorder_y.ko\0".as_ptr() as *const c_char) {
        unload_module(
            b"bpf_test_modorder_x\0".as_ptr() as *const c_char,
            env_verbosity > VERBOSE_NONE,
        );
        return;
    }

    skel = kfunc_module_order__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        b"kfunc_module_order__open_and_load()\0".as_ptr() as *const c_char,
    ) {
        err = -EINVAL;
        unload_module(
            b"bpf_test_modorder_y\0".as_ptr() as *const c_char,
            env_verbosity > VERBOSE_NONE,
        );
        unload_module(
            b"bpf_test_modorder_x\0".as_ptr() as *const c_char,
            env_verbosity > VERBOSE_NONE,
        );
        return;
    }

    test_run_prog((*skel).progs.call_kfunc_xy, &mut test_opts);
    test_run_prog((*skel).progs.call_kfunc_yx, &mut test_opts);

    kfunc_module_order__destroy(skel);

    unload_module(
        b"bpf_test_modorder_y\0".as_ptr() as *const c_char,
        env_verbosity > VERBOSE_NONE,
    );
    unload_module(
        b"bpf_test_modorder_x\0".as_ptr() as *const c_char,
        env_verbosity > VERBOSE_NONE,
    );
}
