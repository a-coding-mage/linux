// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-tgl-match.c - tables and support for TGL ACPI enumeration.
 *
 * Copyright (c) 2019, Intel Corporation.
 *
 */

// C dependencies:
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>
// #include <sound/soc-acpi-intel-ssp-common.h>
// #include "soc-acpi-intel-sdw-mockup-match.h"

use core::ffi::c_char;

use crate::{
    snd_soc_acpi_adr_device, snd_soc_acpi_codecs, snd_soc_acpi_endpoint,
    snd_soc_acpi_link_adr, snd_soc_acpi_mach, snd_soc_acpi_codec_list,
    RT5682_ACPI_HID, RT5682S_ACPI_HID, SND_SOC_ACPI_TPLG_INTEL_AMP_NAME,
    SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME, SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
    SND_SOC_ACPI_TPLG_INTEL_SSP_MSB, SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER,
};

unsafe extern "C" {
    static sdw_mockup_headset_2amps_mic: *const snd_soc_acpi_link_adr;
    static sdw_mockup_headset_1amp_mic: *const snd_soc_acpi_link_adr;
    static sdw_mockup_mic_headset_1amp: *const snd_soc_acpi_link_adr;
}

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! mach {
    ($($field:ident : $value:expr),* $(,)?) => {
        snd_soc_acpi_mach {
            $($field: $value,)*
            ..unsafe { core::mem::zeroed() }
        }
    };
}

static essx_83x6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 3,
    codecs: [cstr!("ESSX8316"), cstr!("ESSX8326"), cstr!("ESSX8336")],
};

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

static rt712_endpoints: [snd_soc_acpi_endpoint; 2] = [
    snd_soc_acpi_endpoint { num: 0, aggregated: 0, group_position: 0, group_id: 0 },
    snd_soc_acpi_endpoint { num: 1, aggregated: 0, group_position: 0, group_id: 0 },
];

static rt711_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000020025D071100u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: cstr!("rt711"),
}];

static rt711_1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000120025D071100u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: cstr!("rt711"),
}];

static rt1308_1_dual_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x000120025D130800u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: cstr!("rt1308-1"),
    },
    snd_soc_acpi_adr_device {
        adr: 0x000122025D130800u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: cstr!("rt1308-2"),
    },
];

static rt1308_1_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000120025D130800u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: cstr!("rt1308-1"),
}];

static rt1308_2_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000220025D130800u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: cstr!("rt1308-1"),
}];

static rt1308_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000120025D130800u64,
    num_endpoints: 1,
    endpoints: &spk_l_endpoint,
    name_prefix: cstr!("rt1308-1"),
}];

static rt1308_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000220025D130800u64,
    num_endpoints: 1,
    endpoints: &spk_r_endpoint,
    name_prefix: cstr!("rt1308-2"),
}];

static rt715_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000021025D071500u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: cstr!("rt715"),
}];

static rt715_3_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000320025D071500u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: cstr!("rt715"),
}];

static mx8373_1_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x000123019F837300u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: cstr!("Right"),
    },
    snd_soc_acpi_adr_device {
        adr: 0x000127019F837300u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: cstr!("Left"),
    },
];

static rt5682_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000021025D568200u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: cstr!("rt5682"),
}];

static rt711_sdca_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000030025D071101u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: cstr!("rt711"),
}];

static rt1316_1_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000131025D131601u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: cstr!("rt1316-1"),
}];

static rt712_0_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000030025D071201u64,
    num_endpoints: rt712_endpoints.len() as _,
    endpoints: rt712_endpoints.as_ptr(),
    name_prefix: cstr!("rt712"),
}];

static rt1712_1_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000130025D171201u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: cstr!("rt712-dmic"),
}];

static rt1316_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000131025D131601u64, /* unique ID is set for some reason */
    num_endpoints: 1,
    endpoints: &spk_l_endpoint,
    name_prefix: cstr!("rt1316-1"),
}];

static rt1316_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000230025D131601u64,
    num_endpoints: 1,
    endpoints: &spk_r_endpoint,
    name_prefix: cstr!("rt1316-2"),
}];

static rt714_3_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000330025D071401u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: cstr!("rt714"),
}];

macro_rules! link {
    ($mask:expr, $adr:ident) => {
        snd_soc_acpi_link_adr {
            mask: $mask,
            num_adr: $adr.len() as _,
            adr_d: $adr.as_ptr(),
        }
    };
}

macro_rules! link_zero {
    () => {
        unsafe { core::mem::zeroed::<snd_soc_acpi_link_adr>() }
    };
}

static tgl_rvp: [snd_soc_acpi_link_adr; 3] = [
    link!(BIT(0), rt711_0_adr),
    link!(BIT(1), rt1308_1_dual_adr),
    link_zero!(),
];

static tgl_rvp_headset_only: [snd_soc_acpi_link_adr; 2] = [
    link!(BIT(0), rt711_0_adr),
    link_zero!(),
];

static tgl_hp: [snd_soc_acpi_link_adr; 3] = [
    link!(BIT(0), rt711_0_adr),
    link!(BIT(1), rt1308_1_single_adr),
    link_zero!(),
];

static tgl_chromebook_base: [snd_soc_acpi_link_adr; 3] = [
    link!(BIT(0), rt5682_0_adr),
    link!(BIT(1), mx8373_1_adr),
    link_zero!(),
];

static tgl_3_in_1_default: [snd_soc_acpi_link_adr; 5] = [
    link!(BIT(0), rt711_0_adr),
    link!(BIT(1), rt1308_1_group1_adr),
    link!(BIT(2), rt1308_2_group1_adr),
    link!(BIT(3), rt715_3_adr),
    link_zero!(),
];

static tgl_3_in_1_mono_amp: [snd_soc_acpi_link_adr; 4] = [
    link!(BIT(0), rt711_0_adr),
    link!(BIT(1), rt1308_1_single_adr),
    link!(BIT(3), rt715_3_adr),
    link_zero!(),
];

static tgl_sdw_rt711_link1_rt1308_link2_rt715_link0: [snd_soc_acpi_link_adr; 4] = [
    link!(BIT(1), rt711_1_adr),
    link!(BIT(2), rt1308_2_single_adr),
    link!(BIT(0), rt715_0_adr),
    link_zero!(),
];

static tgl_3_in_1_sdca: [snd_soc_acpi_link_adr; 5] = [
    link!(BIT(0), rt711_sdca_0_adr),
    link!(BIT(1), rt1316_1_group1_adr),
    link!(BIT(2), rt1316_2_group1_adr),
    link!(BIT(3), rt714_3_adr),
    link_zero!(),
];

static tgl_3_in_1_sdca_mono: [snd_soc_acpi_link_adr; 4] = [
    link!(BIT(0), rt711_sdca_0_adr),
    link!(BIT(1), rt1316_1_single_adr),
    link!(BIT(3), rt714_3_adr),
    link_zero!(),
];

static tgl_712_only: [snd_soc_acpi_link_adr; 3] = [
    link!(BIT(0), rt712_0_single_adr),
    link!(BIT(1), rt1712_1_single_adr),
    link_zero!(),
];

static cs42l43_endpoints: [snd_soc_acpi_endpoint; 4] = [
    snd_soc_acpi_endpoint { /* Jack Playback Endpoint */ num: 0, aggregated: 0, group_position: 0, group_id: 0 },
    snd_soc_acpi_endpoint { /* DMIC Capture Endpoint */ num: 1, aggregated: 0, group_position: 0, group_id: 0 },
    snd_soc_acpi_endpoint { /* Jack Capture Endpoint */ num: 2, aggregated: 0, group_position: 0, group_id: 0 },
    snd_soc_acpi_endpoint { /* Speaker Playback Endpoint */ num: 3, aggregated: 0, group_position: 0, group_id: 0 },
];

static cs42l43_3_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00033001FA424301u64,
    num_endpoints: cs42l43_endpoints.len() as _,
    endpoints: cs42l43_endpoints.as_ptr(),
    name_prefix: cstr!("cs42l43"),
}];

static cs35l56_0_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device { adr: 0x00003301FA355601u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: cstr!("AMP1") },
    snd_soc_acpi_adr_device { adr: 0x00003201FA355601u64, num_endpoints: 1, endpoints: &spk_3_endpoint, name_prefix: cstr!("AMP2") },
];

static cs35l56_1_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device { adr: 0x00013701FA355601u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: cstr!("AMP3") },
    snd_soc_acpi_adr_device { adr: 0x00013601FA355601u64, num_endpoints: 1, endpoints: &spk_2_endpoint, name_prefix: cstr!("AMP4") },
];

static tgl_cs42l43_cs35l56: [snd_soc_acpi_link_adr; 4] = [
    link!(BIT(3), cs42l43_3_adr),
    link!(BIT(0), cs35l56_0_adr),
    link!(BIT(1), cs35l56_1_adr),
    link_zero!(),
];

static tgl_rt5682_rt5682s_hp: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 2,
    codecs: [RT5682_ACPI_HID, RT5682S_ACPI_HID],
};

static tgl_lt6911_hdmi: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [cstr!("INTC10B0")],
};

#[no_mangle]
pub static mut snd_soc_acpi_intel_tgl_machines: [snd_soc_acpi_mach; 4] = [
    mach! {
        comp_ids: &essx_83x6,
        drv_name: cstr!("sof-essx8336"),
        sof_tplg_filename: cstr!("sof-tgl-es8336"), /* the tplg suffix is added at run time */
        tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER |
                         SND_SOC_ACPI_TPLG_INTEL_SSP_MSB |
                         SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
    },
    /*
     * place boards for each headphone codec: sof driver will complete the
     * tplg name and machine driver will detect the amp type
     */
    mach! {
        comp_ids: &tgl_rt5682_rt5682s_hp,
        drv_name: cstr!("tgl_rt5682_def"),
        sof_tplg_filename: cstr!("sof-tgl"), /* the tplg suffix is added at run time */
        tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME |
                         SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME,
    },
    /* place amp-only boards in the end of table */
    mach! {
        id: cstr!("10EC1308"),
        drv_name: cstr!("tgl_rt1308_hdmi_ssp"),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &tgl_lt6911_hdmi as *const _ as *const _,
        sof_tplg_filename: cstr!("sof-tgl-rt1308-ssp2-hdmi-ssp15.tplg"),
    },
    unsafe { core::mem::zeroed() },
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_tgl_machines);

static cs35l56_l_fb_endpoints: [snd_soc_acpi_endpoint; 2] = [
    snd_soc_acpi_endpoint { /* Speaker Playback Endpoint */ num: 0, aggregated: 1, group_position: 0, group_id: 1 },
    snd_soc_acpi_endpoint { /* Feedback Capture Endpoint */ num: 1, aggregated: 1, group_position: 0, group_id: 2 },
];

static cs35l56_r_fb_endpoints: [snd_soc_acpi_endpoint; 2] = [
    snd_soc_acpi_endpoint { /* Speaker Playback Endpoint */ num: 0, aggregated: 1, group_position: 1, group_id: 1 },
    snd_soc_acpi_endpoint { /* Feedback Capture Endpoint */ num: 1, aggregated: 1, group_position: 1, group_id: 2 },
];

static cs35l56_2_fb_endpoints: [snd_soc_acpi_endpoint; 2] = [
    snd_soc_acpi_endpoint { /* Speaker Playback Endpoint */ num: 0, aggregated: 1, group_position: 2, group_id: 1 },
    snd_soc_acpi_endpoint { /* Feedback Capture Endpoint */ num: 1, aggregated: 1, group_position: 2, group_id: 2 },
];

static cs35l56_3_fb_endpoints: [snd_soc_acpi_endpoint; 2] = [
    snd_soc_acpi_endpoint { /* Speaker Playback Endpoint */ num: 0, aggregated: 1, group_position: 3, group_id: 1 },
    snd_soc_acpi_endpoint { /* Feedback Capture Endpoint */ num: 1, aggregated: 1, group_position: 3, group_id: 2 },
];

static cs35l56_4_fb_endpoints: [snd_soc_acpi_endpoint; 2] = [
    snd_soc_acpi_endpoint { /* Speaker Playback Endpoint */ num: 0, aggregated: 1, group_position: 4, group_id: 1 },
    snd_soc_acpi_endpoint { /* Feedback Capture Endpoint */ num: 1, aggregated: 1, group_position: 4, group_id: 2 },
];

static cs35l56_5_fb_endpoints: [snd_soc_acpi_endpoint; 2] = [
    snd_soc_acpi_endpoint { /* Speaker Playback Endpoint */ num: 0, aggregated: 1, group_position: 5, group_id: 1 },
    snd_soc_acpi_endpoint { /* Feedback Capture Endpoint */ num: 1, aggregated: 1, group_position: 5, group_id: 2 },
];

static cs35l56_6_fb_endpoints: [snd_soc_acpi_endpoint; 2] = [
    snd_soc_acpi_endpoint { /* Speaker Playback Endpoint */ num: 0, aggregated: 1, group_position: 6, group_id: 1 },
    snd_soc_acpi_endpoint { /* Feedback Capture Endpoint */ num: 1, aggregated: 1, group_position: 6, group_id: 2 },
];

static cs35l56_7_fb_endpoints: [snd_soc_acpi_endpoint; 2] = [
    snd_soc_acpi_endpoint { /* Speaker Playback Endpoint */ num: 0, aggregated: 1, group_position: 7, group_id: 1 },
    snd_soc_acpi_endpoint { /* Feedback Capture Endpoint */ num: 1, aggregated: 1, group_position: 7, group_id: 2 },
];

static cs35l56_sdw_eight_1_4_fb_adr: [snd_soc_acpi_adr_device; 4] = [
    snd_soc_acpi_adr_device { adr: 0x00003301fa355601u64, num_endpoints: cs35l56_l_fb_endpoints.len() as _, endpoints: cs35l56_l_fb_endpoints.as_ptr(), name_prefix: cstr!("AMP1") },
    snd_soc_acpi_adr_device { adr: 0x00003201fa355601u64, num_endpoints: cs35l56_2_fb_endpoints.len() as _, endpoints: cs35l56_2_fb_endpoints.as_ptr(), name_prefix: cstr!("AMP2") },
    snd_soc_acpi_adr_device { adr: 0x00003101fa355601u64, num_endpoints: cs35l56_4_fb_endpoints.len() as _, endpoints: cs35l56_4_fb_endpoints.as_ptr(), name_prefix: cstr!("AMP3") },
    snd_soc_acpi_adr_device { adr: 0x00003001fa355601u64, num_endpoints: cs35l56_6_fb_endpoints.len() as _, endpoints: cs35l56_6_fb_endpoints.as_ptr(), name_prefix: cstr!("AMP4") },
];

static cs35l56_sdw_eight_5_8_fb_adr: [snd_soc_acpi_adr_device; 4] = [
    snd_soc_acpi_adr_device { adr: 0x00013701fa355601u64, num_endpoints: cs35l56_r_fb_endpoints.len() as _, endpoints: cs35l56_r_fb_endpoints.as_ptr(), name_prefix: cstr!("AMP8") },
    snd_soc_acpi_adr_device { adr: 0x00013601fa355601u64, num_endpoints: cs35l56_3_fb_endpoints.len() as _, endpoints: cs35l56_3_fb_endpoints.as_ptr(), name_prefix: cstr!("AMP7") },
    snd_soc_acpi_adr_device { adr: 0x00013501fa355601u64, num_endpoints: cs35l56_5_fb_endpoints.len() as _, endpoints: cs35l56_5_fb_endpoints.as_ptr(), name_prefix: cstr!("AMP6") },
    snd_soc_acpi_adr_device { adr: 0x00013401fa355601u64, num_endpoints: cs35l56_7_fb_endpoints.len() as _, endpoints: cs35l56_7_fb_endpoints.as_ptr(), name_prefix: cstr!("AMP5") },
];

static up_extreme_cs35l56_sdw_eight: [snd_soc_acpi_link_adr; 3] = [
    link!(BIT(1), cs35l56_sdw_eight_5_8_fb_adr),
    link!(BIT(0), cs35l56_sdw_eight_1_4_fb_adr),
    link_zero!(),
];

/* this table is used when there is no I2S codec present */
#[no_mangle]
pub static mut snd_soc_acpi_intel_tgl_sdw_machines: [snd_soc_acpi_mach; 16] = [
    /* mockup tests need to be first */
    mach! {
        link_mask: GENMASK(3, 0),
        links: unsafe { sdw_mockup_headset_2amps_mic },
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-rt711-rt1308-rt715.tplg"),
    },
    mach! {
        link_mask: BIT(0) | BIT(1) | BIT(3),
        links: unsafe { sdw_mockup_headset_1amp_mic },
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-rt711-rt1308-mono-rt715.tplg"),
    },
    mach! {
        link_mask: BIT(0) | BIT(1) | BIT(2),
        links: unsafe { sdw_mockup_mic_headset_1amp },
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-rt715-rt711-rt1308-mono.tplg"),
    },
    mach! {
        link_mask: 0xF, /* 4 active links required */
        links: tgl_712_only.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-rt712.tplg"),
    },
    mach! {
        link_mask: 0x7,
        links: tgl_sdw_rt711_link1_rt1308_link2_rt715_link0.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-rt715-rt711-rt1308-mono.tplg"),
    },
    mach! {
        link_mask: 0xB,
        links: tgl_cs42l43_cs35l56.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-cs42l43-l3-cs35l56-l01.tplg"),
    },
    mach! {
        link_mask: 0xF, /* 4 active links required */
        links: tgl_3_in_1_default.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-rt711-rt1308-rt715.tplg"),
    },
    mach! {
        /*
         * link_mask should be 0xB, but all links are enabled by BIOS.
         * This entry will be selected if there is no rt1308 exposed
         * on link2 since it will fail to match the above entry.
         */
        link_mask: 0xF,
        links: tgl_3_in_1_mono_amp.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-rt711-rt1308-mono-rt715.tplg"),
    },
    mach! {
        link_mask: 0xF, /* 4 active links required */
        links: tgl_3_in_1_sdca.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-rt711-rt1316-rt714.tplg"),
    },
    mach! {
        /*
         * link_mask should be 0xB, but all links are enabled by BIOS.
         * This entry will be selected if there is no rt1316 amplifier exposed
         * on link2 since it will fail to match the above entry.
         */
        link_mask: 0xF, /* 4 active links required */
        links: tgl_3_in_1_sdca_mono.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-rt711-l0-rt1316-l1-mono-rt714-l3.tplg"),
    },
    mach! {
        link_mask: 0x3, /* rt711 on link 0 and 1 rt1308 on link 1 */
        links: tgl_hp.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-rt711-rt1308.tplg"),
    },
    mach! {
        link_mask: 0x3, /* rt711 on link 0 and 2 rt1308s on link 1 */
        links: tgl_rvp.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-rt711-rt1308.tplg"),
    },
    mach! {
        link_mask: 0x3, /* rt5682 on link0 & 2xmax98373 on link 1 */
        links: tgl_chromebook_base.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-sdw-max98373-rt5682.tplg"),
    },
    mach! {
        link_mask: 0x1, /* rt711 on link 0 */
        links: tgl_rvp_headset_only.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-rt711.tplg"),
    },
    mach! {
        link_mask: BIT(0) | BIT(1),
        links: up_extreme_cs35l56_sdw_eight.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-tgl-cs35l56-l01-fb8.tplg"),
    },
    unsafe { core::mem::zeroed() },
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_tgl_sdw_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
