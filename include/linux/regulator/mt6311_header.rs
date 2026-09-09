/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2015 MediaTek Inc.
 * Author: Henry Chen <henryc.chen@mediatek.com>
 */

pub const MT6311_MAX_REGULATORS: i32 = 2;

#[repr(i32)]
pub enum Mt6311Id {
    MT6311_ID_VDVFS = 0,
    MT6311_ID_VBIASN,
}

pub const MT6311_E1_CID_CODE: i32 = 0x10;
pub const MT6311_E2_CID_CODE: i32 = 0x20;
pub const MT6311_E3_CID_CODE: i32 = 0x30;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
