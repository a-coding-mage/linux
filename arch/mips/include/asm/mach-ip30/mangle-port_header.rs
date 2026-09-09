/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2003, 2004 Ralf Baechle
 */

/* __swizzle_addr_b(port) */
#[macro_export]
macro_rules! __swizzle_addr_b {
    ($port:expr) => {
        ($port) ^ 3
    };
}

/* __swizzle_addr_w(port) */
#[macro_export]
macro_rules! __swizzle_addr_w {
    ($port:expr) => {
        ($port) ^ 2
    };
}

/* __swizzle_addr_l(port) */
#[macro_export]
macro_rules! __swizzle_addr_l {
    ($port:expr) => {
        $port
    };
}

/* __swizzle_addr_q(port) */
#[macro_export]
macro_rules! __swizzle_addr_q {
    ($port:expr) => {
        $port
    };
}

/* ioswabb(a, x) */
#[macro_export]
macro_rules! ioswabb {
    ($a:expr, $x:expr) => {
        $x
    };
}

/* __mem_ioswabb(a, x) */
#[macro_export]
macro_rules! __mem_ioswabb {
    ($a:expr, $x:expr) => {
        $x
    };
}

/* ioswabw(a, x) */
#[macro_export]
macro_rules! ioswabw {
    ($a:expr, $x:expr) => {
        $x
    };
}

/* __mem_ioswabw(a, x) */
#[macro_export]
macro_rules! __mem_ioswabw {
    ($a:expr, $x:expr) => {
        cpu_to_le16($x) as u16
    };
}

/* ioswabl(a, x) */
#[macro_export]
macro_rules! ioswabl {
    ($a:expr, $x:expr) => {
        $x
    };
}

/* __mem_ioswabl(a, x) */
#[macro_export]
macro_rules! __mem_ioswabl {
    ($a:expr, $x:expr) => {
        cpu_to_le32($x) as u32
    };
}

/* ioswabq(a, x) */
#[macro_export]
macro_rules! ioswabq {
    ($a:expr, $x:expr) => {
        $x
    };
}

/* __mem_ioswabq(a, x) */
#[macro_export]
macro_rules! __mem_ioswabq {
    ($a:expr, $x:expr) => {
        cpu_to_le64($x) as u64
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
