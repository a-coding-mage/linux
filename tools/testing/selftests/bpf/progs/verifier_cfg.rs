// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/cfg.c */

// C dependencies translated as external/build context:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[link_section = "socket"]
// __description("unreachable")
// __failure __msg("unreachable")
// __failure_unpriv
#[naked]
pub unsafe extern "C" fn unreachable() {
    core::arch::asm!(
        "exit",
        "exit",
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("unreachable2")
// __failure __msg("unreachable")
// __failure_unpriv
#[naked]
pub unsafe extern "C" fn unreachable2() {
    core::arch::asm!(
        "goto l0_0",
        "goto l0_0",
        "l0_0:",
        "exit",
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("out of range jump")
// __failure __msg("jump out of range")
// __failure_unpriv
#[naked]
pub unsafe extern "C" fn out_of_range_jump() {
    core::arch::asm!(
        "goto l0_1",
        "exit",
        "l0_1:",
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("out of range jump2")
// __failure __msg("jump out of range")
// __failure_unpriv
#[naked]
pub unsafe extern "C" fn out_of_range_jump2() {
    core::arch::asm!(
        "goto -2",
        "exit",
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("loop (back-edge)")
// __failure __msg("unreachable insn 1")
// __msg_unpriv("back-edge")
#[naked]
pub unsafe extern "C" fn loop_back_edge() {
    core::arch::asm!(
        "l0_2:",
        "goto l0_2",
        "exit",
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("loop2 (back-edge)")
// __failure __msg("unreachable insn 4")
// __msg_unpriv("back-edge")
#[naked]
pub unsafe extern "C" fn loop2_back_edge() {
    core::arch::asm!(
        "l0_3:",
        "r1 = r0",
        "r2 = r0",
        "r3 = r0",
        "goto l0_3",
        "exit",
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("conditional loop")
// __failure __msg("infinite loop detected")
// __msg_unpriv("back-edge")
#[naked]
pub unsafe extern "C" fn conditional_loop() {
    core::arch::asm!(
        "r0 = r1",
        "l0_4:",
        "r2 = r0",
        "r3 = r0",
        "if r1 == 0 goto l0_4",
        "exit",
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("conditional loop (2)")
// __success
// __failure_unpriv __msg_unpriv("back-edge from insn 10 to 11")
#[naked]
pub unsafe extern "C" fn conditional_loop2() {
    core::arch::asm!(
        "r9 = 2 ll",
        "r3 = 0x20 ll",
        "r4 = 0x35 ll",
        "r8 = r4",
        "goto l1_5",
        "l0_5:",
        "r9 -= r3",
        "r9 -= r4",
        "r9 -= r8",
        "l1_5:",
        "r8 += r4",
        "if r8 < 0x64 goto l0_5",
        "r0 = r9",
        "exit",
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("unconditional loop after conditional jump")
// __failure __msg("infinite loop detected")
// __failure_unpriv __msg_unpriv("back-edge from insn 3 to 2")
#[naked]
pub unsafe extern "C" fn uncond_loop_after_cond_jmp() {
    core::arch::asm!(
        "r0 = 0",
        "if r0 > 0 goto l1_6",
        "l0_6:",
        "r0 = 1",
        "goto l0_6",
        "l1_6:",
        "exit",
        options(noreturn)
    );
}

// __naked __noinline __used
#[naked]
#[inline(never)]
#[used]
unsafe extern "C" fn never_ending_subprog() -> u64 {
    core::arch::asm!(
        "r0 = r1",
        "goto -1",
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("unconditional loop after conditional jump")
/* infinite loop is detected *after* check_cfg() */
// __failure __msg("infinite loop detected")
#[naked]
pub unsafe extern "C" fn uncond_loop_in_subprog_after_cond_jmp() {
    core::arch::asm!(
        "r0 = 0",
        "if r0 > 0 goto l1_7",
        "l0_7:",
        "r0 += 1",
        "call never_ending_subprog",
        "l1_7:",
        "exit",
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
