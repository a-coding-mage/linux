// SPDX-License-Identifier: GPL-2.0

// CONFIG_MITIGATION_PAGE_TABLE_ISOLATION: the C header includes <asm/pti.h>
// here. Its declarations are supplied by that external dependency.

#[cfg(not(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION))]
#[inline]
pub fn pti_init() {}

#[cfg(not(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION))]
#[inline]
pub fn pti_finalize() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
