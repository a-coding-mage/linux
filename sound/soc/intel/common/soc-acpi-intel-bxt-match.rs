// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-bxt-match.c - tables and support for BXT ACPI enumeration.
 *
 * Copyright (c) 2018, Intel Corporation.
 *
 */

use core::ffi::{c_char, c_ulong, c_void};
use core::ptr;

// Dependencies originally provided by:
// #include <linux/dmi.h>
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>

#[repr(C)]
pub struct dmi_strmatch {
	pub slot: u8,
	pub substr: *const c_char,
}

#[repr(C)]
pub struct dmi_system_id {
	pub callback: Option<unsafe extern "C" fn(*const dmi_system_id) -> i32>,
	pub ident: *const c_char,
	pub matches: [dmi_strmatch; 4],
	pub driver_data: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_acpi_codecs {
	pub num_codecs: i32,
	pub codecs: [*const c_char; 3],
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
	pub id: *const c_char,
	pub comp_ids: *const snd_soc_acpi_codecs,
	pub drv_name: *const c_char,
	pub fw_filename: *const c_char,
	pub machine_quirk: Option<unsafe extern "C" fn(*mut c_void) -> *mut snd_soc_acpi_mach>,
	pub quirk_data: *const c_void,
	pub sof_tplg_filename: *const c_char,
	pub tplg_quirk_mask: u32,
}

unsafe extern "C" {
	pub fn dmi_first_match(ids: *const dmi_system_id) -> *const dmi_system_id;
	pub fn snd_soc_acpi_codec_list(arg: *mut c_void) -> *mut snd_soc_acpi_mach;
}

const DMI_SYS_VENDOR: u8 = 1;
const DMI_BOARD_NAME: u8 = 6;

const SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER: u32 = 1 << 0;
const SND_SOC_ACPI_TPLG_INTEL_SSP_MSB: u32 = 1 << 1;
const SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER: u32 = 1 << 2;

const fn dmi_match(slot: u8, substr: *const c_char) -> dmi_strmatch {
	dmi_strmatch { slot, substr }
}

const APL_RVP: c_ulong = 0;

static APL_TABLE: [dmi_system_id; 2] = [
	dmi_system_id {
		callback: None,
		ident: ptr::null(),
		matches: [
			dmi_match(DMI_SYS_VENDOR, c"Intel Corp.".as_ptr()),
			dmi_match(DMI_BOARD_NAME, c"Apollolake RVP1A".as_ptr()),
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
		],
		driver_data: APL_RVP as *mut c_void,
	},
	dmi_system_id {
		callback: None,
		ident: ptr::null(),
		matches: [
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
		],
		driver_data: ptr::null_mut(),
	},
];

unsafe extern "C" fn apl_quirk(arg: *mut c_void) -> *mut snd_soc_acpi_mach {
	let mach = arg as *mut snd_soc_acpi_mach;
	let dmi_id: *const dmi_system_id;
	let apl_machine_id: c_ulong;

	dmi_id = unsafe { dmi_first_match(APL_TABLE.as_ptr()) };
	if !dmi_id.is_null() {
		apl_machine_id = unsafe { (*dmi_id).driver_data as c_ulong };
		if apl_machine_id == APL_RVP {
			return ptr::null_mut();
		}
	}

	mach
}

static ESSX_83X6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 3,
	codecs: [
		c"ESSX8316".as_ptr(),
		c"ESSX8326".as_ptr(),
		c"ESSX8336".as_ptr(),
	],
};

static BXT_CODECS: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 1,
	codecs: [c"MX98357A".as_ptr(), ptr::null(), ptr::null()],
};

#[unsafe(no_mangle)]
pub static mut snd_soc_acpi_intel_bxt_machines: [snd_soc_acpi_mach; 7] = [
	snd_soc_acpi_mach {
		id: c"INT343A".as_ptr(),
		comp_ids: ptr::null(),
		drv_name: c"bxt_alc298s_i2s".as_ptr(),
		fw_filename: c"intel/dsp_fw_bxtn.bin".as_ptr(),
		machine_quirk: None,
		quirk_data: ptr::null(),
		sof_tplg_filename: c"sof-apl-rt298.tplg".as_ptr(),
		tplg_quirk_mask: 0,
	},
	snd_soc_acpi_mach {
		id: c"DLGS7219".as_ptr(),
		comp_ids: ptr::null(),
		drv_name: c"bxt_da7219_mx98357a".as_ptr(),
		fw_filename: c"intel/dsp_fw_bxtn.bin".as_ptr(),
		machine_quirk: Some(snd_soc_acpi_codec_list),
		quirk_data: &BXT_CODECS as *const snd_soc_acpi_codecs as *const c_void,
		sof_tplg_filename: c"sof-apl-da7219.tplg".as_ptr(),
		tplg_quirk_mask: 0,
	},
	snd_soc_acpi_mach {
		id: c"104C5122".as_ptr(),
		comp_ids: ptr::null(),
		drv_name: c"sof_pcm512x".as_ptr(),
		fw_filename: ptr::null(),
		machine_quirk: None,
		quirk_data: ptr::null(),
		sof_tplg_filename: c"sof-apl-pcm512x.tplg".as_ptr(),
		tplg_quirk_mask: 0,
	},
	snd_soc_acpi_mach {
		id: c"1AEC8804".as_ptr(),
		comp_ids: ptr::null(),
		drv_name: c"sof-wm8804".as_ptr(),
		fw_filename: ptr::null(),
		machine_quirk: None,
		quirk_data: ptr::null(),
		sof_tplg_filename: c"sof-apl-wm8804.tplg".as_ptr(),
		tplg_quirk_mask: 0,
	},
	snd_soc_acpi_mach {
		id: c"INT34C3".as_ptr(),
		comp_ids: ptr::null(),
		drv_name: c"bxt_tdf8532".as_ptr(),
		fw_filename: ptr::null(),
		machine_quirk: Some(apl_quirk),
		quirk_data: ptr::null(),
		sof_tplg_filename: c"sof-apl-tdf8532.tplg".as_ptr(),
		tplg_quirk_mask: 0,
	},
	snd_soc_acpi_mach {
		id: ptr::null(),
		comp_ids: &ESSX_83X6,
		drv_name: c"sof-essx8336".as_ptr(),
		fw_filename: ptr::null(),
		machine_quirk: None,
		quirk_data: ptr::null(),
		sof_tplg_filename: c"sof-apl-es8336".as_ptr(), /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER
			| SND_SOC_ACPI_TPLG_INTEL_SSP_MSB
			| SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
	},
	snd_soc_acpi_mach {
		id: ptr::null(),
		comp_ids: ptr::null(),
		drv_name: ptr::null(),
		fw_filename: ptr::null(),
		machine_quirk: None,
		quirk_data: ptr::null(),
		sof_tplg_filename: ptr::null(),
		tplg_quirk_mask: 0,
	},
];

// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_bxt_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
