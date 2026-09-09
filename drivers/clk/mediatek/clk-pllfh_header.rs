/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Edward-JW Yang <edward-jw.yang@mediatek.com>
 */

// Dependency supplied by the translated clk-pll interface.

#[repr(C)]
pub struct fh_pll_state {
    pub base: *mut core::ffi::c_void,
    pub fh_enable: u32,
    pub ssc_rate: u32,
}

#[repr(C)]
pub struct fh_pll_data {
    pub pll_id: i32,
    pub fh_id: i32,
    pub fh_ver: i32,
    pub fhx_offset: u32,
    pub dds_mask: u32,
    pub slope0_value: u32,
    pub slope1_value: u32,
    pub sfstrx_en: u32,
    pub frddsx_en: u32,
    pub fhctlx_en: u32,
    pub tgl_org: u32,
    pub dvfs_tri: u32,
    pub pcwchg: u32,
    pub dt_val: u32,
    pub df_val: u32,
    pub updnlmt_shft: u32,
    pub msk_frddsx_dys: u32,
    pub msk_frddsx_dts: u32,
}

#[repr(C)]
pub struct mtk_pllfh_data {
    pub state: fh_pll_state,
    pub data: fh_pll_data,
}

#[repr(C)]
pub struct fh_pll_regs {
    pub reg_hp_en: *mut core::ffi::c_void,
    pub reg_clk_con: *mut core::ffi::c_void,
    pub reg_rst_con: *mut core::ffi::c_void,
    pub reg_slope0: *mut core::ffi::c_void,
    pub reg_slope1: *mut core::ffi::c_void,
    pub reg_cfg: *mut core::ffi::c_void,
    pub reg_updnlmt: *mut core::ffi::c_void,
    pub reg_dds: *mut core::ffi::c_void,
    pub reg_dvfs: *mut core::ffi::c_void,
    pub reg_mon: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct mtk_fh {
    pub clk_pll: mtk_clk_pll,
    pub regs: fh_pll_regs,
    pub pllfh_data: *mut mtk_pllfh_data,
    pub ops: *const fh_operation,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
pub struct fh_operation {
    pub hopping: Option<unsafe extern "C" fn(
        fh: *mut mtk_fh,
        new_dds: u32,
        postdiv: u32,
    ) -> i32>,
    pub ssc_enable: Option<unsafe extern "C" fn(fh: *mut mtk_fh, rate: u32) -> i32>,
}

extern "C" {
    pub fn mtk_clk_register_pllfhs(
        dev: *mut device,
        plls: *const mtk_pll_data,
        num_plls: i32,
        pllfhs: *mut mtk_pllfh_data,
        num_pllfhs: i32,
        clk_data: *mut clk_hw_onecell_data,
    ) -> i32;

    pub fn mtk_clk_unregister_pllfhs(
        plls: *const mtk_pll_data,
        num_plls: i32,
        pllfhs: *mut mtk_pllfh_data,
        num_fhs: i32,
        clk_data: *mut clk_hw_onecell_data,
    );

    pub fn fhctl_parse_dt(
        compatible_node: *const u8,
        pllfhs: *mut mtk_pllfh_data,
        num_pllfhs: i32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
