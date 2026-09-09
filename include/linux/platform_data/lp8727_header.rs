/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * LP8727 Micro/Mini USB IC with integrated charger
 *
 *			Copyright (C) 2011 Texas Instruments
 *			Copyright (C) 2011 National Semiconductor
 */

#[repr(C)]
pub enum lp8727_eoc_level {
    LP8727_EOC_5P,
    LP8727_EOC_10P,
    LP8727_EOC_16P,
    LP8727_EOC_20P,
    LP8727_EOC_25P,
    LP8727_EOC_33P,
    LP8727_EOC_50P,
}

#[repr(C)]
pub enum lp8727_ichg {
    LP8727_ICHG_90mA,
    LP8727_ICHG_100mA,
    LP8727_ICHG_400mA,
    LP8727_ICHG_450mA,
    LP8727_ICHG_500mA,
    LP8727_ICHG_600mA,
    LP8727_ICHG_700mA,
    LP8727_ICHG_800mA,
    LP8727_ICHG_900mA,
    LP8727_ICHG_1000mA,
}

/**
 * struct lp8727_chg_param
 * @eoc_level : end of charge level setting
 * @ichg      : charging current
 */
#[repr(C)]
pub struct lp8727_chg_param {
    pub eoc_level: lp8727_eoc_level,
    pub ichg: lp8727_ichg,
}

/**
 * struct lp8727_platform_data
 * @get_batt_present  : check battery status - exists or not
 * @get_batt_level    : get battery voltage (mV)
 * @get_batt_capacity : get battery capacity (%)
 * @get_batt_temp     : get battery temperature
 * @ac                : charging parameters for AC type charger
 * @usb               : charging parameters for USB type charger
 * @debounce_msec     : interrupt debounce time
 */
#[repr(C)]
pub struct lp8727_platform_data {
    pub get_batt_present: Option<unsafe extern "C" fn() -> u8>,
    pub get_batt_level: Option<unsafe extern "C" fn() -> u16>,
    pub get_batt_capacity: Option<unsafe extern "C" fn() -> u8>,
    pub get_batt_temp: Option<unsafe extern "C" fn() -> u8>,
    pub ac: *mut lp8727_chg_param,
    pub usb: *mut lp8727_chg_param,
    pub debounce_msec: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
