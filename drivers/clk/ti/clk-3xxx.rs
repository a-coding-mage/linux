// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP3 Clock init
 *
 * Copyright (C) 2013 Texas Instruments, Inc
 *     Tero Kristo (t-kristo@ti.com)
 */

// External kernel and clock definitions are supplied by the surrounding repository.

const OMAP3430ES2_ST_DSS_IDLE_SHIFT: u8 = 1;
const OMAP3430ES2_ST_HSOTGUSB_IDLE_SHIFT: u8 = 5;
const OMAP3430ES2_ST_SSI_IDLE_SHIFT: u8 = 8;
const OMAP34XX_CM_IDLEST_VAL: u8 = 1;
const AM35XX_IPSS_ICK_MASK: u8 = 0xF;
const AM35XX_IPSS_ICK_EN_ACK_OFFSET: u8 = 0x4;
const AM35XX_IPSS_ICK_FCK_OFFSET: u8 = 0x8;
const AM35XX_IPSS_CLK_IDLEST_VAL: u8 = 0;
const AM35XX_ST_IPSS_SHIFT: u8 = 5;

unsafe fn omap3430es2_clk_ssi_find_idlest(
    clk: *mut clk_hw_omap, idlest_reg: *mut clk_omap_reg,
    idlest_bit: *mut u8, idlest_val: *mut u8,
) {
    core::ptr::copy_nonoverlapping(&(*clk).enable_reg, idlest_reg, 1);
    (*idlest_reg).offset &= !0xf0;
    (*idlest_reg).offset |= 0x20;
    *idlest_bit = OMAP3430ES2_ST_SSI_IDLE_SHIFT;
    *idlest_val = OMAP34XX_CM_IDLEST_VAL;
}

pub static clkhwops_omap3430es2_iclk_ssi_wait: clk_hw_omap_ops = clk_hw_omap_ops {
    allow_idle: Some(omap2_clkt_iclk_allow_idle), deny_idle: Some(omap2_clkt_iclk_deny_idle),
    find_idlest: Some(omap3430es2_clk_ssi_find_idlest), find_companion: Some(omap2_clk_dflt_find_companion),
};

unsafe fn omap3430es2_clk_dss_usbhost_find_idlest(
    clk: *mut clk_hw_omap, idlest_reg: *mut clk_omap_reg,
    idlest_bit: *mut u8, idlest_val: *mut u8,
) {
    core::ptr::copy_nonoverlapping(&(*clk).enable_reg, idlest_reg, 1);
    (*idlest_reg).offset &= !0xf0;
    (*idlest_reg).offset |= 0x20;
    // USBHOST_IDLE has same shift
    *idlest_bit = OMAP3430ES2_ST_DSS_IDLE_SHIFT;
    *idlest_val = OMAP34XX_CM_IDLEST_VAL;
}

pub static clkhwops_omap3430es2_dss_usbhost_wait: clk_hw_omap_ops = clk_hw_omap_ops {
    allow_idle: None, deny_idle: None, find_idlest: Some(omap3430es2_clk_dss_usbhost_find_idlest),
    find_companion: Some(omap2_clk_dflt_find_companion),
};
pub static clkhwops_omap3430es2_iclk_dss_usbhost_wait: clk_hw_omap_ops = clk_hw_omap_ops {
    allow_idle: Some(omap2_clkt_iclk_allow_idle), deny_idle: Some(omap2_clkt_iclk_deny_idle),
    find_idlest: Some(omap3430es2_clk_dss_usbhost_find_idlest), find_companion: Some(omap2_clk_dflt_find_companion),
};

unsafe fn omap3430es2_clk_hsotgusb_find_idlest(
    clk: *mut clk_hw_omap, idlest_reg: *mut clk_omap_reg,
    idlest_bit: *mut u8, idlest_val: *mut u8,
) {
    core::ptr::copy_nonoverlapping(&(*clk).enable_reg, idlest_reg, 1);
    (*idlest_reg).offset &= !0xf0;
    (*idlest_reg).offset |= 0x20;
    *idlest_bit = OMAP3430ES2_ST_HSOTGUSB_IDLE_SHIFT;
    *idlest_val = OMAP34XX_CM_IDLEST_VAL;
}

pub static clkhwops_omap3430es2_iclk_hsotgusb_wait: clk_hw_omap_ops = clk_hw_omap_ops {
    allow_idle: Some(omap2_clkt_iclk_allow_idle), deny_idle: Some(omap2_clkt_iclk_deny_idle),
    find_idlest: Some(omap3430es2_clk_hsotgusb_find_idlest), find_companion: Some(omap2_clk_dflt_find_companion),
};

unsafe fn am35xx_clk_find_idlest(clk: *mut clk_hw_omap, idlest_reg: *mut clk_omap_reg, idlest_bit: *mut u8, idlest_val: *mut u8) {
    core::ptr::copy_nonoverlapping(&(*clk).enable_reg, idlest_reg, 1);
    *idlest_bit = (*clk).enable_bit + AM35XX_IPSS_ICK_EN_ACK_OFFSET;
    *idlest_val = AM35XX_IPSS_CLK_IDLEST_VAL;
}

unsafe fn am35xx_clk_find_companion(clk: *mut clk_hw_omap, other_reg: *mut clk_omap_reg, other_bit: *mut u8) {
    core::ptr::copy_nonoverlapping(&(*clk).enable_reg, other_reg, 1);
    if (*clk).enable_bit & AM35XX_IPSS_ICK_MASK != 0 { *other_bit = (*clk).enable_bit + AM35XX_IPSS_ICK_FCK_OFFSET; }
    else { *other_bit = (*clk).enable_bit - AM35XX_IPSS_ICK_FCK_OFFSET; }
}

pub static clkhwops_am35xx_ipss_module_wait: clk_hw_omap_ops = clk_hw_omap_ops {
    allow_idle: None, deny_idle: None, find_idlest: Some(am35xx_clk_find_idlest), find_companion: Some(am35xx_clk_find_companion),
};

unsafe fn am35xx_clk_ipss_find_idlest(clk: *mut clk_hw_omap, idlest_reg: *mut clk_omap_reg, idlest_bit: *mut u8, idlest_val: *mut u8) {
    core::ptr::copy_nonoverlapping(&(*clk).enable_reg, idlest_reg, 1);
    (*idlest_reg).offset &= !0xf0;
    (*idlest_reg).offset |= 0x20;
    *idlest_bit = AM35XX_ST_IPSS_SHIFT;
    *idlest_val = OMAP34XX_CM_IDLEST_VAL;
}

pub static clkhwops_am35xx_ipss_wait: clk_hw_omap_ops = clk_hw_omap_ops {
    allow_idle: Some(omap2_clkt_iclk_allow_idle), deny_idle: Some(omap2_clkt_iclk_deny_idle),
    find_idlest: Some(am35xx_clk_ipss_find_idlest), find_companion: Some(omap2_clk_dflt_find_companion),
};

static mut omap3xxx_clks: [ti_dt_clk; 3] = [DT_CLK!(core::ptr::null(), "timer_32k_ck", "omap_32k_fck"), DT_CLK!(core::ptr::null(), "timer_sys_ck", "sys_ck"), ti_dt_clk { node_name: core::ptr::null(), clk_name: core::ptr::null() }];
static mut omap36xx_omap3430es2plus_clks: [ti_dt_clk; 5] = [DT_CLK!(core::ptr::null(), "ssi_ssr_fck", "ssi_ssr_fck_3430es2"), DT_CLK!(core::ptr::null(), "ssi_sst_fck", "ssi_sst_fck_3430es2"), DT_CLK!(core::ptr::null(), "hsotgusb_ick", "hsotgusb_ick_3430es2"), DT_CLK!(core::ptr::null(), "ssi_ick", "ssi_ick_3430es2"), ti_dt_clk { node_name: core::ptr::null(), clk_name: core::ptr::null() }];
static mut omap3430es1_clks: [ti_dt_clk; 7] = [DT_CLK!(core::ptr::null(), "ssi_ssr_fck", "ssi_ssr_fck_3430es1"), DT_CLK!(core::ptr::null(), "ssi_sst_fck", "ssi_sst_fck_3430es1"), DT_CLK!(core::ptr::null(), "hsotgusb_ick", "hsotgusb_ick_3430es1"), DT_CLK!(core::ptr::null(), "ssi_ick", "ssi_ick_3430es1"), DT_CLK!(core::ptr::null(), "dss1_alwon_fck", "dss1_alwon_fck_3430es1"), DT_CLK!(core::ptr::null(), "dss_ick", "dss_ick_3430es1"), ti_dt_clk { node_name: core::ptr::null(), clk_name: core::ptr::null() }];
static mut omap36xx_am35xx_omap3430es2plus_clks: [ti_dt_clk; 3] = [DT_CLK!(core::ptr::null(), "dss1_alwon_fck", "dss1_alwon_fck_3430es2"), DT_CLK!(core::ptr::null(), "dss_ick", "dss_ick_3430es2"), ti_dt_clk { node_name: core::ptr::null(), clk_name: core::ptr::null() }];
static mut am35xx_clks: [ti_dt_clk; 5] = [DT_CLK!(core::ptr::null(), "hsotgusb_ick", "hsotgusb_ick_am35xx"), DT_CLK!(core::ptr::null(), "hsotgusb_fck", "hsotgusb_fck_am35xx"), DT_CLK!(core::ptr::null(), "uart4_ick", "uart4_ick_am35xx"), DT_CLK!(core::ptr::null(), "uart4_fck", "uart4_fck_am35xx"), ti_dt_clk { node_name: core::ptr::null(), clk_name: core::ptr::null() }];
static enable_init_clks: [&[u8]; 3] = [b"sdrc_ick", b"gpmc_fck", b"omapctrl_ick"];

enum Omap3Soc { OMAP3_SOC_AM35XX, OMAP3_SOC_OMAP3430_ES1, OMAP3_SOC_OMAP3430_ES2_PLUS, OMAP3_SOC_OMAP3630 }

pub unsafe fn omap3_clk_lock_dpll5() {
    let dpll5_clk = clk_get(core::ptr::null_mut(), b"dpll5_ck\0".as_ptr() as *const i8);
    clk_set_rate(dpll5_clk, OMAP3_DPLL5_FREQ_FOR_USBHOST * 8); clk_prepare_enable(dpll5_clk);
    let dpll5_m2_clk = clk_get(core::ptr::null_mut(), b"dpll5_m2_ck\0".as_ptr() as *const i8);
    clk_prepare_enable(dpll5_m2_clk); clk_set_rate(dpll5_m2_clk, OMAP3_DPLL5_FREQ_FOR_USBHOST);
    clk_disable_unprepare(dpll5_m2_clk); clk_disable_unprepare(dpll5_clk);
}

unsafe fn omap3xxx_dt_clk_init(soc_type: Omap3Soc) -> i32 {
    if matches!(soc_type, Omap3Soc::OMAP3_SOC_AM35XX | Omap3Soc::OMAP3_SOC_OMAP3630 | Omap3Soc::OMAP3_SOC_OMAP3430_ES1 | Omap3Soc::OMAP3_SOC_OMAP3430_ES2_PLUS) { ti_dt_clocks_register(omap3xxx_clks.as_mut_ptr()); }
    if matches!(soc_type, Omap3Soc::OMAP3_SOC_AM35XX) { ti_dt_clocks_register(am35xx_clks.as_mut_ptr()); }
    if matches!(soc_type, Omap3Soc::OMAP3_SOC_OMAP3630 | Omap3Soc::OMAP3_SOC_AM35XX | Omap3Soc::OMAP3_SOC_OMAP3430_ES2_PLUS) { ti_dt_clocks_register(omap36xx_am35xx_omap3430es2plus_clks.as_mut_ptr()); }
    if matches!(soc_type, Omap3Soc::OMAP3_SOC_OMAP3430_ES1) { ti_dt_clocks_register(omap3430es1_clks.as_mut_ptr()); }
    if matches!(soc_type, Omap3Soc::OMAP3_SOC_OMAP3430_ES2_PLUS | Omap3Soc::OMAP3_SOC_OMAP3630) { ti_dt_clocks_register(omap36xx_omap3430es2plus_clks.as_mut_ptr()); }
    omap2_clk_disable_autoidle_all(); ti_clk_add_aliases();
    omap2_clk_enable_init_clocks(enable_init_clks.as_ptr(), enable_init_clks.len());
    if !matches!(soc_type, Omap3Soc::OMAP3_SOC_OMAP3430_ES1) { omap3_clk_lock_dpll5(); }
    0
}

pub unsafe fn omap3430_dt_clk_init() -> i32 { omap3xxx_dt_clk_init(Omap3Soc::OMAP3_SOC_OMAP3430_ES2_PLUS) }
pub unsafe fn omap3630_dt_clk_init() -> i32 { omap3xxx_dt_clk_init(Omap3Soc::OMAP3_SOC_OMAP3630) }
pub unsafe fn am35xx_dt_clk_init() -> i32 { omap3xxx_dt_clk_init(Omap3Soc::OMAP3_SOC_AM35XX) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
