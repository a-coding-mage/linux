/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/media/i2c/adp1653.h
 *
 * Copyright (C) 2008--2011 Nokia Corporation
 *
 * Contact: Sakari Ailus <sakari.ailus@iki.fi>
 *
 * Contributors:
 *\tSakari Ailus <sakari.ailus@iki.fi>
 *\tTuukka Toivonen <tuukkat76@gmail.com>
 */

// C dependencies: linux/i2c.h, linux/mutex.h, linux/videodev2.h,
// media/v4l2-ctrls.h, and media/v4l2-subdev.h.

pub const ADP1653_NAME: &str = "adp1653";
pub const ADP1653_I2C_ADDR: u32 = 0x60 >> 1;

/* Register definitions */
pub const ADP1653_REG_OUT_SEL: u32 = 0x00;
pub const ADP1653_REG_OUT_SEL_HPLED_TORCH_MIN: u32 = 0x01;
pub const ADP1653_REG_OUT_SEL_HPLED_TORCH_MAX: u32 = 0x0b;
pub const ADP1653_REG_OUT_SEL_HPLED_FLASH_MIN: u32 = 0x0c;
pub const ADP1653_REG_OUT_SEL_HPLED_FLASH_MAX: u32 = 0x1f;
pub const ADP1653_REG_OUT_SEL_HPLED_SHIFT: u32 = 3;
pub const ADP1653_REG_OUT_SEL_ILED_MAX: u32 = 0x07;
pub const ADP1653_REG_OUT_SEL_ILED_SHIFT: u32 = 0;

pub const ADP1653_REG_CONFIG: u32 = 0x01;
pub const ADP1653_REG_CONFIG_TMR_CFG: u32 = 1 << 4;
pub const ADP1653_REG_CONFIG_TMR_SET_MAX: u32 = 0x0f;
pub const ADP1653_REG_CONFIG_TMR_SET_SHIFT: u32 = 0;

pub const ADP1653_REG_SW_STROBE: u32 = 0x02;
pub const ADP1653_REG_SW_STROBE_SW_STROBE: u32 = 1 << 0;

pub const ADP1653_REG_FAULT: u32 = 0x03;
pub const ADP1653_REG_FAULT_FLT_SCP: u32 = 1 << 3;
pub const ADP1653_REG_FAULT_FLT_OT: u32 = 1 << 2;
pub const ADP1653_REG_FAULT_FLT_TMR: u32 = 1 << 1;
pub const ADP1653_REG_FAULT_FLT_OV: u32 = 1 << 0;

pub const ADP1653_INDICATOR_INTENSITY_MIN: u32 = 0;
pub const ADP1653_INDICATOR_INTENSITY_STEP: u32 = 2500;
pub const ADP1653_INDICATOR_INTENSITY_MAX: u32 =
    ADP1653_REG_OUT_SEL_ILED_MAX * ADP1653_INDICATOR_INTENSITY_STEP;

#[inline]
pub const fn adp1653_indicator_intensity_ua_to_reg(a: u32) -> u32 {
    a / ADP1653_INDICATOR_INTENSITY_STEP
}

#[inline]
pub const fn adp1653_indicator_intensity_reg_to_ua(a: u32) -> u32 {
    a * ADP1653_INDICATOR_INTENSITY_STEP
}

pub const ADP1653_FLASH_INTENSITY_BASE: u32 = 35;
pub const ADP1653_FLASH_INTENSITY_STEP: u32 = 15;
pub const ADP1653_FLASH_INTENSITY_MIN: u32 = ADP1653_FLASH_INTENSITY_BASE
    + ADP1653_REG_OUT_SEL_HPLED_FLASH_MIN * ADP1653_FLASH_INTENSITY_STEP;
pub const ADP1653_FLASH_INTENSITY_MAX: u32 = ADP1653_FLASH_INTENSITY_MIN
    + (ADP1653_REG_OUT_SEL_HPLED_FLASH_MAX - ADP1653_REG_OUT_SEL_HPLED_FLASH_MIN + 1)
        * ADP1653_FLASH_INTENSITY_STEP;

#[inline]
pub const fn adp1653_flash_intensity_ma_to_reg(a: u32) -> u32 {
    if a < ADP1653_FLASH_INTENSITY_BASE {
        0
    } else {
        (a - ADP1653_FLASH_INTENSITY_BASE) / ADP1653_FLASH_INTENSITY_STEP
    }
}

#[inline]
pub const fn adp1653_flash_intensity_reg_to_ma(a: u32) -> u32 {
    a * ADP1653_FLASH_INTENSITY_STEP + ADP1653_FLASH_INTENSITY_BASE
}

pub const ADP1653_TORCH_INTENSITY_MIN: u32 = ADP1653_FLASH_INTENSITY_BASE
    + ADP1653_REG_OUT_SEL_HPLED_TORCH_MIN * ADP1653_FLASH_INTENSITY_STEP;
pub const ADP1653_TORCH_INTENSITY_MAX: u32 = ADP1653_TORCH_INTENSITY_MIN
    + (ADP1653_REG_OUT_SEL_HPLED_TORCH_MAX - ADP1653_REG_OUT_SEL_HPLED_TORCH_MIN + 1)
        * ADP1653_FLASH_INTENSITY_STEP;

#[repr(C)]
pub struct adp1653_platform_data {
    pub power: Option<unsafe extern "C" fn(sd: *mut v4l2_subdev, on: core::ffi::c_int) -> core::ffi::c_int>,
    pub max_flash_timeout: u32,     /* flash light timeout in us */
    pub max_flash_intensity: u32,   /* led intensity, flash mode, mA */
    pub max_torch_intensity: u32,   /* led intensity, torch mode, mA */
    pub max_indicator_intensity: u32, /* indicator led intensity, uA */
    pub enable_gpio: *mut gpio_desc, /* for device-tree based boot */
}

// Equivalent to the C container_of(sd, struct adp1653_flash, subdev).
#[macro_export]
macro_rules! to_adp1653_flash {
    ($sd:expr) => {
        container_of!($sd, adp1653_flash, subdev)
    };
}

#[repr(C)]
pub struct adp1653_flash {
    pub subdev: v4l2_subdev,
    pub platform_data: *mut adp1653_platform_data,
    pub ctrls: v4l2_ctrl_handler,
    pub led_mode: *mut v4l2_ctrl,
    pub flash_timeout: *mut v4l2_ctrl,
    pub flash_intensity: *mut v4l2_ctrl,
    pub torch_intensity: *mut v4l2_ctrl,
    pub indicator_intensity: *mut v4l2_ctrl,
    pub power_lock: mutex,
    pub power_count: core::ffi::c_int,
    pub fault: core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
