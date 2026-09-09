/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KUnit basic device implementation
 *
 * Helpers for creating and managing fake devices for KUnit tests.
 *
 * Copyright (C) 2023, Google LLC.
 * Author: David Gow <davidgow@google.com>
 */

// The C header guard `_KUNIT_DEVICE_H` is omitted in Rust.
// All declarations below are conditional on CONFIG_KUNIT being enabled.
#[cfg(feature = "CONFIG_KUNIT")]
mod kunit_config {
    use core::ffi::c_char;

    // Supplied by the corresponding kernel headers.
    #[repr(C)]
    pub struct device {
        _private: [u8; 0],
    }

    #[repr(C)]
    pub struct device_driver {
        _private: [u8; 0],
    }

    #[repr(C)]
    pub struct kunit {
        _private: [u8; 0],
    }

    /**
     * kunit_driver_create() - Create a struct device_driver attached to the kunit_bus
     * @test: The test context object.
     * @name: The name to give the created driver.
     *
     * Creates a struct device_driver attached to the kunit_bus, with the name @name.
     * This driver will automatically be cleaned up on test exit.
     *
     * Return: a stub struct device_driver, managed by KUnit, with the name @name.
     */
    unsafe extern "C" {
        pub fn kunit_driver_create(
            test: *mut kunit,
            name: *const c_char,
        ) -> *mut device_driver;

        /**
         * kunit_device_register() - Create a struct device for use in KUnit tests
         * @test: The test context object.
         * @name: The name to give the created device.
         *
         * Creates a struct kunit_device (which is a struct device) with the given name,
         * and a corresponding driver. The device and driver will be cleaned up on test
         * exit, or when kunit_device_unregister is called. See also
         * kunit_device_register_with_driver, if you wish to provide your own
         * struct device_driver.
         *
         * Return: a pointer to a struct device which will be cleaned up when the test
         * exits, or an error pointer if the device could not be allocated or registered.
         */
        pub fn kunit_device_register(
            test: *mut kunit,
            name: *const c_char,
        ) -> *mut device;

        /**
         * kunit_device_register_with_driver() - Create a struct device for use in KUnit tests
         * @test: The test context object.
         * @name: The name to give the created device.
         * @drv: The struct device_driver to associate with the device.
         *
         * Creates a struct kunit_device (which is a struct device) with the given
         * name, and driver. The device will be cleaned up on test exit, or when
         * kunit_device_unregister is called. See also kunit_device_register, if you
         * wish KUnit to create and manage a driver for you.
         *
         * Return: a pointer to a struct device which will be cleaned up when the test
         * exits, or an error pointer if the device could not be allocated or registered.
         */
        pub fn kunit_device_register_with_driver(
            test: *mut kunit,
            name: *const c_char,
            drv: *const device_driver,
        ) -> *mut device;

        /**
         * kunit_device_unregister() - Unregister a KUnit-managed device
         * @test: The test context object which created the device
         * @dev: The device.
         *
         * Unregisters and destroys a struct device which was created with
         * kunit_device_register or kunit_device_register_with_driver. If KUnit created
         * a driver, cleans it up as well.
         */
        pub fn kunit_device_unregister(test: *mut kunit, dev: *mut device);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
