/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright (C) 2019 Netronome Systems, Inc. */

// Dependency equivalent of: #include <linux/pkt_cls.h>

pub const TCA_MPLS_ACT_POP: i32 = 1;
pub const TCA_MPLS_ACT_PUSH: i32 = 2;
pub const TCA_MPLS_ACT_MODIFY: i32 = 3;
pub const TCA_MPLS_ACT_DEC_TTL: i32 = 4;
pub const TCA_MPLS_ACT_MAC_PUSH: i32 = 5;

#[repr(C)]
pub struct tc_mpls {
    // generic TC action fields.
    pub tc_gen: tc_gen,
    // action of type TCA_MPLS_ACT_*.
    pub m_action: i32,
}

pub const TCA_MPLS_UNSPEC: i32 = 0;
pub const TCA_MPLS_TM: i32 = 1; // struct tcf_t; time values associated with action.
pub const TCA_MPLS_PARMS: i32 = 2; // struct tc_mpls; action type and general TC fields.
pub const TCA_MPLS_PAD: i32 = 3;
pub const TCA_MPLS_PROTO: i32 = 4; // be16; eth_type of pushed or next (for pop) header.
pub const TCA_MPLS_LABEL: i32 = 5; // u32; MPLS label. Lower 20 bits are used.
pub const TCA_MPLS_TC: i32 = 6; // u8; MPLS TC field. Lower 3 bits are used.
pub const TCA_MPLS_TTL: i32 = 7; // u8; MPLS TTL field. Must not be 0.
pub const TCA_MPLS_BOS: i32 = 8; // u8; MPLS BOS field. Either 1 or 0.
pub const __TCA_MPLS_MAX: i32 = 9;
pub const TCA_MPLS_MAX: i32 = __TCA_MPLS_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
