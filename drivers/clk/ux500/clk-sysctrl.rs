// SPDX-License-Identifier: GPL-2.0-only
/*
 * Sysctrl clock implementation for ux500 platform.
 *
 * Copyright (C) 2013 ST-Ericsson SA
 * Author: Ulf Hansson <ulf.hansson@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation.

const SYSCTRL_MAX_NUM_PARENTS: usize = 4;

#[repr(C)]
pub struct clk_sysctrl {
    pub hw: clk_hw,
    pub dev: *mut device,
    pub parent_index: u8,
    pub reg_sel: [u16; SYSCTRL_MAX_NUM_PARENTS],
    pub reg_mask: [u8; SYSCTRL_MAX_NUM_PARENTS],
    pub reg_bits: [u8; SYSCTRL_MAX_NUM_PARENTS],
    pub rate: c_ulong,
    pub enable_delay_us: c_ulong,
}

unsafe fn to_clk_sysctrl(hw: *mut clk_hw) -> *mut clk_sysctrl {
    // Equivalent to container_of(_hw, struct clk_sysctrl, hw).
    (hw as *mut u8).sub(core::mem::offset_of!(clk_sysctrl, hw)) as *mut clk_sysctrl
}

/* Sysctrl clock operations. */

unsafe extern "C" fn clk_sysctrl_prepare(hw: *mut clk_hw) -> c_int {
    let clk = &mut *to_clk_sysctrl(hw);
    let ret = ab8500_sysctrl_write(clk.reg_sel[0], clk.reg_mask[0], clk.reg_bits[0]);

    if ret == 0 && clk.enable_delay_us != 0 {
        usleep_range(clk.enable_delay_us, clk.enable_delay_us + (clk.enable_delay_us >> 2));
    }

    ret
}

unsafe extern "C" fn clk_sysctrl_unprepare(hw: *mut clk_hw) {
    let clk = &mut *to_clk_sysctrl(hw);
    if ab8500_sysctrl_clear(clk.reg_sel[0], clk.reg_mask[0]) != 0 {
        dev_err(clk.dev, "clk_sysctrl: %s fail to clear %s.\n", c_str!(__func__), clk_hw_get_name(hw));
    }
}

unsafe extern "C" fn clk_sysctrl_recalc_rate(_hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    (*to_clk_sysctrl(_hw)).rate
}

unsafe extern "C" fn clk_sysctrl_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    let clk = &mut *to_clk_sysctrl(hw);
    let old_index = clk.parent_index;
    let mut ret: c_int = 0;

    if clk.reg_sel[old_index as usize] != 0 {
        ret = ab8500_sysctrl_clear(clk.reg_sel[old_index as usize], clk.reg_mask[old_index as usize]);
        if ret != 0 { return ret; }
    }

    if clk.reg_sel[index as usize] != 0 {
        ret = ab8500_sysctrl_write(clk.reg_sel[index as usize], clk.reg_mask[index as usize], clk.reg_bits[index as usize]);
        if ret != 0 {
            if clk.reg_sel[old_index as usize] != 0 {
                ab8500_sysctrl_write(clk.reg_sel[old_index as usize], clk.reg_mask[old_index as usize], clk.reg_bits[old_index as usize]);
            }
            return ret;
        }
    }
    clk.parent_index = index;
    ret
}

unsafe extern "C" fn clk_sysctrl_get_parent(hw: *mut clk_hw) -> u8 {
    (*to_clk_sysctrl(hw)).parent_index
}

pub static mut clk_sysctrl_gate_ops: clk_ops = clk_ops {
    prepare: Some(clk_sysctrl_prepare), unprepare: Some(clk_sysctrl_unprepare), ..clk_ops::EMPTY
};
pub static mut clk_sysctrl_gate_fixed_rate_ops: clk_ops = clk_ops {
    prepare: Some(clk_sysctrl_prepare), unprepare: Some(clk_sysctrl_unprepare), recalc_rate: Some(clk_sysctrl_recalc_rate), ..clk_ops::EMPTY
};
pub static mut clk_sysctrl_set_parent_ops: clk_ops = clk_ops {
    determine_rate: Some(clk_hw_determine_rate_no_reparent), set_parent: Some(clk_sysctrl_set_parent), get_parent: Some(clk_sysctrl_get_parent), ..clk_ops::EMPTY
};

unsafe fn clk_reg_sysctrl(dev: *mut device, name: *const c_char, parent_names: *const *const c_char, num_parents: u8, reg_sel: *mut u16, reg_mask: *mut u8, reg_bits: *mut u8, rate: c_ulong, enable_delay_us: c_ulong, flags: c_ulong, ops: *const clk_ops) -> *mut clk {
    if dev.is_null() { return ERR_PTR(-EINVAL); }
    if name.is_null() || (num_parents as usize > SYSCTRL_MAX_NUM_PARENTS) { dev_err(dev, "clk_sysctrl: invalid arguments passed\n"); return ERR_PTR(-EINVAL); }
    let clk = devm_kzalloc(dev, core::mem::size_of::<clk_sysctrl>(), GFP_KERNEL) as *mut clk_sysctrl;
    if clk.is_null() { return ERR_PTR(-ENOMEM); }
    (*clk).reg_sel[0] = *reg_sel; (*clk).reg_bits[0] = *reg_bits; (*clk).reg_mask[0] = *reg_mask;
    for i in 1..num_parents as usize { (*clk).reg_sel[i] = *reg_sel.add(i); (*clk).reg_bits[i] = *reg_bits.add(i); (*clk).reg_mask[i] = *reg_mask.add(i); }
    (*clk).parent_index = 0; (*clk).rate = rate; (*clk).enable_delay_us = enable_delay_us; (*clk).dev = dev;
    let mut init = clk_init_data { name, ops, flags, parent_names, num_parents };
    (*clk).hw.init = &mut init;
    let clk_reg = devm_clk_register(dev, &mut (*clk).hw);
    if IS_ERR(clk_reg) { dev_err(dev, "clk_sysctrl: clk_register failed\n"); }
    clk_reg
}

pub unsafe extern "C" fn clk_reg_sysctrl_gate(dev: *mut device, name: *const c_char, parent_name: *const c_char, reg_sel: u16, reg_mask: u8, reg_bits: u8, enable_delay_us: c_ulong, flags: c_ulong) -> *mut clk {
    let parents = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    clk_reg_sysctrl(dev, name, parents, if !parent_name.is_null() { 1 } else { 0 }, &reg_sel as *const _ as *mut _, &reg_mask as *const _ as *mut _, &reg_bits as *const _ as *mut _, 0, enable_delay_us, flags, &clk_sysctrl_gate_ops)
}

pub unsafe extern "C" fn clk_reg_sysctrl_gate_fixed_rate(dev: *mut device, name: *const c_char, parent_name: *const c_char, reg_sel: u16, reg_mask: u8, reg_bits: u8, rate: c_ulong, enable_delay_us: c_ulong, flags: c_ulong) -> *mut clk {
    let parents = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    clk_reg_sysctrl(dev, name, parents, if !parent_name.is_null() { 1 } else { 0 }, &reg_sel as *const _ as *mut _, &reg_mask as *const _ as *mut _, &reg_bits as *const _ as *mut _, rate, enable_delay_us, flags, &clk_sysctrl_gate_fixed_rate_ops)
}

pub unsafe extern "C" fn clk_reg_sysctrl_set_parent(dev: *mut device, name: *const c_char, parent_names: *const *const c_char, num_parents: u8, reg_sel: *mut u16, reg_mask: *mut u8, reg_bits: *mut u8, flags: c_ulong) -> *mut clk {
    clk_reg_sysctrl(dev, name, parent_names, num_parents, reg_sel, reg_mask, reg_bits, 0, 0, flags, &clk_sysctrl_set_parent_ops)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
