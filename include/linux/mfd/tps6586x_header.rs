/* SPDX-License-Identifier: GPL-2.0 */

pub const TPS6586X_SLEW_RATE_INSTANTLY: i32 = 0x00;
pub const TPS6586X_SLEW_RATE_110UV: i32 = 0x01;
pub const TPS6586X_SLEW_RATE_220UV: i32 = 0x02;
pub const TPS6586X_SLEW_RATE_440UV: i32 = 0x03;
pub const TPS6586X_SLEW_RATE_880UV: i32 = 0x04;
pub const TPS6586X_SLEW_RATE_1760UV: i32 = 0x05;
pub const TPS6586X_SLEW_RATE_3520UV: i32 = 0x06;
pub const TPS6586X_SLEW_RATE_7040UV: i32 = 0x07;

pub const TPS6586X_SLEW_RATE_SET: i32 = 0x08;
pub const TPS6586X_SLEW_RATE_MASK: i32 = 0x07;

/* VERSION CRC */
pub const TPS658621A: i32 = 0x15;
pub const TPS658621CD: i32 = 0x2c;
pub const TPS658623: i32 = 0x1b;
pub const TPS658624: i32 = 0x0a;
pub const TPS658640: i32 = 0x01;
pub const TPS658640v2: i32 = 0x02;
pub const TPS658643: i32 = 0x03;

pub const TPS6586X_ID_SYS: i32 = 0;
pub const TPS6586X_ID_SM_0: i32 = 1;
pub const TPS6586X_ID_SM_1: i32 = 2;
pub const TPS6586X_ID_SM_2: i32 = 3;
pub const TPS6586X_ID_LDO_0: i32 = 4;
pub const TPS6586X_ID_LDO_1: i32 = 5;
pub const TPS6586X_ID_LDO_2: i32 = 6;
pub const TPS6586X_ID_LDO_3: i32 = 7;
pub const TPS6586X_ID_LDO_4: i32 = 8;
pub const TPS6586X_ID_LDO_5: i32 = 9;
pub const TPS6586X_ID_LDO_6: i32 = 10;
pub const TPS6586X_ID_LDO_7: i32 = 11;
pub const TPS6586X_ID_LDO_8: i32 = 12;
pub const TPS6586X_ID_LDO_9: i32 = 13;
pub const TPS6586X_ID_LDO_RTC: i32 = 14;
pub const TPS6586X_ID_MAX_REGULATOR: i32 = 15;

pub const TPS6586X_INT_PLDO_0: i32 = 0;
pub const TPS6586X_INT_PLDO_1: i32 = 1;
pub const TPS6586X_INT_PLDO_2: i32 = 2;
pub const TPS6586X_INT_PLDO_3: i32 = 3;
pub const TPS6586X_INT_PLDO_4: i32 = 4;
pub const TPS6586X_INT_PLDO_5: i32 = 5;
pub const TPS6586X_INT_PLDO_6: i32 = 6;
pub const TPS6586X_INT_PLDO_7: i32 = 7;
pub const TPS6586X_INT_COMP_DET: i32 = 8;
pub const TPS6586X_INT_ADC: i32 = 9;
pub const TPS6586X_INT_PLDO_8: i32 = 10;
pub const TPS6586X_INT_PLDO_9: i32 = 11;
pub const TPS6586X_INT_PSM_0: i32 = 12;
pub const TPS6586X_INT_PSM_1: i32 = 13;
pub const TPS6586X_INT_PSM_2: i32 = 14;
pub const TPS6586X_INT_PSM_3: i32 = 15;
pub const TPS6586X_INT_RTC_ALM1: i32 = 16;
pub const TPS6586X_INT_ACUSB_OVP: i32 = 17;
pub const TPS6586X_INT_USB_DET: i32 = 18;
pub const TPS6586X_INT_AC_DET: i32 = 19;
pub const TPS6586X_INT_BAT_DET: i32 = 20;
pub const TPS6586X_INT_CHG_STAT: i32 = 21;
pub const TPS6586X_INT_CHG_TEMP: i32 = 22;
pub const TPS6586X_INT_PP: i32 = 23;
pub const TPS6586X_INT_RESUME: i32 = 24;
pub const TPS6586X_INT_LOW_SYS: i32 = 25;
pub const TPS6586X_INT_RTC_ALM2: i32 = 26;

#[repr(C)]
pub struct tps6586x_settings {
    pub slew_rate: i32,
}

#[repr(C)]
pub struct tps6586x_subdev_info {
    pub id: i32,
    pub name: *const core::ffi::c_char,
    pub platform_data: *mut core::ffi::c_void,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct tps6586x_platform_data {
    pub num_subdevs: i32,
    pub subdevs: *mut tps6586x_subdev_info,
    pub gpio_base: i32,
    pub irq_base: i32,
    pub pm_off: bool,
    pub reg_init_data: [*mut regulator_init_data; TPS6586X_ID_MAX_REGULATOR as usize],
}

/*
 * NOTE: the functions below are not intended for use outside
 * of the TPS6586X sub-device drivers
 */
unsafe extern "C" {
    pub fn tps6586x_write(dev: *mut device, reg: i32, val: u8) -> i32;
    pub fn tps6586x_writes(dev: *mut device, reg: i32, len: i32, val: *mut u8) -> i32;
    pub fn tps6586x_read(dev: *mut device, reg: i32, val: *mut u8) -> i32;
    pub fn tps6586x_reads(dev: *mut device, reg: i32, len: i32, val: *mut u8) -> i32;
    pub fn tps6586x_set_bits(dev: *mut device, reg: i32, bit_mask: u8) -> i32;
    pub fn tps6586x_clr_bits(dev: *mut device, reg: i32, bit_mask: u8) -> i32;
    pub fn tps6586x_update(dev: *mut device, reg: i32, val: u8, mask: u8) -> i32;
    pub fn tps6586x_irq_get_virq(dev: *mut device, irq: i32) -> i32;
    pub fn tps6586x_get_version(dev: *mut device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
