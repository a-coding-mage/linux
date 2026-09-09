/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of uapi/linux/netfilter_ipv6/ip6_tables.h. */

/* Includes and build-time __KERNEL__ conditionals are intentionally omitted;
 * their dependent symbols are expected to be supplied by the surrounding ABI. */

use core::ffi::c_char;

pub const IP6T_FUNCTION_MAXNAMELEN: usize = XT_FUNCTION_MAXNAMELEN;
pub const IP6T_TABLE_MAXNAMELEN: usize = XT_TABLE_MAXNAMELEN;
pub const IP6T_CONTINUE: i32 = XT_CONTINUE;
pub const IP6T_RETURN: i32 = XT_RETURN;
pub const IP6T_TCP_INV_SRCPT: u8 = XT_TCP_INV_SRCPT;
pub const IP6T_TCP_INV_DSTPT: u8 = XT_TCP_INV_DSTPT;
pub const IP6T_TCP_INV_FLAGS: u8 = XT_TCP_INV_FLAGS;
pub const IP6T_TCP_INV_OPTION: u8 = XT_TCP_INV_OPTION;
pub const IP6T_TCP_INV_MASK: u8 = XT_TCP_INV_MASK;
pub const IP6T_UDP_INV_SRCPT: u8 = XT_UDP_INV_SRCPT;
pub const IP6T_UDP_INV_DSTPT: u8 = XT_UDP_INV_DSTPT;
pub const IP6T_UDP_INV_MASK: u8 = XT_UDP_INV_MASK;
pub const IP6T_STANDARD_TARGET: &str = XT_STANDARD_TARGET;
pub const IP6T_ERROR_TARGET: &str = XT_ERROR_TARGET;

#[repr(C)]
pub struct ip6t_ip6 {
    pub src: in6_addr,
    pub dst: in6_addr,
    pub smsk: in6_addr,
    pub dmsk: in6_addr,
    pub iniface: [c_char; IFNAMSIZ],
    pub outiface: [c_char; IFNAMSIZ],
    pub iniface_mask: [u8; IFNAMSIZ],
    pub outiface_mask: [u8; IFNAMSIZ],
    pub proto: u16,
    pub tos: u8,
    pub flags: u8,
    pub invflags: u8,
}

pub const IP6T_F_PROTO: u8 = 0x01;
pub const IP6T_F_TOS: u8 = 0x02;
pub const IP6T_F_GOTO: u8 = 0x04;
pub const IP6T_F_MASK: u8 = 0x07;

pub const IP6T_INV_VIA_IN: u8 = 0x01;
pub const IP6T_INV_VIA_OUT: u8 = 0x02;
pub const IP6T_INV_TOS: u8 = 0x04;
pub const IP6T_INV_SRCIP: u8 = 0x08;
pub const IP6T_INV_DSTIP: u8 = 0x10;
pub const IP6T_INV_FRAG: u8 = 0x20;
pub const IP6T_INV_PROTO: u8 = XT_INV_PROTO;
pub const IP6T_INV_MASK: u8 = 0x7f;

#[repr(C)]
pub struct ip6t_entry {
    pub ipv6: ip6t_ip6,
    pub nfcache: u32,
    pub target_offset: u16,
    pub next_offset: u16,
    pub comefrom: u32,
    pub counters: xt_counters,
    pub elems: [u8; 0],
}

#[repr(C)]
pub struct ip6t_standard {
    pub entry: ip6t_entry,
    pub target: xt_standard_target,
}

#[repr(C)]
pub struct ip6t_error {
    pub entry: ip6t_entry,
    pub target: xt_error_target,
}

#[macro_export]
macro_rules! IP6T_ENTRY_INIT {
    ($size:expr) => {
        ip6t_entry {
            target_offset: core::mem::size_of::<ip6t_entry>() as u16,
            next_offset: $size,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

/* XT_TARGET_INIT and the target structures are supplied by x_tables. */
#[macro_export]
macro_rules! IP6T_STANDARD_INIT {
    ($verdict:expr) => {
        ip6t_standard {
            entry: IP6T_ENTRY_INIT!(core::mem::size_of::<ip6t_standard>() as u16),
            target: XT_TARGET_INIT!(XT_STANDARD_TARGET, core::mem::size_of::<xt_standard_target>() as u16),
        }
    };
}

#[macro_export]
macro_rules! IP6T_ERROR_INIT {
    () => {
        ip6t_error {
            entry: IP6T_ENTRY_INIT!(core::mem::size_of::<ip6t_error>() as u16),
            target: XT_TARGET_INIT!(XT_ERROR_TARGET, core::mem::size_of::<xt_error_target>() as u16),
        }
    };
}

pub const IP6T_BASE_CTL: u32 = 64;
pub const IP6T_SO_SET_REPLACE: u32 = IP6T_BASE_CTL;
pub const IP6T_SO_SET_ADD_COUNTERS: u32 = IP6T_BASE_CTL + 1;
pub const IP6T_SO_SET_MAX: u32 = IP6T_SO_SET_ADD_COUNTERS;
pub const IP6T_SO_GET_INFO: u32 = IP6T_BASE_CTL;
pub const IP6T_SO_GET_ENTRIES: u32 = IP6T_BASE_CTL + 1;
pub const IP6T_SO_GET_REVISION_MATCH: u32 = IP6T_BASE_CTL + 4;
pub const IP6T_SO_GET_REVISION_TARGET: u32 = IP6T_BASE_CTL + 5;
pub const IP6T_SO_GET_MAX: u32 = IP6T_SO_GET_REVISION_TARGET;
pub const IP6T_SO_ORIGINAL_DST: u32 = 80;

#[repr(C)]
pub struct ip6t_icmp {
    pub type_: u8,
    pub code: [u8; 2],
    pub invflags: u8,
}

pub const IP6T_ICMP_INV: u8 = 0x01;

#[repr(C)]
pub struct ip6t_getinfo {
    pub name: [c_char; XT_TABLE_MAXNAMELEN],
    pub valid_hooks: u32,
    pub hook_entry: [u32; NF_INET_NUMHOOKS],
    pub underflow: [u32; NF_INET_NUMHOOKS],
    pub num_entries: u32,
    pub size: u32,
}

#[repr(C)]
pub struct ip6t_replace {
    pub name: [c_char; XT_TABLE_MAXNAMELEN],
    pub valid_hooks: u32,
    pub num_entries: u32,
    pub size: u32,
    pub hook_entry: [u32; NF_INET_NUMHOOKS],
    pub underflow: [u32; NF_INET_NUMHOOKS],
    pub num_counters: u32,
    pub counters: *mut xt_counters,
    pub entries: [ip6t_entry; 0],
}

#[repr(C)]
pub struct ip6t_get_entries {
    pub name: [c_char; XT_TABLE_MAXNAMELEN],
    pub size: u32,
    pub entrytable: [ip6t_entry; 0],
}

#[inline]
pub unsafe fn ip6t_get_target(e: *mut ip6t_entry) -> *mut xt_entry_target {
    (e as *mut u8).add((*e).target_offset as usize) as *mut xt_entry_target
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
