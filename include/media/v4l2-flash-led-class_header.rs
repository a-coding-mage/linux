/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * V4L2 flash LED sub-device registration helpers.
 *
 *	Copyright (C) 2015 Samsung Electronics Co., Ltd
 *	Author: Jacek Anaszewski <j.anaszewski@samsung.com>
 */

// Types and macros referenced here are supplied by the surrounding kernel translation.

/**
 * struct v4l2_flash_ctrl_data - flash control initialization data, filled
 *				basing on the features declared by the LED flash
 *				class driver in the v4l2_flash_config
 * @config:	initialization data for a control
 * @cid:	contains v4l2 flash control id if the config
 *		field was initialized, 0 otherwise
 */
#[repr(C)]
pub struct v4l2_flash_ctrl_data {
    pub config: v4l2_ctrl_config,
    pub cid: u32,
}

/** V4L2 flash operations. */
#[repr(C)]
pub struct v4l2_flash_ops {
    pub external_strobe_set:
        Option<unsafe extern "C" fn(v4l2_flash: *mut v4l2_flash, enable: bool) -> i32>,
    pub intensity_to_led_brightness:
        Option<unsafe extern "C" fn(v4l2_flash: *mut v4l2_flash, intensity: i32) -> led_brightness>,
    pub led_brightness_to_intensity:
        Option<unsafe extern "C" fn(v4l2_flash: *mut v4l2_flash, brightness: led_brightness) -> i32>,
}

/** V4L2 Flash sub-device initialization data. */
#[repr(C)]
pub struct v4l2_flash_config {
    pub dev_name: [core::ffi::c_char; 32],
    pub intensity: led_flash_setting,
    pub flash_faults: u32,
    pub has_external_strobe: u32,
}

/** Flash sub-device context. */
#[repr(C)]
pub struct v4l2_flash {
    pub fled_cdev: *mut led_classdev_flash,
    pub iled_cdev: *mut led_classdev,
    pub ops: *const v4l2_flash_ops,
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub ctrls: *mut *mut v4l2_ctrl,
}

pub unsafe fn v4l2_subdev_to_v4l2_flash(sd: *mut v4l2_subdev) -> *mut v4l2_flash {
    container_of!(sd, v4l2_flash, sd)
}

pub unsafe fn v4l2_ctrl_to_v4l2_flash(c: *mut v4l2_ctrl) -> *mut v4l2_flash {
    container_of!((*c).handler, v4l2_flash, hdl)
}

// When CONFIG_V4L2_FLASH_LED_CLASS is enabled, these are external functions.
#[cfg(feature = "CONFIG_V4L2_FLASH_LED_CLASS")]
extern "C" {
    pub fn v4l2_flash_init(
        dev: *mut device,
        fwn: *mut fwnode_handle,
        fled_cdev: *mut led_classdev_flash,
        ops: *const v4l2_flash_ops,
        config: *mut v4l2_flash_config,
    ) -> *mut v4l2_flash;
    pub fn v4l2_flash_indicator_init(
        dev: *mut device,
        fwn: *mut fwnode_handle,
        iled_cdev: *mut led_classdev,
        config: *mut v4l2_flash_config,
    ) -> *mut v4l2_flash;
    pub fn v4l2_flash_release(v4l2_flash: *mut v4l2_flash);
}

// Fallback definitions when CONFIG_V4L2_FLASH_LED_CLASS is disabled.
#[cfg(not(feature = "CONFIG_V4L2_FLASH_LED_CLASS"))]
pub unsafe fn v4l2_flash_init(
    _dev: *mut device,
    _fwn: *mut fwnode_handle,
    _fled_cdev: *mut led_classdev_flash,
    _ops: *const v4l2_flash_ops,
    _config: *mut v4l2_flash_config,
) -> *mut v4l2_flash {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_V4L2_FLASH_LED_CLASS"))]
pub unsafe fn v4l2_flash_indicator_init(
    _dev: *mut device,
    _fwn: *mut fwnode_handle,
    _iled_cdev: *mut led_classdev,
    _config: *mut v4l2_flash_config,
) -> *mut v4l2_flash {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_V4L2_FLASH_LED_CLASS"))]
pub unsafe fn v4l2_flash_release(_v4l2_flash: *mut v4l2_flash) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
