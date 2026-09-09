// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 MediaTek Inc.
 * Author: Owen Chen <owen.chen@mediatek.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const MTK_WAIT_FENC_DONE_US: u32 = 30;

#[repr(C)]
pub struct MtkClkMux {
    pub hw: ClkHw,
    pub regmap: *mut Regmap,
    pub regmap_hwv: *mut Regmap,
    pub data: *const MtkMux,
    pub lock: *mut SpinlockT,
    pub reparent: bool,
}

#[inline]
unsafe fn to_mtk_clk_mux(hw: *mut ClkHw) -> *mut MtkClkMux {
    (hw as *mut u8).sub(core::mem::offset_of!(MtkClkMux, hw)) as *mut MtkClkMux
}

unsafe fn mtk_clk_mux_fenc_enable_setclr(hw: *mut ClkHw) -> i32 {
    let mux = &mut *to_mtk_clk_mux(hw);
    let mut flags: CUnsignedLong = 0;
    let mut val: u32 = 0;
    let ret: i32;
    if !mux.lock.is_null() { spin_lock_irqsave(mux.lock, &mut flags); } else { __acquire(mux.lock); }
    regmap_write(mux.regmap, (*mux.data).clr_ofs, BIT((*mux.data).gate_shift));
    ret = regmap_read_poll_timeout_atomic(mux.regmap, (*mux.data).fenc_sta_mon_ofs, &mut val, val & BIT((*mux.data).fenc_shift), 1, MTK_WAIT_FENC_DONE_US);
    if !mux.lock.is_null() { spin_unlock_irqrestore(mux.lock, flags); } else { __release(mux.lock); }
    ret
}

unsafe fn mtk_clk_mux_enable_setclr(hw: *mut ClkHw) -> i32 {
    let mux = &mut *to_mtk_clk_mux(hw);
    let mut flags: CUnsignedLong = 0;
    if !mux.lock.is_null() { spin_lock_irqsave(mux.lock, &mut flags); } else { __acquire(mux.lock); }
    regmap_write(mux.regmap, (*mux.data).clr_ofs, BIT((*mux.data).gate_shift));
    if mux.reparent && (*mux.data).upd_shift >= 0 {
        regmap_write(mux.regmap, (*mux.data).upd_ofs, BIT((*mux.data).upd_shift));
        mux.reparent = false;
    }
    if !mux.lock.is_null() { spin_unlock_irqrestore(mux.lock, flags); } else { __release(mux.lock); }
    0
}

unsafe fn mtk_clk_mux_disable_setclr(hw: *mut ClkHw) {
    let mux = &*to_mtk_clk_mux(hw);
    regmap_write(mux.regmap, (*mux.data).set_ofs, BIT((*mux.data).gate_shift));
}

unsafe fn mtk_clk_mux_fenc_is_enabled(hw: *mut ClkHw) -> i32 {
    let mux = &*to_mtk_clk_mux(hw); let mut val = 0;
    regmap_read(mux.regmap, (*mux.data).fenc_sta_mon_ofs, &mut val);
    !!(val & BIT((*mux.data).fenc_shift)) as i32
}

unsafe fn mtk_clk_mux_is_enabled(hw: *mut ClkHw) -> i32 {
    let mux = &*to_mtk_clk_mux(hw); let mut val = 0;
    regmap_read(mux.regmap, (*mux.data).mux_ofs, &mut val);
    ((val & BIT((*mux.data).gate_shift)) == 0) as i32
}

unsafe fn mtk_clk_mux_hwv_fenc_enable(hw: *mut ClkHw) -> i32 {
    let mux = &*to_mtk_clk_mux(hw); let mut val = 0;
    regmap_write(mux.regmap_hwv, (*mux.data).hwv_set_ofs, BIT((*mux.data).gate_shift));
    let mut ret = regmap_read_poll_timeout_atomic(mux.regmap_hwv, (*mux.data).hwv_sta_ofs, &mut val, val & BIT((*mux.data).gate_shift), 0, MTK_WAIT_HWV_DONE_US);
    if ret != 0 { return ret; }
    ret = regmap_read_poll_timeout_atomic(mux.regmap, (*mux.data).fenc_sta_mon_ofs, &mut val, val & BIT((*mux.data).fenc_shift), 1, MTK_WAIT_FENC_DONE_US);
    ret
}

unsafe fn mtk_clk_mux_hwv_disable(hw: *mut ClkHw) {
    let mux = &*to_mtk_clk_mux(hw); let mut val = 0;
    regmap_write(mux.regmap_hwv, (*mux.data).hwv_clr_ofs, BIT((*mux.data).gate_shift));
    regmap_read_poll_timeout_atomic(mux.regmap_hwv, (*mux.data).hwv_sta_ofs, &mut val, val & BIT((*mux.data).gate_shift), 0, MTK_WAIT_HWV_DONE_US);
}

unsafe fn mtk_clk_mux_get_parent(hw: *mut ClkHw) -> u8 {
    let mux = &*to_mtk_clk_mux(hw); let mask = GENMASK((*mux.data).mux_width - 1, 0); let mut val = 0;
    regmap_read(mux.regmap, (*mux.data).mux_ofs, &mut val); val = (val >> (*mux.data).mux_shift) & mask;
    if !(*mux.data).parent_index.is_null() { for i in 0..(*mux.data).num_parents { if *(*mux.data).parent_index.add(i as usize) == val { return i as u8; } } return ((*mux.data).num_parents + 1) as u8; }
    val as u8
}

unsafe fn mtk_clk_mux_set_parent_setclr_lock(hw: *mut ClkHw, mut index: u8) -> i32 {
    let mux = &mut *to_mtk_clk_mux(hw); let mask = GENMASK((*mux.data).mux_width - 1, 0); let (mut val, mut orig) = (0, 0); let mut flags = 0;
    if !mux.lock.is_null() { spin_lock_irqsave(mux.lock, &mut flags); } else { __acquire(mux.lock); }
    if !(*mux.data).parent_index.is_null() { index = *(*mux.data).parent_index.add(index as usize) as u8; }
    regmap_read(mux.regmap, (*mux.data).mux_ofs, &mut orig); val = (orig & !(mask << (*mux.data).mux_shift)) | ((index as u32) << (*mux.data).mux_shift);
    if val != orig { regmap_write(mux.regmap, (*mux.data).clr_ofs, mask << (*mux.data).mux_shift); regmap_write(mux.regmap, (*mux.data).set_ofs, (index as u32) << (*mux.data).mux_shift); if (*mux.data).upd_shift >= 0 { regmap_write(mux.regmap, (*mux.data).upd_ofs, BIT((*mux.data).upd_shift)); mux.reparent = true; } }
    if !mux.lock.is_null() { spin_unlock_irqrestore(mux.lock, flags); } else { __release(mux.lock); } 0
}

unsafe fn mtk_clk_mux_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 { clk_mux_determine_rate_flags(hw, req, 0) }
unsafe fn mtk_clk_mux_uses_hwv(ops: *const ClkOps) -> bool { ops == &mtk_mux_gate_hwv_fenc_clr_set_upd_ops }

#[no_mangle] pub static mtk_mux_clr_set_upd_ops: ClkOps = ClkOps { get_parent: Some(mtk_clk_mux_get_parent), set_parent: Some(mtk_clk_mux_set_parent_setclr_lock), determine_rate: Some(mtk_clk_mux_determine_rate), ..ClkOps::EMPTY };
#[no_mangle] pub static mtk_mux_gate_clr_set_upd_ops: ClkOps = ClkOps { enable: Some(mtk_clk_mux_enable_setclr), disable: Some(mtk_clk_mux_disable_setclr), is_enabled: Some(mtk_clk_mux_is_enabled), get_parent: Some(mtk_clk_mux_get_parent), set_parent: Some(mtk_clk_mux_set_parent_setclr_lock), determine_rate: Some(mtk_clk_mux_determine_rate), ..ClkOps::EMPTY };
#[no_mangle] pub static mtk_mux_gate_fenc_clr_set_upd_ops: ClkOps = ClkOps { enable: Some(mtk_clk_mux_fenc_enable_setclr), disable: Some(mtk_clk_mux_disable_setclr), is_enabled: Some(mtk_clk_mux_fenc_is_enabled), get_parent: Some(mtk_clk_mux_get_parent), set_parent: Some(mtk_clk_mux_set_parent_setclr_lock), determine_rate: Some(mtk_clk_mux_determine_rate), ..ClkOps::EMPTY };
#[no_mangle] pub static mtk_mux_gate_hwv_fenc_clr_set_upd_ops: ClkOps = ClkOps { enable: Some(mtk_clk_mux_hwv_fenc_enable), disable: Some(mtk_clk_mux_hwv_disable), is_enabled: Some(mtk_clk_mux_fenc_is_enabled), get_parent: Some(mtk_clk_mux_get_parent), set_parent: Some(mtk_clk_mux_set_parent_setclr_lock), determine_rate: Some(mtk_clk_mux_determine_rate), ..ClkOps::EMPTY };

unsafe fn mtk_clk_register_mux(dev: *mut Device, mux: *const MtkMux, regmap: *mut Regmap, regmap_hwv: *mut Regmap, lock: *mut SpinlockT) -> *mut ClkHw {
    let clk_mux = kzalloc_obj::<MtkClkMux>(); if clk_mux.is_null() { return ERR_PTR(-ENOMEM); }
    let mut init: ClkInitData = core::mem::zeroed(); init.name = (*mux).name; init.flags = (*mux).flags; init.parent_names = (*mux).parent_names; init.num_parents = (*mux).num_parents; init.ops = (*mux).ops;
    if mtk_clk_mux_uses_hwv(init.ops) && regmap_hwv.is_null() { return dev_err_ptr_probe(dev, -ENXIO, c"regmap not found for hardware voter clocks\0".as_ptr()); }
    (*clk_mux).regmap = regmap; (*clk_mux).regmap_hwv = regmap_hwv; (*clk_mux).data = mux; (*clk_mux).lock = lock; (*clk_mux).hw.init = &mut init;
    let ret = clk_hw_register(dev, &mut (*clk_mux).hw); if ret != 0 { kfree(clk_mux); return ERR_PTR(ret); } &mut (*clk_mux).hw
}

unsafe fn mtk_clk_unregister_mux(hw: *mut ClkHw) { if hw.is_null() { return; } let mux = to_mtk_clk_mux(hw); clk_hw_unregister(hw); kfree(mux); }

#[no_mangle] pub unsafe extern "C" fn mtk_clk_register_muxes(dev: *mut Device, muxes: *const MtkMux, num: i32, node: *mut DeviceNode, lock: *mut SpinlockT, clk_data: *mut ClkHwOnecellData) -> i32 {
    let regmap = device_node_to_regmap(node); if IS_ERR(regmap) { return PTR_ERR(regmap); }
    let regmap_hwv = mtk_clk_get_hwv_regmap(node); if IS_ERR(regmap_hwv) { return PTR_ERR(regmap_hwv); }
    let mut i = 0; while i < num { let mux = &*muxes.add(i as usize); if !IS_ERR_OR_NULL((*clk_data).hws[mux.id as usize]) { i += 1; continue; } let hw = mtk_clk_register_mux(dev, mux, regmap, regmap_hwv, lock); if IS_ERR(hw) { while i > 0 { i -= 1; let m = &*muxes.add(i as usize); if !IS_ERR_OR_NULL((*clk_data).hws[m.id as usize]) { mtk_clk_unregister_mux((*clk_data).hws[m.id as usize]); (*clk_data).hws[m.id as usize] = ERR_PTR(-ENOENT); } } return PTR_ERR(hw); } (*clk_data).hws[mux.id as usize] = hw; i += 1; } 0
}

#[no_mangle] pub unsafe extern "C" fn mtk_clk_unregister_muxes(muxes: *const MtkMux, num: i32, clk_data: *mut ClkHwOnecellData) { if clk_data.is_null() { return; } let mut i = num; while i > 0 { i -= 1; let mux = &*muxes.add(i as usize); if !IS_ERR_OR_NULL((*clk_data).hws[mux.id as usize]) { mtk_clk_unregister_mux((*clk_data).hws[mux.id as usize]); (*clk_data).hws[mux.id as usize] = ERR_PTR(-ENOENT); } } }

unsafe fn mtk_clk_mux_notifier_cb(nb: *mut NotifierBlock, event: CUnsignedLong, data: *mut core::ffi::c_void) -> i32 { let data = data as *mut ClkNotifierData; let hw = __clk_get_hw((*data).clk); let mux_nb = to_mtk_mux_nb(nb); let mut ret = 0; match event { PRE_RATE_CHANGE => { (*mux_nb).original_index = ((*mux_nb).ops).get_parent.unwrap()(hw); ret = ((*mux_nb).ops).set_parent.unwrap()(hw, (*mux_nb).bypass_index); }, POST_RATE_CHANGE | ABORT_RATE_CHANGE => ret = ((*mux_nb).ops).set_parent.unwrap()(hw, (*mux_nb).original_index), _ => {} } notifier_from_errno(ret) }

#[no_mangle] pub unsafe extern "C" fn devm_mtk_clk_mux_notifier_register(dev: *mut Device, clk: *mut Clk, mux_nb: *mut MtkMuxNb) -> i32 { (*mux_nb).nb.notifier_call = Some(mtk_clk_mux_notifier_cb); devm_clk_notifier_register(dev, clk, &mut (*mux_nb).nb) }

// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
