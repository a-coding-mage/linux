// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external dependencies:
// <vmlinux.h>, <bpf/bpf_tracing.h>, "bpf_misc.h",
// "../test_kmods/bpf_testmod.h", "../test_kmods/bpf_testmod_kfunc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

#[repr(C)]
pub struct bpf_timer {
    _data: [u8; 0],
}

#[repr(C)]
pub struct st_ops_args {
    _data: [u8; 0],
}

#[repr(C)]
pub struct bpf_testmod_multi_st_ops {
    pub test_1: *mut c_void,
}

#[repr(C)]
pub struct elem {
    pub timer: bpf_timer,
}

#[repr(C)]
pub struct array_map_def {
    // Original C map definition:
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __uint(max_entries, 1);
    // __type(key, int);
    // __type(value, struct elem);
    _data: [u8; 0],
}

pub const MAP_MAGIC: i32 = 1234;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = ".maps"]
pub static mut array_map: array_map_def = array_map_def { _data: [] };

#[no_mangle]
pub static mut recur: i32 = 0;

#[no_mangle]
pub static mut test_err: i32 = 0;

#[no_mangle]
pub static mut timer_ns: i32 = 0;

#[no_mangle]
pub static mut timer_test_1_ret: i32 = 0;

#[no_mangle]
pub static mut timer_cb_run: i32 = 0;

extern "C" {
    fn bpf_kfunc_multi_st_ops_test_1_assoc(args: *mut st_ops_args) -> i32;
    fn bpf_map_lookup_elem(map: *mut array_map_def, key: *const i32) -> *mut bpf_timer;
    fn bpf_timer_init(timer: *mut bpf_timer, map: *mut array_map_def, flags: i32) -> i32;
    fn bpf_timer_set_callback(
        timer: *mut bpf_timer,
        callback: unsafe extern "C" fn(*mut c_void, *mut i32, *mut bpf_timer) -> i32,
    ) -> i32;
    fn bpf_timer_start(timer: *mut bpf_timer, nsecs: i32, flags: i32) -> i32;
}

#[inline(never)]
unsafe extern "C" fn timer_cb(
    _map: *mut c_void,
    _key: *mut i32,
    _timer: *mut bpf_timer,
) -> i32 {
    let mut args: st_ops_args = core::mem::zeroed();

    recur += 1;
    timer_test_1_ret = bpf_kfunc_multi_st_ops_test_1_assoc(&mut args);
    recur -= 1;

    timer_cb_run += 1;

    0
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn test_1(args: *mut st_ops_args) -> i32 {
    let timer: *mut bpf_timer;
    let key: i32 = 0;

    let _ = args;

    if recur == 0 {
        timer = bpf_map_lookup_elem(&mut array_map, &key);
        if timer.is_null() {
            return 0;
        }

        bpf_timer_init(timer, &mut array_map, 1);
        bpf_timer_set_callback(timer, timer_cb);
        bpf_timer_start(timer, timer_ns, 0);
    }

    MAP_MAGIC
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn syscall_prog(ctx: *mut c_void) -> i32 {
    let mut args: st_ops_args = core::mem::zeroed();
    let ret: i32;

    let _ = ctx;

    ret = bpf_kfunc_multi_st_ops_test_1_assoc(&mut args);
    if ret != MAP_MAGIC {
        test_err += 1;
    }

    0
}

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static mut st_ops_map: bpf_testmod_multi_st_ops = bpf_testmod_multi_st_ops {
    test_1: test_1 as *mut c_void,
};
