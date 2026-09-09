// SPDX-License-Identifier: GPL-2.0
/*
 * Device Tree support for Marvell Berlin SoCs.
 *
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 *
 * based on GPL'ed 2.6 kernel sources
 *  (c) Marvell International Ltd.
 */

use core::ffi::c_char;

// The original source includes <asm/mach/arch.h>, which supplies the
// DT_MACHINE_START/MACHINE_END declarations and machine descriptor type.

static BERLIN_DT_COMPAT: [*const c_char; 2] = [
    c"marvell,berlin".as_ptr(),
    core::ptr::null(),
];

/*
 * DT_MACHINE_START(BERLIN_DT, "Marvell Berlin")
 *     .dt_compat     = berlin_dt_compat,
 *     /*
 *      * with DT probing for L2CCs, berlin_init_machine can be removed.
 *      * Note: 88DE3005 (Armada 1500-mini) uses pl310 l2cc
 *      */
 *     .l2c_aux_val   = 0x30c00000,
 *     .l2c_aux_mask  = 0xfeffffff,
 * MACHINE_END
 */

// Values emitted by the architecture machine-descriptor macro above.
#[allow(dead_code)]
static BERLIN_DT_L2C_AUX_VAL: u32 = 0x30c00000;
#[allow(dead_code)]
static BERLIN_DT_L2C_AUX_MASK: u32 = 0xfeffffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
