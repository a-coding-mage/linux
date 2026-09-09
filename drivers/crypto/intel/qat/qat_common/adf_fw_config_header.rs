/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2023 Intel Corporation */

// Translated from the C header; the original header guard is omitted.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adf_fw_objs {
    ADF_FW_SYM_OBJ,
    ADF_FW_ASYM_OBJ,
    ADF_FW_DC_OBJ,
    ADF_FW_ADMIN_OBJ,
    ADF_FW_CY_OBJ,
    ADF_FW_WCY_OBJ,
}

#[repr(C)]
pub struct adf_fw_config {
    pub ae_mask: u32,
    pub obj: adf_fw_objs,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
