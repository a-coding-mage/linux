// SPDX-License-Identifier: GPL-2.0-only
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, improper_ctypes)]

use core::ptr;

// Dependencies translated from C includes:
// linux/soundwire/sdw_intel.h
// sound/sdca.h
// sound/soc-acpi.h
// sound/soc-acpi-intel-match.h
// sound/soc-acpi-intel-ssp-common.h
// sof-function-topology-lib.h
// soc-acpi-intel-sdca-quirks.h
// soc-acpi-intel-sdw-mockup-match.h
/*
 * soc-acpi-intel-mtl-match.c - tables and support for MTL ACPI enumeration.
 *
 * Copyright (c) 2022, Intel Corporation.
 *
 */


static mtl_rt5682_rt5682s_hp: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 2,
	codecs: {RT5682_ACPI_HID, RT5682S_ACPI_HID},
};

static mtl_essx_83x6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 3,
	codecs: { "ESSX8316", "ESSX8326", "ESSX8336"},
};

static mtl_lt6911_hdmi: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 1,
	codecs: {"INTC10B0"}
};

pub static mut snd_soc_acpi_intel_mtl_machines: [snd_soc_acpi_mach; _] = [
	snd_soc_acpi_initializer! {
		comp_ids: ptr::addr_of!(mtl_essx_83x6),
		drv_name: "mtl_es83x6_c1_h02",
		machine_quirk: snd_soc_acpi_codec_list,
		quirk_data: ptr::addr_of!(mtl_lt6911_hdmi),
		sof_tplg_filename: "sof-mtl-es83x6-ssp1-hdmi-ssp02.tplg",
	},
	snd_soc_acpi_initializer! {
		comp_ids: ptr::addr_of!(mtl_essx_83x6),
		drv_name: "sof-essx8336",
		sof_tplg_filename: "sof-mtl-es8336", /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER |
					SND_SOC_ACPI_TPLG_INTEL_SSP_MSB |
					SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
	},
	snd_soc_acpi_initializer! {
		comp_ids: ptr::addr_of!(mtl_rt5682_rt5682s_hp),
		drv_name: "mtl_rt5682_c1_h02",
		machine_quirk: snd_soc_acpi_codec_list,
		quirk_data: ptr::addr_of!(mtl_lt6911_hdmi),
		sof_tplg_filename: "sof-mtl-rt5682-ssp1-hdmi-ssp02.tplg",
	},
	/* place boards for each headphone codec: sof driver will complete the
	 * tplg name and machine driver will detect the amp type
	 */
	snd_soc_acpi_initializer! {
		id: CS42L42_ACPI_HID,
		drv_name: "mtl_cs42l42_def",
		sof_tplg_filename: "sof-mtl", /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME |
					SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME,
	},
	snd_soc_acpi_initializer! {
		id: DA7219_ACPI_HID,
		drv_name: "mtl_da7219_def",
		sof_tplg_filename: "sof-mtl", /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME |
					SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME,
	},
	snd_soc_acpi_initializer! {
		id: NAU8825_ACPI_HID,
		drv_name: "mtl_nau8825_def",
		sof_tplg_filename: "sof-mtl", /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME |
					SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME,
	},
	snd_soc_acpi_initializer! {
		id: RT5650_ACPI_HID,
		drv_name: "mtl_rt5682_def",
		sof_tplg_filename: "sof-mtl", /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME |
					SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME,
	},
	snd_soc_acpi_initializer! {
		comp_ids: ptr::addr_of!(mtl_rt5682_rt5682s_hp),
		drv_name: "mtl_rt5682_def",
		sof_tplg_filename: "sof-mtl", /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME |
					SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME,
	},
	/* place amp-only boards in the end of table */
	snd_soc_acpi_initializer! {
		id: "INTC10B0",
		drv_name: "mtl_lt6911_hdmi_ssp",
		sof_tplg_filename: "sof-mtl-hdmi-ssp02.tplg",
	},
	Default::default(),
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_mtl_machines);

static single_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint {
	num: 0,
	aggregated: 0,
	group_position: 0,
	group_id: 0,
};

static spk_l_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint {
	num: 0,
	aggregated: 1,
	group_position: 0,
	group_id: 1,
};

static spk_r_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint {
	num: 0,
	aggregated: 1,
	group_position: 1,
	group_id: 1,
};

static tac5xx2_endpoints: [snd_soc_acpi_endpoint; _] = [
	{ /* Playback Endpoint */
		num: 0,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	{ /* Mic Capture Endpoint */
		num: 1,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	{ /* UAJ-HP with Mic Endpoint */
		num: 2,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
];

static tas2883_endpoints: [snd_soc_acpi_endpoint; _] = [
	{ /* Playback Endpoint */
		num: 0,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	{ /* Mic Capture Endpoint */
		num: 1,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
];

static rt712_endpoints: [snd_soc_acpi_endpoint; _] = [
	snd_soc_acpi_initializer! {
		num: 0,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	snd_soc_acpi_initializer! {
		num: 1,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
];

static rt712_vb_endpoints: [snd_soc_acpi_endpoint; _] = [
	snd_soc_acpi_initializer! {
		num: 0,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	snd_soc_acpi_initializer! {
		num: 1,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	snd_soc_acpi_initializer! {
		num: 2,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
];

/*
 * RT722 is a multi-function codec, three endpoints are created for
 * its headset, amp and dmic functions.
 */
static rt722_endpoints: [snd_soc_acpi_endpoint; _] = [
	snd_soc_acpi_initializer! {
		num: 0,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	snd_soc_acpi_initializer! {
		num: 1,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	snd_soc_acpi_initializer! {
		num: 2,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
];

static spk_2_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint {
	num: 0,
	aggregated: 1,
	group_position: 2,
	group_id: 1,
};

static spk_3_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint {
	num: 0,
	aggregated: 1,
	group_position: 3,
	group_id: 1,
};

static rt711_sdca_0_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000030025D071101u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(single_endpoint),
		name_prefix: "rt711"
	}
];

static rt712_0_single_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000030025D071201u64,
		num_endpoints: rt712_endpoints.len(),
		endpoints: rt712_endpoints,
		name_prefix: "rt712"
	}
];

static rt712_vb_0_single_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000030025D071201u64,
		num_endpoints: rt712_vb_endpoints.len(),
		endpoints: rt712_vb_endpoints,
		name_prefix: "rt712"
	}
];

static rt1712_3_single_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000330025D171201u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(single_endpoint),
		name_prefix: "rt712-dmic"
	}
];

static rt722_0_single_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000030025d072201u64,
		num_endpoints: rt722_endpoints.len(),
		endpoints: rt722_endpoints,
		name_prefix: "rt722"
	}
];

static rt713_0_single_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000031025D071301u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(single_endpoint),
		name_prefix: "rt713"
	}
];

static rt1713_3_single_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000331025D171301u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(single_endpoint),
		name_prefix: "rt713-dmic"
	}
];

static mx8373_0_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000023019F837300u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_l_endpoint),
		name_prefix: "Left"
	},
	snd_soc_acpi_initializer! {
		adr: 0x000027019F837300u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_r_endpoint),
		name_prefix: "Right"
	}
];

static rt5682_2_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000221025D568200u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(single_endpoint),
		name_prefix: "rt5682"
	}
];

static rt1316_2_group1_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000230025D131601u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_l_endpoint),
		name_prefix: "rt1316-1"
	}
];

static rt1316_3_group1_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000331025D131601u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_r_endpoint),
		name_prefix: "rt1316-2"
	}
];

static rt1316_1_group2_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000131025D131601u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_l_endpoint),
		name_prefix: "rt1316-1"
	}
];

static rt1316_2_group2_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000230025D131601u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_r_endpoint),
		name_prefix: "rt1316-2"
	}
];

static rt1316_3_single_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000330025D131601u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(single_endpoint),
		name_prefix: "rt1316-1"
	}
];

static rt1318_1_single_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000130025D131801u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(single_endpoint),
		name_prefix: "rt1318-1"
	}
];

static rt1318_1_group1_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000130025D131801u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_l_endpoint),
		name_prefix: "rt1318-1"
	}
];

static rt1318_2_group1_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000232025D131801u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_r_endpoint),
		name_prefix: "rt1318-2"
	}
];

static rt714_0_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000030025D071401u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(single_endpoint),
		name_prefix: "rt714"
	}
];

static rt714_1_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000130025D071401u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(single_endpoint),
		name_prefix: "rt714"
	}
];

static mtl_712_l0_1712_l3: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: rt712_0_single_adr.len(),
		adr_d: rt712_0_single_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 3),
		num_adr: rt1712_3_single_adr.len(),
		adr_d: rt1712_3_single_adr,
	},
	Default::default()
];

static mtl_712_l0: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: rt712_0_single_adr.len(),
		adr_d: rt712_0_single_adr,
	},
	Default::default()
];

static mtl_712_vb_l0: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: rt712_vb_0_single_adr.len(),
		adr_d: rt712_vb_0_single_adr,
	},
	Default::default()
];

static cs42l43_endpoints: [snd_soc_acpi_endpoint; _] = [
	{ /* Jack Playback Endpoint */
		num: 0,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	{ /* DMIC Capture Endpoint */
		num: 1,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	{ /* Jack Capture Endpoint */
		num: 2,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	{ /* Speaker Playback Endpoint */
		num: 3,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
];

static cs42l43_0_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00003001FA424301u64,
		num_endpoints: cs42l43_endpoints.len(),
		endpoints: cs42l43_endpoints,
		name_prefix: "cs42l43"
	}
];

/* CS42L43 - speaker DAI aggregated with 4 amps */
static cs42l43_4amp_spkagg_endpoints: [snd_soc_acpi_endpoint; _] = [
	{ /* Jack Playback Endpoint */
		num: 0,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	{ /* DMIC Capture Endpoint */
		num: 1,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	{ /* Jack Capture Endpoint */
		num: 2,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	{ /* Speaker Playback Endpoint */
		num: 3,
		aggregated: 1,
		group_position: 4,
		group_id: 1,
	},
];

/* CS42L43 on link3 aggregated with 4 amps */
static cs42l43_l3_4amp_spkagg_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00033001FA424301u64,
		num_endpoints: cs42l43_4amp_spkagg_endpoints.len(),
		endpoints: cs42l43_4amp_spkagg_endpoints,
		name_prefix: "cs42l43"
	}
];

static cs35l56_l_fb_endpoints: [snd_soc_acpi_endpoint; _] = [
	{ /* Speaker Playback Endpoint */
		num: 0,
		aggregated: 1,
		group_position: 0,
		group_id: 1,
	},
	{ /* Feedback Capture Endpoint */
		num: 1,
		aggregated: 1,
		group_position: 0,
		group_id: 2,
	},
];

static cs35l56_r_fb_endpoints: [snd_soc_acpi_endpoint; _] = [
	{ /* Speaker Playback Endpoint */
		num: 0,
		aggregated: 1,
		group_position: 1,
		group_id: 1,
	},
	{ /* Feedback Capture Endpoint */
		num: 1,
		aggregated: 1,
		group_position: 1,
		group_id: 2,
	},
];

static cs35l56_2_fb_endpoints: [snd_soc_acpi_endpoint; _] = [
	{ /* Speaker Playback Endpoint */
		num: 0,
		aggregated: 1,
		group_position: 2,
		group_id: 1,
	},
	{ /* Feedback Capture Endpoint */
		num: 1,
		aggregated: 1,
		group_position: 2,
		group_id: 2,
	},
];

static cs35l56_3_fb_endpoints: [snd_soc_acpi_endpoint; _] = [
	{ /* Speaker Playback Endpoint */
		num: 0,
		aggregated: 1,
		group_position: 3,
		group_id: 1,
	},
	{ /* Feedback Capture Endpoint */
		num: 1,
		aggregated: 1,
		group_position: 3,
		group_id: 2,
	},
];

static cs35l56_4_fb_endpoints: [snd_soc_acpi_endpoint; _] = [
	{ /* Speaker Playback Endpoint */
		num: 0,
		aggregated: 1,
		group_position: 4,
		group_id: 1,
	},
	{ /* Feedback Capture Endpoint */
		num: 1,
		aggregated: 1,
		group_position: 4,
		group_id: 2,
	},
];

static cs35l56_5_fb_endpoints: [snd_soc_acpi_endpoint; _] = [
	{ /* Speaker Playback Endpoint */
		num: 0,
		aggregated: 1,
		group_position: 5,
		group_id: 1,
	},
	{ /* Feedback Capture Endpoint */
		num: 1,
		aggregated: 1,
		group_position: 5,
		group_id: 2,
	},
];

static cs35l56_6_fb_endpoints: [snd_soc_acpi_endpoint; _] = [
	{ /* Speaker Playback Endpoint */
		num: 0,
		aggregated: 1,
		group_position: 6,
		group_id: 1,
	},
	{ /* Feedback Capture Endpoint */
		num: 1,
		aggregated: 1,
		group_position: 6,
		group_id: 2,
	},
];

static cs35l56_7_fb_endpoints: [snd_soc_acpi_endpoint; _] = [
	{ /* Speaker Playback Endpoint */
		num: 0,
		aggregated: 1,
		group_position: 7,
		group_id: 1,
	},
	{ /* Feedback Capture Endpoint */
		num: 1,
		aggregated: 1,
		group_position: 7,
		group_id: 2,
	},
];

static cs35l56_0_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00003301FA355601u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_l_endpoint),
		name_prefix: "AMP1"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00003201FA355601u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_2_endpoint),
		name_prefix: "AMP2"
	}
];

static cs35l56_1_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00013701FA355601u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_r_endpoint),
		name_prefix: "AMP3"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00013601FA355601u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_3_endpoint),
		name_prefix: "AMP4"
	}
];

static cs35l56_2_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00023301FA355601u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_l_endpoint),
		name_prefix: "AMP1"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00023201FA355601u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_2_endpoint),
		name_prefix: "AMP2"
	}
];

static cs35l56_0_fb_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00003301FA355601u64,
		num_endpoints: cs35l56_l_fb_endpoints.len(),
		endpoints: cs35l56_l_fb_endpoints,
		name_prefix: "AMP1"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00003201FA355601u64,
		num_endpoints: cs35l56_2_fb_endpoints.len(),
		endpoints: cs35l56_2_fb_endpoints,
		name_prefix: "AMP2"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00003101FA355601u64,
		num_endpoints: cs35l56_4_fb_endpoints.len(),
		endpoints: cs35l56_4_fb_endpoints,
		name_prefix: "AMP3"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00003001FA355601u64,
		num_endpoints: cs35l56_6_fb_endpoints.len(),
		endpoints: cs35l56_6_fb_endpoints,
		name_prefix: "AMP4"
	},
];

static cs35l56_1_fb_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00013701FA355601u64,
		num_endpoints: cs35l56_r_fb_endpoints.len(),
		endpoints: cs35l56_r_fb_endpoints,
		name_prefix: "AMP8"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00013601FA355601u64,
		num_endpoints: cs35l56_3_fb_endpoints.len(),
		endpoints: cs35l56_3_fb_endpoints,
		name_prefix: "AMP7"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00013501FA355601u64,
		num_endpoints: cs35l56_5_fb_endpoints.len(),
		endpoints: cs35l56_5_fb_endpoints,
		name_prefix: "AMP6"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00013401FA355601u64,
		num_endpoints: cs35l56_7_fb_endpoints.len(),
		endpoints: cs35l56_7_fb_endpoints,
		name_prefix: "AMP5"
	},
];

static cs35l56_6amp_1_fb_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00013701FA355601u64,
		num_endpoints: cs35l56_r_fb_endpoints.len(),
		endpoints: cs35l56_r_fb_endpoints,
		name_prefix: "AMP6"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00013601FA355601u64,
		num_endpoints: cs35l56_3_fb_endpoints.len(),
		endpoints: cs35l56_3_fb_endpoints,
		name_prefix: "AMP5"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00013501FA355601u64,
		num_endpoints: cs35l56_5_fb_endpoints.len(),
		endpoints: cs35l56_5_fb_endpoints,
		name_prefix: "AMP4"
	},
];

static cs35l63_6amp_3_fb_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00033001FA356301u64,
		num_endpoints: cs35l56_l_fb_endpoints.len(),
		endpoints: cs35l56_l_fb_endpoints,
		name_prefix: "AMP1"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00033201FA356301u64,
		num_endpoints: cs35l56_2_fb_endpoints.len(),
		endpoints: cs35l56_2_fb_endpoints,
		name_prefix: "AMP3"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00033401FA356301u64,
		num_endpoints: cs35l56_4_fb_endpoints.len(),
		endpoints: cs35l56_4_fb_endpoints,
		name_prefix: "AMP5"
	},
];

static cs35l63_6amp_2_fb_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00023101FA356301u64,
		num_endpoints: cs35l56_r_fb_endpoints.len(),
		endpoints: cs35l56_r_fb_endpoints,
		name_prefix: "AMP2"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00023301FA356301u64,
		num_endpoints: cs35l56_3_fb_endpoints.len(),
		endpoints: cs35l56_3_fb_endpoints,
		name_prefix: "AMP4"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00023501FA356301u64,
		num_endpoints: cs35l56_5_fb_endpoints.len(),
		endpoints: cs35l56_5_fb_endpoints,
		name_prefix: "AMP6"
	},
];

static cs35l56_2_r_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00023201FA355601u64,
		num_endpoints: cs35l56_r_fb_endpoints.len(),
		endpoints: cs35l56_r_fb_endpoints,
		name_prefix: "AMP3"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00023301FA355601u64,
		num_endpoints: cs35l56_3_fb_endpoints.len(),
		endpoints: cs35l56_3_fb_endpoints,
		name_prefix: "AMP4"
	}

];

static cs35l56_3_l_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00033001fa355601u64,
		num_endpoints: cs35l56_l_fb_endpoints.len(),
		endpoints: cs35l56_l_fb_endpoints,
		name_prefix: "AMP1"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00033101fa355601u64,
		num_endpoints: cs35l56_2_fb_endpoints.len(),
		endpoints: cs35l56_2_fb_endpoints,
		name_prefix: "AMP2"
	}
];

static cs35l63_1_fb_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00013001FA356301u64,
		num_endpoints: cs35l56_l_fb_endpoints.len(),
		endpoints: cs35l56_l_fb_endpoints,
		name_prefix: "AMP1"
	},
];

static cs35l63_3_fb_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00033101FA356301u64,
		num_endpoints: cs35l56_r_fb_endpoints.len(),
		endpoints: cs35l56_r_fb_endpoints,
		name_prefix: "AMP2"
	},
];

static rt5682_link2_max98373_link0: [snd_soc_acpi_link_adr; _] = [
	/* Expected order: jack -> amp */
	snd_soc_acpi_initializer! {
		mask: (1u32 << 2),
		num_adr: rt5682_2_adr.len(),
		adr_d: rt5682_2_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: mx8373_0_adr.len(),
		adr_d: mx8373_0_adr,
	},
	Default::default()
];

static mtl_rvp: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: rt711_sdca_0_adr.len(),
		adr_d: rt711_sdca_0_adr,
	},
	Default::default()
];

static mtl_rt722_only: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: rt722_0_single_adr.len(),
		adr_d: rt722_0_single_adr,
	},
	Default::default()
];

static mtl_3_in_1_sdca: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: rt711_sdca_0_adr.len(),
		adr_d: rt711_sdca_0_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 2),
		num_adr: rt1316_2_group1_adr.len(),
		adr_d: rt1316_2_group1_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 3),
		num_adr: rt1316_3_group1_adr.len(),
		adr_d: rt1316_3_group1_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 1),
		num_adr: rt714_1_adr.len(),
		adr_d: rt714_1_adr,
	},
	Default::default()
];

static mtl_sdw_rt1318_l12_rt714_l0: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 1),
		num_adr: rt1318_1_group1_adr.len(),
		adr_d: rt1318_1_group1_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 2),
		num_adr: rt1318_2_group1_adr.len(),
		adr_d: rt1318_2_group1_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: rt714_0_adr.len(),
		adr_d: rt714_0_adr,
	},
	Default::default()
];

static mtl_rt713_l0_rt1316_l12_rt1713_l3: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: rt713_0_single_adr.len(),
		adr_d: rt713_0_single_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 1),
		num_adr: rt1316_1_group2_adr.len(),
		adr_d: rt1316_1_group2_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 2),
		num_adr: rt1316_2_group2_adr.len(),
		adr_d: rt1316_2_group2_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 3),
		num_adr: rt1713_3_single_adr.len(),
		adr_d: rt1713_3_single_adr,
	},
	Default::default()
];

static mtl_rt713_l0_rt1318_l1_rt1713_l3: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: rt713_0_single_adr.len(),
		adr_d: rt713_0_single_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 1),
		num_adr: rt1318_1_single_adr.len(),
		adr_d: rt1318_1_single_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 3),
		num_adr: rt1713_3_single_adr.len(),
		adr_d: rt1713_3_single_adr,
	},
	Default::default()
];

static mtl_rt713_l0_rt1318_l12_rt1713_l3: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: rt713_0_single_adr.len(),
		adr_d: rt713_0_single_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 1),
		num_adr: rt1318_1_group1_adr.len(),
		adr_d: rt1318_1_group1_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 2),
		num_adr: rt1318_2_group1_adr.len(),
		adr_d: rt1318_2_group1_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 3),
		num_adr: rt1713_3_single_adr.len(),
		adr_d: rt1713_3_single_adr,
	},
	Default::default()
];

static mtl_rt713_l0_rt1316_l12: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: rt713_0_single_adr.len(),
		adr_d: rt713_0_single_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 1),
		num_adr: rt1316_1_group2_adr.len(),
		adr_d: rt1316_1_group2_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 2),
		num_adr: rt1316_2_group2_adr.len(),
		adr_d: rt1316_2_group2_adr,
	},
	Default::default()
];

static mtl_rt711_l0_rt1316_l3: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: rt711_sdca_0_adr.len(),
		adr_d: rt711_sdca_0_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 3),
		num_adr: rt1316_3_single_adr.len(),
		adr_d: rt1316_3_single_adr,
	},
	Default::default()
];

static mx8363_2_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x000230019F836300u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_l_endpoint),
		name_prefix: "Left"
	},
	snd_soc_acpi_initializer! {
		adr: 0x000231019F836300u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_r_endpoint),
		name_prefix: "Right"
	}
];

static cs42l42_0_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00001001FA424200u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(single_endpoint),
		name_prefix: "cs42l42"
	}
];

static tac5572_0_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x0000300102557201u64,
		num_endpoints: tac5xx2_endpoints.len(),
		endpoints: tac5xx2_endpoints,
		name_prefix: "tac5572"
	}
];

static tac5672_0_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x0000300102567201u64,
		num_endpoints: tac5xx2_endpoints.len(),
		endpoints: tac5xx2_endpoints,
		name_prefix: "tac5672"
	}
];

static tac5682_0_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x0000300102568201u64,
		num_endpoints: tac5xx2_endpoints.len(),
		endpoints: tac5xx2_endpoints,
		name_prefix: "tac5682"
	}
];

static tas2783_0_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x00003c0102000001u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_l_endpoint),
		name_prefix: "tas2783-1"
	},
	snd_soc_acpi_initializer! {
		adr: 0x0000390102000001u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_r_endpoint),
		name_prefix: "tas2783-2"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00003d0102000001u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_l_endpoint),
		name_prefix: "tas2783-3"
	},
	snd_soc_acpi_initializer! {
		adr: 0x00003a0102000001u64,
		num_endpoints: 1,
		endpoints: ptr::addr_of!(spk_r_endpoint),
		name_prefix: "tas2783-4"
	},
];

static tas2883_0_adr: [snd_soc_acpi_adr_device; _] = [
	snd_soc_acpi_initializer! {
		adr: 0x0000300102288301u64,
		num_endpoints: tas2883_endpoints.len(),
		endpoints: tas2883_endpoints,
		name_prefix: "tas2883"
	}
];

static tac5572_l0: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: tac5572_0_adr.len(),
		adr_d: tac5572_0_adr,
	},
	Default::default()
];

static tac5672_l0: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: tac5672_0_adr.len(),
		adr_d: tac5672_0_adr,
	},
	Default::default()
];

static tac5682_l0: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: tac5682_0_adr.len(),
		adr_d: tac5682_0_adr,
	},
	Default::default()
];

static tas2783_link0: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: tas2783_0_adr.len(),
		adr_d: tas2783_0_adr,
	},
	Default::default()
];

static tas2883_l0: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: tas2883_0_adr.len(),
		adr_d: tas2883_0_adr,
	},
	Default::default()
];

static cs42l42_link0_max98363_link2: [snd_soc_acpi_link_adr; _] = [
	/* Expected order: jack -> amp */
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: cs42l42_0_adr.len(),
		adr_d: cs42l42_0_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 2),
		num_adr: mx8363_2_adr.len(),
		adr_d: mx8363_2_adr,
	},
	Default::default()
];

static mtl_cs42l43_l0: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: cs42l43_0_adr.len(),
		adr_d: cs42l43_0_adr,
	},
	Default::default()
];

static mtl_cs42l43_cs35l56: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: cs42l43_0_adr.len(),
		adr_d: cs42l43_0_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 1),
		num_adr: cs35l56_1_adr.len(),
		adr_d: cs35l56_1_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 2),
		num_adr: cs35l56_2_adr.len(),
		adr_d: cs35l56_2_adr,
	},
	Default::default()
];

static cs42l43_link0_cs35l56_link2_link3: [snd_soc_acpi_link_adr; _] = [
	/* Expected order: jack -> amp */
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: cs42l43_0_adr.len(),
		adr_d: cs42l43_0_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 2),
		num_adr: cs35l56_2_r_adr.len(),
		adr_d: cs35l56_2_r_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 3),
		num_adr: cs35l56_3_l_adr.len(),
		adr_d: cs35l56_3_l_adr,
	},
	Default::default()
];

static cs42l43_link3_cs35l56_x4_link0_link1_spkagg: [snd_soc_acpi_link_adr; _] = [
	/* Expected order: jack -> amp */
	snd_soc_acpi_initializer! {
		mask: (1u32 << 3),
		num_adr: cs42l43_l3_4amp_spkagg_adr.len(),
		adr_d: cs42l43_l3_4amp_spkagg_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 1),
		num_adr: 2,
		adr_d: cs35l56_1_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: 2,
		adr_d: cs35l56_0_adr,
	},
	Default::default()
];

static mtl_cs35l56_x8_link0_link1_fb: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 1),
		num_adr: cs35l56_1_fb_adr.len(),
		adr_d: cs35l56_1_fb_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		num_adr: cs35l56_0_fb_adr.len(),
		adr_d: cs35l56_0_fb_adr,
	},
	Default::default()
];

static mtl_cs35l56_x6_link0_link1_fb: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 1),
		num_adr: cs35l56_6amp_1_fb_adr.len(),
		adr_d: cs35l56_6amp_1_fb_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 0),
		/* First 3 amps in cs35l56_0_fb_adr */
		num_adr: 3,
		adr_d: cs35l56_0_fb_adr,
	},
	Default::default()
];

static mtl_cs35l63_x6_link2_link3_fb: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 3),
		num_adr: cs35l63_6amp_3_fb_adr.len(),
		adr_d: cs35l63_6amp_3_fb_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 2),
		num_adr: cs35l63_6amp_2_fb_adr.len(),
		adr_d: cs35l63_6amp_2_fb_adr,
	},
	Default::default()
];

static mtl_cs35l63_x2_link1_link3_fb: [snd_soc_acpi_link_adr; _] = [
	snd_soc_acpi_initializer! {
		mask: (1u32 << 3),
		num_adr: cs35l63_3_fb_adr.len(),
		adr_d: cs35l63_3_fb_adr,
	},
	snd_soc_acpi_initializer! {
		mask: (1u32 << 1),
		num_adr: cs35l63_1_fb_adr.len(),
		adr_d: cs35l63_1_fb_adr,
	},
	Default::default()
];

/* this table is used when there is no I2S codec present */
pub static mut snd_soc_acpi_intel_mtl_sdw_machines: [snd_soc_acpi_mach; _] = [
	/* mockup tests need to be first */
	snd_soc_acpi_initializer! {
		link_mask: GENMASK(3, 0),
		links: sdw_mockup_headset_2amps_mic,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt711-rt1308-rt715.tplg",
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0) | (1u32 << 1) | (1u32 << 3),
		links: sdw_mockup_headset_1amp_mic,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt711-rt1308-mono-rt715.tplg",
	},
	snd_soc_acpi_initializer! {
		link_mask: GENMASK(2, 0),
		links: sdw_mockup_mic_headset_1amp,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt715-rt711-rt1308-mono.tplg",
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0),
		links: tac5572_l0,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-tac5572.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0),
		links: tac5672_l0,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-tac5672.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0),
		links: tac5682_l0,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-tac5682.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0),
		links: tas2783_link0,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-tas2783.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0),
		links: tas2883_l0,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-tas2883.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: GENMASK(3, 0),
		links: mtl_rt713_l0_rt1316_l12_rt1713_l3,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt713-l0-rt1316-l12-rt1713-l3.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: GENMASK(3, 0),
		links: mtl_rt713_l0_rt1318_l12_rt1713_l3,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt713-l0-rt1318-l12-rt1713-l3.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0) | (1u32 << 1) | (1u32 << 3),
		links: mtl_rt713_l0_rt1318_l1_rt1713_l3,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt713-l0-rt1318-l1-rt1713-l3.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: GENMASK(2, 0),
		links: mtl_rt713_l0_rt1316_l12,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt713-l0-rt1316-l12.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 3) | (1u32 << 0),
		links: mtl_712_l0_1712_l3,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt712-l0-rt1712-l3.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0),
		links: mtl_712_vb_l0,
		drv_name: "sof_sdw",
		machine_check: snd_soc_acpi_intel_sdca_is_device_rt712_vb,
		sof_tplg_filename: "sof-mtl-rt712-vb-l0.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0),
		links: mtl_712_l0,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt712-l0.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: GENMASK(2, 0),
		links: mtl_sdw_rt1318_l12_rt714_l0,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt1318-l12-rt714-l0.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0) | (1u32 << 2) | (1u32 << 3),
		links: cs42l43_link0_cs35l56_link2_link3,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-cs42l43-l0-cs35l56-l23.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0) | (1u32 << 1) | (1u32 << 3),
		links: cs42l43_link3_cs35l56_x4_link0_link1_spkagg,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-cs42l43-l3-cs35l56-l01-spkagg.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: GENMASK(2, 0),
		links: mtl_cs42l43_cs35l56,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-cs42l43-l0-cs35l56-l12.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0) | (1u32 << 1),
		links: mtl_cs35l56_x8_link0_link1_fb,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-cs35l56-l01-fb8.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0) | (1u32 << 1),
		links: mtl_cs35l56_x6_link0_link1_fb,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-cs35l56-l01-fb6.tplg"
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0),
		links: mtl_cs42l43_l0,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-cs42l43-l0.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 1) | (1u32 << 3),
		links: mtl_cs35l63_x2_link1_link3_fb,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-cs35l56-l01-fb8.tplg",
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 2) | (1u32 << 3),
		links: mtl_cs35l63_x6_link2_link3_fb,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-cs35l56-l01-fb6.tplg",
	},
	snd_soc_acpi_initializer! {
		link_mask: GENMASK(3, 0),
		links: mtl_3_in_1_sdca,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt711-l0-rt1316-l23-rt714-l1.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: 0x9, /* 2 active links required */
		links: mtl_rt711_l0_rt1316_l3,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt711-l0-rt1316-l3.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0),
		links: mtl_rt722_only,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt722-l0.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0),
		links: mtl_rvp,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-rt711.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0) | (1u32 << 2),
		links: rt5682_link2_max98373_link0,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-sdw-rt5682-l2-max98373-l0.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	snd_soc_acpi_initializer! {
		link_mask: (1u32 << 0) | (1u32 << 2),
		links: cs42l42_link0_max98363_link2,
		drv_name: "sof_sdw",
		sof_tplg_filename: "sof-mtl-sdw-cs42l42-l0-max98363-l2.tplg",
		get_function_tplg_files: sof_sdw_get_tplg_files,
	},
	Default::default(),
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_mtl_sdw_machines);

// MODULE_IMPORT_NS("SND_SOC_ACPI_INTEL_SDCA_QUIRKS");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
