/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * LED Flash class interface
 *
 * Copyright (C) 2015 Samsung Electronics Co., Ltd.
 * Author: Jacek Anaszewski <j.anaszewski@samsung.com>
 */

// Dependency supplied by the Linux LED subsystem.
use crate::linux::leds::{attribute_group, led_classdev, led_init_data};

pub struct device;
pub struct led_classdev_flash;

pub const LED_FAULT_OVER_VOLTAGE: u32 = 1 << 0;
pub const LED_FAULT_TIMEOUT: u32 = 1 << 1;
pub const LED_FAULT_OVER_TEMPERATURE: u32 = 1 << 2;
pub const LED_FAULT_SHORT_CIRCUIT: u32 = 1 << 3;
pub const LED_FAULT_OVER_CURRENT: u32 = 1 << 4;
pub const LED_FAULT_INDICATOR: u32 = 1 << 5;
pub const LED_FAULT_UNDER_VOLTAGE: u32 = 1 << 6;
pub const LED_FAULT_INPUT_VOLTAGE: u32 = 1 << 7;
pub const LED_FAULT_LED_OVER_TEMPERATURE: u32 = 1 << 8;
pub const LED_NUM_FLASH_FAULTS: u32 = 9;
pub const LED_FLASH_SYSFS_GROUPS_SIZE: usize = 5;

#[repr(C)]
pub struct led_flash_ops {
    /* set flash brightness */
    pub flash_brightness_set: Option<unsafe extern "C" fn(*mut led_classdev_flash, u32) -> i32>,
    /* get flash brightness */
    pub flash_brightness_get: Option<unsafe extern "C" fn(*mut led_classdev_flash, *mut u32) -> i32>,
    /* set flash strobe state */
    pub strobe_set: Option<unsafe extern "C" fn(*mut led_classdev_flash, bool) -> i32>,
    /* get flash strobe state */
    pub strobe_get: Option<unsafe extern "C" fn(*mut led_classdev_flash, *mut bool) -> i32>,
    /* set flash timeout */
    pub timeout_set: Option<unsafe extern "C" fn(*mut led_classdev_flash, u32) -> i32>,
    /* get the flash LED fault */
    pub fault_get: Option<unsafe extern "C" fn(*mut led_classdev_flash, *mut u32) -> i32>,
    /* set flash duration */
    pub duration_set: Option<unsafe extern "C" fn(*mut led_classdev_flash, u32) -> i32>,
}

#[repr(C)]
pub struct led_flash_setting {
    /* maximum allowed value */
    pub min: u32,
    /* maximum allowed value */
    pub max: u32,
    /* step value */
    pub step: u32,
    /* current value */
    pub val: u32,
}

#[repr(C)]
pub struct led_classdev_flash {
    /* led class device */
    pub led_cdev: led_classdev,
    /* flash led specific ops */
    pub ops: *const led_flash_ops,
    /* flash brightness value in microamperes along with its constraints */
    pub brightness: led_flash_setting,
    /* flash timeout value in microseconds along with its constraints */
    pub timeout: led_flash_setting,
    /* flash timeout value in microseconds along with its constraints */
    pub duration: led_flash_setting,
    /* LED Flash class sysfs groups */
    pub sysfs_groups: [*const attribute_group; LED_FLASH_SYSFS_GROUPS_SIZE],
}

pub unsafe fn lcdev_to_flcdev(lcdev: *mut led_classdev) -> *mut led_classdev_flash {
    // led_cdev is the first member, matching container_of for this layout.
    lcdev as *mut led_classdev_flash
}

extern "C" {
    pub fn led_classdev_flash_register_ext(parent: *mut device, fled_cdev: *mut led_classdev_flash, init_data: *mut led_init_data) -> i32;
    pub fn led_classdev_flash_unregister(fled_cdev: *mut led_classdev_flash);
    pub fn devm_led_classdev_flash_register_ext(parent: *mut device, fled_cdev: *mut led_classdev_flash, init_data: *mut led_init_data) -> i32;
    pub fn devm_led_classdev_flash_unregister(parent: *mut device, fled_cdev: *mut led_classdev_flash);
    pub fn led_set_flash_brightness(fled_cdev: *mut led_classdev_flash, brightness: u32) -> i32;
    pub fn led_update_flash_brightness(fled_cdev: *mut led_classdev_flash) -> i32;
    pub fn led_set_flash_timeout(fled_cdev: *mut led_classdev_flash, timeout: u32) -> i32;
    pub fn led_get_flash_fault(fled_cdev: *mut led_classdev_flash, fault: *mut u32) -> i32;
    pub fn led_set_flash_duration(fled_cdev: *mut led_classdev_flash, duration: u32) -> i32;
}

pub unsafe fn led_classdev_flash_register(parent: *mut device, fled_cdev: *mut led_classdev_flash) -> i32 {
    led_classdev_flash_register_ext(parent, fled_cdev, core::ptr::null_mut())
}

pub unsafe fn devm_led_classdev_flash_register(parent: *mut device, fled_cdev: *mut led_classdev_flash) -> i32 {
    devm_led_classdev_flash_register_ext(parent, fled_cdev, core::ptr::null_mut())
}

pub unsafe fn led_set_flash_strobe(fled_cdev: *mut led_classdev_flash, state: bool) -> i32 {
    if fled_cdev.is_null() { return -22; }
    ((*(*fled_cdev).ops).strobe_set.unwrap())(fled_cdev, state)
}

pub unsafe fn led_get_flash_strobe(fled_cdev: *mut led_classdev_flash, state: *mut bool) -> i32 {
    if fled_cdev.is_null() { return -22; }
    match (*(*fled_cdev).ops).strobe_get {
        Some(get) => get(fled_cdev, state),
        None => -22,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
