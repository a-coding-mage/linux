/* SPDX-License-Identifier: GPL-2.0 */

// Translated from bcm63xx_dev_uart.h. The original header guard is omitted
// because Rust modules provide equivalent declaration scoping.

extern "C" {
    pub fn bcm63xx_uart_register(id: core::ffi::c_uint) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
