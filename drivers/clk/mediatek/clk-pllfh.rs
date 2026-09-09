// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Edward-JW Yang <edward-jw.yang@mediatek.com>
 */

// Dependencies supplied by the Linux clock, device-tree, I/O, and FHCTL code.

static mut PLLFH_LOCK: SpinLock = DEFINE_SPINLOCK!();

#[inline]
unsafe fn to_mtk_fh(hw: *mut clk_hw) -> *mut mtk_fh {
    let pll = to_mtk_clk_pll(hw);
    container_of!(pll, mtk_fh, clk_pll)
}

unsafe fn mtk_fhctl_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let pll = to_mtk_clk_pll(hw);
    let fh = to_mtk_fh(hw);
    let mut pcw: u32 = 0;
    let mut postdiv: u32 = 0;

    mtk_pll_calc_values(pll, &mut pcw, &mut postdiv, rate, parent_rate);

    ((*(*fh).ops).hopping)(fh, pcw, postdiv)
}

static MTK_PLLFH_OPS: clk_ops = clk_ops {
    is_prepared: Some(mtk_pll_is_prepared),
    prepare: Some(mtk_pll_prepare),
    unprepare: Some(mtk_pll_unprepare),
    recalc_rate: Some(mtk_pll_recalc_rate),
    determine_rate: Some(mtk_pll_determine_rate),
    set_rate: Some(mtk_fhctl_set_rate),
};

unsafe fn get_pllfh_by_id(
    pllfhs: *mut mtk_pllfh_data,
    num_fhs: c_int,
    pll_id: c_int,
) -> *mut mtk_pllfh_data {
    let mut i = 0;
    while i < num_fhs {
        if (*pllfhs.add(i as usize)).data.pll_id == pll_id {
            return pllfhs.add(i as usize);
        }
        i += 1;
    }
    core::ptr::null_mut()
}

unsafe fn fhctl_parse_dt(compatible_node: *const u8, pllfhs: *mut mtk_pllfh_data, num_fhs: c_int) {
    let mut base: *mut core::ffi::c_void;
    let node: *mut device_node;
    let mut num_clocks: u32;
    let mut pll_id: u32 = 0;
    let mut ssc_rate: u32 = 0;
    let mut offset: c_int;
    let mut i: c_int;

    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), compatible_node);
    if node.is_null() {
        pr_warn!("cannot find \"%s\"\n", compatible_node);
        return;
    }

    base = of_iomap(node, 0);
    if base.is_null() {
        pr_err!("%s(): ioremap failed\n", __func__);
        of_node_put(node);
        return;
    }

    num_clocks = of_clk_get_parent_count(node);
    if num_clocks == 0 {
        pr_err!("%s(): failed to get clocks property\n", __func__);
        iounmap(base);
        of_node_put(node);
        return;
    }

    i = 0;
    while i < num_clocks as c_int {
        offset = i * 2;
        of_property_read_u32_index(node, b"clocks\0".as_ptr(), offset + 1, &mut pll_id);
        of_property_read_u32_index(node, b"mediatek,hopping-ssc-percent\0".as_ptr(), i, &mut ssc_rate);

        let pllfh = get_pllfh_by_id(pllfhs, num_fhs, pll_id as c_int);
        if !pllfh.is_null() {
            (*pllfh).state.fh_enable = 1;
            (*pllfh).state.ssc_rate = ssc_rate;
            (*pllfh).state.base = base;
        }
        i += 1;
    }

    of_node_put(node);
}

unsafe fn pllfh_init(fh: *mut mtk_fh, pllfh_data: *mut mtk_pllfh_data) -> c_int {
    let regs = &mut (*fh).regs;
    let base = (*pllfh_data).state.base;
    let fhx_base = (base as *mut u8).add((*pllfh_data).data.fhx_offset as usize) as *mut core::ffi::c_void;
    let offset = fhctl_get_offset_table((*pllfh_data).data.fh_ver);
    if IS_ERR!(offset) { return PTR_ERR!(offset); }

    regs.reg_hp_en = (base as *mut u8).add((*offset).offset_hp_en as usize) as _;
    regs.reg_clk_con = (base as *mut u8).add((*offset).offset_clk_con as usize) as _;
    regs.reg_rst_con = (base as *mut u8).add((*offset).offset_rst_con as usize) as _;
    regs.reg_slope0 = (base as *mut u8).add((*offset).offset_slope0 as usize) as _;
    regs.reg_slope1 = (base as *mut u8).add((*offset).offset_slope1 as usize) as _;
    regs.reg_cfg = (fhx_base as *mut u8).add((*offset).offset_cfg as usize) as _;
    regs.reg_updnlmt = (fhx_base as *mut u8).add((*offset).offset_updnlmt as usize) as _;
    regs.reg_dds = (fhx_base as *mut u8).add((*offset).offset_dds as usize) as _;
    regs.reg_dvfs = (fhx_base as *mut u8).add((*offset).offset_dvfs as usize) as _;
    regs.reg_mon = (fhx_base as *mut u8).add((*offset).offset_mon as usize) as _;
    (*fh).pllfh_data = pllfh_data;
    (*fh).lock = core::ptr::addr_of_mut!(PLLFH_LOCK);
    (*fh).ops = fhctl_get_ops();
    0
}

unsafe fn fhctl_is_supported_and_enabled(pllfh: *const mtk_pllfh_data) -> bool {
    !pllfh.is_null() && (*pllfh).state.fh_enable == 1
}

unsafe fn mtk_clk_register_pllfh(dev: *mut device, pll_data: *const mtk_pll_data, pllfh_data: *mut mtk_pllfh_data, base: *mut core::ffi::c_void) -> *mut clk_hw {
    let fh = kzalloc_obj!(mtk_fh);
    if fh.is_null() { return ERR_PTR!(-ENOMEM); }
    let ret = pllfh_init(fh, pllfh_data);
    if ret != 0 { kfree!(fh); return ERR_PTR!(ret); }
    (*fh).clk_pll.dev = dev;
    let hw = mtk_clk_register_pll_ops(&mut (*fh).clk_pll, pll_data, base, &MTK_PLLFH_OPS);
    if IS_ERR!(hw) { kfree!(fh); return hw; }
    fhctl_hw_init(fh);
    hw
}

unsafe fn mtk_clk_unregister_pllfh(hw: *mut clk_hw) {
    if hw.is_null() { return; }
    let fh = to_mtk_fh(hw);
    clk_hw_unregister(hw);
    kfree!(fh);
}

unsafe fn mtk_clk_cleanup_pllfhs(iomem_base: *mut core::ffi::c_void, plls: *const mtk_pll_data, num_plls: c_int, iomem_fhctl_base: *mut core::ffi::c_void, pllfhs: *mut mtk_pllfh_data, num_fhs: c_int, clk_data: *mut clk_hw_onecell_data) {
    let mut base = iomem_base;
    let mut fhctl_base = iomem_fhctl_base;
    let mut i = num_plls - 1;
    while i >= 0 {
        let pll = plls.add(i as usize);
        let pllfh = get_pllfh_by_id(pllfhs, num_fhs, (*pll).id);
        if IS_ERR_OR_NULL!((*clk_data).hws[(*pll).id as usize]) { i -= 1; continue; }
        let use_fhctl = fhctl_is_supported_and_enabled(pllfh);
        if base.is_null() { base = mtk_clk_pll_get_base((*clk_data).hws[(*pll).id as usize], pll); }
        if use_fhctl {
            if fhctl_base.is_null() { fhctl_base = (*pllfh).state.base; }
            mtk_clk_unregister_pllfh((*clk_data).hws[(*pll).id as usize]);
        } else { mtk_clk_unregister_pll((*clk_data).hws[(*pll).id as usize]); }
        (*clk_data).hws[(*pll).id as usize] = ERR_PTR!(-ENOENT);
        i -= 1;
    }
    if !fhctl_base.is_null() { iounmap(fhctl_base); }
    if !base.is_null() { iounmap(base); }
}

unsafe fn mtk_clk_register_pllfhs(dev: *mut device, plls: *const mtk_pll_data, num_plls: c_int, pllfhs: *mut mtk_pllfh_data, num_fhs: c_int, clk_data: *mut clk_hw_onecell_data) -> c_int {
    let base = of_iomap((*dev).of_node, 0);
    if base.is_null() { pr_err!("%s(): ioremap failed\n", __func__); return -EINVAL; }
    let mut i = 0;
    while i < num_plls {
        let pll = plls.add(i as usize);
        let pllfh = get_pllfh_by_id(pllfhs, num_fhs, (*pll).id);
        let use_fhctl = fhctl_is_supported_and_enabled(pllfh);
        let hw = if use_fhctl { mtk_clk_register_pllfh(dev, pll, pllfh, base) } else { mtk_clk_register_pll(dev, pll, base) };
        if IS_ERR!(hw) {
            pr_err!("Failed to register %s clk %s: %ld\n", if use_fhctl { b"fhpll\0".as_ptr() } else { b"pll\0".as_ptr() }, (*pll).name, PTR_ERR!(hw));
            mtk_clk_cleanup_pllfhs(base, plls, i, core::ptr::null_mut(), pllfhs, num_fhs, clk_data);
            return PTR_ERR!(hw);
        }
        (*clk_data).hws[(*pll).id as usize] = hw;
        i += 1;
    }
    0
}

unsafe fn mtk_clk_unregister_pllfhs(plls: *const mtk_pll_data, num_plls: c_int, pllfhs: *mut mtk_pllfh_data, num_fhs: c_int, clk_data: *mut clk_hw_onecell_data) {
    if clk_data.is_null() { return; }
    mtk_clk_cleanup_pllfhs(core::ptr::null_mut(), plls, num_plls, core::ptr::null_mut(), pllfhs, num_fhs, clk_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
