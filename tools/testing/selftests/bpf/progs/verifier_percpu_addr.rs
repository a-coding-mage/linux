// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

// Original C condition: #if defined(__TARGET_ARCH_x86)
#[cfg(target_arch = "x86_64")]
#[unsafe(link_section = ".percpu")]
pub static mut percpu_data: i32 = 0;

/*
 * An ld_imm64 of a per-CPU map value is followed by a mov_percpu_addr that
 * reuses the same register, so check that the add resolves into the register
 * the address was loaded into, for every register.
 */
#[cfg(target_arch = "x86_64")]
#[unsafe(link_section = "raw_tp")]
// __description("per-CPU address resolution")
// __success
// __arch_x86_64
// __jited("	movabsq	$0x{{.*}}, %rax")
// __jited("	addq	%gs:{{.*}}, %rax")
// __jited("	movabsq	$0x{{.*}}, %rdi")
// __jited("	addq	%gs:{{.*}}, %rdi")
// __jited("	movabsq	$0x{{.*}}, %rsi")
// __jited("	addq	%gs:{{.*}}, %rsi")
// __jited("	movabsq	$0x{{.*}}, %rdx")
// __jited("	addq	%gs:{{.*}}, %rdx")
// __jited("	movabsq	$0x{{.*}}, %rcx")
// __jited("	addq	%gs:{{.*}}, %rcx")
// __jited("	movabsq	$0x{{.*}}, %r8")
// __jited("	addq	%gs:{{.*}}, %r8")
// __jited("	movabsq	$0x{{.*}}, %rbx")
// __jited("	addq	%gs:{{.*}}, %rbx")
// __jited("	movabsq	$0x{{.*}}, %r13")
// __jited("	addq	%gs:{{.*}}, %r13")
// __jited("	movabsq	$0x{{.*}}, %r14")
// __jited("	addq	%gs:{{.*}}, %r14")
// __jited("	movabsq	$0x{{.*}}, %r15")
// __jited("	addq	%gs:{{.*}}, %r15")
#[unsafe(naked)]
pub unsafe extern "C" fn percpu_addr() {
    core::arch::naked_asm!(
        "r0 = {percpu_data} ll",
        "r1 = {percpu_data} ll",
        "r2 = {percpu_data} ll",
        "r3 = {percpu_data} ll",
        "r4 = {percpu_data} ll",
        "r5 = {percpu_data} ll",
        "r6 = {percpu_data} ll",
        "r7 = {percpu_data} ll",
        "r8 = {percpu_data} ll",
        "r9 = {percpu_data} ll",
        "r0 = 0",
        "exit",
        percpu_data = sym percpu_data,
    );
}

#[cfg(not(target_arch = "x86_64"))]
#[unsafe(link_section = "raw_tp")]
// __description("percpu addr dummy")
// __success
pub extern "C" fn dummy_test() -> i32 {
    return 0;
}

#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";
