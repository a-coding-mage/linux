/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C dependencies: <linux/types.h>, <linux/netlink.h>

#[repr(C)]
pub struct netconfmsg {
    pub ncm_family: u8,
}

#[repr(i32)]
pub enum netconf_attribute {
    NETCONFA_UNSPEC = 0,
    NETCONFA_IFINDEX,
    NETCONFA_FORWARDING,
    NETCONFA_RP_FILTER,
    NETCONFA_MC_FORWARDING,
    NETCONFA_PROXY_NEIGH,
    NETCONFA_IGNORE_ROUTES_WITH_LINKDOWN,
    NETCONFA_INPUT,
    NETCONFA_BC_FORWARDING,
    NETCONFA_FORCE_FORWARDING,
    __NETCONFA_MAX,
}

pub const NETCONFA_MAX: i32 = __NETCONFA_MAX as i32 - 1;
pub const NETCONFA_ALL: i32 = -1;

pub const NETCONFA_IFINDEX_ALL: i32 = -1;
pub const NETCONFA_IFINDEX_DEFAULT: i32 = -2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
