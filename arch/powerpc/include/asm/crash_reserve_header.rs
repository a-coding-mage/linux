/* SPDX-License-Identifier: GPL-2.0 */

/* crash kernel regions are Page size agliged */
// C dependency: PAGE_SIZE is supplied by the surrounding kernel build.
pub const CRASH_ALIGN: usize = PAGE_SIZE;

/* CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION is a build-time condition. */
#[cfg(CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION)]
#[inline]
pub fn arch_add_crash_res_to_iomem() -> bool {
    false
}

/* C macro alias: arch_add_crash_res_to_iomem arch_add_crash_res_to_iomem */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
