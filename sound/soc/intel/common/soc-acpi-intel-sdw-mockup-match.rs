// SPDX-License-Identifier: GPL-2.0-only
//
// soc-acpi-intel-sdw-mockup-match.c - tables and support for SoundWire
// mockup device ACPI enumeration.
//
// Copyright (c) 2021, Intel Corporation.
//

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::ffi::c_char;

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
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_link_adr {
    pub mask: u32,
    pub num_adr: u32,
    pub adr_d: *const snd_soc_acpi_adr_device,
}

unsafe impl Sync for snd_soc_acpi_adr_device {}
unsafe impl Sync for snd_soc_acpi_link_adr {}

static sdw_mockup_single_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint {
    num: 0,
    aggregated: 0,
    group_position: 0,
    group_id: 0,
};

static sdw_mockup_l_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint {
    num: 0,
    aggregated: 1,
    group_position: 0,
    group_id: 1,
};

static sdw_mockup_r_endpoint: snd_soc_acpi_endpoint = snd_soc_acpi_endpoint {
    num: 0,
    aggregated: 1,
    group_position: 1,
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

static sdw_mockup_headset_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x0000000105AA5500u64,
    num_endpoints: 1,
    endpoints: &sdw_mockup_single_endpoint,
    name_prefix: b"sdw_mockup_headset0\0".as_ptr() as *const c_char,
}];

static sdw_mockup_headset_1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x0001000105AA5500u64,
    num_endpoints: 1,
    endpoints: &sdw_mockup_single_endpoint,
    name_prefix: b"sdw_mockup_headset1\0".as_ptr() as *const c_char,
}];

static sdw_mockup_amp_1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000100010555AA00u64,
    num_endpoints: 1,
    endpoints: &sdw_mockup_single_endpoint,
    name_prefix: b"sdw_mockup_amp1\0".as_ptr() as *const c_char,
}];

static sdw_mockup_amp_2_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000200010555AA00u64,
    num_endpoints: 1,
    endpoints: &sdw_mockup_single_endpoint,
    name_prefix: b"sdw_mockup_amp2\0".as_ptr() as *const c_char,
}];

static sdw_mockup_mic_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x0000000105555500u64,
    num_endpoints: 1,
    endpoints: &sdw_mockup_single_endpoint,
    name_prefix: b"sdw_mockup_mic0\0".as_ptr() as *const c_char,
}];

static sdw_mockup_mic_3_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x0003000105555500u64,
    num_endpoints: 1,
    endpoints: &sdw_mockup_single_endpoint,
    name_prefix: b"sdw_mockup_mic3\0".as_ptr() as *const c_char,
}];

static sdw_mockup_amp_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000100010555AA00u64,
    num_endpoints: 1,
    endpoints: &sdw_mockup_l_endpoint,
    name_prefix: b"sdw_mockup_amp1_l\0".as_ptr() as *const c_char,
}];

static sdw_mockup_amp_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000200010555AA00u64,
    num_endpoints: 1,
    endpoints: &sdw_mockup_r_endpoint,
    name_prefix: b"sdw_mockup_amp2_r\0".as_ptr() as *const c_char,
}];

static sdw_mockup_multi_function_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x0000000105AAAA01u64,
    num_endpoints: jack_amp_g1_dmic_endpoints.len() as u32,
    endpoints: jack_amp_g1_dmic_endpoints.as_ptr(),
    name_prefix: b"sdw_mockup_mmulti-function\0".as_ptr() as *const c_char,
}];

pub static sdw_mockup_headset_1amp_mic: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr {
        mask: 1u32 << 0,
        num_adr: sdw_mockup_headset_0_adr.len() as u32,
        adr_d: sdw_mockup_headset_0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 1u32 << 1,
        num_adr: sdw_mockup_amp_1_adr.len() as u32,
        adr_d: sdw_mockup_amp_1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 1u32 << 3,
        num_adr: sdw_mockup_mic_3_adr.len() as u32,
        adr_d: sdw_mockup_mic_3_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: core::ptr::null(),
    },
];

pub static sdw_mockup_headset_2amps_mic: [snd_soc_acpi_link_adr; 5] = [
    snd_soc_acpi_link_adr {
        mask: 1u32 << 0,
        num_adr: sdw_mockup_headset_0_adr.len() as u32,
        adr_d: sdw_mockup_headset_0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 1u32 << 1,
        num_adr: sdw_mockup_amp_1_group1_adr.len() as u32,
        adr_d: sdw_mockup_amp_1_group1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 1u32 << 2,
        num_adr: sdw_mockup_amp_2_group1_adr.len() as u32,
        adr_d: sdw_mockup_amp_2_group1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 1u32 << 3,
        num_adr: sdw_mockup_mic_3_adr.len() as u32,
        adr_d: sdw_mockup_mic_3_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: core::ptr::null(),
    },
];

pub static sdw_mockup_mic_headset_1amp: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr {
        mask: 1u32 << 1,
        num_adr: sdw_mockup_headset_1_adr.len() as u32,
        adr_d: sdw_mockup_headset_1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 1u32 << 2,
        num_adr: sdw_mockup_amp_2_adr.len() as u32,
        adr_d: sdw_mockup_amp_2_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 1u32 << 0,
        num_adr: sdw_mockup_mic_0_adr.len() as u32,
        adr_d: sdw_mockup_mic_0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: core::ptr::null(),
    },
];

pub static sdw_mockup_multi_func: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: 1u32 << 0,
        num_adr: sdw_mockup_multi_function_adr.len() as u32,
        adr_d: sdw_mockup_multi_function_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: core::ptr::null(),
    },
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
