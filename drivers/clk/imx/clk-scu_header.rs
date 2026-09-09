/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright 2018-2021 NXP
 *   Dong Aisheng <aisheng.dong@nxp.com>
 */

// C header dependencies: linux/firmware/imx/sci.h and linux/of.h

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Opaque types supplied by the dependent headers.
pub enum list_head {}
pub enum dev_pm_ops {}
pub enum device_node {}
pub enum device {}
pub enum clk_hw {}
pub enum of_phandle_args {}

pub const IMX_SCU_GPR_CLK_GATE: u8 = 1u8 << 0;
pub const IMX_SCU_GPR_CLK_DIV: u8 = 1u8 << 1;
pub const IMX_SCU_GPR_CLK_MUX: u8 = 1u8 << 2;

#[repr(C)]
pub struct imx_clk_scu_rsrc_table {
    pub rsrc: *const u32,
    pub num: u8,
}

extern "C" {
    pub static mut imx_scu_clks: *mut list_head;
    pub static imx_clk_lpcg_scu_pm_ops: dev_pm_ops;
    pub static imx_clk_scu_rsrc_imx8dxl: imx_clk_scu_rsrc_table;
    pub static imx_clk_scu_rsrc_imx8qxp: imx_clk_scu_rsrc_table;
    pub static imx_clk_scu_rsrc_imx8qm: imx_clk_scu_rsrc_table;

    pub fn imx_clk_scu_module_init() -> c_int;
    pub fn imx_clk_scu_module_exit();
    pub fn imx_clk_scu_init(
        np: *mut device_node,
        data: *const imx_clk_scu_rsrc_table,
    ) -> c_int;
    pub fn imx_scu_of_clk_src_get(
        clkspec: *mut of_phandle_args,
        data: *mut c_void,
    ) -> *mut clk_hw;
    pub fn imx_clk_scu_alloc_dev(
        name: *const c_char,
        parents: *const *const c_char,
        num_parents: c_int,
        rsrc_id: u32,
        clk_type: u8,
    ) -> *mut clk_hw;
    pub fn __imx_clk_scu(
        dev: *mut device,
        name: *const c_char,
        parents: *const *const c_char,
        num_parents: c_int,
        rsrc_id: u32,
        clk_type: u8,
    ) -> *mut clk_hw;
    pub fn imx_clk_scu_unregister();
    pub fn __imx_clk_lpcg_scu(
        dev: *mut device,
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        reg: *mut c_void,
        bit_idx: u8,
        hw_gate: bool,
    ) -> *mut clk_hw;
    pub fn imx_clk_lpcg_scu_unregister(hw: *mut clk_hw);
    pub fn __imx_clk_gpr_scu(
        name: *const c_char,
        parent_name: *const *const c_char,
        num_parents: c_int,
        rsrc_id: u32,
        gpr_id: u8,
        flags: u8,
        invert: bool,
    ) -> *mut clk_hw;
}

#[inline]
pub unsafe fn imx_clk_scu(name: *const c_char, rsrc_id: u32, clk_type: u8) -> *mut clk_hw {
    imx_clk_scu_alloc_dev(name, core::ptr::null(), 0, rsrc_id, clk_type)
}

#[inline]
pub unsafe fn imx_clk_scu2(
    name: *const c_char,
    parents: *const *const c_char,
    num_parents: c_int,
    rsrc_id: u32,
    clk_type: u8,
) -> *mut clk_hw {
    imx_clk_scu_alloc_dev(name, parents, num_parents, rsrc_id, clk_type)
}

#[inline]
pub unsafe fn imx_clk_lpcg_scu_dev(
    dev: *mut device,
    name: *const c_char,
    parent_name: *const c_char,
    flags: c_ulong,
    reg: *mut c_void,
    bit_idx: u8,
    hw_gate: bool,
) -> *mut clk_hw {
    __imx_clk_lpcg_scu(dev, name, parent_name, flags, reg, bit_idx, hw_gate)
}

#[inline]
pub unsafe fn imx_clk_lpcg_scu(
    name: *const c_char,
    parent_name: *const c_char,
    flags: c_ulong,
    reg: *mut c_void,
    bit_idx: u8,
    hw_gate: bool,
) -> *mut clk_hw {
    __imx_clk_lpcg_scu(core::ptr::null_mut(), name, parent_name, flags, reg, bit_idx, hw_gate)
}

#[inline]
pub unsafe fn imx_clk_gate_gpr_scu(
    name: *const c_char,
    parent_name: *const c_char,
    rsrc_id: u32,
    gpr_id: u8,
    invert: bool,
) -> *mut clk_hw {
    __imx_clk_gpr_scu(name, &parent_name, 1, rsrc_id, gpr_id, IMX_SCU_GPR_CLK_GATE, invert)
}

#[inline]
pub unsafe fn imx_clk_divider_gpr_scu(
    name: *const c_char,
    parent_name: *const c_char,
    rsrc_id: u32,
    gpr_id: u8,
) -> *mut clk_hw {
    __imx_clk_gpr_scu(name, &parent_name, 1, rsrc_id, gpr_id, IMX_SCU_GPR_CLK_DIV, false)
}

#[inline]
pub unsafe fn imx_clk_mux_gpr_scu(
    name: *const c_char,
    parent_names: *const *const c_char,
    num_parents: c_int,
    rsrc_id: u32,
    gpr_id: u8,
) -> *mut clk_hw {
    __imx_clk_gpr_scu(name, parent_names, num_parents, rsrc_id, gpr_id, IMX_SCU_GPR_CLK_MUX, false)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
