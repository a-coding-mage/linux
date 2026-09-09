// SPDX-License-Identifier: GPL-2.0+
/*
 * Setup code for SAM9X7.
 *
 * Copyright (C) 2023 Microchip Technology Inc. and its subsidiaries
 *
 * Author: Varshini Rajendran <varshini.rajendran@microchip.com>
 */

use core::ffi::c_char;

// Supplied by the architecture and generic platform code.
unsafe extern "C" {
    fn sam9x7_pm_init();
}

static SAM9X7_DT_BOARD_COMPAT: [*const c_char; 2] = [
    b"microchip,sam9x7\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// Equivalent of:
// DT_MACHINE_START(sam9x7_dt, "Microchip SAM9X7")
//     .init_late = sam9x7_pm_init,
//     .dt_compat = sam9x7_dt_board_compat,
// MACHINE_END
// The machine descriptor type and registration macro are supplied externally.
#[allow(dead_code)]
unsafe fn sam9x7_dt_init_late() {
    sam9x7_pm_init();
}

#[allow(dead_code)]
static SAM9X7_DT_MACHINE_NAME: &str = "Microchip SAM9X7";

#[allow(dead_code)]
static SAM9X7_DT_MACHINE_COMPAT: *const *const c_char = SAM9X7_DT_BOARD_COMPAT.as_ptr();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
