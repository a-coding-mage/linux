/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * max8998.h - Voltage regulator driver for the Maxim 8998
 *
 *  Copyright (C) 2009-2010 Samsung Electronics
 *  Kyungmin Park <kyungmin.park@samsung.com>
 *  Marek Szyprowski <m.szyprowski@samsung.com>
 */

// Dependency supplied by the surrounding kernel translation:
// #include <linux/regulator/machine.h>

/* MAX 8998 regulator ids */
pub const MAX8998_LDO2: i32 = 2;
pub const MAX8998_LDO3: i32 = 3;
pub const MAX8998_LDO4: i32 = 4;
pub const MAX8998_LDO5: i32 = 5;
pub const MAX8998_LDO6: i32 = 6;
pub const MAX8998_LDO7: i32 = 7;
pub const MAX8998_LDO8: i32 = 8;
pub const MAX8998_LDO9: i32 = 9;
pub const MAX8998_LDO10: i32 = 10;
pub const MAX8998_LDO11: i32 = 11;
pub const MAX8998_LDO12: i32 = 12;
pub const MAX8998_LDO13: i32 = 13;
pub const MAX8998_LDO14: i32 = 14;
pub const MAX8998_LDO15: i32 = 15;
pub const MAX8998_LDO16: i32 = 16;
pub const MAX8998_LDO17: i32 = 17;
pub const MAX8998_BUCK1: i32 = 18;
pub const MAX8998_BUCK2: i32 = 19;
pub const MAX8998_BUCK3: i32 = 20;
pub const MAX8998_BUCK4: i32 = 21;
pub const MAX8998_EN32KHZ_AP: i32 = 22;
pub const MAX8998_EN32KHZ_CP: i32 = 23;
pub const MAX8998_ENVICHG: i32 = 24;
pub const MAX8998_ESAFEOUT1: i32 = 25;
pub const MAX8998_ESAFEOUT2: i32 = 26;
pub const MAX8998_CHARGER: i32 = 27;

/**
 * max8998_regulator_data - regulator data
 * @id: regulator id
 * @initdata: regulator init data (contraints, supplies, ...)
 * @reg_node: DT node of regulator (unused on non-DT platforms)
 */
#[repr(C)]
pub struct max8998_regulator_data {
    pub id: ::core::ffi::c_int,
    pub initdata: *mut regulator_init_data,
    pub reg_node: *mut device_node,
}

/**
 * struct max8998_board - packages regulator init data
 * @regulators: array of defined regulators
 * @num_regulators: number of regulators used
 * @irq_base: base IRQ number for max8998, required for IRQs
 * @ono: power onoff IRQ number for max8998
 * @buck_voltage_lock: Do NOT change the values of the following six
 *   registers set by buck?_voltage?. The voltage of BUCK1/2 cannot
 *   be other than the preset values.
 * @buck1_voltage: BUCK1 DVS mode 1 voltage registers
 * @buck2_voltage: BUCK2 DVS mode 2 voltage registers
 * @buck1_default_idx: Default for BUCK1 gpio pin 1, 2
 * @buck2_default_idx: Default for BUCK2 gpio pin.
 * @wakeup: Allow to wake up from suspend
 * @rtc_delay: LP3974 RTC chip bug that requires delay after a register
 * write before reading it.
 * @eoc: End of Charge Level in percent: 10% ~ 45% by 5% step
 *   If it equals 0, leave it unchanged.
 *   Otherwise, it is a invalid value.
 * @restart: Restart Level in mV: 100, 150, 200, and -1 for disable.
 *   If it equals 0, leave it unchanged.
 *   Otherwise, it is a invalid value.
 * @timeout: Full Timeout in hours: 5, 6, 7, and -1 for disable.
 *   If it equals 0, leave it unchanged.
 *   Otherwise, leave it unchanged.
 */
#[repr(C)]
pub struct max8998_platform_data {
    pub regulators: *mut max8998_regulator_data,
    pub num_regulators: ::core::ffi::c_int,
    pub irq_base: ::core::ffi::c_uint,
    pub ono: ::core::ffi::c_int,
    pub buck_voltage_lock: bool,
    pub buck1_voltage: [::core::ffi::c_int; 4],
    pub buck2_voltage: [::core::ffi::c_int; 2],
    pub buck1_default_idx: ::core::ffi::c_int,
    pub buck2_default_idx: ::core::ffi::c_int,
    pub wakeup: bool,
    pub rtc_delay: bool,
    pub eoc: ::core::ffi::c_int,
    pub restart: ::core::ffi::c_int,
    pub timeout: ::core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
