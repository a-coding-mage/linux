// SPDX-License-Identifier: GPL-2.0-or-later
/* Vhost-user protocol */

/* Message flags */
pub const VHOST_USER_FLAG_REPLY: u32 = 1u32 << 2;
pub const VHOST_USER_FLAG_NEED_REPLY: u32 = 1u32 << 3;
/* Feature bits */
pub const VHOST_USER_F_PROTOCOL_FEATURES: u32 = 30;
/* Protocol feature bits */
pub const VHOST_USER_PROTOCOL_F_MQ: u32 = 0;
pub const VHOST_USER_PROTOCOL_F_REPLY_ACK: u32 = 3;
pub const VHOST_USER_PROTOCOL_F_SLAVE_REQ: u32 = 5;
pub const VHOST_USER_PROTOCOL_F_CONFIG: u32 = 9;
pub const VHOST_USER_PROTOCOL_F_INBAND_NOTIFICATIONS: u32 = 14;
/* Vring state index masks */
pub const VHOST_USER_VRING_INDEX_MASK: u32 = 0xff;
pub const VHOST_USER_VRING_POLL_MASK: u32 = 1u32 << 8;

/* Supported version */
pub const VHOST_USER_VERSION: u32 = 1;
/* Supported transport features */
pub const VHOST_USER_SUPPORTED_F: u64 = 1u64 << VHOST_USER_F_PROTOCOL_FEATURES;
/* Supported protocol features */
pub const VHOST_USER_SUPPORTED_PROTOCOL_F: u64 =
    (1u64 << VHOST_USER_PROTOCOL_F_MQ)
        | (1u64 << VHOST_USER_PROTOCOL_F_REPLY_ACK)
        | (1u64 << VHOST_USER_PROTOCOL_F_SLAVE_REQ)
        | (1u64 << VHOST_USER_PROTOCOL_F_CONFIG)
        | (1u64 << VHOST_USER_PROTOCOL_F_INBAND_NOTIFICATIONS);

#[repr(u32)]
pub enum VhostUserRequest {
    VHOST_USER_GET_FEATURES = 1,
    VHOST_USER_SET_FEATURES = 2,
    VHOST_USER_SET_OWNER = 3,
    VHOST_USER_RESET_OWNER = 4,
    VHOST_USER_SET_MEM_TABLE = 5,
    VHOST_USER_SET_LOG_BASE = 6,
    VHOST_USER_SET_LOG_FD = 7,
    VHOST_USER_SET_VRING_NUM = 8,
    VHOST_USER_SET_VRING_ADDR = 9,
    VHOST_USER_SET_VRING_BASE = 10,
    VHOST_USER_GET_VRING_BASE = 11,
    VHOST_USER_SET_VRING_KICK = 12,
    VHOST_USER_SET_VRING_CALL = 13,
    VHOST_USER_SET_VRING_ERR = 14,
    VHOST_USER_GET_PROTOCOL_FEATURES = 15,
    VHOST_USER_SET_PROTOCOL_FEATURES = 16,
    VHOST_USER_GET_QUEUE_NUM = 17,
    VHOST_USER_SET_VRING_ENABLE = 18,
    VHOST_USER_SEND_RARP = 19,
    VHOST_USER_NET_SEND_MTU = 20,
    VHOST_USER_SET_SLAVE_REQ_FD = 21,
    VHOST_USER_IOTLB_MSG = 22,
    VHOST_USER_SET_VRING_ENDIAN = 23,
    VHOST_USER_GET_CONFIG = 24,
    VHOST_USER_SET_CONFIG = 25,
    VHOST_USER_VRING_KICK = 35,
}

#[repr(u32)]
pub enum VhostUserSlaveRequest {
    VHOST_USER_SLAVE_IOTLB_MSG = 1,
    VHOST_USER_SLAVE_CONFIG_CHANGE_MSG = 2,
    VHOST_USER_SLAVE_VRING_HOST_NOTIFIER_MSG = 3,
    VHOST_USER_SLAVE_VRING_CALL = 4,
}

#[repr(C, packed)]
pub struct VhostUserHeader {
    /*
     * Use enum vhost_user_request for outgoing messages,
     * uses enum vhost_user_slave_request for incoming ones.
     */
    pub request: u32,
    pub flags: u32,
    pub size: u32,
}

#[repr(C, packed)]
pub struct VhostUserConfig {
    pub offset: u32,
    pub size: u32,
    pub flags: u32,
    pub payload: [u8; 0], /* Variable length */
}

#[repr(C, packed)]
pub struct VhostUserVringState {
    pub index: u32,
    pub num: u32,
}

#[repr(C, packed)]
pub struct VhostUserVringAddr {
    pub index: u32,
    pub flags: u32,
    pub desc: u64,
    pub used: u64,
    pub avail: u64,
    pub log: u64,
}

#[repr(C, packed)]
pub struct VhostUserMemRegion {
    pub guest_addr: u64,
    pub size: u64,
    pub user_addr: u64,
    pub mmap_offset: u64,
}

#[repr(C, packed)]
pub struct VhostUserMemRegions {
    pub num: u32,
    pub padding: u32,
    pub regions: [VhostUserMemRegion; 2], /* Currently supporting 2 */
}

#[repr(C)]
pub union VhostUserPayload {
    pub integer: u64,
    pub config: VhostUserConfig,
    pub vring_state: VhostUserVringState,
    pub vring_addr: VhostUserVringAddr,
    pub mem_regions: VhostUserMemRegions,
}

#[repr(C, packed)]
pub struct VhostUserMsg {
    pub header: VhostUserHeader,
    pub payload: VhostUserPayload,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
