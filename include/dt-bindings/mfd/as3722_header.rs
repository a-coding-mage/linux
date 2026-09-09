/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides macros for ams AS3722 device bindings.
 *
 * Copyright (c) 2013, NVIDIA Corporation.
 *
 * Author: Laxman Dewangan <ldewangan@nvidia.com>
 *
 */

// External control pins
pub const AS3722_EXT_CONTROL_PIN_ENABLE1: i32 = 1;
pub const AS3722_EXT_CONTROL_PIN_ENABLE2: i32 = 2;
pub const AS3722_EXT_CONTROL_PIN_ENABLE3: i32 = 3;

// Interrupt numbers for AS3722
pub const AS3722_IRQ_LID: i32 = 0;
pub const AS3722_IRQ_ACOK: i32 = 1;
pub const AS3722_IRQ_ENABLE1: i32 = 2;
pub const AS3722_IRQ_OCCUR_ALARM_SD0: i32 = 3;
pub const AS3722_IRQ_ONKEY_LONG_PRESS: i32 = 4;
pub const AS3722_IRQ_ONKEY: i32 = 5;
pub const AS3722_IRQ_OVTMP: i32 = 6;
pub const AS3722_IRQ_LOWBAT: i32 = 7;
pub const AS3722_IRQ_SD0_LV: i32 = 8;
pub const AS3722_IRQ_SD1_LV: i32 = 9;
pub const AS3722_IRQ_SD2_LV: i32 = 10;
pub const AS3722_IRQ_PWM1_OV_PROT: i32 = 11;
pub const AS3722_IRQ_PWM2_OV_PROT: i32 = 12;
pub const AS3722_IRQ_ENABLE2: i32 = 13;
pub const AS3722_IRQ_SD6_LV: i32 = 14;
pub const AS3722_IRQ_RTC_REP: i32 = 15;
pub const AS3722_IRQ_RTC_ALARM: i32 = 16;
pub const AS3722_IRQ_GPIO1: i32 = 17;
pub const AS3722_IRQ_GPIO2: i32 = 18;
pub const AS3722_IRQ_GPIO3: i32 = 19;
pub const AS3722_IRQ_GPIO4: i32 = 20;
pub const AS3722_IRQ_GPIO5: i32 = 21;
pub const AS3722_IRQ_WATCHDOG: i32 = 22;
pub const AS3722_IRQ_ENABLE3: i32 = 23;
pub const AS3722_IRQ_TEMP_SD0_SHUTDOWN: i32 = 24;
pub const AS3722_IRQ_TEMP_SD1_SHUTDOWN: i32 = 25;
pub const AS3722_IRQ_TEMP_SD2_SHUTDOWN: i32 = 26;
pub const AS3722_IRQ_TEMP_SD0_ALARM: i32 = 27;
pub const AS3722_IRQ_TEMP_SD1_ALARM: i32 = 28;
pub const AS3722_IRQ_TEMP_SD6_ALARM: i32 = 29;
pub const AS3722_IRQ_OCCUR_ALARM_SD6: i32 = 30;
pub const AS3722_IRQ_ADC: i32 = 31;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
