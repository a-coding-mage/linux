// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-omap1/opp_data.c
 *
 *  Copyright (C) 2004 - 2005 Nokia corporation
 *  Written by Tuukka Tikkanen <tuukka.tikkanen@elektrobit.com>
 *  Based on clocks.h by Tony Lindgren, Gordon McNutt and RidgeRun, Inc
 */

// Dependencies supplied by the corresponding clock and OPP declarations.

/*-------------------------------------------------------------------------
 * Omap1 MPU rate table
 *-------------------------------------------------------------------------*/
pub static mut omap1_rate_table: [mpu_rate; 15] = [
    /* MPU MHz, xtal MHz, dpll1 MHz, CKCTL, DPLL_CTL
     * NOTE: Comment order here is different from bits in CKCTL value:
     * armdiv, dspdiv, dspmmu, tcdiv, perdiv, lcddiv
     */
    mpu_rate { mpu_rate: 216000000, xtal_rate: 12000000, dpll1_rate: 216000000, ckctl_val: 0x050d, dpllctl_val: 0x2910, flags: CK_1710 },
    mpu_rate { mpu_rate: 195000000, xtal_rate: 13000000, dpll1_rate: 195000000, ckctl_val: 0x050e, dpllctl_val: 0x2790, flags: CK_7XX },
    mpu_rate { mpu_rate: 192000000, xtal_rate: 19200000, dpll1_rate: 192000000, ckctl_val: 0x050f, dpllctl_val: 0x2510, flags: CK_16XX },
    mpu_rate { mpu_rate: 192000000, xtal_rate: 12000000, dpll1_rate: 192000000, ckctl_val: 0x050f, dpllctl_val: 0x2810, flags: CK_16XX },
    mpu_rate { mpu_rate: 96000000, xtal_rate: 12000000, dpll1_rate: 192000000, ckctl_val: 0x055f, dpllctl_val: 0x2810, flags: CK_16XX },
    mpu_rate { mpu_rate: 48000000, xtal_rate: 12000000, dpll1_rate: 192000000, ckctl_val: 0x0baf, dpllctl_val: 0x2810, flags: CK_16XX },
    mpu_rate { mpu_rate: 24000000, xtal_rate: 12000000, dpll1_rate: 192000000, ckctl_val: 0x0fff, dpllctl_val: 0x2810, flags: CK_16XX },
    mpu_rate { mpu_rate: 182000000, xtal_rate: 13000000, dpll1_rate: 182000000, ckctl_val: 0x050e, dpllctl_val: 0x2710, flags: CK_7XX },
    mpu_rate { mpu_rate: 168000000, xtal_rate: 12000000, dpll1_rate: 168000000, ckctl_val: 0x010f, dpllctl_val: 0x2710, flags: CK_16XX | CK_7XX },
    mpu_rate { mpu_rate: 150000000, xtal_rate: 12000000, dpll1_rate: 150000000, ckctl_val: 0x010a, dpllctl_val: 0x2cb0, flags: CK_1510 },
    mpu_rate { mpu_rate: 120000000, xtal_rate: 12000000, dpll1_rate: 120000000, ckctl_val: 0x010a, dpllctl_val: 0x2510, flags: CK_16XX | CK_1510 | CK_310 | CK_7XX },
    mpu_rate { mpu_rate: 96000000, xtal_rate: 12000000, dpll1_rate: 96000000, ckctl_val: 0x0005, dpllctl_val: 0x2410, flags: CK_16XX | CK_1510 | CK_310 | CK_7XX },
    mpu_rate { mpu_rate: 60000000, xtal_rate: 12000000, dpll1_rate: 60000000, ckctl_val: 0x0005, dpllctl_val: 0x2290, flags: CK_16XX | CK_1510 | CK_310 | CK_7XX },
    mpu_rate { mpu_rate: 30000000, xtal_rate: 12000000, dpll1_rate: 60000000, ckctl_val: 0x0555, dpllctl_val: 0x2290, flags: CK_16XX | CK_1510 | CK_310 | CK_7XX },
    mpu_rate { mpu_rate: 0, xtal_rate: 0, dpll1_rate: 0, ckctl_val: 0, dpllctl_val: 0, flags: 0 },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
