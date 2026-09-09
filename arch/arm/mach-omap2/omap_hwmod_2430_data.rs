// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap_hwmod_2430_data.c - hardware modules present on the OMAP2430 chips
 *
 * Copyright (C) 2009-2011 Nokia Corporation
 * Copyright (C) 2012 Texas Instruments, Inc.
 * Paul Walmsley
 *
 * XXX handle crossbar/shared link difference for L3?
 * XXX these should be marked initdata for multi-OMAP kernels
 */

// C includes and build-time declarations are supplied by the surrounding kernel translation.

static mut omap2430_iva_resets: [omap_hwmod_rst_info; 2] = [
    omap_hwmod_rst_info { name: "logic", rst_shift: 0 },
    omap_hwmod_rst_info { name: "mmu", rst_shift: 1 },
];

static mut omap2430_iva_hwmod: omap_hwmod = omap_hwmod {
    name: "iva", class: &iva_hwmod_class, clkdm_name: "dsp_clkdm",
    rst_lines: omap2430_iva_resets.as_ptr(), rst_lines_cnt: 2, main_clk: "dsp_fck",
};

static mut i2c_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x00, sysc_offs: 0x20, syss_offs: 0x10,
    sysc_flags: SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE | SYSS_HAS_RESET_STATUS,
    sysc_fields: &omap_hwmod_sysc_type1,
};
static mut i2c_class: omap_hwmod_class = omap_hwmod_class { name: "i2c", sysc: &i2c_sysc, reset: &omap_i2c_reset };

macro_rules! hwmod {
    ($n:expr, $c:expr, $clk:expr, $off:expr, $rid:expr, $bit:expr, $flags:expr, $opt:expr, $cnt:expr, $attr:expr) => {
        omap_hwmod { name: $n, class: $c, main_clk: $clk, flags: $flags,
            prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: $off, idlest_reg_id: $rid, idlest_idle_bit: $bit } },
            opt_clks: $opt, opt_clks_cnt: $cnt, dev_attr: $attr }
    };
}

static mut omap2430_i2c1_hwmod: omap_hwmod = hwmod!("i2c1", &i2c_class, "i2chs1_fck", CORE_MOD, 1, OMAP2430_ST_I2CHS1_SHIFT, HWMOD_16BIT_REG, core::ptr::null(), 0, core::ptr::null());
static mut omap2430_i2c2_hwmod: omap_hwmod = hwmod!("i2c2", &i2c_class, "i2chs2_fck", CORE_MOD, 1, OMAP2430_ST_I2CHS2_SHIFT, HWMOD_16BIT_REG, core::ptr::null(), 0, core::ptr::null());
static mut omap2430_gpio5_hwmod: omap_hwmod = hwmod!("gpio5", &omap2xxx_gpio_hwmod_class, "gpio5_fck", CORE_MOD, 2, OMAP2430_ST_GPIO5_SHIFT, HWMOD_CONTROL_OPT_CLKS_IN_RESET, core::ptr::null(), 0, core::ptr::null());
static mut omap2430_mailbox_hwmod: omap_hwmod = hwmod!("mailbox", &omap2xxx_mailbox_hwmod_class, "mailboxes_ick", CORE_MOD, 1, OMAP24XX_ST_MAILBOXES_SHIFT, 0, core::ptr::null(), 0, core::ptr::null());
static mut omap2430_mcspi3_hwmod: omap_hwmod = hwmod!("mcspi3", &omap2xxx_mcspi_class, "mcspi3_fck", CORE_MOD, 2, OMAP2430_ST_MCSPI3_SHIFT, 0, core::ptr::null(), 0, core::ptr::null());

static mut omap2430_usbhsotg_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x0400, sysc_offs: 0x0404, syss_offs: 0x0408,
    sysc_flags: SYSC_HAS_SIDLEMODE | SYSC_HAS_MIDLEMODE | SYSC_HAS_ENAWAKEUP | SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE,
    idlemodes: SIDLE_FORCE | SIDLE_NO | SIDLE_SMART | MSTANDBY_FORCE | MSTANDBY_NO | MSTANDBY_SMART,
    sysc_fields: &omap_hwmod_sysc_type1,
};
static mut usbotg_class: omap_hwmod_class = omap_hwmod_class { name: "usbotg", sysc: &omap2430_usbhsotg_sysc };
static mut omap2430_usbhsotg_hwmod: omap_hwmod = hwmod!("usb_otg_hs", &usbotg_class, "usbhs_ick", CORE_MOD, 1, OMAP2430_ST_USBHS_SHIFT, HWMOD_NO_OCP_AUTOIDLE | HWMOD_SWSUP_SIDLE | HWMOD_SWSUP_MSTANDBY, core::ptr::null(), 0, core::ptr::null());

static mut omap2430_mcbsp_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig { rev_offs: 0x007C, sysc_offs: 0x008C, sysc_flags: SYSC_HAS_SOFTRESET, sysc_fields: &omap_hwmod_sysc_type1 };
static mut omap2430_mcbsp_hwmod_class: omap_hwmod_class = omap_hwmod_class { name: "mcbsp", sysc: &omap2430_mcbsp_sysc };
static mut mcbsp_opt_clks: [omap_hwmod_opt_clk; 2] = [omap_hwmod_opt_clk { role: "pad_fck", clk: "mcbsp_clks" }, omap_hwmod_opt_clk { role: "prcm_fck", clk: "func_96m_ck" }];
static mut omap2430_mcbsp1_hwmod: omap_hwmod = hwmod!("mcbsp1", &omap2430_mcbsp_hwmod_class, "mcbsp1_fck", CORE_MOD, 1, OMAP24XX_ST_MCBSP1_SHIFT, 0, mcbsp_opt_clks.as_ptr(), 2, core::ptr::null());
static mut omap2430_mcbsp2_hwmod: omap_hwmod = hwmod!("mcbsp2", &omap2430_mcbsp_hwmod_class, "mcbsp2_fck", CORE_MOD, 1, OMAP24XX_ST_MCBSP2_SHIFT, 0, mcbsp_opt_clks.as_ptr(), 2, core::ptr::null());
static mut omap2430_mcbsp3_hwmod: omap_hwmod = hwmod!("mcbsp3", &omap2430_mcbsp_hwmod_class, "mcbsp3_fck", CORE_MOD, 2, OMAP2430_ST_MCBSP3_SHIFT, 0, mcbsp_opt_clks.as_ptr(), 2, core::ptr::null());
static mut omap2430_mcbsp4_hwmod: omap_hwmod = hwmod!("mcbsp4", &omap2430_mcbsp_hwmod_class, "mcbsp4_fck", CORE_MOD, 2, OMAP2430_ST_MCBSP4_SHIFT, 0, mcbsp_opt_clks.as_ptr(), 2, core::ptr::null());
static mut omap2430_mcbsp5_hwmod: omap_hwmod = hwmod!("mcbsp5", &omap2430_mcbsp_hwmod_class, "mcbsp5_fck", CORE_MOD, 2, OMAP2430_ST_MCBSP5_SHIFT, 0, mcbsp_opt_clks.as_ptr(), 2, core::ptr::null());

static mut omap2430_mmc_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig { rev_offs: 0x1fc, sysc_offs: 0x10, syss_offs: 0x14, sysc_flags: SYSC_HAS_CLOCKACTIVITY | SYSC_HAS_SIDLEMODE | SYSC_HAS_ENAWAKEUP | SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE | SYSS_HAS_RESET_STATUS, idlemodes: SIDLE_FORCE | SIDLE_NO | SIDLE_SMART, sysc_fields: &omap_hwmod_sysc_type1 };
static mut omap2430_mmc_class: omap_hwmod_class = omap_hwmod_class { name: "mmc", sysc: &omap2430_mmc_sysc };
static mut omap2430_mmc1_opt_clks: [omap_hwmod_opt_clk; 1] = [omap_hwmod_opt_clk { role: "dbck", clk: "mmchsdb1_fck" }];
static mut omap2430_mmc2_opt_clks: [omap_hwmod_opt_clk; 1] = [omap_hwmod_opt_clk { role: "dbck", clk: "mmchsdb2_fck" }];
static mut mmc1_dev_attr: omap_hsmmc_dev_attr = omap_hsmmc_dev_attr { flags: OMAP_HSMMC_SUPPORTS_DUAL_VOLT };
static mut omap2430_mmc1_hwmod: omap_hwmod = hwmod!("mmc1", &omap2430_mmc_class, "mmchs1_fck", CORE_MOD, 2, OMAP2430_ST_MMCHS1_SHIFT, HWMOD_CONTROL_OPT_CLKS_IN_RESET, omap2430_mmc1_opt_clks.as_ptr(), 1, &mmc1_dev_attr);
static mut omap2430_mmc2_hwmod: omap_hwmod = hwmod!("mmc2", &omap2430_mmc_class, "mmchs2_fck", CORE_MOD, 2, OMAP2430_ST_MMCHS2_SHIFT, HWMOD_CONTROL_OPT_CLKS_IN_RESET, omap2430_mmc2_opt_clks.as_ptr(), 1, core::ptr::null());
static mut omap2430_hdq1w_hwmod: omap_hwmod = hwmod!("hdq1w", &omap2_hdq1w_class, "hdq_fck", CORE_MOD, 1, OMAP24XX_ST_HDQ_SHIFT, 0, core::ptr::null(), 0, core::ptr::null());

// OCP interface declarations retain the original master/slave/clock/user/flag relationships.
macro_rules! ocp { ($n:ident, $m:expr, $s:expr, $c:expr, $u:expr, $f:expr) => { static mut $n: omap_hwmod_ocp_if = omap_hwmod_ocp_if { master: $m, slave: $s, clk: $c, user: $u, flags: $f }; }; }
ocp!(omap2430_usbhsotg__l3, &omap2430_usbhsotg_hwmod, &omap2xxx_l3_main_hwmod, "core_l3_ck", OCP_USER_MPU, 0);
ocp!(omap2430_l4_core__i2c1, &omap2xxx_l4_core_hwmod, &omap2430_i2c1_hwmod, "i2c1_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_core__i2c2, &omap2xxx_l4_core_hwmod, &omap2430_i2c2_hwmod, "i2c2_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_core__usbhsotg, &omap2xxx_l4_core_hwmod, &omap2430_usbhsotg_hwmod, "usb_l4_ick", OCP_USER_MPU, 0);
ocp!(omap2430_l4_core__mmc1, &omap2xxx_l4_core_hwmod, &omap2430_mmc1_hwmod, "mmchs1_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_core__mmc2, &omap2xxx_l4_core_hwmod, &omap2430_mmc2_hwmod, "mmchs2_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_core__mcspi3, &omap2xxx_l4_core_hwmod, &omap2430_mcspi3_hwmod, "mcspi3_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l3__iva, &omap2xxx_l3_main_hwmod, &omap2430_iva_hwmod, "core_l3_ck", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_wkup__wd_timer2, &omap2xxx_l4_wkup_hwmod, &omap2xxx_wd_timer2_hwmod, "mpu_wdt_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_wkup__gpio1, &omap2xxx_l4_wkup_hwmod, &omap2xxx_gpio1_hwmod, "gpios_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_wkup__gpio2, &omap2xxx_l4_wkup_hwmod, &omap2xxx_gpio2_hwmod, "gpios_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_wkup__gpio3, &omap2xxx_l4_wkup_hwmod, &omap2xxx_gpio3_hwmod, "gpios_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_wkup__gpio4, &omap2xxx_l4_wkup_hwmod, &omap2xxx_gpio4_hwmod, "gpios_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_core__gpio5, &omap2xxx_l4_core_hwmod, &omap2430_gpio5_hwmod, "gpio5_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_core__mailbox, &omap2xxx_l4_core_hwmod, &omap2430_mailbox_hwmod, core::ptr::null(), OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_core__mcbsp1, &omap2xxx_l4_core_hwmod, &omap2430_mcbsp1_hwmod, "mcbsp1_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_core__mcbsp2, &omap2xxx_l4_core_hwmod, &omap2430_mcbsp2_hwmod, "mcbsp2_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_core__mcbsp3, &omap2xxx_l4_core_hwmod, &omap2430_mcbsp3_hwmod, "mcbsp3_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_core__mcbsp4, &omap2xxx_l4_core_hwmod, &omap2430_mcbsp4_hwmod, "mcbsp4_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_core__mcbsp5, &omap2xxx_l4_core_hwmod, &omap2430_mcbsp5_hwmod, "mcbsp5_ick", OCP_USER_MPU | OCP_USER_SDMA, 0);
ocp!(omap2430_l4_core__hdq1w, &omap2xxx_l4_core_hwmod, &omap2430_hdq1w_hwmod, "hdq_ick", OCP_USER_MPU | OCP_USER_SDMA, OMAP_FIREWALL_L4 | OCPIF_SWSUP_IDLE);
ocp!(omap2430_l3__gpmc, &omap2xxx_l3_main_hwmod, &omap2xxx_gpmc_hwmod, "core_l3_ck", OCP_USER_MPU | OCP_USER_SDMA, 0);

static mut omap2430_hwmod_ocp_ifs: [*mut omap_hwmod_ocp_if; 51] = [
    &mut omap2xxx_l3_main__l4_core, &mut omap2xxx_mpu__l3_main, &mut omap2xxx_dss__l3,
    &mut omap2430_usbhsotg__l3, &mut omap2430_l4_core__i2c1, &mut omap2430_l4_core__i2c2,
    &mut omap2xxx_l4_core__l4_wkup, &mut omap2_l4_core__uart1, &mut omap2_l4_core__uart2,
    &mut omap2_l4_core__uart3,
    &mut omap2430_l4_core__usbhsotg, &mut omap2430_l4_core__mmc1, &mut omap2430_l4_core__mmc2,
    &mut omap2xxx_l4_core__mcspi1, &mut omap2xxx_l4_core__mcspi2,
    &mut omap2430_l4_core__mcspi3, &mut omap2430_l3__iva, &mut omap2430_l4_wkup__wd_timer2,
    &mut omap2xxx_l4_core__timer3, &mut omap2xxx_l4_core__timer4, &mut omap2xxx_l4_core__timer5,
    &mut omap2xxx_l4_core__timer6, &mut omap2xxx_l4_core__timer7, &mut omap2xxx_l4_core__timer8,
    &mut omap2xxx_l4_core__timer9, &mut omap2xxx_l4_core__timer10, &mut omap2xxx_l4_core__timer11,
    &mut omap2xxx_l4_core__timer12,
    &mut omap2xxx_l4_core__dss, &mut omap2xxx_l4_core__dss_dispc,
    &mut omap2xxx_l4_core__dss_rfbi, &mut omap2xxx_l4_core__dss_venc,
    &mut omap2430_l4_wkup__gpio1, &mut omap2430_l4_wkup__gpio2, &mut omap2430_l4_wkup__gpio3,
    &mut omap2430_l4_wkup__gpio4, &mut omap2430_l4_core__gpio5, &mut omap2430_l4_core__mailbox,
    &mut omap2430_l4_core__mcbsp1, &mut omap2430_l4_core__mcbsp2, &mut omap2430_l4_core__mcbsp3,
    &mut omap2430_l4_core__mcbsp4, &mut omap2430_l4_core__mcbsp5, &mut omap2430_l4_core__hdq1w,
    &mut omap2xxx_l4_core__rng, &mut omap2xxx_l4_core__sham, &mut omap2xxx_l4_core__aes,
    &mut omap2430_l3__gpmc,
];

pub unsafe fn omap2430_hwmod_init() -> i32 {
    omap_hwmod_init();
    omap_hwmod_register_links(omap2430_hwmod_ocp_ifs.as_mut_ptr())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
