/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/* Copyright (c) 2018 Microsemi Corporation */

// C preprocessor macros translated as Rust macros to preserve call-site behavior.
macro_rules! SERDES1G {
    ($x:expr) => {
        ($x)
    };
}

pub const SERDES1G_MAX: i32 = SERDES1G!(5);

macro_rules! SERDES6G {
    ($x:expr) => {
        (SERDES1G_MAX + 1 + ($x))
    };
}

pub const SERDES6G_MAX: i32 = SERDES6G!(2);
pub const SERDES_MAX: i32 = SERDES6G_MAX + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
