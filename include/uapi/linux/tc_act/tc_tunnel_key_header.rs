/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (c) 2016, Amir Vadai <amir@vadai.me>
 * Copyright (c) 2016, Mellanox Technologies. All rights reserved.
 */

// Dependency supplied by <linux/pkt_cls.h>.

pub const TCA_TUNNEL_KEY_ACT_SET: u32 = 1;
pub const TCA_TUNNEL_KEY_ACT_RELEASE: u32 = 2;

#[repr(C)]
pub struct tc_tunnel_key {
    pub tc_gen: tc_gen,
    pub t_action: ::core::ffi::c_int,
}

pub const TCA_TUNNEL_KEY_UNSPEC: u32 = 0;
pub const TCA_TUNNEL_KEY_TM: u32 = 1;
pub const TCA_TUNNEL_KEY_PARMS: u32 = 2;
pub const TCA_TUNNEL_KEY_ENC_IPV4_SRC: u32 = 3; // be32
pub const TCA_TUNNEL_KEY_ENC_IPV4_DST: u32 = 4; // be32
pub const TCA_TUNNEL_KEY_ENC_IPV6_SRC: u32 = 5; // struct in6_addr
pub const TCA_TUNNEL_KEY_ENC_IPV6_DST: u32 = 6; // struct in6_addr
pub const TCA_TUNNEL_KEY_ENC_KEY_ID: u32 = 7; // be64
pub const TCA_TUNNEL_KEY_PAD: u32 = 8;
pub const TCA_TUNNEL_KEY_ENC_DST_PORT: u32 = 9; // be16
pub const TCA_TUNNEL_KEY_NO_CSUM: u32 = 10; // u8
pub const TCA_TUNNEL_KEY_ENC_OPTS: u32 = 11; // Nested TCA_TUNNEL_KEY_ENC_OPTS_ attributes
pub const TCA_TUNNEL_KEY_ENC_TOS: u32 = 12; // u8
pub const TCA_TUNNEL_KEY_ENC_TTL: u32 = 13; // u8
pub const TCA_TUNNEL_KEY_NO_FRAG: u32 = 14; // flag
pub const __TCA_TUNNEL_KEY_MAX: u32 = 15;
pub const TCA_TUNNEL_KEY_MAX: u32 = __TCA_TUNNEL_KEY_MAX - 1;

pub const TCA_TUNNEL_KEY_ENC_OPTS_UNSPEC: u32 = 0;
pub const TCA_TUNNEL_KEY_ENC_OPTS_GENEVE: u32 = 1; // Nested TCA_TUNNEL_KEY_ENC_OPTS_ attributes
pub const TCA_TUNNEL_KEY_ENC_OPTS_VXLAN: u32 = 2; // Nested TCA_TUNNEL_KEY_ENC_OPTS_ attributes
pub const TCA_TUNNEL_KEY_ENC_OPTS_ERSPAN: u32 = 3; // Nested TCA_TUNNEL_KEY_ENC_OPTS_ attributes
pub const __TCA_TUNNEL_KEY_ENC_OPTS_MAX: u32 = 4;
pub const TCA_TUNNEL_KEY_ENC_OPTS_MAX: u32 = __TCA_TUNNEL_KEY_ENC_OPTS_MAX - 1;

pub const TCA_TUNNEL_KEY_ENC_OPT_GENEVE_UNSPEC: u32 = 0;
pub const TCA_TUNNEL_KEY_ENC_OPT_GENEVE_CLASS: u32 = 1; // be16
pub const TCA_TUNNEL_KEY_ENC_OPT_GENEVE_TYPE: u32 = 2; // u8
pub const TCA_TUNNEL_KEY_ENC_OPT_GENEVE_DATA: u32 = 3; // 4 to 128 bytes
pub const __TCA_TUNNEL_KEY_ENC_OPT_GENEVE_MAX: u32 = 4;
pub const TCA_TUNNEL_KEY_ENC_OPT_GENEVE_MAX: u32 = __TCA_TUNNEL_KEY_ENC_OPT_GENEVE_MAX - 1;

pub const TCA_TUNNEL_KEY_ENC_OPT_VXLAN_UNSPEC: u32 = 0;
pub const TCA_TUNNEL_KEY_ENC_OPT_VXLAN_GBP: u32 = 1; // u32
pub const __TCA_TUNNEL_KEY_ENC_OPT_VXLAN_MAX: u32 = 2;
pub const TCA_TUNNEL_KEY_ENC_OPT_VXLAN_MAX: u32 = __TCA_TUNNEL_KEY_ENC_OPT_VXLAN_MAX - 1;

pub const TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_UNSPEC: u32 = 0;
pub const TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_VER: u32 = 1; // u8
pub const TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_INDEX: u32 = 2; // be32
pub const TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_DIR: u32 = 3; // u8
pub const TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_HWID: u32 = 4; // u8
pub const __TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_MAX: u32 = 5;
pub const TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_MAX: u32 = __TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
