/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: definitions corresponding to linux/types.h and
// linux/netlink.h are supplied by other translated headers.

#[repr(C)]
pub struct net_dm_drop_point {
    pub pc: [u8; 8],
    pub count: u32,
}

pub const NET_DM_CFG_VERSION: i32 = 0;
pub const NET_DM_CFG_ALERT_COUNT: i32 = 1;
pub const NET_DM_CFG_ALERT_DELAY: i32 = 2;
pub const NET_DM_CFG_MAX: i32 = 3;

#[repr(C)]
pub struct net_dm_config_entry {
    pub type_: u32,
    // __attribute__((aligned(8))); u64 has C-compatible 8-byte alignment.
    pub data: u64,
}

#[repr(C)]
pub struct net_dm_config_msg {
    pub entries: u32,
    pub options: [net_dm_config_entry; 0],
}

#[repr(C)]
pub struct net_dm_alert_msg {
    pub entries: u32,
    pub points: [net_dm_drop_point; 0],
}

#[repr(C)]
pub union net_dm_user_msg_u {
    pub user: net_dm_config_msg,
    pub alert: net_dm_alert_msg,
}

#[repr(C)]
pub struct net_dm_user_msg {
    pub u: net_dm_user_msg_u,
}

/* These are the netlink message types for this protocol */

pub const NET_DM_CMD_UNSPEC: i32 = 0;
pub const NET_DM_CMD_ALERT: i32 = 1;
pub const NET_DM_CMD_CONFIG: i32 = 2;
pub const NET_DM_CMD_START: i32 = 3;
pub const NET_DM_CMD_STOP: i32 = 4;
pub const NET_DM_CMD_PACKET_ALERT: i32 = 5;
pub const NET_DM_CMD_CONFIG_GET: i32 = 6;
pub const NET_DM_CMD_CONFIG_NEW: i32 = 7;
pub const NET_DM_CMD_STATS_GET: i32 = 8;
pub const NET_DM_CMD_STATS_NEW: i32 = 9;
pub const _NET_DM_CMD_MAX: i32 = 10;

pub const NET_DM_CMD_MAX: i32 = _NET_DM_CMD_MAX - 1;

/*
 * Our group identifiers
 */
pub const NET_DM_GRP_ALERT: i32 = 1;

pub const NET_DM_ATTR_UNSPEC: i32 = 0;
pub const NET_DM_ATTR_ALERT_MODE: i32 = 1; // u8
pub const NET_DM_ATTR_PC: i32 = 2; // u64
pub const NET_DM_ATTR_SYMBOL: i32 = 3; // string
pub const NET_DM_ATTR_IN_PORT: i32 = 4; // nested
pub const NET_DM_ATTR_TIMESTAMP: i32 = 5; // u64
pub const NET_DM_ATTR_PROTO: i32 = 6; // u16
pub const NET_DM_ATTR_PAYLOAD: i32 = 7; // binary
pub const NET_DM_ATTR_PAD: i32 = 8;
pub const NET_DM_ATTR_TRUNC_LEN: i32 = 9; // u32
pub const NET_DM_ATTR_ORIG_LEN: i32 = 10; // u32
pub const NET_DM_ATTR_QUEUE_LEN: i32 = 11; // u32
pub const NET_DM_ATTR_STATS: i32 = 12; // nested
pub const NET_DM_ATTR_HW_STATS: i32 = 13; // nested
pub const NET_DM_ATTR_ORIGIN: i32 = 14; // u16
pub const NET_DM_ATTR_HW_TRAP_GROUP_NAME: i32 = 15; // string
pub const NET_DM_ATTR_HW_TRAP_NAME: i32 = 16; // string
pub const NET_DM_ATTR_HW_ENTRIES: i32 = 17; // nested
pub const NET_DM_ATTR_HW_ENTRY: i32 = 18; // nested
pub const NET_DM_ATTR_HW_TRAP_COUNT: i32 = 19; // u32
pub const NET_DM_ATTR_SW_DROPS: i32 = 20; // flag
pub const NET_DM_ATTR_HW_DROPS: i32 = 21; // flag
pub const NET_DM_ATTR_FLOW_ACTION_COOKIE: i32 = 22; // binary
pub const NET_DM_ATTR_REASON: i32 = 23; // string

pub const __NET_DM_ATTR_MAX: i32 = 24;
pub const NET_DM_ATTR_MAX: i32 = __NET_DM_ATTR_MAX - 1;

/**
 * enum net_dm_alert_mode - Alert mode.
 * @NET_DM_ALERT_MODE_SUMMARY: A summary of recent drops is sent to user space.
 * @NET_DM_ALERT_MODE_PACKET: Each dropped packet is sent to user space along
 *                            with metadata.
 */
pub const NET_DM_ALERT_MODE_SUMMARY: i32 = 0;
pub const NET_DM_ALERT_MODE_PACKET: i32 = 1;

pub const NET_DM_ATTR_PORT_NETDEV_IFINDEX: i32 = 0; // u32
pub const NET_DM_ATTR_PORT_NETDEV_NAME: i32 = 1; // string
pub const __NET_DM_ATTR_PORT_MAX: i32 = 2;
pub const NET_DM_ATTR_PORT_MAX: i32 = __NET_DM_ATTR_PORT_MAX - 1;

pub const NET_DM_ATTR_STATS_DROPPED: i32 = 0; // u64
pub const __NET_DM_ATTR_STATS_MAX: i32 = 1;
pub const NET_DM_ATTR_STATS_MAX: i32 = __NET_DM_ATTR_STATS_MAX - 1;

pub const NET_DM_ORIGIN_SW: i32 = 0;
pub const NET_DM_ORIGIN_HW: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
