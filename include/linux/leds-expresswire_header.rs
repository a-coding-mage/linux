/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Shared library for Kinetic's ExpressWire protocol.
 * This protocol works by pulsing the ExpressWire IC's control GPIO.
 * ktd2692 and ktd2801 are known to use this protocol.
 *
 * The Linux type `u8` is supplied by the translated dependency context.
 */

use core::ffi::c_ulong;

pub struct gpio_desc;

#[repr(C)]
pub struct expresswire_timing {
	pub poweroff_us: c_ulong,
	pub detect_delay_us: c_ulong,
	pub detect_us: c_ulong,
	pub data_start_us: c_ulong,
	pub end_of_data_low_us: c_ulong,
	pub end_of_data_high_us: c_ulong,
	pub short_bitset_us: c_ulong,
	pub long_bitset_us: c_ulong,
}

#[repr(C)]
pub struct expresswire_common_props {
	pub ctrl_gpio: *mut gpio_desc,
	pub timing: expresswire_timing,
}

unsafe extern "C" {
	pub fn expresswire_power_off(props: *mut expresswire_common_props);
	pub fn expresswire_enable(props: *mut expresswire_common_props);
	pub fn expresswire_write_u8(props: *mut expresswire_common_props, val: u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
