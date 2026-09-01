// SPDX-License-Identifier: GPL-2.0
//
// CS35l41 HDA SPI driver
//
// Copyright 2021 Cirrus Logic, Inc.
//
// Author: Lucas Tanure <tanureal@opensource.cirrus.com>

// C dependencies:
// #include <linux/module.h>
// #include <linux/spi/spi.h>
// #include "cs35l41_hda.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const ENODEV: c_int = 19;
const SPI: c_int = 0;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
	pub dev: device,
	pub irq: c_int,
}

#[repr(C)]
pub struct regmap {
	_private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
	_private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
	pub name: *const c_char,
	pub acpi_match_table: *const acpi_device_id,
	pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct spi_device_id {
	pub name: [c_char; 32],
	pub driver_data: usize,
}

#[repr(C)]
pub struct acpi_device_id {
	pub id: [c_char; 16],
	pub driver_data: usize,
}

#[repr(C)]
pub struct spi_driver {
	pub driver: device_driver,
	pub id_table: *const spi_device_id,
	pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
	pub remove: Option<unsafe extern "C" fn(*mut spi_device)>,
}

unsafe extern "C" {
	static cs35l41_regmap_spi: regmap_config;
	static cs35l41_hda_pm_ops: dev_pm_ops;

	fn strstr(s1: *const c_char, s2: *const c_char) -> *mut c_char;
	fn dev_name(dev: *const device) -> *const c_char;
	fn spi_get_chipselect(spi: *mut spi_device, idx: c_uint) -> c_uint;
	fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
	fn cs35l41_hda_probe(
		dev: *mut device,
		device_name: *const c_char,
		id: c_uint,
		irq: c_int,
		regmap: *mut regmap,
		bus_type: c_int,
	) -> c_int;
	fn cs35l41_hda_remove(dev: *mut device);
}

unsafe extern "C" fn cs35l41_hda_spi_probe(spi: *mut spi_device) -> c_int {
	let device_name: *const c_char;

	/*
	 * Compare against the device name so it works for SPI, normal ACPI
	 * and for ACPI by serial-multi-instantiate matching cases.
	 */
	if !strstr(dev_name(&(*spi).dev), c"CSC3551".as_ptr()).is_null() {
		device_name = c"CSC3551".as_ptr();
	} else {
		return -ENODEV;
	}

	cs35l41_hda_probe(
		&mut (*spi).dev,
		device_name,
		spi_get_chipselect(spi, 0),
		(*spi).irq,
		devm_regmap_init_spi(spi, &cs35l41_regmap_spi),
		SPI,
	)
}

unsafe extern "C" fn cs35l41_hda_spi_remove(spi: *mut spi_device) {
	cs35l41_hda_remove(&mut (*spi).dev);
}

static cs35l41_hda_spi_id: [spi_device_id; 2] = [
	spi_device_id {
		name: [
			b'c' as c_char,
			b's' as c_char,
			b'3' as c_char,
			b'5' as c_char,
			b'l' as c_char,
			b'4' as c_char,
			b'1' as c_char,
			b'-' as c_char,
			b'h' as c_char,
			b'd' as c_char,
			b'a' as c_char,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
		],
		driver_data: 0,
	},
	spi_device_id {
		name: [0; 32],
		driver_data: 0,
	},
];
// MODULE_DEVICE_TABLE(spi, cs35l41_hda_spi_id);

static cs35l41_acpi_hda_match: [acpi_device_id; 2] = [
	acpi_device_id {
		id: [
			b'C' as c_char,
			b'S' as c_char,
			b'C' as c_char,
			b'3' as c_char,
			b'5' as c_char,
			b'5' as c_char,
			b'1' as c_char,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
		],
		driver_data: 0,
	},
	acpi_device_id {
		id: [0; 16],
		driver_data: 0,
	},
];
// MODULE_DEVICE_TABLE(acpi, cs35l41_acpi_hda_match);

static mut cs35l41_spi_driver: spi_driver = spi_driver {
	driver: device_driver {
		name: c"cs35l41-hda".as_ptr(),
		acpi_match_table: cs35l41_acpi_hda_match.as_ptr(),
		pm: unsafe { &cs35l41_hda_pm_ops },
	},
	id_table: cs35l41_hda_spi_id.as_ptr(),
	probe: Some(cs35l41_hda_spi_probe),
	remove: Some(cs35l41_hda_spi_remove),
};
// module_spi_driver(cs35l41_spi_driver);

// MODULE_DESCRIPTION("HDA CS35L41 driver");
// MODULE_IMPORT_NS("SND_HDA_SCODEC_CS35L41");
// MODULE_AUTHOR("Lucas Tanure <tanureal@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
