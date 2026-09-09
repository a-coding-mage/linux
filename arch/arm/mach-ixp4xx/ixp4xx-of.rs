// SPDX-License-Identifier: GPL-2.0
/*
 * IXP4xx Device Tree boot support
 */

use core::ffi::c_char;

/*
 * We handle 4 different SoC families. These compatible strings are enough
 * to provide the core so that different boards can add their more detailed
 * specifics.
 */
static IXP4XX_OF_BOARD_COMPAT: [*const c_char; 5] = [
    b"intel,ixp42x\0".as_ptr() as *const c_char,
    b"intel,ixp43x\0".as_ptr() as *const c_char,
    b"intel,ixp45x\0".as_ptr() as *const c_char,
    b"intel,ixp46x\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

/*
 * Equivalent of:
 *
 * DT_MACHINE_START(IXP4XX_DT, "IXP4xx (Device Tree)")
 *     .dt_compat = ixp4xx_of_board_compat,
 * MACHINE_END
 *
 * The machine descriptor and registration are supplied by the architecture
 * support represented by the original DT_MACHINE_START/MACHINE_END macros.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
