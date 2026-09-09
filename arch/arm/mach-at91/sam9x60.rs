// SPDX-License-Identifier: GPL-2.0+
/*
 * Setup code for SAM9X60.
 *
 * Copyright (C) 2019 Microchip Technology Inc. and its subsidiaries
 *
 * Author: Claudiu Beznea <claudiu.beznea@microchip.com>
 */

// C dependencies supplied by the surrounding architecture and generic setup.

extern "C" {
    fn sam9x60_pm_init();
}

static SAM9X60_DT_BOARD_COMPAT: [*const core::ffi::c_char; 2] = [
    b"microchip,sam9x60\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

// Translation of:
// DT_MACHINE_START(sam9x60_dt, "Microchip SAM9X60")
//     .init_late = sam9x60_pm_init,
//     .dt_compat = sam9x60_dt_board_compat,
// MACHINE_END
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct sam9x60_dt_machine {
    pub init_late: Option<unsafe extern "C" fn()>,
    pub dt_compat: *const *const core::ffi::c_char,
}

#[no_mangle]
pub static mut sam9x60_dt: sam9x60_dt_machine = sam9x60_dt_machine {
    // Maintainer: Microchip
    init_late: Some(sam9x60_pm_init),
    dt_compat: SAM9X60_DT_BOARD_COMPAT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
