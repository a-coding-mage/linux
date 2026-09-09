/* SPDX-License-Identifier: GPL-2.0 */

// The original declarations are active only when assembling C-SKY code
// (__ASSEMBLER__).  They are represented here as Rust macros producing the
// corresponding assembly text.

#[cfg(__CK860__)]
macro_rules! LABLE_ALIGN {
    () => {
        ".balignw 16, 0x6c03"
    };
}

#[cfg(__CK860__)]
macro_rules! PRE_BNEZAD {
    ($r:expr) => {};
}

#[cfg(__CK860__)]
macro_rules! BNEZAD {
    ($r:expr, $l:expr) => {
        concat!("bnezad ", stringify!($r), ", ", stringify!($l))
    };
}

#[cfg(not(__CK860__))]
macro_rules! LABLE_ALIGN {
    () => {
        ".balignw 8, 0x6c03"
    };
}

#[cfg(not(__CK860__))]
macro_rules! PRE_BNEZAD {
    ($r:expr) => {
        concat!("subi ", stringify!($r), ", 1")
    };
}

#[cfg(not(__CK860__))]
macro_rules! BNEZAD {
    ($r:expr, $l:expr) => {
        concat!("bnez ", stringify!($r), ", ", stringify!($l))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
