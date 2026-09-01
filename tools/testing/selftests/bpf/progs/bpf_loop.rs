// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Dependencies from the original C includes:
 * "vmlinux.h", <bpf/bpf_helpers.h>, and "bpf_misc.h".
 */

use core::ffi::c_void;

type __u32 = u32;
type u32 = core::primitive::u32;

const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_ANY: u64 = 0;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
struct callback_ctx {
    output: i32,
}

#[repr(C)]
struct map1_def {
    /* Original BPF map declaration:
     * __uint(type, BPF_MAP_TYPE_HASH);
     * __uint(max_entries, 32);
     * __type(key, int);
     * __type(value, int);
     */
    type_: u32,
    max_entries: u32,
}

#[no_mangle]
#[link_section = ".maps"]
static mut map1: map1_def = map1_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 32,
};

/* These should be set by the user program */
#[no_mangle]
static mut nested_callback_nr_loops: u32 = 0;
#[no_mangle]
static mut stop_index: u32 = -1i32 as u32;
#[no_mangle]
static mut nr_loops: u32 = 0;
#[no_mangle]
static mut pid: i32 = 0;
#[no_mangle]
static mut callback_selector: i32 = 0;

/* Making these global variables so that the userspace program
 * can verify the output through the skeleton
 */
#[no_mangle]
static mut nr_loops_returned: i32 = 0;
#[no_mangle]
static mut g_output: i32 = 0;
#[no_mangle]
static mut err: i32 = 0;

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_loop(
        nr_loops: u32,
        callback: Option<unsafe extern "C" fn(__u32, *mut c_void) -> i32>,
        data: *mut c_void,
        flags: u64,
    ) -> i32;
    fn bpf_map_lookup_elem(map: *mut map1_def, key: *const c_void) -> *mut c_void;
    fn bpf_map_update_elem(map: *mut map1_def, key: *const c_void, value: *const c_void, flags: u64) -> i32;
}

unsafe extern "C" fn callback(index: __u32, data: *mut c_void) -> i32 {
    let ctx: *mut callback_ctx = data as *mut callback_ctx;

    if index >= stop_index {
        return 1;
    }

    (*ctx).output += index as i32;

    0
}

unsafe extern "C" fn empty_callback(_index: __u32, _data: *mut c_void) -> i32 {
    0
}

unsafe extern "C" fn nested_callback2(_index: __u32, data: *mut c_void) -> i32 {
    nr_loops_returned += bpf_loop(nested_callback_nr_loops, Some(callback), data, 0);

    0
}

unsafe extern "C" fn nested_callback1(_index: __u32, data: *mut c_void) -> i32 {
    bpf_loop(nested_callback_nr_loops, Some(nested_callback2), data, 0);
    0
}

/* Original section: SEC("fentry/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn test_prog(_ctx: *mut c_void) -> i32 {
    let mut data: callback_ctx = callback_ctx { output: 0 };

    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return 0;
    }

    nr_loops_returned = bpf_loop(nr_loops, Some(callback), &mut data as *mut _ as *mut c_void, 0);

    if nr_loops_returned < 0 {
        err = nr_loops_returned;
    } else {
        g_output = data.output;
    }

    0
}

/* Original section: SEC("fentry/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn prog_null_ctx(_ctx: *mut c_void) -> i32 {
    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return 0;
    }

    nr_loops_returned = bpf_loop(nr_loops, Some(empty_callback), core::ptr::null_mut(), 0);

    0
}

/* Original section: SEC("fentry/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn prog_invalid_flags(_ctx: *mut c_void) -> i32 {
    let mut data: callback_ctx = callback_ctx { output: 0 };

    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return 0;
    }

    err = bpf_loop(nr_loops, Some(callback), &mut data as *mut _ as *mut c_void, 1);

    0
}

/* Original section: SEC("fentry/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn prog_nested_calls(_ctx: *mut c_void) -> i32 {
    let mut data: callback_ctx = callback_ctx { output: 0 };

    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return 0;
    }

    nr_loops_returned = 0;
    bpf_loop(nr_loops, Some(nested_callback1), &mut data as *mut _ as *mut c_void, 0);

    g_output = data.output;

    0
}

unsafe extern "C" fn callback_set_f0(_i: i32, _ctx: *mut c_void) -> i32 {
    g_output = 0xF0;
    0
}

unsafe extern "C" fn callback_set_0f(_i: i32, _ctx: *mut c_void) -> i32 {
    g_output = 0x0F;
    0
}

/*
 * non-constant callback is a corner case for bpf_loop inline logic
 */
/* Original section: SEC("fentry/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn prog_non_constant_callback(_ctx: *mut c_void) -> i32 {
    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return 0;
    }

    let callback: unsafe extern "C" fn(i32, *mut c_void) -> i32;

    g_output = 0;

    if callback_selector == 0x0F {
        callback = callback_set_0f;
    } else {
        callback = callback_set_f0;
    }

    bpf_loop(
        1,
        Some(core::mem::transmute::<
            unsafe extern "C" fn(i32, *mut c_void) -> i32,
            unsafe extern "C" fn(__u32, *mut c_void) -> i32,
        >(callback)),
        core::ptr::null_mut(),
        0,
    );

    0
}

unsafe extern "C" fn stack_check_inner_callback(_ctx: *mut c_void) -> i32 {
    0
}

unsafe fn map1_lookup_elem(key: i32) -> i32 {
    let val: *mut i32 = bpf_map_lookup_elem(
        &mut map1 as *mut map1_def,
        &key as *const i32 as *const c_void,
    ) as *mut i32;

    if !val.is_null() {
        *val
    } else {
        -1
    }
}

unsafe fn map1_update_elem(key: i32, val: i32) {
    bpf_map_update_elem(
        &mut map1 as *mut map1_def,
        &key as *const i32 as *const c_void,
        &val as *const i32 as *const c_void,
        BPF_ANY,
    );
}

unsafe extern "C" fn stack_check_outer_callback(_ctx: *mut c_void) -> i32 {
    let a: i32 = map1_lookup_elem(1);
    let b: i32 = map1_lookup_elem(2);
    let c: i32 = map1_lookup_elem(3);
    let d: i32 = map1_lookup_elem(4);
    let e: i32 = map1_lookup_elem(5);
    let f: i32 = map1_lookup_elem(6);

    bpf_loop(1, Some(stack_check_inner_callback), core::ptr::null_mut(), 0);

    map1_update_elem(1, a + 1);
    map1_update_elem(2, b + 1);
    map1_update_elem(3, c + 1);
    map1_update_elem(4, d + 1);
    map1_update_elem(5, e + 1);
    map1_update_elem(6, f + 1);

    0
}

/* Some of the local variables in stack_check and
 * stack_check_outer_callback would be allocated on stack by
 * compiler. This test should verify that stack content for these
 * variables is preserved between calls to bpf_loop (might be an issue
 * if loop inlining allocates stack slots incorrectly).
 */
/* Original section: SEC("fentry/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn stack_check(_ctx: *mut c_void) -> i32 {
    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return 0;
    }

    let a: i32 = map1_lookup_elem(7);
    let b: i32 = map1_lookup_elem(8);
    let c: i32 = map1_lookup_elem(9);
    let d: i32 = map1_lookup_elem(10);
    let e: i32 = map1_lookup_elem(11);
    let f: i32 = map1_lookup_elem(12);

    bpf_loop(1, Some(stack_check_outer_callback), core::ptr::null_mut(), 0);

    map1_update_elem(7, a + 1);
    map1_update_elem(8, b + 1);
    map1_update_elem(9, c + 1);
    map1_update_elem(10, d + 1);
    map1_update_elem(11, e + 1);
    map1_update_elem(12, f + 1);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
