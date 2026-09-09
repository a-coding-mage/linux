// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2026, Beijing ESWIN Computing Technology Co., Ltd..
 * All rights reserved.
 *
 * Authors:
 *\tYifeng Huang <huangyifeng@eswincomputing.com>
 *\tXuyang Dong <dongxuyang@eswincomputing.com>
 */

// Dependencies supplied by the Linux kernel and common project headers are
// intentionally referenced but not defined in this translation unit.

const PLL_EN_MASK: u32 = 0x3;
const PLL_REFDIV_MASK: u32 = 0x3f000;
const PLL_FBDIV_MASK: u32 = 0xfff00000;
const PLL_FRAC_MASK: u32 = 0x0ffffff0;
const PLL_POSTDIV1_MASK: u32 = 0x700;
const PLL_POSTDIV2_MASK: u32 = 0x70000;

pub unsafe fn eswin_clk_init(pdev: *mut platform_device, nr_clks: usize) -> *mut eswin_clock_data {
    let eclk_data = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<eswin_clock_data>()
            + nr_clks * core::mem::size_of::<*mut clk_hw>(),
        GFP_KERNEL,
    ) as *mut eswin_clock_data;
    if eclk_data.is_null() { return ERR_PTR(-ENOMEM); }
    (*eclk_data).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*eclk_data).base) { return ERR_PTR(-EINVAL); }
    (*eclk_data).clk_data.num = nr_clks;
    spin_lock_init(&mut (*eclk_data).lock);
    eclk_data
}

unsafe fn eswin_calc_pll(frac_val: *mut u32, fbdiv_val: *mut u32, rate: usize, parent_rate: usize) {
    let mut tmp = rate.wrapping_mul(4) as u64;
    let rem = (tmp % parent_rate as u64) as u32;
    tmp /= parent_rate as u64;
    *fbdiv_val = tmp as u32;
    tmp = (rem as u64) << 24;
    tmp /= parent_rate as u64;
    *frac_val = tmp as u32;
}

unsafe fn to_pll_clk(hw: *mut clk_hw) -> *mut eswin_clk_pll {
    (hw as *mut u8).sub(core::mem::offset_of!(eswin_clk_pll, hw)) as *mut eswin_clk_pll
}

unsafe fn clk_pll_set_rate(hw: *mut clk_hw, rate: usize, parent_rate: usize) -> i32 {
    let clk = to_pll_clk(hw); let mut frac_val = 0; let mut fbdiv_val = 0; let mut val; let mask;
    eswin_calc_pll(&mut frac_val, &mut fbdiv_val, rate, parent_rate);
    val = readl_relaxed((*clk).ctrl_reg0); val &= !PLL_EN_MASK; writel_relaxed(val, (*clk).ctrl_reg0);
    val = readl_relaxed((*clk).ctrl_reg0); val &= !(PLL_REFDIV_MASK | PLL_FBDIV_MASK);
    val |= (fbdiv_val << 20) & PLL_FBDIV_MASK; val |= 1 << 12; writel_relaxed(val, (*clk).ctrl_reg0);
    val = readl_relaxed((*clk).ctrl_reg1); val &= !PLL_FRAC_MASK; val |= (frac_val << 4) & PLL_FRAC_MASK; writel_relaxed(val, (*clk).ctrl_reg1);
    val = readl_relaxed((*clk).ctrl_reg2); val &= !(PLL_POSTDIV1_MASK | PLL_POSTDIV2_MASK); val |= 1 << 8; val |= 1 << 16; writel_relaxed(val, (*clk).ctrl_reg2);
    val = readl_relaxed((*clk).ctrl_reg0); val &= !PLL_EN_MASK; val |= 1; writel_relaxed(val, (*clk).ctrl_reg0);
    mask = ((1u32 << ((*clk).lock_shift + (*clk).lock_width)) - 1) & !((1u32 << (*clk).lock_shift) - 1);
    let ret = readl_poll_timeout((*clk).status_reg, &mut val, val & mask, 1, 50 * 2);
    if ret != 0 { pr_err("failed to lock the pll!\n"); } ret
}

unsafe fn clk_pll_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let clk = to_pll_clk(hw); let mut val = readl_relaxed((*clk).ctrl_reg0) & PLL_FBDIV_MASK;
    let fbdiv_val = (val >> (*clk).fbdiv_shift) as u64;
    val = readl_relaxed((*clk).ctrl_reg1) & PLL_FRAC_MASK;
    let frac_val = (val >> (*clk).frac_shift) as u64;
    let tmp = parent_rate as u64 * frac_val; let q = tmp / (1 << 24); let rem = tmp % (1 << 24);
    (parent_rate as u64 * fbdiv_val + q + if rem != 0 { 1 } else { 0 }) as usize / 4
}

unsafe fn clk_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let clk = to_pll_clk(hw); (*req).rate = (*req).rate.clamp((*clk).min_rate, (*clk).max_rate);
    (*req).min_rate = (*clk).min_rate; (*req).max_rate = (*clk).max_rate; 0
}

pub unsafe fn eswin_clk_register_fixed_rate(dev: *mut device, clks: *mut eswin_fixed_rate_clock, nums: i32, data: *mut eswin_clock_data) -> i32 {
    for i in 0..nums { let c = &mut *clks.add(i as usize); let hw = devm_clk_hw_register_fixed_rate(dev, c.name, core::ptr::null(), c.flags, c.rate); if IS_ERR(hw) { return PTR_ERR(hw); } c.hw = *hw; (*data).clk_data.hws[c.id] = hw; } 0
}

pub unsafe fn eswin_clk_register_pll(dev: *mut device, clks: *mut eswin_pll_clock, nums: i32, data: *mut eswin_clock_data) -> i32 {
    let p_clk = devm_kzalloc(dev, core::mem::size_of::<eswin_clk_pll>() * nums as usize, GFP_KERNEL) as *mut eswin_clk_pll;
    if p_clk.is_null() { return -ENOMEM; }
    for i in 0..nums { let c = &mut *clks.add(i as usize); let p = &mut *p_clk.add(i as usize);
        p.id=c.id; p.ctrl_reg0=(*data).base.add(c.ctrl_reg0); p.fbdiv_shift=c.fbdiv_shift; p.ctrl_reg1=(*data).base.add(c.ctrl_reg1); p.frac_shift=c.frac_shift; p.ctrl_reg2=(*data).base.add(c.ctrl_reg2); p.status_reg=(*data).base.add(c.status_reg); p.lock_shift=c.lock_shift; p.lock_width=c.lock_width; p.max_rate=c.max_rate; p.min_rate=c.min_rate;
        let hw=&mut p.hw; let ret=devm_clk_hw_register(dev, hw); if ret != 0 { return ret; } c.hw=*hw; (*data).clk_data.hws[c.id]=hw;
    } 0
}
pub unsafe fn eswin_clk_register_fixed_factor(dev: *mut device, clks: *mut eswin_fixed_factor_clock, nums: i32, data: *mut eswin_clock_data) -> i32 {
    for i in 0..nums { let c=&mut *clks.add(i as usize); let hw=devm_clk_hw_register_fixed_factor_index(dev,c.name,c.parent_data.index,c.flags,c.mult,c.div); if IS_ERR(hw){return PTR_ERR(hw)} c.hw=*hw; (*data).clk_data.hws[c.id]=hw; } 0
}
pub unsafe fn eswin_clk_register_mux(dev: *mut device, clks: *mut eswin_mux_clock, nums: i32, data: *mut eswin_clock_data) -> i32 {
    for i in 0..nums { let c=&mut *clks.add(i as usize); let hw=devm_clk_hw_register_mux_parent_data_table(dev,c.name,c.parent_data,c.num_parents,c.flags,(*data).base.add(c.reg),c.shift,c.width,c.mux_flags,c.table,&mut (*data).lock); if IS_ERR(hw){return PTR_ERR(hw)} c.hw=*hw; (*data).clk_data.hws[c.id]=hw; } 0
}

unsafe fn _eswin_get_val(mut div: u32, flags: usize, width: u8) -> u32 { let maxdiv = clk_div_mask(width); if div > maxdiv { div = maxdiv; } if flags & ESWIN_PRIV_DIV_MIN_2 != 0 && div < 2 { 2 } else { div } }
unsafe fn eswin_div_get_val(rate: usize, parent_rate: usize, width: u8, flags: usize) -> u32 { _eswin_get_val(((parent_rate as u64 + rate as u64 - 1) / rate as u64) as u32, flags, width) }

pub unsafe fn eswin_register_clkdiv(dev: *mut device, id: u32, name: *const i8, parent_hw: *const clk_hw, flags: usize, reg: *mut core::ffi::c_void, shift: u8, width: u8, clk_divider_flags: usize, priv_flag: usize, lock: *mut spinlock_t) -> *mut clk_hw { let d=devm_kzalloc(dev,core::mem::size_of::<eswin_divider_clock>(),GFP_KERNEL) as *mut eswin_divider_clock; if d.is_null(){return ERR_PTR(-ENOMEM)} (*d).id=id; (*d).ctrl_reg=reg; (*d).shift=shift; (*d).width=width; (*d).div_flags=clk_divider_flags; (*d).priv_flag=priv_flag; (*d).lock=lock; let hw=&mut (*d).hw; if devm_clk_hw_register(dev,hw)!=0{return ERR_PTR(-EINVAL)} hw }
pub unsafe fn eswin_clk_register_divider(dev: *mut device, clks: *mut eswin_divider_clock, nums: i32, data: *mut eswin_clock_data) -> i32 { for i in 0..nums { let c=&mut *clks.add(i as usize); let hw=devm_clk_hw_register_divider_parent_data(dev,c.name,c.parent_data,c.flags,(*data).base.add(c.reg),c.shift,c.width,c.div_flags,&mut (*data).lock); if IS_ERR(hw){return PTR_ERR(hw)} c.hw=*hw; (*data).clk_data.hws[c.id]=hw; } 0 }
pub unsafe fn eswin_clk_register_gate(dev: *mut device, clks: *mut eswin_gate_clock, nums: i32, data: *mut eswin_clock_data) -> i32 { for i in 0..nums { let c=&mut *clks.add(i as usize); let hw=devm_clk_hw_register_gate_parent_data(dev,c.name,c.parent_data,c.flags,(*data).base.add(c.reg),c.bit_idx,c.gate_flags,&mut (*data).lock); if IS_ERR(hw){return PTR_ERR(hw)} c.hw=*hw; (*data).clk_data.hws[c.id]=hw; } 0 }
pub unsafe fn eswin_clk_register_clks(dev: *mut device, clks: *mut eswin_clk_info, nums: i32, data: *mut eswin_clock_data) -> i32 { for i in 0..nums { let info=&mut *clks.add(i as usize); let hw=match info.kind { CLK_FIXED_FACTOR=>devm_clk_hw_register_fixed_factor_parent_hw(dev,info.data.factor.name,(*data).clk_data.hws[info.pid],info.data.factor.flags,info.data.factor.mult,info.data.factor.div), _=>return -EINVAL }; if IS_ERR(hw){return PTR_ERR(hw)} info.hw=*hw; (*data).clk_data.hws[info.id]=hw; } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
