/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (c) 2024,2025, Intel Corporation
 *
 * These are definitions for the mailbox command interface of CXL subsystem.
 */

// C header dependency: linux/types.h.  Little-endian integer fields are
// represented by their native-width integer types here.
pub type __uapi_uuid_t = [u8; 16];

#[repr(C, packed)]
pub struct cxl_mbox_get_sup_feats_in {
    pub count: u32,
    pub start_idx: u16,
    pub reserved: [u8; 2],
}

/* CXL spec r3.2 Table 8-87 command effects */
pub const CXL_CMD_CONFIG_CHANGE_COLD_RESET: u16 = 1u16 << 0;
pub const CXL_CMD_CONFIG_CHANGE_IMMEDIATE: u16 = 1u16 << 1;
pub const CXL_CMD_DATA_CHANGE_IMMEDIATE: u16 = 1u16 << 2;
pub const CXL_CMD_POLICY_CHANGE_IMMEDIATE: u16 = 1u16 << 3;
pub const CXL_CMD_LOG_CHANGE_IMMEDIATE: u16 = 1u16 << 4;
pub const CXL_CMD_SECURITY_STATE_CHANGE: u16 = 1u16 << 5;
pub const CXL_CMD_BACKGROUND: u16 = 1u16 << 6;
pub const CXL_CMD_BGCMD_ABORT_SUPPORTED: u16 = 1u16 << 7;
pub const CXL_CMD_EFFECTS_VALID: u16 = 1u16 << 9;
pub const CXL_CMD_CONFIG_CHANGE_CONV_RESET: u16 = 1u16 << 10;
pub const CXL_CMD_CONFIG_CHANGE_CXL_RESET: u16 = 1u16 << 11;
pub const CXL_CMD_EFFECTS_RESERVED: u16 = 0xf000u16;

#[repr(C, packed)]
pub struct cxl_feat_entry {
    pub uuid: __uapi_uuid_t,
    pub id: u16,
    pub get_feat_size: u16,
    pub set_feat_size: u16,
    pub flags: u32,
    pub get_feat_ver: u8,
    pub set_feat_ver: u8,
    pub effects: u16,
    pub reserved: [u8; 18],
}

/* @flags field for 'struct cxl_feat_entry' */
pub const CXL_FEATURE_F_CHANGEABLE: u32 = 1u32 << 0;
pub const CXL_FEATURE_F_PERSIST_FW_UPDATE: u32 = 1u32 << 4;
pub const CXL_FEATURE_F_DEFAULT_SEL: u32 = 1u32 << 5;
pub const CXL_FEATURE_F_SAVED_SEL: u32 = 1u32 << 6;

#[repr(C, packed)]
pub struct cxl_mbox_get_sup_feats_out {
    pub num_entries: u16,
    pub supported_feats: u16,
    pub reserved: [u8; 4],
    pub ents: [cxl_feat_entry; 0],
}

#[repr(C, packed)]
pub struct cxl_mbox_get_feat_in {
    pub uuid: __uapi_uuid_t,
    pub offset: u16,
    pub count: u16,
    pub selection: u8,
}

pub const CXL_GET_FEAT_SEL_CURRENT_VALUE: u32 = 0;
pub const CXL_GET_FEAT_SEL_DEFAULT_VALUE: u32 = 1;
pub const CXL_GET_FEAT_SEL_SAVED_VALUE: u32 = 2;
pub const CXL_GET_FEAT_SEL_MAX: u32 = 3;

#[repr(C, packed)]
pub struct cxl_mbox_set_feat_in {
    pub uuid: __uapi_uuid_t,
    pub flags: u32,
    pub offset: u16,
    pub version: u8,
    pub rsvd: [u8; 9],
    pub feat_data: [u8; 0],
}

pub const CXL_SET_FEAT_FLAG_FULL_DATA_TRANSFER: u32 = 0;
pub const CXL_SET_FEAT_FLAG_INITIATE_DATA_TRANSFER: u32 = 1;
pub const CXL_SET_FEAT_FLAG_CONTINUE_DATA_TRANSFER: u32 = 2;
pub const CXL_SET_FEAT_FLAG_FINISH_DATA_TRANSFER: u32 = 3;
pub const CXL_SET_FEAT_FLAG_ABORT_DATA_TRANSFER: u32 = 4;
pub const CXL_SET_FEAT_FLAG_DATA_TRANSFER_MAX: u32 = 5;

pub const CXL_SET_FEAT_FLAG_DATA_TRANSFER_MASK: u32 = 0x7;
pub const CXL_SET_FEAT_FLAG_DATA_SAVED_ACROSS_RESET: u32 = 1u32 << 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
