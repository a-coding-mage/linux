/* SPDX-License-Identifier: GPL-2.0 */
/* LED Multicolor class interface
 * Copyright (C) 2019-20 Texas Instruments Incorporated - http://www.ti.com/
 */

/* Dependencies: <linux/leds.h> and <dt-bindings/leds/common.h>. */

/**
 * struct mc_subled - Color component description.
 * @color_index: Color ID.
 * @brightness: Scaled intensity.
 * @intensity: Current intensity.
 * @max_intensity: Maximum supported intensity value.
 * @channel: Channel index.
 *
 * Describes a color component of a multicolor LED. Many multicolor LEDs
 * do not support global brightness control in hardware, so they use
 * the brightness field in connection with led_mc_calc_color_components()
 * to perform the intensity scaling in software.
 * Such drivers should set max_intensity to 0 to signal the multicolor LED core
 * that the maximum global brightness of the LED class device should be used for
 * limiting incoming intensity values.
 *
 * Multicolor LEDs that do support global brightness control in hardware
 * should instead set max_intensity to the maximum intensity value supported
 * by the hardware for a given color component.
 */
#[repr(C)]
pub struct mc_subled {
    pub color_index: ::core::ffi::c_uint,
    pub brightness: ::core::ffi::c_uint,
    pub intensity: ::core::ffi::c_uint,
    pub max_intensity: ::core::ffi::c_uint,
    pub channel: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct led_classdev_mc {
    /* led class device */
    pub led_cdev: crate::led_classdev,
    pub num_colors: ::core::ffi::c_uint,
    pub subled_info: *mut mc_subled,
}

pub unsafe fn lcdev_to_mccdev(
    led_cdev: *mut crate::led_classdev,
) -> *mut led_classdev_mc {
    crate::container_of!(led_cdev, led_classdev_mc, led_cdev)
}

extern "C" {
    /**
     * led_classdev_multicolor_register_ext - register a new object of led_classdev
     *                                      class with support for multicolor LEDs
     * @parent: the multicolor LED to register
     * @mcled_cdev: the led_classdev_mc structure for this device
     * @init_data: the LED class multicolor device initialization data
     *
     * Returns: 0 on success or negative error value on failure
     */
    pub fn led_classdev_multicolor_register_ext(
        parent: *mut crate::device,
        mcled_cdev: *mut led_classdev_mc,
        init_data: *mut crate::led_init_data,
    ) -> ::core::ffi::c_int;

    /**
     * led_classdev_multicolor_unregister - unregisters an object of led_classdev
     *                                      class with support for multicolor LEDs
     * @mcled_cdev: the led_classdev_mc structure for this device
     *
     * Unregister a previously registered via led_classdev_multicolor_register
     * object
     */
    pub fn led_classdev_multicolor_unregister(mcled_cdev: *mut led_classdev_mc);

    /**
     * led_mc_calc_color_components() - Calculates component brightness values of a LED cluster.
     * @mcled_cdev - Multicolor LED class device of the LED cluster.
     * @brightness - Global brightness of the LED cluster.
     *
     * Calculates the brightness values for each color component of a monochrome LED cluster,
     * see Documentation/leds/leds-class-multicolor.rst for details.
     */
    pub fn led_mc_calc_color_components(
        mcled_cdev: *mut led_classdev_mc,
        brightness: crate::led_brightness,
    ) -> ::core::ffi::c_int;

    pub fn devm_led_classdev_multicolor_register_ext(
        parent: *mut crate::device,
        mcled_cdev: *mut led_classdev_mc,
        init_data: *mut crate::led_init_data,
    ) -> ::core::ffi::c_int;

    pub fn devm_led_classdev_multicolor_unregister(
        parent: *mut crate::device,
        mcled_cdev: *mut led_classdev_mc,
    );
}

pub unsafe fn led_classdev_multicolor_register(
    parent: *mut crate::device,
    mcled_cdev: *mut led_classdev_mc,
) -> ::core::ffi::c_int {
    led_classdev_multicolor_register_ext(parent, mcled_cdev, ::core::ptr::null_mut())
}

pub unsafe fn devm_led_classdev_multicolor_register(
    parent: *mut crate::device,
    mcled_cdev: *mut led_classdev_mc,
) -> ::core::ffi::c_int {
    devm_led_classdev_multicolor_register_ext(parent, mcled_cdev, ::core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
