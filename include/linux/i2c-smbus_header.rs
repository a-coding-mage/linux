/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * i2c-smbus.h - SMBus extensions to the I2C protocol
 *
 * Copyright (C) 2010-2019 Jean Delvare <jdelvare@suse.de>
 */

// Dependencies supplied by the Linux I2C, error-pointer, and configuration
// environments are intentionally referenced but not defined here.

/**
 * i2c_smbus_alert_setup - platform data for the smbus_alert i2c client
 * @irq: IRQ number, if the smbus_alert driver should take care of interrupt
 *		handling
 *
 * If irq is not specified, the smbus_alert driver doesn't take care of
 * interrupt handling. In that case it is up to the I2C bus driver to either
 * handle the interrupts or to poll for alerts.
 */
#[repr(C)]
pub struct i2c_smbus_alert_setup {
    pub irq: ::core::ffi::c_int,
}

extern "C" {
    pub fn i2c_new_smbus_alert_device(
        adapter: *mut i2c_adapter,
        setup: *mut i2c_smbus_alert_setup,
    ) -> *mut i2c_client;
    pub fn i2c_handle_smbus_alert(ara: *mut i2c_client) -> ::core::ffi::c_int;
}

// Corresponds to: IS_ENABLED(CONFIG_I2C_SMBUS) && IS_ENABLED(CONFIG_I2C_SLAVE)
#[cfg(all(feature = "CONFIG_I2C_SMBUS", feature = "CONFIG_I2C_SLAVE"))]
extern "C" {
    pub fn i2c_new_slave_host_notify_device(adapter: *mut i2c_adapter) -> *mut i2c_client;
    pub fn i2c_free_slave_host_notify_device(client: *mut i2c_client);
}

#[cfg(not(all(feature = "CONFIG_I2C_SMBUS", feature = "CONFIG_I2C_SLAVE")))]
pub unsafe fn i2c_new_slave_host_notify_device(
    _adapter: *mut i2c_adapter,
) -> *mut i2c_client {
    // Equivalent to ERR_PTR(-ENOSYS); ERR_PTR and ENOSYS are supplied by the
    // surrounding kernel translation.
    ERR_PTR(-ENOSYS)
}

#[cfg(not(all(feature = "CONFIG_I2C_SMBUS", feature = "CONFIG_I2C_SLAVE")))]
pub unsafe fn i2c_free_slave_host_notify_device(_client: *mut i2c_client) {}

// Corresponds to: IS_ENABLED(CONFIG_I2C_SMBUS) && IS_ENABLED(CONFIG_DMI)
#[cfg(all(feature = "CONFIG_I2C_SMBUS", feature = "CONFIG_DMI"))]
extern "C" {
    pub fn i2c_register_spd_write_disable(adap: *mut i2c_adapter);
    pub fn i2c_register_spd_write_enable(adap: *mut i2c_adapter);
}

#[cfg(not(all(feature = "CONFIG_I2C_SMBUS", feature = "CONFIG_DMI")))]
pub unsafe fn i2c_register_spd_write_disable(_adap: *mut i2c_adapter) {}

#[cfg(not(all(feature = "CONFIG_I2C_SMBUS", feature = "CONFIG_DMI")))]
pub unsafe fn i2c_register_spd_write_enable(_adap: *mut i2c_adapter) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
