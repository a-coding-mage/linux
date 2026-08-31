// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/loops1.c */

// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

unsafe extern "C" {
    fn bpf_get_prandom_u32() -> u32;
}

// SEC("xdp")
// __description("bounded loop, count to 4")
// __success __retval(4)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bounded_loop_count_to_4() {
    core::arch::asm!(
        "r0 = 0",
        "0:",
        "r0 += 1",
        "if r0 < 4 goto 0b",
        "exit",
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("bounded loop, count to 20")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bounded_loop_count_to_20() {
    core::arch::asm!(
        "r0 = 0",
        "0:",
        "r0 += 3",
        "if r0 < 20 goto 0b",
        "exit",
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("bounded loop, count from positive unknown to 4")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn from_positive_unknown_to_4() {
    core::arch::asm!(
        "call {bpf_get_prandom_u32}",
        "if r0 s< 0 goto 1f",
        "0:",
        "r0 += 1",
        "if r0 < 4 goto 0b",
        "1:",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("bounded loop, count from totally unknown to 4")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn from_totally_unknown_to_4() {
    core::arch::asm!(
        "call {bpf_get_prandom_u32}",
        "0:",
        "r0 += 1",
        "if r0 < 4 goto 0b",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("bounded loop, count to 4 with equality")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn count_to_4_with_equality() {
    core::arch::asm!(
        "r0 = 0",
        "0:",
        "r0 += 1",
        "if r0 != 4 goto 0b",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("bounded loop, start in the middle")
// __success
// __failure_unpriv __msg_unpriv("back-edge")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loop_start_in_the_middle() {
    core::arch::asm!(
        "r0 = 0",
        "goto 1f",
        "0:",
        "r0 += 1",
        "1:",
        "if r0 < 4 goto 0b",
        "exit",
        options(noreturn)
    );
}

// SEC("xdp")
// __description("bounded loop containing a forward jump")
// __success __retval(4)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loop_containing_a_forward_jump() {
    core::arch::asm!(
        "r0 = 0",
        "0:",
        "r0 += 1",
        "if r0 == r0 goto 1f",
        "1:",
        "if r0 < 4 goto 0b",
        "exit",
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("bounded loop that jumps out rather than in")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jumps_out_rather_than_in() {
    core::arch::asm!(
        "r6 = 0",
        "0:",
        "r6 += 1",
        "if r6 > 10000 goto 1f",
        "call {bpf_get_prandom_u32}",
        "goto 0b",
        "1:",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("infinite loop after a conditional jump")
// __failure __msg("program is too large")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loop_after_a_conditional_jump() {
    core::arch::asm!(
        "r0 = 5",
        "if r0 < 4 goto 1f",
        "0:",
        "r0 += 1",
        "goto 0b",
        "1:",
        "exit",
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("bounded recursion")
// __failure
// __msg("recursive call from")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bounded_recursion() {
    core::arch::asm!(
        "r1 = 0",
        "call bounded_recursion__1",
        "exit",
        options(noreturn)
    );
}

#[unsafe(no_mangle)]
unsafe extern "C" fn bounded_recursion__1() {
    core::arch::asm!(
        "r1 += 1",
        "r0 = r1",
        "if r1 < 4 goto 0f",
        "exit",
        "0:",
        "call bounded_recursion__1",
        "exit",
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("infinite loop in two jumps")
// __failure __msg("loop detected")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn infinite_loop_in_two_jumps() {
    core::arch::asm!(
        "r0 = 0",
        "0:",
        "goto 1f",
        "1:",
        "if r0 < 4 goto 0b",
        "exit",
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("infinite loop: three-jump trick")
// __failure __msg("loop detected")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn infinite_loop_three_jump_trick() {
    core::arch::asm!(
        "r0 = 0",
        "0:",
        "r0 += 1",
        "r0 &= 1",
        "if r0 < 2 goto 1f",
        "exit",
        "1:",
        "r0 += 1",
        "r0 &= 1",
        "if r0 < 2 goto 2f",
        "exit",
        "2:",
        "r0 += 1",
        "r0 &= 1",
        "if r0 < 2 goto 0b",
        "exit",
        options(noreturn)
    );
}

// SEC("xdp")
// __description("not-taken loop with back jump to 1st insn")
// __success __retval(123)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn back_jump_to_1st_insn_1() {
    core::arch::asm!(
        "0:",
        "r0 = 123",
        "if r0 == 4 goto 0b",
        "exit",
        options(noreturn)
    );
}

// SEC("xdp")
// __description("taken loop with back jump to 1st insn")
// __success __retval(55)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn back_jump_to_1st_insn_2() {
    core::arch::asm!(
        "r1 = 10",
        "r2 = 0",
        "call back_jump_to_1st_insn_2__1",
        "exit",
        options(noreturn)
    );
}

#[unsafe(no_mangle)]
unsafe extern "C" fn back_jump_to_1st_insn_2__1() {
    core::arch::asm!(
        "0:",
        "r2 += r1",
        "r1 -= 1",
        "if r1 != 0 goto 0b",
        "r0 = r2",
        "exit",
        options(noreturn)
    );
}

// SEC("xdp")
// __description("taken loop with back jump to 1st insn, 2")
// __success __retval(55)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jump_to_1st_insn_2() {
    core::arch::asm!(
        "r1 = 10",
        "r2 = 0",
        "call jump_to_1st_insn_2__1",
        "exit",
        options(noreturn)
    );
}

#[unsafe(no_mangle)]
unsafe extern "C" fn jump_to_1st_insn_2__1() {
    core::arch::asm!(
        "0:",
        "r2 += r1",
        "r1 -= 1",
        "if w1 != 0 goto 0b",
        "r0 = r2",
        "exit",
        options(noreturn)
    );
}

// SEC("xdp")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn not_an_inifinite_loop() {
    core::arch::asm!(
        "call {bpf_get_prandom_u32}",
        "r0 &= 0xff",
        "*(u64 *)(r10 - 8) = r0",
        "r0 = 0",
        "0:",
        "r0 = *(u64 *)(r10 - 8)",
        "if r0 > 10 goto 1f",
        "r0 += 1",
        "*(u64 *)(r10 - 8) = r0",
        "r0 = 0",
        "goto 0b",
        "1:",
        "r0 = 0",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        options(noreturn)
    );
}

/*
 * This test case triggered a bug in verifier.c:maybe_exit_scc().
 * Speculative execution path reaches stack access instruction,
 * stops and triggers maybe_exit_scc() w/o accompanying maybe_enter_scc() call.
 */
// SEC("socket")
// __arch_x86_64
// __caps_unpriv(CAP_BPF)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn maybe_exit_scc_bug1() {
    core::arch::asm!(
        "r0 = 100",
        "0:",
        // Speculative execution path reaches and stops here.
        "*(u64 *)(r10 - 512) = r0",
        // Condition is always false, but verifier speculatively executes the true branch.
        "if r0 <= 0x0 goto 0b",
        "exit",
        options(noreturn)
    );
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";
