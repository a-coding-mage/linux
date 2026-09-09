/* SPDX-License-Identifier: GPL-2.0 */
/*
 * NUMA support for s390
 *
 * Declare the NUMA core code structures and functions.
 *
 * Copyright IBM Corp. 2015
 */

/* Corresponds to the C header guard: _ASM_S390_NUMA_H. */

/* Corresponds to CONFIG_NUMA. */
#[cfg(feature = "CONFIG_NUMA")]
extern "C" {
    pub fn numa_setup();
}

/* CONFIG_NUMA disabled: static inline void numa_setup(void) { } */
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline]
pub fn numa_setup() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
