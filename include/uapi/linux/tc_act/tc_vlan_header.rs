/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (c) 2014 Jiri Pirko <jiri@resnulli.us>
 */

// Dependency supplied by <linux/pkt_cls.h>.

pub const TCA_VLAN_ACT_POP: i32 = 1;
pub const TCA_VLAN_ACT_PUSH: i32 = 2;
pub const TCA_VLAN_ACT_MODIFY: i32 = 3;
pub const TCA_VLAN_ACT_POP_ETH: i32 = 4;
pub const TCA_VLAN_ACT_PUSH_ETH: i32 = 5;

#[repr(C)]
pub struct tc_vlan {
    pub tc_gen: tc_gen,
    pub v_action: i32,
}

#[repr(i32)]
pub enum tc_vlan_attr {
    TCA_VLAN_UNSPEC = 0,
    TCA_VLAN_TM = 1,
    TCA_VLAN_PARMS = 2,
    TCA_VLAN_PUSH_VLAN_ID = 3,
    TCA_VLAN_PUSH_VLAN_PROTOCOL = 4,
    TCA_VLAN_PAD = 5,
    TCA_VLAN_PUSH_VLAN_PRIORITY = 6,
    TCA_VLAN_PUSH_ETH_DST = 7,
    TCA_VLAN_PUSH_ETH_SRC = 8,
    __TCA_VLAN_MAX = 9,
}

pub const TCA_VLAN_MAX: i32 = (__TCA_VLAN_MAX as i32) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
