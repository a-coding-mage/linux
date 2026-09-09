/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2008 Simtec Electronics
 *      Ben Dooks <ben@simtec.co.uk>
 *      http://armlinux.simtec.co.uk/
 *
 * S3C - IRQ detection types.
 */

/* values for S3C2410_EXTINT0/1/2 and other cpus in the series, including
 * the S3C64XX
 */
pub const S3C2410_EXTINT_LOWLEV: u32 = 0x00;
pub const S3C2410_EXTINT_HILEV: u32 = 0x01;
pub const S3C2410_EXTINT_FALLEDGE: u32 = 0x02;
pub const S3C2410_EXTINT_RISEEDGE: u32 = 0x04;
pub const S3C2410_EXTINT_BOTHEDGE: u32 = 0x06;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
