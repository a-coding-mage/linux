/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015-2016 Socionext Inc.
 *   Author: Masahiro Yamada <yamada.masahiro@socionext.com>
 */

/* CONFIG_CACHE_UNIPHIER is a build-time configuration condition. */

/* Linux errno.h: ENODEV (No such device). */
const ENODEV: i32 = 19;

#[cfg(feature = "CONFIG_CACHE_UNIPHIER")]
extern "C" {
    pub fn uniphier_cache_init() -> i32;
}

#[cfg(not(feature = "CONFIG_CACHE_UNIPHIER"))]
#[inline]
pub fn uniphier_cache_init() -> i32 {
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
