// SPDX-License-Identifier: GPL-2.0-only
/*
 * amd-acp63-acpi-match.c - tables and support for ACP 6.3 platform
 * ACPI enumeration.
 *
 * Copyright 2024 Advanced Micro Devices, Inc.
 */

// C dependencies:
// #include <sound/soc-acpi.h>
// #include "../mach-config.h"

const fn BIT(n: u32) -> u64 {
    1u64 << n
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

static rt711_rt1316_group_adr: [snd_soc_acpi_adr_device; 3] = [
    snd_soc_acpi_adr_device {
        adr: 0x000030025D071101u64,
        num_endpoints: 1,
        endpoints: &single_endpoint,
        name_prefix: b"rt711\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x000030025D131601u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: b"rt1316-1\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x000032025D131601u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: b"rt1316-2\0".as_ptr() as *const ::core::ffi::c_char,
    },
];

static rt714_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x130025d071401u64,
    num_endpoints: 1,
    endpoints: &single_endpoint,
    name_prefix: b"rt714\0".as_ptr() as *const ::core::ffi::c_char,
}];

static acp63_4_in_1_sdca: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt711_rt1316_group_adr.len(),
        adr_d: rt711_rt1316_group_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: rt714_adr.len(),
        adr_d: rt714_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

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

static rt722_0_single_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000030025d072201u64,
    num_endpoints: rt722_endpoints.len(),
    endpoints: rt722_endpoints.as_ptr(),
    name_prefix: b"rt722\0".as_ptr() as *const ::core::ffi::c_char,
}];

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

static cs35l56x4_l1u3210_adr: [snd_soc_acpi_adr_device; 4] = [
    snd_soc_acpi_adr_device {
        adr: 0x00013301FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: b"AMP1\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x00013201FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: b"AMP2\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x00013101FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_2_endpoint,
        name_prefix: b"AMP3\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x00013001FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_3_endpoint,
        name_prefix: b"AMP4\0".as_ptr() as *const ::core::ffi::c_char,
    },
];

static cs35l63x2_l0u01_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x00003001FA356301u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: b"AMP1\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x00003101FA356301u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: b"AMP2\0".as_ptr() as *const ::core::ffi::c_char,
    },
];

static cs35l63x2_l1u01_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x00013001FA356301u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: b"AMP1\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x00013101FA356301u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: b"AMP2\0".as_ptr() as *const ::core::ffi::c_char,
    },
];

static cs35l63x2_l1u13_adr: [snd_soc_acpi_adr_device; 2] = [
    snd_soc_acpi_adr_device {
        adr: 0x00013101FA356301u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: b"AMP1\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x00013301FA356301u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: b"AMP2\0".as_ptr() as *const ::core::ffi::c_char,
    },
];

static cs35l63x4_l0u0246_adr: [snd_soc_acpi_adr_device; 4] = [
    snd_soc_acpi_adr_device {
        adr: 0x00003001FA356301u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: b"AMP1\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x00003201FA356301u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: b"AMP2\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x00003401FA356301u64,
        num_endpoints: 1,
        endpoints: &spk_2_endpoint,
        name_prefix: b"AMP3\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x00003601FA356301u64,
        num_endpoints: 1,
        endpoints: &spk_3_endpoint,
        name_prefix: b"AMP4\0".as_ptr() as *const ::core::ffi::c_char,
    },
];

static cs42l43_l0u0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00003001FA424301u64,
    num_endpoints: cs42l43_endpoints.len(),
    endpoints: cs42l43_endpoints.as_ptr(),
    name_prefix: b"cs42l43\0".as_ptr() as *const ::core::ffi::c_char,
}];

static cs42l43_l0u1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00003101FA424301u64,
    num_endpoints: cs42l43_endpoints.len(),
    endpoints: cs42l43_endpoints.as_ptr(),
    name_prefix: b"cs42l43\0".as_ptr() as *const ::core::ffi::c_char,
}];

static cs42l43b_l0u1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00003101FA2A3B01u64,
    num_endpoints: cs42l43_endpoints.len(),
    endpoints: cs42l43_endpoints.as_ptr(),
    name_prefix: b"cs42l43\0".as_ptr() as *const ::core::ffi::c_char,
}];

static cs42l43_l1u0_cs35l56x4_l1u0123_adr: [snd_soc_acpi_adr_device; 5] = [
    snd_soc_acpi_adr_device {
        adr: 0x00013001FA424301u64,
        num_endpoints: cs42l43_endpoints.len(),
        endpoints: cs42l43_endpoints.as_ptr(),
        name_prefix: b"cs42l43\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x00013001FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint,
        name_prefix: b"AMP1\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x00013101FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint,
        name_prefix: b"AMP2\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x00013201FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_2_endpoint,
        name_prefix: b"AMP3\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_adr_device {
        adr: 0x00013301FA355601u64,
        num_endpoints: 1,
        endpoints: &spk_3_endpoint,
        name_prefix: b"AMP4\0".as_ptr() as *const ::core::ffi::c_char,
    },
];

static cs42l45_l0u0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00003001FA424501u64,
    /* Re-use endpoints, but cs42l45 has no speaker */
    num_endpoints: cs42l43_endpoints.len() - 1,
    endpoints: cs42l43_endpoints.as_ptr(),
    name_prefix: b"cs42l45\0".as_ptr() as *const ::core::ffi::c_char,
}];

static cs42l45_l1u0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x00013001FA424501u64,
    /* Re-use endpoints, but cs42l45 has no speaker */
    num_endpoints: cs42l43_endpoints.len() - 1,
    endpoints: cs42l43_endpoints.as_ptr(),
    name_prefix: b"cs42l45\0".as_ptr() as *const ::core::ffi::c_char,
}];

static acp63_cs35l56x4_l1u3210: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: cs35l56x4_l1u3210_adr.len(),
        adr_d: cs35l56x4_l1u3210_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

static acp63_cs35l63x4_l0u0246: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: cs35l63x4_l0u0246_adr.len(),
        adr_d: cs35l63x4_l0u0246_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

static acp63_cs42l43_l0u1: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: cs42l43_l0u1_adr.len(),
        adr_d: cs42l43_l0u1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

static acp63_cs42l43b_l0u1: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: cs42l43b_l0u1_adr.len(),
        adr_d: cs42l43b_l0u1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

static acp63_cs42l43_l0u0_cs35l56x4_l1u3210: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: cs42l43_l0u0_adr.len(),
        adr_d: cs42l43_l0u0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: cs35l56x4_l1u3210_adr.len(),
        adr_d: cs35l56x4_l1u3210_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

static acp63_cs42l43_l1u0_cs35l56x4_l1u0123: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: cs42l43_l1u0_cs35l56x4_l1u0123_adr.len(),
        adr_d: cs42l43_l1u0_cs35l56x4_l1u0123_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

static acp63_cs42l45_l0u0: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: cs42l45_l0u0_adr.len(),
        adr_d: cs42l45_l0u0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

static acp63_cs42l45_l0u0_cs35l63x2_l1u01: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: cs42l45_l0u0_adr.len(),
        adr_d: cs42l45_l0u0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: cs35l63x2_l1u01_adr.len(),
        adr_d: cs35l63x2_l1u01_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

static acp63_cs42l45_l0u0_cs35l63x2_l1u13: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: cs42l45_l0u0_adr.len(),
        adr_d: cs42l45_l0u0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: cs35l63x2_l1u13_adr.len(),
        adr_d: cs35l63x2_l1u13_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

static acp63_cs42l45_l1u0: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: cs42l45_l1u0_adr.len(),
        adr_d: cs42l45_l1u0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

static acp63_cs42l45_l1u0_cs35l63x2_l0u01: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: cs42l45_l1u0_adr.len(),
        adr_d: cs42l45_l1u0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: cs35l63x2_l0u01_adr.len(),
        adr_d: cs35l63x2_l0u01_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

static acp63_cs42l45_l1u0_cs35l63x4_l0u0246: [snd_soc_acpi_link_adr; 3] = [
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: cs42l45_l1u0_adr.len(),
        adr_d: cs42l45_l1u0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: cs35l63x4_l0u0246_adr.len(),
        adr_d: cs35l63x4_l0u0246_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

static acp63_rt722_only: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt722_0_single_adr.len(),
        adr_d: rt722_0_single_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: ::core::ptr::null(),
    },
];

#[no_mangle]
pub static snd_soc_acpi_amd_acp63_sof_sdw_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(1),
        links: acp63_4_in_1_sdca.as_ptr(),
        drv_name: b"amd_sof_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: b"sof-acp_6_3-rt711-l0-rt1316-l0-rt714-l1.tplg\0".as_ptr()
            as *const ::core::ffi::c_char,
        fw_filename: b"sof-acp_6_3.ri\0".as_ptr() as *const ::core::ffi::c_char,
    },
    snd_soc_acpi_mach {
        link_mask: 0,
        links: ::core::ptr::null(),
        drv_name: ::core::ptr::null(),
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
];
// EXPORT_SYMBOL(snd_soc_acpi_amd_acp63_sof_sdw_machines);

#[no_mangle]
pub static snd_soc_acpi_amd_acp63_sdw_machines: [snd_soc_acpi_mach; 15] = [
    snd_soc_acpi_mach {
        link_mask: BIT(0),
        links: acp63_rt722_only.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(1),
        links: acp63_4_in_1_sdca.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(1),
        links: acp63_cs42l43_l0u0_cs35l56x4_l1u3210.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(1),
        links: acp63_cs42l45_l1u0_cs35l63x4_l0u0246.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(1),
        links: acp63_cs42l45_l0u0_cs35l63x2_l1u01.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(1),
        links: acp63_cs42l45_l0u0_cs35l63x2_l1u13.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0) | BIT(1),
        links: acp63_cs42l45_l1u0_cs35l63x2_l0u01.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(1),
        links: acp63_cs42l43_l1u0_cs35l56x4_l1u0123.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(1),
        links: acp63_cs35l56x4_l1u3210.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0),
        links: acp63_cs35l63x4_l0u0246.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0),
        links: acp63_cs42l43_l0u1.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0),
        links: acp63_cs42l43b_l0u1.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(0),
        links: acp63_cs42l45_l0u0.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: BIT(1),
        links: acp63_cs42l45_l1u0.as_ptr(),
        drv_name: b"amd_sdw\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
    snd_soc_acpi_mach {
        link_mask: 0,
        links: ::core::ptr::null(),
        drv_name: ::core::ptr::null(),
        sof_tplg_filename: ::core::ptr::null(),
        fw_filename: ::core::ptr::null(),
    },
];
// EXPORT_SYMBOL(snd_soc_acpi_amd_acp63_sdw_machines);

// MODULE_DESCRIPTION("AMD ACP6.3 tables and support for ACPI enumeration");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Vijendar.Mukunda@amd.com");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
