/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* The definitions, required to talk to KAME racoon IKE. */
/* Dependency: declarations from <linux/pfkeyv2.h> are supplied externally. */

pub const IPSEC_PORT_ANY: i32 = 0;
pub const IPSEC_ULPROTO_ANY: i32 = 255;
pub const IPSEC_PROTO_ANY: i32 = 255;

pub const IPSEC_MODE_ANY: i32 = 0; /* We do not support this for SA */
pub const IPSEC_MODE_TRANSPORT: i32 = 1;
pub const IPSEC_MODE_TUNNEL: i32 = 2;
pub const IPSEC_MODE_BEET: i32 = 3;
pub const IPSEC_MODE_IPTFS: i32 = 4;

pub const IPSEC_DIR_ANY: i32 = 0;
pub const IPSEC_DIR_INBOUND: i32 = 1;
pub const IPSEC_DIR_OUTBOUND: i32 = 2;
pub const IPSEC_DIR_FWD: i32 = 3; /* It is our own */
pub const IPSEC_DIR_MAX: i32 = 4;
pub const IPSEC_DIR_INVALID: i32 = 5;

pub const IPSEC_POLICY_DISCARD: i32 = 0;
pub const IPSEC_POLICY_NONE: i32 = 1;
pub const IPSEC_POLICY_IPSEC: i32 = 2;
pub const IPSEC_POLICY_ENTRUST: i32 = 3;
pub const IPSEC_POLICY_BYPASS: i32 = 4;

pub const IPSEC_LEVEL_DEFAULT: i32 = 0;
pub const IPSEC_LEVEL_USE: i32 = 1;
pub const IPSEC_LEVEL_REQUIRE: i32 = 2;
pub const IPSEC_LEVEL_UNIQUE: i32 = 3;

pub const IPSEC_MANUAL_REQID_MAX: i32 = 0x3fff;

pub const IPSEC_REPLAYWSIZE: i32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
