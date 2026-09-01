// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-apci-intel-rpl-match.c - tables and support for RPL ACPI enumeration.
 *
 * Copyright (c) 2022 Intel Corporation.
 */

/* Original C dependencies:
 * #include <sound/soc-acpi.h>
 * #include <sound/soc-acpi-intel-match.h>
 * #include <sound/soc-acpi-intel-ssp-common.h>
 */

const fn BIT(nr: u32) -> u32 {
	1u32 << nr
}

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

static cs42l43_endpoints: [snd_soc_acpi_endpoint; 4] = [
	snd_soc_acpi_endpoint { /* Jack Playback Endpoint */
		num: 0,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	snd_soc_acpi_endpoint { /* DMIC Capture Endpoint */
		num: 1,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	snd_soc_acpi_endpoint { /* Jack Capture Endpoint */
		num: 2,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
	snd_soc_acpi_endpoint { /* Speaker Playback Endpoint */
		num: 3,
		aggregated: 0,
		group_position: 0,
		group_id: 0,
	},
];

static cs42l43_0_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x00003001FA424301u64,
		num_endpoints: cs42l43_endpoints.len(),
		endpoints: cs42l43_endpoints.as_ptr(),
		name_prefix: b"cs42l43\0".as_ptr() as *const _,
	},
];

static rt711_0_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000020025D071100u64,
		num_endpoints: 1,
		endpoints: &single_endpoint,
		name_prefix: b"rt711\0".as_ptr() as *const _,
	},
];

static rpl_rvp: [snd_soc_acpi_link_adr; 2] = [
	snd_soc_acpi_link_adr {
		mask: BIT(0),
		num_adr: rt711_0_adr.len(),
		adr_d: rt711_0_adr.as_ptr(),
	},
	snd_soc_acpi_link_adr {},
];

static rt711_sdca_0_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000030025D071101u64,
		num_endpoints: 1,
		endpoints: &single_endpoint,
		name_prefix: b"rt711\0".as_ptr() as *const _,
	},
];

static rt711_sdca_2_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000230025D071101u64,
		num_endpoints: 1,
		endpoints: &single_endpoint,
		name_prefix: b"rt711\0".as_ptr() as *const _,
	},
];

static rt1316_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000131025D131601u64, /* unique ID is set for some reason */
		num_endpoints: 1,
		endpoints: &spk_l_endpoint,
		name_prefix: b"rt1316-1\0".as_ptr() as *const _,
	},
];

static rt1316_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000230025D131601u64,
		num_endpoints: 1,
		endpoints: &spk_r_endpoint,
		name_prefix: b"rt1316-2\0".as_ptr() as *const _,
	},
];

static rt1316_3_group1_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000330025D131601u64,
		num_endpoints: 1,
		endpoints: &spk_r_endpoint,
		name_prefix: b"rt1316-2\0".as_ptr() as *const _,
	},
];

static rt1316_0_group2_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000030025D131601u64,
		num_endpoints: 1,
		endpoints: &spk_l_endpoint,
		name_prefix: b"rt1316-1\0".as_ptr() as *const _,
	},
];

static rt1316_1_group2_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000131025D131601u64,
		num_endpoints: 1,
		endpoints: &spk_r_endpoint,
		name_prefix: b"rt1316-2\0".as_ptr() as *const _,
	},
];

static rt1318_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000132025D131801u64,
		num_endpoints: 1,
		endpoints: &spk_l_endpoint,
		name_prefix: b"rt1318-1\0".as_ptr() as *const _,
	},
];

static rt1318_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000230025D131801u64,
		num_endpoints: 1,
		endpoints: &spk_r_endpoint,
		name_prefix: b"rt1318-2\0".as_ptr() as *const _,
	},
];

static rt714_0_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000030025D071401u64,
		num_endpoints: 1,
		endpoints: &single_endpoint,
		name_prefix: b"rt714\0".as_ptr() as *const _,
	},
];

static rt714_2_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000230025D071401u64,
		num_endpoints: 1,
		endpoints: &single_endpoint,
		name_prefix: b"rt714\0".as_ptr() as *const _,
	},
];

static rt714_3_adr: [snd_soc_acpi_adr_device; 1] = [
	snd_soc_acpi_adr_device {
		adr: 0x000330025D071401u64,
		num_endpoints: 1,
		endpoints: &single_endpoint,
		name_prefix: b"rt714\0".as_ptr() as *const _,
	},
];

static rpl_cs42l43_l0: [snd_soc_acpi_link_adr; 2] = [
	snd_soc_acpi_link_adr {
		mask: BIT(0),
		num_adr: cs42l43_0_adr.len(),
		adr_d: cs42l43_0_adr.as_ptr(),
	},
	snd_soc_acpi_link_adr {},
];

static rpl_sdca_3_in_1: [snd_soc_acpi_link_adr; 5] = [
	snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len(), adr_d: rt711_sdca_0_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1316_1_group1_adr.len(), adr_d: rt1316_1_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt714_2_adr.len(), adr_d: rt714_2_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt1316_3_group1_adr.len(), adr_d: rt1316_3_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr {},
];

static rpl_sdw_rt711_link0_rt1316_link12_rt714_link3: [snd_soc_acpi_link_adr; 5] = [
	snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len(), adr_d: rt711_sdca_0_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1316_1_group1_adr.len(), adr_d: rt1316_1_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1316_2_group1_adr.len(), adr_d: rt1316_2_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt714_3_adr.len(), adr_d: rt714_3_adr.as_ptr() },
	snd_soc_acpi_link_adr {},
];

static rpl_sdw_rt711_link2_rt1316_link01_rt714_link3: [snd_soc_acpi_link_adr; 5] = [
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt711_sdca_2_adr.len(), adr_d: rt711_sdca_2_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt1316_0_group2_adr.len(), adr_d: rt1316_0_group2_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1316_1_group2_adr.len(), adr_d: rt1316_1_group2_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt714_3_adr.len(), adr_d: rt714_3_adr.as_ptr() },
	snd_soc_acpi_link_adr {},
];

static rpl_sdw_rt711_link2_rt1316_link01: [snd_soc_acpi_link_adr; 4] = [
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt711_sdca_2_adr.len(), adr_d: rt711_sdca_2_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt1316_0_group2_adr.len(), adr_d: rt1316_0_group2_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1316_1_group2_adr.len(), adr_d: rt1316_1_group2_adr.as_ptr() },
	snd_soc_acpi_link_adr {},
];

static rpl_sdw_rt711_link0_rt1316_link12: [snd_soc_acpi_link_adr; 4] = [
	snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len(), adr_d: rt711_sdca_0_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1316_1_group1_adr.len(), adr_d: rt1316_1_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1316_2_group1_adr.len(), adr_d: rt1316_2_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr {},
];

static rpl_sdw_rt711_link0_rt1318_link12_rt714_link3: [snd_soc_acpi_link_adr; 5] = [
	snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len(), adr_d: rt711_sdca_0_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1318_1_group1_adr.len(), adr_d: rt1318_1_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1318_2_group1_adr.len(), adr_d: rt1318_2_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt714_3_adr.len(), adr_d: rt714_3_adr.as_ptr() },
	snd_soc_acpi_link_adr {},
];

static rpl_sdw_rt711_link0_rt1318_link12: [snd_soc_acpi_link_adr; 4] = [
	snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len(), adr_d: rt711_sdca_0_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1318_1_group1_adr.len(), adr_d: rt1318_1_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1318_2_group1_adr.len(), adr_d: rt1318_2_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr {},
];

static rpl_sdw_rt1316_link12_rt714_link0: [snd_soc_acpi_link_adr; 4] = [
	snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1316_1_group1_adr.len(), adr_d: rt1316_1_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1316_2_group1_adr.len(), adr_d: rt1316_2_group1_adr.as_ptr() },
	snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt714_0_adr.len(), adr_d: rt714_0_adr.as_ptr() },
	snd_soc_acpi_link_adr {},
];

static rpl_sdca_rvp: [snd_soc_acpi_link_adr; 2] = [
	snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len(), adr_d: rt711_sdca_0_adr.as_ptr() },
	snd_soc_acpi_link_adr {},
];

static rplp_crb: [snd_soc_acpi_link_adr; 2] = [
	snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt711_sdca_2_adr.len(), adr_d: rt711_sdca_2_adr.as_ptr() },
	snd_soc_acpi_link_adr {},
];

static rpl_rt5682_hp: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 2,
	codecs: [RT5682_ACPI_HID, RT5682S_ACPI_HID],
};

static rpl_essx_83x6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 3,
	codecs: [
		b"ESSX8316\0".as_ptr() as *const _,
		b"ESSX8326\0".as_ptr() as *const _,
		b"ESSX8336\0".as_ptr() as *const _,
	],
};

static rpl_max98357a_amp: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 1,
	codecs: [b"MX98357A\0".as_ptr() as *const _],
};

static rpl_lt6911_hdmi: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 1,
	codecs: [b"INTC10B0\0".as_ptr() as *const _],
};

#[no_mangle]
pub static mut snd_soc_acpi_intel_rpl_machines: [snd_soc_acpi_mach; 11] = [
	snd_soc_acpi_mach {
		comp_ids: &rpl_rt5682_hp,
		drv_name: b"rpl_mx98357_rt5682\0".as_ptr() as *const _,
		machine_quirk: Some(snd_soc_acpi_codec_list),
		quirk_data: &rpl_max98357a_amp as *const _ as *const _,
		sof_tplg_filename: b"sof-rpl-max98357a-rt5682.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		comp_ids: &rpl_rt5682_hp,
		drv_name: b"rpl_rt5682_c1_h02\0".as_ptr() as *const _,
		machine_quirk: Some(snd_soc_acpi_codec_list),
		quirk_data: &rpl_lt6911_hdmi as *const _ as *const _,
		sof_tplg_filename: b"sof-rpl-rt5682-ssp1-hdmi-ssp02.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		comp_ids: &rpl_essx_83x6,
		drv_name: b"rpl_es83x6_c1_h02\0".as_ptr() as *const _,
		machine_quirk: Some(snd_soc_acpi_codec_list),
		quirk_data: &rpl_lt6911_hdmi as *const _ as *const _,
		sof_tplg_filename: b"sof-rpl-es83x6-ssp1-hdmi-ssp02.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		comp_ids: &rpl_essx_83x6,
		drv_name: b"sof-essx8336\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-es83x6\0".as_ptr() as *const _, /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER |
			SND_SOC_ACPI_TPLG_INTEL_SSP_MSB |
			SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
	},
	/* place boards for each headphone codec: sof driver will complete the
	 * tplg name and machine driver will detect the amp type
	 */
	snd_soc_acpi_mach {
		id: CS42L42_ACPI_HID,
		drv_name: b"rpl_cs42l42_def\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl\0".as_ptr() as *const _, /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME |
			SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME,
	},
	snd_soc_acpi_mach {
		id: DA7219_ACPI_HID,
		drv_name: b"rpl_da7219_def\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl\0".as_ptr() as *const _, /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME |
			SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME,
	},
	snd_soc_acpi_mach {
		id: NAU8825_ACPI_HID,
		drv_name: b"rpl_nau8825_def\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl\0".as_ptr() as *const _, /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME |
			SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME,
	},
	snd_soc_acpi_mach {
		id: RT5650_ACPI_HID,
		drv_name: b"rpl_rt5682_def\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl\0".as_ptr() as *const _, /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME |
			SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME,
	},
	snd_soc_acpi_mach {
		comp_ids: &rpl_rt5682_hp,
		drv_name: b"rpl_rt5682_def\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl\0".as_ptr() as *const _, /* the tplg suffix is added at run time */
		tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME |
			SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME,
	},
	/* place amp-only boards in the end of table */
	snd_soc_acpi_mach {
		id: b"INTC10B0\0".as_ptr() as *const _,
		drv_name: b"rpl_lt6911_hdmi_ssp\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-nocodec-hdmi-ssp02.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {},
];
/* EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_rpl_machines); */

/* this table is used when there is no I2S codec present */
#[no_mangle]
pub static mut snd_soc_acpi_intel_rpl_sdw_machines: [snd_soc_acpi_mach; 13] = [
	snd_soc_acpi_mach {
		link_mask: BIT(0),
		links: rpl_cs42l43_l0.as_ptr(),
		drv_name: b"sof_sdw\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-cs42l43-l0.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		link_mask: 0xF, /* 4 active links required */
		links: rpl_sdca_3_in_1.as_ptr(),
		drv_name: b"sof_sdw\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-rt711-l0-rt1316-l13-rt714-l2.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		link_mask: 0xF, /* 4 active links required */
		links: rpl_sdw_rt711_link2_rt1316_link01_rt714_link3.as_ptr(),
		drv_name: b"sof_sdw\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-rt711-l2-rt1316-l01-rt714-l3.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		link_mask: 0xF, /* 4 active links required */
		links: rpl_sdw_rt711_link0_rt1316_link12_rt714_link3.as_ptr(),
		drv_name: b"sof_sdw\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-rt711-l0-rt1316-l12-rt714-l3.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		link_mask: 0xF, /* 4 active links required */
		links: rpl_sdw_rt711_link0_rt1318_link12_rt714_link3.as_ptr(),
		drv_name: b"sof_sdw\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-rt711-l0-rt1318-l12-rt714-l3.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		link_mask: 0x7, /* rt711 on link0 & two rt1316s on link1 and link2 */
		links: rpl_sdw_rt711_link0_rt1316_link12.as_ptr(),
		drv_name: b"sof_sdw\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-rt711-l0-rt1316-l12.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		link_mask: 0x7, /* rt711 on link0 & two rt1318s on link1 and link2 */
		links: rpl_sdw_rt711_link0_rt1318_link12.as_ptr(),
		drv_name: b"sof_sdw\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-rt711-l0-rt1318-l12.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		link_mask: 0x7, /* rt714 on link0 & two rt1316s on link1 and link2 */
		links: rpl_sdw_rt1316_link12_rt714_link0.as_ptr(),
		drv_name: b"sof_sdw\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-rt1316-l12-rt714-l0.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		link_mask: 0x7, /* rt711 on link2 & two rt1316s on link0 and link1 */
		links: rpl_sdw_rt711_link2_rt1316_link01.as_ptr(),
		drv_name: b"sof_sdw\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-rt711-l2-rt1316-l01.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		link_mask: 0x1, /* link0 required */
		links: rpl_rvp.as_ptr(),
		drv_name: b"sof_sdw\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-rt711-l0.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		link_mask: 0x1, /* link0 required */
		links: rpl_sdca_rvp.as_ptr(),
		drv_name: b"sof_sdw\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-rt711-l0.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {
		link_mask: 0x4, /* link2 required */
		links: rplp_crb.as_ptr(),
		drv_name: b"sof_sdw\0".as_ptr() as *const _,
		sof_tplg_filename: b"sof-rpl-rt711-l2.tplg\0".as_ptr() as *const _,
	},
	snd_soc_acpi_mach {},
];
/* EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_rpl_sdw_machines); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
