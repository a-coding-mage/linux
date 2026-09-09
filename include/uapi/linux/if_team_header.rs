/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/team.yaml */
/* YNL-GEN uapi header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

pub const TEAM_GENL_NAME: &str = "team";
pub const TEAM_GENL_VERSION: i32 = 1;

pub const TEAM_STRING_MAX_LEN: i32 = 32;
pub const TEAM_GENL_CHANGE_EVENT_MC_GRP_NAME: &str = "change_event";

pub const TEAM_ATTR_UNSPEC: i32 = 0;
pub const TEAM_ATTR_TEAM_IFINDEX: i32 = 1;
pub const TEAM_ATTR_LIST_OPTION: i32 = 2;
pub const TEAM_ATTR_LIST_PORT: i32 = 3;

pub const __TEAM_ATTR_MAX: i32 = 4;
pub const TEAM_ATTR_MAX: i32 = __TEAM_ATTR_MAX - 1;

pub const TEAM_ATTR_ITEM_OPTION_UNSPEC: i32 = 0;
pub const TEAM_ATTR_ITEM_OPTION: i32 = 1;

pub const __TEAM_ATTR_ITEM_OPTION_MAX: i32 = 2;
pub const TEAM_ATTR_ITEM_OPTION_MAX: i32 = __TEAM_ATTR_ITEM_OPTION_MAX - 1;

pub const TEAM_ATTR_OPTION_UNSPEC: i32 = 0;
pub const TEAM_ATTR_OPTION_NAME: i32 = 1;
pub const TEAM_ATTR_OPTION_CHANGED: i32 = 2;
pub const TEAM_ATTR_OPTION_TYPE: i32 = 3;
pub const TEAM_ATTR_OPTION_DATA: i32 = 4;
pub const TEAM_ATTR_OPTION_REMOVED: i32 = 5;
pub const TEAM_ATTR_OPTION_PORT_IFINDEX: i32 = 6;
pub const TEAM_ATTR_OPTION_ARRAY_INDEX: i32 = 7;

pub const __TEAM_ATTR_OPTION_MAX: i32 = 8;
pub const TEAM_ATTR_OPTION_MAX: i32 = __TEAM_ATTR_OPTION_MAX - 1;

pub const TEAM_ATTR_ITEM_PORT_UNSPEC: i32 = 0;
pub const TEAM_ATTR_ITEM_PORT: i32 = 1;

pub const __TEAM_ATTR_ITEM_PORT_MAX: i32 = 2;
pub const TEAM_ATTR_ITEM_PORT_MAX: i32 = __TEAM_ATTR_ITEM_PORT_MAX - 1;

pub const TEAM_ATTR_PORT_UNSPEC: i32 = 0;
pub const TEAM_ATTR_PORT_IFINDEX: i32 = 1;
pub const TEAM_ATTR_PORT_CHANGED: i32 = 2;
pub const TEAM_ATTR_PORT_LINKUP: i32 = 3;
pub const TEAM_ATTR_PORT_SPEED: i32 = 4;
pub const TEAM_ATTR_PORT_DUPLEX: i32 = 5;
pub const TEAM_ATTR_PORT_REMOVED: i32 = 6;

pub const __TEAM_ATTR_PORT_MAX: i32 = 7;
pub const TEAM_ATTR_PORT_MAX: i32 = __TEAM_ATTR_PORT_MAX - 1;

pub const TEAM_CMD_NOOP: i32 = 0;
pub const TEAM_CMD_OPTIONS_SET: i32 = 1;
pub const TEAM_CMD_OPTIONS_GET: i32 = 2;
pub const TEAM_CMD_PORT_LIST_GET: i32 = 3;

pub const __TEAM_CMD_MAX: i32 = 4;
pub const TEAM_CMD_MAX: i32 = __TEAM_CMD_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
