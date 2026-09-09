/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* MH matching stuff */
#[repr(C)]
pub struct ip6t_mh {
    pub types: [u8; 2], /* MH type range */
    pub invflags: u8,   /* Inverse flags */
}

/* Values for "invflags" field in struct ip6t_mh. */
pub const IP6T_MH_INV_TYPE: u8 = 0x01; /* Invert the sense of type. */
pub const IP6T_MH_INV_MASK: u8 = 0x01; /* All possible flags. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
