/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

pub const ETHER_HEADER_LENGTH: u32 = 6 + 6 + 4 + 2;
pub const CFM_MAID_LENGTH: u32 = 48;
pub const CFM_CCM_PDU_LENGTH: u32 = 75;
pub const CFM_PORT_STATUS_TLV_LENGTH: u32 = 4;
pub const CFM_IF_STATUS_TLV_LENGTH: u32 = 4;
pub const CFM_IF_STATUS_TLV_TYPE: u32 = 4;
pub const CFM_PORT_STATUS_TLV_TYPE: u32 = 2;
pub const CFM_ENDE_TLV_TYPE: u32 = 0;
pub const CFM_CCM_MAX_FRAME_LENGTH: u32 = ETHER_HEADER_LENGTH
    + CFM_CCM_PDU_LENGTH
    + CFM_PORT_STATUS_TLV_LENGTH
    + CFM_IF_STATUS_TLV_LENGTH;
pub const CFM_FRAME_PRIO: u32 = 7;
pub const CFM_CCM_TLV_OFFSET: u32 = 70;
pub const CFM_CCM_PDU_MAID_OFFSET: u32 = 10;
pub const CFM_CCM_PDU_MEPID_OFFSET: u32 = 8;
pub const CFM_CCM_PDU_SEQNR_OFFSET: u32 = 4;
pub const CFM_CCM_PDU_TLV_OFFSET: u32 = 74;
pub const CFM_CCM_ITU_RESERVED_SIZE: u32 = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_cfm_common_hdr {
    pub mdlevel_version: u8,
    pub opcode: u8,
    pub flags: u8,
    pub tlv_offset: u8,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum br_cfm_opcodes {
    BR_CFM_OPCODE_CCM = 0x1,
}

/* MEP domain */
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum br_cfm_domain {
    BR_CFM_PORT,
    BR_CFM_VLAN,
}

/* MEP direction */
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum br_cfm_mep_direction {
    BR_CFM_MEP_DIRECTION_DOWN,
    BR_CFM_MEP_DIRECTION_UP,
}

/* CCM interval supported. */
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum br_cfm_ccm_interval {
    BR_CFM_CCM_INTERVAL_NONE,
    BR_CFM_CCM_INTERVAL_3_3_MS,
    BR_CFM_CCM_INTERVAL_10_MS,
    BR_CFM_CCM_INTERVAL_100_MS,
    BR_CFM_CCM_INTERVAL_1_SEC,
    BR_CFM_CCM_INTERVAL_10_SEC,
    BR_CFM_CCM_INTERVAL_1_MIN,
    BR_CFM_CCM_INTERVAL_10_MIN,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
