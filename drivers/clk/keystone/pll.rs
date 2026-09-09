// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PLL clock driver for Keystone devices
 *
 * Copyright (C) 2013 Texas Instruments Inc.
 *\tMurali Karicheri <m-karicheri2@ti.com>
 *\tSantosh Shilimkar <santosh.shilimkar@ti.com>
 */

// Linux dependencies supplied by the surrounding translation unit.

const PLLM_LOW_MASK: u32 = 0x3f;
const PLLM_HIGH_MASK: u32 = 0x7ffc0;
const MAIN_PLLM_HIGH_MASK: u32 = 0x7f000;
const PLLM_HIGH_SHIFT: u32 = 6;
const PLLD_MASK: u32 = 0x3f;
const CLKOD_MASK: u32 = 0x780000;
const CLKOD_SHIFT: u32 = 19;

/**
 * struct clk_pll_data - pll data structure
 * @has_pllctrl: If set to non zero, lower 6 bits of multiplier is in pllm
 *\tregister of pll controller, else it is in the pll_ctrl0((bit 11-6)
 *\tin the pll_ctrl0
 * @phy_pllm: Physical address of PLLM in pll controller. Used when
 *\thas_pllctrl is non zero.
 * @phy_pll_ctl0: Physical address of PLL ctrl0.
 * @pllm: PLL register map address for multiplier bits
 * @pllod: PLL register map address for post divider bits
 * @pll_ctl0: PLL controller map address
 * @pllm_lower_mask: multiplier lower mask
 * @pllm_upper_mask: multiplier upper mask
 * @pllm_upper_shift: multiplier upper shift
 * @plld_mask: divider mask
 * @clkod_mask: output divider mask
 * @clkod_shift: output divider shift
 * @postdiv: Fixed post divider
 */
#[repr(C)]
struct ClkPllData {
    has_pllctrl: bool,
    phy_pllm: u32,
    phy_pll_ctl0: u32,
    pllm: *mut core::ffi::c_void,
    pllod: *mut core::ffi::c_void,
    pll_ctl0: *mut core::ffi::c_void,
    pllm_lower_mask: u32,
    pllm_upper_mask: u32,
    pllm_upper_shift: u32,
    plld_mask: u32,
    clkod_mask: u32,
    clkod_shift: u32,
    postdiv: u32,
}

/** struct clk_pll - Main pll clock */
#[repr(C)]
struct ClkPll {
    hw: ClkHw,
    pll_data: *mut ClkPllData,
}

unsafe fn clk_pll_from_hw(hw: *mut ClkHw) -> *mut ClkPll {
    hw as *mut ClkPll
}

unsafe fn clk_pllclk_recalc(hw: *mut ClkHw, mut parent_rate: usize) -> usize {
    let pll = &mut *clk_pll_from_hw(hw);
    let pll_data = &*pll.pll_data;
    let mut rate = parent_rate;
    let mut mult: u32 = 0;
    let prediv: u32;
    let postdiv: u32;
    let mut val: u32;

    if pll_data.has_pllctrl {
        val = readl(pll_data.pllm);
        mult = val & pll_data.pllm_lower_mask;
    }

    val = readl(pll_data.pll_ctl0);
    mult |= (val & pll_data.pllm_upper_mask) >> pll_data.pllm_upper_shift;
    prediv = val & pll_data.plld_mask;

    if !pll_data.has_pllctrl {
        postdiv = ((val & pll_data.clkod_mask) >> pll_data.clkod_shift) + 1;
    } else if !pll_data.pllod.is_null() {
        let mut postdiv_val = readl(pll_data.pllod);
        postdiv_val = ((postdiv_val & pll_data.clkod_mask) >> pll_data.clkod_shift) + 1;
        postdiv = postdiv_val;
    } else {
        postdiv = pll_data.postdiv;
    }

    rate /= (prediv + 1) as usize;
    rate = rate * (mult + 1) as usize;
    rate /= postdiv as usize;
    rate
}

static CLK_PLL_OPS: ClkOps = ClkOps {
    recalc_rate: Some(clk_pllclk_recalc),
};

unsafe fn clk_register_pll(
    dev: *mut Device,
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    pll_data: *mut ClkPllData,
) -> *mut Clk {
    let pll = kzalloc_obj::<ClkPll>();
    if pll.is_null() {
        return err_ptr(-12);
    }

    let mut init: ClkInitData = core::mem::zeroed();
    init.name = name;
    init.ops = &CLK_PLL_OPS;
    init.flags = 0;
    init.parent_names = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    init.num_parents = if !parent_name.is_null() { 1 } else { 0 };

    (*pll).pll_data = pll_data;
    (*pll).hw.init = &init;

    let clk = clk_register(core::ptr::null_mut(), &mut (*pll).hw);
    if is_err(clk) {
        kfree(pll as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }
    clk
}

/** _of_pll_clk_init - PLL initialisation via DT */
unsafe fn _of_pll_clk_init(node: *mut DeviceNode, pllctrl: bool) {
    let pll_data = kzalloc_obj::<ClkPllData>();
    if pll_data.is_null() {
        pr_err!("{}: Out of memory\n", "_of_pll_clk_init");
        return;
    }

    let parent_name = of_clk_get_parent_name(node, 0);
    if of_property_read_u32(node, b"fixed-postdiv\0".as_ptr() as _, &mut (*pll_data).postdiv) != 0 {
        (*pll_data).clkod_mask = CLKOD_MASK;
        (*pll_data).clkod_shift = CLKOD_SHIFT;
        let i = of_property_match_string(node, b"reg-names\0".as_ptr() as _, b"post-divider\0".as_ptr() as _);
        (*pll_data).pllod = of_iomap(node, i);
    }

    let i = of_property_match_string(node, b"reg-names\0".as_ptr() as _, b"control\0".as_ptr() as _);
    (*pll_data).pll_ctl0 = of_iomap(node, i);
    if (*pll_data).pll_ctl0.is_null() {
        pr_err!("%s: ioremap failed\n", "_of_pll_clk_init");
        iounmap((*pll_data).pllod);
        kfree(pll_data as *mut core::ffi::c_void);
        return;
    }

    (*pll_data).pllm_lower_mask = PLLM_LOW_MASK;
    (*pll_data).pllm_upper_shift = PLLM_HIGH_SHIFT;
    (*pll_data).plld_mask = PLLD_MASK;
    (*pll_data).has_pllctrl = pllctrl;
    if !pllctrl {
        (*pll_data).pllm_upper_mask = PLLM_HIGH_MASK;
    } else {
        (*pll_data).pllm_upper_mask = MAIN_PLLM_HIGH_MASK;
        let i = of_property_match_string(node, b"reg-names\0".as_ptr() as _, b"multiplier\0".as_ptr() as _);
        (*pll_data).pllm = of_iomap(node, i);
        if (*pll_data).pllm.is_null() {
            iounmap((*pll_data).pll_ctl0);
            iounmap((*pll_data).pllod);
            kfree(pll_data as *mut core::ffi::c_void);
            return;
        }
    }

    let clk = clk_register_pll(core::ptr::null_mut(), (*node).name, parent_name, pll_data);
    if !is_err_or_null(clk) {
        of_clk_add_provider(node, of_clk_src_simple_get, clk);
        return;
    }

    pr_err!("%s: error initializing pll %pOFn\n", "_of_pll_clk_init", node);
    kfree(pll_data as *mut core::ffi::c_void);
}

/** of_keystone_pll_clk_init - PLL initialisation DT wrapper */
unsafe fn of_keystone_pll_clk_init(node: *mut DeviceNode) {
    _of_pll_clk_init(node, false);
}

/** of_keystone_main_pll_clk_init - Main PLL initialisation DT wrapper */
unsafe fn of_keystone_main_pll_clk_init(node: *mut DeviceNode) {
    _of_pll_clk_init(node, true);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
