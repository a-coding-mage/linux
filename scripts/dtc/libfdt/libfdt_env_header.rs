/* SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause) */
/*
 * libfdt - Flat Device Tree manipulation
 * Copyright (C) 2006 David Gibson, IBM Corporation.
 * Copyright 2012 Kim Phillips, Freescale Semiconductor.
 */

pub type fdt16_t = u16;
pub type fdt32_t = u32;
pub type fdt64_t = u64;

macro_rules! EXTRACT_BYTE {
    ($x:expr, $n:expr) => {{
        let value = $x;
        unsafe { (*((&value as *const _ as *const u8).add($n))) as u64 }
    }};
}

macro_rules! CPU_TO_FDT16 {
    ($x:expr) => {
        (EXTRACT_BYTE!($x, 0) << 8) | EXTRACT_BYTE!($x, 1)
    };
}

macro_rules! CPU_TO_FDT32 {
    ($x:expr) => {
        (EXTRACT_BYTE!($x, 0) << 24)
            | (EXTRACT_BYTE!($x, 1) << 16)
            | (EXTRACT_BYTE!($x, 2) << 8)
            | EXTRACT_BYTE!($x, 3)
    };
}

macro_rules! CPU_TO_FDT64 {
    ($x:expr) => {
        (EXTRACT_BYTE!($x, 0) << 56)
            | (EXTRACT_BYTE!($x, 1) << 48)
            | (EXTRACT_BYTE!($x, 2) << 40)
            | (EXTRACT_BYTE!($x, 3) << 32)
            | (EXTRACT_BYTE!($x, 4) << 24)
            | (EXTRACT_BYTE!($x, 5) << 16)
            | (EXTRACT_BYTE!($x, 6) << 8)
            | EXTRACT_BYTE!($x, 7)
    };
}

#[inline]
pub fn fdt16_to_cpu(x: fdt16_t) -> u16 {
    CPU_TO_FDT16!(x) as u16
}

#[inline]
pub fn cpu_to_fdt16(x: u16) -> fdt16_t {
    CPU_TO_FDT16!(x) as fdt16_t
}

#[inline]
pub fn fdt32_to_cpu(x: fdt32_t) -> u32 {
    CPU_TO_FDT32!(x) as u32
}

#[inline]
pub fn cpu_to_fdt32(x: u32) -> fdt32_t {
    CPU_TO_FDT32!(x) as fdt32_t
}

#[inline]
pub fn fdt64_to_cpu(x: fdt64_t) -> u64 {
    CPU_TO_FDT64!(x) as u64
}

#[inline]
pub fn cpu_to_fdt64(x: u64) -> fdt64_t {
    CPU_TO_FDT64!(x) as fdt64_t
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
