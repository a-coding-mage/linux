/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/arch/arm/mach-at91/generic.h
 *
 *  Copyright (C) 2005 David Brownell
 */

// CONFIG_PM build-time condition preserved from the source header.
#[cfg(feature = "CONFIG_PM")]
extern "C" {
    pub fn at91rm9200_pm_init();
    pub fn at91sam9_pm_init();
    pub fn sam9x60_pm_init();
    pub fn sam9x7_pm_init();
    pub fn sama5_pm_init();
    pub fn sama5d2_pm_init();
    pub fn sama7_pm_init();
}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub fn at91rm9200_pm_init() {}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub fn at91sam9_pm_init() {}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub fn sam9x60_pm_init() {}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub fn sam9x7_pm_init() {}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub fn sama5_pm_init() {}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub fn sama5d2_pm_init() {}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub fn sama7_pm_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
