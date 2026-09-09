/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/wm8903.h -- Platform data for WM8903
 *
 * Copyright 2010 Wolfson Microelectronics. PLC.
 */

/* Used to enable configuration of a GPIO to all zeros; a gpio_cfg value of
 * zero in platform data means "don't touch this pin". */
pub const WM8903_GPIO_CONFIG_ZERO: u16 = 0x8000;

/* R6 (0x06) - Mic Bias Control 0 */
pub const WM8903_MICDET_THR_MASK: u16 = 0x0030;
pub const WM8903_MICDET_THR_SHIFT: u32 = 4;
pub const WM8903_MICDET_THR_WIDTH: u32 = 2;
pub const WM8903_MICSHORT_THR_MASK: u16 = 0x000C;
pub const WM8903_MICSHORT_THR_SHIFT: u32 = 2;
pub const WM8903_MICSHORT_THR_WIDTH: u32 = 2;
pub const WM8903_MICDET_ENA: u16 = 0x0002;
pub const WM8903_MICDET_ENA_MASK: u16 = 0x0002;
pub const WM8903_MICDET_ENA_SHIFT: u32 = 1;
pub const WM8903_MICDET_ENA_WIDTH: u32 = 1;
pub const WM8903_MICBIAS_ENA: u16 = 0x0001;
pub const WM8903_MICBIAS_ENA_MASK: u16 = 0x0001;
pub const WM8903_MICBIAS_ENA_SHIFT: u32 = 0;
pub const WM8903_MICBIAS_ENA_WIDTH: u32 = 1;

/* WM8903_GPn_FN values */
pub const WM8903_GPn_FN_GPIO_OUTPUT: u32 = 0;
pub const WM8903_GPn_FN_BCLK: u32 = 1;
pub const WM8903_GPn_FN_IRQ_OUTPT: u32 = 2;
pub const WM8903_GPn_FN_GPIO_INPUT: u32 = 3;
pub const WM8903_GPn_FN_MICBIAS_CURRENT_DETECT: u32 = 4;
pub const WM8903_GPn_FN_MICBIAS_SHORT_DETECT: u32 = 5;
pub const WM8903_GPn_FN_DMIC_LR_CLK_OUTPUT: u32 = 6;
pub const WM8903_GPn_FN_FLL_LOCK_OUTPUT: u32 = 8;
pub const WM8903_GPn_FN_FLL_CLOCK_OUTPUT: u32 = 9;

macro_rules! wm8903_gpio_registers {
    ($n:literal, $p:ident) => {
        pub const $p##_FN_MASK: u16 = 0x1F00;
        pub const $p##_FN_SHIFT: u32 = 8;
        pub const $p##_FN_WIDTH: u32 = 5;
        pub const $p##_DIR: u16 = 0x0080;
        pub const $p##_DIR_MASK: u16 = 0x0080;
        pub const $p##_DIR_SHIFT: u32 = 7;
        pub const $p##_DIR_WIDTH: u32 = 1;
        pub const $p##_OP_CFG: u16 = 0x0040;
        pub const $p##_OP_CFG_MASK: u16 = 0x0040;
        pub const $p##_OP_CFG_SHIFT: u32 = 6;
        pub const $p##_OP_CFG_WIDTH: u32 = 1;
        pub const $p##_IP_CFG: u16 = 0x0020;
        pub const $p##_IP_CFG_MASK: u16 = 0x0020;
        pub const $p##_IP_CFG_SHIFT: u32 = 5;
        pub const $p##_IP_CFG_WIDTH: u32 = 1;
        pub const $p##_LVL: u16 = 0x0010;
        pub const $p##_LVL_MASK: u16 = 0x0010;
        pub const $p##_LVL_SHIFT: u32 = 4;
        pub const $p##_LVL_WIDTH: u32 = 1;
        pub const $p##_PD: u16 = 0x0008;
        pub const $p##_PD_MASK: u16 = 0x0008;
        pub const $p##_PD_SHIFT: u32 = 3;
        pub const $p##_PD_WIDTH: u32 = 1;
        pub const $p##_PU: u16 = 0x0004;
        pub const $p##_PU_MASK: u16 = 0x0004;
        pub const $p##_PU_SHIFT: u32 = 2;
        pub const $p##_PU_WIDTH: u32 = 1;
        pub const $p##_INTMODE: u16 = 0x0002;
        pub const $p##_INTMODE_MASK: u16 = 0x0002;
        pub const $p##_INTMODE_SHIFT: u32 = 1;
        pub const $p##_INTMODE_WIDTH: u32 = 1;
        pub const $p##_DB: u16 = 0x0001;
        pub const $p##_DB_MASK: u16 = 0x0001;
        pub const $p##_DB_SHIFT: u32 = 0;
        pub const $p##_DB_WIDTH: u32 = 1;
    };
}

wm8903_gpio_registers!(1, WM8903_GP1);
wm8903_gpio_registers!(2, WM8903_GP2);
wm8903_gpio_registers!(3, WM8903_GP3);
wm8903_gpio_registers!(4, WM8903_GP4);
wm8903_gpio_registers!(5, WM8903_GP5);

pub const WM8903_NUM_GPIO: usize = 5;

#[repr(C)]
pub struct wm8903_platform_data {
    pub irq_active_low: bool,
    pub micdet_cfg: u16,
    pub micdet_delay: i32,
    pub gpio_base: i32,
    pub gpio_cfg: [u32; WM8903_NUM_GPIO],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
