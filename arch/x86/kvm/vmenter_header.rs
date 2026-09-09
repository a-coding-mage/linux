/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation:
// <asm/kvm_vcpu_regs.h>

// These constants correspond to the original BIT() preprocessor macros.
pub const KVM_ENTER_VMRESUME: usize = 1usize << 0;
pub const KVM_ENTER_SAVE_SPEC_CTRL: usize = 1usize << 1;
pub const KVM_ENTER_CLEAR_CPU_BUFFERS_FOR_MMIO: usize = 1usize << 2;

// The following items are assembler-only macros in the C header.  They are
// preserved here as inline-assembly source templates; their registers,
// labels, and external symbols are supplied by the assembler call site.
//
// RESTORE_GUEST_SPEC_CTRL_BODY guest_spec_ctrl:req, label:req:
//   SPEC_CTRL handling: compare the guest value with the host value and write
//   MSR_IA32_SPEC_CTRL when they differ.  On x86-64, load the low and high
//   halves into EAX and EDX; on 32-bit x86, compare both halves before wrmsr.
//   There must be no returns or indirect branches between this code and
//   vmentry, to avoid RSB underflow attacks.
//
// RESTORE_HOST_SPEC_CTRL_BODY guest_spec_ctrl:req, enter_flags:req, label:req:
//   Load the intercepted guest MSR value when KVM_ENTER_SAVE_SPEC_CTRL is set,
//   then restore the host MSR value when it differs.  With legacy IBRS, write
//   the IBRS value after a less-privileged predictor-mode transition even when
//   the values compare equal.
//
// LOAD_REGS src:req, regs_ofs:req, regs:vararg:
//   For each register, load (regs_ofs + reg_num * WORD_SIZE)(src) into it.
// STORE_REGS dst:req, regs_ofs:req, regs:vararg:
//   For each register, store it at (regs_ofs + reg_num * WORD_SIZE)(dst).
// POP_REGS dst:req, regs_ofs:req, regs:vararg:
//   For each register, pop into (regs_ofs + reg_num * WORD_SIZE)(dst).
// CLEAR_REGS regs:vararg:
//   Zero each listed register.

// Original build-time definition: BITS_PER_LONG / 8.
// BITS_PER_LONG is provided by the target architecture configuration.
#[cfg(target_pointer_width = "64")]
pub const WORD_SIZE: usize = 8;
#[cfg(target_pointer_width = "32")]
pub const WORD_SIZE: usize = 4;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
