/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/fou.yaml */
/* YNL-GEN uapi header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

pub const FOU_GENL_NAME: &str = "fou";
pub const FOU_GENL_VERSION: i32 = 1;

pub const FOU_ENCAP_UNSPEC: i32 = 0;
pub const FOU_ENCAP_DIRECT: i32 = 1;
pub const FOU_ENCAP_GUE: i32 = 2;

pub const FOU_ATTR_UNSPEC: i32 = 0;
pub const FOU_ATTR_PORT: i32 = 1;
pub const FOU_ATTR_AF: i32 = 2;
pub const FOU_ATTR_IPPROTO: i32 = 3;
pub const FOU_ATTR_TYPE: i32 = 4;
pub const FOU_ATTR_REMCSUM_NOPARTIAL: i32 = 5;
pub const FOU_ATTR_LOCAL_V4: i32 = 6;
pub const FOU_ATTR_LOCAL_V6: i32 = 7;
pub const FOU_ATTR_PEER_V4: i32 = 8;
pub const FOU_ATTR_PEER_V6: i32 = 9;
pub const FOU_ATTR_PEER_PORT: i32 = 10;
pub const FOU_ATTR_IFINDEX: i32 = 11;

const __FOU_ATTR_MAX: i32 = 12;
pub const FOU_ATTR_MAX: i32 = __FOU_ATTR_MAX - 1;

pub const FOU_CMD_UNSPEC: i32 = 0;
pub const FOU_CMD_ADD: i32 = 1;
pub const FOU_CMD_DEL: i32 = 2;
pub const FOU_CMD_GET: i32 = 3;

const __FOU_CMD_MAX: i32 = 4;
pub const FOU_CMD_MAX: i32 = __FOU_CMD_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
