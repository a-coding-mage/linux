// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Facebook */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;

/* Dependencies from errno.h, string.h, linux/bpf.h, bpf/bpf_helpers.h, and bpf_misc.h. */

const BPF_F_TEST_STATE_FREQ: u64 = 1 << 0;

#[repr(C)]
pub struct bpf_iter_num {
    _private: [u64; 3],
}

unsafe extern "C" {
    fn bpf_iter_num_new(iter: *mut bpf_iter_num, start: i32, end: i32) -> i32;
    fn bpf_iter_num_next(iter: *mut bpf_iter_num) -> *mut i32;
    fn bpf_iter_num_destroy(iter: *mut bpf_iter_num);
    fn bpf_probe_read_kernel(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i64;
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_dynptr_from_mem();
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

/* ITER_HELPERS:
 *   __imm(bpf_iter_num_new),
 *   __imm(bpf_iter_num_next),
 *   __imm(bpf_iter_num_destroy)
 */

/* SEC("?raw_tp")
 * __success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn force_clang_to_emit_btf_for_externs(ctx: *mut c_void) -> i32 {
    /* we need this as a workaround to enforce compiler emitting BTF
     * information for bpf_iter_num_{new,next,destroy}() kfuncs,
     * as, apparently, it doesn't emit it for symbols only referenced from
     * assembly (or cleanup attribute, for that matter, as well)
     */
    let _ = ctx;
    asm!(
        "r1 = 0;",
        "r2 = 0;",
        "goto +0;",
        options(nostack)
    );

    0
}

/* SEC("?raw_tp")
 * __success __log_level(2)
 * __msg("fp-8=iter_num(id=1,state=active,depth=0)")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_and_destroy(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    asm!(
        /* create iterator */
        "r1 = {iter};",
        "r2 = 0;",
        "r3 = 1000;",
        "call {bpf_iter_num_new};",
        /* destroy iterator */
        "r1 = {iter};",
        "call {bpf_iter_num_destroy};",
        iter = in(reg) iter.as_mut_ptr(),
        bpf_iter_num_new = sym bpf_iter_num_new,
        bpf_iter_num_destroy = sym bpf_iter_num_destroy,
    );

    0
}

/* SEC("?raw_tp")
 * __failure __msg("Unreleased reference id=1")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_and_forget_to_destroy_fail(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    asm!(
        /* create iterator */
        "r1 = {iter};",
        "r2 = 0;",
        "r3 = 1000;",
        "call {bpf_iter_num_new};",
        iter = in(reg) iter.as_mut_ptr(),
        bpf_iter_num_new = sym bpf_iter_num_new,
    );

    0
}

/* SEC("?raw_tp")
 * __failure __msg("expected an initialized iter_num as R1")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_without_creating_fail(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    /* init with zeros to stop verifier complaining about uninit stack */
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::zeroed();

    asm!(
        "r1 = {iter};",
        "call {bpf_iter_num_destroy};",
        iter = in(reg) iter.as_mut_ptr(),
        bpf_iter_num_destroy = sym bpf_iter_num_destroy,
    );

    0
}

/* SEC("?raw_tp")
 * __failure __msg("expected an initialized iter_num as R1")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn compromise_iter_w_direct_write_fail(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    asm!(
        /* create iterator */
        "r1 = {iter};",
        "r2 = 0;",
        "r3 = 1000;",
        "call {bpf_iter_num_new};",
        /* directly write over first half of iter state */
        "*(u64 *)({iter} + 0) = r0;",
        /* (attempt to) destroy iterator */
        "r1 = {iter};",
        "call {bpf_iter_num_destroy};",
        iter = in(reg) iter.as_mut_ptr(),
        bpf_iter_num_new = sym bpf_iter_num_new,
        bpf_iter_num_destroy = sym bpf_iter_num_destroy,
    );

    0
}

/* SEC("?raw_tp")
 * __failure __msg("Unreleased reference id=1")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn compromise_iter_w_direct_write_and_skip_destroy_fail(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    asm!(
        /* create iterator */
        "r1 = {iter};",
        "r2 = 0;",
        "r3 = 1000;",
        "call {bpf_iter_num_new};",
        /* directly write over first half of iter state */
        "*(u64 *)({iter} + 0) = r0;",
        /* don't destroy iter, leaking ref, which should fail */
        iter = in(reg) iter.as_mut_ptr(),
        bpf_iter_num_new = sym bpf_iter_num_new,
    );

    0
}

/* SEC("?raw_tp")
 * __failure __msg("expected an initialized iter_num as R1")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn compromise_iter_w_helper_write_fail(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    asm!(
        /* create iterator */
        "r1 = {iter};",
        "r2 = 0;",
        "r3 = 1000;",
        "call {bpf_iter_num_new};",
        /* overwrite 8th byte with bpf_probe_read_kernel() */
        "r1 = {iter};",
        "r1 += 7;",
        "r2 = 1;",
        "r3 = 0;", /* NULL */
        "call {bpf_probe_read_kernel};",
        /* (attempt to) destroy iterator */
        "r1 = {iter};",
        "call {bpf_iter_num_destroy};",
        iter = in(reg) iter.as_mut_ptr(),
        bpf_iter_num_new = sym bpf_iter_num_new,
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        bpf_iter_num_destroy = sym bpf_iter_num_destroy,
    );

    0
}

#[inline(never)]
unsafe extern "C" fn subprog_with_iter() {
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    bpf_iter_num_new(iter.as_mut_ptr(), 0, 1);

    return;
}

/* SEC("?raw_tp")
 * __failure
 * ensure there was a call to subprog, which might happen without __noinline
 * __msg("returning from callee:")
 * __msg("Unreleased reference id=1")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn leak_iter_from_subprog_fail(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    subprog_with_iter();

    0
}

/* SEC("?raw_tp")
 * __success __log_level(2)
 * __msg("fp-8=iter_num(id=1,state=active,depth=0)")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn valid_stack_reuse(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    asm!(
        /* create iterator */
        "r1 = {iter};",
        "r2 = 0;",
        "r3 = 1000;",
        "call {bpf_iter_num_new};",
        /* destroy iterator */
        "r1 = {iter};",
        "call {bpf_iter_num_destroy};",
        /* now reuse same stack slots */
        /* create iterator */
        "r1 = {iter};",
        "r2 = 0;",
        "r3 = 1000;",
        "call {bpf_iter_num_new};",
        /* destroy iterator */
        "r1 = {iter};",
        "call {bpf_iter_num_destroy};",
        iter = in(reg) iter.as_mut_ptr(),
        bpf_iter_num_new = sym bpf_iter_num_new,
        bpf_iter_num_destroy = sym bpf_iter_num_destroy,
    );

    0
}

/* SEC("?raw_tp")
 * __failure __msg("expected uninitialized iter_num as R1")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn double_create_fail(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    asm!(
        /* create iterator */
        "r1 = {iter};",
        "r2 = 0;",
        "r3 = 1000;",
        "call {bpf_iter_num_new};",
        /* (attempt to) create iterator again */
        "r1 = {iter};",
        "r2 = 0;",
        "r3 = 1000;",
        "call {bpf_iter_num_new};",
        /* destroy iterator */
        "r1 = {iter};",
        "call {bpf_iter_num_destroy};",
        iter = in(reg) iter.as_mut_ptr(),
        bpf_iter_num_new = sym bpf_iter_num_new,
        bpf_iter_num_destroy = sym bpf_iter_num_destroy,
    );

    0
}

/* SEC("?raw_tp")
 * __failure __msg("expected an initialized iter_num as R1")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn double_destroy_fail(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    asm!(
        /* create iterator */
        "r1 = {iter};",
        "r2 = 0;",
        "r3 = 1000;",
        "call {bpf_iter_num_new};",
        /* destroy iterator */
        "r1 = {iter};",
        "call {bpf_iter_num_destroy};",
        /* (attempt to) destroy iterator again */
        "r1 = {iter};",
        "call {bpf_iter_num_destroy};",
        iter = in(reg) iter.as_mut_ptr(),
        bpf_iter_num_new = sym bpf_iter_num_new,
        bpf_iter_num_destroy = sym bpf_iter_num_destroy,
    );

    0
}

/* SEC("?raw_tp")
 * __failure __msg("expected an initialized iter_num as R1")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn next_without_new_fail(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    asm!(
        /* don't create iterator and try to iterate*/
        "r1 = {iter};",
        "call {bpf_iter_num_next};",
        /* destroy iterator */
        "r1 = {iter};",
        "call {bpf_iter_num_destroy};",
        iter = in(reg) iter.as_mut_ptr(),
        bpf_iter_num_next = sym bpf_iter_num_next,
        bpf_iter_num_destroy = sym bpf_iter_num_destroy,
    );

    0
}

/* SEC("?raw_tp")
 * __failure __msg("expected an initialized iter_num as R1")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn next_after_destroy_fail(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    asm!(
        /* create iterator */
        "r1 = {iter};",
        "r2 = 0;",
        "r3 = 1000;",
        "call {bpf_iter_num_new};",
        /* destroy iterator */
        "r1 = {iter};",
        "call {bpf_iter_num_destroy};",
        /* don't create iterator and try to iterate*/
        "r1 = {iter};",
        "call {bpf_iter_num_next};",
        iter = in(reg) iter.as_mut_ptr(),
        bpf_iter_num_new = sym bpf_iter_num_new,
        bpf_iter_num_destroy = sym bpf_iter_num_destroy,
        bpf_iter_num_next = sym bpf_iter_num_next,
    );

    0
}

/* SEC("?raw_tp")
 * __failure __msg("invalid read from stack")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_from_iter_slot_fail() -> i32 {
    asm!(
        /* r6 points to struct bpf_iter_num on the stack */
        "r6 = r10;",
        "r6 += -24;",
        /* create iterator */
        "r1 = r6;",
        "r2 = 0;",
        "r3 = 1000;",
        "call {bpf_iter_num_new};",
        /* attempt to leak bpf_iter_num state */
        "r7 = *(u64 *)(r6 + 0);",
        "r8 = *(u64 *)(r6 + 8);",
        /* destroy iterator */
        "r1 = r6;",
        "call {bpf_iter_num_destroy};",
        /* leak bpf_iter_num state */
        "r0 = r7;",
        "if r7 > r8 goto +1;",
        "r0 = r8;",
        "exit;",
        bpf_iter_num_new = sym bpf_iter_num_new,
        bpf_iter_num_destroy = sym bpf_iter_num_destroy,
    );
    core::hint::unreachable_unchecked()
}

#[unsafe(no_mangle)]
pub static mut zero: i32 = 0;

/* SEC("?raw_tp")
 * __failure
 * __flag(BPF_F_TEST_STATE_FREQ)
 * __msg("Unreleased reference")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stacksafe_should_not_conflate_stack_spill_and_iter(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    asm!(
        /* Create a fork in logic, with general setup as follows:
         *   - fallthrough (first) path is valid;
         *   - branch (second) path is invalid.
         * Then depending on what we do in fallthrough vs branch path,
         * we try to detect bugs in func_states_equal(), regsafe(),
         * refsafe(), stack_safe(), and similar by tricking verifier
         * into believing that branch state is a valid subset of
         * a fallthrough state. Verifier should reject overall
         * validation, unless there is a bug somewhere in verifier
         * logic.
         */
        "call {bpf_get_prandom_u32};",
        "r6 = r0;",
        "call {bpf_get_prandom_u32};",
        "r7 = r0;",
        "if r6 > r7 goto bad;", /* fork */
        /* spill r6 into stack slot of bpf_iter_num var */
        "*(u64 *)({iter} + 0) = r6;",
        "goto skip_bad;",
        "bad:",
        /* create iterator in the same stack slot */
        "r1 = {iter};",
        "r2 = 0;",
        "r3 = 1000;",
        "call {bpf_iter_num_new};",
        /* but then forget about it and overwrite it back to r6 spill */
        "*(u64 *)({iter} + 0) = r6;",
        "skip_bad:",
        "goto +0;", /* force checkpoint */
        /* corrupt stack slots, if they are really dynptr */
        "*(u64 *)({iter} + 0) = r6;",
        iter = in(reg) iter.as_mut_ptr(),
        zero = sym zero,
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        bpf_dynptr_from_mem = sym bpf_dynptr_from_mem,
        bpf_iter_num_new = sym bpf_iter_num_new,
    );

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
