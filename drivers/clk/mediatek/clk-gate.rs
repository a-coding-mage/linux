// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: James Liao <jamesjj.liao@mediatek.com>
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct mtk_clk_gate {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub regmap_hwv: *mut regmap,
    pub gate: *const mtk_gate,
}

unsafe fn to_mtk_clk_gate(hw: *mut clk_hw) -> *mut mtk_clk_gate {
    hw as *mut mtk_clk_gate
}

unsafe fn mtk_get_clockgating(hw: *mut clk_hw) -> u32 {
    let cg = to_mtk_clk_gate(hw);
    let mut val: u32 = 0;
    regmap_read((*cg).regmap, (*(*cg).gate).regs.as_ref().unwrap().sta_ofs, &mut val);
    val & (1u32 << (*(*cg).gate).shift)
}

unsafe fn mtk_cg_bit_is_cleared(hw: *mut clk_hw) -> i32 { (mtk_get_clockgating(hw) == 0) as i32 }
unsafe fn mtk_cg_bit_is_set(hw: *mut clk_hw) -> i32 { (mtk_get_clockgating(hw) != 0) as i32 }

unsafe fn mtk_cg_set_bit(hw: *mut clk_hw) {
    let cg = to_mtk_clk_gate(hw);
    regmap_write((*cg).regmap, (*(*cg).gate).regs.as_ref().unwrap().set_ofs, 1u32 << (*(*cg).gate).shift);
}
unsafe fn mtk_cg_clr_bit(hw: *mut clk_hw) {
    let cg = to_mtk_clk_gate(hw);
    regmap_write((*cg).regmap, (*(*cg).gate).regs.as_ref().unwrap().clr_ofs, 1u32 << (*(*cg).gate).shift);
}
unsafe fn mtk_cg_set_bit_no_setclr(hw: *mut clk_hw) {
    let cg = to_mtk_clk_gate(hw);
    regmap_set_bits((*cg).regmap, (*(*cg).gate).regs.as_ref().unwrap().sta_ofs, 1u32 << (*(*cg).gate).shift);
}
unsafe fn mtk_cg_clr_bit_no_setclr(hw: *mut clk_hw) {
    let cg = to_mtk_clk_gate(hw);
    regmap_clear_bits((*cg).regmap, (*(*cg).gate).regs.as_ref().unwrap().sta_ofs, 1u32 << (*(*cg).gate).shift);
}

unsafe fn mtk_cg_enable(hw: *mut clk_hw) -> i32 { mtk_cg_clr_bit(hw); 0 }
unsafe fn mtk_cg_disable(hw: *mut clk_hw) { mtk_cg_set_bit(hw); }
unsafe fn mtk_cg_enable_inv(hw: *mut clk_hw) -> i32 { mtk_cg_set_bit(hw); 0 }
unsafe fn mtk_cg_disable_inv(hw: *mut clk_hw) { mtk_cg_clr_bit(hw); }

unsafe fn mtk_cg_hwv_set_en(hw: *mut clk_hw, enable: bool) -> i32 {
    let cg = to_mtk_clk_gate(hw);
    let mut val: u32 = 0;
    let regs = (*(*cg).gate).hwv_regs.as_ref().unwrap();
    regmap_write((*cg).regmap_hwv, if enable { regs.set_ofs } else { regs.clr_ofs }, 1u32 << (*(*cg).gate).shift);
    regmap_read_poll_timeout_atomic((*cg).regmap_hwv, regs.sta_ofs, &mut val, val & (1u32 << (*(*cg).gate).shift), 0, MTK_WAIT_HWV_DONE_US)
}
unsafe fn mtk_cg_hwv_enable(hw: *mut clk_hw) -> i32 { mtk_cg_hwv_set_en(hw, true) }
unsafe fn mtk_cg_hwv_disable(hw: *mut clk_hw) { mtk_cg_hwv_set_en(hw, false); }
unsafe fn mtk_cg_enable_no_setclr(hw: *mut clk_hw) -> i32 { mtk_cg_clr_bit_no_setclr(hw); 0 }
unsafe fn mtk_cg_disable_no_setclr(hw: *mut clk_hw) { mtk_cg_set_bit_no_setclr(hw); }
unsafe fn mtk_cg_enable_inv_no_setclr(hw: *mut clk_hw) -> i32 { mtk_cg_set_bit_no_setclr(hw); 0 }
unsafe fn mtk_cg_disable_inv_no_setclr(hw: *mut clk_hw) { mtk_cg_clr_bit_no_setclr(hw); }

unsafe fn mtk_cg_uses_hwv(ops: *const clk_ops) -> bool {
    ops == &mtk_clk_gate_hwv_ops_setclr || ops == &mtk_clk_gate_hwv_ops_setclr_inv
}

pub static mtk_clk_gate_ops_setclr: clk_ops = clk_ops { is_enabled: Some(mtk_cg_bit_is_cleared), enable: Some(mtk_cg_enable), disable: Some(mtk_cg_disable) };
pub static mtk_clk_gate_ops_setclr_inv: clk_ops = clk_ops { is_enabled: Some(mtk_cg_bit_is_set), enable: Some(mtk_cg_enable_inv), disable: Some(mtk_cg_disable_inv) };
pub static mtk_clk_gate_hwv_ops_setclr: clk_ops = clk_ops { is_enabled: Some(mtk_cg_bit_is_cleared), enable: Some(mtk_cg_hwv_enable), disable: Some(mtk_cg_hwv_disable) };
pub static mtk_clk_gate_hwv_ops_setclr_inv: clk_ops = clk_ops { is_enabled: Some(mtk_cg_bit_is_set), enable: Some(mtk_cg_hwv_enable), disable: Some(mtk_cg_hwv_disable) };
pub static mtk_clk_gate_ops_no_setclr: clk_ops = clk_ops { is_enabled: Some(mtk_cg_bit_is_cleared), enable: Some(mtk_cg_enable_no_setclr), disable: Some(mtk_cg_disable_no_setclr) };
pub static mtk_clk_gate_ops_no_setclr_inv: clk_ops = clk_ops { is_enabled: Some(mtk_cg_bit_is_set), enable: Some(mtk_cg_enable_inv_no_setclr), disable: Some(mtk_cg_disable_inv_no_setclr) };

unsafe fn mtk_clk_register_gate(dev: *mut device, gate: *const mtk_gate, regmap: *mut regmap, regmap_hwv: *mut regmap) -> *mut clk_hw {
    let cg = kzalloc_obj_mtk_clk_gate();
    if cg.is_null() { return ERR_PTR(-ENOMEM); }
    let mut init: clk_init_data = core::mem::zeroed();
    (*cg).regmap = regmap; (*cg).regmap_hwv = regmap_hwv; (*cg).gate = gate;
    init.name = (*gate).name; init.flags = (*gate).flags | CLK_SET_RATE_PARENT;
    init.parent_names = if !(*gate).parent_name.is_null() { &(*gate).parent_name } else { core::ptr::null() };
    init.num_parents = if !(*gate).parent_name.is_null() { 1 } else { 0 }; init.ops = (*gate).ops;
    if mtk_cg_uses_hwv(init.ops) && regmap_hwv.is_null() { return dev_err_ptr_probe(dev, -ENXIO, b"regmap not found for hardware voter clocks\0".as_ptr() as *const i8); }
    (*cg).hw.init = &init;
    let ret = clk_hw_register(dev, &mut (*cg).hw); if ret != 0 { kfree(cg as *mut core::ffi::c_void); return ERR_PTR(ret); } &mut (*cg).hw
}

unsafe fn mtk_clk_unregister_gate(hw: *mut clk_hw) { if hw.is_null() { return; } let cg = to_mtk_clk_gate(hw); clk_hw_unregister(hw); kfree(cg as *mut core::ffi::c_void); }

pub unsafe fn mtk_clk_register_gates(dev: *mut device, node: *mut device_node, clks: *const mtk_gate, num: i32, clk_data: *mut clk_hw_onecell_data) -> i32 {
    if clk_data.is_null() { return -ENOMEM; }
    let regmap = device_node_to_regmap(node); if IS_ERR(regmap) { pr_err(b"Cannot find regmap\0".as_ptr() as *const i8); return PTR_ERR(regmap); }
    let regmap_hwv = mtk_clk_get_hwv_regmap(node); if IS_ERR(regmap_hwv) { return dev_err_probe(dev, PTR_ERR(regmap_hwv), b"Cannot find hardware voter regmap\0".as_ptr() as *const i8); }
    let mut i = 0; while i < num { let gate = clks.add(i as usize); let hw = mtk_clk_register_gate(dev, gate, regmap, regmap_hwv); if IS_ERR(hw) { while i > 0 { i -= 1; mtk_clk_unregister_gate((*clk_data).hws.add((*clks.add(i as usize)).id as usize).read()); } return PTR_ERR(hw); } (*clk_data).hws.add((*gate).id as usize).write(hw); i += 1; } 0
}

pub unsafe fn mtk_clk_unregister_gates(clks: *const mtk_gate, num: i32, clk_data: *mut clk_hw_onecell_data) { if clk_data.is_null() { return; } let mut i = num; while i > 0 { i -= 1; let hw = (*clk_data).hws.add((*clks.add(i as usize)).id as usize).read(); if !hw.is_null() { mtk_clk_unregister_gate(hw); (*clk_data).hws.add((*clks.add(i as usize)).id as usize).write(ERR_PTR(-ENOENT)); } } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
