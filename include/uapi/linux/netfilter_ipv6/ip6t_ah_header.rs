/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
pub struct ip6t_ah {
    pub spis: [u32; 2], // Security Parameter Index
    pub hdrlen: u32,    // Header Length
    pub hdrres: u8,     // Test of the Reserved Filed
    pub invflags: u8,   // Inverse flags
}

pub const IP6T_AH_SPI: u32 = 0x01;
pub const IP6T_AH_LEN: u32 = 0x02;
pub const IP6T_AH_RES: u32 = 0x04;

// Values for "invflags" field in struct ip6t_ah.
pub const IP6T_AH_INV_SPI: u32 = 0x01; // Invert the sense of spi.
pub const IP6T_AH_INV_LEN: u32 = 0x02; // Invert the sense of length.
pub const IP6T_AH_INV_MASK: u32 = 0x03; // All possible flags.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
