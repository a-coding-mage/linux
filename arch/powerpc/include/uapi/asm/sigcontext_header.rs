/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Translated from asm/powerpc/sigcontext.h. */

use core::ffi::{c_int, c_long, c_ulong};

/* The C header includes compiler, ptrace, and (on powerpc64) ELF definitions. */

#[repr(C)]
pub struct sigcontext {
    pub _unused: [c_ulong; 4],
    pub signal: c_int,
    #[cfg(target_arch = "powerpc64")]
    pub _pad0: c_int,
    pub handler: c_ulong,
    pub oldmask: c_ulong,
    /* __KERNEL__ selects user_pt_regs; userspace selects pt_regs. */
    #[cfg(feature = "kernel")]
    pub regs: *mut user_pt_regs,
    #[cfg(not(feature = "kernel"))]
    pub regs: *mut pt_regs,
    #[cfg(target_arch = "powerpc64")]
    pub gp_regs: elf_gregset_t,
    #[cfg(target_arch = "powerpc64")]
    pub fp_regs: elf_fpregset_t,
    #[cfg(target_arch = "powerpc64")]
    pub v_regs: *mut elf_vrreg_t,
    #[cfg(target_arch = "powerpc64")]
    pub vmx_reserve: [c_long; ELF_NVRREG + ELF_NVRREG + 1 + 32],
}

/*
 * To maintain compatibility with current implementations the sigcontext is
 * extended by appending a pointer (v_regs) to a quadword type (elf_vrreg_t)
 * followed by an unstructured (vmx_reserve) field of 101 doublewords. This
 * allows the array of vector registers to be quadword aligned independent of
 * the alignment of the containing sigcontext or ucontext. It is the
 * responsibility of the code setting the sigcontext to set this pointer to
 * either NULL (if this processor does not support the VMX feature) or the
 * address of the first quadword within the allocated (vmx_reserve) area.
 *
 * The pointer (v_regs) of vector type (elf_vrreg_t) is type compatible with
 * an array of 34 quadword entries (elf_vrregset_t). The entries with indexes
 * 0-31 contain the corresponding vector registers. The entry with index 32
 * contains the vscr as the last word (offset 12) within the quadword. This
 * allows the vscr to be stored as either a quadword (since it must be copied
 * via a vector register to/from storage) or as a word. The entry with index
 * 33 contains the vrsave as the first word (offset 0) within the quadword.
 *
 * Part of the VSX data is stored here also by extending vmx_restore by an
 * additional 32 double words. Architecturally the layout of the VSR
 * registers and how they overlap on top of the legacy FPR and VR registers
 * is shown in the original C header.
 *
 * FPR/VSR 0-31 doubleword 0 is stored in fp_regs, and VMX/VSR 32-63 is
 * stored at the start of vmx_reserve. vmx_reserve is extended for backwards
 * compatility to store VSR 0-31 doubleword 1 after the VMX registers and
 * vscr/vrsave.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
