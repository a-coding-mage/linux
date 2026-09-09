/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from bcm63xx_dev_pcmcia.h. */

/*
 * PCMCIA driver platform data
 */
#[repr(C)]
pub struct bcm63xx_pcmcia_platform_data {
    pub ready_gpio: ::core::ffi::c_uint,
}

unsafe extern "C" {
    pub fn bcm63xx_pcmcia_register() -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
