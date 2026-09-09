// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Setup code for SAMA5
 *
 *  Copyright (C) 2013 Atmel,
 *                2013 Ludovic Desroches <ludovic.desroches@atmel.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.
extern "C" {
    fn sam_secure_init();
    fn sam_linux_is_optee_available() -> bool;
    fn sama5_pm_init();
    fn sama5d2_pm_init();
}

// OP-TEE configures the L2 cache and does not allow modifying it yet.
unsafe fn sama5_l2c310_write_sec(_val: libc::c_ulong, _reg: libc::c_uint) {
}

unsafe fn sama5_secure_cache_init() {
    sam_secure_init();
    // IS_ENABLED(CONFIG_OUTER_CACHE) is a build-time configuration condition.
    if sam_linux_is_optee_available() {
        // outer_cache.write_sec = sama5_l2c310_write_sec;
        // The assignment is supplied by the outer-cache subsystem.
    }
}

static SAMA5_DT_BOARD_COMPAT: [*const libc::c_char; 2] = [
    b"atmel,sama5\0".as_ptr() as *const libc::c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(sama5_dt, "Atmel SAMA5")
// Maintainer: Atmel
// .init_late = sama5_pm_init
// .dt_compat = sama5_dt_board_compat
// MACHINE_END

static SAMA5_ALT_DT_BOARD_COMPAT: [*const libc::c_char; 2] = [
    b"atmel,sama5d4\0".as_ptr() as *const libc::c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(sama5_alt_dt, "Atmel SAMA5")
// Maintainer: Atmel
// .init_late = sama5_pm_init
// .dt_compat = sama5_alt_dt_board_compat
// .l2c_aux_mask = ~0UL
// MACHINE_END

static SAMA5D2_COMPAT: [*const libc::c_char; 2] = [
    b"atmel,sama5d2\0".as_ptr() as *const libc::c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(sama5d2, "Atmel SAMA5")
// Maintainer: Atmel
// .init_early = sama5_secure_cache_init
// .init_late = sama5d2_pm_init
// .dt_compat = sama5d2_compat
// .l2c_aux_mask = ~0UL
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
