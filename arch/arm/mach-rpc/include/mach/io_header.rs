/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/mach-rpc/include/mach/io.h
 *
 *  Copyright (C) 1997 Russell King
 *
 * Modifications:
 *  06-Dec-1997 RMK Created.
 */

// Dependency supplied by mach/hardware.h in the original source.

pub const IO_SPACE_LIMIT: u32 = 0xffff;

/*
 * We need PC style IO addressing for:
 *  - floppy (at 0x3f2,0x3f4,0x3f5,0x3f7)
 *  - parport (at 0x278-0x27a, 0x27b-0x27f, 0x778-0x77a)
 *  - 8250 serial (only for compile)
 *
 * These peripherals are found in an area of MMIO which looks very much
 * like an ISA bus, but with registers at the low byte of each word.
 */
#[macro_export]
macro_rules! __io {
    ($a:expr) => {
        (PCIO_BASE + (($a) << 2))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
