/* SPDX-License-Identifier: GPL-2.0 */
/*
 * altera_uart.h -- Altera UART driver defines.
 */

use ::std::os::raw::{c_uint, c_ulong};

#[repr(C)]
pub struct altera_uart_platform_uart {
    pub mapbase: c_ulong, /* Physical address base */
    pub irq: c_uint,      /* Interrupt vector */
    pub uartclk: c_uint,  /* UART clock rate */
    pub bus_shift: c_uint, /* Bus shift (address stride) */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
