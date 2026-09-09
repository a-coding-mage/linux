/* SPDX-License-Identifier: GPL-2.0 */
/*
 * altera_jtaguart.h -- Altera JTAG UART driver defines.
 */

use core::ffi::{c_uint, c_ulong};

pub const ALTERA_JTAGUART_MAJOR: c_uint = 204;
pub const ALTERA_JTAGUART_MINOR: c_uint = 186;

#[repr(C)]
pub struct altera_jtaguart_platform_uart {
    /* Physical address base */
    pub mapbase: c_ulong,
    /* Interrupt vector */
    pub irq: c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
