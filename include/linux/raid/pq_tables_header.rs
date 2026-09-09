/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2003 H. Peter Anvin - All Rights Reserved
 *
 * Galois field tables for the Linux RAID6 P/Q parity algorithm.
 */

// Dependency equivalent of: #include <linux/types.h>

// The C declarations specify 256-byte alignment via __attribute__((aligned(256))).
unsafe extern "C" {
    pub static raid6_gfmul: [[u8; 256]; 256];
    pub static raid6_vgfmul: [[u8; 32]; 256];
    pub static raid6_gfexp: [u8; 256];
    pub static raid6_gflog: [u8; 256];
    pub static raid6_gfinv: [u8; 256];
    pub static raid6_gfexi: [u8; 256];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
