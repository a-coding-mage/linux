// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright 2011-2012 Calxeda, Inc.
 *  Copyright (C) 2012-2013 Altera Corporation <www.altera.com>
 *
 * Based from clk-highbank.c
 */

// External Linux kernel declarations and included definitions are supplied by
// the surrounding translation unit.

const SOCFPGA_L4_MP_CLK: &str = "l4_mp_clk";
const SOCFPGA_L4_SP_CLK: &str = "l4_sp_clk";
const SOCFPGA_NAND_CLK: &str = "nand_clk";
const SOCFPGA_NAND_X_CLK: &str = "nand_x_clk";
const SOCFPGA_MMC_CLK: &str = "sdmmc_clk";
const SOCFPGA_GPIO_DB_CLK_OFFSET: usize = 0xA8;

// #define to_socfpga_gate_clk(p) container_of(p, struct socfpga_gate_clk, hw.hw)

/* SDMMC Group for System Manager defines */
const SYSMGR_SDMMCGRP_CTRL_OFFSET: usize = 0x108;

unsafe fn socfpga_clk_get_parent(hwclk: *mut clk_hw) -> u8 {
    let mut l4_src: u32;
    let mut perpll_src: u32;
    let name: *const c_char = clk_hw_get_name(hwclk);

    if streq(name, SOCFPGA_L4_MP_CLK) {
        l4_src = readl(clk_mgr_base_addr.add(CLKMGR_L4SRC));
        return (l4_src & 0x1) as u8;
    }
    if streq(name, SOCFPGA_L4_SP_CLK) {
        l4_src = readl(clk_mgr_base_addr.add(CLKMGR_L4SRC));
        return (!!(l4_src & 2)) as u8;
    }

    perpll_src = readl(clk_mgr_base_addr.add(CLKMGR_PERPLL_SRC));
    if streq(name, SOCFPGA_MMC_CLK) {
        return (perpll_src & 0x3) as u8;
    }
    if streq(name, SOCFPGA_NAND_CLK) || streq(name, SOCFPGA_NAND_X_CLK) {
        return ((perpll_src >> 2) & 3) as u8;
    }

    /* QSPI clock */
    ((perpll_src >> 4) & 3) as u8
}

unsafe fn socfpga_clk_set_parent(hwclk: *mut clk_hw, parent: u8) -> i32 {
    let mut src_reg: u32;
    let name: *const c_char = clk_hw_get_name(hwclk);

    if streq(name, SOCFPGA_L4_MP_CLK) {
        src_reg = readl(clk_mgr_base_addr.add(CLKMGR_L4SRC));
        src_reg &= !0x1;
        src_reg |= parent as u32;
        writel(src_reg, clk_mgr_base_addr.add(CLKMGR_L4SRC));
    } else if streq(name, SOCFPGA_L4_SP_CLK) {
        src_reg = readl(clk_mgr_base_addr.add(CLKMGR_L4SRC));
        src_reg &= !0x2;
        src_reg |= (parent as u32) << 1;
        writel(src_reg, clk_mgr_base_addr.add(CLKMGR_L4SRC));
    } else {
        src_reg = readl(clk_mgr_base_addr.add(CLKMGR_PERPLL_SRC));
        if streq(name, SOCFPGA_MMC_CLK) {
            src_reg &= !0x3;
            src_reg |= parent as u32;
        } else if streq(name, SOCFPGA_NAND_CLK) || streq(name, SOCFPGA_NAND_X_CLK) {
            src_reg &= !0xC;
            src_reg |= (parent as u32) << 2;
        } else {
            /* QSPI clock */
            src_reg &= !0x30;
            src_reg |= (parent as u32) << 4;
        }
        writel(src_reg, clk_mgr_base_addr.add(CLKMGR_PERPLL_SRC));
    }

    0
}

unsafe fn socfpga_clk_get_div(socfpgaclk: *mut socfpga_gate_clk) -> u32 {
    let mut div: u32 = 1;
    let mut val: u32;

    if (*socfpgaclk).fixed_div != 0 {
        div = (*socfpgaclk).fixed_div;
    } else if !(*socfpgaclk).div_reg.is_null() {
        val = readl((*socfpgaclk).div_reg) >> (*socfpgaclk).shift;
        val &= (1u32 << (*socfpgaclk).width) - 1;
        /* Check for GPIO_DB_CLK by its offset */
        if (*socfpgaclk).div_reg as usize & SOCFPGA_GPIO_DB_CLK_OFFSET != 0 {
            div = val + 1;
        } else {
            div = 1u32 << val;
        }
    }

    div
}

unsafe fn socfpga_clk_recalc_rate(hwclk: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let socfpgaclk: *mut socfpga_gate_clk = container_of_socfpga_gate_clk(hwclk);
    let div: u32 = socfpga_clk_get_div(socfpgaclk);
    parent_rate / div as c_ulong
}

unsafe fn socfpga_clk_determine_rate(
    hwclk: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let socfpgaclk: *mut socfpga_gate_clk = container_of_socfpga_gate_clk(hwclk);
    let div: u32 = socfpga_clk_get_div(socfpgaclk);

    (*req).rate = (*req).best_parent_rate / div as c_ulong;
    0
}

static mut gateclk_ops: clk_ops = clk_ops {
    recalc_rate: Some(socfpga_clk_recalc_rate),
    determine_rate: Some(socfpga_clk_determine_rate),
    get_parent: Some(socfpga_clk_get_parent),
    set_parent: Some(socfpga_clk_set_parent),
    ..clk_ops::zeroed()
};

pub unsafe fn socfpga_gate_init(node: *mut device_node) {
    let mut clk_gate = [0u32; 2];
    let mut div_reg = [0u32; 3];
    let mut fixed_div: u32;
    let mut hw_clk: *mut clk_hw;
    let mut socfpga_clk: *mut socfpga_gate_clk;
    let mut clk_name: *const c_char = (*node).name;
    let mut parent_name = [core::ptr::null::<c_char>(); SOCFPGA_MAX_PARENTS];
    let mut init: clk_init_data;
    let mut ops: *mut clk_ops;
    let mut rc: i32;

    socfpga_clk = kzalloc_obj::<socfpga_gate_clk>();
    if WARN_ON(socfpga_clk.is_null()) {
        return;
    }

    ops = kmemdup(&gateclk_ops, core::mem::size_of::<clk_ops>(), GFP_KERNEL);
    if WARN_ON(ops.is_null()) {
        kfree(socfpga_clk);
        return;
    }

    rc = of_property_read_u32_array(node, "clk-gate", clk_gate.as_mut_ptr(), 2);
    if rc != 0 {
        clk_gate[0] = 0;
    }

    if clk_gate[0] != 0 {
        (*socfpga_clk).hw.reg = clk_mgr_base_addr.add(clk_gate[0] as usize);
        (*socfpga_clk).hw.bit_idx = clk_gate[1];
        (*ops).enable = clk_gate_ops.enable;
        (*ops).disable = clk_gate_ops.disable;
    }

    rc = of_property_read_u32(node, "fixed-divider", &mut fixed_div);
    (*socfpga_clk).fixed_div = if rc != 0 { 0 } else { fixed_div };

    rc = of_property_read_u32_array(node, "div-reg", div_reg.as_mut_ptr(), 3);
    if rc == 0 {
        (*socfpga_clk).div_reg = clk_mgr_base_addr.add(div_reg[0] as usize);
        (*socfpga_clk).shift = div_reg[1];
        (*socfpga_clk).width = div_reg[2];
    } else {
        (*socfpga_clk).div_reg = core::ptr::null_mut();
    }

    of_property_read_string(node, "clock-output-names", &mut clk_name);

    init.name = clk_name;
    init.ops = ops;
    init.flags = 0;
    init.num_parents = of_clk_parent_fill(node, parent_name.as_mut_ptr(), SOCFPGA_MAX_PARENTS);
    if init.num_parents < 2 {
        (*ops).get_parent = None;
        (*ops).set_parent = None;
    }
    init.parent_names = parent_name.as_ptr();
    (*socfpga_clk).hw.hw.init = &mut init;
    hw_clk = &mut (*socfpga_clk).hw.hw;

    rc = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if rc != 0 {
        pr_err("Could not register clock:%s\n", clk_name);
        kfree(ops);
        kfree(socfpga_clk);
        return;
    }

    rc = of_clk_add_hw_provider(node, of_clk_hw_simple_get, hw_clk);
    if rc != 0 {
        pr_err("Could not register clock provider for node:%s\n", clk_name);
        clk_hw_unregister(hw_clk);
        kfree(ops);
        kfree(socfpga_clk);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
