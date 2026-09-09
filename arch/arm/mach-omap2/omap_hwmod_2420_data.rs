// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap_hwmod_2420_data.c - hardware modules present on the OMAP2420 chips
 *
 * Copyright (C) 2009-2011 Nokia Corporation
 * Copyright (C) 2012 Texas Instruments, Inc.
 * Paul Walmsley
 *
 * XXX handle crossbar/shared link difference for L3?
 * XXX these should be marked initdata for multi-OMAP kernels
 */

// Dependencies are supplied by the corresponding kernel Rust bindings/modules.

/* OMAP2420 hardware module integration data */

static mut iva1_hwmod_class: omap_hwmod_class = omap_hwmod_class { name: "iva1" };

static mut omap2420_iva_resets: [omap_hwmod_rst_info; 1] = [
    omap_hwmod_rst_info { name: "iva", rst_shift: 8 },
];

static mut omap2420_iva_hwmod: omap_hwmod = omap_hwmod {
    name: "iva", class: &raw mut iva1_hwmod_class, clkdm_name: "iva1_clkdm",
    rst_lines: &raw mut omap2420_iva_resets,
    rst_lines_cnt: omap2420_iva_resets.len(), main_clk: "iva1_ifck",
};

static mut dsp_hwmod_class: omap_hwmod_class = omap_hwmod_class { name: "dsp" };
static mut omap2420_dsp_resets: [omap_hwmod_rst_info; 2] = [
    omap_hwmod_rst_info { name: "logic", rst_shift: 0 },
    omap_hwmod_rst_info { name: "mmu", rst_shift: 1 },
];
static mut omap2420_dsp_hwmod: omap_hwmod = omap_hwmod {
    name: "dsp", class: &raw mut dsp_hwmod_class, clkdm_name: "dsp_clkdm",
    rst_lines: &raw mut omap2420_dsp_resets, rst_lines_cnt: omap2420_dsp_resets.len(),
    main_clk: "dsp_fck",
};

static mut i2c_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x00, sysc_offs: 0x20, syss_offs: 0x10,
    sysc_flags: SYSC_HAS_SOFTRESET | SYSS_HAS_RESET_STATUS,
    sysc_fields: &raw mut omap_hwmod_sysc_type1,
};
static mut i2c_class: omap_hwmod_class = omap_hwmod_class {
    name: "i2c", sysc: &raw mut i2c_sysc, reset: &raw mut omap_i2c_reset,
};

static mut omap2420_i2c1_hwmod: omap_hwmod = omap_hwmod {
    name: "i2c1", main_clk: "i2c1_fck",
    prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 {
        module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP2420_ST_I2C1_SHIFT,
    } },
    class: &raw mut i2c_class, flags: HWMOD_16BIT_REG | HWMOD_BLOCK_WFI,
};
static mut omap2420_i2c2_hwmod: omap_hwmod = omap_hwmod {
    name: "i2c2", main_clk: "i2c2_fck",
    prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 {
        module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP2420_ST_I2C2_SHIFT,
    } },
    class: &raw mut i2c_class, flags: HWMOD_16BIT_REG,
};

static mut omap2420_mailbox_hwmod: omap_hwmod = omap_hwmod {
    name: "mailbox", class: &raw mut omap2xxx_mailbox_hwmod_class, main_clk: "mailboxes_ick",
    prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 {
        module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_MAILBOXES_SHIFT,
    } },
};

static mut omap2420_mcbsp_hwmod_class: omap_hwmod_class = omap_hwmod_class { name: "mcbsp" };
static mut mcbsp_opt_clks: [omap_hwmod_opt_clk; 2] = [
    omap_hwmod_opt_clk { role: "pad_fck", clk: "mcbsp_clks" },
    omap_hwmod_opt_clk { role: "prcm_fck", clk: "func_96m_ck" },
];

static mut omap2420_mcbsp1_hwmod: omap_hwmod = omap_hwmod {
    name: "mcbsp1", class: &raw mut omap2420_mcbsp_hwmod_class, main_clk: "mcbsp1_fck",
    prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 {
        module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_MCBSP1_SHIFT,
    } }, opt_clks: &raw mut mcbsp_opt_clks, opt_clks_cnt: mcbsp_opt_clks.len(),
};
static mut omap2420_mcbsp2_hwmod: omap_hwmod = omap_hwmod {
    name: "mcbsp2", class: &raw mut omap2420_mcbsp_hwmod_class, main_clk: "mcbsp2_fck",
    prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 {
        module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_MCBSP2_SHIFT,
    } }, opt_clks: &raw mut mcbsp_opt_clks, opt_clks_cnt: mcbsp_opt_clks.len(),
};

static mut omap2420_msdi_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x3c, sysc_offs: 0x64, syss_offs: 0x68,
    sysc_flags: SYSC_HAS_SOFTRESET | SYSS_HAS_RESET_STATUS,
    sysc_fields: &raw mut omap_hwmod_sysc_type1,
};
static mut omap2420_msdi_hwmod_class: omap_hwmod_class = omap_hwmod_class {
    name: "msdi", sysc: &raw mut omap2420_msdi_sysc, reset: &raw mut omap_msdi_reset,
};
static mut omap2420_msdi1_hwmod: omap_hwmod = omap_hwmod {
    name: "msdi1", class: &raw mut omap2420_msdi_hwmod_class, main_clk: "mmc_fck",
    prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 {
        module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP2420_ST_MMC_SHIFT,
    } }, flags: HWMOD_16BIT_REG,
};
static mut omap2420_hdq1w_hwmod: omap_hwmod = omap_hwmod {
    name: "hdq1w", main_clk: "hdq_fck",
    prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 {
        module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_HDQ_SHIFT,
    } }, class: &raw mut omap2_hdq1w_class,
};

macro_rules! ocp_if { ($n:ident, $m:expr, $s:expr, $c:expr, $u:expr) => {
    static mut $n: omap_hwmod_ocp_if = omap_hwmod_ocp_if { master: $m, slave: $s, clk: $c, user: $u };
} }
ocp_if!(omap2420_l4_core__i2c1, &raw mut omap2xxx_l4_core_hwmod, &raw mut omap2420_i2c1_hwmod, "i2c1_ick", OCP_USER_MPU | OCP_USER_SDMA);
ocp_if!(omap2420_l4_core__i2c2, &raw mut omap2xxx_l4_core_hwmod, &raw mut omap2420_i2c2_hwmod, "i2c2_ick", OCP_USER_MPU | OCP_USER_SDMA);
ocp_if!(omap2420_l3__iva, &raw mut omap2xxx_l3_main_hwmod, &raw mut omap2420_iva_hwmod, "core_l3_ck", OCP_USER_MPU | OCP_USER_SDMA);
ocp_if!(omap2420_l3__dsp, &raw mut omap2xxx_l3_main_hwmod, &raw mut omap2420_dsp_hwmod, "dsp_ick", OCP_USER_MPU | OCP_USER_SDMA);
ocp_if!(omap2420_l4_wkup__wd_timer2, &raw mut omap2xxx_l4_wkup_hwmod, &raw mut omap2xxx_wd_timer2_hwmod, "mpu_wdt_ick", OCP_USER_MPU | OCP_USER_SDMA);
ocp_if!(omap2420_l4_wkup__gpio1, &raw mut omap2xxx_l4_wkup_hwmod, &raw mut omap2xxx_gpio1_hwmod, "gpios_ick", OCP_USER_MPU | OCP_USER_SDMA);
ocp_if!(omap2420_l4_wkup__gpio2, &raw mut omap2xxx_l4_wkup_hwmod, &raw mut omap2xxx_gpio2_hwmod, "gpios_ick", OCP_USER_MPU | OCP_USER_SDMA);
ocp_if!(omap2420_l4_wkup__gpio3, &raw mut omap2xxx_l4_wkup_hwmod, &raw mut omap2xxx_gpio3_hwmod, "gpios_ick", OCP_USER_MPU | OCP_USER_SDMA);
ocp_if!(omap2420_l4_wkup__gpio4, &raw mut omap2xxx_l4_wkup_hwmod, &raw mut omap2xxx_gpio4_hwmod, "gpios_ick", OCP_USER_MPU | OCP_USER_SDMA);
ocp_if!(omap2420_l4_core__mailbox, &raw mut omap2xxx_l4_core_hwmod, &raw mut omap2420_mailbox_hwmod, "", OCP_USER_MPU | OCP_USER_SDMA);
ocp_if!(omap2420_l4_core__mcbsp1, &raw mut omap2xxx_l4_core_hwmod, &raw mut omap2420_mcbsp1_hwmod, "mcbsp1_ick", OCP_USER_MPU | OCP_USER_SDMA);
ocp_if!(omap2420_l4_core__mcbsp2, &raw mut omap2xxx_l4_core_hwmod, &raw mut omap2420_mcbsp2_hwmod, "mcbsp2_ick", OCP_USER_MPU | OCP_USER_SDMA);
ocp_if!(omap2420_l4_core__msdi1, &raw mut omap2xxx_l4_core_hwmod, &raw mut omap2420_msdi1_hwmod, "mmc_ick", OCP_USER_MPU | OCP_USER_SDMA);

static mut omap2420_l4_core__hdq1w: omap_hwmod_ocp_if = omap_hwmod_ocp_if {
    master: &raw mut omap2xxx_l4_core_hwmod, slave: &raw mut omap2420_hdq1w_hwmod,
    clk: "hdq_ick", user: OCP_USER_MPU | OCP_USER_SDMA,
    flags: OMAP_FIREWALL_L4 | OCPIF_SWSUP_IDLE,
};
ocp_if!(omap2420_l3__gpmc, &raw mut omap2xxx_l3_main_hwmod, &raw mut omap2xxx_gpmc_hwmod, "core_l3_ck", OCP_USER_MPU | OCP_USER_SDMA);

static mut omap2420_hwmod_ocp_ifs: [*mut omap_hwmod_ocp_if; 43] = [
    &raw mut omap2xxx_l3_main__l4_core, &raw mut omap2xxx_mpu__l3_main, &raw mut omap2xxx_dss__l3,
    &raw mut omap2xxx_l4_core__mcspi1, &raw mut omap2xxx_l4_core__mcspi2, &raw mut omap2xxx_l4_core__l4_wkup,
    &raw mut omap2_l4_core__uart1, &raw mut omap2_l4_core__uart2, &raw mut omap2_l4_core__uart3,
    &raw mut omap2420_l4_core__i2c1, &raw mut omap2420_l4_core__i2c2, &raw mut omap2420_l3__iva,
    &raw mut omap2420_l3__dsp, &raw mut omap2xxx_l4_core__timer3, &raw mut omap2xxx_l4_core__timer4,
    &raw mut omap2xxx_l4_core__timer5, &raw mut omap2xxx_l4_core__timer6, &raw mut omap2xxx_l4_core__timer7,
    &raw mut omap2xxx_l4_core__timer8, &raw mut omap2xxx_l4_core__timer9, &raw mut omap2xxx_l4_core__timer10,
    &raw mut omap2xxx_l4_core__timer11, &raw mut omap2xxx_l4_core__timer12, &raw mut omap2420_l4_wkup__wd_timer2,
    &raw mut omap2xxx_l4_core__dss, &raw mut omap2xxx_l4_core__dss_dispc, &raw mut omap2xxx_l4_core__dss_rfbi,
    &raw mut omap2xxx_l4_core__dss_venc, &raw mut omap2420_l4_wkup__gpio1, &raw mut omap2420_l4_wkup__gpio2,
    &raw mut omap2420_l4_wkup__gpio3, &raw mut omap2420_l4_wkup__gpio4, &raw mut omap2420_l4_core__mailbox,
    &raw mut omap2420_l4_core__mcbsp1, &raw mut omap2420_l4_core__mcbsp2, &raw mut omap2420_l4_core__msdi1,
    &raw mut omap2xxx_l4_core__rng, &raw mut omap2xxx_l4_core__sham, &raw mut omap2xxx_l4_core__aes,
    &raw mut omap2420_l4_core__hdq1w, &raw mut omap2420_l3__gpmc, core::ptr::null_mut(),
];

pub unsafe fn omap2420_hwmod_init() -> i32 {
    omap_hwmod_init();
    omap_hwmod_register_links(&raw mut omap2420_hwmod_ocp_ifs)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
