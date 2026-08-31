// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/unpriv.c */

// C dependencies removed from executable Rust:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::arch::asm;

unsafe extern "C" {
    static __imm_0: i32;
    static sample_period: usize;
}

// SEC("perf_event")
// __description("unpriv: spill/fill of different pointers ldx")
// __failure __msg("same insn cannot be used with different pointers")
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fill_of_different_pointers_ldx() {
    unsafe {
        asm!(
            "r6 = r10",
            "r6 += {imm_0}",
            "if r1 == 0 goto 0f",
            "r2 = r10",
            "r2 += {__imm_0}",
            "*(u64*)(r6 + 0) = r2",
            "0:",
            "if r1 != 0 goto 1f",
            "*(u64*)(r6 + 0) = r1",
            "1:",
            "r1 = *(u64*)(r6 + 0)",
            "r1 = *(u64*)(r1 + {sample_period})",
            "r0 = 0",
            "exit",
            imm_0 = const -8,
            __imm_0 = sym __imm_0,
            sample_period = sym sample_period,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
