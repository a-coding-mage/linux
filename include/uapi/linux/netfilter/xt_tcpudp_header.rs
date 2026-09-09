/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency equivalent of <linux/types.h> is supplied externally. */

/* TCP matching stuff */
#[repr(C)]
pub struct xt_tcp {
    pub spts: [__u16; 2],    /* Source port range. */
    pub dpts: [__u16; 2],    /* Destination port range. */
    pub option: __u8,        /* TCP Option iff non-zero*/
    pub flg_mask: __u8,      /* TCP flags mask byte */
    pub flg_cmp: __u8,       /* TCP flags compare byte */
    pub invflags: __u8,      /* Inverse flags */
}

/* Values for "inv" field in struct ipt_tcp. */
pub const XT_TCP_INV_SRCPT: u32 = 0x01; /* Invert the sense of source ports. */
pub const XT_TCP_INV_DSTPT: u32 = 0x02; /* Invert the sense of dest ports. */
pub const XT_TCP_INV_FLAGS: u32 = 0x04; /* Invert the sense of TCP flags. */
pub const XT_TCP_INV_OPTION: u32 = 0x08; /* Invert the sense of option test. */
pub const XT_TCP_INV_MASK: u32 = 0x0F; /* All possible flags. */

/* UDP matching stuff */
#[repr(C)]
pub struct xt_udp {
    pub spts: [__u16; 2],    /* Source port range. */
    pub dpts: [__u16; 2],    /* Destination port range. */
    pub invflags: __u8,       /* Inverse flags */
}

/* Values for "invflags" field in struct ipt_udp. */
pub const XT_UDP_INV_SRCPT: u32 = 0x01; /* Invert the sense of source ports. */
pub const XT_UDP_INV_DSTPT: u32 = 0x02; /* Invert the sense of dest ports. */
pub const XT_UDP_INV_MASK: u32 = 0x03; /* All possible flags. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
