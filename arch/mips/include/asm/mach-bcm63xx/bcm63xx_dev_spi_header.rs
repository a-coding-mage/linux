/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux and BCM63xx headers.

unsafe extern "C" {
    pub fn bcm63xx_spi_register() -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
