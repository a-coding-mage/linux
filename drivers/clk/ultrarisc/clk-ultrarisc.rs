// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2026 UltraRISC Technology (Shanghai) Co., Ltd.
 */

// Linux clock-provider, MMIO, module, platform-device, and local header
// declarations are supplied by external dependencies.

#[repr(C)]
pub struct ultrarisc_pll_clk {
    pub hw: clk_hw,
    pub base: *mut core::ffi::c_void,
    pub layout: *const ultrarisc_pll_layout,
}

#[repr(C)]
pub struct ultrarisc_divider_clk {
    pub divider: clk_divider,
    pub gate: clk_gate,
    pub load_mask: u32,
}

#[inline]
unsafe fn to_ultrarisc_pll_clk(hw: *mut clk_hw) -> *mut ultrarisc_pll_clk {
    (hw as *mut u8).sub(core::mem::offset_of!(ultrarisc_pll_clk, hw)) as *mut ultrarisc_pll_clk
}

#[inline]
unsafe fn to_ultrarisc_divider_clk(hw: *mut clk_hw) -> *mut ultrarisc_divider_clk {
    let divider = to_clk_divider(hw);
    (divider as *mut u8).sub(core::mem::offset_of!(ultrarisc_divider_clk, divider))
        as *mut ultrarisc_divider_clk
}

unsafe fn ultrarisc_pll_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let pll = &*to_ultrarisc_pll_clk(hw);
    let layout = &*pll.layout;
    let cfg1 = readl_relaxed((pll.base as *mut u8).add(layout.cfg1_offset as usize) as *const u32);
    let cfg2 = readl_relaxed((pll.base as *mut u8).add(layout.cfg2_offset as usize) as *const u32);

    let frac = field_get(layout.frac_mask, cfg1);
    let m = field_get(layout.m_mask, cfg2);
    let n = field_get(layout.n_mask, cfg2);
    if n == 0 { return 0; }

    let oddiv1_div = 1u32 << field_get(layout.oddiv1_mask, cfg2);
    let oddiv2_div = 1u32 << field_get(layout.oddiv2_mask, cfg2);

    /*
     * The output frequency is calculated as:
     * fvco = parent * (m + frac / 2^24) / n
     * fout = fvco / (2^oddiv1_raw * 2^oddiv2_raw)
     *
     * The output divider values are derived from the raw register field values as:
     * oddivX_div = 1 << oddivX_raw
     */
    let mult = ((m as u64) << 24) + frac as u64;
    let rate = (parent_rate as u64) * mult;
    let den = ((n as u64) << 24) * oddiv1_div as u64 * oddiv2_div as u64;
    ((rate + (den >> 1)) / den) as usize
}

static ultrarisc_pll_ro_ops: clk_ops = clk_ops {
    recalc_rate: Some(ultrarisc_pll_recalc_rate),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn ultrarisc_divider_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let divider = &*to_clk_divider(hw);
    let mut val = readl_relaxed(divider.reg) >> divider.shift;
    val &= clk_div_mask(divider.width);
    divider_recalc_rate(hw, parent_rate, val, divider.table, divider.flags, divider.width)
}

unsafe fn ultrarisc_divider_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let divider = &*to_clk_divider(hw);
    divider_determine_rate(hw, req, divider.table, divider.width, divider.flags)
}

unsafe fn ultrarisc_divider_set_rate(hw: *mut clk_hw, rate: usize, parent_rate: usize) -> i32 {
    let divider_clk = &mut *to_ultrarisc_divider_clk(hw);
    let divider = &mut divider_clk.divider;
    let value = divider_get_val(rate, parent_rate, divider.table, divider.width, divider.flags);
    if value < 0 { return value; }

    let _guard = spin_lock_irqsave(divider.lock);
    let mut val = readl_relaxed(divider.reg);
    val &= !(clk_div_mask(divider.width) << divider.shift);
    val |= (value as u32) << divider.shift;
    writel_relaxed(val, divider.reg);
    if divider_clk.load_mask != 0 {
        /* Program the divider, then write 1 to the write-triggered load bit. */
        writel_relaxed(val | divider_clk.load_mask, divider.reg);
    }
    0
}

static ultrarisc_divider_ops: clk_ops = clk_ops {
    recalc_rate: Some(ultrarisc_divider_recalc_rate),
    determine_rate: Some(ultrarisc_divider_determine_rate),
    set_rate: Some(ultrarisc_divider_set_rate),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn ultrarisc_clk_register_pll(
    dev: *mut device, desc: *const ultrarisc_pll_desc,
    layout: *const ultrarisc_pll_layout, base: *mut core::ffi::c_void,
) -> *mut clk_hw {
    let pdata = clk_parent_data { index: 0 };
    let init = clk_init_data {
        name: (*desc).name, ops: &ultrarisc_pll_ro_ops, parent_data: &pdata,
        num_parents: 1, flags: CLK_GET_RATE_NOCACHE, ..core::mem::zeroed()
    };
    let pll = devm_kzalloc(dev, core::mem::size_of::<ultrarisc_pll_clk>(), GFP_KERNEL)
        as *mut ultrarisc_pll_clk;
    if pll.is_null() { return ERR_PTR(-ENOMEM); }
    (*pll).base = base; (*pll).layout = layout; (*pll).hw.init = &init;
    let ret = devm_clk_hw_register(dev, &mut (*pll).hw);
    if ret != 0 { return ERR_PTR(ret); }
    &mut (*pll).hw
}

unsafe fn ultrarisc_clk_register_divider(
    dev: *mut device, desc: *const ultrarisc_divider_desc, parent_hw: *mut clk_hw,
    base: *mut core::ffi::c_void, lock: *mut spinlock_t,
) -> *mut clk_hw {
    if (*desc).div_width == 0 || lock.is_null() { return ERR_PTR(-EINVAL); }
    let divider = devm_kzalloc(dev, core::mem::size_of::<ultrarisc_divider_clk>(), GFP_KERNEL)
        as *mut ultrarisc_divider_clk;
    if divider.is_null() { return ERR_PTR(-ENOMEM); }
    let reg = (base as *mut u8).add((*desc).offset as usize) as *mut u32;
    (*divider).divider.reg = reg; (*divider).divider.shift = (*desc).div_shift;
    (*divider).divider.width = (*desc).div_width; (*divider).divider.flags = (*desc).divider_flags;
    (*divider).divider.lock = lock; (*divider).load_mask = (*desc).load_mask;
    (*divider).gate.reg = reg; (*divider).gate.bit_idx = (*desc).gate_bit;
    (*divider).gate.flags = (*desc).gate_flags; (*divider).gate.lock = lock;
    let pdata = clk_parent_data { hw: parent_hw };
    devm_clk_hw_register_composite_pdata(dev, (*desc).name, &pdata, 1, core::ptr::null_mut(),
        core::ptr::null(), &mut (*divider).divider.hw, &ultrarisc_divider_ops,
        &mut (*divider).gate.hw, &clk_gate_ops, 0)
}

unsafe fn ultrarisc_clk_register_fixed_factors(
    dev: *mut device, clk_data: *mut clk_hw_onecell_data,
    soc_data: *const ultrarisc_clk_soc_data,
) -> i32 {
    for i in 0..(*soc_data).num_fixed_factors {
        let desc = &(*soc_data).fixed_factors.add(i as usize);
        if desc.id >= (*clk_data).num || desc.parent_id >= (*clk_data).num { return -EINVAL; }
        let parent_hw = *(*clk_data).hws.add(desc.parent_id as usize);
        if parent_hw.is_null() { return -EINVAL; }
        let hw = devm_clk_hw_register_fixed_factor_parent_hw(dev, desc.name, parent_hw, 0, desc.mult, desc.div);
        if IS_ERR(hw) { return PTR_ERR(hw); }
        *(*clk_data).hws.add(desc.id as usize) = hw;
    }
    0
}

unsafe fn ultrarisc_clk_register_plls(pdev: *mut platform_device, data: *mut clk_hw_onecell_data, soc: *const ultrarisc_clk_soc_data, base: *mut core::ffi::c_void) -> i32 {
    let dev = &mut (*pdev).dev;
    for i in 0..(*soc).num_plls {
        let desc = &*(*soc).plls.add(i as usize);
        if desc.id >= (*data).num { dev_err(dev, desc.name, desc.id, (*data).num); return -EINVAL; }
        let hw = ultrarisc_clk_register_pll(dev, desc, (*soc).pll_layout, base);
        if IS_ERR(hw) { return PTR_ERR(hw); }
        *(*data).hws.add(desc.id as usize) = hw;
    }
    0
}

unsafe fn ultrarisc_clk_register_dividers(pdev: *mut platform_device, data: *mut clk_hw_onecell_data, soc: *const ultrarisc_clk_soc_data, base: *mut core::ffi::c_void, lock: *mut spinlock_t) -> i32 {
    let dev = &mut (*pdev).dev;
    for i in 0..(*soc).num_dividers {
        let desc = &*(*soc).dividers.add(i as usize);
        if desc.id >= (*data).num || desc.parent_id >= (*data).num { return -EINVAL; }
        let parent = *(*data).hws.add(desc.parent_id as usize);
        if parent.is_null() { return -EINVAL; }
        let hw = ultrarisc_clk_register_divider(dev, desc, parent, base, lock);
        if IS_ERR(hw) { return PTR_ERR(hw); }
        if desc.max_rate != 0 { clk_hw_set_rate_range(hw, 0, desc.max_rate); }
        *(*data).hws.add(desc.id as usize) = hw;
    }
    0
}

unsafe fn ultrarisc_clk_register_gates(pdev: *mut platform_device, data: *mut clk_hw_onecell_data, soc: *const ultrarisc_clk_soc_data, base: *mut core::ffi::c_void, lock: *mut spinlock_t) -> i32 {
    let dev = &mut (*pdev).dev;
    for i in 0..(*soc).num_gates {
        let desc = &*(*soc).gates.add(i as usize);
        if desc.id >= (*data).num || desc.parent_id >= (*data).num { return -EINVAL; }
        let parent = *(*data).hws.add(desc.parent_id as usize);
        if parent.is_null() { return -EINVAL; }
        let reg = (base as *mut u8).add(desc.offset as usize) as *mut u32;
        let hw = devm_clk_hw_register_gate_parent_hw(dev, desc.name, parent, 0, reg, desc.gate_bit, desc.gate_flags, lock);
        if IS_ERR(hw) { return PTR_ERR(hw); }
        *(*data).hws.add(desc.id as usize) = hw;
    }
    0
}

pub unsafe fn ultrarisc_clk_probe(pdev: *mut platform_device, soc: *const ultrarisc_clk_soc_data) -> i32 {
    if soc.is_null() { return -EINVAL; }
    let dev = &mut (*pdev).dev;
    let lock = devm_kzalloc(dev, core::mem::size_of::<spinlock_t>(), GFP_KERNEL) as *mut spinlock_t;
    if lock.is_null() { return -ENOMEM; }
    spin_lock_init(lock);
    let data = devm_kzalloc(dev, core::mem::size_of::<clk_hw_onecell_data>() + (*soc).num_clks as usize * core::mem::size_of::<*mut clk_hw>(), GFP_KERNEL) as *mut clk_hw_onecell_data;
    if data.is_null() { return -ENOMEM; }
    (*data).num = (*soc).num_clks;
    let base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) { return PTR_ERR(base); }
    let mut ret = ultrarisc_clk_register_plls(pdev, data, soc, base);
    if ret != 0 { return ret; }
    ret = ultrarisc_clk_register_fixed_factors(dev, data, soc); if ret != 0 { return ret; }
    ret = ultrarisc_clk_register_dividers(pdev, data, soc, base, lock); if ret != 0 { return ret; }
    ret = ultrarisc_clk_register_gates(pdev, data, soc, base, lock); if ret != 0 { return ret; }
    for i in 0..(*data).num { if (*data).hws.add(i as usize).read().is_null() { dev_err(dev, i); return -EINVAL; } }
    devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get, data)
}

// EXPORT_SYMBOL_NS_GPL(ultrarisc_clk_probe, "CLK_ULTRARISC");
// MODULE_DESCRIPTION("UltraRISC clock core driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
