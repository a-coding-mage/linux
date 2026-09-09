// SPDX-License-Identifier: GPL-2.0
/*
 * Zynq UltraScale+ MPSoC PLL driver
 *
 *  Copyright (C) 2016-2018 Xilinx
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct zynqmp_pll {
    pub hw: clk_hw,
    pub clk_id: u32,
    pub set_pll_mode: bool,
}

const PLL_FBDIV_MIN: u32 = 25;
const PLL_FBDIV_MAX: u32 = 125;
const PS_PLL_VCO_MIN: u64 = 1_500_000_000;
const PS_PLL_VCO_MAX: u64 = 3_000_000_000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum pll_mode {
    PLL_MODE_INT = 0,
    PLL_MODE_FRAC = 1,
    PLL_MODE_ERROR = 2,
}

const FRAC_OFFSET: u32 = 0x8;
const PLLFCFG_FRAC_EN: u32 = 1u32 << 31;
const FRAC_DIV: u32 = 1u32 << 16;

#[inline]
unsafe fn to_zynqmp_pll(hw: *mut clk_hw) -> *mut zynqmp_pll {
    hw as *mut zynqmp_pll
}

#[inline]
unsafe fn zynqmp_pll_get_mode(hw: *mut clk_hw) -> pll_mode {
    let clk = &*to_zynqmp_pll(hw);
    let clk_id = clk.clk_id;
    let clk_name = clk_hw_get_name(hw);
    let mut ret_payload = [0u32; PAYLOAD_ARG_CNT];

    let ret = zynqmp_pm_get_pll_frac_mode(clk_id, ret_payload.as_mut_ptr());
    if ret != 0 {
        pr_debug!("{}() PLL get frac mode failed for {}, ret = {}\n", "zynqmp_pll_get_mode", clk_name, ret);
        return pll_mode::PLL_MODE_ERROR;
    }

    match ret_payload[1] {
        0 => pll_mode::PLL_MODE_INT,
        1 => pll_mode::PLL_MODE_FRAC,
        _ => pll_mode::PLL_MODE_ERROR,
    }
}

#[inline]
unsafe fn zynqmp_pll_set_mode(hw: *mut clk_hw, on: bool) {
    let clk = &mut *to_zynqmp_pll(hw);
    let clk_id = clk.clk_id;
    let clk_name = clk_hw_get_name(hw);
    let mode = if on { pll_mode::PLL_MODE_FRAC as u32 } else { pll_mode::PLL_MODE_INT as u32 };

    let ret = zynqmp_pm_set_pll_frac_mode(clk_id, mode);
    if ret != 0 {
        pr_debug!("{}() PLL set frac mode failed for {}, ret = {}\n", "zynqmp_pll_set_mode", clk_name, ret);
    } else {
        clk.set_pll_mode = true;
    }
}

unsafe fn zynqmp_pll_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let req = &mut *req;
    if req.rate > PS_PLL_VCO_MAX {
        let div = (req.rate + PS_PLL_VCO_MAX - 1) / PS_PLL_VCO_MAX;
        req.rate /= div;
    }
    if req.rate < PS_PLL_VCO_MIN {
        let mult = (PS_PLL_VCO_MIN + req.rate - 1) / req.rate;
        req.rate *= mult;
    }

    let mut fbdiv = (req.rate + req.best_parent_rate / 2) / req.best_parent_rate;
    if fbdiv < PLL_FBDIV_MIN || fbdiv > PLL_FBDIV_MAX {
        fbdiv = fbdiv.clamp(PLL_FBDIV_MIN, PLL_FBDIV_MAX);
        req.rate = req.best_parent_rate * fbdiv;
    }
    0
}

unsafe fn zynqmp_pll_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let clk = &*to_zynqmp_pll(hw);
    let mut fbdiv = 0u32;
    let mut ret_payload = [0u32; PAYLOAD_ARG_CNT];
    if zynqmp_pm_clock_getdivider(clk.clk_id, &mut fbdiv) != 0 { return 0; }
    let mode = zynqmp_pll_get_mode(hw);
    if mode == pll_mode::PLL_MODE_ERROR { return 0; }
    let mut rate = parent_rate * fbdiv as u64;
    if mode == pll_mode::PLL_MODE_FRAC {
        zynqmp_pm_get_pll_frac_data(clk.clk_id, ret_payload.as_mut_ptr());
        rate += parent_rate * ret_payload[1] as u64 / FRAC_DIV as u64;
    }
    rate
}

unsafe fn zynqmp_pll_set_rate(hw: *mut clk_hw, rate: u64, parent_rate: u64) -> i32 {
    let clk = &*to_zynqmp_pll(hw);
    let rate_div = rate * FRAC_DIV as u64 / parent_rate;
    let f = rate_div % FRAC_DIV as u64;
    zynqmp_pll_set_mode(hw, f != 0);
    if f != 0 {
        let m = (rate_div / FRAC_DIV as u64).clamp(PLL_FBDIV_MIN as u64, PLL_FBDIV_MAX as u64);
        let frac = parent_rate * f / FRAC_DIV as u64;
        let ret = zynqmp_pm_clock_setdivider(clk.clk_id, m as u32);
        if ret != 0 { pr_debug!("{}() set divider failed for {}, ret = {}\n", "zynqmp_pll_set_rate", clk_hw_get_name(hw), ret); }
        zynqmp_pm_set_pll_frac_data(clk.clk_id, f as u32);
        return (parent_rate * m + frac) as i32;
    }
    let fbdiv = ((rate + parent_rate / 2) / parent_rate).clamp(PLL_FBDIV_MIN as u64, PLL_FBDIV_MAX as u64);
    let ret = zynqmp_pm_clock_setdivider(clk.clk_id, fbdiv as u32);
    if ret != 0 { pr_debug!("{}() set divider failed for {}, ret = {}\n", "zynqmp_pll_set_rate", clk_hw_get_name(hw), ret); }
    (parent_rate * fbdiv) as i32
}

unsafe fn zynqmp_pll_is_enabled(hw: *mut clk_hw) -> i32 {
    let clk = &*to_zynqmp_pll(hw);
    let mut state = 0u32;
    if zynqmp_pm_clock_getstate(clk.clk_id, &mut state) != 0 { return -5; }
    if state != 0 { 1 } else { 0 }
}

unsafe fn zynqmp_pll_enable(hw: *mut clk_hw) -> i32 {
    let clk = &mut *to_zynqmp_pll(hw);
    if zynqmp_pll_is_enabled(hw) != 0 && !clk.set_pll_mode { return 0; }
    clk.set_pll_mode = false;
    zynqmp_pm_clock_enable(clk.clk_id)
}

unsafe fn zynqmp_pll_disable(hw: *mut clk_hw) {
    let clk = &*to_zynqmp_pll(hw);
    if zynqmp_pll_is_enabled(hw) == 0 { return; }
    zynqmp_pm_clock_disable(clk.clk_id);
}

static zynqmp_pll_ops: clk_ops = clk_ops {
    enable: Some(zynqmp_pll_enable),
    disable: Some(zynqmp_pll_disable),
    is_enabled: Some(zynqmp_pll_is_enabled),
    determine_rate: Some(zynqmp_pll_determine_rate),
    recalc_rate: Some(zynqmp_pll_recalc_rate),
    set_rate: Some(zynqmp_pll_set_rate),
};

pub unsafe fn zynqmp_clk_register_pll(name: *const i8, clk_id: u32, parents: *const *const i8,
                                      _num_parents: u8, nodes: *const clock_topology) -> *mut clk_hw {
    let mut init: clk_init_data = core::mem::zeroed();
    init.name = name;
    init.ops = &zynqmp_pll_ops;
    init.flags = zynqmp_clk_map_common_ccf_flags((*nodes).flag);
    init.parent_names = parents;
    init.num_parents = 1;
    let pll = kzalloc_obj::<zynqmp_pll>();
    if pll.is_null() { return ERR_PTR(-12); }
    (*pll).hw.init = &init;
    (*pll).clk_id = clk_id;
    let hw = &mut (*pll).hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 { kfree(pll as *mut core::ffi::c_void); return ERR_PTR(ret); }
    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
