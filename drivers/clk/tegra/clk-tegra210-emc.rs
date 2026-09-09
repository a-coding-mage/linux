// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2015-2020, NVIDIA CORPORATION.  All rights reserved.
 */

// Linux dependencies supplied by the surrounding kernel translation.

const CLK_SOURCE_EMC: u32 = 0x19c;
const CLK_SOURCE_EMC_2X_CLK_SRC: u32 = 0xe0000000;
const CLK_SOURCE_EMC_MC_EMC_SAME_FREQ: u32 = 1 << 16;
const CLK_SOURCE_EMC_2X_CLK_DIVISOR: u32 = 0xff;

const CLK_SRC_PLLM: u8 = 0;
const CLK_SRC_PLLC: u8 = 1;
const CLK_SRC_PLLP: u8 = 2;
const CLK_SRC_CLK_M: u8 = 3;
const CLK_SRC_PLLM_UD: u8 = 4;
const CLK_SRC_PLLMB_UD: u8 = 5;
const CLK_SRC_PLLMB: u8 = 6;
const CLK_SRC_PLLP_UD: u8 = 7;

#[repr(C)]
pub struct tegra210_clk_emc {
    pub hw: clk_hw,
    pub regs: *mut core::ffi::c_void,
    pub provider: *mut tegra210_clk_emc_provider,
    pub parents: [*mut clk; 8],
}

#[inline]
unsafe fn to_tegra210_clk_emc(hw: *mut clk_hw) -> *mut tegra210_clk_emc {
    (hw as *mut u8).sub(core::mem::offset_of!(tegra210_clk_emc, hw)) as *mut tegra210_clk_emc
}

static TEGRA210_CLK_EMC_PARENTS: [&[u8]; 8] = [
    b"pll_m\0", b"pll_c\0", b"pll_p\0", b"clk_m\0",
    b"pll_m_ud\0", b"pll_mb_ud\0", b"pll_mb\0", b"pll_p_ud\0",
];

unsafe fn tegra210_clk_emc_get_parent(hw: *mut clk_hw) -> u8 {
    let emc = to_tegra210_clk_emc(hw);
    let value = readl_relaxed((*emc).regs.add(CLK_SOURCE_EMC as usize) as *const u32);
    ((value & CLK_SOURCE_EMC_2X_CLK_SRC) >> 29) as u8
}

unsafe fn tegra210_clk_emc_recalc_rate(hw: *mut clk_hw, _parent_rate: ulong) -> ulong {
    let emc = to_tegra210_clk_emc(hw);
    let parent_rate = clk_hw_get_rate(clk_hw_get_parent(hw));
    let value = readl_relaxed((*emc).regs.add(CLK_SOURCE_EMC as usize) as *const u32);
    let div = ((value & CLK_SOURCE_EMC_2X_CLK_DIVISOR) + 2) as ulong;
    (parent_rate * 2 + div - 1) / div
}

unsafe fn tegra210_clk_emc_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let emc = to_tegra210_clk_emc(hw);
    let provider = (*emc).provider;
    if provider.is_null() || (*provider).configs.is_null() || (*provider).num_configs == 0 {
        (*req).rate = clk_hw_get_rate(hw);
        return 0;
    }
    for i in 0..(*provider).num_configs {
        let config = &*(*provider).configs.add(i);
        if config.rate >= (*req).rate { (*req).rate = config.rate; return 0; }
    }
    (*req).rate = (*(*provider).configs.add((*provider).num_configs - 1)).rate;
    0
}

unsafe fn tegra210_clk_emc_find_parent(emc: *mut tegra210_clk_emc, index: u8) -> *mut clk {
    let parent = clk_hw_get_parent_by_index(&mut (*emc).hw, index);
    let name = clk_hw_get_name(parent);
    // XXX implement cache?
    __clk_lookup(name)
}

unsafe fn tegra210_clk_emc_set_rate(hw: *mut clk_hw, rate: ulong, _parent_rate: ulong) -> i32 {
    let emc = to_tegra210_clk_emc(hw);
    let provider = (*emc).provider;
    if (*provider).configs.is_null() || (*provider).num_configs == 0 { return -22; }
    let mut i = 0;
    while i < (*provider).num_configs && (*(*provider).configs.add(i)).rate < rate { i += 1; }
    let config = if i == (*provider).num_configs { &mut *(*provider).configs.add(i - 1) } else { &mut *(*provider).configs.add(i) };
    let old_idx = tegra210_clk_emc_get_parent(hw);
    let mut new_idx = ((config.value & CLK_SOURCE_EMC_2X_CLK_SRC) >> 29) as u8;
    let old = clk_hw_get_parent_by_index(hw, old_idx);
    let mut new = clk_hw_get_parent_by_index(hw, new_idx);
    let (index, parent) = if config.parent_rate != clk_hw_get_rate(old) {
        if new_idx == old_idx {
            new_idx = match new_idx { CLK_SRC_PLLM => CLK_SRC_PLLMB, CLK_SRC_PLLM_UD => CLK_SRC_PLLMB_UD, CLK_SRC_PLLMB_UD => CLK_SRC_PLLM_UD, CLK_SRC_PLLMB => CLK_SRC_PLLM, x => x };
            if new_idx == old_idx { return -22; }
            new = clk_hw_get_parent_by_index(hw, new_idx);
        }
        (new_idx, new)
    } else { (old_idx, old) };
    let clk = tegra210_clk_emc_find_parent(emc, index);
    if IS_ERR(clk) { return PTR_ERR(clk); }
    if clk_get_rate(clk) != config.parent_rate { let err = clk_set_rate(clk, config.parent_rate); if err < 0 { return err; } }
    if parent != old { let err = clk_prepare_enable(clk); if err < 0 { return err; } }
    config.value = (config.value & !CLK_SOURCE_EMC_2X_CLK_SRC) | ((index as u32) << 29);
    let err = ((*provider).set_rate)( (*provider).dev, config);
    if err < 0 { if parent != old { clk_disable_unprepare(clk); } return err; }
    if parent != old {
        let old_clk = tegra210_clk_emc_find_parent(emc, old_idx);
        if IS_ERR(old_clk) { return PTR_ERR(old_clk); }
        clk_hw_reparent(hw, parent);
        clk_disable_unprepare(old_clk);
    }
    err
}

static TEGRA210_CLK_EMC_OPS: clk_ops = clk_ops {
    get_parent: Some(tegra210_clk_emc_get_parent),
    recalc_rate: Some(tegra210_clk_emc_recalc_rate),
    determine_rate: Some(tegra210_clk_emc_determine_rate),
    set_rate: Some(tegra210_clk_emc_set_rate),
};

pub unsafe fn tegra210_clk_register_emc(np: *mut device_node, regs: *mut core::ffi::c_void) -> *mut clk {
    let emc = kzalloc::<tegra210_clk_emc>();
    if emc.is_null() { return ERR_PTR(-12); }
    (*emc).regs = regs;
    let init = clk_init_data { name: b"emc\0".as_ptr(), ops: &TEGRA210_CLK_EMC_OPS, flags: CLK_IS_CRITICAL | CLK_GET_RATE_NOCACHE, parent_names: TEGRA210_CLK_EMC_PARENTS.as_ptr(), num_parents: 8 };
    (*emc).hw.init = &init;
    let clk = clk_register(core::ptr::null_mut(), &mut (*emc).hw);
    if IS_ERR(clk) { kfree(emc); }
    clk
}

pub unsafe fn tegra210_clk_emc_attach(clk: *mut clk, provider: *mut tegra210_clk_emc_provider) -> i32 {
    let hw = __clk_get_hw(clk);
    let emc = to_tegra210_clk_emc(hw);
    if !try_module_get((*provider).owner) { return -19; }
    for i in 0..(*provider).num_configs {
        let config = &mut *(*provider).configs.add(i);
        let div = config.value & CLK_SOURCE_EMC_2X_CLK_DIVISOR;
        let src = ((config.value & CLK_SOURCE_EMC_2X_CLK_SRC) >> 29) as u8;
        if div & 1 != 0 { module_put((*provider).owner); return -22; }
        let same_freq = config.value & CLK_SOURCE_EMC_MC_EMC_SAME_FREQ != 0;
        if same_freq != config.same_freq { module_put((*provider).owner); return -22; }
        let parent = clk_hw_get_parent_by_index(hw, src);
        config.parent = src;
        let calc = config.rate * (1 + (div / 2) as ulong);
        if src == CLK_SRC_PLLM || src == CLK_SRC_PLLM_UD { config.parent_rate = calc; }
        else { config.parent_rate = clk_hw_get_rate(parent); if config.parent_rate != calc { module_put((*provider).owner); return -22; } }
    }
    (*emc).provider = provider;
    0
}

pub unsafe fn tegra210_clk_emc_detach(clk: *mut clk) {
    let emc = to_tegra210_clk_emc(__clk_get_hw(clk));
    module_put((*(*emc).provider).owner);
    (*emc).provider = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
