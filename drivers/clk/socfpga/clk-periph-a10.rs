// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Altera Corporation. All rights reserved
 */

// Linux dependencies supplied by the surrounding translation unit:
// slab, clk-provider, io, of, and clk.h.

const CLK_MGR_FREE_SHIFT: u32 = 16;
const CLK_MGR_FREE_MASK: u32 = 0x7;

const SOCFPGA_MPU_FREE_CLK: &str = "mpu_free_clk";
const SOCFPGA_NOC_FREE_CLK: &str = "noc_free_clk";
const SOCFPGA_SDMMC_FREE_CLK: &str = "sdmmc_free_clk";

unsafe fn to_socfpga_periph_clk(
    p: *mut crate::clk::clk_hw,
) -> *mut crate::clk::socfpga_periph_clk {
    crate::container_of!(p, crate::clk::socfpga_periph_clk, hw.hw)
}

unsafe extern "C" {
    static mut clk_mgr_a10_base_addr: *mut core::ffi::c_void;
}

unsafe fn clk_periclk_recalc_rate(
    hwclk: *mut crate::clk::clk_hw,
    parent_rate: usize,
) -> usize {
    let socfpgaclk = to_socfpga_periph_clk(hwclk);
    let div: u32;

    if (*socfpgaclk).fixed_div != 0 {
        div = (*socfpgaclk).fixed_div;
    } else if !(*socfpgaclk).div_reg.is_null() {
        div = (crate::readl((*socfpgaclk).div_reg) >> (*socfpgaclk).shift)
            & ((1u32 << (*socfpgaclk).width) - 1);
        div = div + 1;
    } else {
        div = (crate::readl((*socfpgaclk).hw.reg) & 0x7ff) + 1;
    }

    parent_rate / div as usize
}

unsafe fn clk_periclk_get_parent(hwclk: *mut crate::clk::clk_hw) -> u8 {
    let socfpgaclk = to_socfpga_periph_clk(hwclk);
    let clk_src: u32;
    let name = crate::clk_hw_get_name(hwclk);

    clk_src = crate::readl((*socfpgaclk).hw.reg);
    if crate::streq(name, SOCFPGA_MPU_FREE_CLK)
        || crate::streq(name, SOCFPGA_NOC_FREE_CLK)
        || crate::streq(name, SOCFPGA_SDMMC_FREE_CLK)
    {
        ((clk_src >> CLK_MGR_FREE_SHIFT) & CLK_MGR_FREE_MASK) as u8
    } else {
        0
    }
}

static periclk_ops: crate::clk::clk_ops = crate::clk::clk_ops {
    recalc_rate: Some(clk_periclk_recalc_rate),
    get_parent: Some(clk_periclk_get_parent),
};

unsafe fn __socfpga_periph_init(
    node: *mut crate::of::device_node,
    ops: *const crate::clk::clk_ops,
) {
    let mut reg: u32 = 0;
    let mut hw_clk: *mut crate::clk::clk_hw;
    let mut periph_clk: *mut crate::clk::socfpga_periph_clk;
    let mut clk_name = (*node).name;
    let mut parent_name: [*const core::ffi::c_char; crate::clk::SOCFPGA_MAX_PARENTS] =
        [core::ptr::null(); crate::clk::SOCFPGA_MAX_PARENTS];
    let mut init: crate::clk::clk_init_data;
    let mut rc: i32;
    let mut fixed_div: u32 = 0;
    let mut div_reg: [u32; 3] = [0; 3];

    crate::of_property_read_u32(node, b"reg\0".as_ptr(), &mut reg);

    periph_clk = crate::kzalloc_obj();
    if crate::WARN_ON(periph_clk.is_null()) {
        return;
    }

    (*periph_clk).hw.reg = (clk_mgr_a10_base_addr as *mut u8).add(reg as usize)
        as *mut u32;

    rc = crate::of_property_read_u32_array(node, b"div-reg\0".as_ptr(), div_reg.as_mut_ptr(), 3);
    if rc == 0 {
        (*periph_clk).div_reg = (clk_mgr_a10_base_addr as *mut u8).add(div_reg[0] as usize)
            as *mut u32;
        (*periph_clk).shift = div_reg[1];
        (*periph_clk).width = div_reg[2];
    } else {
        (*periph_clk).div_reg = core::ptr::null_mut();
    }

    rc = crate::of_property_read_u32(node, b"fixed-divider\0".as_ptr(), &mut fixed_div);
    if rc != 0 {
        (*periph_clk).fixed_div = 0;
    } else {
        (*periph_clk).fixed_div = fixed_div;
    }

    crate::of_property_read_string(node, b"clock-output-names\0".as_ptr(), &mut clk_name);

    init.name = clk_name;
    init.ops = ops;
    init.flags = 0;
    init.num_parents = crate::of_clk_parent_fill(node, parent_name.as_mut_ptr(), crate::clk::SOCFPGA_MAX_PARENTS);
    init.parent_names = parent_name.as_ptr();
    (*periph_clk).hw.hw.init = &init;
    hw_clk = &mut (*periph_clk).hw.hw;

    rc = crate::clk_hw_register(core::ptr::null_mut(), hw_clk);
    if rc != 0 {
        crate::pr_err(b"Could not register clock:%s\n\0".as_ptr(), clk_name);
        goto_err_clk_hw_register(periph_clk);
        return;
    }

    rc = crate::of_clk_add_hw_provider(node, crate::of_clk_hw_simple_get, hw_clk);
    if rc != 0 {
        crate::pr_err(b"Could not register clock provider for node:%s\n\0".as_ptr(), clk_name);
        crate::clk_hw_unregister(hw_clk);
        goto_err_clk_hw_register(periph_clk);
    }
}

unsafe fn goto_err_clk_hw_register(periph_clk: *mut crate::clk::socfpga_periph_clk) {
    crate::kfree(periph_clk);
}

pub unsafe fn socfpga_a10_periph_init(node: *mut crate::of::device_node) {
    __socfpga_periph_init(node, &periclk_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
