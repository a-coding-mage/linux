/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * bq2415x charger driver
 *
 * Copyright (C) 2011-2013  Pali Rohár <pali@kernel.org>
 */

/*
 * This is platform data for bq2415x chip. It contains default board
 * voltages and currents which can be also later configured via sysfs. If
 * value is -1 then default chip value (specified in datasheet) will be
 * used.
 *
 * Value resistor_sense is needed for configuring charge and
 * termination current. If it is less or equal to zero, configuring charge
 * and termination current will not be possible.
 *
 * For automode support is needed to provide name of power supply device
 * in value notify_device. Device driver must immediately report property
 * POWER_SUPPLY_PROP_CURRENT_MAX when current changed.
 */

/// Supported modes with maximal current limit.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum bq2415x_mode {
    BQ2415X_MODE_OFF,             /* offline mode (charger disabled) */
    BQ2415X_MODE_NONE,            /* unknown charger (100mA) */
    BQ2415X_MODE_HOST_CHARGER,    /* usb host/hub charger (500mA) */
    BQ2415X_MODE_DEDICATED_CHARGER, /* dedicated charger (unlimited) */
    BQ2415X_MODE_BOOST,           /* boost mode (charging disabled) */
}

#[repr(C)]
pub struct bq2415x_platform_data {
    pub current_limit: ::core::ffi::c_int,              /* mA */
    pub weak_battery_voltage: ::core::ffi::c_int,       /* mV */
    pub battery_regulation_voltage: ::core::ffi::c_int, /* mV */
    pub charge_current: ::core::ffi::c_int,             /* mA */
    pub termination_current: ::core::ffi::c_int,        /* mA */
    pub resistor_sense: ::core::ffi::c_int,             /* m ohm */
    pub notify_device: *const ::core::ffi::c_char,      /* name */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
