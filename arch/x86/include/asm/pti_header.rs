/* SPDX-License-Identifier: GPL-2.0 */

/* Translation of the C header guard and non-assembler portion of pti.h. */

/* CONFIG_MITIGATION_PAGE_TABLE_ISOLATION */
#[cfg(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION)]
extern "C" {
    pub fn pti_init();
    pub fn pti_check_boottime_disable();
    pub fn pti_finalize();
}

#[cfg(not(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION))]
#[inline]
pub fn pti_check_boottime_disable() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
