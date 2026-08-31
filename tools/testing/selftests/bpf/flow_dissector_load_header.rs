/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */

// C header dependencies:
// #include <bpf/bpf.h>
// #include <bpf/libbpf.h>
// #include "testing_helpers.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static BPF_PROG_TYPE_FLOW_DISSECTOR: c_uint;
    pub static BPF_ANY: c_uint;

    pub fn bpf_prog_test_load(
        path: *const c_char,
        prog_type: c_uint,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;

    pub fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;

    pub fn bpf_program__fd(prog: *const bpf_program) -> c_int;

    pub fn bpf_object__find_map_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_map;

    pub fn bpf_map__fd(map: *const bpf_map) -> c_int;

    pub fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: c_uint,
    ) -> c_int;

    pub fn bpf_object__next_program(
        obj: *const bpf_object,
        prog: *mut bpf_program,
    ) -> *mut bpf_program;
}

pub unsafe fn bpf_flow_load(
    obj: *mut *mut bpf_object,
    path: *const c_char,
    prog_name: *const c_char,
    map_name: *const c_char,
    keys_map_name: *const c_char,
    prog_fd: *mut c_int,
    keys_fd: *mut c_int,
) -> c_int {
    let mut prog: *mut bpf_program;
    let main_prog: *mut bpf_program;
    let prog_array: *mut bpf_map;
    let keys: *mut bpf_map;
    let prog_array_fd: c_int;
    let ret: c_int;
    let mut fd: c_int;
    let mut i: c_int;

    ret = unsafe { bpf_prog_test_load(path, BPF_PROG_TYPE_FLOW_DISSECTOR, obj, prog_fd) };
    if ret != 0 {
        return ret;
    }

    main_prog = unsafe { bpf_object__find_program_by_name(*obj, prog_name) };
    if main_prog.is_null() {
        return -1;
    }

    unsafe {
        *prog_fd = bpf_program__fd(main_prog);
    }
    if unsafe { *prog_fd } < 0 {
        return -1;
    }

    prog_array = unsafe { bpf_object__find_map_by_name(*obj, map_name) };
    if prog_array.is_null() {
        return -1;
    }

    prog_array_fd = unsafe { bpf_map__fd(prog_array) };
    if prog_array_fd < 0 {
        return -1;
    }

    if !keys_map_name.is_null() && !keys_fd.is_null() {
        keys = unsafe { bpf_object__find_map_by_name(*obj, keys_map_name) };
        if keys.is_null() {
            return -1;
        }

        unsafe {
            *keys_fd = bpf_map__fd(keys);
        }
        if unsafe { *keys_fd } < 0 {
            return -1;
        }
    }

    i = 0;
    prog = unsafe { bpf_object__next_program(*obj, core::ptr::null_mut()) };
    while !prog.is_null() {
        fd = unsafe { bpf_program__fd(prog) };
        if fd < 0 {
            return fd;
        }

        if fd != unsafe { *prog_fd } {
            unsafe {
                bpf_map_update_elem(
                    prog_array_fd,
                    (&i as *const c_int).cast::<c_void>(),
                    (&fd as *const c_int).cast::<c_void>(),
                    BPF_ANY,
                );
            }
            i += 1;
        }

        prog = unsafe { bpf_object__next_program(*obj, prog) };
    }

    0
}
