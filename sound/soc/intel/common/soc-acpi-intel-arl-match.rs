// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-apci-intel-arl-match.c - tables and support for ARL ACPI enumeration.
 *
 * Copyright (c) 2023 Intel Corporation.
 */

/* Dependencies from:
 * <sound/soc-acpi.h>
 * <sound/soc-acpi-intel-match.h>
 * <sound/soc-acpi-intel-ssp-common.h>
 * "soc-acpi-intel-sdca-quirks.h"
 * "sof-function-topology-lib.h"
 */

use core::ffi::c_char;

use crate::{
    snd_soc_acpi_adr_device, snd_soc_acpi_codecs, snd_soc_acpi_endpoint,
    snd_soc_acpi_intel_sdca_is_device_rt712_vb, snd_soc_acpi_link_adr,
    snd_soc_acpi_mach, snd_soc_acpi_codec_list, sof_sdw_get_tplg_files,
    RT5682S_ACPI_HID, RT5682_ACPI_HID, SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
    SND_SOC_ACPI_TPLG_INTEL_SSP_MSB, SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER,
};

const fn BIT(nr: u32) -> u64 {
    1u64 << nr
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
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

static jack_amp_g1_dmic_endpoints: [snd_soc_acpi_endpoint; 3] = [
    /* Jack Endpoint */
    snd_soc_acpi_endpoint {
        num: 0,
        aggregated: 0,
        group_position: 0,
        group_id: 0,
    },
    /* Amp Endpoint, work as spk_l_endpoint */
    snd_soc_acpi_endpoint {
        num: 1,
        aggregated: 1,
        group_position: 0,
        group_id: 1,
    },
    /* DMIC Endpoint */
    snd_soc_acpi_endpoint {
        num: 2,
        aggregated: 0,
        group_position: 0,
        group_id: 0,
    },
];

static cs35l56_2_lr_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x00023001FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint as *const _ as *const snd_soc_acpi_endpoint,
        name_prefix: c_str!("AMP1"),
    },
    snd_soc_acpi_adr_device {
        adr: 0x00023101FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint as *const _ as *const snd_soc_acpi_endpoint,
        name_prefix: c_str!("AMP2"),
    },
];

static cs35l56_3_lr_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x00033001FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint as *const _ as *const snd_soc_acpi_endpoint,
        name_prefix: c_str!("AMP1"),
    },
    snd_soc_acpi_adr_device {
        adr: 0x00033401FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint as *const _ as *const snd_soc_acpi_endpoint,
        name_prefix: c_str!("AMP2"),
    },
];

static cs35l56_2_r_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x00023201FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint as *const _ as *const snd_soc_acpi_endpoint,
        name_prefix: c_str!("AMP3"),
    },
    snd_soc_acpi_adr_device {
        adr: 0x00023301FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_3_endpoint as *const _ as *const snd_soc_acpi_endpoint,
        name_prefix: c_str!("AMP4"),
    },
];

static cs35l56_3_l_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x00033001fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint as *const _ as *const snd_soc_acpi_endpoint,
        name_prefix: c_str!("AMP1"),
    },
    snd_soc_acpi_adr_device {
        adr: 0x00033101fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_2_endpoint as *const _ as *const snd_soc_acpi_endpoint,
        name_prefix: c_str!("AMP2"),
    },
];

static cs35l56_2_r1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00023101FA355601u64,
    num_endpoints: 1,
    endpoints: &spk_r_endpoint as *const _ as *const snd_soc_acpi_endpoint,
    name_prefix: c_str!("AMP2"),
}];

static cs35l56_3_l3_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00033301fa355601u64,
    num_endpoints: 1,
    endpoints: &spk_l_endpoint as *const _ as *const snd_soc_acpi_endpoint,
    name_prefix: c_str!("AMP1"),
}];

static cs35l56_2_r3_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00023301fa355601u64,
    num_endpoints: 1,
    endpoints: &spk_r_endpoint as *const _ as *const snd_soc_acpi_endpoint,
    name_prefix: c_str!("AMP2"),
}];

static cs35l56_3_l1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00033101fa355601u64,
    num_endpoints: 1,
    endpoints: &spk_l_endpoint as *const _ as *const snd_soc_acpi_endpoint,
    name_prefix: c_str!("AMP1"),
}];

static cs42l43_endpoints: [snd_soc_acpi_endpoint; 4] = [
    snd_soc_acpi_endpoint { /* Jack Playback Endpoint */ num: 0, aggregated: 0, group_position: 0, group_id: 0 },
    snd_soc_acpi_endpoint { /* DMIC Capture Endpoint */ num: 1, aggregated: 0, group_position: 0, group_id: 0 },
    snd_soc_acpi_endpoint { /* Jack Capture Endpoint */ num: 2, aggregated: 0, group_position: 0, group_id: 0 },
    snd_soc_acpi_endpoint { /* Speaker Playback Endpoint */ num: 3, aggregated: 0, group_position: 0, group_id: 0 },
];

static es9356_endpoints: [snd_soc_acpi_endpoint; 4] = [
    snd_soc_acpi_endpoint { /* Jack Playback Endpoint */ num: 0, aggregated: 0, group_position: 0, group_id: 0 },
    snd_soc_acpi_endpoint { /* DMIC Capture Endpoint */ num: 1, aggregated: 0, group_position: 0, group_id: 0 },
    snd_soc_acpi_endpoint { /* Jack Capture Endpoint */ num: 2, aggregated: 0, group_position: 0, group_id: 0 },
    snd_soc_acpi_endpoint { /* Speaker Playback Endpoint */ num: 3, aggregated: 0, group_position: 0, group_id: 0 },
];

static es9356_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00013004b3935601u64,
    num_endpoints: es9356_endpoints.len(),
    endpoints: es9356_endpoints.as_ptr(),
    name_prefix: c_str!("es9356"),
}];

static cs42l43_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00003001FA424301u64,
    num_endpoints: cs42l43_endpoints.len(),
    endpoints: cs42l43_endpoints.as_ptr(),
    name_prefix: c_str!("cs42l43"),
}];

static cs42l43_2_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00023001FA424301u64,
    num_endpoints: cs42l43_endpoints.len(),
    endpoints: cs42l43_endpoints.as_ptr(),
    name_prefix: c_str!("cs42l43"),
}];

static rt711_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000020025D071100u64,
    num_endpoints: 1,
    endpoints: &single_endpoint as *const _ as *const snd_soc_acpi_endpoint,
    name_prefix: c_str!("rt711"),
}];

static rt711_sdca_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000030025D071101u64,
    num_endpoints: 1,
    endpoints: &single_endpoint as *const _ as *const snd_soc_acpi_endpoint,
    name_prefix: c_str!("rt711"),
}];

static rt722_0_agg_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000030025D072201u64,
    num_endpoints: jack_amp_g1_dmic_endpoints.len(),
    endpoints: jack_amp_g1_dmic_endpoints.as_ptr(),
    name_prefix: c_str!("rt722"),
}];

static rt712_0_agg_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000030025D071201u64,
    num_endpoints: jack_amp_g1_dmic_endpoints.len(),
    endpoints: jack_amp_g1_dmic_endpoints.as_ptr(),
    name_prefix: c_str!("rt712"),
}];

static rt1316_3_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000330025D131601u64,
    num_endpoints: 1,
    endpoints: &single_endpoint as *const _ as *const snd_soc_acpi_endpoint,
    name_prefix: c_str!("rt1316-1"),
}];

static rt1320_2_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000230025D132001u64,
    num_endpoints: 1,
    endpoints: &single_endpoint as *const _ as *const snd_soc_acpi_endpoint,
    name_prefix: c_str!("rt1320-1"),
}];

static arl_n_mrd_es9356_link1: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: es9356_adr.len(), adr_d: es9356_adr.as_ptr() },
    unsafe { core::mem::zeroed() },
];

static rt1320_3_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000330025D132001u64,
    num_endpoints: 1,
    endpoints: &spk_r_endpoint as *const _ as *const snd_soc_acpi_endpoint,
    name_prefix: c_str!("rt1320-1"),
}];

static arl_cs42l43_l0: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs42l43_0_adr.len(), adr_d: cs42l43_0_adr.as_ptr() },
    unsafe { core::mem::zeroed() },
];

static arl_cs42l43_l2: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: cs42l43_2_adr.len(), adr_d: cs42l43_2_adr.as_ptr() },
    unsafe { core::mem::zeroed() },
];

static arl_cs42l43_l2_cs35l56_l3: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: cs42l43_2_adr.len(), adr_d: cs42l43_2_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(3), num_adr: cs35l56_3_lr_adr.len(), adr_d: cs35l56_3_lr_adr.as_ptr() },
    unsafe { core::mem::zeroed() },
];

static arl_cs42l43_l0_cs35l56_l2: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs42l43_0_adr.len(), adr_d: cs42l43_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: cs35l56_2_lr_adr.len(), adr_d: cs35l56_2_lr_adr.as_ptr() },
    unsafe { core::mem::zeroed() },
];

static arl_cs42l43_l0_cs35l56_l23: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs42l43_0_adr.len(), adr_d: cs42l43_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: cs35l56_2_r_adr.len(), adr_d: cs35l56_2_r_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(3), num_adr: cs35l56_3_l_adr.len(), adr_d: cs35l56_3_l_adr.as_ptr() },
    unsafe { core::mem::zeroed() },
];

static arl_cs42l43_l0_cs35l56_2_l23: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs42l43_0_adr.len(), adr_d: cs42l43_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: cs35l56_2_r1_adr.len(), adr_d: cs35l56_2_r1_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(3), num_adr: cs35l56_3_l3_adr.len(), adr_d: cs35l56_3_l3_adr.as_ptr() },
    unsafe { core::mem::zeroed() },
];

static arl_cs42l43_l0_cs35l56_3_l23: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs42l43_0_adr.len(), adr_d: cs42l43_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: cs35l56_2_r3_adr.len(), adr_d: cs35l56_2_r3_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(3), num_adr: cs35l56_3_l1_adr.len(), adr_d: cs35l56_3_l1_adr.as_ptr() },
    unsafe { core::mem::zeroed() },
];

static arl_rvp: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_0_adr.len(), adr_d: rt711_0_adr.as_ptr() },
    unsafe { core::mem::zeroed() },
];

static arl_sdca_rvp: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len(), adr_d: rt711_sdca_0_adr.as_ptr() },
    unsafe { core::mem::zeroed() },
];

static arl_rt711_l0_rt1316_l3: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len(), adr_d: rt711_sdca_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt1316_3_single_adr.len(), adr_d: rt1316_3_single_adr.as_ptr() },
    unsafe { core::mem::zeroed() },
];

static arl_rt722_l0_rt1320_l2: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt722_0_agg_adr.len(), adr_d: rt722_0_agg_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1320_2_single_adr.len(), adr_d: rt1320_2_single_adr.as_ptr() },
    unsafe { core::mem::zeroed() },
];

static arl_rt712_l0_rt1320_l3: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt712_0_agg_adr.len(), adr_d: rt712_0_agg_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt1320_3_group1_adr.len(), adr_d: rt1320_3_group1_adr.as_ptr() },
    unsafe { core::mem::zeroed() },
];

static arl_essx_83x6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 3,
    codecs: [c_str!("ESSX8316"), c_str!("ESSX8326"), c_str!("ESSX8336")],
};

static arl_rt5682_hp: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 2,
    codecs: [RT5682_ACPI_HID, RT5682S_ACPI_HID],
};

static arl_lt6911_hdmi: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [c_str!("INTC10B0")],
};

#[no_mangle]
pub static mut snd_soc_acpi_intel_arl_machines: [snd_soc_acpi_mach; 5] = [
    snd_soc_acpi_mach {
        comp_ids: &arl_essx_83x6 as *const _,
        drv_name: c_str!("arl_es83x6_c1_h02"),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &arl_lt6911_hdmi as *const _ as *const _,
        sof_tplg_filename: c_str!("sof-arl-es83x6-ssp1-hdmi-ssp02.tplg"),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        comp_ids: &arl_essx_83x6 as *const _,
        drv_name: c_str!("sof-essx8336"),
        sof_tplg_filename: c_str!("sof-arl-es8336"), /* the tplg suffix is added at run time */
        tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER |
            SND_SOC_ACPI_TPLG_INTEL_SSP_MSB |
            SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        comp_ids: &arl_rt5682_hp as *const _,
        drv_name: c_str!("arl_rt5682_c1_h02"),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &arl_lt6911_hdmi as *const _ as *const _,
        sof_tplg_filename: c_str!("sof-arl-rt5682-ssp1-hdmi-ssp02.tplg"),
        ..unsafe { core::mem::zeroed() }
    },
    /* place amp-only boards in the end of table */
    snd_soc_acpi_mach {
        id: c_str!("INTC10B0"),
        drv_name: c_str!("arl_lt6911_hdmi_ssp"),
        sof_tplg_filename: c_str!("sof-arl-hdmi-ssp02.tplg"),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];

#[no_mangle]
pub static mut snd_soc_acpi_intel_arl_sdw_machines: [snd_soc_acpi_mach; 14] = [
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(2) | BIT(3),
        links: arl_cs42l43_l0_cs35l56_l23.as_ptr(),
        drv_name: c_str!("sof_sdw"),
        sof_tplg_filename: c_str!("sof-arl-cs42l43-l0-cs35l56-l23.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(2) | BIT(3),
        links: arl_cs42l43_l0_cs35l56_2_l23.as_ptr(),
        drv_name: c_str!("sof_sdw"),
        sof_tplg_filename: c_str!("sof-arl-cs42l43-l0-cs35l56-l23.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(2) | BIT(3),
        links: arl_cs42l43_l0_cs35l56_3_l23.as_ptr(),
        drv_name: c_str!("sof_sdw"),
        sof_tplg_filename: c_str!("sof-arl-cs42l43-l0-cs35l56-l23.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(2),
        links: arl_cs42l43_l0_cs35l56_l2.as_ptr(),
        drv_name: c_str!("sof_sdw"),
        sof_tplg_filename: c_str!("sof-arl-cs42l43-l0-cs35l56-l2.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(2),
        links: arl_rt722_l0_rt1320_l2.as_ptr(),
        drv_name: c_str!("sof_sdw"),
        sof_tplg_filename: c_str!("sof-arl-rt722-l0_rt1320-l2.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(3),
        links: arl_rt711_l0_rt1316_l3.as_ptr(),
        drv_name: c_str!("sof_sdw"),
        sof_tplg_filename: c_str!("sof-arl-rt711-l0-rt1316-l3.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(3),
        links: arl_rt712_l0_rt1320_l3.as_ptr(),
        drv_name: c_str!("sof_sdw"),
        machine_check: Some(snd_soc_acpi_intel_sdca_is_device_rt712_vb),
        sof_tplg_filename: c_str!("sof-arl-rt712-l0-rt1320-l3.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: BIT(2) | BIT(3),
        links: arl_cs42l43_l2_cs35l56_l3.as_ptr(),
        drv_name: c_str!("sof_sdw"),
        sof_tplg_filename: c_str!("sof-arl-cs42l43-l2-cs35l56-l3.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0),
        links: arl_cs42l43_l0.as_ptr(),
        drv_name: c_str!("sof_sdw"),
        sof_tplg_filename: c_str!("sof-arl-cs42l43-l0.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: 0x1, /* link0 required */
        links: arl_rvp.as_ptr(),
        drv_name: c_str!("sof_sdw"),
        sof_tplg_filename: c_str!("sof-arl-rt711.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: 0x1, /* link0 required */
        links: arl_sdca_rvp.as_ptr(),
        drv_name: c_str!("sof_sdw"),
        sof_tplg_filename: c_str!("sof-arl-rt711-l0.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: BIT(2),
        links: arl_cs42l43_l2.as_ptr(),
        drv_name: c_str!("sof_sdw"),
        sof_tplg_filename: c_str!("sof-arl-cs42l43-l2.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: BIT(1),
        links: arl_n_mrd_es9356_link1.as_ptr(),
        drv_name: c_str!("sof_sdw"),
        sof_tplg_filename: c_str!("sof-arl-es9356.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
