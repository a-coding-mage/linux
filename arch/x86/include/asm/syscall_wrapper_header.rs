/* SPDX-License-Identifier: GPL-2.0 */
/*
 * syscall_wrapper.h - x86 specific wrappers to syscall definitions
 *
 * Rust translation of the source header.  The C preprocessor interfaces below
 * are retained as declarative macro documentation because their token-pasting,
 * variadic expansion, and build-time configuration semantics are supplied by
 * the surrounding kernel translation unit.
 */

use core::ffi::c_uint;

/* Supplied by the translated asm/ptrace dependency. */
pub enum pt_regs {}

extern "C" {
    pub fn __x64_sys_ni_syscall(regs: *const pt_regs) -> c_long;
    pub fn __ia32_sys_ni_syscall(regs: *const pt_regs) -> c_long;
    pub fn __x64_sys_getcpu(regs: *const pt_regs) -> c_long;
    pub fn __x64_sys_gettimeofday(regs: *const pt_regs) -> c_long;
    pub fn __x64_sys_time(regs: *const pt_regs) -> c_long;
}

pub type c_long = isize;

/*
 * Instead of the generic __SYSCALL_DEFINEx() definition, the x86 version takes
 * struct pt_regs *regs as the only argument of the syscall stub(s) named as:
 * __x64_sys_*(), __ia32_sys_*(), __ia32_compat_sys_*(), __x64_compat_sys_*().
 * Registers are decoded according to the ABI: 64-bit RDI, RSI, RDX, R10, R8,
 * R9; 32-bit EBX, ECX, EDX, ESI, EDI, EBP.  The stubs pass decoded arguments
 * through __se_sys_*() and then __do_sys_*(), with inline wrappers where
 * appropriate.  The source assembly example and its register-clearing intent
 * are preserved by the original comments and this semantic summary.
 */

/* C macro mappings retained verbatim in semantic form:
 * SC_X86_64_REGS_TO_ARGS(x, ...) => __MAP(x, __SC_ARGS, ,,
 *   regs->di,,regs->si,,regs->dx,,regs->r10,,regs->r8,,regs->r9)
 * SYSCALL_PT_ARG6..ARG1 map arguments in order to regs->bx, cx, dx, si, di, bp.
 * __SC_COMPAT_CAST(t, a) casts a to unsigned int with the signedness selected by
 * __TYPE_IS_L(t); SC_IA32_REGS_TO_ARGS uses that mapping.
 * __SYS_STUB0/__SYS_STUBx declare ABI stubs, allow error injection, and alias
 * or call the corresponding __do_/__se_ functions.
 * __COND_SYSCALL declares weak ABI stubs returning sys_ni_syscall().
 * __X64_* are enabled by CONFIG_X86_64; __IA32_* by CONFIG_X86_32 or
 * CONFIG_IA32_EMULATION; IA32 compat by CONFIG_IA32_EMULATION; X32 compat by
 * CONFIG_X86_X32_ABI.
 * COMPAT_SYSCALL_DEFINE0/DEFINEx and COND_SYSCALL_COMPAT are enabled by
 * CONFIG_COMPAT and generate both IA32 and X32 wrappers.
 * __SYSCALL_DEFINEx generates __se_sys*, __do_sys*, x64 and ia32 stubs,
 * argument casts/tests/protection, and preserves return ordering.
 * SYSCALL_DEFINE0 emits SYSCALL_METADATA and zero-argument stubs; COND_SYSCALL
 * emits conditional x64 and ia32 stubs.
 */

/*
 * For VSYSCALLS, these syscalls use the pt_regs-based calling convention for
 * in-kernel use; declarations are provided above.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
