/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Format of an ARP firewall descriptor.
 * src, tgt, src_mask, tgt_mask, arpop, and arpop_mask are stored in
 * network byte order; flags are stored in host byte order.
 *
 * C header dependencies are intentionally referenced as external Rust items.
 */

pub const ARPT_FUNCTION_MAXNAMELEN: usize = XT_FUNCTION_MAXNAMELEN;
pub const ARPT_TABLE_MAXNAMELEN: usize = XT_TABLE_MAXNAMELEN;
pub const ARPT_CONTINUE: u32 = XT_CONTINUE;
pub const ARPT_RETURN: u32 = XT_RETURN;
pub const ARPT_STANDARD_TARGET: &str = XT_STANDARD_TARGET;
pub const ARPT_ERROR_TARGET: &str = XT_ERROR_TARGET;

pub const ARPT_DEV_ADDR_LEN_MAX: usize = 16;

#[repr(C)]
pub struct arpt_devaddr_info {
    pub addr: [core::ffi::c_char; ARPT_DEV_ADDR_LEN_MAX],
    pub mask: [core::ffi::c_char; ARPT_DEV_ADDR_LEN_MAX],
}

#[repr(C)]
pub struct arpt_arp {
    pub src: in_addr,
    pub tgt: in_addr,
    pub smsk: in_addr,
    pub tmsk: in_addr,
    pub arhln: __u8,
    pub arhln_mask: __u8,
    pub src_devaddr: arpt_devaddr_info,
    pub tgt_devaddr: arpt_devaddr_info,
    pub arpop: __be16,
    pub arpop_mask: __be16,
    pub arhrd: __be16,
    pub arhrd_mask: __be16,
    pub arpro: __be16,
    pub arpro_mask: __be16,
    pub iniface: [core::ffi::c_char; IFNAMSIZ],
    pub outiface: [core::ffi::c_char; IFNAMSIZ],
    pub iniface_mask: [u8; IFNAMSIZ],
    pub outiface_mask: [u8; IFNAMSIZ],
    pub flags: __u8,
    pub invflags: __u16,
}

pub const ARPT_F_MASK: u32 = 0x00;
pub const ARPT_INV_VIA_IN: u32 = 0x0001;
pub const ARPT_INV_VIA_OUT: u32 = 0x0002;
pub const ARPT_INV_SRCIP: u32 = 0x0004;
pub const ARPT_INV_TGTIP: u32 = 0x0008;
pub const ARPT_INV_SRCDEVADDR: u32 = 0x0010;
pub const ARPT_INV_TGTDEVADDR: u32 = 0x0020;
pub const ARPT_INV_ARPOP: u32 = 0x0040;
pub const ARPT_INV_ARPHRD: u32 = 0x0080;
pub const ARPT_INV_ARPPRO: u32 = 0x0100;
pub const ARPT_INV_ARPHLN: u32 = 0x0200;
pub const ARPT_INV_MASK: u32 = 0x03FF;

#[repr(C)]
pub struct arpt_entry {
    pub arp: arpt_arp,
    pub target_offset: __u16,
    pub next_offset: __u16,
    pub comefrom: core::ffi::c_uint,
    pub counters: xt_counters,
    pub elems: [u8; 0],
}

pub const ARPT_BASE_CTL: u32 = 96;
pub const ARPT_SO_SET_REPLACE: u32 = ARPT_BASE_CTL;
pub const ARPT_SO_SET_ADD_COUNTERS: u32 = ARPT_BASE_CTL + 1;
pub const ARPT_SO_SET_MAX: u32 = ARPT_SO_SET_ADD_COUNTERS;
pub const ARPT_SO_GET_INFO: u32 = ARPT_BASE_CTL;
pub const ARPT_SO_GET_ENTRIES: u32 = ARPT_BASE_CTL + 1;
pub const ARPT_SO_GET_REVISION_TARGET: u32 = ARPT_BASE_CTL + 3;
pub const ARPT_SO_GET_MAX: u32 = ARPT_SO_GET_REVISION_TARGET;

#[repr(C)]
pub struct arpt_getinfo {
    pub name: [core::ffi::c_char; XT_TABLE_MAXNAMELEN],
    pub valid_hooks: core::ffi::c_uint,
    pub hook_entry: [core::ffi::c_uint; NF_ARP_NUMHOOKS],
    pub underflow: [core::ffi::c_uint; NF_ARP_NUMHOOKS],
    pub num_entries: core::ffi::c_uint,
    pub size: core::ffi::c_uint,
}

#[repr(C)]
pub struct arpt_replace {
    pub name: [core::ffi::c_char; XT_TABLE_MAXNAMELEN],
    pub valid_hooks: core::ffi::c_uint,
    pub num_entries: core::ffi::c_uint,
    pub size: core::ffi::c_uint,
    pub hook_entry: [core::ffi::c_uint; NF_ARP_NUMHOOKS],
    pub underflow: [core::ffi::c_uint; NF_ARP_NUMHOOKS],
    pub num_counters: core::ffi::c_uint,
    pub counters: *mut xt_counters,
    pub entries: [arpt_entry; 0],
}

#[repr(C)]
pub struct arpt_get_entries {
    pub name: [core::ffi::c_char; XT_TABLE_MAXNAMELEN],
    pub size: core::ffi::c_uint,
    pub entrytable: [arpt_entry; 0],
}

#[inline]
pub unsafe fn arpt_get_target(e: *mut arpt_entry) -> *mut xt_entry_target {
    (e as *mut u8).add((*e).target_offset as usize) as *mut xt_entry_target
}

/* ARPT_ENTRY_ITERATE maps to the external XT_ENTRY_ITERATE macro. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
