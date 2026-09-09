/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied externally by <linux/cpumask.h>.

#[repr(C)]
pub struct arch_tlbflush_unmap_batch {
    /*
     * Each bit set is a CPU that potentially has a TLB entry for one of
     * the PFNs being flushed..
     */
    pub cpumask: cpumask,
    /*
     * Set if pages were unmapped from any MM, even one that does not
     * have active CPUs in its cpumask.
     */
    pub unmapped_pages: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
