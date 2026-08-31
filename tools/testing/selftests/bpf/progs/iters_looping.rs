// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies: <errno.h>, <string.h>, <linux/bpf.h>,
// <bpf/bpf_helpers.h>, and "bpf_misc.h".

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;

#[repr(C)]
pub struct bpf_iter_num {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_iter_num_new(iter: *mut bpf_iter_num, start: i32, end: i32) -> i32;
    fn bpf_iter_num_next(iter: *mut bpf_iter_num) -> *mut u32;
    fn bpf_iter_num_destroy(iter: *mut bpf_iter_num);
}

pub const BPF_F_TEST_STATE_FREQ: u32 = 1 << 3;

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// ITER_HELPERS:
//   __imm(bpf_iter_num_new),
//   __imm(bpf_iter_num_next),
//   __imm(bpf_iter_num_destroy)

// SEC("?raw_tp")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn force_clang_to_emit_btf_for_externs(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    /* we need this as a workaround to enforce compiler emitting BTF
     * information for bpf_iter_num_{new,next,destroy}() kfuncs,
     * as, apparently, it doesn't emit it for symbols only referenced from
     * assembly (or cleanup attribute, for that matter, as well)
     */
    // bpf_repeat(0);

    0
}

// SEC("?raw_tp")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn consume_first_item_only(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    unsafe {
        asm!(
            /* create iterator */
            "r1 = {iter};",
            "r2 = 0;",
            "r3 = 1000;",
            "call {bpf_iter_num_new};",

            /* consume first item */
            "r1 = {iter};",
            "call {bpf_iter_num_next};",

            "if r0 == 0 goto +1;",
            "r0 = *(u32 *)(r0 + 0);",

            /* destroy iterator */
            "r1 = {iter};",
            "call {bpf_iter_num_destroy};",
            iter = in(reg) iter.as_mut_ptr(),
            bpf_iter_num_new = sym bpf_iter_num_new,
            bpf_iter_num_next = sym bpf_iter_num_next,
            bpf_iter_num_destroy = sym bpf_iter_num_destroy,
        );
    }

    0
}

// SEC("?raw_tp")
// __failure
// __msg("R0 invalid mem access 'scalar'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn missing_null_check_fail(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    unsafe {
        asm!(
            /* create iterator */
            "r1 = {iter};",
            "r2 = 0;",
            "r3 = 1000;",
            "call {bpf_iter_num_new};",

            /* consume first element */
            "r1 = {iter};",
            "call {bpf_iter_num_next};",

            /* FAIL: deref with no NULL check */
            "r1 = *(u32 *)(r0 + 0);",

            /* destroy iterator */
            "r1 = {iter};",
            "call {bpf_iter_num_destroy};",
            iter = in(reg) iter.as_mut_ptr(),
            bpf_iter_num_new = sym bpf_iter_num_new,
            bpf_iter_num_next = sym bpf_iter_num_next,
            bpf_iter_num_destroy = sym bpf_iter_num_destroy,
        );
    }

    0
}

// SEC("?raw_tp")
// __failure
// __msg("invalid access to memory, mem_size=4 off=0 size=8")
// __msg("R0 min value is outside of the allowed memory range")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wrong_sized_read_fail(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    unsafe {
        asm!(
            /* create iterator */
            "r1 = {iter};",
            "r2 = 0;",
            "r3 = 1000;",
            "call {bpf_iter_num_new};",

            /* consume first element */
            "r1 = {iter};",
            "call {bpf_iter_num_next};",

            "if r0 == 0 goto +1;",
            /* FAIL: deref more than available 4 bytes */
            "r0 = *(u64 *)(r0 + 0);",

            /* destroy iterator */
            "r1 = {iter};",
            "call {bpf_iter_num_destroy};",
            iter = in(reg) iter.as_mut_ptr(),
            bpf_iter_num_new = sym bpf_iter_num_new,
            bpf_iter_num_next = sym bpf_iter_num_next,
            bpf_iter_num_destroy = sym bpf_iter_num_destroy,
        );
    }

    0
}

// SEC("?raw_tp")
// __success
// __log_level(2)
// __flag(BPF_F_TEST_STATE_FREQ)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn simplest_loop(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    unsafe {
        asm!(
            "r6 = 0;", /* init sum */

            /* create iterator */
            "r1 = {iter};",
            "r2 = 0;",
            "r3 = 10;",
            "call {bpf_iter_num_new};",

            "1:",
            /* consume next item */
            "r1 = {iter};",
            "call {bpf_iter_num_next};",

            "if r0 == 0 goto 2f;",
            "r0 = *(u32 *)(r0 + 0);",
            "r6 += r0;", /* accumulate sum */
            "goto 1b;",

            "2:",
            /* destroy iterator */
            "r1 = {iter};",
            "call {bpf_iter_num_destroy};",
            iter = in(reg) iter.as_mut_ptr(),
            bpf_iter_num_new = sym bpf_iter_num_new,
            bpf_iter_num_next = sym bpf_iter_num_next,
            bpf_iter_num_destroy = sym bpf_iter_num_destroy,
            out("r6") _,
        );
    }

    0
}

// __used
#[used]
static iterator_with_diff_stack_depth: unsafe extern "C" fn(i32) =
    iterator_with_diff_stack_depth_impl;

unsafe extern "C" fn iterator_with_diff_stack_depth_impl(x: i32) {
    let _ = x;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    unsafe {
        asm!(
            "if r1 == 42 goto 0f;",
            "*(u64 *)(r10 - 128) = 0;",
            "0:",
            /* create iterator */
            "r1 = {iter};",
            "r2 = 0;",
            "r3 = 10;",
            "call {bpf_iter_num_new};",
            "1:",
            /* consume next item */
            "r1 = {iter};",
            "call {bpf_iter_num_next};",
            "if r0 == 0 goto 2f;",
            "goto 1b;",
            "2:",
            /* destroy iterator */
            "r1 = {iter};",
            "call {bpf_iter_num_destroy};",
            iter = in(reg) iter.as_mut_ptr(),
            bpf_iter_num_new = sym bpf_iter_num_new,
            bpf_iter_num_next = sym bpf_iter_num_next,
            bpf_iter_num_destroy = sym bpf_iter_num_destroy,
            out("r6") _,
        );
    }
}

// SEC("socket")
// __success
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn widening_stack_size_bug(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    /*
     * Depending on iterator_with_diff_stack_depth() parameter value,
     * subprogram stack depth is either 8 or 128 bytes. Arrange values so
     * that it is 128 on a first call and 8 on a second. This triggered a
     * bug in verifier's widen_imprecise_scalars() logic.
     */
    unsafe {
        asm!(
            "r6 = 0;",
            "r1 = 0;",
            "1:",
            "call iterator_with_diff_stack_depth;",
            "r1 = 42;",
            "r6 += 1;",
            "if r6 < 2 goto 1b;",
            "r0 = 0;",
            "exit;",
            out("r6") _,
        );
    }
    0
}
