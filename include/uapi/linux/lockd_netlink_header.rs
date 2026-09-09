/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/lockd.yaml */
/* YNL-GEN uapi header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// C header guard: _UAPI_LINUX_LOCKD_NETLINK_H

pub const LOCKD_FAMILY_NAME: &str = "lockd";
pub const LOCKD_FAMILY_VERSION: i32 = 1;

pub const LOCKD_A_SERVER_GRACETIME: i32 = 1;
pub const LOCKD_A_SERVER_TCP_PORT: i32 = LOCKD_A_SERVER_GRACETIME + 1;
pub const LOCKD_A_SERVER_UDP_PORT: i32 = LOCKD_A_SERVER_TCP_PORT + 1;

pub const __LOCKD_A_SERVER_MAX: i32 = LOCKD_A_SERVER_UDP_PORT + 1;
pub const LOCKD_A_SERVER_MAX: i32 = __LOCKD_A_SERVER_MAX - 1;

pub const LOCKD_CMD_SERVER_SET: i32 = 1;
pub const LOCKD_CMD_SERVER_GET: i32 = LOCKD_CMD_SERVER_SET + 1;

pub const __LOCKD_CMD_MAX: i32 = LOCKD_CMD_SERVER_GET + 1;
pub const LOCKD_CMD_MAX: i32 = __LOCKD_CMD_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
