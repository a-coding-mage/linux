/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The original header emits assembler records consumed by the kernel build
 * tooling. Rust's inline assembly is used here to preserve that behavior.
 */

macro_rules! DEFINE {
    ($sym:ident, $val:expr) => {
        core::arch::asm!(
            concat!("\n.ascii \"->", stringify!($sym), " %0 ", stringify!($val), "\""),
            const $val,
        )
    };
}

macro_rules! BLANK {
    () => {
        core::arch::asm!("\n.ascii \"->\"")
    };
}

macro_rules! OFFSET {
    ($sym:ident, $str:ty, $mem:ident) => {
        DEFINE!($sym, core::mem::offset_of!($str, $mem))
    };
}

macro_rules! COMMENT {
    ($x:expr) => {
        core::arch::asm!(concat!("\n.ascii \"->#", $x, "\""))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
