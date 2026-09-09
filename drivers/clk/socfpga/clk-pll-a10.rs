// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Altera Corporation. All rights reserved
 */

/* Clock Manager offsets */
const CLK_MGR_PLL_CLK_SRC_SHIFT: u32 = 8;
const CLK_MGR_PLL_CLK_SRC_MASK: u32 = 0x3;

/* Clock bypass bits */
const SOCFPGA_PLL_BG_PWRDWN: u32 = 0;
const SOCFPGA_PLL_PWR_DOWN: u32 = 1;
const SOCFPGA_PLL_EXT_ENA: u32 = 2;
const SOCFPGA_PLL_DIVF_MASK: u32 = 0x0000_1fff;
const SOCFPGA_PLL_DIVF_SHIFT: u32 = 0;
const SOCFPGA_PLL_DIVQ_MASK: u32 = 0x003f_0000;
const SOCFPGA_PLL_DIVQ_SHIFT: u32 = 16;
const SOCFGPA_MAX_PARENTS: usize = 5;

const SOCFPGA_MAIN_PLL_CLK: &str = "main_pll";
const SOCFPGA_PERIP_PLL_CLK: &str = "periph_pll";

/* The definitions below are supplied by the surrounding clock framework. */
extern "C" {
    static mut clk_mgr_a10_base_addr: *mut core::ffi::c_void;
}

unsafe fn clk_pll_recalc_rate(
    hwclk: *mut clk_hw,
    parent_rate: libc::c_ulong,
) -> libc::c_ulong {
    let socfpgaclk = to_socfpga_clk(hwclk);
    let reg = readl((*socfpgaclk).hw.reg.add(0x4));
    let divf = (reg & SOCFPGA_PLL_DIVF_MASK) >> SOCFPGA_PLL_DIVF_SHIFT;
    let divq = (reg & SOCFPGA_PLL_DIVQ_MASK) >> SOCFPGA_PLL_DIVQ_SHIFT;
    let mut vco_freq = (parent_rate as u64).wrapping_mul((divf + 1) as u64);
    vco_freq /= (1 + divq) as u64;
    vco_freq as libc::c_ulong
}

unsafe fn clk_pll_get_parent(hwclk: *mut clk_hw) -> u8 {
    let socfpgaclk = to_socfpga_clk(hwclk);
    let pll_src = readl((*socfpgaclk).hw.reg);
    ((pll_src >> CLK_MGR_PLL_CLK_SRC_SHIFT) & CLK_MGR_PLL_CLK_SRC_MASK) as u8
}

static CLK_PLL_OPS: clk_ops = clk_ops {
    recalc_rate: Some(clk_pll_recalc_rate),
    get_parent: Some(clk_pll_get_parent),
};

unsafe fn __socfpga_pll_init(node: *mut device_node, ops: *const clk_ops) {
    let mut reg: u32 = 0;
    let mut hw_clk: *mut clk_hw;
    let pll_clk: *mut socfpga_pll;
    let mut clk_name = (*node).name;
    let mut parent_name: [*const core::ffi::c_char; SOCFGPA_MAX_PARENTS] = [core::ptr::null(); SOCFGPA_MAX_PARENTS];
    let mut init: clk_init_data = core::mem::zeroed();
    let clkmgr_np: *mut device_node;
    let rc: i32;
    let mut i: usize = 0;

    of_property_read_u32(node, b"reg\0".as_ptr() as *const _, &mut reg);

    pll_clk = kzalloc_obj::<socfpga_pll>();
    if warn_on(pll_clk.is_null()) {
        return;
    }

    clkmgr_np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"altr,clk-mgr\0".as_ptr() as *const _);
    clk_mgr_a10_base_addr = of_iomap(clkmgr_np, 0);
    of_node_put(clkmgr_np);
    bug_on(clk_mgr_a10_base_addr.is_null());
    (*pll_clk).hw.reg = clk_mgr_a10_base_addr.add(reg as usize) as *mut u32;

    of_property_read_string(node, b"clock-output-names\0".as_ptr() as *const _, &mut clk_name);

    init.name = clk_name;
    init.ops = ops;
    init.flags = 0;

    while i < SOCFGPA_MAX_PARENTS {
        parent_name[i] = of_clk_get_parent_name(node, i as i32);
        if parent_name[i].is_null() {
            break;
        }
        i += 1;
    }
    init.num_parents = i as u8;
    init.parent_names = parent_name.as_ptr();
    (*pll_clk).hw.hw.init = &mut init;

    (*pll_clk).hw.bit_idx = SOCFPGA_PLL_EXT_ENA as u8;
    hw_clk = &mut (*pll_clk).hw.hw;

    rc = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if rc != 0 {
        pr_err(b"Could not register clock:%s\n\0".as_ptr() as *const _, clk_name);
        goto_err_clk_hw_register(pll_clk);
        return;
    }

    rc = of_clk_add_hw_provider(node, of_clk_hw_simple_get, hw_clk);
    if rc != 0 {
        pr_err(b"Could not register clock provider for node:%s\n\0".as_ptr() as *const _, clk_name);
        clk_hw_unregister(hw_clk);
        goto_err_clk_hw_register(pll_clk);
    }
}

unsafe fn goto_err_clk_hw_register(pll_clk: *mut socfpga_pll) {
    kfree(pll_clk as *mut core::ffi::c_void);
}

pub unsafe fn socfpga_a10_pll_init(node: *mut device_node) {
    __socfpga_pll_init(node, &CLK_PLL_OPS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
