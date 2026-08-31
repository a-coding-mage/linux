// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/const_or.c */

// C dependencies removed from executable Rust:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

unsafe extern "C" {
    fn bpf_probe_read_kernel() -> i32;
}

// SEC("tracepoint")
// __description("constant register |= constant should keep constant type")
// __success
#[unsafe(no_mangle)]
#[unsafe(link_section = "tracepoint")]
pub unsafe extern "C" fn constant_should_keep_constant_type() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -48",
        "r2 = 34",
        "r2 |= 13",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("constant register |= constant should not bypass stack boundary checks")
// __failure __msg("invalid write to stack R1 off=-48 size=58")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tracepoint")]
pub unsafe extern "C" fn not_bypass_stack_boundary_checks_1() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -48",
        "r2 = 34",
        "r2 |= 24",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("constant register |= constant register should keep constant type")
// __success
#[unsafe(no_mangle)]
#[unsafe(link_section = "tracepoint")]
pub unsafe extern "C" fn register_should_keep_constant_type() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -48",
        "r2 = 34",
        "r4 = 13",
        "r2 |= r4",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("constant register |= constant register should not bypass stack boundary checks")
// __failure __msg("invalid write to stack R1 off=-48 size=58")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tracepoint")]
pub unsafe extern "C" fn not_bypass_stack_boundary_checks_2() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -48",
        "r2 = 34",
        "r4 = 24",
        "r2 |= r4",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";
