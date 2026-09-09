// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP2xxx CM module functions
 *
 * Copyright (C) 2009 Nokia Corporation
 * Copyright (C) 2008-2010, 2012 Texas Instruments, Inc.
 * Paul Walmsley
 * Rajendra Nayak <rnayak@ti.com>
 */

// C dependencies supplied by the surrounding translation unit.

const DPLL_AUTOIDLE_DISABLE: u8 = 0x0;
const OMAP2XXX_DPLL_AUTOIDLE_LOW_POWER_STOP: u8 = 0x3;
const OMAP2XXX_APLL_AUTOIDLE_DISABLE: u8 = 0x0;
const OMAP2XXX_APLL_AUTOIDLE_LOW_POWER_STOP: u8 = 0x3;
const EN_APLL_LOCKED: u8 = 3;

static OMAP2XXX_CM_IDLEST_OFFS: [u8; 4] = [CM_IDLEST1, CM_IDLEST2, OMAP2430_CM_IDLEST3, OMAP24XX_CM_IDLEST4];

unsafe fn _write_clktrctrl(c: u8, module: i16, mask: u32) {
    let mut v = omap2_cm_read_mod_reg(module, OMAP2_CM_CLKSTCTRL);
    v &= !mask;
    v |= (c as u32) << mask.trailing_zeros();
    omap2_cm_write_mod_reg(v, module, OMAP2_CM_CLKSTCTRL);
}

unsafe fn omap2xxx_cm_is_clkdm_in_hwsup(module: i16, mask: u32) -> bool {
    let mut v = omap2_cm_read_mod_reg(module, OMAP2_CM_CLKSTCTRL);
    v &= mask;
    v >>= mask.trailing_zeros();
    v == OMAP24XX_CLKSTCTRL_ENABLE_AUTO
}

unsafe fn omap2xxx_cm_clkdm_enable_hwsup(module: i16, mask: u32) {
    _write_clktrctrl(OMAP24XX_CLKSTCTRL_ENABLE_AUTO as u8, module, mask);
}

unsafe fn omap2xxx_cm_clkdm_disable_hwsup(module: i16, mask: u32) {
    _write_clktrctrl(OMAP24XX_CLKSTCTRL_DISABLE_AUTO as u8, module, mask);
}

unsafe fn _omap2xxx_set_dpll_autoidle(m: u8) {
    let mut v = omap2_cm_read_mod_reg(PLL_MOD, CM_AUTOIDLE);
    v &= !OMAP24XX_AUTO_DPLL_MASK;
    v |= (m as u32) << OMAP24XX_AUTO_DPLL_SHIFT;
    omap2_cm_write_mod_reg(v, PLL_MOD, CM_AUTOIDLE);
}

pub unsafe fn omap2xxx_cm_set_dpll_disable_autoidle() {
    _omap2xxx_set_dpll_autoidle(OMAP2XXX_DPLL_AUTOIDLE_LOW_POWER_STOP);
}

pub unsafe fn omap2xxx_cm_set_dpll_auto_low_power_stop() {
    _omap2xxx_set_dpll_autoidle(DPLL_AUTOIDLE_DISABLE);
}

unsafe fn omap2xxx_cm_split_idlest_reg(idlest_reg: *mut clk_omap_reg, prcm_inst: *mut i16, idlest_reg_id: *mut u8) -> i32 {
    let mut idlest_offs: u8;
    let mut i: usize = 0;
    idlest_offs = (*idlest_reg).offset as u8;
    while i < OMAP2XXX_CM_IDLEST_OFFS.len() {
        if idlest_offs == OMAP2XXX_CM_IDLEST_OFFS[i] {
            *idlest_reg_id = (i + 1) as u8;
            break;
        }
        i += 1;
    }
    if i == OMAP2XXX_CM_IDLEST_OFFS.len() { return -EINVAL; }
    let offs = ((*idlest_reg).offset as u64 & 0xff00) as i16;
    *prcm_inst = offs;
    0
}

unsafe fn omap2xxx_cm_wait_module_ready(_part: u8, prcm_mod: i16, idlest_id: u16, idlest_shift: u8) -> i32 {
    let mut i: i32 = 0;
    if idlest_id == 0 || idlest_id > OMAP2XXX_CM_IDLEST_OFFS.len() as u16 { return -EINVAL; }
    let cm_idlest_reg = OMAP2XXX_CM_IDLEST_OFFS[(idlest_id - 1) as usize];
    let mask = 1u32 << idlest_shift;
    while i < MAX_MODULE_READY_TIME {
        if (omap2_cm_read_mod_reg(prcm_mod, cm_idlest_reg) & mask) == mask { break; }
        i += 1;
    }
    if i < MAX_MODULE_READY_TIME { 0 } else { -EBUSY }
}

unsafe fn omap2xxx_clkdm_allow_idle(clkdm: *mut clockdomain) {
    omap2xxx_cm_clkdm_enable_hwsup((*(*clkdm).pwrdm.ptr).prcm_offs, (*clkdm).clktrctrl_mask);
}
unsafe fn omap2xxx_clkdm_deny_idle(clkdm: *mut clockdomain) {
    omap2xxx_cm_clkdm_disable_hwsup((*(*clkdm).pwrdm.ptr).prcm_offs, (*clkdm).clktrctrl_mask);
}
unsafe fn omap2xxx_clkdm_clk_enable(clkdm: *mut clockdomain) -> i32 {
    if (*clkdm).clktrctrl_mask == 0 { return 0; }
    let hwsup = omap2xxx_cm_is_clkdm_in_hwsup((*(*clkdm).pwrdm.ptr).prcm_offs, (*clkdm).clktrctrl_mask);
    if !hwsup && ((*clkdm).flags & CLKDM_CAN_FORCE_WAKEUP) != 0 { omap2xxx_clkdm_wakeup(clkdm); }
    0
}
unsafe fn omap2xxx_clkdm_clk_disable(clkdm: *mut clockdomain) -> i32 {
    if (*clkdm).clktrctrl_mask == 0 { return 0; }
    let hwsup = omap2xxx_cm_is_clkdm_in_hwsup((*(*clkdm).pwrdm.ptr).prcm_offs, (*clkdm).clktrctrl_mask);
    if !hwsup && ((*clkdm).flags & CLKDM_CAN_FORCE_SLEEP) != 0 { omap2xxx_clkdm_sleep(clkdm); }
    0
}

pub static mut omap2_clkdm_operations: clkdm_ops = clkdm_ops {
    clkdm_add_wkdep: omap2_clkdm_add_wkdep, clkdm_del_wkdep: omap2_clkdm_del_wkdep,
    clkdm_read_wkdep: omap2_clkdm_read_wkdep, clkdm_clear_all_wkdeps: omap2_clkdm_clear_all_wkdeps,
    clkdm_sleep: omap2xxx_clkdm_sleep, clkdm_wakeup: omap2xxx_clkdm_wakeup,
    clkdm_allow_idle: omap2xxx_clkdm_allow_idle, clkdm_deny_idle: omap2xxx_clkdm_deny_idle,
    clkdm_clk_enable: omap2xxx_clkdm_clk_enable, clkdm_clk_disable: omap2xxx_clkdm_clk_disable,
};

pub unsafe fn omap2xxx_cm_fclks_active() -> i32 {
    let f1 = omap2_cm_read_mod_reg(CORE_MOD, CM_FCLKEN1);
    let f2 = omap2_cm_read_mod_reg(CORE_MOD, OMAP24XX_CM_FCLKEN2);
    if (f1 | f2) != 0 { 1 } else { 0 }
}

pub unsafe fn omap2xxx_cm_mpu_retention_allowed() -> i32 {
    let mut l = omap2_cm_read_mod_reg(CORE_MOD, CM_FCLKEN1);
    if (l & (OMAP2420_EN_MMC_MASK | OMAP24XX_EN_UART2_MASK | OMAP24XX_EN_UART1_MASK | OMAP24XX_EN_MCSPI2_MASK | OMAP24XX_EN_MCSPI1_MASK | OMAP24XX_EN_DSS1_MASK)) != 0 { return 0; }
    l = omap2_cm_read_mod_reg(CORE_MOD, OMAP24XX_CM_FCLKEN2);
    if (l & OMAP24XX_EN_UART3_MASK) != 0 { return 0; }
    1
}

pub unsafe fn omap2xxx_cm_get_core_clk_src() -> u32 { omap2_cm_read_mod_reg(PLL_MOD, CM_CLKSEL2) & OMAP24XX_CORE_CLK_SRC_MASK }
pub unsafe fn omap2xxx_cm_get_core_pll_config() -> u32 { omap2_cm_read_mod_reg(PLL_MOD, CM_CLKSEL2) }

pub unsafe fn omap2xxx_cm_set_mod_dividers(mpu: u32, dsp: u32, gfx: u32, core: u32, mdm: u32) {
    omap2_cm_write_mod_reg(mpu, MPU_MOD, CM_CLKSEL);
    omap2_cm_write_mod_reg(dsp, OMAP24XX_DSP_MOD, CM_CLKSEL);
    omap2_cm_write_mod_reg(gfx, GFX_MOD, CM_CLKSEL);
    let tmp = omap2_cm_read_mod_reg(CORE_MOD, CM_CLKSEL1) & OMAP24XX_CLKSEL_DSS2_MASK;
    omap2_cm_write_mod_reg(core | tmp, CORE_MOD, CM_CLKSEL1);
    if mdm != 0 { omap2_cm_write_mod_reg(mdm, OMAP2430_MDM_MOD, CM_CLKSEL); }
}

static omap2xxx_cm_ll_data: cm_ll_data = cm_ll_data { split_idlest_reg: omap2xxx_cm_split_idlest_reg, wait_module_ready: omap2xxx_cm_wait_module_ready };

pub unsafe fn omap2xxx_cm_init(_data: *const omap_prcm_init_data) -> i32 { cm_register(&omap2xxx_cm_ll_data) }
unsafe fn omap2xxx_cm_exit() { cm_unregister(&omap2xxx_cm_ll_data); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
