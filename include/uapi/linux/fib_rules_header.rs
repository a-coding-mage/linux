/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C dependencies: linux/types.h and linux/rtnetlink.h.

/* rule is permanent, and cannot be deleted */
pub const FIB_RULE_PERMANENT: u32 = 0x00000001;
pub const FIB_RULE_INVERT: u32 = 0x00000002;
pub const FIB_RULE_UNRESOLVED: u32 = 0x00000004;
pub const FIB_RULE_IIF_DETACHED: u32 = 0x00000008;
pub const FIB_RULE_DEV_DETACHED: u32 = FIB_RULE_IIF_DETACHED;
pub const FIB_RULE_OIF_DETACHED: u32 = 0x00000010;

/* try to find source address in routing lookups */
pub const FIB_RULE_FIND_SADDR: u32 = 0x00010000;

#[repr(C)]
pub struct fib_rule_hdr {
    pub family: u8,
    pub dst_len: u8,
    pub src_len: u8,
    pub tos: u8,
    pub table: u8,
    pub res1: u8, // reserved
    pub res2: u8, // reserved
    pub action: u8,
    pub flags: u32,
}

#[repr(C)]
pub struct fib_rule_uid_range {
    pub start: u32,
    pub end: u32,
}

#[repr(C)]
pub struct fib_rule_port_range {
    pub start: u16,
    pub end: u16,
}

#[repr(i32)]
pub enum fib_rule_attr {
    FRA_UNSPEC,
    FRA_DST, // destination address
    FRA_SRC, // source address
    FRA_IIFNAME, // interface name
    // C alias: FRA_IFNAME = FRA_IIFNAME
    FRA_GOTO, // target to jump to (FR_ACT_GOTO)
    FRA_UNUSED2,
    FRA_PRIORITY, // priority/preference
    FRA_UNUSED3,
    FRA_UNUSED4,
    FRA_UNUSED5,
    FRA_FWMARK, // mark
    FRA_FLOW, // flow/class id
    FRA_TUN_ID,
    FRA_SUPPRESS_IFGROUP,
    FRA_SUPPRESS_PREFIXLEN,
    FRA_TABLE, // Extended table id
    FRA_FWMASK, // mask for netfilter mark
    FRA_OIFNAME,
    FRA_PAD,
    FRA_L3MDEV, // iif or oif is l3mdev goto its table
    FRA_UID_RANGE, // UID range
    FRA_PROTOCOL, // Originator of the rule
    FRA_IP_PROTO, // ip proto
    FRA_SPORT_RANGE, // sport
    FRA_DPORT_RANGE, // dport
    FRA_DSCP, // dscp
    FRA_FLOWLABEL, // flowlabel
    FRA_FLOWLABEL_MASK, // flowlabel mask
    FRA_SPORT_MASK, // sport mask
    FRA_DPORT_MASK, // dport mask
    FRA_DSCP_MASK, // dscp mask
    __FRA_MAX,
}

pub const FRA_IFNAME: fib_rule_attr = fib_rule_attr::FRA_IIFNAME;
pub const FRA_MAX: i32 = fib_rule_attr::__FRA_MAX as i32 - 1;

#[repr(i32)]
pub enum fib_rule_action {
    FR_ACT_UNSPEC,
    FR_ACT_TO_TBL, // Pass to fixed table
    FR_ACT_GOTO, // Jump to another rule
    FR_ACT_NOP, // No operation
    FR_ACT_RES3,
    FR_ACT_RES4,
    FR_ACT_BLACKHOLE, // Drop without notification
    FR_ACT_UNREACHABLE, // Drop with ENETUNREACH
    FR_ACT_PROHIBIT, // Drop with EACCES
    __FR_ACT_MAX,
}

pub const FR_ACT_MAX: i32 = fib_rule_action::__FR_ACT_MAX as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
