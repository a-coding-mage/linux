/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: linux/pkt_cls.h and linux/types.h provide `tc_gen`, `__be32`,
// and `__u32` in the surrounding translation unit.

#[repr(i32)]
pub enum TcNatAttribute {
    TCA_NAT_UNSPEC = 0,
    TCA_NAT_PARMS,
    TCA_NAT_TM,
    TCA_NAT_PAD,
    __TCA_NAT_MAX,
}

pub const TCA_NAT_MAX: i32 = TcNatAttribute::__TCA_NAT_MAX as i32 - 1;

pub const TCA_NAT_FLAG_EGRESS: u32 = 1;

#[repr(C)]
pub struct tc_nat {
    pub tc_gen: tc_gen,
    pub old_addr: __be32,
    pub new_addr: __be32,
    pub mask: __be32,
    pub flags: __u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
