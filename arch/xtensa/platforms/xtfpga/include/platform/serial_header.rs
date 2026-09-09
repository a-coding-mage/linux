/*
 * arch/xtensa/platform/xtavnet/include/platform/serial.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001, 2006 Tensilica Inc.
 */

// Dependency supplied by platform/hardware.h.

/// Equivalent to the C macro:
/// `(*(long *)XTFPGA_CLKFRQ_VADDR / 16)`.
#[macro_export]
macro_rules! BASE_BAUD {
    () => {{
        unsafe { *(XTFPGA_CLKFRQ_VADDR as *const core::ffi::c_long) / 16 }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
