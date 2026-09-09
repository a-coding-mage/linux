/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8962.h  --  WM8962 Soc Audio driver platform data
 */

pub const WM8962_MAX_GPIO: usize = 6;

/* Use to set GPIO default values to zero */
pub const WM8962_GPIO_SET: u32 = 0x10000;

pub const WM8962_GPIO_FN_CLKOUT: u32 = 0;
pub const WM8962_GPIO_FN_LOGIC: u32 = 1;
pub const WM8962_GPIO_FN_SDOUT: u32 = 2;
pub const WM8962_GPIO_FN_IRQ: u32 = 3;
pub const WM8962_GPIO_FN_THERMAL: u32 = 4;
pub const WM8962_GPIO_FN_PLL2_LOCK: u32 = 6;
pub const WM8962_GPIO_FN_PLL3_LOCK: u32 = 7;
pub const WM8962_GPIO_FN_FLL_LOCK: u32 = 9;
pub const WM8962_GPIO_FN_DRC_ACT: u32 = 10;
pub const WM8962_GPIO_FN_WSEQ_DONE: u32 = 11;
pub const WM8962_GPIO_FN_ALC_NG_ACT: u32 = 12;
pub const WM8962_GPIO_FN_ALC_PEAK_LIMIT: u32 = 13;
pub const WM8962_GPIO_FN_ALC_SATURATION: u32 = 14;
pub const WM8962_GPIO_FN_ALC_LEVEL_THR: u32 = 15;
pub const WM8962_GPIO_FN_ALC_LEVEL_LOCK: u32 = 16;
pub const WM8962_GPIO_FN_FIFO_ERR: u32 = 17;
pub const WM8962_GPIO_FN_OPCLK: u32 = 18;
pub const WM8962_GPIO_FN_DMICCLK: u32 = 19;
pub const WM8962_GPIO_FN_DMICDAT: u32 = 20;
pub const WM8962_GPIO_FN_MICD: u32 = 21;
pub const WM8962_GPIO_FN_MICSCD: u32 = 22;

#[repr(C)]
pub struct wm8962_pdata {
    pub mclk: *mut clk,
    pub gpio_base: i32,
    pub gpio_init: [u32; WM8962_MAX_GPIO],

    /* Setup for microphone detection, raw value to be written to
     * R48(0x30) - only microphone related bits will be updated.
     * Detection may be enabled here for use with signals brought
     * out on the GPIOs. */
    pub mic_cfg: u32,

    pub irq_active_low: bool,

    pub spk_mono: bool, /* Speaker outputs tied together as mono */

    /**
     * This flag should be set if one or both IN4 inputs is wired
     * in a DC measurement configuration.
     */
    pub in4_dc_measure: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
