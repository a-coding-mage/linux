/* SPDX-License-Identifier: GPL-2.0 */
// Header guard: _ARCH_ARM64_TLBBATCH_H

#[repr(C)]
pub struct arch_tlbflush_unmap_batch {
    /*
     * For arm64, HW can do TLB shootdown, so we don't need to record a
     * cpumask for sending IPIs.
     */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
