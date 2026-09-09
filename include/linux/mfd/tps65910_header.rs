Warning: truncated output (original token count: 8631)
Total output lines: 786

/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * tps65910.h  --  TI TPS6591x
 *
 * Copyright 2010-2011 Texas Instruments Inc.
 *
 * Author: Graeme Gregory <gg@slimlogic.co.uk>
 * Author: Jorge Eduardo Candelaria <jedu@slimlogic.co.uk>
 * Author: Arnaud Deconinck <a-deconinck@ti.com>
 */

 __LINUX_MFD_TPS65910_H

// External dependencies from the original header are supplied by other files.
// External dependencies from the original header are supplied by other files.

/* TPS chip id list */
pub const TPS65910: u32 = 0;
pub const TPS65911: u32 = 1;
/* TPS regulator type list */
pub const REGULATOR_LDO: u32 = 0;
pub const REGULATOR_DCDC: u32 = 1;
/*
 * List of registers for component TPS65910
 *
 */

pub const TPS65910_SECONDS: u32 = 0x0;
pub const TPS65910_MINUTES: u32 = 0x1;
pub const TPS65910_HOURS: u32 = 0x2;
pub const TPS65910_DAYS: u32 = 0x3;
pub const TPS65910_MONTHS: u32 = 0x4;
pub const TPS65910_YEARS: u32 = 0x5;
pub const TPS65910_WEEKS: u32 = 0x6;
pub const TPS65910_ALARM_SECONDS: u32 = 0x8;
pub const TPS65910_ALARM_MINUTES: u32 = 0x9;
pub const TPS65910_ALARM_HOURS: u32 = 0xA;
pub const TPS65910_ALARM_DAYS: u32 = 0xB;
pub const TPS65910_ALARM_MONTHS: u32 = 0xC;
pub const TPS65910_ALARM_YEARS: u32 = 0xD;
pub const TPS65910_RTC_CTRL: u32 = 0x10;
pub const TPS65910_RTC_STATUS: u32 = 0x11;
pub const TPS65910_RTC_INTERRUPTS: u32 = 0x12;
pub const TPS65910_RTC_COMP_LSB: u32 = 0x13;
pub const TPS65910_RTC_COMP_MSB: u32 = 0x14;
pub const TPS65910_RTC_RES_PROG: u32 = 0x15;
pub const TPS65910_RTC_RESET_STATUS: u32 = 0x16;
pub const TPS65910_BCK1: u32 = 0x17;
pub const TPS65910_BCK2: u32 = 0x18;
pub const TPS65910_BCK3: u32 = 0x19;
pub const TPS65910_BCK4: u32 = 0x1A;
pub const TPS65910_BCK5: u32 = 0x1B;
pub const TPS65910_PUADEN: u32 = 0x1C;
pub const TPS65910_REF: u32 = 0x1D;
pub const TPS65910_VRTC: u32 = 0x1E;
pub const TPS65910_VIO: u32 = 0x20;
pub const TPS65910_VDD1: u32 = 0x21;
pub const TPS65910_VDD1_OP: u32 = 0x22;
pub const TPS65910_VDD1_SR: u32 = 0x2…7631 tokens truncated…u32 = 14;
/* External sleep controls through EN1/EN2/EN3/SLEEP inputs */
pub const TPS65910_SLEEP_CONTROL_EXT_INPUT_EN1: u32 = 0x1;
pub const TPS65910_SLEEP_CONTROL_EXT_INPUT_EN2: u32 = 0x2;
pub const TPS65910_SLEEP_CONTROL_EXT_INPUT_EN3: u32 = 0x4;
pub const TPS65911_SLEEP_CONTROL_EXT_INPUT_SLEEP: u32 = 0x8;
/*
 * Sleep keepon data: Maintains the state in sleep mode
 * @therm_keepon: Keep on the thermal monitoring in sleep state.
 * @clkout32k_keepon: Keep on the 32KHz clock output in sleep state.
 * @i2chs_keepon: Keep on high speed internal clock in sleep state.
 */
#[repr(C)]
pub struct tps65910_sleep_keepon_data {
    pub therm_keepon: u32,
    pub clkout32k_keepon: u32,
    pub i2chs_keepon: u32,
}

/**
 * struct tps65910_board
 * Board platform data may be used to initialize regulators.
 */

#[repr(C)]
pub struct tps65910_board {
    pub gpio_base: core::ffi::c_int,
    pub irq: core::ffi::c_int,
    pub irq_base: core::ffi::c_int,
    pub vmbch_threshold: core::ffi::c_int,
    pub vmbch2_threshold: core::ffi::c_int,
    pub en_ck32k_xtal: bool,
    pub en_dev_slp: bool,
    pub pm_off: bool,
    pub slp_keepon: tps65910_sleep_keepon_data,
    pub en_gpio_sleep: [bool; TPS6591X_MAX_NUM_GPIO as usize],
    pub regulator_ext_sleep_control: [core::ffi::c_ulong; TPS65910_NUM_REGS as usize],
    pub tps65910_pmic_init_data: [*mut regulator_init_data; TPS65910_NUM_REGS as usize],
}

/**
 * struct tps65910 - tps65910 sub-driver chip access routines
 */

#[repr(C)]
pub struct tps65910 {
    pub dev: *mut device,
    pub i2c_client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub id: core::ffi::c_ulong,
    pub of_plat_data: *mut tps65910_board,
    pub chip_irq: core::ffi::c_int,
    pub irq_data: *mut regmap_irq_chip_data,
}

#[repr(C)]
pub struct tps65910_platform_data {
    pub irq: core::ffi::c_int,
    pub irq_base: core::ffi::c_int,
}

pub unsafe fn tps65910_chip_id(tps65910: *mut tps65910) -> core::ffi::c_int {
    (*tps65910).id as core::ffi::c_int
}




// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
