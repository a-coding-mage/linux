// SPDX-License-Identifier: GPL-2.0
//
// Dependency intent: declarations formerly supplied by <asm/setup.h> and
// "decompress.h" are provided by the surrounding translation unit.

use core::ffi::c_char;

unsafe extern "C" {
    fn prom_putchar(c: c_char);
}

pub unsafe fn putc(c: c_char) {
    unsafe {
        prom_putchar(c);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
