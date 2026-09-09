/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2012-2014 Broadcom Corporation */

/* Corresponds to CONFIG_ARCH_BCM_MOBILE_L2_CACHE. */
#[cfg(CONFIG_ARCH_BCM_MOBILE_L2_CACHE)]
unsafe extern "C" {
    pub fn kona_l2_cache_init();
}

#[cfg(not(CONFIG_ARCH_BCM_MOBILE_L2_CACHE))]
#[inline]
pub fn kona_l2_cache_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
