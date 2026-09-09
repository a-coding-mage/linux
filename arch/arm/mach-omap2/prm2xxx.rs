// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP2xxx PRM module functions
 *
 * Copyright (C) 2010-2012 Texas Instruments, Inc.
 * Copyright (C) 2010 Nokia Corporation
 * Benoît Cousson
 * Paul Walmsley
 * Rajendra Nayak <rnayak@ti.com>
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * OMAP24xx PM_PWSTCTRL_*.POWERSTATE and PM_PWSTST_*.LASTSTATEENTERED bits -
 * these are reversed from the bits used on OMAP3+
 */
const OMAP24XX_PWRDM_POWER_ON: u8 = 0x0;
const OMAP24XX_PWRDM_POWER_RET: u8 = 0x1;
const OMAP24XX_PWRDM_POWER_OFF: u8 = 0x3;

/*
 * omap2xxx_prm_reset_src_map - map from bits in the PRM_RSTST_WKUP
 * hardware register (which are specific to the OMAP2xxx SoCs) to
 * reset source ID bit shifts (which is an OMAP SoC-independent
 * enumeration)
 */
static mut OMAP2XXX_PRM_RESET_SRC_MAP: [prm_reset_src_map; 7] = [
    prm_reset_src_map { reg_shift: OMAP_GLOBALCOLD_RST_SHIFT, std_shift: OMAP_GLOBAL_COLD_RST_SRC_ID_SHIFT },
    prm_reset_src_map { reg_shift: OMAP_GLOBALWARM_RST_SHIFT, std_shift: OMAP_GLOBAL_WARM_RST_SRC_ID_SHIFT },
    prm_reset_src_map { reg_shift: OMAP24XX_SECU_VIOL_RST_SHIFT, std_shift: OMAP_SECU_VIOL_RST_SRC_ID_SHIFT },
    prm_reset_src_map { reg_shift: OMAP24XX_MPU_WD_RST_SHIFT, std_shift: OMAP_MPU_WD_RST_SRC_ID_SHIFT },
    prm_reset_src_map { reg_shift: OMAP24XX_SECU_WD_RST_SHIFT, std_shift: OMAP_SECU_WD_RST_SRC_ID_SHIFT },
    prm_reset_src_map { reg_shift: OMAP24XX_EXTWMPU_RST_SHIFT, std_shift: OMAP_EXTWARM_RST_SRC_ID_SHIFT },
    prm_reset_src_map { reg_shift: -1, std_shift: -1 },
];

/** Return the last SoC reset source. */
unsafe fn omap2xxx_prm_read_reset_sources() -> u32 {
    let mut p = OMAP2XXX_PRM_RESET_SRC_MAP.as_ptr();
    let mut r: u32 = 0;
    let v = omap2_prm_read_mod_reg(WKUP_MOD, OMAP2_RM_RSTST);
    while (*p).reg_shift >= 0 && (*p).std_shift >= 0 {
        if v & (1u32 << (*p).reg_shift) != 0 {
            r |= 1u32 << (*p).std_shift;
        }
        p = p.add(1);
    }
    r
}

/** Convert OMAP2xxx power state to the common power state. */
fn omap2xxx_pwrst_to_common_pwrst(omap2xxx_pwrst: u8) -> i32 {
    match omap2xxx_pwrst {
        OMAP24XX_PWRDM_POWER_OFF => PWRDM_POWER_OFF as i32,
        OMAP24XX_PWRDM_POWER_RET => PWRDM_POWER_RET as i32,
        OMAP24XX_PWRDM_POWER_ON => PWRDM_POWER_ON as i32,
        _ => -EINVAL,
    }
}

/** Use DPLL reset to reboot the OMAP SoC. */
unsafe fn omap2xxx_prm_dpll_reset() {
    omap2_prm_set_mod_reg_bits(OMAP_RST_DPLL3_MASK, WKUP_MOD, OMAP2_RM_RSTCTRL);
    // OCP barrier
    omap2_prm_read_mod_reg(WKUP_MOD, OMAP2_RM_RSTCTRL);
}

/** Clear wakeup status bits for a module. */
unsafe fn omap2xxx_prm_clear_mod_irqs(module: i16, regs: u8, wkst_mask: u32) -> i32 {
    let mut wkst = omap2_prm_read_mod_reg(module, regs);
    wkst &= wkst_mask;
    omap2_prm_write_mod_reg(wkst, module, regs);
    0
}

pub unsafe fn omap2xxx_clkdm_sleep(clkdm: *mut clockdomain) -> i32 {
    omap2_prm_set_mod_reg_bits(OMAP24XX_FORCESTATE_MASK, (*(*clkdm).pwrdm).ptr.as_ref().unwrap().prcm_offs, OMAP2_PM_PWSTCTRL);
    0
}

pub unsafe fn omap2xxx_clkdm_wakeup(clkdm: *mut clockdomain) -> i32 {
    omap2_prm_clear_mod_reg_bits(OMAP24XX_FORCESTATE_MASK, (*(*clkdm).pwrdm).ptr.as_ref().unwrap().prcm_offs, OMAP2_PM_PWSTCTRL);
    0
}

unsafe fn omap2xxx_pwrdm_set_next_pwrst(pwrdm: *mut powerdomain, pwrst: u8) -> i32 {
    let omap24xx_pwrst = match pwrst {
        PWRDM_POWER_OFF => OMAP24XX_PWRDM_POWER_OFF,
        PWRDM_POWER_RET => OMAP24XX_PWRDM_POWER_RET,
        PWRDM_POWER_ON => OMAP24XX_PWRDM_POWER_ON,
        _ => return -EINVAL,
    };
    omap2_prm_rmw_mod_reg_bits(OMAP_POWERSTATE_MASK, (omap24xx_pwrst as u32) << OMAP_POWERSTATE_SHIFT, (*pwrdm).prcm_offs, OMAP2_PM_PWSTCTRL);
    0
}

unsafe fn omap2xxx_pwrdm_read_next_pwrst(pwrdm: *mut powerdomain) -> i32 {
    let p = omap2_prm_read_mod_bits_shift((*pwrdm).prcm_offs, OMAP2_PM_PWSTCTRL, OMAP_POWERSTATE_MASK);
    omap2xxx_pwrst_to_common_pwrst(p as u8)
}

unsafe fn omap2xxx_pwrdm_read_pwrst(pwrdm: *mut powerdomain) -> i32 {
    let p = omap2_prm_read_mod_bits_shift((*pwrdm).prcm_offs, OMAP2_PM_PWSTST, OMAP_POWERSTATEST_MASK);
    omap2xxx_pwrst_to_common_pwrst(p as u8)
}

static mut omap2_pwrdm_operations: pwrdm_ops = pwrdm_ops {
    pwrdm_set_next_pwrst: Some(omap2xxx_pwrdm_set_next_pwrst),
    pwrdm_read_next_pwrst: Some(omap2xxx_pwrdm_read_next_pwrst),
    pwrdm_read_pwrst: Some(omap2xxx_pwrdm_read_pwrst),
    pwrdm_set_logic_retst: Some(omap2_pwrdm_set_logic_retst),
    pwrdm_set_mem_onst: Some(omap2_pwrdm_set_mem_onst),
    pwrdm_set_mem_retst: Some(omap2_pwrdm_set_mem_retst),
    pwrdm_read_mem_pwrst: Some(omap2_pwrdm_read_mem_pwrst),
    pwrdm_read_mem_retst: Some(omap2_pwrdm_read_mem_retst),
    pwrdm_wait_transition: Some(omap2_pwrdm_wait_transition),
};

static mut OMAP2XXX_PRM_LL_DATA: prm_ll_data = prm_ll_data {
    read_reset_sources: Some(omap2xxx_prm_read_reset_sources),
    assert_hardreset: Some(omap2_prm_assert_hardreset),
    deassert_hardreset: Some(omap2_prm_deassert_hardreset),
    is_hardreset_asserted: Some(omap2_prm_is_hardreset_asserted),
    reset_system: Some(omap2xxx_prm_dpll_reset),
    clear_mod_irqs: Some(omap2xxx_prm_clear_mod_irqs),
};

pub unsafe fn omap2xxx_prm_init(_data: *const omap_prcm_init_data) -> i32 {
    prm_register(&mut OMAP2XXX_PRM_LL_DATA)
}

unsafe fn omap2xxx_prm_exit() {
    prm_unregister(&mut OMAP2XXX_PRM_LL_DATA);
}

// __exitcall(omap2xxx_prm_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
