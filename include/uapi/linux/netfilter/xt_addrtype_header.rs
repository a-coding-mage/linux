/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: __u16 and __u32 are supplied by linux/types.h in the C source.

pub const XT_ADDRTYPE_INVERT_SOURCE: u32 = 0x0001;
pub const XT_ADDRTYPE_INVERT_DEST: u32 = 0x0002;
pub const XT_ADDRTYPE_LIMIT_IFACE_IN: u32 = 0x0004;
pub const XT_ADDRTYPE_LIMIT_IFACE_OUT: u32 = 0x0008;

/* rtn_type enum values from rtnetlink.h, but shifted */
pub const XT_ADDRTYPE_UNSPEC: u32 = 1 << 0;
pub const XT_ADDRTYPE_UNICAST: u32 = 1 << 1; /* 1 << RTN_UNICAST */
pub const XT_ADDRTYPE_LOCAL: u32 = 1 << 2; /* 1 << RTN_LOCAL, etc */
pub const XT_ADDRTYPE_BROADCAST: u32 = 1 << 3;
pub const XT_ADDRTYPE_ANYCAST: u32 = 1 << 4;
pub const XT_ADDRTYPE_MULTICAST: u32 = 1 << 5;
pub const XT_ADDRTYPE_BLACKHOLE: u32 = 1 << 6;
pub const XT_ADDRTYPE_UNREACHABLE: u32 = 1 << 7;
pub const XT_ADDRTYPE_PROHIBIT: u32 = 1 << 8;
pub const XT_ADDRTYPE_THROW: u32 = 1 << 9;
pub const XT_ADDRTYPE_NAT: u32 = 1 << 10;
pub const XT_ADDRTYPE_XRESOLVE: u32 = 1 << 11;

#[repr(C)]
pub struct xt_addrtype_info_v1 {
    pub source: u16, /* source-type mask */
    pub dest: u16, /* dest-type mask */
    pub flags: u32,
}

/* revision 0 */
#[repr(C)]
pub struct xt_addrtype_info {
    pub source: u16, /* source-type mask */
    pub dest: u16, /* dest-type mask */
    pub invert_source: u32,
    pub invert_dest: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
