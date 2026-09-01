// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-lnl-match.c - tables and support for LNL ACPI enumeration.
 *
 * Copyright (c) 2023, Intel Corporation
 *
 */

// C dependencies:
// <sound/soc-acpi.h>
// <sound/soc-acpi-intel-match.h>
// "sof-function-topology-lib.h"
// "soc-acpi-intel-sdca-quirks.h"
// "soc-acpi-intel-sdw-mockup-match.h"

#[repr(C)]
pub struct snd_soc_acpi_endpoint {
    pub num: u32,
    pub aggregated: u32,
    pub group_position: u32,
    pub group_id: u32,
}

#[repr(C)]
pub struct snd_soc_acpi_adr_device {
    pub adr: u64,
    pub num_endpoints: usize,
    pub endpoints: *const snd_soc_acpi_endpoint,
    pub name_prefix: *const core::ffi::c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_link_adr {
    pub mask: u32,
    pub num_adr: usize,
    pub adr_d: *const snd_soc_acpi_adr_device,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub link_mask: u32,
    pub links: *const snd_soc_acpi_link_adr,
    pub drv_name: *const core::ffi::c_char,
    pub sof_tplg_filename: *const core::ffi::c_char,
    pub machine_check: Option<unsafe extern "C" fn() -> bool>,
    pub get_function_tplg_files: Option<unsafe extern "C" fn()>,
}

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

unsafe extern "C" {
    static sdw_mockup_headset_2amps_mic: [snd_soc_acpi_link_adr; 0];
    static sdw_mockup_headset_1amp_mic: [snd_soc_acpi_link_adr; 0];
    static sdw_mockup_mic_headset_1amp: [snd_soc_acpi_link_adr; 0];
    static sdw_mockup_multi_func: [snd_soc_acpi_link_adr; 0];

    fn sof_sdw_get_tplg_files();
    fn snd_soc_acpi_intel_sdca_is_device_rt712_vb() -> bool;
}

pub static mut snd_soc_acpi_intel_lnl_machines: [snd_soc_acpi_mach; 1] = [
    unsafe { core::mem::zeroed() },
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_lnl_machines);

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

static spk_1_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint {
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

static spk_4_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint {
    num: 0,
    aggregated: 1,
    group_position: 4,
    group_id: 1,
};

static spk_5_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint {
    num: 0,
    aggregated: 1,
    group_position: 5,
    group_id: 1,
};

static spk_6_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint {
    num: 0,
    aggregated: 1,
    group_position: 6,
    group_id: 1,
};

static rt712_endpoints: [snd_soc_acpi_endpoint; 2] = [
    snd_soc_acpi_endpoint {
        num: 0,
        aggregated: 0,
        group_position: 0,
        group_id: 0,
    },
    snd_soc_acpi_endpoint {
        num: 1,
        aggregated: 0,
        group_position: 0,
        group_id: 0,
    },
];

/*
 * RT722 is a multi-function codec, three endpoints are created for
 * its headset, amp and dmic functions.
 */
static rt722_endpoints: [snd_soc_acpi_endpoint; 3] = [
    snd_soc_acpi_endpoint {
        num: 0,
        aggregated: 0,
        group_position: 0,
        group_id: 0,
    },
    snd_soc_acpi_endpoint {
        num: 1,
        aggregated: 0,
        group_position: 0,
        group_id: 0,
    },
    snd_soc_acpi_endpoint {
        num: 2,
        aggregated: 0,
        group_position: 0,
        group_id: 0,
    },
];

static jack_dmic_endpoints: [snd_soc_acpi_endpoint; 2] = [
    /* Jack Endpoint */
    snd_soc_acpi_endpoint {
        num: 0,
        aggregated: 0,
        group_position: 0,
        group_id: 0,
    },
    /* DMIC Endpoint */
    snd_soc_acpi_endpoint {
        num: 1,
        aggregated: 0,
        group_position: 0,
        group_id: 0,
    },
];

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

static cs42l43_amp_spkagg_endpoints: [snd_soc_acpi_endpoint; 4] = [
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
        aggregated: 1,
        group_position: 0,
        group_id: 1,
    },
];

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const core::ffi::c_char
    };
}

static cs35l56_2_l_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x00023001FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: cstr!("AMP1"),
    },
    snd_soc_acpi_adr_device {
        adr: 0x00023101FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_2_endpoint,
        name_prefix: cstr!("AMP2"),
    },
];

static cs35l56_3_r_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x00033201fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: cstr!("AMP3"),
    },
    snd_soc_acpi_adr_device {
        adr: 0x00033301fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_3_endpoint,
        name_prefix: cstr!("AMP4"),
    },
];

static cs35l56_3_lr_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x00033001fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: cstr!("AMP1"),
    },
    snd_soc_acpi_adr_device {
        adr: 0x00033101fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: cstr!("AMP2"),
    },
];

static cs35l56_1_3amp_adr: [snd_soc_acpi_adr_device; 3] = [
    snd_soc_acpi_adr_device {
        adr: 0x00013001fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_1_endpoint,
        name_prefix: cstr!("AMP1"),
    },
    snd_soc_acpi_adr_device {
        adr: 0x00013101fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_2_endpoint,
        name_prefix: cstr!("AMP2"),
    },
    snd_soc_acpi_adr_device {
        adr: 0x00013201fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_3_endpoint,
        name_prefix: cstr!("AMP3"),
    },
];

static cs35l56_3_3amp_adr: [snd_soc_acpi_adr_device; 3] = [
    snd_soc_acpi_adr_device {
        adr: 0x00033301fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_4_endpoint,
        name_prefix: cstr!("AMP4"),
    },
    snd_soc_acpi_adr_device {
        adr: 0x00033401fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_5_endpoint,
        name_prefix: cstr!("AMP5"),
    },
    snd_soc_acpi_adr_device {
        adr: 0x00033501fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_6_endpoint,
        name_prefix: cstr!("AMP6"),
    },
];

static cs42l43_0_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x00003001FA424301u64,
        num_endpoints: cs42l43_endpoints.len(),
        endpoints: cs42l43_endpoints.as_ptr(),
        name_prefix: cstr!("cs42l43"),
    },
];

static cs42l43_2_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x00023001fa424301u64,
        num_endpoints: cs42l43_amp_spkagg_endpoints.len(),
        endpoints: cs42l43_amp_spkagg_endpoints.as_ptr(),
        name_prefix: cstr!("cs42l43"),
    },
];

static rt711_sdca_0_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000030025D071101u64,
        num_endpoints: 1,
        endpoints: &single_endpoint,
        name_prefix: cstr!("rt711"),
    },
];

static rt712_2_single_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000230025D071201u64,
        num_endpoints: rt712_endpoints.len(),
        endpoints: rt712_endpoints.as_ptr(),
        name_prefix: cstr!("rt712"),
    },
];

static rt1712_3_single_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000330025D171201u64,
        num_endpoints: 1,
        endpoints: &single_endpoint,
        name_prefix: cstr!("rt712-dmic"),
    },
];

static rt712_vb_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000230025D071201u64,
        num_endpoints: jack_amp_g1_dmic_endpoints.len(),
        endpoints: jack_amp_g1_dmic_endpoints.as_ptr(),
        name_prefix: cstr!("rt712"),
    },
];

static rt722_0_single_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000030025d072201u64,
        num_endpoints: rt722_endpoints.len(),
        endpoints: rt722_endpoints.as_ptr(),
        name_prefix: cstr!("rt722"),
    },
];

static rt1316_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000230025D131601u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: cstr!("rt1316-1"),
    },
];

static rt1316_3_group1_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000331025D131601u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: cstr!("rt1316-2"),
    },
];

static rt1318_1_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000133025D131801u64,
        num_endpoints: 1,
        endpoints: &single_endpoint,
        name_prefix: cstr!("rt1318-1"),
    },
];

static rt1318_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000130025D131801u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: cstr!("rt1318-1"),
    },
];

static rt1318_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000232025D131801u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: cstr!("rt1318-2"),
    },
];

static rt1320_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000130025D132001u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: cstr!("rt1320-1"),
    },
];

static rt1320_2_group2_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000231025D132001u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: cstr!("rt1320-2"),
    },
];

static rt1320_1_group2_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000130025D132001u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: cstr!("rt1320-1"),
    },
];

static rt1320_3_group2_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000330025D132001u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: cstr!("rt1320-2"),
    },
];

static rt713_0_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000031025D071301u64,
        num_endpoints: 1,
        endpoints: &single_endpoint,
        name_prefix: cstr!("rt713"),
    },
];

static rt713_vb_2_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000230025d071301u64,
        num_endpoints: jack_dmic_endpoints.len(),
        endpoints: jack_dmic_endpoints.as_ptr(),
        name_prefix: cstr!("rt713"),
    },
];

static rt714_0_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000030025D071401u64,
        num_endpoints: 1,
        endpoints: &single_endpoint,
        name_prefix: cstr!("rt714"),
    },
];

static rt714_1_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000130025D071401u64,
        num_endpoints: 1,
        endpoints: &single_endpoint,
        name_prefix: cstr!("rt714"),
    },
];

const LINK_ADR_ZERO: snd_soc_acpi_link_adr = snd_soc_acpi_link_adr {
    mask: 0,
    num_adr: 0,
    adr_d: core::ptr::null(),
};

static lnl_cs42l43_l0: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: cs42l43_0_adr.len(),
        adr_d: cs42l43_0_adr.as_ptr(),
    },
    LINK_ADR_ZERO,
];

static lnl_cs42l43_l0_cs35l56_l3: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: cs42l43_0_adr.len(),
        adr_d: cs42l43_0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(3),
        num_adr: cs35l56_3_lr_adr.len(),
        adr_d: cs35l56_3_lr_adr.as_ptr(),
    },
    LINK_ADR_ZERO,
];

static lnl_cs42l43_l0_cs35l56_l23: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: cs42l43_0_adr.len(),
        adr_d: cs42l43_0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(2),
        num_adr: cs35l56_2_l_adr.len(),
        adr_d: cs35l56_2_l_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(3),
        num_adr: cs35l56_3_r_adr.len(),
        adr_d: cs35l56_3_r_adr.as_ptr(),
    },
    LINK_ADR_ZERO,
];

static lnl_cs42l43_l2_cs35l56x6_l13: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr {
        mask: BIT(2),
        num_adr: cs42l43_2_adr.len(),
        adr_d: cs42l43_2_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: cs35l56_1_3amp_adr.len(),
        adr_d: cs35l56_1_3amp_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(3),
        num_adr: cs35l56_3_3amp_adr.len(),
        adr_d: cs35l56_3_3amp_adr.as_ptr(),
    },
    LINK_ADR_ZERO,
];

static lnl_rvp: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt711_sdca_0_adr.len(),
        adr_d: rt711_sdca_0_adr.as_ptr(),
    },
    LINK_ADR_ZERO,
];

static lnl_712_only: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr {
        mask: BIT(2),
        num_adr: rt712_2_single_adr.len(),
        adr_d: rt712_2_single_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(3),
        num_adr: rt1712_3_single_adr.len(),
        adr_d: rt1712_3_single_adr.as_ptr(),
    },
    LINK_ADR_ZERO,
];

static lnl_rt722_only: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt722_0_single_adr.len(),
        adr_d: rt722_0_single_adr.as_ptr(),
    },
    LINK_ADR_ZERO,
];

static lnl_3_in_1_sdca: [snd_soc_acpi_link_adr; 5] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt711_sdca_0_adr.len(),
        adr_d: rt711_sdca_0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(2),
        num_adr: rt1316_2_group1_adr.len(),
        adr_d: rt1316_2_group1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(3),
        num_adr: rt1316_3_group1_adr.len(),
        adr_d: rt1316_3_group1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: rt714_1_adr.len(),
        adr_d: rt714_1_adr.as_ptr(),
    },
    LINK_ADR_ZERO,
];

static lnl_sdw_rt1318_l12_rt714_l0: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: rt1318_1_group1_adr.len(),
        adr_d: rt1318_1_group1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(2),
        num_adr: rt1318_2_group1_adr.len(),
        adr_d: rt1318_2_group1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt714_0_adr.len(),
        adr_d: rt714_0_adr.as_ptr(),
    },
    LINK_ADR_ZERO,
];

static lnl_sdw_rt1320_l12_rt714_l0: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: rt1320_1_group2_adr.len(),
        adr_d: rt1320_1_group2_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(2),
        num_adr: rt1320_2_group2_adr.len(),
        adr_d: rt1320_2_group2_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt714_0_adr.len(),
        adr_d: rt714_0_adr.as_ptr(),
    },
    LINK_ADR_ZERO,
];

static lnl_sdw_rt713_l0_rt1318_l1: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt713_0_adr.len(),
        adr_d: rt713_0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: rt1318_1_adr.len(),
        adr_d: rt1318_1_adr.as_ptr(),
    },
    LINK_ADR_ZERO,
];

static lnl_sdw_rt713_vb_l2_rt1320_l13: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr {
        mask: BIT(2),
        num_adr: rt713_vb_2_adr.len(),
        adr_d: rt713_vb_2_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: rt1320_1_group2_adr.len(),
        adr_d: rt1320_1_group2_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(3),
        num_adr: rt1320_3_group2_adr.len(),
        adr_d: rt1320_3_group2_adr.as_ptr(),
    },
    LINK_ADR_ZERO,
];

static lnl_sdw_rt712_vb_l2_rt1320_l1: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr {
        mask: BIT(2),
        num_adr: rt712_vb_2_group1_adr.len(),
        adr_d: rt712_vb_2_group1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: rt1320_1_group1_adr.len(),
        adr_d: rt1320_1_group1_adr.as_ptr(),
    },
    LINK_ADR_ZERO,
];

const MACH_ZERO: snd_soc_acpi_mach = snd_soc_acpi_mach {
    link_mask: 0,
    links: core::ptr::null(),
    drv_name: core::ptr::null(),
    sof_tplg_filename: core::ptr::null(),
    machine_check: None,
    get_function_tplg_files: None,
};

/* this table is used when there is no I2S codec present */
/* this table is used when there is no I2S codec present */
pub static mut snd_soc_acpi_intel_lnl_sdw_machines: [snd_soc_acpi_mach; 18] = [
    /* mockup tests need to be first */
    snd_soc_acpi_mach {
        link_mask: GENMASK(3, 0),
        links: unsafe { sdw_mockup_headset_2amps_mic.as_ptr() },
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-rt711-rt1308-rt715.tplg"),
        machine_check: None,
        get_function_tplg_files: None,
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(1) | BIT(3),
        links: unsafe { sdw_mockup_headset_1amp_mic.as_ptr() },
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-rt711-rt1308-mono-rt715.tplg"),
        machine_check: None,
        get_function_tplg_files: None,
    },
    snd_soc_acpi_mach {
        link_mask: GENMASK(2, 0),
        links: unsafe { sdw_mockup_mic_headset_1amp.as_ptr() },
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-rt715-rt711-rt1308-mono.tplg"),
        machine_check: None,
        get_function_tplg_files: None,
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0),
        links: unsafe { sdw_mockup_multi_func.as_ptr() },
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-rt722-l0.tplg"), /* Reuse the existing tplg file */
        machine_check: None,
        get_function_tplg_files: None,
    },
    snd_soc_acpi_mach {
        link_mask: GENMASK(3, 0),
        links: lnl_3_in_1_sdca.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-rt711-l0-rt1316-l23-rt714-l1.tplg"),
        machine_check: None,
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(2) | BIT(3),
        links: lnl_cs42l43_l0_cs35l56_l23.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-cs42l43-l0-cs35l56-l23.tplg"),
        machine_check: None,
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(1) | BIT(2) | BIT(3),
        links: lnl_cs42l43_l2_cs35l56x6_l13.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-cs42l43-l2-cs35l56x6-l13.tplg"),
        machine_check: None,
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(3),
        links: lnl_cs42l43_l0_cs35l56_l3.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-cs42l43-l0-cs35l56-l3.tplg"),
        machine_check: None,
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0),
        links: lnl_cs42l43_l0.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-cs42l43-l0.tplg"),
        machine_check: None,
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0),
        links: lnl_rvp.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-rt711.tplg"),
        machine_check: None,
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(2) | BIT(3),
        links: lnl_712_only.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-rt712-l2-rt1712-l3.tplg"),
        machine_check: None,
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0),
        links: lnl_rt722_only.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-rt722-l0.tplg"),
        machine_check: None,
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
    },
    snd_soc_acpi_mach {
        link_mask: GENMASK(2, 0),
        links: lnl_sdw_rt1318_l12_rt714_l0.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-rt1318-l12-rt714-l0.tplg"),
        machine_check: None,
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
    },
    snd_soc_acpi_mach {
        link_mask: GENMASK(2, 0),
        links: lnl_sdw_rt1320_l12_rt714_l0.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-rt1320-l12-rt714-l0.tplg"),
        machine_check: None,
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(1),
        links: lnl_sdw_rt713_l0_rt1318_l1.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        sof_tplg_filename: cstr!("sof-lnl-rt713-l0-rt1318-l1.tplg"),
        machine_check: None,
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(1) | BIT(2),
        links: lnl_sdw_rt712_vb_l2_rt1320_l1.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        machine_check: Some(snd_soc_acpi_intel_sdca_is_device_rt712_vb),
        sof_tplg_filename: cstr!("sof-lnl-rt712-l2-rt1320-l1.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(1) | BIT(2) | BIT(3),
        links: lnl_sdw_rt713_vb_l2_rt1320_l13.as_ptr(),
        drv_name: cstr!("sof_sdw"),
        machine_check: Some(snd_soc_acpi_intel_sdca_is_device_rt712_vb),
        sof_tplg_filename: cstr!("sof-lnl-rt713-l2-rt1320-l13.tplg"),
        get_function_tplg_files: Some(sof_sdw_get_tplg_files),
    },
    MACH_ZERO,
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_lnl_sdw_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
