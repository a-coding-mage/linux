// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// Dependencies in the original C source:
// errno.h, string.h, linux/bpf.h, bpf/bpf_helpers.h, bpf_misc.h,
// ../../../tools/include/linux/filter.h

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::asm;

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;
const BPF_F_TEST_STATE_FREQ: u32 = 1 << 0;
const BPF_REG_0: u32 = 0;
const BPF_REG_1: u32 = 1;
const BPF_REG_10: u32 = 10;

// External BPF helper symbols supplied by the BPF build environment.
unsafe extern "C" {
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_loop(nr_loops: u32, callback_fn: *const (), callback_ctx: *mut (), flags: u64) -> i64;
    fn bpf_tail_call(ctx: *mut (), prog_array_map: *const (), index: u32) -> i64;
}

const fn BPF_MOVSX64_REG(dst: u32, src: u32, off: u32) -> u64 {
    ((dst as u64) << 0) | ((src as u64) << 8) | ((off as u64) << 16)
}

#[unsafe(link_section = ".data.vals")]
#[unsafe(no_mangle)]
pub static mut vals: [i32; 4] = [1, 2, 3, 4];

#[inline(never)]
#[used]
unsafe extern "C" fn identity_subprog() -> u64 {
    /* the simplest *static* 64-bit identity function */
    unsafe {
        asm!(
            "r0 = r1",
            "exit",
            options(noreturn)
        );
    }
}

#[inline(never)]
#[used]
#[unsafe(no_mangle)]
pub extern "C" fn global_identity_subprog(x: __u64) -> u64 {
    /* the simplest *global* 64-bit identity function */
    x
}

#[inline(never)]
#[used]
unsafe extern "C" fn callback_subprog() -> u64 {
    /* the simplest callback function */
    unsafe {
        asm!(
            "r0 = 0",
            "exit",
            options(noreturn)
        );
    }
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("7: (0f) r1 += r0")
// __msg("mark_precise: frame0: regs=r0 stack= before 6: (bf) r1 = r7")
// __msg("mark_precise: frame0: regs=r0 stack= before 5: (27) r0 *= 4")
// __msg("mark_precise: frame0: regs=r0 stack= before 11: (95) exit")
// __msg("mark_precise: frame1: regs=r0 stack= before 10: (bf) r0 = r1")
// __msg("mark_precise: frame1: regs=r1 stack= before 4: (85) call pc+5")
// __msg("mark_precise: frame0: regs=r1 stack= before 3: (bf) r1 = r6")
// __msg("mark_precise: frame0: regs=r6 stack= before 2: (b7) r6 = 3")
#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn subprog_result_precise() -> i32 {
    unsafe {
        asm!(
            "r6 = 3",
            "r1 = r6",
            "call identity_subprog",
            "r0 *= 4",
            "r1 = {vals}",
            "r1 += r0",
            "r0 = *(u32 *)(r1 + 0)",
            "exit",
            vals = sym vals,
            options(noreturn)
        );
    }
}

#[inline(never)]
#[used]
unsafe extern "C" fn fp_leaking_subprog() -> u64 {
    unsafe {
        asm!(
            ".8byte {r0_eq_r10_cast_s8}",
            "exit",
            r0_eq_r10_cast_s8 = const BPF_MOVSX64_REG(BPF_REG_0, BPF_REG_10, 8),
            options(noreturn)
        );
    }
}

#[inline(never)]
#[used]
unsafe extern "C" fn sneaky_fp_leaking_subprog() -> u64 {
    unsafe {
        asm!(
            "r1 = r10",
            ".8byte {r0_eq_r1_cast_s8}",
            "exit",
            r0_eq_r1_cast_s8 = const BPF_MOVSX64_REG(BPF_REG_0, BPF_REG_1, 8),
            options(noreturn)
        );
    }
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("6: (0f) r1 += r0")
// __msg("mark_precise: frame0: last_idx 6 first_idx 0 subseq_idx -1")
// __msg("mark_precise: frame0: regs=r0 stack= before 5: (bf) r1 = r6")
// __msg("mark_precise: frame0: regs=r0 stack= before 4: (27) r0 *= 4")
// __msg("mark_precise: frame0: regs=r0 stack= before 3: (57) r0 &= 3")
// __msg("mark_precise: frame0: regs=r0 stack= before 10: (95) exit")
// __msg("mark_precise: frame1: regs=r0 stack= before 9: (bf) r0 = (s8)r10")
// __msg("7: R0=scalar")
#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fp_precise_subprog_result() -> i32 {
    unsafe {
        asm!(
            "call fp_leaking_subprog",
            "r0 &= 3",
            "r0 *= 4",
            "r1 = {vals}",
            "r1 += r0",
            "r0 = *(u32 *)(r1 + 0)",
            "exit",
            vals = sym vals,
            options(noreturn)
        );
    }
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("6: (0f) r1 += r0")
// __msg("mark_precise: frame0: last_idx 6 first_idx 0 subseq_idx -1")
// __msg("mark_precise: frame0: regs=r0 stack= before 5: (bf) r1 = r6")
// __msg("mark_precise: frame0: regs=r0 stack= before 4: (27) r0 *= 4")
// __msg("mark_precise: frame0: regs=r0 stack= before 3: (57) r0 &= 3")
// __msg("mark_precise: frame0: regs=r0 stack= before 11: (95) exit")
// __msg("mark_precise: frame1: regs=r0 stack= before 10: (bf) r0 = (s8)r1")
/* here r1 is marked precise, even though it's fp register, but that's fine
 * because by the time we get out of subprogram it has to be derived from r10
 * anyways, at which point we'll break precision chain
 */
// __msg("mark_precise: frame1: regs=r1 stack= before 9: (bf) r1 = r10")
// __msg("7: R0=scalar")
#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sneaky_fp_precise_subprog_result() -> i32 {
    unsafe {
        asm!(
            "call sneaky_fp_leaking_subprog",
            "r0 &= 3",
            "r0 *= 4",
            "r1 = {vals}",
            "r1 += r0",
            "r0 = *(u32 *)(r1 + 0)",
            "exit",
            vals = sym vals,
            options(noreturn)
        );
    }
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("9: (0f) r1 += r0")
// __msg("mark_precise: frame0: last_idx 9 first_idx 0")
// __msg("mark_precise: frame0: regs=r0 stack= before 8: (bf) r1 = r7")
// __msg("mark_precise: frame0: regs=r0 stack= before 7: (27) r0 *= 4")
// __msg("mark_precise: frame0: regs=r0 stack= before 5: (a5) if r0 < 0x4 goto pc+1")
// __msg("mark_precise: frame0: regs=r0 stack= before 4: (85) call pc+7")
#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_subprog_result_precise() -> i32 {
    unsafe {
        asm!(
            "r6 = 3",
            "r1 = r6",
            "call global_identity_subprog",
            "if r0 < {vals_arr_sz} goto 1f",
            "r0 = {vals_arr_sz} - 1",
            "1:",
            "r0 *= 4",
            "r1 = {vals}",
            "r1 += r0",
            "r0 = *(u32 *)(r1 + 0)",
            "exit",
            vals = sym vals,
            vals_arr_sz = const 4,
            options(noreturn)
        );
    }
}

#[inline(never)]
#[used]
unsafe extern "C" fn loop_callback_bad() -> u64 {
    /* bpf_loop() callback that can return values outside of [0, 1] range */
    unsafe {
        asm!(
            "call {bpf_get_prandom_u32}",
            "if r0 s> 1000 goto 1f",
            "r0 = 0",
            "1:",
            "goto +0",
            "exit",
            bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
            options(noreturn)
        );
    }
}

// SEC("?raw_tp")
// __failure __log_level(2)
// __flag(BPF_F_TEST_STATE_FREQ)
/* check that fallthrough code path marks r0 as precise */
// __msg("mark_precise: frame1: regs=r0 stack= before 11: (b7) r0 = 0")
/* check that we have branch code path doing its own validation */
// __msg("from 10 to 12: frame1: R0=scalar(smin=umin=1001")
/* check that branch code path marks r0 as precise, before failing */
// __msg("mark_precise: frame1: regs=r0 stack= before 9: (85) call bpf_get_prandom_u32#7")
// __msg("At callback return the register R0 has smin=1001 should have been in [0, 1]")
#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn callback_precise_return_fail() -> i32 {
    unsafe {
        asm!(
            "r1 = 1",
            "r2 = {loop_callback_bad}",
            "r3 = 0",
            "r4 = 0",
            "call {bpf_loop}",
            "r0 = 0",
            "exit",
            loop_callback_bad = sym loop_callback_bad,
            bpf_loop = sym bpf_loop,
            options(noreturn)
        );
    }
}

// SEC("?raw_tp")
// __success __log_level(2)
/* First simulated path does not include callback body,
 * r1 and r4 are always precise for bpf_loop() calls.
 */
// __msg("9: (85) call bpf_loop#181")
// __msg("mark_precise: frame0: last_idx 9 first_idx 9 subseq_idx -1")
// __msg("mark_precise: frame0: parent state regs=r4 stack=:")
// __msg("mark_precise: frame0: last_idx 8 first_idx 0 subseq_idx 9")
// __msg("mark_precise: frame0: regs=r4 stack= before 8: (b7) r4 = 0")
// __msg("mark_precise: frame0: last_idx 9 first_idx 9 subseq_idx -1")
// __msg("mark_precise: frame0: parent state regs=r1 stack=:")
// __msg("mark_precise: frame0: last_idx 8 first_idx 0 subseq_idx 9")
// __msg("mark_precise: frame0: regs=r1 stack= before 8: (b7) r4 = 0")
// __msg("mark_precise: frame0: regs=r1 stack= before 7: (b7) r3 = 0")
// __msg("mark_precise: frame0: regs=r1 stack= before 6: (bf) r2 = r8")
// __msg("mark_precise: frame0: regs=r1 stack= before 5: (bf) r1 = r6")
// __msg("mark_precise: frame0: regs=r6 stack= before 4: (b7) r6 = 3")
/* r6 precision propagation */
// __msg("14: (0f) r1 += r6")
// __msg("mark_precise: frame0: last_idx 14 first_idx 9")
// __msg("mark_precise: frame0: regs=r6 stack= before 13: (bf) r1 = r7")
// __msg("mark_precise: frame0: regs=r6 stack= before 12: (27) r6 *= 4")
// __msg("mark_precise: frame0: regs=r6 stack= before 11: (25) if r6 > 0x3 goto pc+4")
// __msg("mark_precise: frame0: regs=r0,r6 stack= before 10: (bf) r6 = r0")
// __msg("mark_precise: frame0: regs=r0 stack= before 9: (85) call bpf_loop")
/* State entering callback body popped from states stack */
// __msg("from 9 to 17: frame1:")
// __msg("17: frame1: R10=fp0 cb")
// __msg("17: (b7) r0 = 0")
// __msg("18: (95) exit")
// __msg("returning from callee:")
// __msg("to caller at 9:")
// __msg("frame 0: propagating r1,r4")
// __msg("mark_precise: frame0: last_idx 9 first_idx 9 subseq_idx -1")
// __msg("mark_precise: frame0: regs=r1,r4 stack= before 18: (95) exit")
// __msg("from 18 to 9: safe")
#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn callback_result_precise() -> i32 {
    unsafe {
        asm!(
            "r6 = 3",
            "r1 = r6",
            "r2 = {callback_subprog}",
            "r3 = 0",
            "r4 = 0",
            "call {bpf_loop}",
            "r6 = r0",
            "if r6 > 3 goto 1f",
            "r6 *= 4",
            "r1 = {vals}",
            "r1 += r6",
            "r0 = *(u32 *)(r1 + 0)",
            "1:",
            "exit",
            vals = sym vals,
            callback_subprog = sym callback_subprog,
            bpf_loop = sym bpf_loop,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parent_callee_saved_reg_precise() -> i32 {
    unsafe {
        asm!(
            "r6 = 3",
            "r1 = 0",
            "call identity_subprog",
            "r6 *= 4",
            "r1 = {vals}",
            "r1 += r6",
            "r0 = *(u32 *)(r1 + 0)",
            "exit",
            vals = sym vals,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parent_callee_saved_reg_precise_global() -> i32 {
    unsafe {
        asm!(
            "r6 = 3",
            "r1 = 0",
            "call global_identity_subprog",
            "r6 *= 4",
            "r1 = {vals}",
            "r1 += r6",
            "r0 = *(u32 *)(r1 + 0)",
            "exit",
            vals = sym vals,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parent_callee_saved_reg_precise_with_callback() -> i32 {
    unsafe {
        asm!(
            "r6 = 3",
            "r1 = 1",
            "r2 = {callback_subprog}",
            "r3 = 0",
            "r4 = 0",
            "call {bpf_loop}",
            "r6 *= 4",
            "r1 = {vals}",
            "r1 += r6",
            "r0 = *(u32 *)(r1 + 0)",
            "exit",
            vals = sym vals,
            callback_subprog = sym callback_subprog,
            bpf_loop = sym bpf_loop,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parent_stack_slot_precise() -> i32 {
    unsafe {
        asm!(
            "r6 = 3",
            "*(u64 *)(r10 - 8) = r6",
            "r1 = 0",
            "call identity_subprog",
            "r6 = *(u64 *)(r10 - 8)",
            "r6 *= 4",
            "r1 = {vals}",
            "r1 += r6",
            "r0 = *(u32 *)(r1 + 0)",
            "exit",
            vals = sym vals,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parent_stack_slot_precise_global() -> i32 {
    unsafe {
        asm!(
            "r6 = 3",
            "*(u64 *)(r10 - 8) = r6",
            "r1 = 0",
            "call global_identity_subprog",
            "r6 = *(u64 *)(r10 - 8)",
            "r6 *= 4",
            "r1 = {vals}",
            "r1 += r6",
            "r0 = *(u32 *)(r1 + 0)",
            "exit",
            vals = sym vals,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parent_stack_slot_precise_with_callback() -> i32 {
    unsafe {
        asm!(
            "r6 = 3",
            "*(u64 *)(r10 - 8) = r6",
            "r1 = r6",
            "r2 = {callback_subprog}",
            "r3 = 0",
            "r4 = 0",
            "call {bpf_loop}",
            "r6 = *(u64 *)(r10 - 8)",
            "r6 *= 4",
            "r1 = {vals}",
            "r1 += r6",
            "r0 = *(u32 *)(r1 + 0)",
            "exit",
            vals = sym vals,
            callback_subprog = sym callback_subprog,
            bpf_loop = sym bpf_loop,
            options(noreturn)
        );
    }
}

#[inline(never)]
#[used]
unsafe extern "C" fn subprog_with_precise_arg(x: __u64) -> __u64 {
    unsafe { vals[x as usize] as __u64 } /* x is forced to be precise */
}

#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn subprog_arg_precise() -> i32 {
    unsafe {
        asm!(
            "r6 = 3",
            "r1 = r6",
            "call subprog_with_precise_arg",
            "r0 += r6",
            "exit",
            options(noreturn)
        );
    }
}

/* r1 is pointer to stack slot;
 * r2 is a register to spill into that slot
 * subprog also spills r2 into its own stack slot
 */
#[inline(never)]
#[used]
unsafe extern "C" fn subprog_spill_reg_precise() -> __u64 {
    unsafe {
        asm!(
            "*(u64 *)(r1 + 0) = r2",
            "*(u64 *)(r10 - 16) = r2",
            "r0 = *(u64 *)(r10 - 16)",
            "r2 = *(u64 *)(r1 + 0)",
            "r0 += r2",
            "exit",
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn subprog_spill_into_parent_stack_slot_precise() -> i32 {
    unsafe {
        asm!(
            "r6 = 1",
            "r1 = r10",
            "r1 += -8",
            "r2 = r6",
            "call subprog_spill_reg_precise",
            "r7 = *(u64 *)(r10 - 8)",
            "r7 *= 4",
            "r1 = {vals}",
            "r1 += r7",
            "r0 = *(u32 *)(r1 + 0)",
            "exit",
            vals = sym vals,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stack_slot_aliases_precision() -> i32 {
    unsafe {
        asm!(
            "r6 = 1",
            "r1 = r6",
            "call identity_subprog",
            "r7 = r10",
            "r7 += -8",
            "r8 = r10",
            "r8 += -32",
            "*(u64 *)(r10 - 16) = r0",
            "r0 = *(u64 *)(r7 - 8)",
            "*(u64 *)(r8 + 16) = r0",
            "r0 = *(u64 *)(r8 + 16)",
            "*(u64 *)(r7 - 8) = r0",
            "r0 = *(u64 *)(r10 - 16)",
            "r0 *= 4",
            "r1 = {vals}",
            "r1 += r0",
            "r0 = *(u32 *)(r1 + 0)",
            "exit",
            vals = sym vals,
            options(noreturn)
        );
    }
}

#[repr(C)]
pub struct map_array_def {
    type_: u32,
    max_entries: u32,
    key_size: u32,
    value_size: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static map_array: map_array_def = map_array_def {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u32>() as u32,
};

#[inline(never)]
#[used]
unsafe extern "C" fn identity_tail_call() -> u64 {
    /* the simplest identity function involving a tail call */
    unsafe {
        asm!(
            "r6 = r2",
            "r2 = {map_array} ll",
            "r3 = 0",
            "call {bpf_tail_call}",
            "r0 = r6",
            "exit",
            map_array = sym map_array,
            bpf_tail_call = sym bpf_tail_call,
            options(noreturn)
        );
    }
}

// SEC("?raw_tp")
// __failure __log_level(2)
// __msg("13: (85) call bpf_tail_call#12")
// __msg("mark_precise: frame1: last_idx 13 first_idx 0 subseq_idx -1 ")
// __msg("returning from callee:")
// __msg("frame1: R0=scalar() R6=3 R10=fp0")
// __msg("to caller at 4:")
// __msg("R0=scalar() R6=map_value(map=.data.vals,ks=4,vs=16) R10=fp0")
// __msg("6: (0f) r1 += r0")
// __msg("mark_precise: frame0: regs=r0 stack= before 5: (bf) r1 = r6")
// __msg("mark_precise: frame0: regs=r0 stack= before 4: (27) r0 *= 4")
// __msg("mark_precise: frame0: parent state regs=r0 stack=:  R0=Pscalar() R6=map_value(map=.data.vals,ks=4,vs=16) R10=fp0")
// __msg("math between map_value pointer and register with unbounded min value is not allowed")
#[unsafe(link_section = "?raw_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn subprog_result_tail_call() -> i32 {
    unsafe {
        asm!(
            "r2 = 3",
            "call identity_tail_call",
            "r0 *= 4",
            "r1 = {vals}",
            "r1 += r0",
            "r0 = *(u32 *)(r1 + 0)",
            "exit",
            vals = sym vals,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
