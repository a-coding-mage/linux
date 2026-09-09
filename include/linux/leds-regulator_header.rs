/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * leds-regulator.h - platform data structure for regulator driven LEDs.
 *
 * Copyright (C) 2009 Antonio Ospite <ospite@studenti.unina.it>
 */

/*
 * Use "vled" as supply id when declaring the regulator consumer:
 *
 * static struct regulator_consumer_supply pcap_regulator_VVIB_consumers [] = {
 * 	{ .dev_name = "leds-regulator.0", .supply = "vled" },
 * };
 *
 * If you have several regulator driven LEDs, you can append a numerical id to
 * .dev_name as done above, and use the same id when declaring the platform
 * device:
 *
 * static struct led_regulator_platform_data a780_vibrator_data = {
 * 	.name   = "a780::vibrator",
 * };
 *
 * static struct platform_device a780_vibrator = {
 * 	.name = "leds-regulator",
 * 	.id   = 0,
 * 	.dev  = {
 * 		.platform_data = &a780_vibrator_data,
 * 	},
 * };
 */

// The C header includes <linux/leds.h>; its declarations are supplied externally.

#[repr(C)]
pub struct led_regulator_platform_data {
    /// LED name as expected by LED class.
    pub name: *mut core::ffi::c_char,
    /// Initial brightness value.
    pub brightness: led_brightness,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
