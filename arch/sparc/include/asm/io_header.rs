/* SPDX-License-Identifier: GPL-2.0 */

// On SPARC64, the C source includes <asm/io_64.h>; otherwise it includes
// <asm/io_32.h>. Those architecture-specific declarations are supplied by
// the surrounding translation unit.

/*
 * Defines used for both SPARC32 and SPARC64
 */

/* Big endian versions of memory read/write routines */
#[macro_export]
macro_rules! readb_be {
    ($addr:expr) => {
        __raw_readb!($addr)
    };
}

#[macro_export]
macro_rules! readw_be {
    ($addr:expr) => {
        __raw_readw!($addr)
    };
}

#[macro_export]
macro_rules! readl_be {
    ($addr:expr) => {
        __raw_readl!($addr)
    };
}

#[macro_export]
macro_rules! writeb_be {
    ($b:expr, $addr:expr) => {
        __raw_writeb!($b, $addr)
    };
}

#[macro_export]
macro_rules! writel_be {
    ($w:expr, $addr:expr) => {
        __raw_writel!($w, $addr)
    };
}

#[macro_export]
macro_rules! writew_be {
    ($l:expr, $addr:expr) => {
        __raw_writew!($l, $addr)
    };
}

// The C source also includes <asm-generic/io.h>; its declarations and macros
// are supplied by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
