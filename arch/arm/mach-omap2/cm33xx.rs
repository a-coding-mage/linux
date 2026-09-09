// SPDX-License-Identifier: GPL-2.0-only
/*
 * AM33XX CM functions
 *
 * Copyright (C) 2011-2012 Texas Instruments Incorporated - https://www.ti.com/
 * Vaibhav Hiremath <hvaibhav@ti.com>
 *
 * Reference taken from OMAP4 cminst44xx.c
 */

// Linux dependencies supplied by other translation units.

pub const CLKCTRL_IDLEST_FUNCTIONAL: u32 = 0x0;
pub const CLKCTRL_IDLEST_INTRANSITION: u32 = 0x1;
pub const CLKCTRL_IDLEST_INTERFACE_IDLE: u32 = 0x2;
pub const CLKCTRL_IDLEST_DISABLED: u32 = 0x3;

extern "C" {
    static mut cm_base: CmBase;
    fn readl_relaxed(addr: *const u8) -> u32;
    fn writel_relaxed(val: u32, addr: *mut u8);
    fn __ffs(word: u32) -> u32;
    fn cm_register(data: *const cm_ll_data) -> i32;
    fn cm_unregister(data: *const cm_ll_data);
}

#[repr(C)]
pub struct CmBase {
    pub va: *mut u8,
    pub pa: u32,
}

#[repr(C)]
pub struct clockdomain {
    pub cm_inst: u16,
    pub clkdm_offs: u16,
    pub flags: u32,
    pub context: u32,
}

#[repr(C)]
pub struct clkdm_ops {
    pub clkdm_sleep: Option<unsafe extern "C" fn(*mut clockdomain) -> i32>,
    pub clkdm_wakeup: Option<unsafe extern "C" fn(*mut clockdomain) -> i32>,
    pub clkdm_allow_idle: Option<unsafe extern "C" fn(*mut clockdomain)>,
    pub clkdm_deny_idle: Option<unsafe extern "C" fn(*mut clockdomain)>,
    pub clkdm_clk_enable: Option<unsafe extern "C" fn(*mut clockdomain) -> i32>,
    pub clkdm_clk_disable: Option<unsafe extern "C" fn(*mut clockdomain) -> i32>,
    pub clkdm_save_context: Option<unsafe extern "C" fn(*mut clockdomain) -> i32>,
    pub clkdm_restore_context: Option<unsafe extern "C" fn(*mut clockdomain) -> i32>,
}

#[repr(C)]
pub struct cm_ll_data {
    pub wait_module_ready: Option<unsafe extern "C" fn(u8, i16, u16, u8) -> i32>,
    pub wait_module_idle: Option<unsafe extern "C" fn(u8, i16, u16, u8) -> i32>,
    pub module_enable: Option<unsafe extern "C" fn(u8, u8, u16, u16)>,
    pub module_disable: Option<unsafe extern "C" fn(u8, u16, u16)>,
    pub xlate_clkctrl: Option<unsafe extern "C" fn(u8, u16, u16) -> u32>,
}

extern "C" {
    static pm_suspend_target_state: i32;
}

pub const AM33XX_IDLEST_MASK: u32 = 0;
pub const AM33XX_IDLEST_SHIFT: u32 = 0;
pub const AM33XX_CLKTRCTRL_MASK: u32 = 0;
pub const AM33XX_CLKTRCTRL_SHIFT: u32 = 0;
pub const AM33XX_MODULEMODE_MASK: u32 = 0;
pub const AM33XX_MODULEMODE_SHIFT: u32 = 0;
pub const OMAP34XX_CLKSTCTRL_ENABLE_AUTO: u8 = 0;
pub const OMAP34XX_CLKSTCTRL_DISABLE_AUTO: u8 = 0;
pub const OMAP34XX_CLKSTCTRL_FORCE_SLEEP: u8 = 0;
pub const OMAP34XX_CLKSTCTRL_FORCE_WAKEUP: u8 = 0;
pub const CLKDM_CAN_FORCE_WAKEUP: u32 = 0;
pub const CLKDM_CAN_FORCE_SLEEP: u32 = 0;
pub const CLKDM_STANDBY_FORCE_WAKEUP: u32 = 0;
pub const MAX_MODULE_READY_TIME: i32 = 0;
pub const EBUSY: i32 = 16;

#[inline]
unsafe fn am33xx_cm_read_reg(inst: u16, idx: u16) -> u32 {
    readl_relaxed(cm_base.va.add(inst as usize + idx as usize))
}

#[inline]
unsafe fn am33xx_cm_write_reg(val: u32, inst: u16, idx: u16) {
    writel_relaxed(val, cm_base.va.add(inst as usize + idx as usize));
}

#[inline]
unsafe fn am33xx_cm_rmw_reg_bits(mask: u32, bits: u32, inst: i16, idx: i16) -> u32 {
    let mut v = am33xx_cm_read_reg(inst as u16, idx as u16);
    v &= !mask;
    v |= bits;
    am33xx_cm_write_reg(v, inst as u16, idx as u16);
    v
}

#[inline]
unsafe fn am33xx_cm_read_reg_bits(inst: u16, idx: i16, mask: u32) -> u32 {
    let mut v = am33xx_cm_read_reg(inst, idx as u16);
    v &= mask;
    v >>= __ffs(mask);
    v
}

unsafe fn _clkctrl_idlest(inst: u16, clkctrl_offs: u16) -> u32 {
    let mut v = am33xx_cm_read_reg(inst, clkctrl_offs);
    v &= AM33XX_IDLEST_MASK;
    v >>= AM33XX_IDLEST_SHIFT;
    v
}

unsafe fn _is_module_ready(inst: u16, clkctrl_offs: u16) -> bool {
    let v = _clkctrl_idlest(inst, clkctrl_offs);
    v == CLKCTRL_IDLEST_FUNCTIONAL || v == CLKCTRL_IDLEST_INTERFACE_IDLE
}

unsafe fn _clktrctrl_write(c: u8, inst: u16, cdoffs: u16) {
    let mut v = am33xx_cm_read_reg(inst, cdoffs);
    v &= !AM33XX_CLKTRCTRL_MASK;
    v |= (c as u32) << AM33XX_CLKTRCTRL_SHIFT;
    am33xx_cm_write_reg(v, inst, cdoffs);
}

unsafe fn am33xx_cm_is_clkdm_in_hwsup(inst: u16, cdoffs: u16) -> bool {
    let mut v = am33xx_cm_read_reg(inst, cdoffs);
    v &= AM33XX_CLKTRCTRL_MASK;
    v >>= AM33XX_CLKTRCTRL_SHIFT;
    v == OMAP34XX_CLKSTCTRL_ENABLE_AUTO as u32
}

unsafe fn am33xx_cm_clkdm_enable_hwsup(inst: u16, cdoffs: u16) { _clktrctrl_write(OMAP34XX_CLKSTCTRL_ENABLE_AUTO, inst, cdoffs); }
unsafe fn am33xx_cm_clkdm_disable_hwsup(inst: u16, cdoffs: u16) { _clktrctrl_write(OMAP34XX_CLKSTCTRL_DISABLE_AUTO, inst, cdoffs); }
unsafe fn am33xx_cm_clkdm_force_sleep(inst: u16, cdoffs: u16) { _clktrctrl_write(OMAP34XX_CLKSTCTRL_FORCE_SLEEP, inst, cdoffs); }
unsafe fn am33xx_cm_clkdm_force_wakeup(inst: u16, cdoffs: u16) { _clktrctrl_write(OMAP34XX_CLKSTCTRL_FORCE_WAKEUP, inst, cdoffs); }

unsafe fn am33xx_cm_wait_module_ready(_part: u8, inst: i16, clkctrl_offs: u16, _bit_shift: u8) -> i32 {
    let mut i = 0;
    while !_is_module_ready(inst as u16, clkctrl_offs) && i < MAX_MODULE_READY_TIME { i += 1; }
    if i < MAX_MODULE_READY_TIME { 0 } else { -EBUSY }
}

unsafe fn am33xx_cm_wait_module_idle(_part: u8, inst: i16, clkctrl_offs: u16, _bit_shift: u8) -> i32 {
    let mut i = 0;
    while _clkctrl_idlest(inst as u16, clkctrl_offs) != CLKCTRL_IDLEST_DISABLED && i < MAX_MODULE_READY_TIME { i += 1; }
    if i < MAX_MODULE_READY_TIME { 0 } else { -EBUSY }
}

unsafe fn am33xx_cm_module_enable(mode: u8, _part: u8, inst: u16, clkctrl_offs: u16) {
    let mut v = am33xx_cm_read_reg(inst, clkctrl_offs);
    v &= !AM33XX_MODULEMODE_MASK;
    v |= (mode as u32) << AM33XX_MODULEMODE_SHIFT;
    am33xx_cm_write_reg(v, inst, clkctrl_offs);
}

unsafe fn am33xx_cm_module_disable(_part: u8, inst: u16, clkctrl_offs: u16) {
    let mut v = am33xx_cm_read_reg(inst, clkctrl_offs);
    v &= !AM33XX_MODULEMODE_MASK;
    am33xx_cm_write_reg(v, inst, clkctrl_offs);
}

unsafe fn am33xx_clkdm_sleep(clkdm: *mut clockdomain) -> i32 { am33xx_cm_clkdm_force_sleep((*clkdm).cm_inst, (*clkdm).clkdm_offs); 0 }
unsafe fn am33xx_clkdm_wakeup(clkdm: *mut clockdomain) -> i32 { am33xx_cm_clkdm_force_wakeup((*clkdm).cm_inst, (*clkdm).clkdm_offs); 0 }
unsafe fn am33xx_clkdm_allow_idle(clkdm: *mut clockdomain) { am33xx_cm_clkdm_enable_hwsup((*clkdm).cm_inst, (*clkdm).clkdm_offs); }
unsafe fn am33xx_clkdm_deny_idle(clkdm: *mut clockdomain) { am33xx_cm_clkdm_disable_hwsup((*clkdm).cm_inst, (*clkdm).clkdm_offs); }

unsafe fn am33xx_clkdm_clk_enable(clkdm: *mut clockdomain) -> i32 {
    if (*clkdm).flags & CLKDM_CAN_FORCE_WAKEUP != 0 { return am33xx_clkdm_wakeup(clkdm); }
    0
}

unsafe fn am33xx_clkdm_clk_disable(clkdm: *mut clockdomain) -> i32 {
    let hwsup = am33xx_cm_is_clkdm_in_hwsup((*clkdm).cm_inst, (*clkdm).clkdm_offs);
    if !hwsup && (*clkdm).flags & CLKDM_CAN_FORCE_SLEEP != 0 { am33xx_clkdm_sleep(clkdm); }
    0
}

unsafe fn am33xx_cm_xlate_clkctrl(_part: u8, inst: u16, offset: u16) -> u32 { cm_base.pa + inst as u32 + offset as u32 }

unsafe fn am33xx_clkdm_save_context(clkdm: *mut clockdomain) -> i32 {
    (*clkdm).context = am33xx_cm_read_reg_bits((*clkdm).cm_inst, (*clkdm).clkdm_offs as i16, AM33XX_CLKTRCTRL_MASK);
    0
}

unsafe fn am33xx_clkdm_restore_context(clkdm: *mut clockdomain) -> i32 {
    match (*clkdm).context as u8 {
        OMAP34XX_CLKSTCTRL_DISABLE_AUTO => am33xx_clkdm_deny_idle(clkdm),
        OMAP34XX_CLKSTCTRL_FORCE_SLEEP => { am33xx_clkdm_sleep(clkdm); },
        OMAP34XX_CLKSTCTRL_FORCE_WAKEUP => { am33xx_clkdm_wakeup(clkdm); },
        OMAP34XX_CLKSTCTRL_ENABLE_AUTO => am33xx_clkdm_allow_idle(clkdm),
        _ => {},
    }
    0
}

pub static mut am33xx_clkdm_operations: clkdm_ops = clkdm_ops {
    clkdm_sleep: Some(am33xx_clkdm_sleep), clkdm_wakeup: Some(am33xx_clkdm_wakeup),
    clkdm_allow_idle: Some(am33xx_clkdm_allow_idle), clkdm_deny_idle: Some(am33xx_clkdm_deny_idle),
    clkdm_clk_enable: Some(am33xx_clkdm_clk_enable), clkdm_clk_disable: Some(am33xx_clkdm_clk_disable),
    clkdm_save_context: Some(am33xx_clkdm_save_context), clkdm_restore_context: Some(am33xx_clkdm_restore_context),
};

static am33xx_cm_ll_data: cm_ll_data = cm_ll_data {
    wait_module_ready: Some(am33xx_cm_wait_module_ready), wait_module_idle: Some(am33xx_cm_wait_module_idle),
    module_enable: Some(am33xx_cm_module_enable), module_disable: Some(am33xx_cm_module_disable),
    xlate_clkctrl: Some(am33xx_cm_xlate_clkctrl),
};

pub unsafe fn am33xx_cm_init(_data: *const omap_prcm_init_data) -> i32 { cm_register(&am33xx_cm_ll_data) }
unsafe fn am33xx_cm_exit() { cm_unregister(&am33xx_cm_ll_data); }

#[repr(C)]
pub struct omap_prcm_init_data;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
