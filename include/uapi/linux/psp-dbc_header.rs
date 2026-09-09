/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Userspace interface for AMD Dynamic Boost Control (DBC)
 *
 * Copyright (C) 2023 Advanced Micro Devices, Inc.
 *
 * Author: Mario Limonciello <mario.limonciello@amd.com>
 */

//! AMD Dynamic Boost Control (DBC) interface.

// The C header includes <linux/types.h>; these aliases preserve its fixed-width
// userspace ABI types.
pub type __u8 = u8;
pub type __u32 = u32;

pub const DBC_NONCE_SIZE: usize = 16;
pub const DBC_SIG_SIZE: usize = 32;
pub const DBC_UID_SIZE: usize = 16;

/// Nonce exchange structure (input/output).
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct dbc_user_nonce {
    pub auth_needed: __u32,
    pub nonce: [__u8; DBC_NONCE_SIZE],
    pub signature: [__u8; DBC_SIG_SIZE],
}

/// UID exchange structure (input).
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct dbc_user_setuid {
    pub uid: [__u8; DBC_UID_SIZE],
    pub signature: [__u8; DBC_SIG_SIZE],
}

/// Parameter exchange structure (input/output).
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct dbc_user_param {
    pub msg_index: __u32,
    pub param: __u32,
    pub signature: [__u8; DBC_SIG_SIZE],
}

pub const DBC_IOC_TYPE: u8 = b'D';

// Linux _IO* encodings for the packed structures above (_IOWR/_IOW).
pub const DBCIOCNONCE: u32 = 0xC034_4401;
pub const DBCIOCUID: u32 = 0x4030_4402;
pub const DBCIOCPARAM: u32 = 0xC028_4403;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum dbc_cmd_msg {
    PARAM_GET_FMAX_CAP = 0x3,
    PARAM_SET_FMAX_CAP = 0x4,
    PARAM_GET_PWR_CAP = 0x5,
    PARAM_SET_PWR_CAP = 0x6,
    PARAM_GET_GFX_MODE = 0x7,
    PARAM_SET_GFX_MODE = 0x8,
    PARAM_GET_CURR_TEMP = 0x9,
    PARAM_GET_FMAX_MAX = 0xA,
    PARAM_GET_FMAX_MIN = 0xB,
    PARAM_GET_SOC_PWR_MAX = 0xC,
    PARAM_GET_SOC_PWR_MIN = 0xD,
    PARAM_GET_SOC_PWR_CUR = 0xE,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
