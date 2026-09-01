// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-cnl-match.c - tables and support for CNL ACPI enumeration.
 *
 * Copyright (c) 2018, Intel Corporation.
 *
 */

// C dependencies:
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>
// #include "soc-acpi-intel-sdw-mockup-match.h"

use core::ffi::c_char;

const fn BIT(nr: u32) -> u32 {
	1u32 << nr
}

const fn GENMASK(h: u32, l: u32) -> u32 {
	((!0u32) << l) & ((!0u32) >> (u32::BITS - 1 - h))
}

unsafe extern "C" {
	static sdw_mockup_headset_2amps_mic: [snd_soc_acpi_link_adr; 0];
	static sdw_mockup_headset_1amp_mic: [snd_soc_acpi_link_adr; 0];
}

static essx_83x6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 3,
	codecs: [
		c"ESSX8316".as_ptr() as *const c_char,
		c"ESSX8326".as_ptr() as *const c_char,
		c"ESSX8336".as_ptr() as *const c_char,
	],
};

#[no_mangle]
pub static mut snd_soc_acpi_intel_cnl_machines: [snd_soc_acpi_mach; 3] = [
	snd_soc_acpi_mach {
		id: c"INT34C2".as_ptr() as *const c_char,
		drv_name: c"cnl_rt274".as_ptr() as *const c_char,
		fw_filename: c"intel/dsp_fw_cnl.bin".as_ptr() as *const c_char,
		sof_tplg_filename: c"sof-cnl-rt274.tplg".as_ptr() as *const c_char,
		..unsafe { core::mem::zeroed() }
	},
	snd_soc_acpi_mach {
		comp_ids: &essx_83x6,
		drv_name: c"sof-essx8336".as_ptr() as *const c_char,
		/* cnl and cml are identical */
		sof_tplg_filename: c"sof-cml-es8336".as_ptr() as *const c_char, /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER |
			SND_SOC_ACPI_TPLG_INTEL_SSP_MSB |
			SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
		..unsafe { core::mem::zeroed() }
	},
	unsafe { core::mem::zeroed() },
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_cnl_machines);

static single_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint {
	num: 0,
	aggregated: 0,
	group_position: 0,
	group_id: 0,
};

static rt5682_2_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000220025D568200u64,
		num_endpoints: 1,
		endpoints: &single_endpoint,
		name_prefix: c"rt5682".as_ptr() as *const c_char,
	},
];

static up_extreme_rt5682_2: [snd_soc_acpi_link_adr; 2] = [
	snd_soc_acpi_link_adr {
		mask: BIT(2),
		num_adr: rt5682_2_adr.len(),
		adr_d: rt5682_2_adr.as_ptr(),
	},
	unsafe { core::mem::zeroed() },
];

#[no_mangle]
pub static mut snd_soc_acpi_intel_cnl_sdw_machines: [snd_soc_acpi_mach; 4] = [
	snd_soc_acpi_mach {
		link_mask: BIT(2),
		links: up_extreme_rt5682_2.as_ptr(),
		drv_name: c"sof_sdw".as_ptr() as *const c_char,
		sof_tplg_filename: c"sof-cnl-rt5682-sdw2.tplg".as_ptr() as *const c_char,
		..unsafe { core::mem::zeroed() }
	},
	snd_soc_acpi_mach {
		link_mask: GENMASK(3, 0),
		links: unsafe { sdw_mockup_headset_2amps_mic.as_ptr() },
		drv_name: c"sof_sdw".as_ptr() as *const c_char,
		sof_tplg_filename: c"sof-cml-rt711-rt1308-rt715.tplg".as_ptr() as *const c_char,
		..unsafe { core::mem::zeroed() }
	},
	snd_soc_acpi_mach {
		link_mask: BIT(0) | BIT(1) | BIT(3),
		links: unsafe { sdw_mockup_headset_1amp_mic.as_ptr() },
		drv_name: c"sof_sdw".as_ptr() as *const c_char,
		sof_tplg_filename: c"sof-cml-rt711-rt1308-mono-rt715.tplg".as_ptr() as *const c_char,
		..unsafe { core::mem::zeroed() }
	},
	unsafe { core::mem::zeroed() },
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_cnl_sdw_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
