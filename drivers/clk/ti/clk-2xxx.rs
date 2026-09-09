// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP2 Clock init
 *
 * Copyright (C) 2013 Texas Instruments, Inc
 *     Tero Kristo (t-kristo@ti.com)
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct TiDtClk {
    pub node_name: *const core::ffi::c_char,
    pub con_id: *const core::ffi::c_char,
    pub clk_name: *const core::ffi::c_char,
}

macro_rules! dt_clk {
    ($node:expr, $con:expr, $clk:expr) => {
        TiDtClk {
            node_name: $node,
            con_id: $con,
            clk_name: $clk,
        }
    };
}

macro_rules! cstr {
    ($s:literal) => { concat!($s, "\0").as_ptr() as *const core::ffi::c_char };
}

static mut OMAP2XXX_CLKS: &[TiDtClk] = &[
    dt_clk!(core::ptr::null(), cstr!("func_32k_ck"), cstr!("func_32k_ck")),
    dt_clk!(core::ptr::null(), cstr!("secure_32k_ck"), cstr!("secure_32k_ck")),
    dt_clk!(core::ptr::null(), cstr!("virt_12m_ck"), cstr!("virt_12m_ck")),
    dt_clk!(core::ptr::null(), cstr!("virt_13m_ck"), cstr!("virt_13m_ck")),
    dt_clk!(core::ptr::null(), cstr!("virt_19200000_ck"), cstr!("virt_19200000_ck")),
    dt_clk!(core::ptr::null(), cstr!("virt_26m_ck"), cstr!("virt_26m_ck")),
    dt_clk!(core::ptr::null(), cstr!("aplls_clkin_ck"), cstr!("aplls_clkin_ck")),
    dt_clk!(core::ptr::null(), cstr!("aplls_clkin_x2_ck"), cstr!("aplls_clkin_x2_ck")),
    dt_clk!(core::ptr::null(), cstr!("osc_ck"), cstr!("osc_ck")),
    dt_clk!(core::ptr::null(), cstr!("sys_ck"), cstr!("sys_ck")),
    dt_clk!(core::ptr::null(), cstr!("alt_ck"), cstr!("alt_ck")),
    dt_clk!(core::ptr::null(), cstr!("mcbsp_clks"), cstr!("mcbsp_clks")),
    dt_clk!(core::ptr::null(), cstr!("dpll_ck"), cstr!("dpll_ck")),
    dt_clk!(core::ptr::null(), cstr!("apll96_ck"), cstr!("apll96_ck")),
    dt_clk!(core::ptr::null(), cstr!("apll54_ck"), cstr!("apll54_ck")),
    dt_clk!(core::ptr::null(), cstr!("func_54m_ck"), cstr!("func_54m_ck")),
    dt_clk!(core::ptr::null(), cstr!("core_ck"), cstr!("core_ck")),
    dt_clk!(core::ptr::null(), cstr!("func_96m_ck"), cstr!("func_96m_ck")),
    dt_clk!(core::ptr::null(), cstr!("func_48m_ck"), cstr!("func_48m_ck")),
    dt_clk!(core::ptr::null(), cstr!("func_12m_ck"), cstr!("func_12m_ck")),
    dt_clk!(core::ptr::null(), cstr!("sys_clkout_src"), cstr!("sys_clkout_src")),
    dt_clk!(core::ptr::null(), cstr!("sys_clkout"), cstr!("sys_clkout")),
    dt_clk!(core::ptr::null(), cstr!("emul_ck"), cstr!("emul_ck")),
    dt_clk!(core::ptr::null(), cstr!("mpu_ck"), cstr!("mpu_ck")),
    dt_clk!(core::ptr::null(), cstr!("dsp_fck"), cstr!("dsp_fck")),
    dt_clk!(core::ptr::null(), cstr!("gfx_3d_fck"), cstr!("gfx_3d_fck")),
    dt_clk!(core::ptr::null(), cstr!("gfx_2d_fck"), cstr!("gfx_2d_fck")),
    dt_clk!(core::ptr::null(), cstr!("gfx_ick"), cstr!("gfx_ick")),
    dt_clk!(cstr!("omapdss_dss"), cstr!("ick"), cstr!("dss_ick")),
    dt_clk!(core::ptr::null(), cstr!("dss_ick"), cstr!("dss_ick")),
    dt_clk!(core::ptr::null(), cstr!("dss1_fck"), cstr!("dss1_fck")),
    dt_clk!(core::ptr::null(), cstr!("dss2_fck"), cstr!("dss2_fck")),
    dt_clk!(core::ptr::null(), cstr!("dss_54m_fck"), cstr!("dss_54m_fck")),
    dt_clk!(core::ptr::null(), cstr!("core_l3_ck"), cstr!("core_l3_ck")),
    dt_clk!(core::ptr::null(), cstr!("ssi_fck"), cstr!("ssi_ssr_sst_fck")),
    dt_clk!(core::ptr::null(), cstr!("usb_l4_ick"), cstr!("usb_l4_ick")),
    dt_clk!(core::ptr::null(), cstr!("l4_ck"), cstr!("l4_ck")),
    dt_clk!(core::ptr::null(), cstr!("ssi_l4_ick"), cstr!("ssi_l4_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpt1_ick"), cstr!("gpt1_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpt1_fck"), cstr!("gpt1_fck")),
    dt_clk!(core::ptr::null(), cstr!("gpt2_ick"), cstr!("gpt2_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpt2_fck"), cstr!("gpt2_fck")),
    dt_clk!(core::ptr::null(), cstr!("gpt3_ick"), cstr!("gpt3_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpt3_fck"), cstr!("gpt3_fck")),
    dt_clk!(core::ptr::null(), cstr!("gpt4_ick"), cstr!("gpt4_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpt4_fck"), cstr!("gpt4_fck")),
    dt_clk!(core::ptr::null(), cstr!("gpt5_ick"), cstr!("gpt5_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpt5_fck"), cstr!("gpt5_fck")),
    dt_clk!(core::ptr::null(), cstr!("gpt6_ick"), cstr!("gpt6_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpt6_fck"), cstr!("gpt6_fck")),
    dt_clk!(core::ptr::null(), cstr!("gpt7_ick"), cstr!("gpt7_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpt7_fck"), cstr!("gpt7_fck")),
    dt_clk!(core::ptr::null(), cstr!("gpt8_ick"), cstr!("gpt8_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpt8_fck"), cstr!("gpt8_fck")),
    dt_clk!(core::ptr::null(), cstr!("gpt9_ick"), cstr!("gpt9_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpt9_fck"), cstr!("gpt9_fck")),
    dt_clk!(core::ptr::null(), cstr!("gpt10_ick"), cstr!("gpt10_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpt10_fck"), cstr!("gpt10_fck")),
    dt_clk!(core::ptr::null(), cstr!("gpt11_ick"), cstr!("gpt11_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpt11_fck"), cstr!("gpt11_fck")),
    dt_clk!(core::ptr::null(), cstr!("gpt12_ick"), cstr!("gpt12_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpt12_fck"), cstr!("gpt12_fck")),
    dt_clk!(cstr!("omap-mcbsp.1"), cstr!("ick"), cstr!("mcbsp1_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcbsp1_ick"), cstr!("mcbsp1_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcbsp1_fck"), cstr!("mcbsp1_fck")),
    dt_clk!(cstr!("omap-mcbsp.2"), cstr!("ick"), cstr!("mcbsp2_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcbsp2_ick"), cstr!("mcbsp2_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcbsp2_fck"), cstr!("mcbsp2_fck")),
    dt_clk!(cstr!("omap2_mcspi.1"), cstr!("ick"), cstr!("mcspi1_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcspi1_ick"), cstr!("mcspi1_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcspi1_fck"), cstr!("mcspi1_fck")),
    dt_clk!(cstr!("omap2_mcspi.2"), cstr!("ick"), cstr!("mcspi2_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcspi2_ick"), cstr!("mcspi2_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcspi2_fck"), cstr!("mcspi2_fck")),
    dt_clk!(core::ptr::null(), cstr!("uart1_ick"), cstr!("uart1_ick")),
    dt_clk!(core::ptr::null(), cstr!("uart1_fck"), cstr!("uart1_fck")),
    dt_clk!(core::ptr::null(), cstr!("uart2_ick"), cstr!("uart2_ick")),
    dt_clk!(core::ptr::null(), cstr!("uart2_fck"), cstr!("uart2_fck")),
    dt_clk!(core::ptr::null(), cstr!("uart3_ick"), cstr!("uart3_ick")),
    dt_clk!(core::ptr::null(), cstr!("uart3_fck"), cstr!("uart3_fck")),
    dt_clk!(core::ptr::null(), cstr!("gpios_ick"), cstr!("gpios_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpios_fck"), cstr!("gpios_fck")),
    dt_clk!(cstr!("omap_wdt"), cstr!("ick"), cstr!("mpu_wdt_ick")),
    dt_clk!(core::ptr::null(), cstr!("mpu_wdt_ick"), cstr!("mpu_wdt_ick")),
    dt_clk!(core::ptr::null(), cstr!("mpu_wdt_fck"), cstr!("mpu_wdt_fck")),
    dt_clk!(core::ptr::null(), cstr!("sync_32k_ick"), cstr!("sync_32k_ick")),
    dt_clk!(core::ptr::null(), cstr!("wdt1_ick"), cstr!("wdt1_ick")),
    dt_clk!(core::ptr::null(), cstr!("omapctrl_ick"), cstr!("omapctrl_ick")),
    dt_clk!(cstr!("omap24xxcam"), cstr!("fck"), cstr!("cam_fck")),
    dt_clk!(core::ptr::null(), cstr!("cam_fck"), cstr!("cam_fck")),
    dt_clk!(cstr!("omap24xxcam"), cstr!("ick"), cstr!("cam_ick")),
    dt_clk!(core::ptr::null(), cstr!("cam_ick"), cstr!("cam_ick")),
    dt_clk!(core::ptr::null(), cstr!("mailboxes_ick"), cstr!("mailboxes_ick")),
    dt_clk!(core::ptr::null(), cstr!("wdt4_ick"), cstr!("wdt4_ick")),
    dt_clk!(core::ptr::null(), cstr!("wdt4_fck"), cstr!("wdt4_fck")),
    dt_clk!(core::ptr::null(), cstr!("mspro_ick"), cstr!("mspro_ick")),
    dt_clk!(core::ptr::null(), cstr!("mspro_fck"), cstr!("mspro_fck")),
    dt_clk!(core::ptr::null(), cstr!("fac_ick"), cstr!("fac_ick")),
    dt_clk!(core::ptr::null(), cstr!("fac_fck"), cstr!("fac_fck")),
    dt_clk!(cstr!("omap_hdq.0"), cstr!("ick"), cstr!("hdq_ick")),
    dt_clk!(core::ptr::null(), cstr!("hdq_ick"), cstr!("hdq_ick")),
    dt_clk!(cstr!("omap_hdq.0"), cstr!("fck"), cstr!("hdq_fck")),
    dt_clk!(core::ptr::null(), cstr!("hdq_fck"), cstr!("hdq_fck")),
    dt_clk!(cstr!("omap_i2c.1"), cstr!("ick"), cstr!("i2c1_ick")),
    dt_clk!(core::ptr::null(), cstr!("i2c1_ick"), cstr!("i2c1_ick")),
    dt_clk!(cstr!("omap_i2c.2"), cstr!("ick"), cstr!("i2c2_ick")),
    dt_clk!(core::ptr::null(), cstr!("i2c2_ick"), cstr!("i2c2_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpmc_fck"), cstr!("gpmc_fck")),
    dt_clk!(core::ptr::null(), cstr!("sdma_fck"), cstr!("sdma_fck")),
    dt_clk!(core::ptr::null(), cstr!("sdma_ick"), cstr!("sdma_ick")),
    dt_clk!(core::ptr::null(), cstr!("sdrc_ick"), cstr!("sdrc_ick")),
    dt_clk!(core::ptr::null(), cstr!("des_ick"), cstr!("des_ick")),
    dt_clk!(cstr!("omap-sham"), cstr!("ick"), cstr!("sha_ick")),
    dt_clk!(core::ptr::null(), cstr!("sha_ick"), cstr!("sha_ick")),
    dt_clk!(cstr!("omap_rng"), cstr!("ick"), cstr!("rng_ick")),
    dt_clk!(core::ptr::null(), cstr!("rng_ick"), cstr!("rng_ick")),
    dt_clk!(cstr!("omap-aes"), cstr!("ick"), cstr!("aes_ick")),
    dt_clk!(core::ptr::null(), cstr!("aes_ick"), cstr!("aes_ick")),
    dt_clk!(core::ptr::null(), cstr!("pka_ick"), cstr!("pka_ick")),
    dt_clk!(core::ptr::null(), cstr!("usb_fck"), cstr!("usb_fck")),
    dt_clk!(core::ptr::null(), cstr!("timer_32k_ck"), cstr!("func_32k_ck")),
    dt_clk!(core::ptr::null(), cstr!("timer_sys_ck"), cstr!("sys_ck")),
    dt_clk!(core::ptr::null(), cstr!("timer_ext_ck"), cstr!("alt_ck")),
];

static OMAP2420_CLKS: &[TiDtClk] = &[
    dt_clk!(core::ptr::null(), cstr!("sys_clkout2_src"), cstr!("sys_clkout2_src")),
    dt_clk!(core::ptr::null(), cstr!("sys_clkout2"), cstr!("sys_clkout2")),
    dt_clk!(core::ptr::null(), cstr!("dsp_ick"), cstr!("dsp_ick")),
    dt_clk!(core::ptr::null(), cstr!("iva1_ifck"), cstr!("iva1_ifck")),
    dt_clk!(core::ptr::null(), cstr!("iva1_mpu_int_ifck"), cstr!("iva1_mpu_int_ifck")),
    dt_clk!(core::ptr::null(), cstr!("wdt3_ick"), cstr!("wdt3_ick")),
    dt_clk!(core::ptr::null(), cstr!("wdt3_fck"), cstr!("wdt3_fck")),
    dt_clk!(cstr!("mmci-omap.0"), cstr!("ick"), cstr!("mmc_ick")),
    dt_clk!(core::ptr::null(), cstr!("mmc_ick"), cstr!("mmc_ick")),
    dt_clk!(cstr!("mmci-omap.0"), cstr!("fck"), cstr!("mmc_fck")),
    dt_clk!(core::ptr::null(), cstr!("mmc_fck"), cstr!("mmc_fck")),
    dt_clk!(core::ptr::null(), cstr!("eac_ick"), cstr!("eac_ick")),
    dt_clk!(core::ptr::null(), cstr!("eac_fck"), cstr!("eac_fck")),
    dt_clk!(core::ptr::null(), cstr!("i2c1_fck"), cstr!("i2c1_fck")),
    dt_clk!(core::ptr::null(), cstr!("i2c2_fck"), cstr!("i2c2_fck")),
    dt_clk!(core::ptr::null(), cstr!("vlynq_ick"), cstr!("vlynq_ick")),
    dt_clk!(core::ptr::null(), cstr!("vlynq_fck"), cstr!("vlynq_fck")),
    dt_clk!(cstr!("musb-hdrc"), cstr!("fck"), cstr!("osc_ck")),
];

static OMAP2430_CLKS: &[TiDtClk] = &[
    dt_clk!(cstr!("twl"), cstr!("fck"), cstr!("osc_ck")),
    dt_clk!(core::ptr::null(), cstr!("iva2_1_ick"), cstr!("iva2_1_ick")),
    dt_clk!(core::ptr::null(), cstr!("mdm_ick"), cstr!("mdm_ick")),
    dt_clk!(core::ptr::null(), cstr!("mdm_osc_ck"), cstr!("mdm_osc_ck")),
    dt_clk!(cstr!("omap-mcbsp.3"), cstr!("ick"), cstr!("mcbsp3_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcbsp3_ick"), cstr!("mcbsp3_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcbsp3_fck"), cstr!("mcbsp3_fck")),
    dt_clk!(cstr!("omap-mcbsp.4"), cstr!("ick"), cstr!("mcbsp4_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcbsp4_ick"), cstr!("mcbsp4_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcbsp4_fck"), cstr!("mcbsp4_fck")),
    dt_clk!(cstr!("omap-mcbsp.5"), cstr!("ick"), cstr!("mcbsp5_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcbsp5_ick"), cstr!("mcbsp5_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcbsp5_fck"), cstr!("mcbsp5_fck")),
    dt_clk!(cstr!("omap2_mcspi.3"), cstr!("ick"), cstr!("mcspi3_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcspi3_ick"), cstr!("mcspi3_ick")),
    dt_clk!(core::ptr::null(), cstr!("mcspi3_fck"), cstr!("mcspi3_fck")),
    dt_clk!(core::ptr::null(), cstr!("icr_ick"), cstr!("icr_ick")),
    dt_clk!(core::ptr::null(), cstr!("i2chs1_fck"), cstr!("i2chs1_fck")),
    dt_clk!(core::ptr::null(), cstr!("i2chs2_fck"), cstr!("i2chs2_fck")),
    dt_clk!(cstr!("musb-omap2430"), cstr!("ick"), cstr!("usbhs_ick")),
    dt_clk!(core::ptr::null(), cstr!("usbhs_ick"), cstr!("usbhs_ick")),
    dt_clk!(cstr!("omap_hsmmc.0"), cstr!("ick"), cstr!("mmchs1_ick")),
    dt_clk!(core::ptr::null(), cstr!("mmchs1_ick"), cstr!("mmchs1_ick")),
    dt_clk!(core::ptr::null(), cstr!("mmchs1_fck"), cstr!("mmchs1_fck")),
    dt_clk!(cstr!("omap_hsmmc.1"), cstr!("ick"), cstr!("mmchs2_ick")),
    dt_clk!(core::ptr::null(), cstr!("mmchs2_ick"), cstr!("mmchs2_ick")),
    dt_clk!(core::ptr::null(), cstr!("mmchs2_fck"), cstr!("mmchs2_fck")),
    dt_clk!(core::ptr::null(), cstr!("gpio5_ick"), cstr!("gpio5_ick")),
    dt_clk!(core::ptr::null(), cstr!("gpio5_fck"), cstr!("gpio5_fck")),
    dt_clk!(core::ptr::null(), cstr!("mdm_intc_ick"), cstr!("mdm_intc_ick")),
    dt_clk!(cstr!("omap_hsmmc.0"), cstr!("mmchsdb_fck"), cstr!("mmchsdb1_fck")),
    dt_clk!(core::ptr::null(), cstr!("mmchsdb1_fck"), cstr!("mmchsdb1_fck")),
    dt_clk!(cstr!("omap_hsmmc.1"), cstr!("mmchsdb_fck"), cstr!("mmchsdb2_fck")),
    dt_clk!(core::ptr::null(), cstr!("mmchsdb2_fck"), cstr!("mmchsdb2_fck")),
];

static ENABLE_INIT_CLKS: &[&core::ffi::CStr] = &[
    c"apll96_ck", c"apll54_ck", c"sync_32k_ick", c"omapctrl_ick", c"gpmc_fck", c"sdrc_ick",
];

#[repr(i32)]
enum Omap2Soc { Omap2420 = 0, Omap2430 = 1 }

extern "C" {
    fn ti_dt_clocks_register(clks: *const TiDtClk);
    fn omap2xxx_clkt_vps_init();
    fn omap2_clk_disable_autoidle_all();
    fn omap2_clk_enable_init_clocks(clks: *const *const core::ffi::c_char, count: usize);
    fn clk_get_sys(dev_id: *const core::ffi::c_char, con_id: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn clk_get_rate(clk: *mut core::ffi::c_void) -> libc::c_ulong;
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

#[no_mangle]
pub unsafe extern "C" fn omap2xxx_dt_clk_init(soc_type: i32) -> i32 {
    ti_dt_clocks_register(OMAP2XXX_CLKS.as_ptr());
    if soc_type == Omap2Soc::Omap2420 as i32 {
        ti_dt_clocks_register(OMAP2420_CLKS.as_ptr());
    } else {
        ti_dt_clocks_register(OMAP2430_CLKS.as_ptr());
    }
    omap2xxx_clkt_vps_init();
    omap2_clk_disable_autoidle_all();
    omap2_clk_enable_init_clocks(ENABLE_INIT_CLKS.as_ptr() as *const *const core::ffi::c_char, ENABLE_INIT_CLKS.len());
    pr_info(c"Clocking rate (Crystal/DPLL/MPU): %ld.%01ld/%ld/%ld MHz\n".as_ptr(),
        clk_get_rate(clk_get_sys(core::ptr::null(), c"sys_ck".as_ptr())) / 1_000_000,
        (clk_get_rate(clk_get_sys(core::ptr::null(), c"sys_ck".as_ptr())) / 100_000) % 10,
        clk_get_rate(clk_get_sys(core::ptr::null(), c"dpll_ck".as_ptr())) / 1_000_000,
        clk_get_rate(clk_get_sys(core::ptr::null(), c"mpu_ck".as_ptr())) / 1_000_000);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
