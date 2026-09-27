/* SPDX-License-Identifier: GPL-2.0-only */
// Original C header guard: BRCMNAND_PLAT_DATA_H

#[repr(C)]
pub struct brcmnand_platform_data {
    pub chip_select: kernel::ffi::c_int,
    pub part_probe_types: *const *const kernel::ffi::c_char,
    pub ecc_stepsize: kernel::ffi::c_uint,
    pub ecc_strength: kernel::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
