/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Power Management driver for Marvell Kirkwood SoCs
 *
 * Copyright (C) 2013 Ezequiel Garcia <ezequiel@free-electrons.com>
 * Copyright (C) 2010 Simon Guinot <sguinot@lacie.com>
 */

/* Equivalent of the C CONFIG_PM conditional. */
#[cfg(feature = "CONFIG_PM")]
unsafe extern "C" {
    pub fn kirkwood_pm_init();
}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub fn kirkwood_pm_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
