/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides macros for MAXIM MAX77620 device bindings.
 *
 * Copyright (c) 2016, NVIDIA Corporation.
 * Author: Laxman Dewangan <ldewangan@nvidia.com>
 */

/* MAX77620 interrupts */
pub const MAX77620_IRQ_TOP_GLBL: i32 = 0; // Low-Battery
pub const MAX77620_IRQ_TOP_SD: i32 = 1; // SD power fail
pub const MAX77620_IRQ_TOP_LDO: i32 = 2; // LDO power fail
pub const MAX77620_IRQ_TOP_GPIO: i32 = 3; // GPIO internal int to MAX77620
pub const MAX77620_IRQ_TOP_RTC: i32 = 4; // RTC
pub const MAX77620_IRQ_TOP_32K: i32 = 5; // 32kHz oscillator
pub const MAX77620_IRQ_TOP_ONOFF: i32 = 6; // ON/OFF oscillator
pub const MAX77620_IRQ_LBT_MBATLOW: i32 = 7; // Thermal alarm status, > 120C
pub const MAX77620_IRQ_LBT_TJALRM1: i32 = 8; // Thermal alarm status, > 120C
pub const MAX77620_IRQ_LBT_TJALRM2: i32 = 9; // Thermal alarm status, > 140C

/* FPS event source */
pub const MAX77620_FPS_EVENT_SRC_EN0: i32 = 0;
pub const MAX77620_FPS_EVENT_SRC_EN1: i32 = 1;
pub const MAX77620_FPS_EVENT_SRC_SW: i32 = 2;

/* Device state when FPS event LOW  */
pub const MAX77620_FPS_INACTIVE_STATE_SLEEP: i32 = 0;
pub const MAX77620_FPS_INACTIVE_STATE_LOW_POWER: i32 = 1;

/* FPS source */
pub const MAX77620_FPS_SRC_0: i32 = 0;
pub const MAX77620_FPS_SRC_1: i32 = 1;
pub const MAX77620_FPS_SRC_2: i32 = 2;
pub const MAX77620_FPS_SRC_NONE: i32 = 3;
pub const MAX77620_FPS_SRC_DEF: i32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
