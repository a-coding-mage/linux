// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-nvl-match.c - tables and support for NVL ACPI enumeration.
 *
 * Copyright (c) 2025, Intel Corporation.
 *
 */

use core::ffi::{c_char, c_void};

/* Dependencies from:
 * <sound/soc-acpi.h>
 * <sound/soc-acpi-intel-match.h>
 * <sound/soc-acpi-intel-ssp-common.h>
 * "soc-acpi-intel-sdw-mockup-match.h"
 */

#[repr(C)]
pub struct snd_soc_acpi_codecs {
	pub num_codecs: u32,
	pub codecs: [*const c_char; 3],
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
	pub id: *const c_char,
	pub comp_ids: *const snd_soc_acpi_codecs,
	pub link_mask: u32,
	pub links: *const c_void,
	pub drv_name: *const c_char,
	pub machine_quirk: Option<unsafe extern "C" fn()>,
	pub quirk_data: *const c_void,
	pub sof_tplg_filename: *const c_char,
	pub tplg_quirk_mask: u32,
}

const fn bit(n: u32) -> u32 {
	1u32 << n
}

const fn genmask(high: u32, low: u32) -> u32 {
	(!0u32 << low) & (!0u32 >> (31 - high))
}

unsafe extern "C" {
	static sdw_mockup_headset_2amps_mic: c_void;
	static sdw_mockup_headset_1amp_mic: c_void;
	static sdw_mockup_mic_headset_1amp: c_void;

	fn snd_soc_acpi_codec_list();
}

/* Preprocessor constants supplied by included headers in C. */
const RT5682_ACPI_HID: *const c_char = b"10EC5682\0".as_ptr() as *const c_char;
const RT5682S_ACPI_HID: *const c_char = b"RTL5682\0".as_ptr() as *const c_char;
const SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER: u32 = 1 << 0;
const SND_SOC_ACPI_TPLG_INTEL_SSP_MSB: u32 = 1 << 1;
const SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER: u32 = 1 << 2;

static nvl_essx_83x6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 3,
	codecs: [
		b"ESSX8316\0".as_ptr() as *const c_char,
		b"ESSX8326\0".as_ptr() as *const c_char,
		b"ESSX8336\0".as_ptr() as *const c_char,
	],
};

static nvl_lt6911_hdmi: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 1,
	codecs: [
		b"INTC10B0\0".as_ptr() as *const c_char,
		core::ptr::null(),
		core::ptr::null(),
	],
};

static nvl_rt5682_rt5682s_hp: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 2,
	codecs: [
		RT5682_ACPI_HID,
		RT5682S_ACPI_HID,
		core::ptr::null(),
	],
};

#[unsafe(no_mangle)]
pub static mut snd_soc_acpi_intel_nvl_machines: [snd_soc_acpi_mach; 6] = [
	snd_soc_acpi_mach {
		id: core::ptr::null(),
		comp_ids: &nvl_essx_83x6,
		link_mask: 0,
		links: core::ptr::null(),
		drv_name: b"nvl_es83x6_c1_h02\0".as_ptr() as *const c_char,
		machine_quirk: Some(snd_soc_acpi_codec_list),
		quirk_data: &nvl_lt6911_hdmi as *const snd_soc_acpi_codecs as *const c_void,
		sof_tplg_filename: b"sof-nvl-es83x6-ssp1-hdmi-ssp02.tplg\0".as_ptr() as *const c_char,
		tplg_quirk_mask: 0,
	},
	snd_soc_acpi_mach {
		id: core::ptr::null(),
		comp_ids: &nvl_essx_83x6,
		link_mask: 0,
		links: core::ptr::null(),
		drv_name: b"sof-essx8336\0".as_ptr() as *const c_char,
		machine_quirk: None,
		quirk_data: core::ptr::null(),
		sof_tplg_filename: b"sof-nvl-es8336\0".as_ptr() as *const c_char, /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER |
				 SND_SOC_ACPI_TPLG_INTEL_SSP_MSB |
				 SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
	},
	snd_soc_acpi_mach {
		id: core::ptr::null(),
		comp_ids: &nvl_rt5682_rt5682s_hp,
		link_mask: 0,
		links: core::ptr::null(),
		drv_name: b"nvl_rt5682_c1_h02\0".as_ptr() as *const c_char,
		machine_quirk: Some(snd_soc_acpi_codec_list),
		quirk_data: &nvl_lt6911_hdmi as *const snd_soc_acpi_codecs as *const c_void,
		sof_tplg_filename: b"sof-nvl-rt5682-ssp1-hdmi-ssp02.tplg\0".as_ptr() as *const c_char,
		tplg_quirk_mask: 0,
	},
	snd_soc_acpi_mach {
		id: core::ptr::null(),
		comp_ids: &nvl_rt5682_rt5682s_hp,
		link_mask: 0,
		links: core::ptr::null(),
		drv_name: b"sof_rt5682\0".as_ptr() as *const c_char,
		machine_quirk: None,
		quirk_data: core::ptr::null(),
		sof_tplg_filename: b"sof-nvl-rt5682\0".as_ptr() as *const c_char, /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER |
				 SND_SOC_ACPI_TPLG_INTEL_SSP_MSB,
	},
	/* place amp/hdmi-in only boards in the end of table */
	snd_soc_acpi_mach {
		id: b"INTC10B0\0".as_ptr() as *const c_char,
		comp_ids: core::ptr::null(),
		link_mask: 0,
		links: core::ptr::null(),
		drv_name: b"nvl_lt6911_hdmi_ssp\0".as_ptr() as *const c_char,
		machine_quirk: None,
		quirk_data: core::ptr::null(),
		sof_tplg_filename: b"sof-nvl-hdmi-ssp02.tplg\0".as_ptr() as *const c_char,
		tplg_quirk_mask: 0,
	},
	snd_soc_acpi_mach {
		id: core::ptr::null(),
		comp_ids: core::ptr::null(),
		link_mask: 0,
		links: core::ptr::null(),
		drv_name: core::ptr::null(),
		machine_quirk: None,
		quirk_data: core::ptr::null(),
		sof_tplg_filename: core::ptr::null(),
		tplg_quirk_mask: 0,
	},
];
/* EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_nvl_machines); */

/* this table is used when there is no I2S codec present */
#[unsafe(no_mangle)]
pub static mut snd_soc_acpi_intel_nvl_sdw_machines: [snd_soc_acpi_mach; 4] = [
	/* mockup tests need to be first */
	snd_soc_acpi_mach {
		id: core::ptr::null(),
		comp_ids: core::ptr::null(),
		link_mask: genmask(3, 0),
		links: unsafe { &sdw_mockup_headset_2amps_mic as *const c_void },
		drv_name: b"sof_sdw\0".as_ptr() as *const c_char,
		machine_quirk: None,
		quirk_data: core::ptr::null(),
		sof_tplg_filename: b"sof-nvl-rt711-rt1308-rt715.tplg\0".as_ptr() as *const c_char,
		tplg_quirk_mask: 0,
	},
	snd_soc_acpi_mach {
		id: core::ptr::null(),
		comp_ids: core::ptr::null(),
		link_mask: bit(0) | bit(1) | bit(3),
		links: unsafe { &sdw_mockup_headset_1amp_mic as *const c_void },
		drv_name: b"sof_sdw\0".as_ptr() as *const c_char,
		machine_quirk: None,
		quirk_data: core::ptr::null(),
		sof_tplg_filename: b"sof-nvl-rt711-rt1308-mono-rt715.tplg\0".as_ptr() as *const c_char,
		tplg_quirk_mask: 0,
	},
	snd_soc_acpi_mach {
		id: core::ptr::null(),
		comp_ids: core::ptr::null(),
		link_mask: genmask(2, 0),
		links: unsafe { &sdw_mockup_mic_headset_1amp as *const c_void },
		drv_name: b"sof_sdw\0".as_ptr() as *const c_char,
		machine_quirk: None,
		quirk_data: core::ptr::null(),
		sof_tplg_filename: b"sof-nvl-rt715-rt711-rt1308-mono.tplg\0".as_ptr() as *const c_char,
		tplg_quirk_mask: 0,
	},
	snd_soc_acpi_mach {
		id: core::ptr::null(),
		comp_ids: core::ptr::null(),
		link_mask: 0,
		links: core::ptr::null(),
		drv_name: core::ptr::null(),
		machine_quirk: None,
		quirk_data: core::ptr::null(),
		sof_tplg_filename: core::ptr::null(),
		tplg_quirk_mask: 0,
	},
];
/* EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_nvl_sdw_machines); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
