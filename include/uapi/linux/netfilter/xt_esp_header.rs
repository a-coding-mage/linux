/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
pub struct xt_esp {
    pub spis: [u32; 2], /* Security Parameter Index */
    pub invflags: u8,   /* Inverse flags */
}

/* Values for "invflags" field in struct xt_esp. */
pub const XT_ESP_INV_SPI: u8 = 0x01;  /* Invert the sense of spi. */
pub const XT_ESP_INV_MASK: u8 = 0x01; /* All possible flags. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
