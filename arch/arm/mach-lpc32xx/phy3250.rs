// SPDX-License-Identifier: GPL-2.0+
/*
 * Platform support for LPC32xx SoC
 *
 * Author: Kevin Wells <kevin.wells@nxp.com>
 *
 * Copyright (C) 2012 Roland Stigge <stigge@antcom.de>
 * Copyright (C) 2010 NXP Semiconductors
 */

// C dependencies: <asm/mach/arch.h> and "common.h".

use core::ffi::c_char;

extern "C" {
    fn lpc32xx_check_uid();
    fn lpc32xx_pm_init();
    fn lpc32xx_serial_init();
    fn lpc32xx_map_io();
}

// __init
unsafe fn lpc3250_machine_init() {
    lpc32xx_check_uid();
    lpc32xx_pm_init();
    lpc32xx_serial_init();
}

// __initconst
static lpc32xx_dt_compat: [*const c_char; 5] = [
    b"nxp,lpc3220\0".as_ptr() as *const c_char,
    b"nxp,lpc3230\0".as_ptr() as *const c_char,
    b"nxp,lpc3240\0".as_ptr() as *const c_char,
    b"nxp,lpc3250\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

/*
 * DT_MACHINE_START(LPC32XX_DT, "LPC32XX SoC (Flattened Device Tree)")
 *     .atag_offset = 0x100,
 *     .map_io = lpc32xx_map_io,
 *     .init_machine = lpc3250_machine_init,
 *     .dt_compat = lpc32xx_dt_compat,
 * MACHINE_END
 *
 * The machine-description declaration is supplied by the architecture
 * headers and is preserved here as the corresponding source-level intent.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
