/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2021 Microchip Inc.
 *
 * Author: Kavyasree Kotagiri <kavyasree.kotagiri@microchip.com>
 */

// Translated from the C header; the original header guard is omitted.

pub const GCK_ID_QSPI0: u32 = 0;
pub const GCK_ID_QSPI1: u32 = 1;
pub const GCK_ID_QSPI2: u32 = 2;
pub const GCK_ID_SDMMC0: u32 = 3;
pub const GCK_ID_PI: u32 = 4;
pub const GCK_ID_MCAN0: u32 = 5;
pub const GCK_ID_MCAN1: u32 = 6;
pub const GCK_ID_FLEXCOM0: u32 = 7;
pub const GCK_ID_FLEXCOM1: u32 = 8;
pub const GCK_ID_FLEXCOM2: u32 = 9;
pub const GCK_ID_FLEXCOM3: u32 = 10;
pub const GCK_ID_FLEXCOM4: u32 = 11;
pub const GCK_ID_TIMER: u32 = 12;
pub const GCK_ID_USB_REFCLK: u32 = 13;

/* Gate clocks */
pub const GCK_GATE_UHPHS: u32 = 14;
pub const GCK_GATE_UDPHS: u32 = 15;
pub const GCK_GATE_MCRAMC: u32 = 16;
pub const GCK_GATE_HMATRIX: u32 = 17;

pub const N_CLOCKS: u32 = 18;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
