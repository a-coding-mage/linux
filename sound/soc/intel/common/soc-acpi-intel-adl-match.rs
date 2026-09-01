// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-apci-intel-adl-match.c - tables and support for ADL ACPI enumeration.
 *
 * Copyright (c) 2020, Intel Corporation.
 */

// C includes translated as external dependency intent:
// <sound/soc-acpi.h>
// <sound/soc-acpi-intel-match.h>
// <sound/soc-acpi-intel-ssp-common.h>

const fn BIT(n: u32) -> u64 {
    1u64 << n
}

const SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER: u64 = 1u64 << 0;
const SND_SOC_ACPI_TPLG_INTEL_SSP_MSB: u64 = 1u64 << 1;
const SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER: u64 = 1u64 << 2;
const SND_SOC_ACPI_TPLG_INTEL_AMP_NAME: u64 = 1u64 << 3;
const SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME: u64 = 1u64 << 4;

const RT5682_ACPI_HID: *const u8 = b"10EC5682\0".as_ptr();
const RT5682S_ACPI_HID: *const u8 = b"RTL5682\0".as_ptr();
const CS42L42_ACPI_HID: *const u8 = b"10134242\0".as_ptr();
const DA7219_ACPI_HID: *const u8 = b"DLGS7219\0".as_ptr();
const NAU8825_ACPI_HID: *const u8 = b"10508825\0".as_ptr();
const RT5650_ACPI_HID: *const u8 = b"10EC5650\0".as_ptr();

#[repr(C)]
pub struct snd_soc_acpi_codecs {
    pub num_codecs: u32,
    pub codecs: [*const u8; 3],
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
    pub adr: u64,
    pub num_endpoints: u32,
    pub endpoints: *const snd_soc_acpi_endpoint,
    pub name_prefix: *const u8,
}

#[repr(C)]
pub struct snd_soc_acpi_link_adr {
    pub mask: u64,
    pub num_adr: u32,
    pub adr_d: *const snd_soc_acpi_adr_device,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: *const u8,
    pub comp_ids: *const snd_soc_acpi_codecs,
    pub drv_name: *const u8,
    pub machine_quirk: Option<unsafe extern "C" fn()>,
    pub quirk_data: *const core::ffi::c_void,
    pub sof_tplg_filename: *const u8,
    pub tplg_quirk_mask: u64,
    pub link_mask: u64,
    pub links: *const snd_soc_acpi_link_adr,
}

unsafe extern "C" {
    fn snd_soc_acpi_codec_list();
}

static essx_83x6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 3,
    codecs: [b"ESSX8316\0".as_ptr(), b"ESSX8326\0".as_ptr(), b"ESSX8336\0".as_ptr()],
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

static cs35l56_2_r_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x00023201FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: b"AMP3\0".as_ptr(),
    },
    snd_soc_acpi_adr_device {
        adr: 0x00023301FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_3_endpoint,
        name_prefix: b"AMP4\0".as_ptr(),
    },
];

static cs35l56_3_l_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x00033001fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: b"AMP1\0".as_ptr(),
    },
    snd_soc_acpi_adr_device {
        adr: 0x00033101fa355601u64,
        num_endpoints: 1,
        endpoints: &spk_2_endpoint,
        name_prefix: b"AMP2\0".as_ptr(),
    },
];

static cs42l43_endpoints: [snd_soc_acpi_endpoint; 4] = [
    snd_soc_acpi_endpoint {
        /* Jack Playback Endpoint */
        num: 0,
        aggregated: 0,
        group_position: 0,
        group_id: 0,
    },
    snd_soc_acpi_endpoint {
        /* DMIC Capture Endpoint */
        num: 1,
        aggregated: 0,
        group_position: 0,
        group_id: 0,
    },
    snd_soc_acpi_endpoint {
        /* Jack Capture Endpoint */
        num: 2,
        aggregated: 0,
        group_position: 0,
        group_id: 0,
    },
    snd_soc_acpi_endpoint {
        /* Speaker Playback Endpoint */
        num: 3,
        aggregated: 0,
        group_position: 0,
        group_id: 0,
    },
];

static cs42l43_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00003001FA424301u64,
    num_endpoints: cs42l43_endpoints.len() as u32,
    endpoints: cs42l43_endpoints.as_ptr(),
    name_prefix: b"cs42l43\0".as_ptr(),
}];

static rt711_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000020025D071100u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: b"rt711\0".as_ptr(),
}];

static rt1308_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000120025D130800u64,
    num_endpoints: 1,
    endpoints: &spk_l_endpoint,
    name_prefix: b"rt1308-1\0".as_ptr(),
}];

static rt1308_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000220025D130800u64,
    num_endpoints: 1,
    endpoints: &spk_r_endpoint,
    name_prefix: b"rt1308-2\0".as_ptr(),
}];

static rt715_3_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000320025D071500u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: b"rt715\0".as_ptr(),
}];

static rt711_sdca_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000030025D071101u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: b"rt711\0".as_ptr(),
}];

static rt711_sdca_2_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000230025D071101u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: b"rt711\0".as_ptr(),
}];

static rt1316_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000131025D131601u64, /* unique ID is set for some reason */
    num_endpoints: 1,
    endpoints: &spk_l_endpoint,
    name_prefix: b"rt1316-1\0".as_ptr(),
}];

static rt1316_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000230025D131601u64,
    num_endpoints: 1,
    endpoints: &spk_r_endpoint,
    name_prefix: b"rt1316-2\0".as_ptr(),
}];

static rt1316_3_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000330025D131601u64,
    num_endpoints: 1,
    endpoints: &spk_r_endpoint,
    name_prefix: b"rt1316-2\0".as_ptr(),
}];

static rt1316_0_group2_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000031025D131601u64,
    num_endpoints: 1,
    endpoints: &spk_l_endpoint,
    name_prefix: b"rt1316-1\0".as_ptr(),
}];

static rt1316_1_group2_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000130025D131601u64,
    num_endpoints: 1,
    endpoints: &spk_r_endpoint,
    name_prefix: b"rt1316-2\0".as_ptr(),
}];

static rt1316_2_group2_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000232025D131601u64,
    num_endpoints: 1,
    endpoints: &spk_r_endpoint,
    name_prefix: b"rt1316-2\0".as_ptr(),
}];

static rt1316_1_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000130025D131601u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: b"rt1316-1\0".as_ptr(),
}];

static rt1316_2_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000230025D131601u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: b"rt1316-1\0".as_ptr(),
}];

static rt1316_3_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000330025D131601u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: b"rt1316-1\0".as_ptr(),
}];

static rt714_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000030025D071401u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: b"rt714\0".as_ptr(),
}];

static rt714_2_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000230025D071401u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: b"rt714\0".as_ptr(),
}];

static rt714_3_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000330025D071401u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: b"rt714\0".as_ptr(),
}];

static adl_default: [snd_soc_acpi_link_adr; 5] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_0_adr.len() as u32, adr_d: rt711_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1308_1_group1_adr.len() as u32, adr_d: rt1308_1_group1_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1308_2_group1_adr.len() as u32, adr_d: rt1308_2_group1_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt715_3_adr.len() as u32, adr_d: rt715_3_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_sdca_default: [snd_soc_acpi_link_adr; 5] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len() as u32, adr_d: rt711_sdca_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1316_1_group1_adr.len() as u32, adr_d: rt1316_1_group1_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1316_2_group1_adr.len() as u32, adr_d: rt1316_2_group1_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt714_3_adr.len() as u32, adr_d: rt714_3_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_sdca_3_in_1: [snd_soc_acpi_link_adr; 5] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len() as u32, adr_d: rt711_sdca_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1316_1_group1_adr.len() as u32, adr_d: rt1316_1_group1_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt714_2_adr.len() as u32, adr_d: rt714_2_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt1316_3_group1_adr.len() as u32, adr_d: rt1316_3_group1_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_sdw_rt711_link2_rt1316_link01_rt714_link3: [snd_soc_acpi_link_adr; 5] = [
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt711_sdca_2_adr.len() as u32, adr_d: rt711_sdca_2_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt1316_0_group2_adr.len() as u32, adr_d: rt1316_0_group2_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1316_1_group2_adr.len() as u32, adr_d: rt1316_1_group2_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt714_3_adr.len() as u32, adr_d: rt714_3_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_sdw_rt711_link2_rt1316_link01: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt711_sdca_2_adr.len() as u32, adr_d: rt711_sdca_2_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt1316_0_group2_adr.len() as u32, adr_d: rt1316_0_group2_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1316_1_group2_adr.len() as u32, adr_d: rt1316_1_group2_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_sdw_rt1316_link12_rt714_link0: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1316_1_group1_adr.len() as u32, adr_d: rt1316_1_group1_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1316_2_group1_adr.len() as u32, adr_d: rt1316_2_group1_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt714_0_adr.len() as u32, adr_d: rt714_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_sdw_rt1316_link1_rt714_link0: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(1), num_adr: rt1316_1_single_adr.len() as u32, adr_d: rt1316_1_single_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt714_0_adr.len() as u32, adr_d: rt714_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_sdw_rt1316_link2_rt714_link3: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1316_2_single_adr.len() as u32, adr_d: rt1316_2_single_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt714_3_adr.len() as u32, adr_d: rt714_3_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_sdw_rt1316_link2_rt714_link0: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1316_2_single_adr.len() as u32, adr_d: rt1316_2_single_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt714_0_adr.len() as u32, adr_d: rt714_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_sdw_rt711_link0_rt1316_link3: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len() as u32, adr_d: rt711_sdca_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(3), num_adr: rt1316_3_single_adr.len() as u32, adr_d: rt1316_3_single_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_sdw_rt711_link0_rt1316_link2: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len() as u32, adr_d: rt711_sdca_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1316_2_single_adr.len() as u32, adr_d: rt1316_2_single_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static mx8373_2_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device { adr: 0x000223019F837300u64, num_endpoints: 1, endpoints: &spk_l_endpoint, name_prefix: b"Left\0".as_ptr() },
    snd_soc_acpi_adr_device { adr: 0x000227019F837300u64, num_endpoints: 1, endpoints: &spk_r_endpoint, name_prefix: b"Right\0".as_ptr() },
];

static rt5682_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000021025D568200u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: b"rt5682\0".as_ptr(),
}];

static adl_cs42l43_l0_cs35l56_l23: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: cs42l43_0_adr.len() as u32, adr_d: cs42l43_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: cs35l56_2_r_adr.len() as u32, adr_d: cs35l56_2_r_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(3), num_adr: cs35l56_3_l_adr.len() as u32, adr_d: cs35l56_3_l_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_rvp: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_0_adr.len() as u32, adr_d: rt711_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adlps_rvp: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt711_sdca_0_adr.len() as u32, adr_d: rt711_sdca_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_chromebook_base: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt5682_0_adr.len() as u32, adr_d: rt5682_0_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: mx8373_2_adr.len() as u32, adr_d: mx8373_2_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_sdw_rt1316_link02: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr { mask: BIT(0), num_adr: rt1316_0_group2_adr.len() as u32, adr_d: rt1316_0_group2_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: BIT(2), num_adr: rt1316_2_group2_adr.len() as u32, adr_d: rt1316_2_group2_adr.as_ptr() },
    snd_soc_acpi_link_adr { mask: 0, num_adr: 0, adr_d: core::ptr::null() },
];

static adl_max98357a_amp: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [b"MX98357A\0".as_ptr(), core::ptr::null(), core::ptr::null()],
};

static adl_rt5682_rt5682s_hp: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 2,
    codecs: [RT5682_ACPI_HID, RT5682S_ACPI_HID, core::ptr::null()],
};

static adl_rt1019p_amp: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [b"RTL1019\0".as_ptr(), core::ptr::null(), core::ptr::null()],
};

static adl_lt6911_hdmi: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [b"INTC10B0\0".as_ptr(), core::ptr::null(), core::ptr::null()],
};

#[unsafe(no_mangle)]
pub static mut snd_soc_acpi_intel_adl_machines: [snd_soc_acpi_mach; 13] = [
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: &adl_rt5682_rt5682s_hp, drv_name: b"adl_mx98357_rt5682\0".as_ptr(), machine_quirk: Some(snd_soc_acpi_codec_list), quirk_data: &adl_max98357a_amp as *const _ as *const core::ffi::c_void, sof_tplg_filename: b"sof-adl-max98357a-rt5682.tplg\0".as_ptr(), tplg_quirk_mask: 0, link_mask: 0, links: core::ptr::null() },
    snd_soc_acpi_mach { id: b"10508825\0".as_ptr(), comp_ids: core::ptr::null(), drv_name: b"adl_rt1019p_8825\0".as_ptr(), machine_quirk: Some(snd_soc_acpi_codec_list), quirk_data: &adl_rt1019p_amp as *const _ as *const core::ffi::c_void, sof_tplg_filename: b"sof-adl-rt1019-nau8825.tplg\0".as_ptr(), tplg_quirk_mask: 0, link_mask: 0, links: core::ptr::null() },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: &adl_rt5682_rt5682s_hp, drv_name: b"adl_rt5682_c1_h02\0".as_ptr(), machine_quirk: Some(snd_soc_acpi_codec_list), quirk_data: &adl_lt6911_hdmi as *const _ as *const core::ffi::c_void, sof_tplg_filename: b"sof-adl-rt5682-ssp1-hdmi-ssp02.tplg\0".as_ptr(), tplg_quirk_mask: 0, link_mask: 0, links: core::ptr::null() },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: &essx_83x6, drv_name: b"adl_es83x6_c1_h02\0".as_ptr(), machine_quirk: Some(snd_soc_acpi_codec_list), quirk_data: &adl_lt6911_hdmi as *const _ as *const core::ffi::c_void, sof_tplg_filename: b"sof-adl-es83x6-ssp1-hdmi-ssp02.tplg\0".as_ptr(), tplg_quirk_mask: 0, link_mask: 0, links: core::ptr::null() },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: &essx_83x6, drv_name: b"sof-essx8336\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-es8336\0".as_ptr(), tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER | SND_SOC_ACPI_TPLG_INTEL_SSP_MSB | SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER, link_mask: 0, links: core::ptr::null() },
    /* place boards for each headphone codec: sof driver will complete the
     * tplg name and machine driver will detect the amp type
     */
    snd_soc_acpi_mach { id: CS42L42_ACPI_HID, comp_ids: core::ptr::null(), drv_name: b"adl_cs42l42_def\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl\0".as_ptr(), tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME | SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME, link_mask: 0, links: core::ptr::null() },
    snd_soc_acpi_mach { id: DA7219_ACPI_HID, comp_ids: core::ptr::null(), drv_name: b"adl_da7219_def\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl\0".as_ptr(), tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME | SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME, link_mask: 0, links: core::ptr::null() },
    snd_soc_acpi_mach { id: NAU8825_ACPI_HID, comp_ids: core::ptr::null(), drv_name: b"adl_nau8825_def\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl\0".as_ptr(), tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME | SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME, link_mask: 0, links: core::ptr::null() },
    snd_soc_acpi_mach { id: RT5650_ACPI_HID, comp_ids: core::ptr::null(), drv_name: b"adl_rt5682_def\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl\0".as_ptr(), tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME | SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME, link_mask: 0, links: core::ptr::null() },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: &adl_rt5682_rt5682s_hp, drv_name: b"adl_rt5682_def\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl\0".as_ptr(), tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_AMP_NAME | SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME, link_mask: 0, links: core::ptr::null() },
    /* place amp-only boards in the end of table */
    snd_soc_acpi_mach { id: b"CSC3541\0".as_ptr(), comp_ids: core::ptr::null(), drv_name: b"adl_cs35l41\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-cs35l41.tplg\0".as_ptr(), tplg_quirk_mask: 0, link_mask: 0, links: core::ptr::null() },
    snd_soc_acpi_mach { id: b"INTC10B0\0".as_ptr(), comp_ids: core::ptr::null(), drv_name: b"adl_lt6911_hdmi_ssp\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-nocodec-hdmi-ssp02.tplg\0".as_ptr(), tplg_quirk_mask: 0, link_mask: 0, links: core::ptr::null() },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), drv_name: core::ptr::null(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: core::ptr::null(), tplg_quirk_mask: 0, link_mask: 0, links: core::ptr::null() },
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_adl_machines);

/* this table is used when there is no I2S codec present */
#[unsafe(no_mangle)]
pub static mut snd_soc_acpi_intel_adl_sdw_machines: [snd_soc_acpi_mach; 17] = [
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: BIT(0) | BIT(2) | BIT(3), links: adl_cs42l43_l0_cs35l56_l23.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-cs42l43-l0-cs35l56-l23.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0xF, /* 4 active links required */ links: adl_default.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt711-l0-rt1308-l12-rt715-l3.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0xF, /* 4 active links required */ links: adl_sdca_default.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt711-l0-rt1316-l12-rt714-l3.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0xF, /* 4 active links required */ links: adl_sdca_3_in_1.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt711-l0-rt1316-l13-rt714-l2.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0xF, /* 4 active links required */ links: adl_sdw_rt711_link2_rt1316_link01_rt714_link3.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt711-l2-rt1316-l01-rt714-l3.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0x7, /* rt1316 on link0 and link1 & rt711 on link2*/ links: adl_sdw_rt711_link2_rt1316_link01.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt711-l2-rt1316-l01.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0xC, /* rt1316 on link2 & rt714 on link3 */ links: adl_sdw_rt1316_link2_rt714_link3.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt1316-l2-mono-rt714-l3.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0x7, /* rt714 on link0 & two rt1316s on link1 and link2 */ links: adl_sdw_rt1316_link12_rt714_link0.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt1316-l12-rt714-l0.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0x3, /* rt1316 on link1 & rt714 on link0 */ links: adl_sdw_rt1316_link1_rt714_link0.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt1316-l1-mono-rt714-l0.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0x5, /* 2 active links required */ links: adl_sdw_rt1316_link2_rt714_link0.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt1316-l2-mono-rt714-l0.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0x9, /* 2 active links required */ links: adl_sdw_rt711_link0_rt1316_link3.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt711-l0-rt1316-l3.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0x5, /* 2 active links required */ links: adl_sdw_rt711_link0_rt1316_link2.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt711-l0-rt1316-l2.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0x1, /* link0 required */ links: adl_rvp.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt711.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0x1, /* link0 required */ links: adlps_rvp.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt711.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: 0x5, /* rt5682 on link0 & 2xmax98373 on link 2 */ links: adl_chromebook_base.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-sdw-max98373-rt5682.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), link_mask: BIT(0) | BIT(2), links: adl_sdw_rt1316_link02.as_ptr(), drv_name: b"sof_sdw\0".as_ptr(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: b"sof-adl-rt1316-l02.tplg\0".as_ptr(), tplg_quirk_mask: 0 },
    snd_soc_acpi_mach { id: core::ptr::null(), comp_ids: core::ptr::null(), drv_name: core::ptr::null(), machine_quirk: None, quirk_data: core::ptr::null(), sof_tplg_filename: core::ptr::null(), tplg_quirk_mask: 0, link_mask: 0, links: core::ptr::null() },
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_adl_sdw_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
