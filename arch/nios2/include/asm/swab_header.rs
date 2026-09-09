/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2012 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2011 Pyramid Technical Consultants, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License. See the file COPYING in the main directory of this
 * archive for more details.
 */

// Dependency intent: linux/types.h and asm-generic/swab.h provide the
// corresponding integer types and generic byte-swap declarations.

// CONFIG_NIOS2_CI_SWAB_SUPPORT and the compiler's __GNUC__ condition are
// build-time conditions retained here as Rust cfg conditions.
#[cfg(all(CONFIG_NIOS2_CI_SWAB_SUPPORT, __GNUC__))]
mod nios2_ci_swab {
    // __builtin_custom_ini(CONFIG_NIOS2_CI_SWAB_NO, (x)) is a Nios II
    // compiler builtin; its implementation is supplied externally.
    extern "C" {
        fn __builtin_custom_ini(no: u32, x: u32) -> u32;
    }

    // CONFIG_NIOS2_CI_SWAB_NO is supplied by the build configuration.
    extern "C" {
        static CONFIG_NIOS2_CI_SWAB_NO: u32;
    }

    #[inline]
    pub unsafe fn __nios2_swab(x: u32) -> u32 {
        __builtin_custom_ini(CONFIG_NIOS2_CI_SWAB_NO, x)
    }

    #[inline]
    pub unsafe fn __arch_swab16(x: u16) -> u16 {
        __nios2_swab((x as u32) << 16) as u16
    }

    #[inline]
    pub unsafe fn __arch_swab32(x: u32) -> u32 {
        __nios2_swab(x)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
