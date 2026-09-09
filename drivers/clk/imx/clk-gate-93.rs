// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2022 NXP
 *
 * Peng Fan <peng.fan@nxp.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const DIRECT_OFFSET: usize = 0x0;
const LPM_SETTING_OFF: u32 = 0x0;
const LPM_SETTING_ON: u32 = 0x4;
const LPM_CUR_OFFSET: usize = 0x1c;
const AUTHEN_OFFSET: usize = 0x30;
const CPULPM_EN: u32 = 1 << 2;
const TZ_NS_SHIFT: u32 = 9;
const TZ_NS_MASK: u32 = 1 << 9;
const WHITE_LIST_SHIFT: u32 = 16;

#[repr(C)]
struct Imx93ClkGate {
    hw: ClkHw,
    reg: *mut core::ffi::c_void,
    bit_idx: u32,
    val: u32,
    mask: u32,
    lock: *mut Spinlock,
    share_count: *mut u32,
}

// External kernel types and functions.
#[repr(C)] struct ClkHw { init: *mut ClkInitData }
#[repr(C)] struct ClkInitData {
    name: *const core::ffi::c_char,
    ops: *const ClkOps,
    flags: u32,
    parent_names: *const *const core::ffi::c_char,
    num_parents: u8,
}
#[repr(C)] struct ClkOps {
    enable: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>,
    disable: Option<unsafe extern "C" fn(*mut ClkHw)>,
    disable_unused: Option<unsafe extern "C" fn(*mut ClkHw)>,
    is_enabled: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>,
}
#[repr(C)] struct Spinlock;
#[repr(C)] struct Device;
extern "C" {
    static mut imx_ccm_lock: Spinlock;
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn clk_hw_register(dev: *mut Device, hw: *mut ClkHw) -> i32;
    fn kfree(ptr: *mut Imx93ClkGate);
    fn kzalloc_obj() -> *mut Imx93ClkGate;
    fn err_ptr(error: i32) -> *mut ClkHw;
    fn warn_on(condition: bool) -> bool;
}

const CLK_SET_RATE_PARENT: u32 = 1 << 2;
const CLK_OPS_PARENT_ENABLE: u32 = 1 << 3;

unsafe fn gate_from_hw(hw: *mut ClkHw) -> *mut Imx93ClkGate {
    hw as *mut Imx93ClkGate
}

unsafe fn imx93_clk_gate_do_hardware(hw: *mut ClkHw, enable: bool) {
    let gate = &mut *gate_from_hw(hw);
    let mut val = readl(gate.reg.add(AUTHEN_OFFSET));
    if val & CPULPM_EN != 0 {
        val = if enable { LPM_SETTING_ON } else { LPM_SETTING_OFF };
        writel(val, gate.reg.add(LPM_CUR_OFFSET));
    } else {
        val = readl(gate.reg.add(DIRECT_OFFSET));
        val &= !(gate.mask << gate.bit_idx);
        if enable { val |= (gate.val & gate.mask) << gate.bit_idx; }
        writel(val, gate.reg.add(DIRECT_OFFSET));
    }
}

unsafe extern "C" fn imx93_clk_gate_enable(hw: *mut ClkHw) -> i32 {
    let gate = &mut *gate_from_hw(hw);
    let mut flags = 0usize;
    spin_lock_irqsave(gate.lock, &mut flags);
    if !gate.share_count.is_null() {
        let old = *gate.share_count;
        *gate.share_count = old.wrapping_add(1);
        if old > 0 { spin_unlock_irqrestore(gate.lock, flags); return 0; }
    }
    imx93_clk_gate_do_hardware(hw, true);
    spin_unlock_irqrestore(gate.lock, flags);
    0
}

unsafe extern "C" fn imx93_clk_gate_disable(hw: *mut ClkHw) {
    let gate = &mut *gate_from_hw(hw);
    let mut flags = 0usize;
    spin_lock_irqsave(gate.lock, &mut flags);
    if !gate.share_count.is_null() {
        if warn_on(*gate.share_count == 0) { spin_unlock_irqrestore(gate.lock, flags); return; }
        *gate.share_count = (*gate.share_count).wrapping_sub(1);
        if *gate.share_count > 0 { spin_unlock_irqrestore(gate.lock, flags); return; }
    }
    imx93_clk_gate_do_hardware(hw, false);
    spin_unlock_irqrestore(gate.lock, flags);
}

unsafe fn imx93_clk_gate_reg_is_enabled(gate: *mut Imx93ClkGate) -> i32 {
    let gate = &*gate;
    let mut val = readl(gate.reg.add(AUTHEN_OFFSET));
    if val & CPULPM_EN != 0 {
        val = readl(gate.reg.add(LPM_CUR_OFFSET));
        if val == LPM_SETTING_ON { return 1; }
    } else {
        val = readl(gate.reg);
        if ((val >> gate.bit_idx) & gate.mask) == gate.val { return 1; }
    }
    0
}

unsafe extern "C" fn imx93_clk_gate_is_enabled(hw: *mut ClkHw) -> i32 {
    let gate = &mut *gate_from_hw(hw);
    let mut flags = 0usize;
    spin_lock_irqsave(gate.lock, &mut flags);
    let ret = imx93_clk_gate_reg_is_enabled(gate);
    spin_unlock_irqrestore(gate.lock, flags);
    ret
}

unsafe extern "C" fn imx93_clk_gate_disable_unused(hw: *mut ClkHw) {
    let gate = &mut *gate_from_hw(hw);
    let mut flags = 0usize;
    spin_lock_irqsave(gate.lock, &mut flags);
    if gate.share_count.is_null() || *gate.share_count == 0 { imx93_clk_gate_do_hardware(hw, false); }
    spin_unlock_irqrestore(gate.lock, flags);
}

static IMX93_CLK_GATE_OPS: ClkOps = ClkOps { enable: Some(imx93_clk_gate_enable), disable: Some(imx93_clk_gate_disable), disable_unused: Some(imx93_clk_gate_disable_unused), is_enabled: Some(imx93_clk_gate_is_enabled) };
static IMX93_CLK_GATE_RO_OPS: ClkOps = ClkOps { enable: None, disable: None, disable_unused: None, is_enabled: Some(imx93_clk_gate_is_enabled) };

#[no_mangle]
pub unsafe extern "C" fn imx93_clk_gate(
    dev: *mut Device, name: *const core::ffi::c_char, parent_name: *const core::ffi::c_char,
    flags: u32, reg: *mut core::ffi::c_void, bit_idx: u32, val: u32, mask: u32,
    domain_id: u32, share_count: *mut u32,
) -> *mut ClkHw {
    let gate = kzalloc_obj();
    if gate.is_null() { return err_ptr(-12); }
    (*gate).reg = reg;
    (*gate).lock = &raw mut imx_ccm_lock;
    (*gate).bit_idx = bit_idx;
    (*gate).val = val;
    (*gate).mask = mask;
    (*gate).share_count = share_count;
    let mut init = ClkInitData { name, ops: &IMX93_CLK_GATE_OPS, flags: flags | CLK_SET_RATE_PARENT | CLK_OPS_PARENT_ENABLE, parent_names: if parent_name.is_null() { core::ptr::null() } else { &parent_name }, num_parents: if parent_name.is_null() { 0 } else { 1 } };
    (*gate).hw.init = &mut init;
    let hw = &mut (*gate).hw as *mut ClkHw;
    let authen = readl(reg.add(AUTHEN_OFFSET));
    if authen & TZ_NS_MASK == 0 || authen & (1u32 << (WHITE_LIST_SHIFT + domain_id)) == 0 { init.ops = &IMX93_CLK_GATE_RO_OPS; }
    let ret = clk_hw_register(dev, hw);
    if ret != 0 { kfree(gate); return err_ptr(ret); }
    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
