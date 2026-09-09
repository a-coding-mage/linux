/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2014 Samsung Electronics Co., Ltd.
 * Sylwester Nawrocki <s.nawrocki@samsung.com>
 */

// Dependency: linux/types.h

pub struct device_node;

// Equivalent to: #if defined(CONFIG_OF) && defined(CONFIG_COMMON_CLK)
#[cfg(all(feature = "CONFIG_OF", feature = "CONFIG_COMMON_CLK"))]
unsafe extern "C" {
    pub fn of_clk_set_defaults(node: *mut device_node, clk_supplier: bool) -> i32;
}

// Equivalent to the fallback when CONFIG_OF or CONFIG_COMMON_CLK is absent.
#[cfg(not(all(feature = "CONFIG_OF", feature = "CONFIG_COMMON_CLK")))]
#[inline]
pub unsafe fn of_clk_set_defaults(_node: *mut device_node, _clk_supplier: bool) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
