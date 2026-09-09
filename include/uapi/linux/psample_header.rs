/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Anonymous enum constants from the C header.
pub const PSAMPLE_ATTR_IIFINDEX: i32 = 0;
pub const PSAMPLE_ATTR_OIFINDEX: i32 = 1;
pub const PSAMPLE_ATTR_ORIGSIZE: i32 = 2;
pub const PSAMPLE_ATTR_SAMPLE_GROUP: i32 = 3;
pub const PSAMPLE_ATTR_GROUP_SEQ: i32 = 4;
pub const PSAMPLE_ATTR_SAMPLE_RATE: i32 = 5; /* u32, ratio between observed and
                                              * sampled packets or scaled probability
                                              * if PSAMPLE_ATTR_SAMPLE_PROBABILITY
                                              * is set.
                                              */
pub const PSAMPLE_ATTR_DATA: i32 = 6;
pub const PSAMPLE_ATTR_GROUP_REFCOUNT: i32 = 7;
pub const PSAMPLE_ATTR_TUNNEL: i32 = 8;
pub const PSAMPLE_ATTR_PAD: i32 = 9;
pub const PSAMPLE_ATTR_OUT_TC: i32 = 10; /* u16 */
pub const PSAMPLE_ATTR_OUT_TC_OCC: i32 = 11; /* u64, bytes */
pub const PSAMPLE_ATTR_LATENCY: i32 = 12; /* u64, nanoseconds */
pub const PSAMPLE_ATTR_TIMESTAMP: i32 = 13; /* u64, nanoseconds */
pub const PSAMPLE_ATTR_PROTO: i32 = 14; /* u16 */
pub const PSAMPLE_ATTR_USER_COOKIE: i32 = 15; /* binary, user provided data */
pub const PSAMPLE_ATTR_SAMPLE_PROBABILITY: i32 = 16; /* no argument, interpret rate in
                                                       * PSAMPLE_ATTR_SAMPLE_RATE as a
                                                       * probability scaled 0 - U32_MAX.
                                                       */
pub const __PSAMPLE_ATTR_MAX: i32 = 17;

pub const PSAMPLE_CMD_SAMPLE: i32 = 0;
pub const PSAMPLE_CMD_GET_GROUP: i32 = 1;
pub const PSAMPLE_CMD_NEW_GROUP: i32 = 2;
pub const PSAMPLE_CMD_DEL_GROUP: i32 = 3;

pub const PSAMPLE_TUNNEL_KEY_ATTR_ID: i32 = 0; /* be64 Tunnel ID */
pub const PSAMPLE_TUNNEL_KEY_ATTR_IPV4_SRC: i32 = 1; /* be32 src IP address. */
pub const PSAMPLE_TUNNEL_KEY_ATTR_IPV4_DST: i32 = 2; /* be32 dst IP address. */
pub const PSAMPLE_TUNNEL_KEY_ATTR_TOS: i32 = 3; /* u8 Tunnel IP ToS. */
pub const PSAMPLE_TUNNEL_KEY_ATTR_TTL: i32 = 4; /* u8 Tunnel IP TTL. */
pub const PSAMPLE_TUNNEL_KEY_ATTR_DONT_FRAGMENT: i32 = 5; /* No argument, set DF. */
pub const PSAMPLE_TUNNEL_KEY_ATTR_CSUM: i32 = 6; /* No argument. CSUM packet. */
pub const PSAMPLE_TUNNEL_KEY_ATTR_OAM: i32 = 7; /* No argument. OAM frame. */
pub const PSAMPLE_TUNNEL_KEY_ATTR_GENEVE_OPTS: i32 = 8; /* Array of Geneve options. */
pub const PSAMPLE_TUNNEL_KEY_ATTR_TP_SRC: i32 = 9; /* be16 src Transport Port. */
pub const PSAMPLE_TUNNEL_KEY_ATTR_TP_DST: i32 = 10; /* be16 dst Transport Port. */
pub const PSAMPLE_TUNNEL_KEY_ATTR_VXLAN_OPTS: i32 = 11; /* Nested VXLAN opts* */
pub const PSAMPLE_TUNNEL_KEY_ATTR_IPV6_SRC: i32 = 12; /* struct in6_addr src IPv6 address. */
pub const PSAMPLE_TUNNEL_KEY_ATTR_IPV6_DST: i32 = 13; /* struct in6_addr dst IPv6 address. */
pub const PSAMPLE_TUNNEL_KEY_ATTR_PAD: i32 = 14;
pub const PSAMPLE_TUNNEL_KEY_ATTR_ERSPAN_OPTS: i32 = 15; /* struct erspan_metadata */
pub const PSAMPLE_TUNNEL_KEY_ATTR_IPV4_INFO_BRIDGE: i32 = 16; /* No argument. IPV4_INFO_BRIDGE mode. */
pub const __PSAMPLE_TUNNEL_KEY_ATTR_MAX: i32 = 17;

/* Can be overridden at runtime by module option */
pub const PSAMPLE_ATTR_MAX: i32 = __PSAMPLE_ATTR_MAX - 1;

pub const PSAMPLE_NL_MCGRP_CONFIG_NAME: &str = "config";
pub const PSAMPLE_NL_MCGRP_SAMPLE_NAME: &str = "packets";
pub const PSAMPLE_GENL_NAME: &str = "psample";
pub const PSAMPLE_GENL_VERSION: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
