/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Edward-JW Yang <edward-jw.yang@mediatek.com>
 */

// Dependency intent from C: #include "clk-pllfh.h"

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum fhctl_variant {
    FHCTL_PLLFH_V1,
    FHCTL_PLLFH_V2,
}

#[repr(C)]
pub struct fhctl_offset {
    pub offset_hp_en: u32,
    pub offset_clk_con: u32,
    pub offset_rst_con: u32,
    pub offset_slope0: u32,
    pub offset_slope1: u32,
    pub offset_cfg: u32,
    pub offset_updnlmt: u32,
    pub offset_dds: u32,
    pub offset_dvfs: u32,
    pub offset_mon: u32,
}

extern "C" {
    pub fn fhctl_get_offset_table(v: fhctl_variant) -> *const fhctl_offset;
    pub fn fhctl_get_ops() -> *const fh_operation;
    pub fn fhctl_hw_init(fh: *mut mtk_fh);
}

// External types supplied by the included C dependency "clk-pllfh.h".
#[repr(C)]
pub struct fh_operation {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_fh {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
