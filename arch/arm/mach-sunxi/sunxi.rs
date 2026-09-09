// SPDX-License-Identifier: GPL-2.0
/*
 * Device Tree support for Allwinner A1X SoCs
 *
 * Copyright (C) 2012 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 *
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_void};

extern "C" {
    fn of_clk_init(data: *const c_void);
    fn sun6i_reset_init();
    fn timer_probe();
    fn secure_cntvoff_init();
}

// The following compatibility tables correspond to the DT_MACHINE_START
// registrations in the original source.
static SUNXI_BOARD_DT_COMPAT: [*const c_char; 6] = [
    b"allwinner,sun4i-a10\0".as_ptr() as *const c_char,
    b"allwinner,sun5i-a10s\0".as_ptr() as *const c_char,
    b"allwinner,sun5i-a13\0".as_ptr() as *const c_char,
    b"allwinner,sun5i-r8\0".as_ptr() as *const c_char,
    b"nextthing,gr8\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

#[allow(dead_code)]
static SUN6I_BOARD_DT_COMPAT: [*const c_char; 3] = [
    b"allwinner,sun6i-a31\0".as_ptr() as *const c_char,
    b"allwinner,sun6i-a31s\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

unsafe extern "C" fn sun6i_timer_init() {
    of_clk_init(core::ptr::null());
    // CONFIG_RESET_CONTROLLER is a build-time configuration condition.
    #[cfg(feature = "CONFIG_RESET_CONTROLLER")]
    sun6i_reset_init();
    timer_probe();
}

// DT_MACHINE_START(SUNXI_DT, "Allwinner sun4i/sun5i Families")
//     .dt_compat = sunxi_board_dt_compat
// MACHINE_END
// DT_MACHINE_START(SUN6I_DT, "Allwinner sun6i (A31) Family")
//     .init_time = sun6i_timer_init, .dt_compat = sun6i_board_dt_compat
// MACHINE_END

static SUN7I_BOARD_DT_COMPAT: [*const c_char; 2] = [
    b"allwinner,sun7i-a20\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(SUN7I_DT, "Allwinner sun7i (A20) Family")
//     .dt_compat = sun7i_board_dt_compat
// MACHINE_END

static SUN8I_BOARD_DT_COMPAT: [*const c_char; 8] = [
    b"allwinner,sun8i-a23\0".as_ptr() as *const c_char,
    b"allwinner,sun8i-a33\0".as_ptr() as *const c_char,
    b"allwinner,sun8i-h2-plus\0".as_ptr() as *const c_char,
    b"allwinner,sun8i-h3\0".as_ptr() as *const c_char,
    b"allwinner,sun8i-r40\0".as_ptr() as *const c_char,
    b"allwinner,sun8i-v3\0".as_ptr() as *const c_char,
    b"allwinner,sun8i-v3s\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(SUN8I_DT, "Allwinner sun8i Family")
//     .init_time = sun6i_timer_init, .dt_compat = sun8i_board_dt_compat
// MACHINE_END

unsafe extern "C" fn sun8i_a83t_cntvoff_init() {
    // CONFIG_SMP is a build-time configuration condition.
    #[cfg(feature = "CONFIG_SMP")]
    secure_cntvoff_init();
}

static SUN8I_A83T_CNTVOFF_BOARD_DT_COMPAT: [*const c_char; 2] = [
    b"allwinner,sun8i-a83t\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(SUN8I_A83T_CNTVOFF_DT, "Allwinner A83t board")
//     .init_early = sun8i_a83t_cntvoff_init,
//     .init_time = sun6i_timer_init,
//     .dt_compat = sun8i_a83t_cntvoff_board_dt_compat
// MACHINE_END

static SUN9I_BOARD_DT_COMPAT: [*const c_char; 2] = [
    b"allwinner,sun9i-a80\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(SUN9I_DT, "Allwinner sun9i Family")
//     .dt_compat = sun9i_board_dt_compat
// MACHINE_END

static SUNIV_BOARD_DT_COMPAT: [*const c_char; 2] = [
    b"allwinner,suniv-f1c100s\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(SUNIV_DT, "Allwinner suniv Family")
//     .dt_compat = suniv_board_dt_compat
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
