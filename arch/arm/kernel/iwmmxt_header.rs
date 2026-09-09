/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the assembler definitions from iwmmxt.h.

// Register numbers emitted by the original .irp loop.
pub const LW_R: [u32; 16] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
];
pub const LR: [u32; 16] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
];

pub const LW_CSSF: u32 = 0x2;
pub const LW_CASF: u32 = 0x3;
pub const LW_CGR0: u32 = 0x8;
pub const LW_CGR1: u32 = 0x9;
pub const LW_CGR2: u32 = 0xa;
pub const LW_CGR3: u32 = 0xb;

// The C preprocessor token-pasting/register aliases are represented by
// integer register numbers in these Rust macros. The original .inst directive
// emitted the resulting instruction word into the enclosing assembly stream.
#[macro_export]
macro_rules! wldrd {
    ($reg:expr, $base:expr, $offset:expr) => {
        0xedd00100u32
            | (($reg as u32) << 12)
            | (($base as u32) << 16)
            | (($offset as u32) >> 2)
    };
}

#[macro_export]
macro_rules! wldrw {
    ($reg:expr, $base:expr, $offset:expr) => {
        0xfd900100u32
            | (($reg as u32) << 12)
            | (($base as u32) << 16)
            | (($offset as u32) >> 2)
    };
}

#[macro_export]
macro_rules! wstrd {
    ($reg:expr, $base:expr, $offset:expr) => {
        0xedc00100u32
            | (($reg as u32) << 12)
            | (($base as u32) << 16)
            | (($offset as u32) >> 2)
    };
}

#[macro_export]
macro_rules! wstrw {
    ($reg:expr, $base:expr, $offset:expr) => {
        0xfd800100u32
            | (($reg as u32) << 12)
            | (($base as u32) << 16)
            | (($offset as u32) >> 2)
    };
}

// In the original header these definitions are enabled only for Clang and
// emit ARM coprocessor instructions (mrc/mcr). Rust has no file-local mapping
// for assembler register-token operands, so their exact instruction text is
// preserved here as declarative macros for the target-specific integration.
#[cfg(clang)]
pub const WCON: &str = "c1";

#[cfg(clang)]
#[macro_export]
macro_rules! tmrc {
    ($dest:tt, $control:tt) => {
        core::compile_error!("tmrc! requires target-specific ARM inline assembly: mrc p1, 0, $dest, $control, c0, 0")
    };
}

#[cfg(clang)]
#[macro_export]
macro_rules! tmcr {
    ($control:tt, $src:tt) => {
        core::compile_error!("tmcr! requires target-specific ARM inline assembly: mcr p1, 0, $src, $control, c0, 0")
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
