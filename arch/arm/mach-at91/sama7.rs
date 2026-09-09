// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Setup code for SAMA7
 *
 * Copyright (C) 2021 Microchip Technology, Inc. and its subsidiaries
 *
 */

use core::ffi::c_char;

// Supplied by the platform power-management implementation.
unsafe extern "C" {
    fn sama7_pm_init();
}

static SAMA7_COMPAT_MICROCHIP_SAMA7: &[u8] = b"microchip,sama7\0";

static mut sama7_dt_board_compat: [*const c_char; 2] = [
    SAMA7_COMPAT_MICROCHIP_SAMA7.as_ptr() as *const c_char,
    core::ptr::null(),
];

// Corresponds to the architecture-specific DT_MACHINE_START/MACHINE_END
// descriptor emitted by the ARM machine-description macros.
#[repr(C)]
struct DtMachine {
    name: *const c_char,
    init_late: Option<unsafe extern "C" fn()>,
    dt_compat: *const *const c_char,
}

static SAMA7_DT_NAME: &[u8] = b"Microchip SAMA7\0";

static mut sama7_dt: DtMachine = DtMachine {
    name: SAMA7_DT_NAME.as_ptr() as *const c_char,
    // Maintainer: Microchip
    init_late: Some(sama7_pm_init),
    dt_compat: sama7_dt_board_compat.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
