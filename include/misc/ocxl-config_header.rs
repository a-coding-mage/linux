// SPDX-License-Identifier: GPL-2.0+
// Copyright 2017 IBM Corp.

/*
 * This file lists the various constants used to read the
 * configuration space of an opencapi adapter.
 *
 * It follows the specification for opencapi 3.0
 */

pub const OCXL_EXT_CAP_ID_DVSEC: i32 = 0x23;

pub const OCXL_DVSEC_VENDOR_OFFSET: i32 = 0x4;
pub const OCXL_DVSEC_ID_OFFSET: i32 = 0x8;
pub const OCXL_DVSEC_TL_ID: i32 = 0xF000;
pub const OCXL_DVSEC_TL_BACKOFF_TIMERS: i32 = 0x10;
pub const OCXL_DVSEC_TL_RECV_CAP: i32 = 0x18;
pub const OCXL_DVSEC_TL_SEND_CAP: i32 = 0x20;
pub const OCXL_DVSEC_TL_RECV_RATE: i32 = 0x30;
pub const OCXL_DVSEC_TL_SEND_RATE: i32 = 0x50;
pub const OCXL_DVSEC_FUNC_ID: i32 = 0xF001;
pub const OCXL_DVSEC_FUNC_OFF_INDEX: i32 = 0x08;
pub const OCXL_DVSEC_FUNC_OFF_ACTAG: i32 = 0x0C;
pub const OCXL_DVSEC_AFU_INFO_ID: i32 = 0xF003;
pub const OCXL_DVSEC_AFU_INFO_AFU_IDX: i32 = 0x0A;
pub const OCXL_DVSEC_AFU_INFO_OFF: i32 = 0x0C;
pub const OCXL_DVSEC_AFU_INFO_DATA: i32 = 0x10;
pub const OCXL_DVSEC_AFU_CTRL_ID: i32 = 0xF004;
pub const OCXL_DVSEC_AFU_CTRL_AFU_IDX: i32 = 0x0A;
pub const OCXL_DVSEC_AFU_CTRL_TERM_PASID: i32 = 0x0C;
pub const OCXL_DVSEC_AFU_CTRL_ENABLE: i32 = 0x0F;
pub const OCXL_DVSEC_AFU_CTRL_PASID_SUP: i32 = 0x10;
pub const OCXL_DVSEC_AFU_CTRL_PASID_EN: i32 = 0x11;
pub const OCXL_DVSEC_AFU_CTRL_PASID_BASE: i32 = 0x14;
pub const OCXL_DVSEC_AFU_CTRL_ACTAG_SUP: i32 = 0x18;
pub const OCXL_DVSEC_AFU_CTRL_ACTAG_EN: i32 = 0x1A;
pub const OCXL_DVSEC_AFU_CTRL_ACTAG_BASE: i32 = 0x1C;
pub const OCXL_DVSEC_VENDOR_ID: i32 = 0xF0F0;
pub const OCXL_DVSEC_VENDOR_CFG_VERS: i32 = 0x0C;
pub const OCXL_DVSEC_VENDOR_TLX_VERS: i32 = 0x10;
pub const OCXL_DVSEC_VENDOR_DLX_VERS: i32 = 0x20;
pub const OCXL_DVSEC_VENDOR_RESET_RELOAD: i32 = 0x38;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
