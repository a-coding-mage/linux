// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Load Analog Devices SigmaStudio firmware files
 *
 * Copyright 2009-2011 Analog Devices Inc.
 */

// C dependencies:
// #include <linux/regmap.h>
// #include <linux/export.h>
// #include <linux/module.h>
// #include "sigmadsp.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type size_t = usize;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sigmadsp_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sigmadsp {
	pub control_data: *mut c_void,
	pub write: Option<
		unsafe extern "C" fn(
			control_data: *mut c_void,
			addr: c_uint,
			data: *const u8,
			len: size_t,
		) -> c_int,
	>,
	pub read: Option<
		unsafe extern "C" fn(
			control_data: *mut c_void,
			addr: c_uint,
			data: *mut u8,
			len: size_t,
		) -> c_int,
	>,
}

extern "C" {
	fn regmap_raw_write(
		map: *mut c_void,
		reg: c_uint,
		val: *const c_void,
		val_len: size_t,
	) -> c_int;
	fn regmap_raw_read(
		map: *mut c_void,
		reg: c_uint,
		val: *mut c_void,
		val_len: size_t,
	) -> c_int;
	fn devm_sigmadsp_init(
		dev: *mut device,
		ops: *const sigmadsp_ops,
		firmware_name: *const c_char,
	) -> *mut sigmadsp;
	fn IS_ERR(ptr: *const c_void) -> bool;
}

unsafe extern "C" fn sigmadsp_write_regmap(
	control_data: *mut c_void,
	addr: c_uint,
	data: *const u8,
	len: size_t,
) -> c_int {
	unsafe { regmap_raw_write(control_data, addr, data as *const c_void, len) }
}

unsafe extern "C" fn sigmadsp_read_regmap(
	control_data: *mut c_void,
	addr: c_uint,
	data: *mut u8,
	len: size_t,
) -> c_int {
	unsafe { regmap_raw_read(control_data, addr, data as *mut c_void, len) }
}

/**
 * devm_sigmadsp_init_regmap() - Initialize SigmaDSP instance
 * @dev: The parent device
 * @regmap: Regmap instance to use
 * @ops: The sigmadsp_ops to use for this instance
 * @firmware_name: Name of the firmware file to load
 *
 * Allocates a SigmaDSP instance and loads the specified firmware file.
 *
 * Returns a pointer to a struct sigmadsp on success, or a PTR_ERR() on error.
 */
#[no_mangle]
pub unsafe extern "C" fn devm_sigmadsp_init_regmap(
	dev: *mut device,
	regmap: *mut regmap,
	ops: *const sigmadsp_ops,
	firmware_name: *const c_char,
) -> *mut sigmadsp {
	let sigmadsp: *mut sigmadsp;

	sigmadsp = unsafe { devm_sigmadsp_init(dev, ops, firmware_name) };
	if unsafe { IS_ERR(sigmadsp as *const c_void) } {
		return sigmadsp;
	}

	unsafe {
		(*sigmadsp).control_data = regmap as *mut c_void;
		(*sigmadsp).write = Some(sigmadsp_write_regmap);
		(*sigmadsp).read = Some(sigmadsp_read_regmap);
	}

	sigmadsp
}

// EXPORT_SYMBOL_GPL(devm_sigmadsp_init_regmap);

// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_DESCRIPTION("SigmaDSP regmap firmware loader");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
