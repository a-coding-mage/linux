/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Rust translation of the UAPI ip_tables.h header.
 * External types and constants are supplied by the corresponding headers.
 */

/* The non-kernel compatibility aliases map iptables names to x_tables names. */
pub const IPT_FUNCTION_MAXNAMELEN: usize = XT_FUNCTION_MAXNAMELEN;
pub const IPT_TABLE_MAXNAMELEN: usize = XT_TABLE_MAXNAMELEN;
pub const IPT_CONTINUE: i32 = XT_CONTINUE;
pub const IPT_RETURN: i32 = XT_RETURN;
pub const IPT_TCP_INV_SRCPT: u8 = XT_TCP_INV_SRCPT;
pub const IPT_TCP_INV_DSTPT: u8 = XT_TCP_INV_DSTPT;
pub const IPT_TCP_INV_FLAGS: u8 = XT_TCP_INV_FLAGS;
pub const IPT_TCP_INV_OPTION: u8 = XT_TCP_INV_OPTION;
pub const IPT_TCP_INV_MASK: u8 = XT_TCP_INV_MASK;
pub const IPT_UDP_INV_SRCPT: u8 = XT_UDP_INV_SRCPT;
pub const IPT_UDP_INV_DSTPT: u8 = XT_UDP_INV_DSTPT;
pub const IPT_UDP_INV_MASK: u8 = XT_UDP_INV_MASK;
pub const IPT_STANDARD_TARGET: &str = XT_STANDARD_TARGET;
pub const IPT_ERROR_TARGET: &str = XT_ERROR_TARGET;

#[repr(C)]
pub struct ipt_ip {
    pub src: in_addr,
    pub dst: in_addr,
    pub smsk: in_addr,
    pub dmsk: in_addr,
    pub iniface: [std::ffi::c_char; IFNAMSIZ],
    pub outiface: [std::ffi::c_char; IFNAMSIZ],
    pub iniface_mask: [u8; IFNAMSIZ],
    pub outiface_mask: [u8; IFNAMSIZ],
    pub proto: u16,
    pub flags: u8,
    pub invflags: u8,
}

pub const IPT_F_FRAG: u8 = 0x01;
pub const IPT_F_GOTO: u8 = 0x02;
pub const IPT_F_MASK: u8 = 0x03;
pub const IPT_INV_VIA_IN: u8 = 0x01;
pub const IPT_INV_VIA_OUT: u8 = 0x02;
pub const IPT_INV_TOS: u8 = 0x04;
pub const IPT_INV_SRCIP: u8 = 0x08;
pub const IPT_INV_DSTIP: u8 = 0x10;
pub const IPT_INV_FRAG: u8 = 0x20;
pub const IPT_INV_PROTO: u8 = XT_INV_PROTO;
pub const IPT_INV_MASK: u8 = 0x7F;

#[repr(C)]
pub struct ipt_entry {
    pub ip: ipt_ip,
    pub nfcache: std::ffi::c_uint,
    pub target_offset: u16,
    pub next_offset: u16,
    pub comefrom: std::ffi::c_uint,
    pub counters: xt_counters,
    pub elems: [u8; 0],
}

pub const IPT_BASE_CTL: u32 = 64;
pub const IPT_SO_SET_REPLACE: u32 = IPT_BASE_CTL;
pub const IPT_SO_SET_ADD_COUNTERS: u32 = IPT_BASE_CTL + 1;
pub const IPT_SO_SET_MAX: u32 = IPT_SO_SET_ADD_COUNTERS;
pub const IPT_SO_GET_INFO: u32 = IPT_BASE_CTL;
pub const IPT_SO_GET_ENTRIES: u32 = IPT_BASE_CTL + 1;
pub const IPT_SO_GET_REVISION_MATCH: u32 = IPT_BASE_CTL + 2;
pub const IPT_SO_GET_REVISION_TARGET: u32 = IPT_BASE_CTL + 3;
pub const IPT_SO_GET_MAX: u32 = IPT_SO_GET_REVISION_TARGET;

#[repr(C)]
pub struct ipt_icmp {
    pub type_: u8,
    pub code: [u8; 2],
    pub invflags: u8,
}

pub const IPT_ICMP_INV: u8 = 0x01;

#[repr(C)]
pub struct ipt_getinfo {
    pub name: [std::ffi::c_char; XT_TABLE_MAXNAMELEN],
    pub valid_hooks: std::ffi::c_uint,
    pub hook_entry: [std::ffi::c_uint; NF_INET_NUMHOOKS],
    pub underflow: [std::ffi::c_uint; NF_INET_NUMHOOKS],
    pub num_entries: std::ffi::c_uint,
    pub size: std::ffi::c_uint,
}

#[repr(C)]
pub struct ipt_replace {
    pub name: [std::ffi::c_char; XT_TABLE_MAXNAMELEN],
    pub valid_hooks: std::ffi::c_uint,
    pub num_entries: std::ffi::c_uint,
    pub size: std::ffi::c_uint,
    pub hook_entry: [std::ffi::c_uint; NF_INET_NUMHOOKS],
    pub underflow: [std::ffi::c_uint; NF_INET_NUMHOOKS],
    pub num_counters: std::ffi::c_uint,
    pub counters: *mut xt_counters,
    pub entries: [ipt_entry; 0],
}

#[repr(C)]
pub struct ipt_get_entries {
    pub name: [std::ffi::c_char; XT_TABLE_MAXNAMELEN],
    pub size: std::ffi::c_uint,
    pub entrytable: [ipt_entry; 0],
}

#[inline]
pub unsafe fn ipt_get_target(e: *mut ipt_entry) -> *mut xt_entry_target {
    (e as *mut u8).add((*e).target_offset as usize) as *mut xt_entry_target
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
