/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/mfd/wm831x/pdata.h -- Platform data for WM831x
 *
 * Copyright 2009 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// Forward declarations supplied by the surrounding translation unit.
#[repr(C)]
pub struct wm831x {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator_init_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wm831x_backlight_pdata {
    pub isink: i32,     // ISINK to use, 1 or 2
    pub max_uA: i32,    // Maximum current to allow
}

#[repr(C)]
pub struct wm831x_backup_pdata {
    pub charger_enable: i32,
    pub no_constant_voltage: i32,  // Disable constant voltage charging
    pub vlim: i32,   // Voltage limit in millivolts
    pub ilim: i32,   // Current limit in microamps
}

#[repr(C)]
pub struct wm831x_battery_pdata {
    pub enable: i32,         // Enable charging
    pub fast_enable: i32,    // Enable fast charging
    pub off_mask: i32,       // Mask OFF while charging
    pub trickle_ilim: i32,   // Trickle charge current limit, in mA
    pub vsel: i32,           // Target voltage, in mV
    pub eoc_iterm: i32,      // End of trickle charge current, in mA
    pub fast_ilim: i32,      // Fast charge current limit, in mA
    pub timeout: i32,        // Charge cycle timeout, in minutes
}

/**
 * Configuration for the WM831x DC-DC BuckWise convertors.  This
 * should be passed as driver_data in the regulator_init_data.
 *
 * Currently all the configuration is for the fast DVS switching
 * support of the devices.  This allows MFPs on the device to be
 * configured as an input to switch between two output voltages,
 * allowing voltage transitions without the expense of an access over
 * I2C or SPI buses.
 */
#[repr(C)]
pub struct wm831x_buckv_pdata {
    pub dvs_control_src: i32, // Hardware DVS source to use (1 or 2)
    pub dvs_init_state: i32,  // DVS state to expect on startup
    pub dvs_state_gpio: i32,  // CPU GPIO to use for monitoring status
}

/* Sources for status LED configuration.  Values are register values
 * plus 1 to allow for a zero default for preserve.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum wm831x_status_src {
    WM831X_STATUS_PRESERVE = 0,  /* Keep the current hardware setting */
    WM831X_STATUS_OTP = 1,
    WM831X_STATUS_POWER = 2,
    WM831X_STATUS_CHARGER = 3,
    WM831X_STATUS_MANUAL = 4,
}

#[repr(C)]
pub struct wm831x_status_pdata {
    pub default_src: wm831x_status_src,
    pub name: *const core::ffi::c_char,
    pub default_trigger: *const core::ffi::c_char,
}

#[repr(C)]
pub struct wm831x_touch_pdata {
    pub fivewire: i32,          // 1 for five wire mode, 0 for 4 wire
    pub isel: i32,              // Current for pen down (uA)
    pub rpu: i32,               // Pen down sensitivity resistor divider
    pub pressure: i32,          // Report pressure (boolean)
    pub data_irq: u32,          // Touch data ready IRQ
    pub data_irqf: i32,         // IRQ flags for data ready IRQ
    pub pd_irq: u32,            // Touch pendown detect IRQ
    pub pd_irqf: i32,           // IRQ flags for pen down IRQ
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum wm831x_watchdog_action {
    WM831X_WDOG_NONE = 0,
    WM831X_WDOG_INTERRUPT = 1,
    WM831X_WDOG_RESET = 2,
    WM831X_WDOG_WAKE = 3,
}

#[repr(C)]
pub struct wm831x_watchdog_pdata {
    pub primary: wm831x_watchdog_action,
    pub secondary: wm831x_watchdog_action,
    // C bit-field: unsigned int software:1;
    pub software: u32,
}

pub const WM831X_MAX_STATUS: usize = 2;
pub const WM831X_MAX_DCDC: usize = 4;
pub const WM831X_MAX_EPE: usize = 2;
pub const WM831X_MAX_LDO: usize = 11;
pub const WM831X_MAX_ISINK: usize = 2;

pub const WM831X_GPIO_CONFIGURE: u32 = 0x10000;
pub const WM831X_GPIO_NUM: usize = 16;

#[repr(C)]
pub struct wm831x_pdata {
    /** Used to distinguish multiple WM831x chips */
    pub wm831x_num: i32,

    /** Called before subdevices are set up */
    pub pre_init: Option<unsafe extern "C" fn(wm831x: *mut wm831x) -> i32>,
    /** Called after subdevices are set up */
    pub post_init: Option<unsafe extern "C" fn(wm831x: *mut wm831x) -> i32>,

    /** Put the /IRQ line into CMOS mode */
    pub irq_cmos: bool,

    /** Disable the touchscreen */
    pub disable_touch: bool,

    /** The driver should initiate a power off sequence during shutdown */
    pub soft_shutdown: bool,

    pub irq_base: i32,
    pub gpio_base: i32,
    pub gpio_defaults: [i32; WM831X_GPIO_NUM],
    pub backlight: *mut wm831x_backlight_pdata,
    pub backup: *mut wm831x_backup_pdata,
    pub battery: *mut wm831x_battery_pdata,
    pub touch: *mut wm831x_touch_pdata,
    pub watchdog: *mut wm831x_watchdog_pdata,

    /** LED1 = 0 and so on */
    pub status: [*mut wm831x_status_pdata; WM831X_MAX_STATUS],
    /** DCDC1 = 0 and so on */
    pub dcdc: [*mut regulator_init_data; WM831X_MAX_DCDC],
    /** EPE1 = 0 and so on */
    pub epe: [*mut regulator_init_data; WM831X_MAX_EPE],
    /** LDO1 = 0 and so on */
    pub ldo: [*mut regulator_init_data; WM831X_MAX_LDO],
    /** ISINK1 = 0 and so on*/
    pub isink: [*mut regulator_init_data; WM831X_MAX_ISINK],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
