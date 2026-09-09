/* linux/mfd/tps6507x.h
 *
 * Functions to access TPS65070 power management chip.
 *
 * Copyright (c) 2009 RidgeRun (todd.fischer@ridgerun.com)
 *
 * For licencing details see kernel-base/COPYING
 */

// Registers, all 8 bits.

pub const TPS6507X_REG_PPATH1: u8 = 0x01;
pub const TPS6507X_CHG_USB: u8 = 1u8 << 7;
pub const TPS6507X_CHG_AC: u8 = 1u8 << 6;
pub const TPS6507X_CHG_USB_PW_ENABLE: u8 = 1u8 << 5;
pub const TPS6507X_CHG_AC_PW_ENABLE: u8 = 1u8 << 4;
pub const TPS6507X_CHG_AC_CURRENT: u8 = 1u8 << 2;
pub const TPS6507X_CHG_USB_CURRENT: u8 = 1u8 << 0;

pub const TPS6507X_REG_INT: u8 = 0x02;
pub const TPS6507X_REG_MASK_AC_USB: u8 = 1u8 << 7;
pub const TPS6507X_REG_MASK_TSC: u8 = 1u8 << 6;
pub const TPS6507X_REG_MASK_PB_IN: u8 = 1u8 << 5;
pub const TPS6507X_REG_TSC_INT: u8 = 1u8 << 3;
pub const TPS6507X_REG_PB_IN_INT: u8 = 1u8 << 2;
pub const TPS6507X_REG_AC_USB_APPLIED: u8 = 1u8 << 1;
pub const TPS6507X_REG_AC_USB_REMOVED: u8 = 1u8 << 0;

pub const TPS6507X_REG_CHGCONFIG0: u8 = 0x03;
pub const TPS6507X_REG_CHGCONFIG1: u8 = 0x04;
pub const TPS6507X_CON_CTRL1_DCDC1_ENABLE: u8 = 1u8 << 4;
pub const TPS6507X_CON_CTRL1_DCDC2_ENABLE: u8 = 1u8 << 3;
pub const TPS6507X_CON_CTRL1_DCDC3_ENABLE: u8 = 1u8 << 2;
pub const TPS6507X_CON_CTRL1_LDO1_ENABLE: u8 = 1u8 << 1;
pub const TPS6507X_CON_CTRL1_LDO2_ENABLE: u8 = 1u8 << 0;
pub const TPS6507X_REG_CHGCONFIG2: u8 = 0x05;
pub const TPS6507X_REG_CHGCONFIG3: u8 = 0x06;

pub const TPS6507X_REG_ADCONFIG: u8 = 0x07;
pub const TPS6507X_ADCONFIG_AD_ENABLE: u8 = 1u8 << 7;
pub const TPS6507X_ADCONFIG_START_CONVERSION: u8 = 1u8 << 6;
pub const TPS6507X_ADCONFIG_CONVERSION_DONE: u8 = 1u8 << 5;
pub const TPS6507X_ADCONFIG_VREF_ENABLE: u8 = 1u8 << 4;
pub const TPS6507X_ADCONFIG_INPUT_AD_IN1: u8 = 0;
pub const TPS6507X_ADCONFIG_INPUT_AD_IN2: u8 = 1;
pub const TPS6507X_ADCONFIG_INPUT_AD_IN3: u8 = 2;
pub const TPS6507X_ADCONFIG_INPUT_AD_IN4: u8 = 3;
pub const TPS6507X_ADCONFIG_INPUT_TS_PIN: u8 = 4;
pub const TPS6507X_ADCONFIG_INPUT_BAT_CURRENT: u8 = 5;
pub const TPS6507X_ADCONFIG_INPUT_AC_VOLTAGE: u8 = 6;
pub const TPS6507X_ADCONFIG_INPUT_SYS_VOLTAGE: u8 = 7;
pub const TPS6507X_ADCONFIG_INPUT_CHARGER_VOLTAGE: u8 = 8;
pub const TPS6507X_ADCONFIG_INPUT_BAT_VOLTAGE: u8 = 9;
pub const TPS6507X_ADCONFIG_INPUT_THRESHOLD_VOLTAGE: u8 = 10;
pub const TPS6507X_ADCONFIG_INPUT_ISET1_VOLTAGE: u8 = 11;
pub const TPS6507X_ADCONFIG_INPUT_ISET2_VOLTAGE: u8 = 12;
pub const TPS6507X_ADCONFIG_INPUT_REAL_TSC: u8 = 14;
pub const TPS6507X_ADCONFIG_INPUT_TSC: u8 = 15;

pub const TPS6507X_REG_TSCMODE: u8 = 0x08;
pub const TPS6507X_TSCMODE_X_POSITION: u8 = 0;
pub const TPS6507X_TSCMODE_Y_POSITION: u8 = 1;
pub const TPS6507X_TSCMODE_PRESSURE: u8 = 2;
pub const TPS6507X_TSCMODE_X_PLATE: u8 = 3;
pub const TPS6507X_TSCMODE_Y_PLATE: u8 = 4;
pub const TPS6507X_TSCMODE_STANDBY: u8 = 5;
pub const TPS6507X_TSCMODE_ADC_INPUT: u8 = 6;
pub const TPS6507X_TSCMODE_DISABLE: u8 = 7;

pub const TPS6507X_REG_ADRESULT_1: u8 = 0x09;
pub const TPS6507X_REG_ADRESULT_2: u8 = 0x0A;
pub const TPS6507X_REG_ADRESULT_2_MASK: u8 = (1u8 << 1) | (1u8 << 0);
pub const TPS6507X_REG_PGOOD: u8 = 0x0B;
pub const TPS6507X_REG_PGOODMASK: u8 = 0x0C;
pub const TPS6507X_REG_CON_CTRL1: u8 = 0x0D;
pub const TPS6507X_REG_CON_CTRL2: u8 = 0x0E;
pub const TPS6507X_REG_CON_CTRL3: u8 = 0x0F;
pub const TPS6507X_REG_DEFDCDC1: u8 = 0x10;
pub const TPS6507X_DEFDCDC1_DCDC1_EXT_ADJ_EN: u8 = 1u8 << 7;
pub const TPS6507X_DEFDCDC1_DCDC1_MASK: u8 = 0x3F;
pub const TPS6507X_REG_DEFDCDC2_LOW: u8 = 0x11;
pub const TPS6507X_DEFDCDC2_LOW_DCDC2_MASK: u8 = 0x3F;
pub const TPS6507X_REG_DEFDCDC2_HIGH: u8 = 0x12;
pub const TPS6507X_DEFDCDC2_HIGH_DCDC2_MASK: u8 = 0x3F;
pub const TPS6507X_REG_DEFDCDC3_LOW: u8 = 0x13;
pub const TPS6507X_DEFDCDC3_LOW_DCDC3_MASK: u8 = 0x3F;
pub const TPS6507X_REG_DEFDCDC3_HIGH: u8 = 0x14;
pub const TPS6507X_DEFDCDC3_HIGH_DCDC3_MASK: u8 = 0x3F;
pub const TPS6507X_REG_DEFSLEW: u8 = 0x15;
pub const TPS6507X_REG_LDO_CTRL1: u8 = 0x16;
pub const TPS6507X_REG_LDO_CTRL1_LDO1_MASK: u8 = 0x0F;
pub const TPS6507X_REG_DEFLDO2: u8 = 0x17;
pub const TPS6507X_REG_DEFLDO2_LDO2_MASK: u8 = 0x3F;
pub const TPS6507X_REG_WLED_CTRL1: u8 = 0x18;
pub const TPS6507X_REG_WLED_CTRL2: u8 = 0x19;
pub const TPS6507X_DEFDCDCX_DCDC_MASK: u8 = 0x3F;
pub const TPS6507X_MAX_REGISTER: u8 = 0x19;

// External kernel types supplied by other translation units.
pub enum regulator_init_data {}
pub enum touchscreen_init_data {}
pub enum device {}
pub enum i2c_client {}
pub enum tps6507x_pmic {}

#[repr(C)]
pub struct tps6507x_board {
    pub tps6507x_pmic_init_data: *mut regulator_init_data,
    pub tps6507x_ts_init_data: *mut touchscreen_init_data,
}

#[repr(C)]
pub struct tps6507x_dev {
    pub dev: *mut device,
    pub i2c_client: *mut i2c_client,
    pub read_dev: Option<unsafe extern "C" fn(*mut tps6507x_dev, i8, i32, *mut core::ffi::c_void) -> i32>,
    pub write_dev: Option<unsafe extern "C" fn(*mut tps6507x_dev, i8, i32, *mut core::ffi::c_void) -> i32>,
    pub pmic: *mut tps6507x_pmic,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
