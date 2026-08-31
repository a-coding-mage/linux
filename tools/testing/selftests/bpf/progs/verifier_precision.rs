// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023 SUSE LLC */
// C dependencies removed from executable Rust:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, "../../../include/linux/filter.h", "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct precision_map {
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __uint(max_entries, 1);
    // __type(key, __u32);
    // __type(value, __u64);
    _private: [u8; 0],
}

// SEC(".maps")
#[no_mangle]
pub static mut precision_map: precision_map = precision_map { _private: [] };

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r2 stack= before 3: (bf) r1 = r10")
// __msg("mark_precise: frame0: regs=r2 stack= before 2: (55) if r2 != 0xfffffff8 goto pc+2")
// __msg("mark_precise: frame0: regs=r2 stack= before 1: (87) r2 = -r2")
// __msg("mark_precise: frame0: regs=r2 stack= before 0: (b7) r2 = 8")
#[naked]
pub unsafe extern "C" fn bpf_neg() -> i32 {
    core::arch::asm!(
        "r2 = 8;",
        "r2 = -r2;",
        "if r2 != -8 goto 1f;",
        "r1 = r10;",
        "r1 += r2;",
        "1:",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r2 stack= before 3: (bf) r1 = r10")
// __msg("mark_precise: frame0: regs=r2 stack= before 2: (55) if r2 != 0x0 goto pc+2")
// __msg("mark_precise: frame0: regs=r2 stack= before 1: (d4) r2 = le16 r2")
// __msg("mark_precise: frame0: regs=r2 stack= before 0: (b7) r2 = 0")
#[naked]
pub unsafe extern "C" fn bpf_end_to_le() -> i32 {
    core::arch::asm!(
        "r2 = 0;",
        "r2 = le16 r2;",
        "if r2 != 0 goto 1f;",
        "r1 = r10;",
        "r1 += r2;",
        "1:",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r2 stack= before 3: (bf) r1 = r10")
// __msg("mark_precise: frame0: regs=r2 stack= before 2: (55) if r2 != 0x0 goto pc+2")
// __msg("mark_precise: frame0: regs=r2 stack= before 1: (dc) r2 = be16 r2")
// __msg("mark_precise: frame0: regs=r2 stack= before 0: (b7) r2 = 0")
#[naked]
pub unsafe extern "C" fn bpf_end_to_be() -> i32 {
    core::arch::asm!(
        "r2 = 0;",
        "r2 = be16 r2;",
        "if r2 != 0 goto 1f;",
        "r1 = r10;",
        "r1 += r2;",
        "1:",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

// C condition preserved:
// #if (defined(__TARGET_ARCH_arm64) || defined(__TARGET_ARCH_x86) || \
//      (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64) || \
//      defined(__TARGET_ARCH_arm) || defined(__TARGET_ARCH_s390) || \
//      defined(__TARGET_ARCH_loongarch)) && __clang_major__ >= 18

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r2 stack= before 3: (bf) r1 = r10")
// __msg("mark_precise: frame0: regs=r2 stack= before 2: (55) if r2 != 0x0 goto pc+2")
// __msg("mark_precise: frame0: regs=r2 stack= before 1: (d7) r2 = bswap16 r2")
// __msg("mark_precise: frame0: regs=r2 stack= before 0: (b7) r2 = 0")
#[naked]
pub unsafe extern "C" fn bpf_end_bswap() -> i32 {
    core::arch::asm!(
        "r2 = 0;",
        "r2 = bswap16 r2;",
        "if r2 != 0 goto 1f;",
        "r1 = r10;",
        "r1 += r2;",
        "1:",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

// C condition preserved: #ifdef CAN_USE_LOAD_ACQ_STORE_REL

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r2 stack= before 3: (bf) r3 = r10")
// __msg("mark_precise: frame0: regs=r2 stack= before 2: (db) r2 = load_acquire((u64 *)(r10 -8))")
// __msg("mark_precise: frame0: regs= stack=-8 before 1: (7b) *(u64 *)(r10 -8) = r1")
// __msg("mark_precise: frame0: regs=r1 stack= before 0: (b7) r1 = 8")
#[naked]
pub unsafe extern "C" fn bpf_load_acquire() -> i32 {
    core::arch::asm!(
        "r1 = 8;",
        "*(u64 *)(r10 - 8) = r1;",
        ".8byte {load_acquire_insn};",
        "r3 = r10;",
        "r3 += r2;",
        "r0 = 0;",
        "exit;",
        load_acquire_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_LOAD_ACQ, BPF_REG_2, BPF_REG_10, -8),
        options(noreturn)
    );
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r1 stack= before 3: (bf) r2 = r10")
// __msg("mark_precise: frame0: regs=r1 stack= before 2: (79) r1 = *(u64 *)(r10 -8)")
// __msg("mark_precise: frame0: regs= stack=-8 before 1: (db) store_release((u64 *)(r10 -8), r1)")
// __msg("mark_precise: frame0: regs=r1 stack= before 0: (b7) r1 = 8")
#[naked]
pub unsafe extern "C" fn bpf_store_release() -> i32 {
    core::arch::asm!(
        "r1 = 8;",
        ".8byte {store_release_insn};",
        "r1 = *(u64 *)(r10 - 8);",
        "r2 = r10;",
        "r2 += r1;",
        "r0 = 0;",
        "exit;",
        store_release_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_STORE_REL, BPF_REG_10, BPF_REG_1, -8),
        options(noreturn)
    );
}

// #endif /* CAN_USE_LOAD_ACQ_STORE_REL */
// #endif /* v4 instruction */

// SEC("?raw_tp")
// __success __log_level(2)
/*
 * Without the bug fix there will be no history between "last_idx 3 first_idx 3"
 * and "parent state regs=" lines. "R0=6" parts are here to help anchor
 * expected log messages to the one specific mark_chain_precision operation.
 *
 * This is quite fragile: if verifier checkpointing heuristic changes, this
 * might need adjusting.
 */
// __msg("2: (07) r0 += 1                       ; R0=6")
// __msg("3: (35) if r0 >= 0xa goto pc+1")
// __msg("mark_precise: frame0: last_idx 3 first_idx 3 subseq_idx -1")
// __msg("mark_precise: frame0: regs=r0 stack= before 2: (07) r0 += 1")
// __msg("mark_precise: frame0: regs=r0 stack= before 1: (07) r0 += 1")
// __msg("mark_precise: frame0: regs=r0 stack= before 4: (05) goto pc-4")
// __msg("mark_precise: frame0: regs=r0 stack= before 3: (35) if r0 >= 0xa goto pc+1")
// __msg("mark_precise: frame0: parent state regs= stack=:  R0=P4")
// __msg("3: R0=6")
#[naked]
pub unsafe extern "C" fn state_loop_first_last_equal() -> i32 {
    core::arch::asm!(
        "r0 = 0;",
        "l0_{0}:",
        "r0 += 1;",
        "r0 += 1;",
        "if r0 >= 10 goto l1_{0};",
        "goto l0_{0};",
        "l1_{0}:",
        "exit;",
        const 0,
        options(noreturn)
    );
}

#[used]
#[naked]
unsafe extern "C" fn __bpf_cond_op_r10() {
    core::arch::asm!(
        "r2 = 2314885393468386424 ll;",
        "goto +0;",
        "if r2 <= r10 goto +3;",
        "if r1 >= -1835016 goto +0;",
        "if r2 <= 8 goto +0;",
        "if r3 <= 0 goto +0;",
        "exit;",
        options(noreturn)
    );
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("8: (bd) if r2 <= r10 goto pc+3")
// __msg("9: (35) if r1 >= 0xffe3fff8 goto pc+0")
// __msg("10: (b5) if r2 <= 0x8 goto pc+0")
// __msg("mark_precise: frame1: last_idx 10 first_idx 0 subseq_idx -1")
// __msg("mark_precise: frame1: regs=r2 stack= before 9: (35) if r1 >= 0xffe3fff8 goto pc+0")
// __msg("mark_precise: frame1: regs=r2 stack= before 8: (bd) if r2 <= r10 goto pc+3")
// __msg("mark_precise: frame1: regs=r2 stack= before 7: (05) goto pc+0")
#[naked]
pub unsafe extern "C" fn bpf_cond_op_r10() {
    core::arch::asm!(
        "r3 = 0 ll;",
        "call __bpf_cond_op_r10;",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("3: (bf) r3 = r10")
// __msg("4: (bd) if r3 <= r2 goto pc+1")
// __msg("5: (b5) if r2 <= 0x8 goto pc+2")
// __msg("mark_precise: frame0: last_idx 5 first_idx 0 subseq_idx -1")
// __msg("mark_precise: frame0: regs=r2 stack= before 4: (bd) if r3 <= r2 goto pc+1")
// __msg("mark_precise: frame0: regs=r2 stack= before 3: (bf) r3 = r10")
#[naked]
pub unsafe extern "C" fn bpf_cond_op_not_r10() {
    core::arch::asm!(
        "r0 = 0;",
        "r2 = 2314885393468386424 ll;",
        "r3 = r10;",
        "if r3 <= r2 goto +1;",
        "if r2 <= 8 goto +2;",
        "r0 = 2 ll;",
        "exit;",
        options(noreturn)
    );
}

// SEC("lsm.s/socket_connect")
// __success __log_level(2)
// __msg("0: (b7) r0 = 1                        ; R0=1")
// __msg("1: (84) w0 = -w0                      ; R0=0xffffffff")
// __msg("mark_precise: frame0: last_idx 2 first_idx 0 subseq_idx -1")
// __msg("mark_precise: frame0: regs=r0 stack= before 1: (84) w0 = -w0")
// __msg("mark_precise: frame0: regs=r0 stack= before 0: (b7) r0 = 1")
#[naked]
pub unsafe extern "C" fn bpf_neg_2() -> i32 {
    /*
     * lsm.s/socket_connect requires a return value within [-4095, 0].
     * Returning -1 is allowed
     */
    core::arch::asm!("r0 = 1;", "w0 = -w0;", "exit;", options(noreturn));
}

// SEC("lsm.s/socket_connect")
// __failure __msg("At program exit the register R0 has")
#[naked]
pub unsafe extern "C" fn bpf_neg_3() -> i32 {
    /*
     * lsm.s/socket_connect requires a return value within [-4095, 0].
     * Returning -10000 is not allowed.
     */
    core::arch::asm!("r0 = 10000;", "w0 = -w0;", "exit;", options(noreturn));
}

// SEC("lsm.s/socket_connect")
// __success __log_level(2)
// __msg("0: (b7) r0 = 1                        ; R0=1")
// __msg("1: (87) r0 = -r0                      ; R0=-1")
// __msg("mark_precise: frame0: last_idx 2 first_idx 0 subseq_idx -1")
// __msg("mark_precise: frame0: regs=r0 stack= before 1: (87) r0 = -r0")
// __msg("mark_precise: frame0: regs=r0 stack= before 0: (b7) r0 = 1")
#[naked]
pub unsafe extern "C" fn bpf_neg_4() -> i32 {
    /*
     * lsm.s/socket_connect requires a return value within [-4095, 0].
     * Returning -1 is allowed
     */
    core::arch::asm!("r0 = 1;", "r0 = -r0;", "exit;", options(noreturn));
}

// SEC("lsm.s/socket_connect")
// __failure __msg("At program exit the register R0 has")
#[naked]
pub unsafe extern "C" fn bpf_neg_5() -> i32 {
    /*
     * lsm.s/socket_connect requires a return value within [-4095, 0].
     * Returning -10000 is not allowed.
     */
    core::arch::asm!("r0 = 10000;", "r0 = -r0;", "exit;", options(noreturn));
}

macro_rules! atomic_precision_fn {
    ($name:ident, $ret:ty, $insn_name:ident, $op:expr, [$($body:literal),+ $(,)?]) => {
        #[naked]
        pub unsafe extern "C" fn $name() -> $ret {
            core::arch::asm!(
                $($body,)+
                $insn_name = const $op,
                options(noreturn)
            );
        }
    };
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r2 stack= before 4: (bf) r3 = r10")
// __msg("mark_precise: frame0: regs=r2 stack= before 3: (db) r2 = atomic64_fetch_add((u64 *)(r10 -8), r2)")
// __msg("mark_precise: frame0: regs= stack=-8 before 2: (b7) r2 = 0")
// __msg("mark_precise: frame0: regs= stack=-8 before 1: (7b) *(u64 *)(r10 -8) = r1")
// __msg("mark_precise: frame0: regs=r1 stack= before 0: (b7) r1 = 8")
atomic_precision_fn!(bpf_atomic_fetch_add_precision, i32, fetch_add_insn, BPF_ATOMIC_OP(BPF_DW, BPF_ADD | BPF_FETCH, BPF_REG_10, BPF_REG_2, -8), [
    "r1 = 8;", "*(u64 *)(r10 - 8) = r1;", "r2 = 0;", ".8byte {fetch_add_insn};",
    "r3 = r10;", "r3 += r2;", "r0 = 0;", "exit;"
]);

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r2 stack= before 4: (bf) r3 = r10")
// __msg("mark_precise: frame0: regs=r2 stack= before 3: (db) r2 = atomic64_xchg((u64 *)(r10 -8), r2)")
// __msg("mark_precise: frame0: regs= stack=-8 before 2: (b7) r2 = 0")
// __msg("mark_precise: frame0: regs= stack=-8 before 1: (7b) *(u64 *)(r10 -8) = r1")
// __msg("mark_precise: frame0: regs=r1 stack= before 0: (b7) r1 = 8")
atomic_precision_fn!(bpf_atomic_xchg_precision, i32, xchg_insn, BPF_ATOMIC_OP(BPF_DW, BPF_XCHG, BPF_REG_10, BPF_REG_2, -8), [
    "r1 = 8;", "*(u64 *)(r10 - 8) = r1;", "r2 = 0;", ".8byte {xchg_insn};",
    "r3 = r10;", "r3 += r2;", "r0 = 0;", "exit;"
]);

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r2 stack= before 4: (bf) r3 = r10")
// __msg("mark_precise: frame0: regs=r2 stack= before 3: (db) r2 = atomic64_fetch_or((u64 *)(r10 -8), r2)")
// __msg("mark_precise: frame0: regs= stack=-8 before 2: (b7) r2 = 0")
// __msg("mark_precise: frame0: regs= stack=-8 before 1: (7b) *(u64 *)(r10 -8) = r1")
// __msg("mark_precise: frame0: regs=r1 stack= before 0: (b7) r1 = 8")
atomic_precision_fn!(bpf_atomic_fetch_or_precision, i32, fetch_or_insn, BPF_ATOMIC_OP(BPF_DW, BPF_OR | BPF_FETCH, BPF_REG_10, BPF_REG_2, -8), [
    "r1 = 8;", "*(u64 *)(r10 - 8) = r1;", "r2 = 0;", ".8byte {fetch_or_insn};",
    "r3 = r10;", "r3 += r2;", "r0 = 0;", "exit;"
]);

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r2 stack= before 4: (bf) r3 = r10")
// __msg("mark_precise: frame0: regs=r2 stack= before 3: (db) r2 = atomic64_fetch_and((u64 *)(r10 -8), r2)")
// __msg("mark_precise: frame0: regs= stack=-8 before 2: (b7) r2 = 0")
// __msg("mark_precise: frame0: regs= stack=-8 before 1: (7b) *(u64 *)(r10 -8) = r1")
// __msg("mark_precise: frame0: regs=r1 stack= before 0: (b7) r1 = 8")
atomic_precision_fn!(bpf_atomic_fetch_and_precision, i32, fetch_and_insn, BPF_ATOMIC_OP(BPF_DW, BPF_AND | BPF_FETCH, BPF_REG_10, BPF_REG_2, -8), [
    "r1 = 8;", "*(u64 *)(r10 - 8) = r1;", "r2 = 0;", ".8byte {fetch_and_insn};",
    "r3 = r10;", "r3 += r2;", "r0 = 0;", "exit;"
]);

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r2 stack= before 4: (bf) r3 = r10")
// __msg("mark_precise: frame0: regs=r2 stack= before 3: (db) r2 = atomic64_fetch_xor((u64 *)(r10 -8), r2)")
// __msg("mark_precise: frame0: regs= stack=-8 before 2: (b7) r2 = 0")
// __msg("mark_precise: frame0: regs= stack=-8 before 1: (7b) *(u64 *)(r10 -8) = r1")
// __msg("mark_precise: frame0: regs=r1 stack= before 0: (b7) r1 = 8")
atomic_precision_fn!(bpf_atomic_fetch_xor_precision, i32, fetch_xor_insn, BPF_ATOMIC_OP(BPF_DW, BPF_XOR | BPF_FETCH, BPF_REG_10, BPF_REG_2, -8), [
    "r1 = 8;", "*(u64 *)(r10 - 8) = r1;", "r2 = 0;", ".8byte {fetch_xor_insn};",
    "r3 = r10;", "r3 += r2;", "r0 = 0;", "exit;"
]);

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r0 stack= before 5: (bf) r3 = r10")
// __msg("mark_precise: frame0: regs=r0 stack= before 4: (db) r0 = atomic64_cmpxchg((u64 *)(r10 -8), r0, r2)")
// __msg("mark_precise: frame0: regs= stack=-8 before 3: (b7) r2 = 0")
// __msg("mark_precise: frame0: regs= stack=-8 before 2: (b7) r0 = 0")
// __msg("mark_precise: frame0: regs= stack=-8 before 1: (7b) *(u64 *)(r10 -8) = r1")
// __msg("mark_precise: frame0: regs=r1 stack= before 0: (b7) r1 = 8")
atomic_precision_fn!(bpf_atomic_cmpxchg_precision, i32, cmpxchg_insn, BPF_ATOMIC_OP(BPF_DW, BPF_CMPXCHG, BPF_REG_10, BPF_REG_2, -8), [
    "r1 = 8;", "*(u64 *)(r10 - 8) = r1;", "r0 = 0;", "r2 = 0;", ".8byte {cmpxchg_insn};",
    "r3 = r10;", "r3 += r0;", "r0 = 0;", "exit;"
]);

/* Regression test for dual precision: Both the fetched value (r2) and
 * a reread of the same stack slot (r3) are tracked for precision. After
 * the atomic operation, the stack slot is STACK_MISC. Thus, the ldx at
 * insn 4 does NOT set INSN_F_STACK_ACCESS. Precision for the stack slot
 * propagates solely through the atomic fetch's load side (insn 3).
 */
// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r2,r3 stack= before 4: (79) r3 = *(u64 *)(r10 -8)")
// __msg("mark_precise: frame0: regs=r2 stack= before 3: (db) r2 = atomic64_fetch_add((u64 *)(r10 -8), r2)")
// __msg("mark_precise: frame0: regs= stack=-8 before 2: (b7) r2 = 0")
// __msg("mark_precise: frame0: regs= stack=-8 before 1: (7b) *(u64 *)(r10 -8) = r1")
// __msg("mark_precise: frame0: regs=r1 stack= before 0: (b7) r1 = 8")
atomic_precision_fn!(bpf_atomic_fetch_add_dual_precision, i32, fetch_add_insn, BPF_ATOMIC_OP(BPF_DW, BPF_ADD | BPF_FETCH, BPF_REG_10, BPF_REG_2, -8), [
    "r1 = 8;", "*(u64 *)(r10 - 8) = r1;", "r2 = 0;", ".8byte {fetch_add_insn};",
    "r3 = *(u64 *)(r10 - 8);", "r4 = r2;", "r4 += r3;", "r4 &= 7;", "r5 = r10;",
    "r5 += r4;", "r0 = 0;", "exit;"
]);

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r0,r3 stack= before 5: (79) r3 = *(u64 *)(r10 -8)")
// __msg("mark_precise: frame0: regs=r0 stack= before 4: (db) r0 = atomic64_cmpxchg((u64 *)(r10 -8), r0, r2)")
// __msg("mark_precise: frame0: regs= stack=-8 before 3: (b7) r2 = 0")
// __msg("mark_precise: frame0: regs= stack=-8 before 2: (b7) r0 = 8")
// __msg("mark_precise: frame0: regs= stack=-8 before 1: (7b) *(u64 *)(r10 -8) = r1")
// __msg("mark_precise: frame0: regs=r1 stack= before 0: (b7) r1 = 8")
atomic_precision_fn!(bpf_atomic_cmpxchg_dual_precision, i32, cmpxchg_insn, BPF_ATOMIC_OP(BPF_DW, BPF_CMPXCHG, BPF_REG_10, BPF_REG_2, -8), [
    "r1 = 8;", "*(u64 *)(r10 - 8) = r1;", "r0 = 8;", "r2 = 0;", ".8byte {cmpxchg_insn};",
    "r3 = *(u64 *)(r10 - 8);", "r4 = r0;", "r4 += r3;", "r4 &= 7;", "r5 = r10;",
    "r5 += r4;", "r0 = 0;", "exit;"
]);

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r1 stack= before 10: (57) r1 &= 7")
// __msg("mark_precise: frame0: regs=r1 stack= before 9: (db) r1 = atomic64_fetch_add((u64 *)(r0 +0), r1)")
// __not_msg("falling back to forcing all scalars precise")
atomic_precision_fn!(bpf_atomic_fetch_add_map_precision, i32, fetch_add_insn, BPF_ATOMIC_OP(BPF_DW, BPF_ADD | BPF_FETCH, BPF_REG_0, BPF_REG_1, 0), [
    "r1 = 0;", "*(u64 *)(r10 - 8) = r1;", "r2 = r10;", "r2 += -8;",
    "r1 = {precision_map} ll;", "call {bpf_map_lookup_elem};", "if r0 == 0 goto 1f;",
    "r1 = 0;", ".8byte {fetch_add_insn};", "r1 &= 7;", "r2 = r10;", "r2 += r1;",
    "1: r0 = 0;", "exit;"
]);

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r0 stack= before 12: (57) r0 &= 7")
// __msg("mark_precise: frame0: regs=r0 stack= before 11: (db) r0 = atomic64_cmpxchg((u64 *)(r6 +0), r0, r1)")
// __not_msg("falling back to forcing all scalars precise")
atomic_precision_fn!(bpf_atomic_cmpxchg_map_precision, i32, cmpxchg_insn, BPF_ATOMIC_OP(BPF_DW, BPF_CMPXCHG, BPF_REG_6, BPF_REG_1, 0), [
    "r1 = 0;", "*(u64 *)(r10 - 8) = r1;", "r2 = r10;", "r2 += -8;",
    "r1 = {precision_map} ll;", "call {bpf_map_lookup_elem};", "if r0 == 0 goto 1f;",
    "r6 = r0;", "r0 = 0;", "r1 = 0;", ".8byte {cmpxchg_insn};", "r0 &= 7;",
    "r2 = r10;", "r2 += r0;", "1: r0 = 0;", "exit;"
]);

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r1 stack= before 10: (57) r1 &= 7")
// __msg("mark_precise: frame0: regs=r1 stack= before 9: (c3) r1 = atomic_fetch_add((u32 *)(r0 +0), r1)")
// __not_msg("falling back to forcing all scalars precise")
atomic_precision_fn!(bpf_atomic_fetch_add_32bit_precision, i32, fetch_add_insn, BPF_ATOMIC_OP(BPF_W, BPF_ADD | BPF_FETCH, BPF_REG_0, BPF_REG_1, 0), [
    "r1 = 0;", "*(u64 *)(r10 - 8) = r1;", "r2 = r10;", "r2 += -8;",
    "r1 = {precision_map} ll;", "call {bpf_map_lookup_elem};", "if r0 == 0 goto 1f;",
    "r1 = 0;", ".8byte {fetch_add_insn};", "r1 &= 7;", "r2 = r10;", "r2 += r1;",
    "1: r0 = 0;", "exit;"
]);

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r0 stack= before 12: (57) r0 &= 7")
// __msg("mark_precise: frame0: regs=r0 stack= before 11: (c3) r0 = atomic_cmpxchg((u32 *)(r6 +0), r0, r1)")
// __not_msg("falling back to forcing all scalars precise")
atomic_precision_fn!(bpf_atomic_cmpxchg_32bit_precision, i32, cmpxchg_insn, BPF_ATOMIC_OP(BPF_W, BPF_CMPXCHG, BPF_REG_6, BPF_REG_1, 0), [
    "r1 = 0;", "*(u64 *)(r10 - 8) = r1;", "r2 = r10;", "r2 += -8;",
    "r1 = {precision_map} ll;", "call {bpf_map_lookup_elem};", "if r0 == 0 goto 1f;",
    "r6 = r0;", "r0 = 0;", "r1 = 0;", ".8byte {cmpxchg_insn};", "r0 &= 7;",
    "r2 = r10;", "r2 += r0;", "1: r0 = 0;", "exit;"
]);

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

extern "Rust" {
    fn BPF_ATOMIC_OP(size: u32, op: u32, dst: u32, src: u32, off: i32) -> u64;
}

extern "Rust" {
    static BPF_DW: u32;
    static BPF_W: u32;
    static BPF_LOAD_ACQ: u32;
    static BPF_STORE_REL: u32;
    static BPF_ADD: u32;
    static BPF_FETCH: u32;
    static BPF_XCHG: u32;
    static BPF_OR: u32;
    static BPF_AND: u32;
    static BPF_XOR: u32;
    static BPF_CMPXCHG: u32;
    static BPF_REG_0: u32;
    static BPF_REG_1: u32;
    static BPF_REG_2: u32;
    static BPF_REG_6: u32;
    static BPF_REG_10: u32;
}
