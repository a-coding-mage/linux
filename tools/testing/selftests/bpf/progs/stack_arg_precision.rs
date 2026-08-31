// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies translated as external items:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "../test_kmods/bpf_testmod_kfunc.h"
// #include "bpf_misc.h"

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_kfunc_call_stack_arg_mem(
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        mem: *mut core::ffi::c_void,
        mem__sz: u64,
    ) -> i32;
    fn bpf_get_prandom_u32() -> u32;
}

// Original C condition:
// #if (defined(__TARGET_ARCH_x86) || defined(__TARGET_ARCH_arm64)) && \
//      defined(__BPF_FEATURE_STACK_ARGUMENT)

/* Force kfunc extern BTF generation for inline asm call below.
 * Uses its own SEC so it's not included as a .text subprog.
 * The '?' prefix sets autoload=false so libbpf won't load it.
 */
#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __btf_kfunc_gen(ctx: *mut __sk_buff) -> i32 {
    let mut buf: [u8; 8] = [0; 8];

    unsafe {
        bpf_kfunc_call_stack_arg_mem(
            0,
            0,
            0,
            0,
            0,
            buf.as_mut_ptr() as *mut core::ffi::c_void,
            core::mem::size_of_val(&buf) as u64,
        )
    }
}

/*
 * Test precision backtracking across bpf-to-bpf call for kfunc stack arg.
 * subprog_call_mem_kfunc receives a size as incoming stack arg (arg6)
 * and forwards it as mem__sz (arg7) to bpf_kfunc_call_stack_arg_mem.
 */
#[inline(never)]
#[used]
static SUBPROG_CALL_MEM_KFUNC_USED: unsafe extern "C" fn(i64, i64, i64, i64, i64, i64) -> i64 =
    subprog_call_mem_kfunc;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn subprog_call_mem_kfunc(
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
    size: i64,
) -> i64 {
    unsafe {
        core::arch::asm!(
            "r1 = *(u64 *)(r11 + 8)",        /* r1 = incoming arg6 (size) */
            "r2 = 0x0807060504030201 ll",    /* r2 = buf contents */
            "*(u64 *)(r10 - 8) = r2",        /* store buf to stack */
            "r2 = r10",
            "r2 += -8",                      /* r2 = &buf */
            "*(u64 *)(r11 - 8) = r2",        /* outgoing arg6 = buf */
            "*(u64 *)(r11 - 16) = r1",       /* outgoing arg7 = size */
            "r1 = 1",
            "r2 = 2",
            "r3 = 3",
            "r4 = 4",
            "r5 = 5",
            "call {bpf_kfunc_call_stack_arg_mem}",
            "exit",
            bpf_kfunc_call_stack_arg_mem = sym bpf_kfunc_call_stack_arg_mem,
            options(noreturn)
        );
    }
}

// SEC("tc")
// __description("stack_arg: precision backtracking across bpf2bpf call for kfunc")
// __success
// __log_level(2)
// __flag(BPF_F_TEST_STATE_FREQ)
// __btf_func_path("btf__stack_arg_precision.bpf.o")
// __msg("mark_precise: frame1: last_idx 26 first_idx 13 subseq_idx -1")
// __msg("mark_precise: frame1: regs= stack= before 25: (b7) r5 = 5")
// __msg("mark_precise: frame1: regs= stack= before 24: (b7) r4 = 4")
// __msg("mark_precise: frame1: regs= stack= before 23: (b7) r3 = 3")
// __msg("mark_precise: frame1: regs= stack= before 22: (b7) r2 = 2")
// __msg("mark_precise: frame1: regs= stack= before 21: (b7) r1 = 1")
// __msg("mark_precise: frame1: regs= stack= before 20: (7b) *(u64 *)(r11 -16) = r1")
// __msg("mark_precise: frame1: regs=r1 stack= before 19: (7b) *(u64 *)(r11 -8) = r2")
// __msg("mark_precise: frame1: regs=r1 stack= before 18: (07) r2 += -8")
// __msg("mark_precise: frame1: regs=r1 stack= before 17: (bf) r2 = r10")
// __msg("mark_precise: frame1: regs=r1 stack= before 16: (7b) *(u64 *)(r10 -8) = r2")
// __msg("mark_precise: frame1: regs=r1 stack= before 14: (18) r2 = 0x807060504030201")
// __msg("mark_precise: frame1: regs=r1 stack= before 13: (79) r1 = *(u64 *)(r11 +8)")
// __msg("mark_precise: frame1: parent state regs= stack=:  frame1: R10=fp0")
// __msg("mark_precise: frame0: parent state regs= stack=:  R10=fp0")
// __msg("mark_precise: frame1: last_idx 11 first_idx 11 subseq_idx 13")
// __msg("mark_precise: frame1: regs= stack= before 11: (85) call pc+1")
// __msg("mark_precise: frame0: parent state regs= stack=:  R1=1 R2=2 R3=3 R4=4 R5=5 R10=fp0")
// __msg("mark_precise: frame0: last_idx 9 first_idx 7 subseq_idx 11")
// __msg("mark_precise: frame0: regs= stack= before 9: (05) goto pc+1")
// __msg("mark_precise: frame0: regs= stack= before 8: (7a) *(u64 *)(r11 -8) = 4")
// __msg("mark_precise: frame1: last_idx 26 first_idx 13 subseq_idx -1 ")
// __msg("mark_precise: frame1: regs= stack= before 25: (b7) r5 = 5")
// __msg("mark_precise: frame1: regs= stack= before 24: (b7) r4 = 4")
// __msg("mark_precise: frame1: regs= stack= before 23: (b7) r3 = 3")
// __msg("mark_precise: frame1: regs= stack= before 22: (b7) r2 = 2")
// __msg("mark_precise: frame1: regs= stack= before 21: (b7) r1 = 1")
// __msg("mark_precise: frame1: regs= stack= before 20: (7b) *(u64 *)(r11 -16) = r1")
// __msg("mark_precise: frame1: regs=r1 stack= before 19: (7b) *(u64 *)(r11 -8) = r2")
// __msg("mark_precise: frame1: regs=r1 stack= before 18: (07) r2 += -8")
// __msg("mark_precise: frame1: regs=r1 stack= before 17: (bf) r2 = r10")
// __msg("mark_precise: frame1: regs=r1 stack= before 16: (7b) *(u64 *)(r10 -8) = r2")
// __msg("mark_precise: frame1: regs=r1 stack= before 14: (18) r2 = 0x807060504030201")
// __msg("mark_precise: frame1: regs=r1 stack= before 13: (79) r1 = *(u64 *)(r11 +8)")
// __msg("mark_precise: frame1: parent state regs= stack=:  frame1: R10=fp0")
// __msg("mark_precise: frame0: parent state regs= stack=:  R10=fp0")
// __msg("mark_precise: frame1: last_idx 11 first_idx 11 subseq_idx 13 ")
// __msg("mark_precise: frame1: regs= stack= before 11: (85) call pc+1")
// __msg("mark_precise: frame0: parent state regs= stack=:  R1=1 R2=2 R3=3 R4=4 R5=5 R10=fp0")
// __msg("mark_precise: frame0: last_idx 10 first_idx 10 subseq_idx 11 ")
// __msg("mark_precise: frame0: regs= stack= before 10: (7a) *(u64 *)(r11 -8) = 6")
#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stack_arg_precision_bpf2bpf() {
    unsafe {
        core::arch::asm!(
            "call {bpf_get_prandom_u32}",
            "r6 = r0",
            "r1 = 1",
            "r2 = 2",
            "r3 = 3",
            "r4 = 4",
            "r5 = 5",
            "if r6 < 2 goto 0f",
            "*(u64 *)(r11 - 8) = 4",
            "goto 1f",
            "0:",
            "*(u64 *)(r11 - 8) = 6",
            "1:",
            "call {subprog_call_mem_kfunc}",
            "exit",
            bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
            subprog_call_mem_kfunc = sym subprog_call_mem_kfunc,
            options(noreturn)
        );
    }
}

// Original C #else branch for unsupported targets:
// SEC("socket")
// __description("stack_arg_precision: not supported, dummy test")
// __success
#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
pub extern "C" fn dummy_test() -> i32 {
    0
}

// #endif

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
