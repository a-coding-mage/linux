// SPDX-License-Identifier: GPL-2.0
/*
 * NXP ISP1301 USB transceiver driver
 *
 * Copyright (C) 2012 Roland Stigge <stigge@antcom.de>
 */

// Dependency intent: the C header includes <linux/of.h> for `struct device_node`.

/* I2C Register definitions: */

pub const ISP1301_I2C_MODE_CONTROL_1: u32 = 0x04; // u8 read, set, +1 clear

pub const MC1_SPEED_REG: u32 = 1 << 0;
pub const MC1_SUSPEND_REG: u32 = 1 << 1;
pub const MC1_DAT_SE0: u32 = 1 << 2;
pub const MC1_TRANSPARENT: u32 = 1 << 3;
pub const MC1_BDIS_ACON_EN: u32 = 1 << 4;
pub const MC1_OE_INT_EN: u32 = 1 << 5;
pub const MC1_UART_EN: u32 = 1 << 6;
pub const MC1_MASK: u32 = 0x7f;

pub const ISP1301_I2C_MODE_CONTROL_2: u32 = 0x12; // u8 read, set, +1 clear

pub const MC2_GLOBAL_PWR_DN: u32 = 1 << 0;
pub const MC2_SPD_SUSP_CTRL: u32 = 1 << 1;
pub const MC2_BI_DI: u32 = 1 << 2;
pub const MC2_TRANSP_BDIR0: u32 = 1 << 3;
pub const MC2_TRANSP_BDIR1: u32 = 1 << 4;
pub const MC2_AUDIO_EN: u32 = 1 << 5;
pub const MC2_PSW_EN: u32 = 1 << 6;
pub const MC2_EN2V7: u32 = 1 << 7;

pub const ISP1301_I2C_OTG_CONTROL_1: u32 = 0x06; // u8 read, set, +1 clear

pub const OTG1_DP_PULLUP: u32 = 1 << 0;
pub const OTG1_DM_PULLUP: u32 = 1 << 1;
pub const OTG1_DP_PULLDOWN: u32 = 1 << 2;
pub const OTG1_DM_PULLDOWN: u32 = 1 << 3;
pub const OTG1_ID_PULLDOWN: u32 = 1 << 4;
pub const OTG1_VBUS_DRV: u32 = 1 << 5;
pub const OTG1_VBUS_DISCHRG: u32 = 1 << 6;
pub const OTG1_VBUS_CHRG: u32 = 1 << 7;

pub const ISP1301_I2C_OTG_CONTROL_2: u32 = 0x10; // u8 readonly

pub const OTG_B_SESS_END: u32 = 1 << 6;
pub const OTG_B_SESS_VLD: u32 = 1 << 7;

pub const ISP1301_I2C_INTERRUPT_SOURCE: u32 = 0x8;
pub const ISP1301_I2C_INTERRUPT_LATCH: u32 = 0xA;
pub const ISP1301_I2C_INTERRUPT_FALLING: u32 = 0xC;
pub const ISP1301_I2C_INTERRUPT_RISING: u32 = 0xE;

pub const INT_VBUS_VLD: u32 = 1 << 0;
pub const INT_SESS_VLD: u32 = 1 << 1;
pub const INT_DP_HI: u32 = 1 << 2;
pub const INT_ID_GND: u32 = 1 << 3;
pub const INT_DM_HI: u32 = 1 << 4;
pub const INT_ID_FLOAT: u32 = 1 << 5;
pub const INT_BDIS_ACON: u32 = 1 << 6;
pub const INT_CR_INT: u32 = 1 << 7;

pub const ISP1301_I2C_REG_CLEAR_ADDR: u32 = 1; // Register Address Modifier

#[allow(non_camel_case_types)]
pub enum i2c_client {}
#[allow(non_camel_case_types)]
pub enum device_node {}

extern "C" {
    pub fn isp1301_get_client(node: *mut device_node) -> *mut i2c_client;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
