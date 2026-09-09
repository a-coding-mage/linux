// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 SiFive, Inc.
 * Copyright (C) 2020 Zong Li
 */

// C dependencies: linux delay/io/clock APIs and sifive PRCI definitions.

unsafe fn __prci_readl(pd: *mut __prci_data, offs: u32) -> u32 {
    readl_relaxed((*pd).va.add(offs as usize))
}

unsafe fn __prci_writel(v: u32, offs: u32, pd: *mut __prci_data) {
    writel_relaxed(v, (*pd).va.add(offs as usize));
}

unsafe fn __prci_wrpll_unpack(c: *mut wrpll_cfg, r: u32) {
    let mut v = r & PRCI_COREPLLCFG0_DIVR_MASK;
    v >>= PRCI_COREPLLCFG0_DIVR_SHIFT;
    (*c).divr = v;
    v = (r & PRCI_COREPLLCFG0_DIVF_MASK) >> PRCI_COREPLLCFG0_DIVF_SHIFT;
    (*c).divf = v;
    v = (r & PRCI_COREPLLCFG0_DIVQ_MASK) >> PRCI_COREPLLCFG0_DIVQ_SHIFT;
    (*c).divq = v;
    v = (r & PRCI_COREPLLCFG0_RANGE_MASK) >> PRCI_COREPLLCFG0_RANGE_SHIFT;
    (*c).range = v;
    (*c).flags &= WRPLL_FLAGS_INT_FEEDBACK_MASK | WRPLL_FLAGS_EXT_FEEDBACK_MASK;
    // external feedback mode not supported
    (*c).flags |= WRPLL_FLAGS_INT_FEEDBACK_MASK;
}

unsafe fn __prci_wrpll_pack(c: *const wrpll_cfg) -> u32 {
    let mut r = 0;
    r |= (*c).divr << PRCI_COREPLLCFG0_DIVR_SHIFT;
    r |= (*c).divf << PRCI_COREPLLCFG0_DIVF_SHIFT;
    r |= (*c).divq << PRCI_COREPLLCFG0_DIVQ_SHIFT;
    r |= (*c).range << PRCI_COREPLLCFG0_RANGE_SHIFT;
    // external feedback mode not supported
    r |= PRCI_COREPLLCFG0_FSE_MASK;
    r
}

unsafe fn __prci_wrpll_read_cfg0(pd: *mut __prci_data, pwd: *mut __prci_wrpll_data) {
    __prci_wrpll_unpack(&mut (*pwd).c, __prci_readl(pd, (*pwd).cfg0_offs));
}

unsafe fn __prci_wrpll_write_cfg0(pd: *mut __prci_data, pwd: *mut __prci_wrpll_data, c: *mut wrpll_cfg) {
    __prci_writel(__prci_wrpll_pack(c), (*pwd).cfg0_offs, pd);
    core::ptr::copy_nonoverlapping(c, &mut (*pwd).c, 1);
}

unsafe fn __prci_wrpll_write_cfg1(pd: *mut __prci_data, pwd: *mut __prci_wrpll_data, enable: u32) {
    __prci_writel(enable, (*pwd).cfg1_offs, pd);
}

pub unsafe fn sifive_prci_wrpll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let pc = clk_hw_to_prci_clock(hw);
    wrpll_calc_output_rate(&(*(*pc).pwd).c, parent_rate)
}

pub unsafe fn sifive_prci_wrpll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let pc = clk_hw_to_prci_clock(hw);
    let pwd = (*pc).pwd;
    let mut c = (*pwd).c;
    wrpll_configure_for_rate(&mut c, (*req).rate, (*req).best_parent_rate);
    (*req).rate = wrpll_calc_output_rate(&c, (*req).best_parent_rate);
    0
}

pub unsafe fn sifive_prci_wrpll_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let pc = clk_hw_to_prci_clock(hw);
    let pwd = (*pc).pwd;
    let pd = (*pc).pd;
    let r = wrpll_configure_for_rate(&mut (*pwd).c, rate, parent_rate);
    if r != 0 { return r; }
    if let Some(f) = (*pwd).enable_bypass { f(pd); }
    __prci_wrpll_write_cfg0(pd, pwd, &mut (*pwd).c);
    udelay(wrpll_calc_max_lock_us(&(*pwd).c));
    0
}

pub unsafe fn sifive_clk_is_enabled(hw: *mut clk_hw) -> c_int {
    let pc = clk_hw_to_prci_clock(hw);
    let pwd = (*pc).pwd;
    let r = __prci_readl((*pc).pd, (*pwd).cfg1_offs);
    if r & PRCI_COREPLLCFG1_CKE_MASK != 0 { 1 } else { 0 }
}

pub unsafe fn sifive_prci_clock_enable(hw: *mut clk_hw) -> c_int {
    let pc = clk_hw_to_prci_clock(hw); let pwd = (*pc).pwd; let pd = (*pc).pd;
    if sifive_clk_is_enabled(hw) != 0 { return 0; }
    __prci_wrpll_write_cfg1(pd, pwd, PRCI_COREPLLCFG1_CKE_MASK);
    if let Some(f) = (*pwd).disable_bypass { f(pd); }
    0
}

pub unsafe fn sifive_prci_clock_disable(hw: *mut clk_hw) {
    let pc = clk_hw_to_prci_clock(hw); let pwd = (*pc).pwd; let pd = (*pc).pd;
    if let Some(f) = (*pwd).enable_bypass { f(pd); }
    let r = __prci_readl(pd, (*pwd).cfg1_offs) & !PRCI_COREPLLCFG1_CKE_MASK;
    __prci_wrpll_write_cfg1(pd, pwd, r);
}

pub unsafe fn sifive_prci_tlclksel_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let pd = (*clk_hw_to_prci_clock(hw)).pd;
    let v = __prci_readl(pd, PRCI_CLKMUXSTATUSREG_OFFSET) & PRCI_CLKMUXSTATUSREG_TLCLKSEL_STATUS_MASK;
    div_u64(parent_rate, if v != 0 { 1 } else { 2 })
}

pub unsafe fn sifive_prci_hfpclkplldiv_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let pd = (*clk_hw_to_prci_clock(hw)).pd;
    div_u64(parent_rate, __prci_readl(pd, PRCI_HFPCLKPLLDIV_OFFSET) as u64 + 2)
}

pub unsafe fn sifive_prci_coreclksel_use_hfclk(pd: *mut __prci_data) { let mut r=__prci_readl(pd,PRCI_CORECLKSEL_OFFSET); r|=PRCI_CORECLKSEL_CORECLKSEL_MASK; __prci_writel(r,PRCI_CORECLKSEL_OFFSET,pd); let _=__prci_readl(pd,PRCI_CORECLKSEL_OFFSET); }
pub unsafe fn sifive_prci_coreclksel_use_corepll(pd: *mut __prci_data) { let mut r=__prci_readl(pd,PRCI_CORECLKSEL_OFFSET); r&=!PRCI_CORECLKSEL_CORECLKSEL_MASK; __prci_writel(r,PRCI_CORECLKSEL_OFFSET,pd); let _=__prci_readl(pd,PRCI_CORECLKSEL_OFFSET); }
pub unsafe fn sifive_prci_coreclksel_use_final_corepll(pd: *mut __prci_data) { sifive_prci_coreclksel_use_corepll(pd); }
pub unsafe fn sifive_prci_corepllsel_use_dvfscorepll(pd: *mut __prci_data) { let mut r=__prci_readl(pd,PRCI_COREPLLSEL_OFFSET); r|=PRCI_COREPLLSEL_COREPLLSEL_MASK; __prci_writel(r,PRCI_COREPLLSEL_OFFSET,pd); let _=__prci_readl(pd,PRCI_COREPLLSEL_OFFSET); }
pub unsafe fn sifive_prci_corepllsel_use_corepll(pd: *mut __prci_data) { let mut r=__prci_readl(pd,PRCI_COREPLLSEL_OFFSET); r&=!PRCI_COREPLLSEL_COREPLLSEL_MASK; __prci_writel(r,PRCI_COREPLLSEL_OFFSET,pd); let _=__prci_readl(pd,PRCI_COREPLLSEL_OFFSET); }
pub unsafe fn sifive_prci_hfpclkpllsel_use_hfclk(pd: *mut __prci_data) { let mut r=__prci_readl(pd,PRCI_HFPCLKPLLSEL_OFFSET); r|=PRCI_HFPCLKPLLSEL_HFPCLKPLLSEL_MASK; __prci_writel(r,PRCI_HFPCLKPLLSEL_OFFSET,pd); let _=__prci_readl(pd,PRCI_HFPCLKPLLSEL_OFFSET); }
pub unsafe fn sifive_prci_hfpclkpllsel_use_hfpclkpll(pd: *mut __prci_data) { let mut r=__prci_readl(pd,PRCI_HFPCLKPLLSEL_OFFSET); r&=!PRCI_HFPCLKPLLSEL_HFPCLKPLLSEL_MASK; __prci_writel(r,PRCI_HFPCLKPLLSEL_OFFSET,pd); let _=__prci_readl(pd,PRCI_HFPCLKPLLSEL_OFFSET); }

pub unsafe fn sifive_prci_pcie_aux_clock_is_enabled(hw: *mut clk_hw) -> c_int { let pd=(*clk_hw_to_prci_clock(hw)).pd; if __prci_readl(pd,PRCI_PCIE_AUX_OFFSET)&PRCI_PCIE_AUX_EN_MASK!=0 {1} else {0} }
pub unsafe fn sifive_prci_pcie_aux_clock_enable(hw: *mut clk_hw) -> c_int { let pd=(*clk_hw_to_prci_clock(hw)).pd; if sifive_prci_pcie_aux_clock_is_enabled(hw)!=0{return 0;} __prci_writel(1,PRCI_PCIE_AUX_OFFSET,pd); let _=__prci_readl(pd,PRCI_PCIE_AUX_OFFSET); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
