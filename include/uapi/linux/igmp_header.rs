/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Linux NET3: Internet Group Management Protocol [IGMP]
 *
 * Translated from the corresponding UAPI C header.
 */

/* C dependencies: <linux/types.h> and <asm/byteorder.h>. */

/* IGMP protocol structures. */

/* Header in on-cable format. */
#[repr(C)]
pub struct igmphdr {
    pub type_: u8,
    pub code: u8,
    pub csum: u16,
    pub group: u32,
}

/* V3 group record types [grec_type]. */
pub const IGMPV3_MODE_IS_INCLUDE: u32 = 1;
pub const IGMPV3_MODE_IS_EXCLUDE: u32 = 2;
pub const IGMPV3_CHANGE_TO_INCLUDE: u32 = 3;
pub const IGMPV3_CHANGE_TO_EXCLUDE: u32 = 4;
pub const IGMPV3_ALLOW_NEW_SOURCES: u32 = 5;
pub const IGMPV3_BLOCK_OLD_SOURCES: u32 = 6;

#[repr(C)]
pub struct igmpv3_grec {
    pub grec_type: u8,
    pub grec_auxwords: u8,
    pub grec_nsrcs: u16,
    pub grec_mca: u32,
    pub grec_src: [u32; 0],
}

#[repr(C)]
pub struct igmpv3_report {
    pub type_: u8,
    pub resv1: u8,
    pub csum: u16,
    pub resv2: u16,
    pub ngrec: u16,
    pub grec: [igmpv3_grec; 0],
}

#[repr(C)]
pub struct igmpv3_query {
    pub type_: u8,
    pub code: u8,
    pub csum: u16,
    pub group: u32,
    /* C bit-fields: qrv:3, suppress:1, resv:4 (or reverse order by endian). */
    pub qrv_suppress_resv: u8,
    pub qqic: u8,
    pub nsrcs: u16,
    pub srcs: [u32; 0],
}

pub const IGMP_HOST_MEMBERSHIP_QUERY: u32 = 0x11;
pub const IGMP_HOST_MEMBERSHIP_REPORT: u32 = 0x12;
pub const IGMP_DVMRP: u32 = 0x13;
pub const IGMP_PIM: u32 = 0x14;
pub const IGMP_TRACE: u32 = 0x15;
pub const IGMPV2_HOST_MEMBERSHIP_REPORT: u32 = 0x16;
pub const IGMP_HOST_LEAVE_MESSAGE: u32 = 0x17;
pub const IGMPV3_HOST_MEMBERSHIP_REPORT: u32 = 0x22;

pub const IGMP_MTRACE_RESP: u32 = 0x1e;
pub const IGMP_MTRACE: u32 = 0x1f;
pub const IGMP_MRDISC_ADV: u32 = 0x30;

/* BSD compatibility names. */
pub const IGMP_DELAYING_MEMBER: u32 = 0x01;
pub const IGMP_IDLE_MEMBER: u32 = 0x02;
pub const IGMP_LAZY_MEMBER: u32 = 0x03;
pub const IGMP_SLEEPING_MEMBER: u32 = 0x04;
pub const IGMP_AWAKENING_MEMBER: u32 = 0x05;

pub const IGMP_MINLEN: u32 = 8;
pub const IGMP_MAX_HOST_REPORT_DELAY: u32 = 10;
pub const IGMP_TIMER_SCALE: u32 = 10;
pub const IGMP_AGE_THRESHOLD: u32 = 400;

/* htonl constants, represented in network byte order. */
pub const IGMP_ALL_HOSTS: u32 = 0xE0000001u32.to_be();
pub const IGMP_ALL_ROUTER: u32 = 0xE0000002u32.to_be();
pub const IGMPV3_ALL_MCR: u32 = 0xE0000016u32.to_be();
pub const IGMP_LOCAL_GROUP: u32 = 0xE0000000u32.to_be();
pub const IGMP_LOCAL_GROUP_MASK: u32 = 0xFFFFFF00u32.to_be();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
