// SPDX-License-Identifier: GPL-2.0-only
/*
 * amd-acp70-acpi-match.c - tables and support for ACP 7.0 & ACP7.1
 * ACPI enumeration.
 *
 * Copyright 2025 Advanced Micro Devices, Inc.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_uint};

type u64_ = u64;

const fn BIT(n: c_uint) -> u32 {
    1u32 << n
}

#[repr(C)]
pub struct snd_soc_acpi_endpoint {
    pub num: u32,
    pub aggregated: u32,
    pub group_position: u32,
    pub group_id: u32,
}

#[repr(C)]
pub struct snd_soc_acpi_adr_device {
    pub adr: u64_,
    pub num_endpoints: u32,
    pub endpoints: *const snd_soc_acpi_endpoint,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_link_adr {
    pub mask: u32,
    pub num_adr: u32,
    pub adr_d: *const snd_soc_acpi_adr_device,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub link_mask: u32,
    pub links: *const snd_soc_acpi_link_adr,
    pub machine_check: Option<unsafe extern "C" fn(*mut snd_soc_acpi_mach) -> bool>,
    pub drv_name: *const c_char,
    pub sof_tplg_filename: *const c_char,
    pub fw_filename: *const c_char,
}

unsafe impl Sync for snd_soc_acpi_adr_device {}
unsafe impl Sync for snd_soc_acpi_link_adr {}
unsafe impl Sync for snd_soc_acpi_mach {}

unsafe extern "C" {
    fn snd_soc_acpi_amd_sdca_is_device_rt712_vb(mach: *mut snd_soc_acpi_mach) -> bool;
}

const fn null_link() -> snd_soc_acpi_link_adr {
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: core::ptr::null(),
    }
}

const fn null_mach() -> snd_soc_acpi_mach {
    snd_soc_acpi_mach {
        link_mask: 0,
        links: core::ptr::null(),
        machine_check: None,
        drv_name: core::ptr::null(),
        sof_tplg_filename: core::ptr::null(),
        fw_filename: core::ptr::null(),
    }
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
    snd_soc_acpi_endpoint { num: 0, aggregated: 0, group_position: 0, group_id: 0 },
    /* Amp Endpoint, work as spk_l_endpoint */
    snd_soc_acpi_endpoint { num: 1, aggregated: 1, group_position: 0, group_id: 1 },
    /* DMIC Endpoint */
    snd_soc_acpi_endpoint { num: 2, aggregated: 0, group_position: 0, group_id: 0 },
];

static jack_dmic_endpoints: [snd_soc_acpi_endpoint; 2] = [
    /* Jack Endpoint */
    snd_soc_acpi_endpoint { num: 0, aggregated: 0, group_position: 0, group_id: 0 },
    /*
     * rt721 endpoint #2 maps to AIF3 (internal DMIC capture).
     * Endpoint #1 is AIF2 amp path and is handled by external amps
     * on this platform.
     */
    snd_soc_acpi_endpoint { num: 2, aggregated: 0, group_position: 0, group_id: 0 },
];

static rt712_vb_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000130025D071201u64,
    num_endpoints: jack_amp_g1_dmic_endpoints.len() as u32,
    endpoints: jack_amp_g1_dmic_endpoints.as_ptr(),
    name_prefix: c"rt712".as_ptr(),
}];

static rt711_rt1316_group_adr: [snd_soc_acpi_adr_device; 3] = [
    snd_soc_acpi_adr_device { adr: 0x000030025D071101u64, num_endpoints: 1, endpoints: &single_endpoint, name_prefix: c"rt711".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x000030025D131601u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"rt1316-1".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x000032025D131601u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"rt1316-2".as_ptr() },
];

static rt714_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x130025d071401u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: c"rt714".as_ptr(),
}];

static acp70_4_in_1_sdca: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_rt1316_group_adr.len() as u32, adr_d: rt711_rt1316_group_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt714_adr.len() as u32, adr_d: rt714_adr.as_ptr() },
    null_link(),
];

static rt722_endpoints: [snd_soc_acpi_endpoint; 3] = [
    snd_soc_acpi_endpoint { num: 0, aggregated: 0, group_position: 0, group_id: 0 },
    snd_soc_acpi_endpoint { num: 1, aggregated: 0, group_position: 0, group_id: 0 },
    snd_soc_acpi_endpoint { num: 2, aggregated: 0, group_position: 0, group_id: 0 },
];

static rt722_0_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000030025d072201u64,
    num_endpoints: rt722_endpoints.len() as u32,
    endpoints: rt722_endpoints.as_ptr(),
    name_prefix: c"rt722".as_ptr(),
}];

static rt1320_1_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000130025D132001u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: c"rt1320-1".as_ptr(),
}];

static cs42l43_endpoints: [snd_soc_acpi_endpoint; 4] = [
    snd_soc_acpi_endpoint { num: 0, aggregated: 0, group_position: 0, group_id: 0 }, /* Jack Playback Endpoint */
    snd_soc_acpi_endpoint { num: 1, aggregated: 0, group_position: 0, group_id: 0 }, /* DMIC Capture Endpoint */
    snd_soc_acpi_endpoint { num: 2, aggregated: 0, group_position: 0, group_id: 0 }, /* Jack Capture Endpoint */
    snd_soc_acpi_endpoint { num: 3, aggregated: 0, group_position: 0, group_id: 0 }, /* Speaker Playback Endpoint */
];

static cs35l56x4_l1u3210_adr: [snd_soc_acpi_adr_device; 4] = [
    snd_soc_acpi_adr_device { adr: 0x00013301FA355601u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"AMP1".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00013201FA355601u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"AMP2".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00013101FA355601u64, num_endpoints: 1, endpoints: &spk_2_endpoint, name_prefix: c"AMP3".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00013001FA355601u64, num_endpoints: 1, endpoints: &spk_3_endpoint, name_prefix: c"AMP4".as_ptr() },
];

static cs35l63x2_l0u01_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device { adr: 0x00003001FA356301u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"AMP1".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00003101FA356301u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"AMP2".as_ptr() },
];

static cs35l63x2_l1u01_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device { adr: 0x00013001FA356301u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"AMP1".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00013101FA356301u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"AMP2".as_ptr() },
];

static cs35l63x2_l1u13_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device { adr: 0x00013101FA356301u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"AMP1".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00013301FA356301u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"AMP2".as_ptr() },
];

static cs35l63x4_l0u0246_adr: [snd_soc_acpi_adr_device; 4] = [
    snd_soc_acpi_adr_device { adr: 0x00003001FA356301u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"AMP1".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00003201FA356301u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"AMP2".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00003401FA356301u64, num_endpoints: 1, endpoints: &spk_2_endpoint, name_prefix: c"AMP3".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00003601FA356301u64, num_endpoints: 1, endpoints: &spk_3_endpoint, name_prefix: c"AMP4".as_ptr() },
];

static cs42l43_l0u0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x00003001FA424301u64, num_endpoints: cs42l43_endpoints.len() as u32, endpoints: cs42l43_endpoints.as_ptr(), name_prefix: c"cs42l43".as_ptr() }];
static cs42l43_l0u1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x00003101FA424301u64, num_endpoints: cs42l43_endpoints.len() as u32, endpoints: cs42l43_endpoints.as_ptr(), name_prefix: c"cs42l43".as_ptr() }];
static cs42l43b_l0u1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x00003101FA2A3B01u64, num_endpoints: cs42l43_endpoints.len() as u32, endpoints: cs42l43_endpoints.as_ptr(), name_prefix: c"cs42l43".as_ptr() }];

static cs42l43_l1u0_cs35l56x4_l1u0123_adr: [snd_soc_acpi_adr_device; 5] = [
    snd_soc_acpi_adr_device { adr: 0x00013001FA424301u64, num_endpoints: cs42l43_endpoints.len() as u32, endpoints: cs42l43_endpoints.as_ptr(), name_prefix: c"cs42l43".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00013001FA355601u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"AMP1".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00013101FA355601u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"AMP2".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00013201FA355601u64, num_endpoints: 1, endpoints: &spk_2_endpoint, name_prefix: c"AMP3".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00013301FA355601u64, num_endpoints: 1, endpoints: &spk_3_endpoint, name_prefix: c"AMP4".as_ptr() },
];

static cs42l45_l0u0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00003001FA424501u64,
    /* Re-use endpoints, but cs42l45 has no speaker */
    num_endpoints: (cs42l43_endpoints.len() - 1) as u32,
    endpoints: cs42l43_endpoints.as_ptr(),
    name_prefix: c"cs42l45".as_ptr(),
}];

static cs42l45_l1u0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00013001FA424501u64,
    /* Re-use endpoints, but cs42l45 has no speaker */
    num_endpoints: (cs42l43_endpoints.len() - 1) as u32,
    endpoints: cs42l43_endpoints.as_ptr(),
    name_prefix: c"cs42l45".as_ptr(),
}];

static acp70_cs35l56x4_l1u3210: [snd_soc_acpi_link_adr; 2] = [snd_soc_acpi_link_adr { mask: BIT(1), num_adr: cs35l56x4_l1u3210_adr.len() as u32, adr_d: cs35l56x4_l1u3210_adr.as_ptr() }, null_link()];
static acp70_cs35l63x4_l0u0246: [snd_soc_acpi_link_adr; 2] = [snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs35l63x4_l0u0246_adr.len() as u32, adr_d: cs35l63x4_l0u0246_adr.as_ptr() }, null_link()];
static acp70_cs42l43_l0u1: [snd_soc_acpi_link_adr; 2] = [snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs42l43_l0u1_adr.len() as u32, adr_d: cs42l43_l0u1_adr.as_ptr() }, null_link()];
static acp70_cs42l43b_l0u1: [snd_soc_acpi_link_adr; 2] = [snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs42l43b_l0u1_adr.len() as u32, adr_d: cs42l43b_l0u1_adr.as_ptr() }, null_link()];

static acp70_cs42l43_l0u0_cs35l56x4_l1u3210: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs42l43_l0u0_adr.len() as u32, adr_d: cs42l43_l0u0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: cs35l56x4_l1u3210_adr.len() as u32, adr_d: cs35l56x4_l1u3210_adr.as_ptr() },
    null_link(),
];
static acp70_cs42l43_l1u0_cs35l56x4_l1u0123: [snd_soc_acpi_link_adr; 2] = [snd_soc_acpi_link_adr { mask: BIT(1), num_adr: cs42l43_l1u0_cs35l56x4_l1u0123_adr.len() as u32, adr_d: cs42l43_l1u0_cs35l56x4_l1u0123_adr.as_ptr() }, null_link()];
static acp70_cs42l45_l0u0: [snd_soc_acpi_link_adr; 2] = [snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs42l45_l0u0_adr.len() as u32, adr_d: cs42l45_l0u0_adr.as_ptr() }, null_link()];
static acp70_cs42l45_l0u0_cs35l63x2_l1u01: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs42l45_l0u0_adr.len() as u32, adr_d: cs42l45_l0u0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: cs35l63x2_l1u01_adr.len() as u32, adr_d: cs35l63x2_l1u01_adr.as_ptr() },
    null_link(),
];
static acp70_cs42l45_l0u0_cs35l63x2_l1u13: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs42l45_l0u0_adr.len() as u32, adr_d: cs42l45_l0u0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: cs35l63x2_l1u13_adr.len() as u32, adr_d: cs35l63x2_l1u13_adr.as_ptr() },
    null_link(),
];
static acp70_cs42l45_l1u0: [snd_soc_acpi_link_adr; 2] = [snd_soc_acpi_link_adr { mask: BIT(1), num_adr: cs42l45_l1u0_adr.len() as u32, adr_d: cs42l45_l1u0_adr.as_ptr() }, null_link()];
static acp70_cs42l45_l1u0_cs35l63x2_l0u01: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: cs42l45_l1u0_adr.len() as u32, adr_d: cs42l45_l1u0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs35l63x2_l0u01_adr.len() as u32, adr_d: cs35l63x2_l0u01_adr.as_ptr() },
    null_link(),
];
static acp70_cs42l45_l1u0_cs35l63x4_l0u0246: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: cs42l45_l1u0_adr.len() as u32, adr_d: cs42l45_l1u0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs35l63x4_l0u0246_adr.len() as u32, adr_d: cs35l63x4_l0u0246_adr.as_ptr() },
    null_link(),
];
static acp70_alc712_vb_l1: [snd_soc_acpi_link_adr; 2] = [snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt712_vb_1_group1_adr.len() as u32, adr_d: rt712_vb_1_group1_adr.as_ptr() }, null_link()];
static acp70_rt722_only: [snd_soc_acpi_link_adr; 2] = [snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt722_0_single_adr.len() as u32, adr_d: rt722_0_single_adr.as_ptr() }, null_link()];
static acp70_rt722_l0_rt1320_l1: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt722_0_single_adr.len() as u32, adr_d: rt722_0_single_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1320_1_single_adr.len() as u32, adr_d: rt1320_1_single_adr.as_ptr() },
    null_link(),
];

static tas2783_2_adr: [snd_soc_acpi_adr_device; 4] = [
    snd_soc_acpi_adr_device { adr: 0x00003c0102000001u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"tas2783-1".as_ptr() }, /* left */
    snd_soc_acpi_adr_device { adr: 0x00003d0102000001u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"tas2783-2".as_ptr() }, /* right */
    snd_soc_acpi_adr_device { adr: 0x0000390102000001u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"tas2783-3".as_ptr() }, /* left */
    snd_soc_acpi_adr_device { adr: 0x00003a0102000001u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"tas2783-4".as_ptr() }, /* right */
];
static acp70_tas2783_2: [snd_soc_acpi_link_adr; 2] = [snd_soc_acpi_link_adr { mask: BIT(0), num_adr: tas2783_2_adr.len() as u32, adr_d: tas2783_2_adr.as_ptr() }, null_link()];

static rt1320_0_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000030025D132001u64, num_endpoints: 1, endpoints: &single_endpoint, name_prefix: c"rt1320-1".as_ptr() }];
static rt722_1_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000130025d072201u64, num_endpoints: rt722_endpoints.len() as u32, endpoints: rt722_endpoints.as_ptr(), name_prefix: c"rt722".as_ptr() }];
static acp70_rt1320_l0_rt722_l1: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt1320_0_single_adr.len() as u32, adr_d: rt1320_0_single_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt722_1_single_adr.len() as u32, adr_d: rt722_1_single_adr.as_ptr() },
    null_link(),
];

static rt721_l1u0_tas2783x2_l1u8b_adr: [snd_soc_acpi_adr_device; 3] = [
    snd_soc_acpi_adr_device {
        adr: 0x000130025D072101u64,
        /*
         * On this platform speakers are provided by two TAS2783 amps.
         * Keep rt721 as UAJ + DMIC only.
         */
        num_endpoints: jack_dmic_endpoints.len() as u32,
        endpoints: jack_dmic_endpoints.as_ptr(),
        name_prefix: c"rt721".as_ptr(),
    },
    snd_soc_acpi_adr_device { adr: 0x0001380102000001u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"tas2783-1".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x00013B0102000001u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"tas2783-2".as_ptr() },
];
static acp70_rt721_l1u0_tas2783x2_l1u8b: [snd_soc_acpi_link_adr; 2] = [snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt721_l1u0_tas2783x2_l1u8b_adr.len() as u32, adr_d: rt721_l1u0_tas2783x2_l1u8b_adr.as_ptr() }, null_link()];

static rt712_vb_l1u0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000130025D071201u64,
    /*
     * On this platform speakers are provided by two TAS2783 amps,
     * so the AIF2 amp path is left unused: jack + DMIC only.
     */
    num_endpoints: jack_dmic_endpoints.len() as u32,
    endpoints: jack_dmic_endpoints.as_ptr(),
    name_prefix: c"rt712".as_ptr(),
}];

/*
 * Unique ID 0xC drives the left speaker and 0x9 the right one. The order of the
 * entries matters as much as the endpoints: asoc_sdw_parse_sdw_endpoints()
 * appends them to the dailink in array order and sdw_compute_slave_ports()
 * hands out payload block offsets in that same order, so the first entry is the
 * one that receives channel 0.
 */
static tas2783x2_l0u9c_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device { adr: 0x00003C0102000001u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: c"tas2783-1".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x0000390102000001u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: c"tas2783-2".as_ptr() },
];

/* HP OmniBook X Flip 14-kc0xxx (board 8EA1) */
static acp70_tas2783x2_l0u9c_rt712_vb_l1u0: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: tas2783x2_l0u9c_adr.len() as u32, adr_d: tas2783x2_l0u9c_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt712_vb_l1u0_adr.len() as u32, adr_d: rt712_vb_l1u0_adr.as_ptr() },
    null_link(),
];

static rt721_endpoints: [snd_soc_acpi_endpoint; 3] = [
    snd_soc_acpi_endpoint { num: 0, aggregated: 0, group_position: 0, group_id: 0 }, /* Jack Playback/Capture Endpoint (AIF1) */
    snd_soc_acpi_endpoint { num: 1, aggregated: 0, group_position: 0, group_id: 0 }, /* Speaker Amplifier Endpoint (AIF2, internal amp) */
    snd_soc_acpi_endpoint { num: 2, aggregated: 0, group_position: 0, group_id: 0 }, /* DMIC Capture Endpoint (AIF3) */
];
static rt721_1_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device { adr: 0x000130025D072101u64, num_endpoints: rt721_endpoints.len() as u32, endpoints: rt721_endpoints.as_ptr(), name_prefix: c"rt721".as_ptr() }];
static acp70_rt721_only: [snd_soc_acpi_link_adr; 2] = [snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt721_1_single_adr.len() as u32, adr_d: rt721_1_single_adr.as_ptr() }, null_link()];

#[unsafe(no_mangle)]
pub static snd_soc_acpi_amd_acp70_sdw_machines: [snd_soc_acpi_mach; 22] = [
    snd_soc_acpi_mach { link_mask: BIT(0), links: acp70_tas2783_2.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0) | BIT(1), links: acp70_tas2783x2_l0u9c_rt712_vb_l1u0.as_ptr(), machine_check: Some(snd_soc_acpi_amd_sdca_is_device_rt712_vb), drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0) | BIT(1), links: acp70_rt1320_l0_rt722_l1.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0) | BIT(1), links: acp70_rt722_l0_rt1320_l1.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0) | BIT(1), links: acp70_4_in_1_sdca.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0) | BIT(1), links: acp70_cs42l43_l0u0_cs35l56x4_l1u3210.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0) | BIT(1), links: acp70_cs42l45_l1u0_cs35l63x4_l0u0246.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0) | BIT(1), links: acp70_cs42l45_l0u0_cs35l63x2_l1u01.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0) | BIT(1), links: acp70_cs42l45_l0u0_cs35l63x2_l1u13.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0) | BIT(1), links: acp70_cs42l45_l1u0_cs35l63x2_l0u01.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(1), links: acp70_cs42l43_l1u0_cs35l56x4_l1u0123.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(1), links: acp70_cs35l56x4_l1u3210.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0), links: acp70_cs35l63x4_l0u0246.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0), links: acp70_rt722_only.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0), links: acp70_cs42l43_l0u1.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0), links: acp70_cs42l43b_l0u1.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(0), links: acp70_cs42l45_l0u0.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(1), links: acp70_cs42l45_l1u0.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(1), links: acp70_alc712_vb_l1.as_ptr(), machine_check: Some(snd_soc_acpi_amd_sdca_is_device_rt712_vb), drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(1), links: acp70_rt721_l1u0_tas2783x2_l1u8b.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    snd_soc_acpi_mach { link_mask: BIT(1), links: acp70_rt721_only.as_ptr(), machine_check: None, drv_name: c"amd_sdw".as_ptr(), sof_tplg_filename: core::ptr::null(), fw_filename: core::ptr::null() },
    null_mach(),
];

#[unsafe(no_mangle)]
pub static snd_soc_acpi_amd_acp70_sof_sdw_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach {
        link_mask: BIT(0),
        links: acp70_rt722_only.as_ptr(),
        machine_check: None,
        drv_name: c"amd_sof_sdw".as_ptr(),
        sof_tplg_filename: c"sof-acp_7_0-rt722-l0.tplg".as_ptr(),
        fw_filename: c"sof-acp_7_0.ri".as_ptr(),
    },
    null_mach(),
];

/* MODULE_DESCRIPTION("AMD ACP7.0 & ACP7.1 tables and support for ACPI enumeration"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_AUTHOR("Vijendar.Mukunda@amd.com"); */
/*
 * #if IS_ENABLED(CONFIG_SND_SOC_ACPI_AMD_SDCA_QUIRKS)
 * MODULE_IMPORT_NS("SND_SOC_SDCA");
 * #endif
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
