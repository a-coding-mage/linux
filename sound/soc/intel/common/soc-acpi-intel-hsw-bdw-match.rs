// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-hsw-bdw-match.c - tables and support for ACPI enumeration.
 *
 * Copyright (c) 2017, Intel Corporation.
 */

use core::ffi::c_char;

// Dependencies in the original C source:
// #include <linux/dmi.h>
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>

#[repr(C)]
pub struct snd_soc_acpi_mach {
	pub id: *const c_char,
	pub drv_name: *const c_char,
	pub sof_tplg_filename: *const c_char,
}

#[no_mangle]
pub static mut snd_soc_acpi_intel_broadwell_machines: [snd_soc_acpi_mach; 5] = [
	snd_soc_acpi_mach {
		id: b"INT343A\0".as_ptr() as *const c_char,
		drv_name: b"bdw_rt286\0".as_ptr() as *const c_char,
		sof_tplg_filename: b"sof-bdw-rt286.tplg\0".as_ptr() as *const c_char,
	},
	snd_soc_acpi_mach {
		id: b"10EC5650\0".as_ptr() as *const c_char,
		drv_name: b"bdw-rt5650\0".as_ptr() as *const c_char,
		sof_tplg_filename: b"sof-bdw-rt5650.tplg\0".as_ptr() as *const c_char,
	},
	snd_soc_acpi_mach {
		id: b"RT5677CE\0".as_ptr() as *const c_char,
		drv_name: b"bdw-rt5677\0".as_ptr() as *const c_char,
		sof_tplg_filename: b"sof-bdw-rt5677.tplg\0".as_ptr() as *const c_char,
	},
	snd_soc_acpi_mach {
		id: b"INT33CA\0".as_ptr() as *const c_char,
		drv_name: b"hsw_rt5640\0".as_ptr() as *const c_char,
		sof_tplg_filename: b"sof-bdw-rt5640.tplg\0".as_ptr() as *const c_char,
	},
	snd_soc_acpi_mach {
		id: core::ptr::null(),
		drv_name: core::ptr::null(),
		sof_tplg_filename: core::ptr::null(),
	},
];

// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_broadwell_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
