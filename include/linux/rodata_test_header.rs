/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rodata_test.h: functional test for mark_rodata_ro function
 *
 * (C) Copyright 2008 Intel Corporation
 * Author: Arjan van de Ven <arjan@linux.intel.com>
 */

/* Corresponds to the C build-time condition CONFIG_DEBUG_RODATA_TEST. */
#[cfg(feature = "CONFIG_DEBUG_RODATA_TEST")]
extern "C" {
    pub fn rodata_test();
}

#[cfg(not(feature = "CONFIG_DEBUG_RODATA_TEST"))]
#[inline]
pub fn rodata_test() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
