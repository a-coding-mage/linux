/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * soc-acpi-intel-get-tplg.h - get-tplg-files ops
 *
 * Copyright (c) 2025, Intel Corporation.
 *
 */

// C header guard omitted in Rust:
// _SND_SOC_ACPI_INTEL_GET_TPLG_H

use core::ffi::c_char;

#[repr(C)]
pub struct snd_soc_card {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn sof_sdw_get_tplg_files(
        card: *mut snd_soc_card,
        mach: *const snd_soc_acpi_mach,
        prefix: *const c_char,
        tplg_files: *mut *const *const c_char,
        best_effort: bool,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
