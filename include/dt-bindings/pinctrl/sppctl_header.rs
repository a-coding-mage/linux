/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Sunplus dt-bindings Pinctrl header file
 * Copyright (C) Sunplus Tech / Tibbo Tech.
 * Author: Dvorkin Dmitry <dvorkin@tibbo.com>
 */

pub const IOP_G_MASTE: u32 = 0x01 << 0;
pub const IOP_G_FIRST: u32 = 0x01 << 1;

pub const SPPCTL_PCTL_G_PMUX: u32 = 0x00 | IOP_G_MASTE;
pub const SPPCTL_PCTL_G_GPIO: u32 = IOP_G_FIRST | IOP_G_MASTE;
pub const SPPCTL_PCTL_G_IOPP: u32 = IOP_G_FIRST | 0x00;

pub const SPPCTL_PCTL_L_OUT: u32 = 0x01 << 0; /* Output LOW        */
pub const SPPCTL_PCTL_L_OU1: u32 = 0x01 << 1; /* Output HIGH       */
pub const SPPCTL_PCTL_L_INV: u32 = 0x01 << 2; /* Input Invert      */
pub const SPPCTL_PCTL_L_ONV: u32 = 0x01 << 3; /* Output Invert     */
pub const SPPCTL_PCTL_L_ODR: u32 = 0x01 << 4; /* Output Open Drain */

/*
 * pack into 32-bit value:
 * pin# (8bit), typ (8bit), function (8bit), flag (8bit)
 */
#[inline]
pub const fn SPPCTL_IOPAD(pin: u32, typ: u32, fun: u32, flg: u32) -> u32 {
    (pin << 24) | (typ << 16) | (fun << 8) | flg
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
