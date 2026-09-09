/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Industrial I/O in kernel access map interface.
 *
 * Copyright (c) 2011 Jonathan Cameron
 */

// C forward declarations.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_map {
    _private: [u8; 0],
}

/**
 * iio_map_array_register() - tell the core about inkernel consumers
 * @indio_dev: provider device
 * @map: array of mappings specifying association of channel with client
 */
extern "C" {
    pub fn iio_map_array_register(
        indio_dev: *mut iio_dev,
        map: *const iio_map,
    ) -> ::core::ffi::c_int;

    /**
     * iio_map_array_unregister() - tell the core to remove consumer mappings for
     *                              the given provider device
     * @indio_dev: provider device
     */
    pub fn iio_map_array_unregister(indio_dev: *mut iio_dev) -> ::core::ffi::c_int;

    /**
     * devm_iio_map_array_register - device-managed version of iio_map_array_register
     * @dev: Device object to which to bind the unwinding of this registration
     * @indio_dev: Pointer to the iio_dev structure
     * @maps: Pointer to an IIO map object which is to be registered to this IIO device
     *
     * This function will call iio_map_array_register() to register an IIO map object
     * and will also hook a callback to the iio_map_array_unregister() function to
     * handle de-registration of the IIO map object when the device's refcount goes to
     * zero.
     */
    pub fn devm_iio_map_array_register(
        dev: *mut device,
        indio_dev: *mut iio_dev,
        maps: *const iio_map,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
