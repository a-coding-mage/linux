/* SPDX-License-Identifier: GPL-2.0 */
/* mt9t112 Camera
 *
 * Copyright (C) 2009 Renesas Solutions Corp.
 * Kuninori Morimoto <morimoto.kuninori@renesas.com>
 */

// Translated from the C header; the original include guard is omitted.

#[repr(C)]
pub struct mt9t112_pll_divider {
    pub m: u8,
    pub n: u8,
    pub p1: u8,
    pub p2: u8,
    pub p3: u8,
    pub p4: u8,
    pub p5: u8,
    pub p6: u8,
    pub p7: u8,
}

/**
 * struct mt9t112_platform_data - mt9t112 driver interface
 * @flags:                    Sensor media bus configuration.
 * @divider:                  Sensor PLL configuration
 */
#[repr(C)]
pub struct mt9t112_platform_data {
    pub flags: u32,
    pub divider: mt9t112_pll_divider,
}

pub const MT9T112_FLAG_PCLK_RISING_EDGE: u32 = 1u32 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
