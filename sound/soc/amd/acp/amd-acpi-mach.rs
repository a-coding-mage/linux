// SPDX-License-Identifier: GPL-2.0-only
/*
 * amd-acpi-match.c - tables and support for ACP platforms
 * ACPI enumeration.
 *
 * Copyright 2025 Advanced Micro Devices, Inc.
 */

/* Dependency intent from C source: #include <sound/soc-acpi.h> */

#[repr(C)]
pub struct snd_soc_acpi_codecs {
    pub num_codecs: i32,
    pub codecs: [*const u8; 1],
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: *const u8,
    pub drv_name: *const u8,
    pub machine_quirk: Option<unsafe extern "C" fn()>,
    pub quirk_data: *const core::ffi::c_void,
}

unsafe extern "C" {
    pub fn snd_soc_acpi_codec_list();
}

static amp_rt1019: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [b"10EC1019\0".as_ptr()],
};

static amp_max: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [b"MX98360A\0".as_ptr()],
};

#[unsafe(no_mangle)]
pub static mut snd_soc_acpi_amd_acp_machines: [snd_soc_acpi_mach; 6] = [
    snd_soc_acpi_mach {
        id: b"10EC5682\0".as_ptr(),
        drv_name: b"acp3xalc56821019\0".as_ptr(),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: (&amp_rt1019 as *const snd_soc_acpi_codecs).cast::<core::ffi::c_void>(),
    },
    snd_soc_acpi_mach {
        id: b"RTL5682\0".as_ptr(),
        drv_name: b"acp3xalc5682sm98360\0".as_ptr(),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: (&amp_max as *const snd_soc_acpi_codecs).cast::<core::ffi::c_void>(),
    },
    snd_soc_acpi_mach {
        id: b"RTL5682\0".as_ptr(),
        drv_name: b"acp3xalc5682s1019\0".as_ptr(),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: (&amp_rt1019 as *const snd_soc_acpi_codecs).cast::<core::ffi::c_void>(),
    },
    snd_soc_acpi_mach {
        id: b"AMDI1019\0".as_ptr(),
        drv_name: b"renoir-acp\0".as_ptr(),
        machine_quirk: None,
        quirk_data: core::ptr::null(),
    },
    snd_soc_acpi_mach {
        id: b"ESSX8336\0".as_ptr(),
        drv_name: b"acp3x-es83xx\0".as_ptr(),
        machine_quirk: None,
        quirk_data: core::ptr::null(),
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        machine_quirk: None,
        quirk_data: core::ptr::null(),
    },
];
/* EXPORT_SYMBOL_NS_GPL(snd_soc_acpi_amd_acp_machines, "SND_SOC_ACP_COMMON"); */

#[unsafe(no_mangle)]
pub static mut snd_soc_acpi_amd_rmb_acp_machines: [snd_soc_acpi_mach; 4] = [
    snd_soc_acpi_mach {
        id: b"10508825\0".as_ptr(),
        drv_name: b"rmb-nau8825-max\0".as_ptr(),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: (&amp_max as *const snd_soc_acpi_codecs).cast::<core::ffi::c_void>(),
    },
    snd_soc_acpi_mach {
        id: b"AMDI0007\0".as_ptr(),
        drv_name: b"rembrandt-acp\0".as_ptr(),
        machine_quirk: None,
        quirk_data: core::ptr::null(),
    },
    snd_soc_acpi_mach {
        id: b"RTL5682\0".as_ptr(),
        drv_name: b"rmb-rt5682s-rt1019\0".as_ptr(),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: (&amp_rt1019 as *const snd_soc_acpi_codecs).cast::<core::ffi::c_void>(),
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        machine_quirk: None,
        quirk_data: core::ptr::null(),
    },
];
/* EXPORT_SYMBOL_NS_GPL(snd_soc_acpi_amd_rmb_acp_machines, "SND_SOC_ACP_COMMON"); */

#[unsafe(no_mangle)]
pub static mut snd_soc_acpi_amd_acp63_acp_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach {
        id: b"AMDI0052\0".as_ptr(),
        drv_name: b"acp63-acp\0".as_ptr(),
        machine_quirk: None,
        quirk_data: core::ptr::null(),
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        machine_quirk: None,
        quirk_data: core::ptr::null(),
    },
];
/* EXPORT_SYMBOL_NS_GPL(snd_soc_acpi_amd_acp63_acp_machines, "SND_SOC_ACP_COMMON"); */

#[unsafe(no_mangle)]
pub static mut snd_soc_acpi_amd_acp70_acp_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach {
        id: b"AMDI0029\0".as_ptr(),
        drv_name: b"acp70-acp\0".as_ptr(),
        machine_quirk: None,
        quirk_data: core::ptr::null(),
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        machine_quirk: None,
        quirk_data: core::ptr::null(),
    },
];
/* EXPORT_SYMBOL_NS_GPL(snd_soc_acpi_amd_acp70_acp_machines, "SND_SOC_ACP_COMMON"); */

/* MODULE_DESCRIPTION("AMD ACP tables and support for ACPI enumeration"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_AUTHOR("Venkataprasad.potturu@amd.com"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
