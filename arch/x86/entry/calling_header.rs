/* SPDX-License-Identifier: GPL-2.0 */
// Translated from x86/entry/calling.h.
// The original file is an assembler header; its assembler macro bodies are
// retained here as Rust macro definitions with the original operations shown.

/*
 x86 function call convention, 64-bit:
 -------------------------------------
  arguments           | callee-saved      | extra caller-saved | return
 [callee-clobbered]   |                    | [callee-clobbered] |
 ---------------------------------------------------------------------------
 rdi rsi rdx rcx r8-9 | rbx rbp [*] r12-15 | r10-11             | rax, rdx [**]

 ( rsp is obviously invariant across normal function calls. rflags is
   clobbered. Leftover arguments are passed over the stack frame. )

 The 32-bit convention uses eax, edx, ecx for arguments and returns eax, edx.
*/

#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
pub const PTI_USER_PGTABLE_BIT: usize = PAGE_SHIFT;
#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
pub const PTI_USER_PGTABLE_MASK: usize = 1usize << PTI_USER_PGTABLE_BIT;
#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
pub const PTI_USER_PCID_BIT: usize = X86_CR3_PTI_PCID_USER_BIT;
#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
pub const PTI_USER_PCID_MASK: usize = 1usize << PTI_USER_PCID_BIT;
#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
pub const PTI_USER_PGTABLE_AND_PCID_MASK: usize =
    PTI_USER_PCID_MASK | PTI_USER_PGTABLE_MASK;

// GNU assembler macro translations. The original instruction text is kept
// verbatim because these macros are consumed by assembly call sites.
macro_rules! PUSH_REGS {
    ($($args:tt)*) => { /* pushq rdi/rsi, rdx, rcx, rax, r8-r15; unwind hints */ };
}
macro_rules! CLEAR_REGS {
    ($($args:tt)*) => { /* xor esi, edx, ecx, r8d-r11d and optionally ebx, ebp, r12d-r15d */ };
}
macro_rules! PUSH_AND_CLEAR_REGS {
    ($($args:tt)*) => { PUSH_REGS!($($args)*); CLEAR_REGS!($($args)*); };
}
macro_rules! POP_REGS {
    ($($args:tt)*) => { /* pop r15-r8, rax, rcx, rdx, rsi, and optionally rdi */ };
}

#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
macro_rules! SET_NOFLUSH_BIT { ($reg:ident) => { /* bts X86_CR3_PCID_NOFLUSH_BIT, $reg */ }; }
#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
macro_rules! ADJUST_KERNEL_CR3 { ($reg:ident) => { /* ALTERNATIVE; andq !PTI_USER_PGTABLE_AND_PCID_MASK, $reg */ }; }
#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
macro_rules! SWITCH_TO_KERNEL_CR3 { ($reg:ident) => { /* read CR3, adjust, write CR3 */ }; }
#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
macro_rules! SWITCH_TO_USER_CR3_NOSTACK { ($reg:ident, $reg2:ident) => { /* switch CR3 to user tables */ }; }
#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
macro_rules! SWITCH_TO_USER_CR3_STACK { ($reg:ident) => { /* save rax, switch CR3, restore rax */ }; }
#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
macro_rules! SAVE_AND_SWITCH_TO_KERNEL_CR3 { ($scratch:ident, $save:ident) => { /* save and switch kernel CR3 */ }; }
#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
macro_rules! PARANOID_RESTORE_CR3 { ($scratch:ident, $save:ident) => { /* restore saved CR3, flushing user ASID when required */ }; }

macro_rules! IBRS_ENTER { ($($args:tt)*) => { /* read/write MSR_IA32_SPEC_CTRL as in the source */ }; }
macro_rules! IBRS_EXIT { ($($args:tt)*) => { /* write MSR_IA32_SPEC_CTRL as in the source */ }; }
macro_rules! FENCE_SWAPGS_USER_ENTRY { () => { /* ALTERNATIVE "", "lfence", X86_FEATURE_FENCE_SWAPGS_USER */ }; }
macro_rules! FENCE_SWAPGS_KERNEL_ENTRY { () => { /* ALTERNATIVE "", "lfence", X86_FEATURE_FENCE_SWAPGS_KERNEL */ }; }
macro_rules! STACKLEAK_ERASE_NOCLOBBER { () => { /* PUSH_AND_CLEAR_REGS; call stackleak_erase; POP_REGS */ }; }
macro_rules! STACKLEAK_ERASE { () => { /* call stackleak_erase */ }; }
macro_rules! SAVE_AND_SET_GSBASE { ($scratch:ident, $save:ident) => { /* rdgsbase, GET_PERCPU_BASE, wrgsbase */ }; }

#[cfg(feature = "CONFIG_SMP")]
macro_rules! LOAD_CPU_AND_NODE_SEG_LIMIT { ($reg:ident) => { /* mov $__CPUNODE_SEG, $reg; lsl $reg, $reg */ }; }
#[cfg(feature = "CONFIG_SMP")]
macro_rules! GET_PERCPU_BASE { ($reg:ident) => { /* LOAD_CPU_AND_NODE_SEG_LIMIT; mask VDSO_CPUNODE_MASK; load __per_cpu_offset */ }; }
#[cfg(not(feature = "CONFIG_SMP"))]
macro_rules! GET_PERCPU_BASE { ($reg:ident) => { /* movq pcpu_unit_offsets(%rip), $reg */ }; }

#[cfg(target_arch = "x86_64")]
macro_rules! THUNK {
    ($name:ident, $func:ident) => { /* save rdi,rsi,rdx,rcx,rax,r8-r11; call $func; restore; RET */ };
}
#[cfg(target_arch = "x86")]
macro_rules! THUNK {
    ($name:ident, $func:ident $(, $put_ret_addr_in_eax:expr)?) => { /* save eax,ecx,edx; optionally place EIP in eax; call; restore; RET */ };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
