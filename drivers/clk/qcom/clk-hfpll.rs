// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than implemented in this file.

const PLL_OUTCTRL: u32 = 1 << 0;
const PLL_BYPASSNL: u32 = 1 << 1;
const PLL_RESET_N: u32 = 1 << 2;

/* Initialize a HFPLL at a given rate and enable it. */
unsafe fn __clk_hfpll_init_once(hw: *mut clk_hw) {
    let h: *mut clk_hfpll = to_clk_hfpll(hw);
    let hd: *const hfpll_data = (*h).d;
    let regmap: *mut regmap = (*h).clkr.regmap;

    if likely((*h).init_done) {
        return;
    }

    /* Configure PLL parameters for integer mode. */
    if (*hd).config_val != 0 {
        regmap_write(regmap, (*hd).config_reg, (*hd).config_val);
    }
    regmap_write(regmap, (*hd).m_reg, 0);
    regmap_write(regmap, (*hd).n_reg, 1);

    if (*hd).user_reg != 0 {
        let mut regval: u32 = (*hd).user_val;
        let rate: c_ulong;

        rate = clk_hw_get_rate(hw);

        /* Pick the right VCO. */
        if (*hd).user_vco_mask != 0 && rate > (*hd).low_vco_max_rate {
            regval |= (*hd).user_vco_mask;
        }
        regmap_write(regmap, (*hd).user_reg, regval);
    }

    /* Write L_VAL from conf if it exist */
    if (*hd).l_val != 0 {
        regmap_write(regmap, (*hd).l_reg, (*hd).l_val);
    }

    if (*hd).droop_reg != 0 {
        regmap_write(regmap, (*hd).droop_reg, (*hd).droop_val);
    }

    (*h).init_done = true;
}

unsafe fn __clk_hfpll_enable(hw: *mut clk_hw) {
    let h: *mut clk_hfpll = to_clk_hfpll(hw);
    let hd: *const hfpll_data = (*h).d;
    let regmap: *mut regmap = (*h).clkr.regmap;
    let mut val: u32 = 0;

    __clk_hfpll_init_once(hw);

    /* Disable PLL bypass mode. */
    regmap_update_bits(regmap, (*hd).mode_reg, PLL_BYPASSNL, PLL_BYPASSNL);

    /*
     * H/W requires a 5us delay between disabling the bypass and
     * de-asserting the reset. Delay 10us just to be safe.
     */
    udelay(10);

    /* De-assert active-low PLL reset. */
    regmap_update_bits(regmap, (*hd).mode_reg, PLL_RESET_N, PLL_RESET_N);

    /* Wait for PLL to lock. */
    if (*hd).status_reg != 0 {
        /*
         * Busy wait. Should never timeout, we add a timeout to
         * prevent any sort of stall.
         */
        regmap_read_poll_timeout(regmap, (*hd).status_reg, &mut val,
                                 (val & (1u32 << (*hd).lock_bit)) == 0,
                                 0, 100 * USEC_PER_MSEC);
    } else {
        udelay(60);
    }

    /* Enable PLL output. */
    regmap_update_bits(regmap, (*hd).mode_reg, PLL_OUTCTRL, PLL_OUTCTRL);
}

/* Enable an already-configured HFPLL. */
unsafe fn clk_hfpll_enable(hw: *mut clk_hw) -> c_int {
    let mut flags: c_ulong = 0;
    let h: *mut clk_hfpll = to_clk_hfpll(hw);
    let hd: *const hfpll_data = (*h).d;
    let regmap: *mut regmap = (*h).clkr.regmap;
    let mut mode: u32 = 0;

    spin_lock_irqsave(&mut (*h).lock, &mut flags);
    regmap_read(regmap, (*hd).mode_reg, &mut mode);
    if (mode & (PLL_BYPASSNL | PLL_RESET_N | PLL_OUTCTRL)) == 0 {
        __clk_hfpll_enable(hw);
    }
    spin_unlock_irqrestore(&mut (*h).lock, flags);

    0
}

unsafe fn __clk_hfpll_disable(h: *mut clk_hfpll) {
    let hd: *const hfpll_data = (*h).d;
    let regmap: *mut regmap = (*h).clkr.regmap;

    /*
     * Disable the PLL output, disable test mode, enable the bypass mode,
     * and assert the reset.
     */
    regmap_update_bits(regmap, (*hd).mode_reg,
                       PLL_BYPASSNL | PLL_RESET_N | PLL_OUTCTRL, 0);
}

unsafe fn clk_hfpll_disable(hw: *mut clk_hw) {
    let h: *mut clk_hfpll = to_clk_hfpll(hw);
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*h).lock, &mut flags);
    __clk_hfpll_disable(h);
    spin_unlock_irqrestore(&mut (*h).lock, flags);
}

unsafe fn clk_hfpll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let h: *mut clk_hfpll = to_clk_hfpll(hw);
    let hd: *const hfpll_data = (*h).d;
    let mut rrate: c_ulong;

    (*req).rate = clamp((*req).rate, (*hd).min_rate, (*hd).max_rate);

    rrate = div_round_up((*req).rate, (*req).best_parent_rate)
        .wrapping_mul((*req).best_parent_rate);
    if rrate > (*hd).max_rate {
        rrate = rrate.wrapping_sub((*req).best_parent_rate);
    }

    (*req).rate = rrate;
    0
}

/*
 * For optimization reasons, assumes no downstream clocks are actively using
 * it.
 */
unsafe fn clk_hfpll_set_rate(hw: *mut clk_hw, rate: c_ulong,
                             parent_rate: c_ulong) -> c_int {
    let h: *mut clk_hfpll = to_clk_hfpll(hw);
    let hd: *const hfpll_data = (*h).d;
    let regmap: *mut regmap = (*h).clkr.regmap;
    let mut flags: c_ulong = 0;
    let l_val: u32 = rate / parent_rate;
    let mut val: u32 = 0;
    let enabled: bool;

    spin_lock_irqsave(&mut (*h).lock, &mut flags);

    enabled = clk_hw_is_enabled(hw);
    if enabled {
        __clk_hfpll_disable(h);
    }

    /* Pick the right VCO. */
    if (*hd).user_reg != 0 && (*hd).user_vco_mask != 0 {
        regmap_read(regmap, (*hd).user_reg, &mut val);
        if rate <= (*hd).low_vco_max_rate {
            val &= !(*hd).user_vco_mask;
        } else {
            val |= (*hd).user_vco_mask;
        }
        regmap_write(regmap, (*hd).user_reg, val);
    }

    regmap_write(regmap, (*hd).l_reg, l_val);

    if enabled {
        __clk_hfpll_enable(hw);
    }

    spin_unlock_irqrestore(&mut (*h).lock, flags);
    0
}

unsafe fn clk_hfpll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let h: *mut clk_hfpll = to_clk_hfpll(hw);
    let hd: *const hfpll_data = (*h).d;
    let regmap: *mut regmap = (*h).clkr.regmap;
    let mut l_val: u32 = 0;

    regmap_read(regmap, (*hd).l_reg, &mut l_val);
    (l_val as c_ulong).wrapping_mul(parent_rate)
}

unsafe fn clk_hfpll_init(hw: *mut clk_hw) -> c_int {
    let h: *mut clk_hfpll = to_clk_hfpll(hw);
    let hd: *const hfpll_data = (*h).d;
    let regmap: *mut regmap = (*h).clkr.regmap;
    let mut mode: u32 = 0;
    let mut status: u32 = 0;

    regmap_read(regmap, (*hd).mode_reg, &mut mode);
    if mode != (PLL_BYPASSNL | PLL_RESET_N | PLL_OUTCTRL) {
        __clk_hfpll_init_once(hw);
        return 0;
    }

    if (*hd).status_reg != 0 {
        regmap_read(regmap, (*hd).status_reg, &mut status);
        if (status & (1u32 << (*hd).lock_bit)) == 0 {
            WARN(1, "HFPLL %s is ON, but not locked!\n", clk_hw_get_name(hw));
            clk_hfpll_disable(hw);
            __clk_hfpll_init_once(hw);
        }
    }

    0
}

unsafe fn hfpll_is_enabled(hw: *mut clk_hw) -> bool {
    let h: *mut clk_hfpll = to_clk_hfpll(hw);
    let hd: *const hfpll_data = (*h).d;
    let regmap: *mut regmap = (*h).clkr.regmap;
    let mut mode: u32 = 0;

    regmap_read(regmap, (*hd).mode_reg, &mut mode);
    mode &= 0x7;
    mode == (PLL_BYPASSNL | PLL_RESET_N | PLL_OUTCTRL)
}

const clk_ops_hfpll: clk_ops = clk_ops {
    enable: Some(clk_hfpll_enable),
    disable: Some(clk_hfpll_disable),
    is_enabled: Some(hfpll_is_enabled),
    determine_rate: Some(clk_hfpll_determine_rate),
    set_rate: Some(clk_hfpll_set_rate),
    recalc_rate: Some(clk_hfpll_recalc_rate),
    init: Some(clk_hfpll_init),
};

EXPORT_SYMBOL_GPL!(clk_ops_hfpll);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
