/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *      Ben Dooks <ben@simtec.co.uk>
 *      http://armlinux.simtec.co.uk/
 *
 * S3C64XX - GPIO memory port register definitions
 */

// The header guard is intentionally omitted; Rust module loading provides
// equivalent protection.  S3C64XX_GPIOREG is supplied by the GPIO register
// definitions and is preserved as a macro dependency.

macro_rules! S3C64XX_MEM0CONSTOP {
    () => { S3C64XX_GPIOREG!(0x1B0) };
}

macro_rules! S3C64XX_MEM1CONSTOP {
    () => { S3C64XX_GPIOREG!(0x1B4) };
}

macro_rules! S3C64XX_MEM0CONSLP0 {
    () => { S3C64XX_GPIOREG!(0x1C0) };
}

macro_rules! S3C64XX_MEM0CONSLP1 {
    () => { S3C64XX_GPIOREG!(0x1C4) };
}

macro_rules! S3C64XX_MEM1CONSLP {
    () => { S3C64XX_GPIOREG!(0x1C8) };
}

macro_rules! S3C64XX_MEM0DRVCON {
    () => { S3C64XX_GPIOREG!(0x1D0) };
}

macro_rules! S3C64XX_MEM1DRVCON {
    () => { S3C64XX_GPIOREG!(0x1D4) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
