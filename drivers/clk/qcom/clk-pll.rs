// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013, The Linux Foundation. All rights reserved.
 */

// External kernel dependencies are supplied by the surrounding repository.

const PLL_OUTCTRL: u32 = BIT(0);
const PLL_BYPASSNL: u32 = BIT(1);
const PLL_RESET_N: u32 = BIT(2);

unsafe fn clk_pll_enable(hw: *mut clk_hw) -> c_int {
    let pll = to_clk_pll(hw);
    let mut ret: c_int;
    let mask: u32;
    let mut val: u32 = 0;

    mask = PLL_OUTCTRL | PLL_RESET_N | PLL_BYPASSNL;
    ret = regmap_read((*(*pll).clkr).regmap, (*pll).mode_reg, &mut val);
    if ret != 0 {
        return ret;
    }

    /* Skip if already enabled or in FSM mode */
    if (val & mask) == mask || (val & PLL_VOTE_FSM_ENA) != 0 {
        return 0;
    }

    /* Disable PLL bypass mode. */
    ret = regmap_update_bits((*(*pll).clkr).regmap, (*pll).mode_reg,
                             PLL_BYPASSNL, PLL_BYPASSNL);
    if ret != 0 {
        return ret;
    }

    /*
     * H/W requires a 5us delay between disabling the bypass and
     * de-asserting the reset. Delay 10us just to be safe.
     */
    udelay(10);

    /* De-assert active-low PLL reset. */
    ret = regmap_update_bits((*(*pll).clkr).regmap, (*pll).mode_reg,
                             PLL_RESET_N, PLL_RESET_N);
    if ret != 0 {
        return ret;
    }

    /* Wait until PLL is locked. */
    udelay(50);

    /* Enable PLL output. */
    regmap_update_bits((*(*pll).clkr).regmap, (*pll).mode_reg,
                       PLL_OUTCTRL, PLL_OUTCTRL)
}

unsafe fn clk_pll_disable(hw: *mut clk_hw) {
    let pll = to_clk_pll(hw);
    let mask: u32;
    let mut val: u32 = 0;

    regmap_read((*(*pll).clkr).regmap, (*pll).mode_reg, &mut val);
    /* Skip if in FSM mode */
    if (val & PLL_VOTE_FSM_ENA) != 0 {
        return;
    }
    mask = PLL_OUTCTRL | PLL_RESET_N | PLL_BYPASSNL;
    regmap_update_bits((*(*pll).clkr).regmap, (*pll).mode_reg, mask, 0);
}

unsafe fn clk_pll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let pll = to_clk_pll(hw);
    let mut l: u32 = 0;
    let mut m: u32 = 0;
    let mut n: u32 = 0;
    let mut config: u32 = 0;
    let mut rate: c_ulong;
    let mut tmp: u64;

    regmap_read((*(*pll).clkr).regmap, (*pll).l_reg, &mut l);
    regmap_read((*(*pll).clkr).regmap, (*pll).m_reg, &mut m);
    regmap_read((*(*pll).clkr).regmap, (*pll).n_reg, &mut n);

    l &= 0x3ff;
    m &= 0x7ffff;
    n &= 0x7ffff;

    rate = parent_rate.wrapping_mul(l as c_ulong);
    if n != 0 {
        tmp = parent_rate as u64;
        tmp = tmp.wrapping_mul(m as u64);
        tmp /= n as u64;
        rate = rate.wrapping_add(tmp as c_ulong);
    }
    if (*pll).post_div_width != 0 {
        regmap_read((*(*pll).clkr).regmap, (*pll).config_reg, &mut config);
        config >>= (*pll).post_div_shift;
        config &= BIT((*pll).post_div_width) - 1;
        rate /= (config + 1) as c_ulong;
    }

    rate
}

unsafe fn find_freq(mut f: *const pll_freq_tbl, rate: c_ulong) -> *const pll_freq_tbl {
    if f.is_null() {
        return core::ptr::null();
    }

    while (*f).freq != 0 {
        if rate <= (*f).freq {
            return f;
        }
        f = f.add(1);
    }

    core::ptr::null()
}

unsafe fn clk_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let pll = to_clk_pll(hw);
    let f = find_freq((*pll).freq_tbl, (*req).rate);

    if f.is_null() {
        (*req).rate = clk_pll_recalc_rate(hw, (*req).best_parent_rate);
    } else {
        (*req).rate = (*f).freq;
    }

    0
}

unsafe fn clk_pll_set_rate(hw: *mut clk_hw, rate: c_ulong, _p_rate: c_ulong) -> c_int {
    let pll = to_clk_pll(hw);
    let f = find_freq((*pll).freq_tbl, rate);
    let mut mode: u32 = 0;
    let enable_mask = PLL_OUTCTRL | PLL_BYPASSNL | PLL_RESET_N;

    if f.is_null() {
        return -EINVAL;
    }

    regmap_read((*(*pll).clkr).regmap, (*pll).mode_reg, &mut mode);
    let enabled = (mode & enable_mask) == enable_mask;

    if enabled {
        clk_pll_disable(hw);
    }

    regmap_update_bits((*(*pll).clkr).regmap, (*pll).l_reg, 0x3ff, (*f).l);
    regmap_update_bits((*(*pll).clkr).regmap, (*pll).m_reg, 0x7ffff, (*f).m);
    regmap_update_bits((*(*pll).clkr).regmap, (*pll).n_reg, 0x7ffff, (*f).n);
    regmap_write((*(*pll).clkr).regmap, (*pll).config_reg, (*f).ibits);

    if enabled {
        clk_pll_enable(hw);
    }

    0
}

pub static clk_pll_ops: clk_ops = clk_ops {
    .enable = Some(clk_pll_enable),
    .disable = Some(clk_pll_disable),
    .recalc_rate = Some(clk_pll_recalc_rate),
    .determine_rate = Some(clk_pll_determine_rate),
    .set_rate = Some(clk_pll_set_rate),
};

unsafe fn wait_for_pll(pll: *mut clk_pll) -> c_int {
    let mut val: u32 = 0;
    let mut ret: c_int;
    let mut count: c_int = 200;
    let name = clk_hw_get_name(&mut (*(*pll).clkr).hw);

    /* Wait for pll to enable. */
    while count > 0 {
        ret = regmap_read((*(*pll).clkr).regmap, (*pll).status_reg, &mut val);
        if ret != 0 {
            return ret;
        }
        if (val & BIT((*pll).status_bit)) != 0 {
            return 0;
        }
        udelay(1);
        count -= 1;
    }

    WARN(1, "%s didn't enable after voting for it!\n", name);
    -ETIMEDOUT
}

unsafe fn clk_pll_vote_enable(hw: *mut clk_hw) -> c_int {
    let pll = to_clk_pll(clk_hw_get_parent(hw));
    let ret = clk_enable_regmap(hw);
    if ret != 0 {
        return ret;
    }

    wait_for_pll(pll)
}

pub static clk_pll_vote_ops: clk_ops = clk_ops {
    .enable = Some(clk_pll_vote_enable),
    .disable = Some(clk_disable_regmap),
};

unsafe fn clk_pll_configure(pll: *mut clk_pll, regmap: *mut regmap,
                            config: *const pll_config) {
    let mut val: u32;
    let mut mask: u32;

    regmap_write(regmap, (*pll).l_reg, (*config).l);
    regmap_write(regmap, (*pll).m_reg, (*config).m);
    regmap_write(regmap, (*pll).n_reg, (*config).n);

    val = (*config).vco_val;
    val |= (*config).pre_div_val;
    val |= (*config).post_div_val;
    val |= (*config).mn_ena_mask;
    val |= (*config).main_output_mask;
    val |= (*config).aux_output_mask;

    mask = (*config).vco_mask;
    mask |= (*config).pre_div_mask;
    mask |= (*config).post_div_mask;
    mask |= (*config).mn_ena_mask;
    mask |= (*config).main_output_mask;
    mask |= (*config).aux_output_mask;

    regmap_update_bits(regmap, (*pll).config_reg, mask, val);
}

pub unsafe fn clk_pll_configure_sr(pll: *mut clk_pll, regmap: *mut regmap,
                                   config: *const pll_config, fsm_mode: bool) {
    clk_pll_configure(pll, regmap, config);
    if fsm_mode {
        qcom_pll_set_fsm_mode(regmap, (*pll).mode_reg, 1, 8);
    }
}

pub unsafe fn clk_pll_configure_sr_hpm_lp(pll: *mut clk_pll, regmap: *mut regmap,
                                          config: *const pll_config, fsm_mode: bool) {
    clk_pll_configure(pll, regmap, config);
    if fsm_mode {
        qcom_pll_set_fsm_mode(regmap, (*pll).mode_reg, 1, 0);
    }
}

unsafe fn clk_pll_sr2_enable(hw: *mut clk_hw) -> c_int {
    let pll = to_clk_pll(hw);
    let mut mode: u32 = 0;
    let mut ret = regmap_read((*(*pll).clkr).regmap, (*pll).mode_reg, &mut mode);
    if ret != 0 {
        return ret;
    }

    /* Disable PLL bypass mode. */
    ret = regmap_update_bits((*(*pll).clkr).regmap, (*pll).mode_reg,
                             PLL_BYPASSNL, PLL_BYPASSNL);
    if ret != 0 {
        return ret;
    }

    /*
     * H/W requires a 5us delay between disabling the bypass and
     * de-asserting the reset. Delay 10us just to be safe.
     */
    udelay(10);

    /* De-assert active-low PLL reset. */
    ret = regmap_update_bits((*(*pll).clkr).regmap, (*pll).mode_reg,
                             PLL_RESET_N, PLL_RESET_N);
    if ret != 0 {
        return ret;
    }

    ret = wait_for_pll(pll);
    if ret != 0 {
        return ret;
    }

    /* Enable PLL output. */
    regmap_update_bits((*(*pll).clkr).regmap, (*pll).mode_reg,
                       PLL_OUTCTRL, PLL_OUTCTRL)
}

unsafe fn clk_pll_sr2_set_rate(hw: *mut clk_hw, rate: c_ulong,
                               _prate: c_ulong) -> c_int {
    let pll = to_clk_pll(hw);
    let f = find_freq((*pll).freq_tbl, rate);
    let mut mode: u32 = 0;
    let enable_mask = PLL_OUTCTRL | PLL_BYPASSNL | PLL_RESET_N;

    if f.is_null() {
        return -EINVAL;
    }

    regmap_read((*(*pll).clkr).regmap, (*pll).mode_reg, &mut mode);
    let enabled = (mode & enable_mask) == enable_mask;

    if enabled {
        clk_pll_disable(hw);
    }

    regmap_update_bits((*(*pll).clkr).regmap, (*pll).l_reg, 0x3ff, (*f).l);
    regmap_update_bits((*(*pll).clkr).regmap, (*pll).m_reg, 0x7ffff, (*f).m);
    regmap_update_bits((*(*pll).clkr).regmap, (*pll).n_reg, 0x7ffff, (*f).n);

    if enabled {
        clk_pll_sr2_enable(hw);
    }

    0
}

pub static clk_pll_sr2_ops: clk_ops = clk_ops {
    .enable = Some(clk_pll_sr2_enable),
    .disable = Some(clk_pll_disable),
    .set_rate = Some(clk_pll_sr2_set_rate),
    .recalc_rate = Some(clk_pll_recalc_rate),
    .determine_rate = Some(clk_pll_determine_rate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
