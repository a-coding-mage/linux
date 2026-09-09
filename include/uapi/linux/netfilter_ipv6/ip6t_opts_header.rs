/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: __u32, __u8, and __u16 correspond to Linux UAPI integer types.

pub const IP6T_OPTS_OPTSNR: usize = 16;

#[repr(C)]
pub struct ip6t_opts {
    pub hdrlen: u32,              /* Header Length */
    pub flags: u8,                /*  */
    pub invflags: u8,             /* Inverse flags */
    pub opts: [u16; IP6T_OPTS_OPTSNR], /* opts */
    pub optsnr: u8,               /* Nr of OPts */
}

pub const IP6T_OPTS_LEN: u32 = 0x01;
pub const IP6T_OPTS_OPTS: u32 = 0x02;
pub const IP6T_OPTS_NSTRICT: u32 = 0x04;

/* Values for "invflags" field in struct ip6t_rt. */
pub const IP6T_OPTS_INV_LEN: u32 = 0x01;  /* Invert the sense of length. */
pub const IP6T_OPTS_INV_MASK: u32 = 0x01; /* All possible flags. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
