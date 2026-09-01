// SPDX-License-Identifier: GPL-2.0
//
// Translated from C. Original includes:
// <vmlinux.h>
// <bpf/bpf_tracing.h>
// <bpf/bpf_helpers.h>
// "../test_kmods/bpf_testmod_kfunc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

#[repr(C)]
pub struct prog_test_ref_kfunc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map_value {
    pub ptr: *mut prog_test_ref_kfunc,
}

type c_int = i32;
type c_ulong = u64;

const BPF_MAP_TYPE_ARRAY: u32 = 2;

#[repr(C)]
pub struct array_map_def {
    pub type_: u32,
    pub max_entries: u32,
}

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __type(key, int);
//     __type(value, struct map_value);
//     __uint(max_entries, 16);
// } array_map SEC(".maps");
#[no_mangle]
#[link_section = ".maps"]
pub static mut array_map: array_map_def = array_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 16,
};

extern "C" {
    fn bpf_kfunc_call_test_release(p: *mut c_void);
    fn bpf_kfunc_call_test_acquire(sl: *mut c_ulong) -> *mut prog_test_ref_kfunc;
    fn bpf_for_each_map_elem(
        map: *mut c_void,
        callback: unsafe extern "C" fn(
            map: *mut c_void,
            key: *mut c_void,
            value: *mut c_void,
            ctx: *mut c_void,
        ) -> c_int,
        ctx: *mut c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_kptr_xchg(dst: *mut *mut prog_test_ref_kfunc, src: *mut prog_test_ref_kfunc)
        -> *mut prog_test_ref_kfunc;
}

#[inline(never)]
unsafe extern "C" fn cb1(
    _map: *mut c_void,
    _key: *mut c_void,
    _value: *mut c_void,
    ctx: *mut c_void,
) -> c_int {
    let p = *(ctx as *mut *mut c_void);
    bpf_kfunc_call_test_release(p);
    /* Without the fix this would cause underflow */
    0
}

#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn underflow_prog(_ctx: *mut c_void) -> c_int {
    let mut sl: c_ulong = 0;

    let p = bpf_kfunc_call_test_acquire(&mut sl);
    if p.is_null() {
        return 0;
    }
    let mut p = p;
    bpf_for_each_map_elem(
        &mut array_map as *mut _ as *mut c_void,
        cb1,
        &mut p as *mut _ as *mut c_void,
        0,
    );
    bpf_kfunc_call_test_release(p as *mut c_void);
    0
}

#[inline(always)]
unsafe extern "C" fn cb2(
    _map: *mut c_void,
    _key: *mut c_void,
    _value: *mut c_void,
    ctx: *mut c_void,
) -> c_int {
    let mut sl: c_ulong = 0;

    *(ctx as *mut *mut c_void) = bpf_kfunc_call_test_acquire(&mut sl) as *mut c_void;
    /* Without the fix this would leak memory */
    0
}

#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn leak_prog(_ctx: *mut c_void) -> c_int {
    let key: c_int = 0;
    let v = bpf_map_lookup_elem(
        &mut array_map as *mut _ as *mut c_void,
        &key as *const _ as *const c_void,
    ) as *mut map_value;
    if v.is_null() {
        return 0;
    }

    let mut p: *mut prog_test_ref_kfunc = core::ptr::null_mut();
    bpf_for_each_map_elem(
        &mut array_map as *mut _ as *mut c_void,
        cb2,
        &mut p as *mut _ as *mut c_void,
        0,
    );
    p = bpf_kptr_xchg(&mut (*v).ptr, p);
    if !p.is_null() {
        bpf_kfunc_call_test_release(p as *mut c_void);
    }
    0
}

#[inline(always)]
unsafe extern "C" fn cb(
    _map: *mut c_void,
    _key: *mut c_void,
    _value: *mut c_void,
    _ctx: *mut c_void,
) -> c_int {
    0
}

#[inline(always)]
unsafe extern "C" fn cb3(
    _map: *mut c_void,
    _key: *mut c_void,
    _value: *mut c_void,
    _ctx: *mut c_void,
) -> c_int {
    let mut sl: c_ulong = 0;
    let mut p: *mut c_void;

    bpf_kfunc_call_test_acquire(&mut sl);
    bpf_for_each_map_elem(
        &mut array_map as *mut _ as *mut c_void,
        cb,
        &mut p as *mut _ as *mut c_void,
        0,
    );
    /* It should only complain here, not in cb. This is why we need
     * callback_ref to be set to frameno.
     */
    0
}

#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn nested_cb(_ctx: *mut c_void) -> c_int {
    let mut sl: c_ulong = 0;
    let mut sp: c_int = 0;

    let p = bpf_kfunc_call_test_acquire(&mut sl);
    if p.is_null() {
        return 0;
    }
    bpf_for_each_map_elem(
        &mut array_map as *mut _ as *mut c_void,
        cb3,
        &mut sp as *mut _ as *mut c_void,
        0,
    );
    bpf_kfunc_call_test_release(p as *mut c_void);
    0
}

#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn non_cb_transfer_ref(_ctx: *mut c_void) -> c_int {
    let mut sl: c_ulong = 0;

    let mut p = bpf_kfunc_call_test_acquire(&mut sl);
    if p.is_null() {
        return 0;
    }
    cb1(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        &mut p as *mut _ as *mut c_void,
    );
    bpf_kfunc_call_test_acquire(&mut sl);
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
