/* SPDX-License-Identifier: GPL-2.0 */
/*
 * LCD Lowlevel Control Abstraction
 *
 * Copyright (C) 2003,2004 Hewlett-Packard Company
 */

// Dependencies supplied by the surrounding kernel translation.

pub const LCD_POWER_ON: i32 = 0;
pub const LCD_POWER_REDUCED: i32 = 1; // deprecated; don't use in new code
pub const LCD_POWER_REDUCED_VSYNC_SUSPEND: i32 = 2; // deprecated; don't use in new code
pub const LCD_POWER_OFF: i32 = 4;

#[repr(C)]
pub struct lcd_properties {
    /* The maximum value for contrast (read-only) */
    pub max_contrast: i32,
}

#[repr(C)]
pub struct lcd_ops {
    /* Get the LCD panel power status (0: full on, 1..3: controller
       power on, flat panel power off, 4: full off), see FB_BLANK_XXX */
    pub get_power: Option<unsafe extern "C" fn(*mut lcd_device) -> i32>,
    /* Enable or disable power to the LCD (0: on; 4: off, see FB_BLANK_XXX) */
    pub set_power: Option<unsafe extern "C" fn(*mut lcd_device, i32) -> i32>,
    /* Get the current contrast setting (0-max_contrast) */
    pub get_contrast: Option<unsafe extern "C" fn(*mut lcd_device) -> i32>,
    /* Set LCD panel contrast */
    pub set_contrast: Option<unsafe extern "C" fn(*mut lcd_device, i32) -> i32>,
    /* Set LCD panel mode (resolutions ...) */
    pub set_mode: Option<unsafe extern "C" fn(*mut lcd_device, u32, u32) -> i32>,
    /* Check if the LCD controls the given display device. */
    pub controls_device:
        Option<unsafe extern "C" fn(*mut lcd_device, *mut device) -> bool>,
}

#[repr(C)]
pub struct lcd_device {
    pub props: lcd_properties,
    /* This protects the 'ops' field. */
    pub ops_lock: mutex,
    /* If this is NULL, the backing module is unloaded */
    pub ops: *const lcd_ops,
    /* Serialise access to set_power method */
    pub update_lock: mutex,
    /** @entry: List entry of all registered lcd devices */
    pub entry: list_head,
    pub dev: device,
}

#[repr(C)]
pub struct lcd_platform_data {
    /* reset lcd panel device. */
    pub reset: Option<unsafe extern "C" fn(*mut lcd_device) -> i32>,
    /* on or off to lcd panel. */
    pub power_on: Option<unsafe extern "C" fn(*mut lcd_device, i32) -> i32>,
    /* it indicates whether lcd panel was enabled from bootloader or not. */
    pub lcd_enabled: i32,
    pub reset_delay: u32,
    pub power_on_delay: u32,
    pub power_off_delay: u32,
    /* it could be used for any purpose. */
    pub pdata: *mut core::ffi::c_void,
}

pub unsafe fn lcd_set_power(ld: *mut lcd_device, power: i32) {
    mutex_lock(&mut (*ld).update_lock);
    if !(*ld).ops.is_null() {
        let ops = &*(*ld).ops;
        if let Some(set_power) = ops.set_power {
            set_power(ld, power);
        }
    }
    mutex_unlock(&mut (*ld).update_lock);
}

unsafe extern "C" {
    pub fn lcd_device_register(
        name: *const core::ffi::c_char,
        parent: *mut device,
        devdata: *mut core::ffi::c_void,
        ops: *const lcd_ops,
    ) -> *mut lcd_device;
    pub fn devm_lcd_device_register(
        dev: *mut device,
        name: *const core::ffi::c_char,
        parent: *mut device,
        devdata: *mut core::ffi::c_void,
        ops: *const lcd_ops,
    ) -> *mut lcd_device;
    pub fn lcd_device_unregister(ld: *mut lcd_device);
    pub fn devm_lcd_device_unregister(dev: *mut device, ld: *mut lcd_device);
    pub fn lcd_notify_blank_all(display_dev: *mut device, power: i32);
    pub fn lcd_notify_mode_change_all(display_dev: *mut device, width: u32, height: u32);
}

// In the C header these notifications are empty when
// IS_REACHABLE(CONFIG_LCD_CLASS_DEVICE) is false.

pub unsafe fn lcd_get_data(ld_dev: *mut lcd_device) -> *mut core::ffi::c_void {
    dev_get_drvdata(&mut (*ld_dev).dev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
