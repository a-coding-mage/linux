// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-sdca-quirks.c - tables and support for SDCA quirks
 *
 * Copyright (c) 2024, Intel Corporation.
 *
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies from:
// <linux/dmi.h>
// <linux/soundwire/sdw_intel.h>
// <sound/sdca.h>
// <sound/soc-acpi.h>
// "soc-acpi-intel-sdca-quirks.h"

#[repr(C)]
pub struct sdw_intel_ctx {
    pub peripherals: *mut sdw_intel_peripherals,
}

#[repr(C)]
pub struct sdw_intel_peripherals {
    pub num_peripherals: c_int,
    pub array: *mut *mut c_void,
}

#[repr(C)]
pub struct dmi_strmatch {
    pub slot: c_int,
    pub substr: *const c_char,
}

#[repr(C)]
pub struct dmi_system_id {
    pub callback: Option<unsafe extern "C" fn(*const dmi_system_id) -> c_int>,
    pub ident: *const c_char,
    pub matches: [dmi_strmatch; 4],
    pub driver_data: *mut c_void,
}

unsafe extern "C" {
    fn sdca_device_quirk_match(device: *mut c_void, quirk: u64) -> bool;
    fn dmi_check_system(list: *const dmi_system_id) -> c_int;
}

// External constants/macros supplied by the included headers.
const SDCA_QUIRKS_RT712_VB: u64 = 0;
const DMI_SYS_VENDOR: c_int = 0;

/*
 * Pretend machine quirk. The argument type is not the traditional
 * 'struct snd_soc_acpi_mach' pointer but instead the sdw_intel_ctx
 * which contains the peripheral information required for the
 * SoundWire/SDCA filter on the SMART_MIC setup and interface
 * revision. When the return value is false, the entry in the
 * 'snd_soc_acpi_mach' table needs to be skipped.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_acpi_intel_sdca_is_device_rt712_vb(arg: *mut c_void) -> bool {
    let ctx = arg as *mut sdw_intel_ctx;
    let mut i: c_int;

    if ctx.is_null() {
        return false;
    }

    i = 0;
    while i < (*(*ctx).peripherals).num_peripherals {
        if sdca_device_quirk_match(
            *(*(*ctx).peripherals).array.offset(i as isize),
            SDCA_QUIRKS_RT712_VB,
        ) {
            return true;
        }

        i += 1;
    }

    false
}
// EXPORT_SYMBOL_NS(snd_soc_acpi_intel_sdca_is_device_rt712_vb, "SND_SOC_ACPI_INTEL_SDCA_QUIRKS");

static FUNCTION_TOPOLOGY_QUIRK_TABLE: [dmi_system_id; 2] = [
    dmi_system_id {
        callback: None,
        ident: core::ptr::null(),
        matches: [
            dmi_strmatch {
                slot: DMI_SYS_VENDOR,
                substr: b"Google\0".as_ptr() as *const c_char,
            },
            dmi_strmatch {
                slot: 0,
                substr: core::ptr::null(),
            },
            dmi_strmatch {
                slot: 0,
                substr: core::ptr::null(),
            },
            dmi_strmatch {
                slot: 0,
                substr: core::ptr::null(),
            },
        ],
        driver_data: core::ptr::null_mut(),
    },
    dmi_system_id {
        callback: None,
        ident: core::ptr::null(),
        matches: [
            dmi_strmatch {
                slot: 0,
                substr: core::ptr::null(),
            },
            dmi_strmatch {
                slot: 0,
                substr: core::ptr::null(),
            },
            dmi_strmatch {
                slot: 0,
                substr: core::ptr::null(),
            },
            dmi_strmatch {
                slot: 0,
                substr: core::ptr::null(),
            },
        ],
        driver_data: core::ptr::null_mut(),
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_acpi_intel_no_function_topology(_arg: *mut c_void) -> bool {
    dmi_check_system(FUNCTION_TOPOLOGY_QUIRK_TABLE.as_ptr()) != 0
}
// EXPORT_SYMBOL_NS(snd_soc_acpi_intel_no_function_topology, "SND_SOC_ACPI_INTEL_SDCA_QUIRKS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_acpi_intel_rt712_vb_no_function_topology(
    arg: *mut c_void,
) -> bool {
    snd_soc_acpi_intel_sdca_is_device_rt712_vb(arg)
        && snd_soc_acpi_intel_no_function_topology(arg)
}
// EXPORT_SYMBOL_NS(snd_soc_acpi_intel_rt712_vb_no_function_topology,
//                  "SND_SOC_ACPI_INTEL_SDCA_QUIRKS");

// MODULE_DESCRIPTION("ASoC ACPI Intel SDCA quirks");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("SND_SOC_SDCA");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
