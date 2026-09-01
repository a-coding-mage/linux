// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-ptl-match.c - tables and support for PTL ACPI enumeration.
 *
 * Copyright (c) 2024, Intel Corporation.
 *
 * Order of entries in snd_soc_acpi_intel_ptl_sdw_machines[] matters.
 * Check subset of link mask when matching the machine driver, rule is
 * superset match should be ordered before subset matches.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const fn BIT(n: c_uint) -> c_ulong {
	1usize.wrapping_shl(n) as c_ulong
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_ulong {
	((!0usize).wrapping_shl(l) & (!0usize).wrapping_shr((usize::BITS - 1) - h)) as c_ulong
}

const SND_SOC_ACPI_TPLG_INTEL_AMP_NAME: c_ulong = 1 << 0;
const SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME: c_ulong = 1 << 1;
const SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER: c_ulong = 1 << 2;
const SND_SOC_ACPI_TPLG_INTEL_SSP_MSB: c_ulong = 1 << 3;
const SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER: c_ulong = 1 << 4;

const RT5682_ACPI_HID: *const c_char = c"10EC5682".as_ptr();
const RT5682S_ACPI_HID: *const c_char = c"RTL5682".as_ptr();

#[repr(C)]
pub struct snd_soc_acpi_codecs {
	pub num_codecs: c_uint,
	pub codecs: [*const c_char; 8],
}

#[repr(C)]
pub struct snd_soc_acpi_endpoint {
	pub num: c_uint,
	pub aggregated: c_uint,
	pub group_position: c_uint,
	pub group_id: c_uint,
}

#[repr(C)]
pub struct snd_soc_acpi_adr_device {
	pub adr: u64,
	pub num_endpoints: c_uint,
	pub endpoints: *const snd_soc_acpi_endpoint,
	pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_link_adr {
	pub mask: c_ulong,
	pub num_adr: c_uint,
	pub adr_d: *const snd_soc_acpi_adr_device,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
	pub id: *const c_char,
	pub comp_ids: *const snd_soc_acpi_codecs,
	pub drv_name: *const c_char,
	pub machine_quirk: Option<unsafe extern "C" fn(*const snd_soc_acpi_mach) -> c_int>,
	pub quirk_data: *const c_void,
	pub sof_tplg_filename: *const c_char,
	pub tplg_quirk_mask: c_ulong,
	pub link_mask: c_ulong,
	pub links: *const snd_soc_acpi_link_adr,
	pub machine_check: Option<unsafe extern "C" fn(*const snd_soc_acpi_mach) -> c_int>,
	pub get_function_tplg_files: Option<unsafe extern "C" fn(*const snd_soc_acpi_mach) -> c_int>,
}

const NULL_MACH: snd_soc_acpi_mach = snd_soc_acpi_mach {
	id: core::ptr::null(),
	comp_ids: core::ptr::null(),
	drv_name: core::ptr::null(),
	machine_quirk: None,
	quirk_data: core::ptr::null(),
	sof_tplg_filename: core::ptr::null(),
	tplg_quirk_mask: 0,
	link_mask: 0,
	links: core::ptr::null(),
	machine_check: None,
	get_function_tplg_files: None,
};

const NULL_LINK: snd_soc_acpi_link_adr = snd_soc_acpi_link_adr {
	mask: 0,
	num_adr: 0,
	adr_d: core::ptr::null(),
};

unsafe extern "C" {
	fn snd_soc_acpi_codec_list(mach: *const snd_soc_acpi_mach) -> c_int;
	fn snd_soc_acpi_intel_sdca_is_device_rt712_vb(mach: *const snd_soc_acpi_mach) -> c_int;
	fn snd_soc_acpi_intel_no_function_topology(mach: *const snd_soc_acpi_mach) -> c_int;
	fn snd_soc_acpi_intel_rt712_vb_no_function_topology(mach: *const snd_soc_acpi_mach) -> c_int;
	fn sof_sdw_get_tplg_files(mach: *const snd_soc_acpi_mach) -> c_int;

	static sdw_mockup_headset_2amps_mic: [snd_soc_acpi_link_adr; 0];
	static sdw_mockup_headset_1amp_mic: [snd_soc_acpi_link_adr; 0];
	static sdw_mockup_mic_headset_1amp: [snd_soc_acpi_link_adr; 0];
	static sdw_mockup_multi_func: [snd_soc_acpi_link_adr; 0];
}

static ptl_rt5682_rt5682s_hp: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 2,
	codecs: [RT5682_ACPI_HID, RT5682S_ACPI_HID, core::ptr::null(), core::ptr::null(),
		core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()],
};

static ptl_essx_83x6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 3,
	codecs: [c"ESSX8316".as_ptr(), c"ESSX8326".as_ptr(), c"ESSX8336".as_ptr(),
		core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(),
		core::ptr::null()],
};

static ptl_lt6911_hdmi: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 1,
	codecs: [c"INTC10B0".as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(),
		core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()],
};

#[no_mangle]
pub static mut snd_soc_acpi_intel_ptl_machines: [snd_soc_acpi_mach; 6] = [
	snd_soc_acpi_mach {
		comp_ids: &ptl_rt5682_rt5682s_hp,
		drv_name: c"ptl_rt5682_c1_h02".as_ptr(),
		machine_quirk: Some(snd_soc_acpi_codec_list),
		quirk_data: &ptl_lt6911_hdmi as *const _ as *const c_void,
		sof_tplg_filename: c"sof-ptl-rt5682-ssp1-hdmi-ssp02.tplg".as_ptr(),
		..NULL_MACH
	},
	snd_soc_acpi_mach {
		comp_ids: &ptl_rt5682_rt5682s_hp,
		drv_name: c"ptl_rt5682_def".as_ptr(),
		sof_tplg_filename: c"sof-ptl".as_ptr(), /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME |
			SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME,
		..NULL_MACH
	},
	snd_soc_acpi_mach {
		comp_ids: &ptl_essx_83x6,
		drv_name: c"ptl_es83x6_c1_h02".as_ptr(),
		machine_quirk: Some(snd_soc_acpi_codec_list),
		quirk_data: &ptl_lt6911_hdmi as *const _ as *const c_void,
		sof_tplg_filename: c"sof-ptl-es83x6-ssp1-hdmi-ssp02.tplg".as_ptr(),
		..NULL_MACH
	},
	snd_soc_acpi_mach {
		comp_ids: &ptl_essx_83x6,
		drv_name: c"sof-essx8336".as_ptr(),
		sof_tplg_filename: c"sof-ptl-es8336".as_ptr(), /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER |
			SND_SOC_ACPI_TPLG_INTEL_SSP_MSB |
			SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
		..NULL_MACH
	},
	/* place amp-only boards in the end of table */
	snd_soc_acpi_mach {
		id: c"INTC10B0".as_ptr(),
		drv_name: c"ptl_lt6911_hdmi_ssp".as_ptr(),
		sof_tplg_filename: c"sof-ptl-hdmi-ssp02.tplg".as_ptr(),
		..NULL_MACH
	},
	NULL_MACH,
];
/* EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_ptl_machines); */

static single_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint { num: 0, aggregated: 0, group_position: 0, group_id: 0 };
static spk_l_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint { num: 0, aggregated: 1, group_position: 0, group_id: 1 };
static spk_r_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint { num: 0, aggregated: 1, group_position: 1, group_id: 1 };

static jack_dmic_endpoints: [snd_soc_acpi_endpoint; 2] = [
	/* Jack Endpoint */
	snd_soc_acpi_endpoint { num: 0, aggregated: 0, group_position: 0, group_id: 0 },
	/* DMIC Endpoint */
	snd_soc_acpi_endpoint { num: 1, aggregated: 0, group_position: 0, group_id: 0 },
];

static jack_amp_g1_dmic_endpoints: [snd_soc_acpi_endpoint; 3] = [
	/* Jack Endpoint */
	snd_soc_acpi_endpoint { num: 0, aggregated: 0, group_position: 0, group_id: 0 },
	/* Amp Endpoint, work as spk_l_endpoint */
	snd_soc_acpi_endpoint { num: 1, aggregated: 1, group_position: 0, group_id: 1 },
	/* DMIC Endpoint */
	snd_soc_acpi_endpoint { num: 2, aggregated: 0, group_position: 0, group_id: 0 },
];

static cs42l43_amp_spkagg_endpoints: [snd_soc_acpi_endpoint; 4] = [
	snd_soc_acpi_endpoint { num: 0, aggregated: 0, group_position: 0, group_id: 0 }, /* Jack Playback Endpoint */
	snd_soc_acpi_endpoint { num: 1, aggregated: 0, group_position: 0, group_id: 0 }, /* DMIC Capture Endpoint */
	snd_soc_acpi_endpoint { num: 2, aggregated: 0, group_position: 0, group_id: 0 }, /* Jack Capture Endpoint */
	snd_soc_acpi_endpoint { num: 3, aggregated: 1, group_position: 0, group_id: 1 }, /* Speaker Playback Endpoint */
];

static cs42l43_3_agg_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
	adr: 0x00033001FA424301u64, num_endpoints: cs42l43_amp_spkagg_endpoints.len() as c_uint,
	endpoints: cs42l43_amp_spkagg_endpoints.as_ptr(), name_prefix: c"cs42l43".as_ptr()
}];
static cs35l56_2_lr_adr: [snd_soc_acpi_adr_device; 2] = [
	snd_soc_acpi_adr_device { adr: 0x00023001fa355601u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"AMP1".as_ptr() },
	snd_soc_acpi_adr_device { adr: 0x00023101fa355601u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"AMP2".as_ptr() },
];
static rt711_sdca_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000030025D071101u64, num_endpoints: 1, endpoints: &single_endpoint, name_prefix: c"rt711".as_ptr() }];
static rt712_vb_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000230025D071201u64, num_endpoints: jack_amp_g1_dmic_endpoints.len() as c_uint, endpoints: jack_amp_g1_dmic_endpoints.as_ptr(), name_prefix: c"rt712".as_ptr() }];
static rt712_vb_3_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000330025D071201u64, num_endpoints: jack_amp_g1_dmic_endpoints.len() as c_uint, endpoints: jack_amp_g1_dmic_endpoints.as_ptr(), name_prefix: c"rt712".as_ptr() }];
static rt713_vb_2_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000230025d071301u64, num_endpoints: jack_dmic_endpoints.len() as c_uint, endpoints: jack_dmic_endpoints.as_ptr(), name_prefix: c"rt713".as_ptr() }];
static rt713_vb_3_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000330025D071301u64, num_endpoints: jack_dmic_endpoints.len() as c_uint, endpoints: jack_dmic_endpoints.as_ptr(), name_prefix: c"rt713".as_ptr() }];
static rt1320_3_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000330025D132001u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"rt1320-1".as_ptr() }];
static rt722_0_agg_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000030025d072201u64, num_endpoints: jack_amp_g1_dmic_endpoints.len() as c_uint, endpoints: jack_amp_g1_dmic_endpoints.as_ptr(), name_prefix: c"rt722".as_ptr() }];
static rt1320_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000130025D132001u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"rt1320-1".as_ptr() }];
static rt1320_1_group2_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000130025D132001u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"rt1320-1".as_ptr() }];
static rt1320_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000230025D132001u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"rt1320-1".as_ptr() }];
static rt1320_2_group2_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000230025D132001u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"rt1320-2".as_ptr() }];
static rt1320_2_group2_l_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000230025D132001u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"rt1320-1".as_ptr() }];
static rt1320_3_group2_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000330025D132001u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"rt1320-2".as_ptr() }];

static ptl_cs42l43_agg_l3_cs35l56_l2: [snd_soc_acpi_link_adr; 3] = [
	snd_soc_acpi_link_adr { mask: BIT(3), num_adr: cs42l43_3_agg_adr.len() as c_uint, adr_d: cs42l43_3_agg_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: cs35l56_2_lr_adr.len() as c_uint, adr_d: cs35l56_2_lr_adr.as_ptr() },
	NULL_LINK,
];
static ptl_rt722_l0_rt1320_l23: [snd_soc_acpi_link_adr; 4] = [
	snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt722_0_agg_adr.len() as c_uint, adr_d: rt722_0_agg_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1320_2_group2_l_adr.len() as c_uint, adr_d: rt1320_2_group2_l_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt1320_3_group2_adr.len() as c_uint, adr_d: rt1320_3_group2_adr.as_ptr() },
	NULL_LINK,
];
static ptl_rvp: [snd_soc_acpi_link_adr; 2] = [
	snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len() as c_uint, adr_d: rt711_sdca_0_adr.as_ptr() },
	NULL_LINK,
];
static ptl_sdw_rt713_vb_l2_rt1320_l13: [snd_soc_acpi_link_adr; 4] = [
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt713_vb_2_adr.len() as c_uint, adr_d: rt713_vb_2_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1320_1_group2_adr.len() as c_uint, adr_d: rt1320_1_group2_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt1320_3_group2_adr.len() as c_uint, adr_d: rt1320_3_group2_adr.as_ptr() },
	NULL_LINK,
];
static ptl_sdw_rt713_vb_l3_rt1320_l12: [snd_soc_acpi_link_adr; 4] = [
	snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt713_vb_3_adr.len() as c_uint, adr_d: rt713_vb_3_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1320_1_group2_adr.len() as c_uint, adr_d: rt1320_1_group2_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1320_2_group2_adr.len() as c_uint, adr_d: rt1320_2_group2_adr.as_ptr() },
	NULL_LINK,
];
static ptl_sdw_rt713_vb_l3_rt1320_l1: [snd_soc_acpi_link_adr; 3] = [
	snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt713_vb_3_adr.len() as c_uint, adr_d: rt713_vb_3_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1320_1_group2_adr.len() as c_uint, adr_d: rt1320_1_group2_adr.as_ptr() },
	NULL_LINK,
];
static ptl_sdw_rt712_vb_l2_rt1320_l1: [snd_soc_acpi_link_adr; 3] = [
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt712_vb_2_group1_adr.len() as c_uint, adr_d: rt712_vb_2_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1320_1_group1_adr.len() as c_uint, adr_d: rt1320_1_group1_adr.as_ptr() },
	NULL_LINK,
];
static ptl_sdw_rt712_vb_l3_rt1320_l2: [snd_soc_acpi_link_adr; 3] = [
	snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt712_vb_3_group1_adr.len() as c_uint, adr_d: rt712_vb_3_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1320_2_group1_adr.len() as c_uint, adr_d: rt1320_2_group1_adr.as_ptr() },
	NULL_LINK,
];
static ptl_sdw_rt712_vb_l3_rt1320_l3: [snd_soc_acpi_link_adr; 3] = [
	snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt712_vb_3_group1_adr.len() as c_uint, adr_d: rt712_vb_3_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt1320_3_group1_adr.len() as c_uint, adr_d: rt1320_3_group1_adr.as_ptr() },
	NULL_LINK,
];

/* this table is used when there is no I2S codec present */
#[no_mangle]
pub static mut snd_soc_acpi_intel_ptl_sdw_machines: [snd_soc_acpi_mach; 14] = [
	/* Order Priority: mockup > most links > most bit link-mask > alphabetical */
	snd_soc_acpi_mach { link_mask: GENMASK(3, 0), links: unsafe { sdw_mockup_headset_2amps_mic.as_ptr() }, drv_name: c"sof_sdw".as_ptr(), sof_tplg_filename: c"sof-ptl-rt711-rt1308-rt715.tplg".as_ptr(), ..NULL_MACH },
	snd_soc_acpi_mach { link_mask: BIT(0) | BIT(1) | BIT(3), links: unsafe { sdw_mockup_headset_1amp_mic.as_ptr() }, drv_name: c"sof_sdw".as_ptr(), sof_tplg_filename: c"sof-ptl-rt711-rt1308-mono-rt715.tplg".as_ptr(), ..NULL_MACH },
	snd_soc_acpi_mach { link_mask: GENMASK(2, 0), links: unsafe { sdw_mockup_mic_headset_1amp.as_ptr() }, drv_name: c"sof_sdw".as_ptr(), sof_tplg_filename: c"sof-ptl-rt715-rt711-rt1308-mono.tplg".as_ptr(), ..NULL_MACH },
	snd_soc_acpi_mach { link_mask: BIT(0), links: unsafe { sdw_mockup_multi_func.as_ptr() }, drv_name: c"sof_sdw".as_ptr(), sof_tplg_filename: c"sof-ptl-rt722.tplg".as_ptr(), ..NULL_MACH }, /* Reuse the existing tplg file */
	snd_soc_acpi_mach { link_mask: BIT(1) | BIT(2) | BIT(3), links: ptl_sdw_rt713_vb_l2_rt1320_l13.as_ptr(), drv_name: c"sof_sdw".as_ptr(), machine_check: Some(snd_soc_acpi_intel_sdca_is_device_rt712_vb), sof_tplg_filename: c"sof-ptl-rt713-l2-rt1320-l13.tplg".as_ptr(), get_function_tplg_files: Some(sof_sdw_get_tplg_files), ..NULL_MACH },
	snd_soc_acpi_mach { link_mask: BIT(1) | BIT(2) | BIT(3), links: ptl_sdw_rt713_vb_l3_rt1320_l12.as_ptr(), drv_name: c"sof_sdw".as_ptr(), machine_check: Some(snd_soc_acpi_intel_sdca_is_device_rt712_vb), sof_tplg_filename: c"sof-ptl-rt713-l3-rt1320-l12.tplg".as_ptr(), get_function_tplg_files: Some(sof_sdw_get_tplg_files), ..NULL_MACH },
	snd_soc_acpi_mach { link_mask: BIT(1) | BIT(3), links: ptl_sdw_rt713_vb_l3_rt1320_l1.as_ptr(), drv_name: c"sof_sdw".as_ptr(), sof_tplg_filename: c"sof-ptl-rt713-l3-rt1320-l1.tplg".as_ptr(), get_function_tplg_files: Some(sof_sdw_get_tplg_files), ..NULL_MACH },
	snd_soc_acpi_mach { link_mask: BIT(0) | BIT(2) | BIT(3), links: ptl_rt722_l0_rt1320_l23.as_ptr(), drv_name: c"sof_sdw".as_ptr(), sof_tplg_filename: c"sof-ptl-rt722-l0-rt1320-l23.tplg".as_ptr(), machine_check: Some(snd_soc_acpi_intel_no_function_topology), ..NULL_MACH },
	snd_soc_acpi_mach { link_mask: BIT(1) | BIT(2), links: ptl_sdw_rt712_vb_l2_rt1320_l1.as_ptr(), drv_name: c"sof_sdw".as_ptr(), machine_check: Some(snd_soc_acpi_intel_sdca_is_device_rt712_vb), sof_tplg_filename: c"sof-ptl-rt712-l2-rt1320-l1.tplg".as_ptr(), get_function_tplg_files: Some(sof_sdw_get_tplg_files), ..NULL_MACH },
	snd_soc_acpi_mach { link_mask: BIT(2) | BIT(3), links: ptl_sdw_rt712_vb_l3_rt1320_l2.as_ptr(), drv_name: c"sof_sdw".as_ptr(), machine_check: Some(snd_soc_acpi_intel_sdca_is_device_rt712_vb), sof_tplg_filename: c"sof-ptl-rt712-l3-rt1320-l2.tplg".as_ptr(), get_function_tplg_files: Some(sof_sdw_get_tplg_files), ..NULL_MACH },
	snd_soc_acpi_mach { link_mask: BIT(2) | BIT(3), links: ptl_cs42l43_agg_l3_cs35l56_l2.as_ptr(), drv_name: c"sof_sdw".as_ptr(), machine_check: Some(snd_soc_acpi_intel_no_function_topology), sof_tplg_filename: c"sof-ptl-cs42l43-agg-l3-cs35l56-l2.tplg".as_ptr(), ..NULL_MACH },
	snd_soc_acpi_mach { link_mask: BIT(0), links: ptl_rvp.as_ptr(), drv_name: c"sof_sdw".as_ptr(), sof_tplg_filename: c"sof-ptl-rt711.tplg".as_ptr(), get_function_tplg_files: Some(sof_sdw_get_tplg_files), ..NULL_MACH },
	snd_soc_acpi_mach { link_mask: BIT(3), links: ptl_sdw_rt712_vb_l3_rt1320_l3.as_ptr(), drv_name: c"sof_sdw".as_ptr(), machine_check: Some(snd_soc_acpi_intel_rt712_vb_no_function_topology), sof_tplg_filename: c"sof-ptl-rt712-l3-rt1320-l3.tplg".as_ptr(), ..NULL_MACH },
	NULL_MACH,
];
/* EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_ptl_sdw_machines); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
