// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016-2017 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// C dependencies: linux/clk-provider.h, linux/kernel.h, linux/of.h,
// linux/regmap.h, linux/slab.h, linux/mfd/syscon.h, and
// dt-bindings/clock/boston-clock.h.

const BOSTON_PLAT_MMCMDIV: u32 = 0x30;
const BOSTON_PLAT_MMCMDIV_CLK0DIV: u32 = 0xff << 0;
const BOSTON_PLAT_MMCMDIV_INPUT: u32 = 0xff << 8;
const BOSTON_PLAT_MMCMDIV_MUL: u32 = 0xff << 16;
const BOSTON_PLAT_MMCMDIV_CLK1DIV: u32 = 0xff << 24;

const BOSTON_CLK_COUNT: usize = 3;

// Supplied by dt-bindings/clock/boston-clock.h.
const BOSTON_CLK_INPUT: usize = 0;
const BOSTON_CLK_SYS: usize = 1;
const BOSTON_CLK_CPU: usize = 2;

unsafe fn ext_field(val: u32, mask: u32) -> u32 {
    (val & mask) >> (mask.trailing_zeros())
}

unsafe fn clk_boston_setup(np: *mut device_node) {
    let mut in_freq: libc::c_ulong;
    let mut cpu_freq: libc::c_ulong;
    let mut sys_freq: libc::c_ulong;
    let mut mmcmdiv: libc::c_uint = 0;
    let mut mul: libc::c_uint;
    let mut cpu_div: libc::c_uint;
    let mut sys_div: libc::c_uint;
    let mut onecell: *mut clk_hw_onecell_data;
    let regmap: *mut regmap;
    let mut hw: *mut clk_hw;
    let mut err: libc::c_int;

    regmap = syscon_node_to_regmap((*np).parent);
    if IS_ERR(regmap) {
        pr_err!("failed to find regmap\n");
        return;
    }

    err = regmap_read(regmap, BOSTON_PLAT_MMCMDIV, &mut mmcmdiv);
    if err != 0 {
        pr_err!("failed to read mmcm_div register: %d\n", err);
        return;
    }

    in_freq = (ext_field(mmcmdiv, BOSTON_PLAT_MMCMDIV_INPUT) as libc::c_ulong)
        .wrapping_mul(1_000_000);
    mul = ext_field(mmcmdiv, BOSTON_PLAT_MMCMDIV_MUL);

    sys_div = ext_field(mmcmdiv, BOSTON_PLAT_MMCMDIV_CLK0DIV);
    sys_freq = mult_frac(in_freq, mul, sys_div);

    cpu_div = ext_field(mmcmdiv, BOSTON_PLAT_MMCMDIV_CLK1DIV);
    cpu_freq = mult_frac(in_freq, mul, cpu_div);

    // Equivalent to kzalloc_flex(*onecell, hws, BOSTON_CLK_COUNT).
    onecell = kzalloc_flex_clk_hw_onecell_data(BOSTON_CLK_COUNT);
    if onecell.is_null() {
        return;
    }

    (*onecell).num = BOSTON_CLK_COUNT;

    hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), c"input".as_ptr(),
                                    core::ptr::null(), 0, in_freq);
    if IS_ERR(hw) {
        pr_err!("failed to register input clock: %pe\n", hw);
        goto_fail_input(onecell);
        return;
    }
    (*onecell).hws[BOSTON_CLK_INPUT] = hw;

    hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), c"sys".as_ptr(),
                                    c"input".as_ptr(), 0, sys_freq);
    if IS_ERR(hw) {
        pr_err!("failed to register sys clock: %pe\n", hw);
        clk_hw_unregister_fixed_rate((*onecell).hws[BOSTON_CLK_INPUT]);
        goto_fail_input(onecell);
        return;
    }
    (*onecell).hws[BOSTON_CLK_SYS] = hw;

    hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), c"cpu".as_ptr(),
                                    c"input".as_ptr(), 0, cpu_freq);
    if IS_ERR(hw) {
        pr_err!("failed to register cpu clock: %pe\n", hw);
        clk_hw_unregister_fixed_rate((*onecell).hws[BOSTON_CLK_SYS]);
        clk_hw_unregister_fixed_rate((*onecell).hws[BOSTON_CLK_INPUT]);
        goto_fail_input(onecell);
        return;
    }
    (*onecell).hws[BOSTON_CLK_CPU] = hw;

    err = of_clk_add_hw_provider(np, of_clk_hw_onecell_get, onecell);
    if err != 0 {
        pr_err!("failed to add DT provider: %d\n", err);
        clk_hw_unregister_fixed_rate((*onecell).hws[BOSTON_CLK_CPU]);
        clk_hw_unregister_fixed_rate((*onecell).hws[BOSTON_CLK_SYS]);
        clk_hw_unregister_fixed_rate((*onecell).hws[BOSTON_CLK_INPUT]);
        goto_fail_input(onecell);
    }
}

unsafe fn goto_fail_input(onecell: *mut clk_hw_onecell_data) {
    kfree(onecell as *mut libc::c_void);
}

// Use CLK_OF_DECLARE so this driver is probed early enough to provide the CPU
// frequency for use with the GIC or cop0 counters/timers.
CLK_OF_DECLARE!(clk_boston, "img,boston-clock", clk_boston_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
