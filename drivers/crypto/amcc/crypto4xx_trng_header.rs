/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * AMCC SoC PPC4xx Crypto Driver
 *
 * Copyright (c) 2008 Applied Micro Circuits Corporation.
 * All rights reserved. James Hsiao <jhsiao@amcc.com>
 *
 * This file defines the security context
 * associate format.
 */

// CONFIG_HW_RANDOM_PPC4XX selects the hardware-random-number-generator support.
// `crypto4xx_core_device` is supplied by the surrounding driver dependencies.
#[cfg(feature = "CONFIG_HW_RANDOM_PPC4XX")]
extern "C" {
    pub fn ppc4xx_trng_probe(core_dev: *mut crypto4xx_core_device);
    pub fn ppc4xx_trng_remove(core_dev: *mut crypto4xx_core_device);
}

#[cfg(not(feature = "CONFIG_HW_RANDOM_PPC4XX"))]
#[inline]
pub unsafe fn ppc4xx_trng_probe(_dev: *mut crypto4xx_core_device) {}

#[cfg(not(feature = "CONFIG_HW_RANDOM_PPC4XX"))]
#[inline]
pub unsafe fn ppc4xx_trng_remove(_dev: *mut crypto4xx_core_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
