/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/* Documentation/netlink/specs/net_shaper.yaml */
/* YNL-GEN uapi header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

pub const NET_SHAPER_FAMILY_NAME: &str = "net-shaper";
pub const NET_SHAPER_FAMILY_VERSION: i32 = 1;

/**
 * enum net_shaper_scope - Defines the shaper @id interpretation.
 * @NET_SHAPER_SCOPE_UNSPEC: The scope is not specified.
 * @NET_SHAPER_SCOPE_NETDEV: The main shaper for the given network device.
 * @NET_SHAPER_SCOPE_QUEUE: The shaper is attached to the given device queue,
 *   the @id represents the queue number.
 * @NET_SHAPER_SCOPE_NODE: The shaper allows grouping of queues or other node
 *   shapers; can be nested in either @netdev shapers or other @node shapers,
 *   allowing placement in any location of the scheduling tree, except leaves
 *   and root.
 */
#[repr(i32)]
pub enum net_shaper_scope {
    NET_SHAPER_SCOPE_UNSPEC = 0,
    NET_SHAPER_SCOPE_NETDEV = 1,
    NET_SHAPER_SCOPE_QUEUE = 2,
    NET_SHAPER_SCOPE_NODE = 3,
}

pub const __NET_SHAPER_SCOPE_MAX: i32 = 4;
pub const NET_SHAPER_SCOPE_MAX: i32 = __NET_SHAPER_SCOPE_MAX - 1;

/**
 * enum net_shaper_metric - Different metric supported by the shaper.
 * @NET_SHAPER_METRIC_BPS: Shaper operates on a bits per second basis.
 * @NET_SHAPER_METRIC_PPS: Shaper operates on a packets per second basis.
 */
#[repr(i32)]
pub enum net_shaper_metric {
    NET_SHAPER_METRIC_BPS = 0,
    NET_SHAPER_METRIC_PPS = 1,
}

pub const NET_SHAPER_A_HANDLE: i32 = 1;
pub const NET_SHAPER_A_METRIC: i32 = 2;
pub const NET_SHAPER_A_BW_MIN: i32 = 3;
pub const NET_SHAPER_A_BW_MAX: i32 = 4;
pub const NET_SHAPER_A_BURST: i32 = 5;
pub const NET_SHAPER_A_PRIORITY: i32 = 6;
pub const NET_SHAPER_A_WEIGHT: i32 = 7;
pub const NET_SHAPER_A_IFINDEX: i32 = 8;
pub const NET_SHAPER_A_PARENT: i32 = 9;
pub const NET_SHAPER_A_LEAVES: i32 = 10;
pub const __NET_SHAPER_A_MAX: i32 = 11;
pub const NET_SHAPER_A_MAX: i32 = __NET_SHAPER_A_MAX - 1;

pub const NET_SHAPER_A_HANDLE_SCOPE: i32 = 1;
pub const NET_SHAPER_A_HANDLE_ID: i32 = 2;
pub const __NET_SHAPER_A_HANDLE_MAX: i32 = 3;
pub const NET_SHAPER_A_HANDLE_MAX: i32 = __NET_SHAPER_A_HANDLE_MAX - 1;

pub const NET_SHAPER_A_CAPS_IFINDEX: i32 = 1;
pub const NET_SHAPER_A_CAPS_SCOPE: i32 = 2;
pub const NET_SHAPER_A_CAPS_SUPPORT_METRIC_BPS: i32 = 3;
pub const NET_SHAPER_A_CAPS_SUPPORT_METRIC_PPS: i32 = 4;
pub const NET_SHAPER_A_CAPS_SUPPORT_NESTING: i32 = 5;
pub const NET_SHAPER_A_CAPS_SUPPORT_BW_MIN: i32 = 6;
pub const NET_SHAPER_A_CAPS_SUPPORT_BW_MAX: i32 = 7;
pub const NET_SHAPER_A_CAPS_SUPPORT_BURST: i32 = 8;
pub const NET_SHAPER_A_CAPS_SUPPORT_PRIORITY: i32 = 9;
pub const NET_SHAPER_A_CAPS_SUPPORT_WEIGHT: i32 = 10;
pub const __NET_SHAPER_A_CAPS_MAX: i32 = 11;
pub const NET_SHAPER_A_CAPS_MAX: i32 = __NET_SHAPER_A_CAPS_MAX - 1;

pub const NET_SHAPER_CMD_GET: i32 = 1;
pub const NET_SHAPER_CMD_SET: i32 = 2;
pub const NET_SHAPER_CMD_DELETE: i32 = 3;
pub const NET_SHAPER_CMD_GROUP: i32 = 4;
pub const NET_SHAPER_CMD_CAP_GET: i32 = 5;
pub const __NET_SHAPER_CMD_MAX: i32 = 6;
pub const NET_SHAPER_CMD_MAX: i32 = __NET_SHAPER_CMD_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
