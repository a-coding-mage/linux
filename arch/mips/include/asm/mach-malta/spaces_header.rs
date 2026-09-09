/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2014 Imagination Technologies Ltd.
 */

// The C header guard `_ASM_MALTA_SPACES_H` is omitted in Rust.

// CONFIG_EVA is a build-time C configuration condition.  The Rust equivalent
// is represented by the `CONFIG_EVA` feature.
#[cfg(feature = "CONFIG_EVA")]
pub const PAGE_OFFSET: usize = 0x0usize;

#[cfg(feature = "CONFIG_EVA")]
pub const PHYS_OFFSET: usize = 0x80000000usize;

#[cfg(feature = "CONFIG_EVA")]
pub const HIGHMEM_START: usize = 0xffff0000usize;

#[cfg(feature = "CONFIG_EVA")]
#[macro_export]
macro_rules! __pa_symbol {
    ($x:expr) => {
        ($x as usize)
    };
}

// Dependency supplied by the generic Malta/platform spaces header:
// #include <asm/mach-generic/spaces.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
