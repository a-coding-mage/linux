/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2022 ARM Ltd.
 */

// C dependencies: <linux/bug.h>, <linux/percpu.h>, and <asm/fixmap.h>.

use core::ffi::c_char;

extern "C" {
    pub static mut vectors: [c_char; 0];
    pub static mut tramp_vectors: [c_char; 0];
    pub static mut __bp_harden_el1_vectors: [c_char; 0];
}

/*
 * Note: the order of this enum corresponds to two arrays in entry.S:
 * tramp_vecs and __bp_harden_el1_vectors. By default the canonical
 * 'full fat' vectors are used directly.
 */
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum arm64_bp_harden_el1_vectors {
    #[cfg(feature = "CONFIG_MITIGATE_SPECTRE_BRANCH_HISTORY")]
    /*
     * Perform the BHB loop mitigation, before branching to the canonical
     * vectors.
     */
    EL1_VECTOR_BHB_LOOP,

    #[cfg(feature = "CONFIG_MITIGATE_SPECTRE_BRANCH_HISTORY")]
    /*
     * Make the SMC call for firmware mitigation, before branching to the
     * canonical vectors.
     */
    EL1_VECTOR_BHB_FW,

    #[cfg(feature = "CONFIG_MITIGATE_SPECTRE_BRANCH_HISTORY")]
    /*
     * Use the ClearBHB instruction, before branching to the canonical
     * vectors.
     */
    EL1_VECTOR_BHB_CLEAR_INSN,

    /*
     * Remap the kernel before branching to the canonical vectors.
     */
    EL1_VECTOR_KPTI,
}

#[cfg(not(feature = "CONFIG_MITIGATE_SPECTRE_BRANCH_HISTORY"))]
pub const EL1_VECTOR_BHB_LOOP: isize = -1;
#[cfg(not(feature = "CONFIG_MITIGATE_SPECTRE_BRANCH_HISTORY"))]
pub const EL1_VECTOR_BHB_FW: isize = -1;
#[cfg(not(feature = "CONFIG_MITIGATE_SPECTRE_BRANCH_HISTORY"))]
pub const EL1_VECTOR_BHB_CLEAR_INSN: isize = -1;

/* The vectors to use on return from EL0. e.g. to remap the kernel */
// DECLARE_PER_CPU_READ_MOSTLY(const char *, this_cpu_vector);
extern "C" {
    pub static mut this_cpu_vector: *const c_char;
}

#[cfg(not(feature = "CONFIG_UNMAP_KERNEL_AT_EL0"))]
pub const TRAMP_VALIAS: usize = 0;

// These symbols/constants are supplied by the kernel environment.
extern "C" {
    fn cpus_have_cap(cap: u32) -> bool;
    fn WARN_ON_ONCE(condition: bool) -> bool;
}

// ARM64_UNMAP_KERNEL_AT_EL0 and SZ_2K are supplied by <asm/fixmap.h>.
pub unsafe fn arm64_get_bp_hardening_vector(
    slot: arm64_bp_harden_el1_vectors,
) -> *const c_char {
    if cpus_have_cap(ARM64_UNMAP_KERNEL_AT_EL0) {
        return (TRAMP_VALIAS + SZ_2K * slot as usize) as *const c_char;
    }

    let _ = WARN_ON_ONCE(slot == arm64_bp_harden_el1_vectors::EL1_VECTOR_KPTI);

    __bp_harden_el1_vectors.as_ptr().add(SZ_2K * slot as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
