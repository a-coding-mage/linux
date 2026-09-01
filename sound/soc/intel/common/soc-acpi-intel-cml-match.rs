// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-cml-match.c - tables and support for CML ACPI enumeration.
 *
 * Copyright (c) 2019, Intel Corporation.
 *
 */

use core::ffi::{c_char, c_void};

// Dependencies from <sound/soc-acpi.h> and <sound/soc-acpi-intel-match.h>.
extern "C" {
    fn snd_soc_acpi_codec_list(mach: *mut snd_soc_acpi_mach) -> isize;
}

const fn BIT(nr: u32) -> u64 {
    1u64 << nr
}

const SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER: u64 = 1 << 0;
const SND_SOC_ACPI_TPLG_INTEL_SSP_MSB: u64 = 1 << 1;
const SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER: u64 = 1 << 2;

#[repr(C)]
pub struct snd_soc_acpi_codecs {
    pub num_codecs: u32,
    pub codecs: [*const c_char; 3],
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
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_link_adr {
    pub mask: u64,
    pub num_adr: usize,
    pub adr_d: *const snd_soc_acpi_adr_device,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: *const c_char,
    pub drv_name: *const c_char,
    pub machine_quirk: Option<unsafe extern "C" fn(*mut snd_soc_acpi_mach) -> isize>,
    pub quirk_data: *const c_void,
    pub sof_tplg_filename: *const c_char,
    pub tplg_quirk_mask: u64,
    pub comp_ids: *const snd_soc_acpi_codecs,
    pub link_mask: u64,
    pub links: *const snd_soc_acpi_link_adr,
}

const fn null_mach() -> snd_soc_acpi_mach {
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        machine_quirk: None,
        quirk_data: core::ptr::null(),
        sof_tplg_filename: core::ptr::null(),
        tplg_quirk_mask: 0,
        comp_ids: core::ptr::null(),
        link_mask: 0,
        links: core::ptr::null(),
    }
}

const fn null_link_adr() -> snd_soc_acpi_link_adr {
    snd_soc_acpi_link_adr {
        mask: 0,
        num_adr: 0,
        adr_d: core::ptr::null(),
    }
}

static essx_83x6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 3,
    codecs: [
        b"ESSX8316\0".as_ptr() as *const c_char,
        b"ESSX8326\0".as_ptr() as *const c_char,
        b"ESSX8336\0".as_ptr() as *const c_char,
    ],
};

static rt1011_spk_codecs: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [
        b"10EC1011\0".as_ptr() as *const c_char,
        core::ptr::null(),
        core::ptr::null(),
    ],
};

static rt1015_spk_codecs: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [
        b"10EC1015\0".as_ptr() as *const c_char,
        core::ptr::null(),
        core::ptr::null(),
    ],
};

static max98357a_spk_codecs: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [
        b"MX98357A\0".as_ptr() as *const c_char,
        core::ptr::null(),
        core::ptr::null(),
    ],
};

static max98390_spk_codecs: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [
        b"MX98390\0".as_ptr() as *const c_char,
        core::ptr::null(),
        core::ptr::null(),
    ],
};

/*
 * The order of the three entries with .id = "10EC5682" matters
 * here, because DSDT tables expose an ACPI HID for the MAX98357A
 * speaker amplifier which is not populated on the board.
 */
#[no_mangle]
pub static mut snd_soc_acpi_intel_cml_machines: [snd_soc_acpi_mach; 8] = [
    snd_soc_acpi_mach {
        id: b"10EC5682\0".as_ptr() as *const c_char,
        drv_name: b"cml_rt5682_def\0".as_ptr() as *const c_char,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &rt1011_spk_codecs as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-cml-rt1011-rt5682.tplg\0".as_ptr() as *const c_char,
        ..null_mach()
    },
    snd_soc_acpi_mach {
        id: b"10EC5682\0".as_ptr() as *const c_char,
        drv_name: b"cml_rt5682_def\0".as_ptr() as *const c_char,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &rt1015_spk_codecs as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-cml-rt1011-rt5682.tplg\0".as_ptr() as *const c_char,
        ..null_mach()
    },
    snd_soc_acpi_mach {
        id: b"10EC5682\0".as_ptr() as *const c_char,
        drv_name: b"cml_rt5682_def\0".as_ptr() as *const c_char,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &max98357a_spk_codecs as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-cml-rt5682-max98357a.tplg\0".as_ptr() as *const c_char,
        ..null_mach()
    },
    snd_soc_acpi_mach {
        id: b"10EC5682\0".as_ptr() as *const c_char,
        drv_name: b"cml_rt5682_def\0".as_ptr() as *const c_char,
        sof_tplg_filename: b"sof-cml-rt5682.tplg\0".as_ptr() as *const c_char,
        ..null_mach()
    },
    snd_soc_acpi_mach {
        id: b"DLGS7219\0".as_ptr() as *const c_char,
        drv_name: b"cml_da7219_def\0".as_ptr() as *const c_char,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &max98357a_spk_codecs as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-cml-da7219-max98357a.tplg\0".as_ptr() as *const c_char,
        ..null_mach()
    },
    snd_soc_acpi_mach {
        id: b"DLGS7219\0".as_ptr() as *const c_char,
        drv_name: b"cml_da7219_def\0".as_ptr() as *const c_char,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &max98390_spk_codecs as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-cml-da7219-max98390.tplg\0".as_ptr() as *const c_char,
        ..null_mach()
    },
    snd_soc_acpi_mach {
        comp_ids: &essx_83x6 as *const snd_soc_acpi_codecs,
        drv_name: b"sof-essx8336\0".as_ptr() as *const c_char,
        sof_tplg_filename: b"sof-cml-es8336\0".as_ptr() as *const c_char, /* the tplg suffix is added at run time */
        tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER |
            SND_SOC_ACPI_TPLG_INTEL_SSP_MSB |
            SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
        ..null_mach()
    },
    null_mach(),
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_cml_machines);

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

static rt700_1_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000110025D070000u64,
        num_endpoints: 1,
        endpoints: &single_endpoint as *const snd_soc_acpi_endpoint,
        name_prefix: b"rt700\0".as_ptr() as *const c_char,
    },
];

static cml_rvp: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: rt700_1_adr.len(),
        adr_d: rt700_1_adr.as_ptr(),
    },
    null_link_adr(),
];

static rt711_0_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000020025D071100u64,
        num_endpoints: 1,
        endpoints: &single_endpoint as *const snd_soc_acpi_endpoint,
        name_prefix: b"rt711\0".as_ptr() as *const c_char,
    },
];

static rt1308_1_single_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000120025D130800u64,
        num_endpoints: 1,
        endpoints: &single_endpoint as *const snd_soc_acpi_endpoint,
        name_prefix: b"rt1308-1\0".as_ptr() as *const c_char,
    },
];

static rt1308_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000120025D130800u64,
        num_endpoints: 1,
        endpoints: &spk_l_endpoint as *const snd_soc_acpi_endpoint,
        name_prefix: b"rt1308-1\0".as_ptr() as *const c_char,
    },
];

static rt1308_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000220025D130800u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint as *const snd_soc_acpi_endpoint,
        name_prefix: b"rt1308-2\0".as_ptr() as *const c_char,
    },
];

static rt715_3_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000320025D071500u64,
        num_endpoints: 1,
        endpoints: &single_endpoint as *const snd_soc_acpi_endpoint,
        name_prefix: b"rt715\0".as_ptr() as *const c_char,
    },
];

static rt711_sdca_0_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000030025D071101u64,
        num_endpoints: 1,
        endpoints: &single_endpoint as *const snd_soc_acpi_endpoint,
        name_prefix: b"rt711\0".as_ptr() as *const c_char,
    },
];

static rt1316_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000131025D131601u64, /* unique ID is set for some reason */
        num_endpoints: 1,
        endpoints: &spk_l_endpoint as *const snd_soc_acpi_endpoint,
        name_prefix: b"rt1316-1\0".as_ptr() as *const c_char,
    },
];

static rt1316_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000230025D131601u64,
        num_endpoints: 1,
        endpoints: &spk_r_endpoint as *const snd_soc_acpi_endpoint,
        name_prefix: b"rt1316-2\0".as_ptr() as *const c_char,
    },
];

static rt714_3_adr: [snd_soc_acpi_adr_device; 1] = [
    snd_soc_acpi_adr_device {
        adr: 0x000330025D071401u64,
        num_endpoints: 1,
        endpoints: &single_endpoint as *const snd_soc_acpi_endpoint,
        name_prefix: b"rt714\0".as_ptr() as *const c_char,
    },
];

static cml_3_in_1_default: [snd_soc_acpi_link_adr; 5] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt711_0_adr.len(),
        adr_d: rt711_0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: rt1308_1_group1_adr.len(),
        adr_d: rt1308_1_group1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(2),
        num_adr: rt1308_2_group1_adr.len(),
        adr_d: rt1308_2_group1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(3),
        num_adr: rt715_3_adr.len(),
        adr_d: rt715_3_adr.as_ptr(),
    },
    null_link_adr(),
];

static cml_3_in_1_mono_amp: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt711_0_adr.len(),
        adr_d: rt711_0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: rt1308_1_single_adr.len(),
        adr_d: rt1308_1_single_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(3),
        num_adr: rt715_3_adr.len(),
        adr_d: rt715_3_adr.as_ptr(),
    },
    null_link_adr(),
];

static cml_3_in_1_sdca: [snd_soc_acpi_link_adr; 5] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt711_sdca_0_adr.len(),
        adr_d: rt711_sdca_0_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: rt1316_1_group1_adr.len(),
        adr_d: rt1316_1_group1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(2),
        num_adr: rt1316_2_group1_adr.len(),
        adr_d: rt1316_2_group1_adr.as_ptr(),
    },
    snd_soc_acpi_link_adr {
        mask: BIT(3),
        num_adr: rt714_3_adr.len(),
        adr_d: rt714_3_adr.as_ptr(),
    },
    null_link_adr(),
];

#[no_mangle]
pub static mut snd_soc_acpi_intel_cml_sdw_machines: [snd_soc_acpi_mach; 5] = [
    snd_soc_acpi_mach {
        link_mask: 0xF, /* 4 active links required */
        links: cml_3_in_1_default.as_ptr(),
        drv_name: b"sof_sdw\0".as_ptr() as *const c_char,
        sof_tplg_filename: b"sof-cml-rt711-rt1308-rt715.tplg\0".as_ptr() as *const c_char,
        ..null_mach()
    },
    snd_soc_acpi_mach {
        link_mask: 0xF, /* 4 active links required */
        links: cml_3_in_1_sdca.as_ptr(),
        drv_name: b"sof_sdw\0".as_ptr() as *const c_char,
        sof_tplg_filename: b"sof-cml-rt711-rt1316-rt714.tplg\0".as_ptr() as *const c_char,
        ..null_mach()
    },
    snd_soc_acpi_mach {
        /*
         * link_mask should be 0xB, but all links are enabled by BIOS.
         * This entry will be selected if there is no rt1308 exposed
         * on link2 since it will fail to match the above entry.
         */
        link_mask: 0xF,
        links: cml_3_in_1_mono_amp.as_ptr(),
        drv_name: b"sof_sdw\0".as_ptr() as *const c_char,
        sof_tplg_filename: b"sof-cml-rt711-rt1308-mono-rt715.tplg\0".as_ptr() as *const c_char,
        ..null_mach()
    },
    snd_soc_acpi_mach {
        link_mask: 0x2, /* RT700 connected on Link1 */
        links: cml_rvp.as_ptr(),
        drv_name: b"sof_sdw\0".as_ptr() as *const c_char,
        sof_tplg_filename: b"sof-cml-rt700.tplg\0".as_ptr() as *const c_char,
        ..null_mach()
    },
    null_mach(),
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_cml_sdw_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
