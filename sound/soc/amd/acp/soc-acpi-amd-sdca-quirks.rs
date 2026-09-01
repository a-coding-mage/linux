// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-amd-sdca-quirks.c - tables and support for SDCA quirks
 *
 * Copyright(c) 2025 Advanced Micro Devices, Inc. All rights reserved.
 *
 */

// C dependencies:
// #include <linux/soundwire/sdw_amd.h>
// #include <sound/sdca.h>
// #include <sound/soc-acpi.h>
// #include "soc-acpi-amd-sdca-quirks.h"

use core::ffi::c_void;

#[repr(C)]
pub struct sdw_amd_peripherals {
    pub num_peripherals: i32,
    pub array: *mut *mut c_void,
}

#[repr(C)]
pub struct sdw_amd_ctx {
    pub peripherals: *mut sdw_amd_peripherals,
}

unsafe extern "C" {
    fn sdca_device_quirk_match(dev: *mut c_void, quirks: u32) -> bool;
    static SDCA_QUIRKS_RT712_VB: u32;
}

/*
 * Pretend machine quirk. The argument type is not the traditional
 * 'struct snd_soc_acpi_mach' pointer but instead the sdw_amd_ctx
 * which contains the peripheral information required for the
 * SoundWire/SDCA filter on the SMART_MIC setup and interface
 * revision. When the return value is false, the entry in the
 * 'snd_soc_acpi_mach' table needs to be skipped.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_acpi_amd_sdca_is_device_rt712_vb(arg: *mut c_void) -> bool {
    let ctx = arg as *mut sdw_amd_ctx;
    let mut i: i32;

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

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
