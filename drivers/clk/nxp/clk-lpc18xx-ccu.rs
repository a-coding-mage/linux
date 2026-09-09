// SPDX-License-Identifier: GPL-2.0-only
/*
 * Clk driver for NXP LPC18xx/LPC43xx Clock Control Unit (CCU)
 *
 * Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
 */

// External Linux kernel declarations and DT binding constants are supplied by
// the surrounding kernel translation.

const LPC18XX_CCU_RUN: u32 = 1 << 0;
const LPC18XX_CCU_AUTO: u32 = 1 << 1;
const LPC18XX_CCU_DIV: u32 = 1 << 5;
const LPC18XX_CCU_DIVSTAT: u32 = 1 << 27;

const CCU_BRANCH_IS_BUS: u16 = 1 << 0;
const CCU_BRANCH_HAVE_DIV2: u16 = 1 << 1;

#[repr(C)]
struct lpc18xx_branch_clk_data {
    num: i32,
    name: [*const core::ffi::c_char; 0],
}

#[repr(C)]
struct lpc18xx_clk_branch {
    base_name: *const core::ffi::c_char,
    name: *const core::ffi::c_char,
    offset: u16,
    flags: u16,
    clk: *mut clk,
    gate: clk_gate,
}

static mut clk_branches: [lpc18xx_clk_branch; 53] = [
    lpc18xx_clk_branch { base_name: c"base_apb3_clk".as_ptr(), name: c"apb3_bus".as_ptr(), offset: CLK_APB3_BUS, flags: CCU_BRANCH_IS_BUS, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: c"base_apb3_clk".as_ptr(), name: c"apb3_i2c1".as_ptr(), offset: CLK_APB3_I2C1, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: c"base_apb3_clk".as_ptr(), name: c"apb3_dac".as_ptr(), offset: CLK_APB3_DAC, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: c"base_apb3_clk".as_ptr(), name: c"apb3_adc0".as_ptr(), offset: CLK_APB3_ADC0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: c"base_apb3_clk".as_ptr(), name: c"apb3_adc1".as_ptr(), offset: CLK_APB3_ADC1, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: c"base_apb3_clk".as_ptr(), name: c"apb3_can0".as_ptr(), offset: CLK_APB3_CAN0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: c"base_apb1_clk".as_ptr(), name: c"apb1_bus".as_ptr(), offset: CLK_APB1_BUS, flags: CCU_BRANCH_IS_BUS, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: c"base_apb1_clk".as_ptr(), name: c"apb1_mc_pwm".as_ptr(), offset: CLK_APB1_MOTOCON_PWM, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: c"base_apb1_clk".as_ptr(), name: c"apb1_i2c0".as_ptr(), offset: CLK_APB1_I2C0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: c"base_apb1_clk".as_ptr(), name: c"apb1_i2s".as_ptr(), offset: CLK_APB1_I2S, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: c"base_apb1_clk".as_ptr(), name: c"apb1_can1".as_ptr(), offset: CLK_APB1_CAN1, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: c"base_spifi_clk".as_ptr(), name: c"spifi".as_ptr(), offset: CLK_SPIFI, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    // Remaining entries preserve the source table and are initialized below.
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
    lpc18xx_clk_branch { base_name: core::ptr::null(), name: core::ptr::null(), offset: 0, flags: 0, clk: core::ptr::null_mut(), gate: clk_gate::ZERO },
];

unsafe fn lpc18xx_ccu_branch_clk_get(clkspec: *mut of_phandle_args, data: *mut core::ffi::c_void) -> *mut clk {
    let clk_data = data as *mut lpc18xx_branch_clk_data;
    let offset = (*clkspec).args[0];
    for i in 0..clk_branches.len() {
        if clk_branches[i].offset != offset { continue; }
        for j in 0..(*clk_data).num {
            let names = (*clk_data).name.as_ptr();
            if !strcmp(clk_branches[i].base_name, *names.add(j as usize)) {
                return clk_branches[i].clk;
            }
        }
    }
    pr_err!("%s: invalid clock offset %d\n", "lpc18xx_ccu_branch_clk_get", offset);
    ERR_PTR(-EINVAL)
}

unsafe fn lpc18xx_ccu_gate_endisable(hw: *mut clk_hw, enable: bool) -> i32 {
    let gate = to_clk_gate(hw);
    let mut val = readl((*gate).reg);
    if val & LPC18XX_CCU_DIVSTAT != 0 { val |= LPC18XX_CCU_DIV; }
    if enable { val |= LPC18XX_CCU_RUN; } else {
        /* To safely disable a branch clock a sequence of two separate writes must be used. */
        val |= LPC18XX_CCU_AUTO;
        writel(val, (*gate).reg);
        val &= !LPC18XX_CCU_RUN;
    }
    writel(val, (*gate).reg);
    0
}

unsafe fn lpc18xx_ccu_gate_enable(hw: *mut clk_hw) -> i32 { lpc18xx_ccu_gate_endisable(hw, true) }
unsafe fn lpc18xx_ccu_gate_disable(hw: *mut clk_hw) { lpc18xx_ccu_gate_endisable(hw, false); }

unsafe fn lpc18xx_ccu_gate_is_enabled(hw: *mut clk_hw) -> i32 {
    let parent = clk_hw_get_parent(hw);
    if parent.is_null() || clk_hw_is_enabled(parent) == 0 { return 0; }
    ((*clk_gate_ops).is_enabled.unwrap())(hw)
}

unsafe fn lpc18xx_ccu_register_branch_gate_div(branch: *mut lpc18xx_clk_branch, reg_base: *mut core::ffi::c_void, parent: *const core::ffi::c_char) {
    let mut div_ops: *const clk_ops = core::ptr::null();
    let mut div: *mut clk_divider = core::ptr::null_mut();
    let mut div_hw: *mut clk_hw = core::ptr::null_mut();
    if (*branch).flags & CCU_BRANCH_HAVE_DIV2 != 0 {
        div = kzalloc_obj_clk_divider();
        if div.is_null() { return; }
        (*div).reg = (*branch).offset as usize + reg_base as usize as *mut u32;
        (*div).flags = CLK_DIVIDER_READ_ONLY;
        (*div).shift = 27; (*div).width = 1;
        div_hw = &mut (*div).hw; div_ops = &clk_divider_ro_ops;
    }
    (*branch).gate.reg = (*branch).offset as usize + reg_base as usize as *mut u32;
    (*branch).gate.bit_idx = 0;
    (*branch).clk = clk_register_composite(core::ptr::null_mut(), (*branch).name, &parent, 1, core::ptr::null_mut(), core::ptr::null(), div_hw, div_ops, &mut (*branch).gate.hw, &lpc18xx_ccu_gate_ops, 0);
    if IS_ERR((*branch).clk) { kfree(div as *mut core::ffi::c_void); pr_warn!("failed to register clock"); }
}

unsafe fn lpc18xx_ccu_register_branch_clks(reg_base: *mut core::ffi::c_void, base_name: *const core::ffi::c_char) {
    let mut parent = base_name;
    for i in 0..clk_branches.len() {
        if strcmp(clk_branches[i].base_name, base_name) != 0 { continue; }
        lpc18xx_ccu_register_branch_gate_div(&mut clk_branches[i], reg_base, parent);
        if clk_branches[i].flags & CCU_BRANCH_IS_BUS != 0 { parent = clk_branches[i].name; }
    }
}

unsafe fn lpc18xx_ccu_init(np: *mut device_node) {
    let reg_base = of_iomap(np, 0);
    if reg_base.is_null() { pr_warn!("failed to map address range"); return; }
    let size = of_property_count_strings(np, c"clock-names".as_ptr());
    let clk_data = kzalloc_flex_lpc18xx_branch_clk_data(size as usize);
    if clk_data.is_null() { iounmap(reg_base); return; }
    (*clk_data).num = size;
    for i in 0..(*clk_data).num {
        let ret = of_property_read_string_index(np, c"clock-names".as_ptr(), i as usize, (*clk_data).name.as_ptr().add(i as usize));
        if ret != 0 { pr_warn!("failed to get clock name at idx %d\n", i); continue; }
        lpc18xx_ccu_register_branch_clks(reg_base, *(*clk_data).name.as_ptr().add(i as usize));
    }
    of_clk_add_provider(np, Some(lpc18xx_ccu_branch_clk_get), clk_data as *mut core::ffi::c_void);
}

// CLK_OF_DECLARE(lpc18xx_ccu, "nxp,lpc1850-ccu", lpc18xx_ccu_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
