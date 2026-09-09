/* linux/mfd/tps65010.h
 *
 * Functions to access TPS65010 power management device.
 *
 * Copyright (C) 2004 Dirk Behme <dirk.behme@de.bosch.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 */

// C header guard: __LINUX_I2C_TPS65010_H

pub struct gpio_chip;

/* Registers, all 8 bits */
pub const TPS_CHGSTATUS: u32 = 0x01;
pub const TPS_CHG_USB: u32 = 1 << 7;
pub const TPS_CHG_AC: u32 = 1 << 6;
pub const TPS_CHG_THERM: u32 = 1 << 5;
pub const TPS_CHG_TERM: u32 = 1 << 4;
pub const TPS_CHG_TAPER_TMO: u32 = 1 << 3;
pub const TPS_CHG_CHG_TMO: u32 = 1 << 2;
pub const TPS_CHG_PRECHG_TMO: u32 = 1 << 1;
pub const TPS_CHG_TEMP_ERR: u32 = 1 << 0;
pub const TPS_REGSTATUS: u32 = 0x02;
pub const TPS_REG_ONOFF: u32 = 1 << 7;
pub const TPS_REG_COVER: u32 = 1 << 6;
pub const TPS_REG_UVLO: u32 = 1 << 5;
pub const TPS_REG_NO_CHG: u32 = 1 << 4; // tps65013
pub const TPS_REG_PG_LD02: u32 = 1 << 3;
pub const TPS_REG_PG_LD01: u32 = 1 << 2;
pub const TPS_REG_PG_MAIN: u32 = 1 << 1;
pub const TPS_REG_PG_CORE: u32 = 1 << 0;
pub const TPS_MASK1: u32 = 0x03;
pub const TPS_MASK2: u32 = 0x04;
pub const TPS_ACKINT1: u32 = 0x05;
pub const TPS_ACKINT2: u32 = 0x06;
pub const TPS_CHGCONFIG: u32 = 0x07;
pub const TPS_CHARGE_POR: u32 = 1 << 7; // 65010/65012
pub const TPS65013_AUA: u32 = 1 << 7; // 65011/65013
pub const TPS_CHARGE_RESET: u32 = 1 << 6;
pub const TPS_CHARGE_FAST: u32 = 1 << 5;
pub const TPS_CHARGE_CURRENT: u32 = 3 << 3;
pub const TPS_VBUS_500MA: u32 = 1 << 2;
pub const TPS_VBUS_CHARGING: u32 = 1 << 1;
pub const TPS_CHARGE_ENABLE: u32 = 1 << 0;
pub const TPS_LED1_ON: u32 = 0x08;
pub const TPS_LED1_PER: u32 = 0x09;
pub const TPS_LED2_ON: u32 = 0x0a;
pub const TPS_LED2_PER: u32 = 0x0b;
pub const TPS_VDCDC1: u32 = 0x0c;
pub const TPS_ENABLE_LP: u32 = 1 << 3;
pub const TPS_VDCDC2: u32 = 0x0d;
pub const TPS_LP_COREOFF: u32 = 1 << 7;
pub const TPS_VCORE_1_8V: u32 = 7 << 4;
pub const TPS_VCORE_1_5V: u32 = 6 << 4;
pub const TPS_VCORE_1_4V: u32 = 5 << 4;
pub const TPS_VCORE_1_3V: u32 = 4 << 4;
pub const TPS_VCORE_1_2V: u32 = 3 << 4;
pub const TPS_VCORE_1_1V: u32 = 2 << 4;
pub const TPS_VCORE_1_0V: u32 = 1 << 4;
pub const TPS_VCORE_0_85V: u32 = 0 << 4;
pub const TPS_VCORE_LP_1_2V: u32 = 3 << 2;
pub const TPS_VCORE_LP_1_1V: u32 = 2 << 2;
pub const TPS_VCORE_LP_1_0V: u32 = 1 << 2;
pub const TPS_VCORE_LP_0_85V: u32 = 0 << 2;
pub const TPS_VIB: u32 = 1 << 1;
pub const TPS_VCORE_DISCH: u32 = 1 << 0;
pub const TPS_VREGS1: u32 = 0x0e;
pub const TPS_LDO2_ENABLE: u32 = 1 << 7;
pub const TPS_LDO2_OFF: u32 = 1 << 6;
pub const TPS_VLDO2_3_0V: u32 = 3 << 4;
pub const TPS_VLDO2_2_75V: u32 = 2 << 4;
pub const TPS_VLDO2_2_5V: u32 = 1 << 4;
pub const TPS_VLDO2_1_8V: u32 = 0 << 4;
pub const TPS_LDO1_ENABLE: u32 = 1 << 3;
pub const TPS_LDO1_OFF: u32 = 1 << 2;
pub const TPS_VLDO1_3_0V: u32 = 3 << 0;
pub const TPS_VLDO1_2_75V: u32 = 2 << 0;
pub const TPS_VLDO1_2_5V: u32 = 1 << 0;
pub const TPS_VLDO1_ADJ: u32 = 0 << 0;
pub const TPS_MASK3: u32 = 0x0f;
pub const TPS_DEFGPIO: u32 = 0x10;

pub const LED1: u32 = 1;
pub const LED2: u32 = 2;
pub const OFF: u32 = 0;
pub const ON: u32 = 1;
pub const BLINK: u32 = 2;
pub const GPIO1: u32 = 1;
pub const GPIO2: u32 = 2;
pub const GPIO3: u32 = 3;
pub const GPIO4: u32 = 4;
pub const LOW: u32 = 0;
pub const HIGH: u32 = 1;

extern "C" {
    pub fn tps65010_set_vbus_draw(mA: u32) -> i32;
    pub fn tps65010_set_gpio_out_value(gpio: u32, value: u32) -> i32;
    pub fn tps65010_set_led(led: u32, mode: u32) -> i32;
    pub fn tps65010_set_vib(value: u32) -> i32;
    pub fn tps65010_set_low_pwr(mode: u32) -> i32;
    pub fn tps65010_config_vregs1(value: u32) -> i32;
    pub fn tps65013_set_low_pwr(mode: u32) -> i32;
    pub fn tps65010_config_vdcdc2(value: u32) -> i32;
}

pub struct i2c_client;

pub struct tps65010_board {
    pub outmask: u32,
    pub setup: Option<unsafe extern "C" fn(client: *mut i2c_client, gc: *mut gpio_chip) -> i32>,
    pub teardown: Option<unsafe extern "C" fn(client: *mut i2c_client, gc: *mut gpio_chip)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
