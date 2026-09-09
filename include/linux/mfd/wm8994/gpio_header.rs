/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/mfd/wm8994/gpio.h - GPIO configuration for WM8994
 *
 * Copyright 2009 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

pub const WM8994_GPIO_MAX: i32 = 11;

pub const WM8994_GP_FN_PIN_SPECIFIC: i32 = 0;
pub const WM8994_GP_FN_GPIO: i32 = 1;
pub const WM8994_GP_FN_SDOUT: i32 = 2;
pub const WM8994_GP_FN_IRQ: i32 = 3;
pub const WM8994_GP_FN_TEMPERATURE: i32 = 4;
pub const WM8994_GP_FN_MICBIAS1_DET: i32 = 5;
pub const WM8994_GP_FN_MICBIAS1_SHORT: i32 = 6;
pub const WM8994_GP_FN_MICBIAS2_DET: i32 = 7;
pub const WM8994_GP_FN_MICBIAS2_SHORT: i32 = 8;
pub const WM8994_GP_FN_FLL1_LOCK: i32 = 9;
pub const WM8994_GP_FN_FLL2_LOCK: i32 = 10;
pub const WM8994_GP_FN_SRC1_LOCK: i32 = 11;
pub const WM8994_GP_FN_SRC2_LOCK: i32 = 12;
pub const WM8994_GP_FN_DRC1_ACT: i32 = 13;
pub const WM8994_GP_FN_DRC2_ACT: i32 = 14;
pub const WM8994_GP_FN_DRC3_ACT: i32 = 15;
pub const WM8994_GP_FN_WSEQ_STATUS: i32 = 16;
pub const WM8994_GP_FN_FIFO_ERROR: i32 = 17;
pub const WM8994_GP_FN_OPCLK: i32 = 18;
pub const WM8994_GP_FN_THW: i32 = 19;
pub const WM8994_GP_FN_DCS_DONE: i32 = 20;
pub const WM8994_GP_FN_FLL1_OUT: i32 = 21;
pub const WM8994_GP_FN_FLL2_OUT: i32 = 22;

pub const WM8994_GPN_DIR: i32 = 0x8000; /* GPN_DIR */
pub const WM8994_GPN_DIR_MASK: i32 = 0x8000; /* GPN_DIR */
pub const WM8994_GPN_DIR_SHIFT: i32 = 15; /* GPN_DIR */
pub const WM8994_GPN_DIR_WIDTH: i32 = 1; /* GPN_DIR */
pub const WM8994_GPN_PU: i32 = 0x4000; /* GPN_PU */
pub const WM8994_GPN_PU_MASK: i32 = 0x4000; /* GPN_PU */
pub const WM8994_GPN_PU_SHIFT: i32 = 14; /* GPN_PU */
pub const WM8994_GPN_PU_WIDTH: i32 = 1; /* GPN_PU */
pub const WM8994_GPN_PD: i32 = 0x2000; /* GPN_PD */
pub const WM8994_GPN_PD_MASK: i32 = 0x2000; /* GPN_PD */
pub const WM8994_GPN_PD_SHIFT: i32 = 13; /* GPN_PD */
pub const WM8994_GPN_PD_WIDTH: i32 = 1; /* GPN_PD */
pub const WM8994_GPN_POL: i32 = 0x0400; /* GPN_POL */
pub const WM8994_GPN_POL_MASK: i32 = 0x0400; /* GPN_POL */
pub const WM8994_GPN_POL_SHIFT: i32 = 10; /* GPN_POL */
pub const WM8994_GPN_POL_WIDTH: i32 = 1; /* GPN_POL */
pub const WM8994_GPN_OP_CFG: i32 = 0x0200; /* GPN_OP_CFG */
pub const WM8994_GPN_OP_CFG_MASK: i32 = 0x0200; /* GPN_OP_CFG */
pub const WM8994_GPN_OP_CFG_SHIFT: i32 = 9; /* GPN_OP_CFG */
pub const WM8994_GPN_OP_CFG_WIDTH: i32 = 1; /* GPN_OP_CFG */
pub const WM8994_GPN_DB: i32 = 0x0100; /* GPN_DB */
pub const WM8994_GPN_DB_MASK: i32 = 0x0100; /* GPN_DB */
pub const WM8994_GPN_DB_SHIFT: i32 = 8; /* GPN_DB */
pub const WM8994_GPN_DB_WIDTH: i32 = 1; /* GPN_DB */
pub const WM8994_GPN_LVL: i32 = 0x0040; /* GPN_LVL */
pub const WM8994_GPN_LVL_MASK: i32 = 0x0040; /* GPN_LVL */
pub const WM8994_GPN_LVL_SHIFT: i32 = 6; /* GPN_LVL */
pub const WM8994_GPN_LVL_WIDTH: i32 = 1; /* GPN_LVL */
pub const WM8994_GPN_FN_MASK: i32 = 0x001F; /* GPN_FN - [4:0] */
pub const WM8994_GPN_FN_SHIFT: i32 = 0; /* GPN_FN - [4:0] */
pub const WM8994_GPN_FN_WIDTH: i32 = 5; /* GPN_FN - [4:0] */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
