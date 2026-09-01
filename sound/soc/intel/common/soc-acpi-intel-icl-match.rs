// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-icl-match.c - tables and support for ICL ACPI enumeration.
 *
 * Copyright (c) 2018, Intel Corporation.
 *
 */

// C includes translated as external dependencies:
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const ESSX_83X6_CODECS: [*const core::ffi::c_char; 3] = [
    b"ESSX8316\0".as_ptr() as *const core::ffi::c_char,
    b"ESSX8326\0".as_ptr() as *const core::ffi::c_char,
    b"ESSX8336\0".as_ptr() as *const core::ffi::c_char,
];

static essx_83x6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 3,
    codecs: ESSX_83X6_CODECS,
};

#[no_mangle]
pub static mut snd_soc_acpi_intel_icl_machines: [snd_soc_acpi_mach; 4] = [
    snd_soc_acpi_mach {
        id: b"INT34C2\0".as_ptr() as *const core::ffi::c_char,
        drv_name: b"icl_rt274\0".as_ptr() as *const core::ffi::c_char,
        fw_filename: b"intel/dsp_fw_icl.bin\0".as_ptr() as *const core::ffi::c_char,
        sof_tplg_filename: b"sof-icl-rt274.tplg\0".as_ptr() as *const core::ffi::c_char,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        id: b"10EC5682\0".as_ptr() as *const core::ffi::c_char,
        drv_name: b"icl_rt5682_def\0".as_ptr() as *const core::ffi::c_char,
        sof_tplg_filename: b"sof-icl-rt5682.tplg\0".as_ptr() as *const core::ffi::c_char,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        comp_ids: &essx_83x6 as *const snd_soc_acpi_codecs,
        drv_name: b"sof-essx8336\0".as_ptr() as *const core::ffi::c_char,
        // the tplg suffix is added at run time
        sof_tplg_filename: b"sof-icl-es8336\0".as_ptr() as *const core::ffi::c_char,
        tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER
            | SND_SOC_ACPI_TPLG_INTEL_SSP_MSB
            | SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_icl_machines);

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

static rt700_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000010025D070000u64,
    num_endpoints: 1,
    endpoints: &single_endpoint as *const snd_soc_acpi_endpoint,
    name_prefix: b"rt700\0".as_ptr() as *const core::ffi::c_char,
}];

static icl_rvp: [snd_soc_acpi_link_adr; 2] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt700_0_adr.len(),
        adr_d: rt700_0_adr.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];

static rt711_0_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000020025D071100u64,
    num_endpoints: 1,
    endpoints: &single_endpoint as *const snd_soc_acpi_endpoint,
    name_prefix: b"rt711\0".as_ptr() as *const core::ffi::c_char,
}];

static rt1308_1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000120025D130800u64,
    num_endpoints: 1,
    endpoints: &single_endpoint as *const snd_soc_acpi_endpoint,
    name_prefix: b"rt1308-1\0".as_ptr() as *const core::ffi::c_char,
}];

static rt1308_1_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000120025D130800u64,
    num_endpoints: 1,
    endpoints: &spk_l_endpoint as *const snd_soc_acpi_endpoint,
    name_prefix: b"rt1308-1\0".as_ptr() as *const core::ffi::c_char,
}];

static rt1308_2_group1_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000220025D130800u64,
    num_endpoints: 1,
    endpoints: &spk_r_endpoint as *const snd_soc_acpi_endpoint,
    name_prefix: b"rt1308-2\0".as_ptr() as *const core::ffi::c_char,
}];

static rt715_3_adr: [snd_soc_acpi_adr_device; 1] = [snd_soc_acpi_adr_device {
    adr: 0x000320025D071500u64,
    num_endpoints: 1,
    endpoints: &single_endpoint as *const snd_soc_acpi_endpoint,
    name_prefix: b"rt715\0".as_ptr() as *const core::ffi::c_char,
}];

static icl_3_in_1_default: [snd_soc_acpi_link_adr; 5] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt711_0_adr.len(),
        adr_d: rt711_0_adr.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: rt1308_1_group1_adr.len(),
        adr_d: rt1308_1_group1_adr.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_link_adr {
        mask: BIT(2),
        num_adr: rt1308_2_group1_adr.len(),
        adr_d: rt1308_2_group1_adr.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_link_adr {
        mask: BIT(3),
        num_adr: rt715_3_adr.len(),
        adr_d: rt715_3_adr.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];

static icl_3_in_1_mono_amp: [snd_soc_acpi_link_adr; 4] = [
    snd_soc_acpi_link_adr {
        mask: BIT(0),
        num_adr: rt711_0_adr.len(),
        adr_d: rt711_0_adr.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_link_adr {
        mask: BIT(1),
        num_adr: rt1308_1_adr.len(),
        adr_d: rt1308_1_adr.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_link_adr {
        mask: BIT(3),
        num_adr: rt715_3_adr.len(),
        adr_d: rt715_3_adr.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];

#[no_mangle]
pub static mut snd_soc_acpi_intel_icl_sdw_machines: [snd_soc_acpi_mach; 4] = [
    snd_soc_acpi_mach {
        link_mask: 0xF, // 4 active links required
        links: icl_3_in_1_default.as_ptr(),
        drv_name: b"sof_sdw\0".as_ptr() as *const core::ffi::c_char,
        sof_tplg_filename: b"sof-icl-rt711-rt1308-rt715.tplg\0".as_ptr()
            as *const core::ffi::c_char,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: 0xB, // 3 active links required
        links: icl_3_in_1_mono_amp.as_ptr(),
        drv_name: b"sof_sdw\0".as_ptr() as *const core::ffi::c_char,
        sof_tplg_filename: b"sof-icl-rt711-rt1308-rt715-mono.tplg\0".as_ptr()
            as *const core::ffi::c_char,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_acpi_mach {
        link_mask: 0x1, // rt700 connected on link0
        links: icl_rvp.as_ptr(),
        drv_name: b"sof_sdw\0".as_ptr() as *const core::ffi::c_char,
        sof_tplg_filename: b"sof-icl-rt700.tplg\0".as_ptr() as *const core::ffi::c_char,
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_icl_sdw_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
