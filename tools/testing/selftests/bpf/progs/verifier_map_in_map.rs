// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/map_in_map.c */

// C dependencies translated by reference:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;

extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_ringbuf_reserve_dynptr(
        ringbuf: *mut core::ffi::c_void,
        size: __u32,
        flags: __u64,
        ptr: *mut bpf_dynptr,
    ) -> core::ffi::c_long;
    fn bpf_ringbuf_submit_dynptr(ptr: *mut bpf_dynptr, flags: __u64);
}

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_ARRAY_OF_MAPS: u32 = 12;
const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_RINGBUF: u32 = 27;
const BPF_F_INNER_MAP: u32 = 0x1000;
const BPF_F_TEST_STATE_FREQ: u32 = 1 << 3;

#[repr(C)]
pub struct bpf_dynptr {
    _opaque: [u64; 2],
}

#[repr(C)]
struct map_in_map_values {
    type_: u32,
    max_entries: u32,
    key: i32,
    value: i32,
}

#[repr(C)]
struct map_in_map_def {
    type_: u32,
    max_entries: u32,
    key: i32,
    value: i32,
    values: map_in_map_values,
}

// SEC(".maps")
#[no_mangle]
static mut map_in_map: map_in_map_def = map_in_map_def {
    type_: BPF_MAP_TYPE_ARRAY_OF_MAPS,
    max_entries: 1,
    key: 0,
    value: 0,
    values: map_in_map_values {
        type_: BPF_MAP_TYPE_ARRAY,
        max_entries: 1,
        key: 0,
        value: 0,
    },
};

#[repr(C)]
struct map_in_map_dyn_values {
    type_: u32,
    map_flags: u32,
    max_entries: u32,
    key: i32,
    value: i64,
}

#[repr(C)]
struct map_in_map_dyn_def {
    type_: u32,
    max_entries: u32,
    key: i32,
    value: i32,
    values: map_in_map_dyn_values,
}

// SEC(".maps")
#[no_mangle]
static mut map_in_map_dyn: map_in_map_dyn_def = map_in_map_dyn_def {
    type_: BPF_MAP_TYPE_ARRAY_OF_MAPS,
    max_entries: 1,
    key: 0,
    value: 0,
    values: map_in_map_dyn_values {
        type_: BPF_MAP_TYPE_ARRAY,
        map_flags: BPF_F_INNER_MAP,
        max_entries: 8,
        key: 0,
        value: 0,
    },
};

// SEC("socket")
// __description("map in map access")
// __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn map_in_map_access() {
    asm!(
        "r1 = 0",
        "*(u32*)(r10 - 4) = r1",
        "r2 = r10",
        "r2 += -4",
        "r1 = {map_in_map} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r1 = 0",
        "*(u32*)(r10 - 4) = r1",
        "r2 = r10",
        "r2 += -4",
        "r1 = r0",
        "call {bpf_map_lookup_elem}",
        "0:",
        "r0 = 0",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_in_map = sym map_in_map,
        options(noreturn)
    );
}

// SEC("socket")
// __description("map in map dynamic inner array lookup is nullable")
// __failure __msg("invalid mem access 'map_value_or_null'")
#[no_mangle]
pub unsafe extern "C" fn map_in_map_dynamic_inner_array_lookup_is_nullable() {
    asm!(
        "r1 = 0",
        "*(u32*)(r10 - 4) = r1",
        "r2 = r10",
        "r2 += -4",
        "r1 = {map_in_map_dyn} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "*(u32*)(r10 - 8) = 4",
        "r2 = r10",
        "r2 += -8",
        "r1 = r0",
        "call {bpf_map_lookup_elem}",
        "r0 = *(u64 *)(r0 + 0)",
        "0:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_in_map_dyn = sym map_in_map_dyn,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("map in map state pruning")
// __success __msg("processed 15 insns")
// __log_level(2) __retval(0) __flag(BPF_F_TEST_STATE_FREQ)
#[no_mangle]
pub unsafe extern "C" fn map_in_map_state_pruning() {
    asm!(
        "r1 = 0",
        "*(u32*)(r10 - 4) = r1",
        "r6 = r10",
        "r6 += -4",
        "r2 = r6",
        "r1 = {map_in_map} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 != 0 goto 0f",
        "exit",
        "0:",
        "r2 = r6",
        "r1 = r0",
        "call {bpf_map_lookup_elem}",
        "if r0 != 0 goto 2f",
        "r2 = r6",
        "r1 = {map_in_map} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 != 0 goto 1f",
        "exit",
        "1:",
        "r2 = r6",
        "r1 = r0",
        "call {bpf_map_lookup_elem}",
        "if r0 != 0 goto 2f",
        "exit",
        "2:",
        "r0 = *(u32*)(r0 + 0)",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_in_map = sym map_in_map,
        options(noreturn)
    );
}

// SEC("socket")
// __description("invalid inner map pointer")
// __failure __msg("R1 pointer arithmetic on map_ptr prohibited")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn invalid_inner_map_pointer() {
    asm!(
        "r1 = 0",
        "*(u32*)(r10 - 4) = r1",
        "r2 = r10",
        "r2 += -4",
        "r1 = {map_in_map} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r1 = 0",
        "*(u32*)(r10 - 4) = r1",
        "r2 = r10",
        "r2 += -4",
        "r1 = r0",
        "r1 += 8",
        "call {bpf_map_lookup_elem}",
        "0:",
        "r0 = 0",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_in_map = sym map_in_map,
        options(noreturn)
    );
}

// SEC("socket")
// __description("forgot null checking on the inner map pointer")
// __failure __msg("R1 type=map_ptr_or_null expected=map_ptr")
// __msg("map_ptr_or_null, but this argument accepts map_ptr")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn on_the_inner_map_pointer() {
    asm!(
        "r1 = 0",
        "*(u32*)(r10 - 4) = r1",
        "r2 = r10",
        "r2 += -4",
        "r1 = {map_in_map} ll",
        "call {bpf_map_lookup_elem}",
        "r1 = 0",
        "*(u32*)(r10 - 4) = r1",
        "r2 = r10",
        "r2 += -4",
        "r1 = r0",
        "call {bpf_map_lookup_elem}",
        "r0 = 0",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_in_map = sym map_in_map,
        options(noreturn)
    );
}

// SEC("socket")
// __description("map_ptr is never null")
// __success
#[no_mangle]
pub unsafe extern "C" fn map_ptr_is_never_null() {
    asm!(
        "r0 = 0",
        "r1 = {map_in_map} ll",
        "if r1 != 0 goto 0f",
        "r10 = 42",
        "0:",
        "exit",
        map_in_map = sym map_in_map,
        options(noreturn)
    );
}

// SEC("socket")
// __description("map_ptr is never null inner")
// __success
#[no_mangle]
pub unsafe extern "C" fn map_ptr_is_never_null_inner() {
    asm!(
        "r1 = 0",
        "*(u32*)(r10 - 4) = r1",
        "r2 = r10",
        "r2 += -4",
        "r1 = {map_in_map} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "if r0 != 0 goto 0f",
        "r10 = 42",
        "0:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_in_map = sym map_in_map,
        options(noreturn)
    );
}

// SEC("socket")
// __description("map_ptr is never null inner spill fill")
// __success
#[no_mangle]
pub unsafe extern "C" fn map_ptr_is_never_null_inner_spill_fill() {
    asm!(
        "r1 = 0",
        "*(u32*)(r10 - 4) = r1",
        "r2 = r10",
        "r2 += -4",
        "r1 = {map_in_map} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 != 0 goto 0f",
        "exit",
        "0:",
        "*(u64 *)(r10 -16) = r0",
        "r1 = *(u64 *)(r10 -16)",
        "if r1 == 0 goto 1f",
        "exit",
        "1:",
        "r10 = 42",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_in_map = sym map_in_map,
        options(noreturn)
    );
}

#[repr(C)]
struct rb_in_map_values {
    type_: u32,
    max_entries: u32,
}

#[repr(C)]
struct rb_in_map_def {
    type_: u32,
    max_entries: u32,
    key: i32,
    value: i32,
    values: rb_in_map_values,
}

// SEC(".maps")
#[no_mangle]
static mut rb_in_map: rb_in_map_def = rb_in_map_def {
    type_: BPF_MAP_TYPE_ARRAY_OF_MAPS,
    max_entries: 1,
    key: 0,
    value: 0,
    values: rb_in_map_values {
        type_: BPF_MAP_TYPE_RINGBUF,
        max_entries: 64 * 1024,
    },
};

#[repr(C)]
struct rb_ctx {
    rb: *mut core::ffi::c_void,
    dptr: bpf_dynptr,
}

#[inline(always)]
unsafe fn __rb_event_reserve(sz: __u32) -> rb_ctx {
    let mut rb_ctx = rb_ctx {
        rb: core::ptr::null_mut(),
        dptr: core::mem::zeroed(),
    };
    let rb: *mut core::ffi::c_void;
    let cpu: __u32 = bpf_get_smp_processor_id();
    let rb_slot: __u32 = cpu & 1;

    rb = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(rb_in_map).cast::<core::ffi::c_void>(),
        core::ptr::addr_of!(rb_slot).cast::<core::ffi::c_void>(),
    );
    if rb.is_null() {
        return rb_ctx;
    }

    rb_ctx.rb = rb;
    bpf_ringbuf_reserve_dynptr(rb, sz, 0, core::ptr::addr_of_mut!(rb_ctx.dptr));

    rb_ctx
}

#[inline(never)]
unsafe fn __rb_event_submit(ctx: *mut rb_ctx) {
    if (*ctx).rb.is_null() {
        return;
    }

    /* If the verifier (incorrectly) concludes that ctx->rb can be
     * NULL at this point, we'll get "BPF_EXIT instruction in main
     * prog would lead to reference leak" error
     */
    bpf_ringbuf_submit_dynptr(core::ptr::addr_of_mut!((*ctx).dptr), 0);
}

// SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn map_ptr_is_never_null_rb(ctx: *mut core::ffi::c_void) -> i32 {
    let mut event_ctx: rb_ctx = __rb_event_reserve(256);
    __rb_event_submit(core::ptr::addr_of_mut!(event_ctx));
    0
}

// SEC("license")
#[no_mangle]
static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
