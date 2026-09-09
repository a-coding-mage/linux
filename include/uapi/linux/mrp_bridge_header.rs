/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

// Translated from the Linux UAPI header.
// Dependencies from <linux/types.h> and <linux/if_ether.h> are supplied externally.

pub const MRP_MAX_FRAME_LENGTH: i32 = 200;
pub const MRP_DEFAULT_PRIO: i32 = 0x8000;
pub const MRP_DOMAIN_UUID_LENGTH: i32 = 16;
pub const MRP_VERSION: i32 = 1;
pub const MRP_FRAME_PRIO: i32 = 7;
pub const MRP_OUI_LENGTH: i32 = 3;
pub const MRP_MANUFACTURE_DATA_LENGTH: i32 = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_ring_role_type {
    BR_MRP_RING_ROLE_DISABLED,
    BR_MRP_RING_ROLE_MRC,
    BR_MRP_RING_ROLE_MRM,
    BR_MRP_RING_ROLE_MRA,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_in_role_type {
    BR_MRP_IN_ROLE_DISABLED,
    BR_MRP_IN_ROLE_MIC,
    BR_MRP_IN_ROLE_MIM,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_ring_state_type {
    BR_MRP_RING_STATE_OPEN,
    BR_MRP_RING_STATE_CLOSED,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_in_state_type {
    BR_MRP_IN_STATE_OPEN,
    BR_MRP_IN_STATE_CLOSED,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_port_state_type {
    BR_MRP_PORT_STATE_DISABLED,
    BR_MRP_PORT_STATE_BLOCKED,
    BR_MRP_PORT_STATE_FORWARDING,
    BR_MRP_PORT_STATE_NOT_CONNECTED,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_port_role_type {
    BR_MRP_PORT_ROLE_PRIMARY,
    BR_MRP_PORT_ROLE_SECONDARY,
    BR_MRP_PORT_ROLE_INTER,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_tlv_header_type {
    BR_MRP_TLV_HEADER_END = 0x0,
    BR_MRP_TLV_HEADER_COMMON = 0x1,
    BR_MRP_TLV_HEADER_RING_TEST = 0x2,
    BR_MRP_TLV_HEADER_RING_TOPO = 0x3,
    BR_MRP_TLV_HEADER_RING_LINK_DOWN = 0x4,
    BR_MRP_TLV_HEADER_RING_LINK_UP = 0x5,
    BR_MRP_TLV_HEADER_IN_TEST = 0x6,
    BR_MRP_TLV_HEADER_IN_TOPO = 0x7,
    BR_MRP_TLV_HEADER_IN_LINK_DOWN = 0x8,
    BR_MRP_TLV_HEADER_IN_LINK_UP = 0x9,
    BR_MRP_TLV_HEADER_IN_LINK_STATUS = 0xa,
    BR_MRP_TLV_HEADER_OPTION = 0x7f,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_sub_tlv_header_type {
    BR_MRP_SUB_TLV_HEADER_TEST_MGR_NACK = 0x1,
    BR_MRP_SUB_TLV_HEADER_TEST_PROPAGATE = 0x2,
    BR_MRP_SUB_TLV_HEADER_TEST_AUTO_MGR = 0x3,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
