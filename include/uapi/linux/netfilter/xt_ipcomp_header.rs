/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency corresponding to <linux/types.h> is supplied externally.

#[repr(C)]
pub struct xt_ipcomp {
    pub spis: [__u32; 2], // Security Parameter Index
    pub invflags: __u8,   // Inverse flags
    pub hdrres: __u8,     // Test of the Reserved Filed
}

// Values for "invflags" field in struct xt_ipcomp.
pub const XT_IPCOMP_INV_SPI: __u8 = 0x01; // Invert the sense of spi.
pub const XT_IPCOMP_INV_MASK: __u8 = 0x01; // All possible flags.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
