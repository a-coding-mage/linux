// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2012 Steffen Trumtrar, Pengutronix
 *
 * based on imx27-dt.c
 */

// C dependencies:
// #include <asm/mach/arch.h>
// #include "common.h"
// #include "mx35.h"

use core::ffi::c_char;

unsafe extern "C" {
    fn mx35_map_io();
    fn imx35_init_early();
}

// static const char * const imx35_dt_board_compat[] __initconst = {
//     "fsl,imx35",
//     NULL
// };
#[used]
#[link_section = ".init.rodata"]
static IMX35_DT_BOARD_COMPAT: [*const c_char; 2] = [
    b"fsl,imx35\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(IMX35_DT, "Freescale i.MX35 (Device Tree Support)")
//     .l2c_aux_val  = 0,
//     .l2c_aux_mask = ~0,
//     .map_io       = mx35_map_io,
//     .init_early   = imx35_init_early,
//     .dt_compat    = imx35_dt_board_compat,
// MACHINE_END
//
// The DT_MACHINE_START/MACHINE_END macros expand to the architecture's
// machine descriptor definition supplied by <asm/mach/arch.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
