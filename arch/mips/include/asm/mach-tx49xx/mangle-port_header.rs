/* SPDX-License-Identifier: GPL-2.0 */

// C header guard omitted; this file is intended to be included once.

macro_rules! __swizzle_addr_b {
    ($port:expr) => {
        $port
    };
}

macro_rules! __swizzle_addr_w {
    ($port:expr) => {
        $port
    };
}

macro_rules! __swizzle_addr_l {
    ($port:expr) => {
        $port
    };
}

macro_rules! __swizzle_addr_q {
    ($port:expr) => {
        $port
    };
}

macro_rules! ioswabb {
    ($a:expr, $x:expr) => {
        $x
    };
}

macro_rules! __mem_ioswabb {
    ($a:expr, $x:expr) => {
        $x
    };
}

macro_rules! ioswabw {
    ($a:expr, $x:expr) => {
        le16_to_cpu($x)
    };
}

macro_rules! __mem_ioswabw {
    ($a:expr, $x:expr) => {
        $x
    };
}

macro_rules! ioswabl {
    ($a:expr, $x:expr) => {
        le32_to_cpu($x)
    };
}

macro_rules! __mem_ioswabl {
    ($a:expr, $x:expr) => {
        $x
    };
}

macro_rules! ioswabq {
    ($a:expr, $x:expr) => {
        le64_to_cpu($x)
    };
}

macro_rules! __mem_ioswabq {
    ($a:expr, $x:expr) => {
        $x
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
