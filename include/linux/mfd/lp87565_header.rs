/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Functions to access LP87565 power management chip.
 *
 * Copyright (C) 2017 Texas Instruments Incorporated - https://www.ti.com/
 */

// External kernel types are supplied by dependent translation units.
const fn bit(n: u32) -> u32 { 1u32 << n }

#[repr(i32)]
pub enum lp87565_device_type {
    LP87565_DEVICE_TYPE_UNKNOWN = 0,
    LP87565_DEVICE_TYPE_LP87524_Q1,
    LP87565_DEVICE_TYPE_LP87561_Q1,
    LP87565_DEVICE_TYPE_LP87565_Q1,
}

// All register addresses
pub const LP87565_REG_DEV_REV: u32 = 0x00;
pub const LP87565_REG_OTP_REV: u32 = 0x01;
pub const LP87565_REG_BUCK0_CTRL_1: u32 = 0x02;
pub const LP87565_REG_BUCK0_CTRL_2: u32 = 0x03;
pub const LP87565_REG_BUCK1_CTRL_1: u32 = 0x04;
pub const LP87565_REG_BUCK1_CTRL_2: u32 = 0x05;
pub const LP87565_REG_BUCK2_CTRL_1: u32 = 0x06;
pub const LP87565_REG_BUCK2_CTRL_2: u32 = 0x07;
pub const LP87565_REG_BUCK3_CTRL_1: u32 = 0x08;
pub const LP87565_REG_BUCK3_CTRL_2: u32 = 0x09;
pub const LP87565_REG_BUCK0_VOUT: u32 = 0x0A;
pub const LP87565_REG_BUCK0_FLOOR_VOUT: u32 = 0x0B;
pub const LP87565_REG_BUCK1_VOUT: u32 = 0x0C;
pub const LP87565_REG_BUCK1_FLOOR_VOUT: u32 = 0x0D;
pub const LP87565_REG_BUCK2_VOUT: u32 = 0x0E;
pub const LP87565_REG_BUCK2_FLOOR_VOUT: u32 = 0x0F;
pub const LP87565_REG_BUCK3_VOUT: u32 = 0x10;
pub const LP87565_REG_BUCK3_FLOOR_VOUT: u32 = 0x11;
pub const LP87565_REG_BUCK0_DELAY: u32 = 0x12;
pub const LP87565_REG_BUCK1_DELAY: u32 = 0x13;
pub const LP87565_REG_BUCK2_DELAY: u32 = 0x14;
pub const LP87565_REG_BUCK3_DELAY: u32 = 0x15;
pub const LP87565_REG_GPO2_DELAY: u32 = 0x16;
pub const LP87565_REG_GPO3_DELAY: u32 = 0x17;
pub const LP87565_REG_RESET: u32 = 0x18;
pub const LP87565_REG_CONFIG: u32 = 0x19;
pub const LP87565_REG_INT_TOP_1: u32 = 0x1A;
pub const LP87565_REG_INT_TOP_2: u32 = 0x1B;
pub const LP87565_REG_INT_BUCK_0_1: u32 = 0x1C;
pub const LP87565_REG_INT_BUCK_2_3: u32 = 0x1D;
pub const LP87565_REG_TOP_STAT: u32 = 0x1E;
pub const LP87565_REG_BUCK_0_1_STAT: u32 = 0x1F;
pub const LP87565_REG_BUCK_2_3_STAT: u32 = 0x20;
pub const LP87565_REG_TOP_MASK_1: u32 = 0x21;
pub const LP87565_REG_TOP_MASK_2: u32 = 0x22;
pub const LP87565_REG_BUCK_0_1_MASK: u32 = 0x23;
pub const LP87565_REG_BUCK_2_3_MASK: u32 = 0x24;
pub const LP87565_REG_SEL_I_LOAD: u32 = 0x25;
pub const LP87565_REG_I_LOAD_2: u32 = 0x26;
pub const LP87565_REG_I_LOAD_1: u32 = 0x27;
pub const LP87565_REG_PGOOD_CTRL1: u32 = 0x28;
pub const LP87565_REG_PGOOD_CTRL2: u32 = 0x29;
pub const LP87565_REG_PGOOD_FLT: u32 = 0x2A;
pub const LP87565_REG_PLL_CTRL: u32 = 0x2B;
pub const LP87565_REG_PIN_FUNCTION: u32 = 0x2C;
pub const LP87565_REG_GPIO_CONFIG: u32 = 0x2D;
pub const LP87565_REG_GPIO_IN: u32 = 0x2E;
pub const LP87565_REG_GPIO_OUT: u32 = 0x2F;
pub const LP87565_REG_MAX: u32 = LP87565_REG_GPIO_OUT;

// Register field definitions
pub const LP87565_DEV_REV_DEV_ID: u32 = 0xC0;
pub const LP87565_DEV_REV_ALL_LAYER: u32 = 0x30;
pub const LP87565_DEV_REV_METAL_LAYER: u32 = 0x0F;
pub const LP87565_OTP_REV_OTP_ID: u32 = 0xFF;
pub const LP87565_BUCK_CTRL_1_EN: u32 = bit(7);
pub const LP87565_BUCK_CTRL_1_EN_PIN_CTRL: u32 = bit(6);
pub const LP87565_BUCK_CTRL_1_PIN_SELECT_EN: u32 = 0x30;
pub const LP87565_BUCK_CTRL_1_ROOF_FLOOR_EN: u32 = bit(3);
pub const LP87565_BUCK_CTRL_1_RDIS_EN: u32 = bit(2);
pub const LP87565_BUCK_CTRL_1_FPWM: u32 = bit(1);
// Bit0 is reserved for BUCK1 and BUCK3 and valid only for BUCK0 and BUCK2
pub const LP87565_BUCK_CTRL_1_FPWM_MP_0_2: u32 = bit(0);
pub const LP87565_BUCK_CTRL_2_ILIM: u32 = 0x38;
pub const LP87565_BUCK_CTRL_2_SLEW_RATE: u32 = 0x07;
pub const LP87565_BUCK_VSET: u32 = 0xFF;
pub const LP87565_BUCK_FLOOR_VSET: u32 = 0xFF;
pub const LP87565_BUCK_SHUTDOWN_DELAY: u32 = 0xF0;
pub const LP87565_BUCK_STARTUP_DELAY: u32 = 0x0F;
pub const LP87565_GPIO_SHUTDOWN_DELAY: u32 = 0xF0;
pub const LP87565_GPIO_STARTUP_DELAY: u32 = 0x0F;
pub const LP87565_RESET_SW_RESET: u32 = bit(0);
pub const LP87565_CONFIG_DOUBLE_DELAY: u32 = bit(7);
pub const LP87565_CONFIG_CLKIN_PD: u32 = bit(6);
pub const LP87565_CONFIG_EN4_PD: u32 = bit(5);
pub const LP87565_CONFIG_EN3_PD: u32 = bit(4);
pub const LP87565_CONFIG_TDIE_WARN_LEVEL: u32 = bit(3);
pub const LP87565_CONFIG_EN2_PD: u32 = bit(2);
pub const LP87565_CONFIG_EN1_PD: u32 = bit(1);
pub const LP87565_INT_GPIO: u32 = bit(7);
pub const LP87565_INT_BUCK23: u32 = bit(6);
pub const LP87565_INT_BUCK01: u32 = bit(5);
pub const LP87565_NO_SYNC_CLK: u32 = bit(4);
pub const LP87565_TDIE_SD: u32 = bit(3);
pub const LP87565_TDIE_WARN: u32 = bit(2);
pub const LP87565_INT_OVP: u32 = bit(1);
pub const LP87565_I_LOAD_READY: u32 = bit(0);
pub const LP87565_INT_TOP2_RESET_REG: u32 = bit(0);
pub const LP87565_BUCK1_PG_INT: u32 = bit(6);
pub const LP87565_BUCK1_SC_INT: u32 = bit(5);
pub const LP87565_BUCK1_ILIM_INT: u32 = bit(4);
pub const LP87565_BUCK0_PG_INT: u32 = bit(2);
pub const LP87565_BUCK0_SC_INT: u32 = bit(1);
pub const LP87565_BUCK0_ILIM_INT: u32 = bit(0);
pub const LP87565_BUCK3_PG_INT: u32 = bit(6);
pub const LP87565_BUCK3_SC_INT: u32 = bit(5);
pub const LP87565_BUCK3_ILIM_INT: u32 = bit(4);
pub const LP87565_BUCK2_PG_INT: u32 = bit(2);
pub const LP87565_BUCK2_SC_INT: u32 = bit(1);
pub const LP87565_BUCK2_ILIM_INT: u32 = bit(0);
pub const LP87565_SYNC_CLK_STAT: u32 = bit(4);
pub const LP87565_TDIE_SD_STAT: u32 = bit(3);
pub const LP87565_TDIE_WARN_STAT: u32 = bit(2);
pub const LP87565_OVP_STAT: u32 = bit(1);
pub const LP87565_BUCK1_STAT: u32 = bit(7);
pub const LP87565_BUCK1_PG_STAT: u32 = bit(6);
pub const LP87565_BUCK1_ILIM_STAT: u32 = bit(4);
pub const LP87565_BUCK0_STAT: u32 = bit(3);
pub const LP87565_BUCK0_PG_STAT: u32 = bit(2);
pub const LP87565_BUCK0_ILIM_STAT: u32 = bit(0);
pub const LP87565_BUCK3_STAT: u32 = bit(7);
pub const LP87565_BUCK3_PG_STAT: u32 = bit(6);
pub const LP87565_BUCK3_ILIM_STAT: u32 = bit(4);
pub const LP87565_BUCK2_STAT: u32 = bit(3);
pub const LP87565_BUCK2_PG_STAT: u32 = bit(2);
pub const LP87565_BUCK2_ILIM_STAT: u32 = bit(0);
pub const LPL87565_GPIO_MASK: u32 = bit(7);
pub const LPL87565_SYNC_CLK_MASK: u32 = bit(4);
pub const LPL87565_TDIE_WARN_MASK: u32 = bit(2);
pub const LPL87565_I_LOAD_READY_MASK: u32 = bit(0);
pub const LPL87565_RESET_REG_MASK: u32 = bit(0);
pub const LPL87565_BUCK1_PG_MASK: u32 = bit(6);
pub const LPL87565_BUCK1_ILIM_MASK: u32 = bit(4);
pub const LPL87565_BUCK0_PG_MASK: u32 = bit(2);
pub const LPL87565_BUCK0_ILIM_MASK: u32 = bit(0);
pub const LPL87565_BUCK3_PG_MASK: u32 = bit(6);
pub const LPL87565_BUCK3_ILIM_MASK: u32 = bit(4);
pub const LPL87565_BUCK2_PG_MASK: u32 = bit(2);
pub const LPL87565_BUCK2_ILIM_MASK: u32 = bit(0);
pub const LP87565_LOAD_CURRENT_BUCK_SELECT: u32 = 0x3;
pub const LP87565_I_LOAD2_BUCK_LOAD_CURRENT: u32 = 0x3;
pub const LP87565_I_LOAD1_BUCK_LOAD_CURRENT: u32 = 0xFF;
pub const LP87565_PG3_SEL: u32 = 0xC0;
pub const LP87565_PG2_SEL: u32 = 0x30;
pub const LP87565_PG1_SEL: u32 = 0x0C;
pub const LP87565_PG0_SEL: u32 = 0x03;
pub const LP87565_HALF_DAY: u32 = bit(7);
pub const LP87565_EN_PG0_NINT: u32 = bit(6);
pub const LP87565_PGOOD_SET_DELAY: u32 = bit(5);
pub const LP87565_EN_PGFLT_STAT: u32 = bit(4);
pub const LP87565_PGOOD_WINDOW: u32 = bit(2);
pub const LP87565_PGOOD_OD: u32 = bit(1);
pub const LP87565_PGOOD_POL: u32 = bit(0);
pub const LP87565_PG3_FLT: u32 = bit(3);
pub const LP87565_PG2_FLT: u32 = bit(2);
pub const LP87565_PG1_FLT: u32 = bit(1);
pub const LP87565_PG0_FLT: u32 = bit(0);
pub const LP87565_PLL_MODE: u32 = 0xC0;
pub const LP87565_EXT_CLK_FREQ: u32 = 0x1F;
pub const LP87565_EN_SPREAD_SPEC: u32 = bit(7);
pub const LP87565_EN_PIN_CTRL_GPIO3: u32 = bit(6);
pub const LP87565_EN_PIN_SELECT_GPIO3: u32 = bit(5);
pub const LP87565_EN_PIN_CTRL_GPIO2: u32 = bit(4);
pub const LP87565_EN_PIN_SELECT_GPIO2: u32 = bit(3);
pub const LP87565_GPIO3_SEL: u32 = bit(2);
pub const LP87565_GPIO2_SEL: u32 = bit(1);
pub const LP87565_GPIO1_SEL: u32 = bit(0);
pub const LP87565_GPIO3_OD: u32 = bit(6);
pub const LP87565_GPIO2_OD: u32 = bit(5);
pub const LP87565_GPIO1_OD: u32 = bit(4);
pub const LP87565_GPIO3_DIR: u32 = bit(2);
pub const LP87565_GPIO2_DIR: u32 = bit(1);
pub const LP87565_GPIO1_DIR: u32 = bit(0);
pub const LP87565_GPIO3_IN: u32 = bit(2);
pub const LP87565_GPIO2_IN: u32 = bit(1);
pub const LP87565_GPIO1_IN: u32 = bit(0);
pub const LP87565_GPIO3_OUT: u32 = bit(2);
pub const LP87565_GPIO2_OUT: u32 = bit(1);
pub const LP87565_GPIO1_OUT: u32 = bit(0);

/**
 * struct LP87565 - state holder for the LP87565 driver
 * @dev: struct device pointer for MFD device
 * @rev: revision of the LP87565
 * @dev_type: The device type for example lp87565-q1
 * @lock: lock guarding the data structure
 * @regmap: register map of the LP87565 PMIC
 *
 * Device data may be used to access the LP87565 chip
 */
#[repr(C)]
pub struct lp87565 {
    pub dev: *mut device,
    pub rev: u8,
    pub dev_type: u8,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
