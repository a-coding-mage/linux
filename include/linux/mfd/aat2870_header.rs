/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/include/linux/mfd/aat2870.h
 *
 * Copyright (c) 2011, NVIDIA Corporation.
 * Author: Jin Park <jinyoungp@nvidia.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/debugfs.h and linux/i2c.h

/* Register offsets */
pub const AAT2870_BL_CH_EN: u8 = 0x00;
pub const AAT2870_BLM: u8 = 0x01;
pub const AAT2870_BLS: u8 = 0x02;
pub const AAT2870_BL1: u8 = 0x03;
pub const AAT2870_BL2: u8 = 0x04;
pub const AAT2870_BL3: u8 = 0x05;
pub const AAT2870_BL4: u8 = 0x06;
pub const AAT2870_BL5: u8 = 0x07;
pub const AAT2870_BL6: u8 = 0x08;
pub const AAT2870_BL7: u8 = 0x09;
pub const AAT2870_BL8: u8 = 0x0A;
pub const AAT2870_FLR: u8 = 0x0B;
pub const AAT2870_FM: u8 = 0x0C;
pub const AAT2870_FS: u8 = 0x0D;
pub const AAT2870_ALS_CFG0: u8 = 0x0E;
pub const AAT2870_ALS_CFG1: u8 = 0x0F;
pub const AAT2870_ALS_CFG2: u8 = 0x10;
pub const AAT2870_AMB: u8 = 0x11;
pub const AAT2870_ALS0: u8 = 0x12;
pub const AAT2870_ALS1: u8 = 0x13;
pub const AAT2870_ALS2: u8 = 0x14;
pub const AAT2870_ALS3: u8 = 0x15;
pub const AAT2870_ALS4: u8 = 0x16;
pub const AAT2870_ALS5: u8 = 0x17;
pub const AAT2870_ALS6: u8 = 0x18;
pub const AAT2870_ALS7: u8 = 0x19;
pub const AAT2870_ALS8: u8 = 0x1A;
pub const AAT2870_ALS9: u8 = 0x1B;
pub const AAT2870_ALSA: u8 = 0x1C;
pub const AAT2870_ALSB: u8 = 0x1D;
pub const AAT2870_ALSC: u8 = 0x1E;
pub const AAT2870_ALSD: u8 = 0x1F;
pub const AAT2870_ALSE: u8 = 0x20;
pub const AAT2870_ALSF: u8 = 0x21;
pub const AAT2870_SUB_SET: u8 = 0x22;
pub const AAT2870_SUB_CTRL: u8 = 0x23;
pub const AAT2870_LDO_AB: u8 = 0x24;
pub const AAT2870_LDO_CD: u8 = 0x25;
pub const AAT2870_LDO_EN: u8 = 0x26;
pub const AAT2870_REG_NUM: u8 = 0x27;

/* Device IDs */
#[repr(C)]
pub enum aat2870_id {
    AAT2870_ID_BL,
    AAT2870_ID_LDOA,
    AAT2870_ID_LDOB,
    AAT2870_ID_LDOC,
    AAT2870_ID_LDOD,
}

/* Backlight channels */
pub const AAT2870_BL_CH1: u8 = 0x01;
pub const AAT2870_BL_CH2: u8 = 0x02;
pub const AAT2870_BL_CH3: u8 = 0x04;
pub const AAT2870_BL_CH4: u8 = 0x08;
pub const AAT2870_BL_CH5: u8 = 0x10;
pub const AAT2870_BL_CH6: u8 = 0x20;
pub const AAT2870_BL_CH7: u8 = 0x40;
pub const AAT2870_BL_CH8: u8 = 0x80;
pub const AAT2870_BL_CH_ALL: u8 = 0xFF;

/* Backlight current magnitude (mA) */
#[repr(C)]
pub enum aat2870_current {
    AAT2870_CURRENT_0_45 = 1,
    AAT2870_CURRENT_0_90,
    AAT2870_CURRENT_1_80,
    AAT2870_CURRENT_2_70,
    AAT2870_CURRENT_3_60,
    AAT2870_CURRENT_4_50,
    AAT2870_CURRENT_5_40,
    AAT2870_CURRENT_6_30,
    AAT2870_CURRENT_7_20,
    AAT2870_CURRENT_8_10,
    AAT2870_CURRENT_9_00,
    AAT2870_CURRENT_9_90,
    AAT2870_CURRENT_10_8,
    AAT2870_CURRENT_11_7,
    AAT2870_CURRENT_12_6,
    AAT2870_CURRENT_13_5,
    AAT2870_CURRENT_14_4,
    AAT2870_CURRENT_15_3,
    AAT2870_CURRENT_16_2,
    AAT2870_CURRENT_17_1,
    AAT2870_CURRENT_18_0,
    AAT2870_CURRENT_18_9,
    AAT2870_CURRENT_19_8,
    AAT2870_CURRENT_20_7,
    AAT2870_CURRENT_21_6,
    AAT2870_CURRENT_22_5,
    AAT2870_CURRENT_23_4,
    AAT2870_CURRENT_24_3,
    AAT2870_CURRENT_25_2,
    AAT2870_CURRENT_26_1,
    AAT2870_CURRENT_27_0,
    AAT2870_CURRENT_27_9,
}

#[repr(C)]
pub struct aat2870_register {
    pub readable: bool,
    pub writeable: bool,
    pub value: u8,
}

#[repr(C)]
pub struct aat2870_data {
    pub dev: *mut device,
    pub client: *mut i2c_client,
    pub io_lock: mutex,
    pub reg_cache: *mut aat2870_register, /* register cache */
    pub en_pin: i32, /* enable GPIO pin (if < 0, ignore this value) */
    pub is_enable: bool,
    /* init and uninit for platform specified */
    pub init: Option<unsafe extern "C" fn(*mut aat2870_data) -> i32>,
    pub uninit: Option<unsafe extern "C" fn(*mut aat2870_data)>,
    /* i2c io funcntions */
    pub read: Option<unsafe extern "C" fn(*mut aat2870_data, u8, *mut u8) -> i32>,
    pub write: Option<unsafe extern "C" fn(*mut aat2870_data, u8, u8) -> i32>,
    pub update: Option<unsafe extern "C" fn(*mut aat2870_data, u8, u8, u8) -> i32>,
}

#[repr(C)]
pub struct aat2870_subdev_info {
    pub id: i32,
    pub name: *const core::ffi::c_char,
    pub platform_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct aat2870_platform_data {
    pub en_pin: i32, /* enable GPIO pin (if < 0, ignore this value) */
    pub subdevs: *mut aat2870_subdev_info,
    pub num_subdevs: i32,
    /* init and uninit for platform specified */
    pub init: Option<unsafe extern "C" fn(*mut aat2870_data) -> i32>,
    pub uninit: Option<unsafe extern "C" fn(*mut aat2870_data)>,
}

#[repr(C)]
pub struct aat2870_bl_platform_data {
    /* backlight channels, default is AAT2870_BL_CH_ALL */
    pub channels: i32,
    /* backlight current magnitude, default is AAT2870_CURRENT_27_9 */
    pub max_current: i32,
    /* maximum brightness, default is 255 */
    pub max_brightness: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
