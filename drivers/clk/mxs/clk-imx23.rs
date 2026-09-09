// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the Linux clock, OF, IO, and local clock headers
// are intentionally left as external names.

use core::ffi::c_void;

static mut clkctrl: *mut c_void = core::ptr::null_mut();
static mut digctrl: *mut c_void = core::ptr::null_mut();

const SET: usize = 0x4;
const CLR: usize = 0x8;

macro_rules! reg {
    ($base:expr, $offset:expr) => {
        (($base as usize).wrapping_add($offset) as *mut c_void)
    };
}

macro_rules! CLKCTRL { () => { clkctrl }; }
macro_rules! DIGCTRL { () => { digctrl }; }
macro_rules! PLLCTRL0 { () => { reg!(CLKCTRL!(), 0x0000) }; }
macro_rules! CPU { () => { reg!(CLKCTRL!(), 0x0020) }; }
macro_rules! HBUS { () => { reg!(CLKCTRL!(), 0x0030) }; }
macro_rules! XBUS { () => { reg!(CLKCTRL!(), 0x0040) }; }
macro_rules! XTAL { () => { reg!(CLKCTRL!(), 0x0050) }; }
macro_rules! PIX { () => { reg!(CLKCTRL!(), 0x0060) }; }
macro_rules! SSP { () => { reg!(CLKCTRL!(), 0x0070) }; }
macro_rules! GPMI { () => { reg!(CLKCTRL!(), 0x0080) }; }
macro_rules! SPDIF { () => { reg!(CLKCTRL!(), 0x0090) }; }
macro_rules! EMI { () => { reg!(CLKCTRL!(), 0x00a0) }; }
macro_rules! SAIF { () => { reg!(CLKCTRL!(), 0x00c0) }; }
macro_rules! ETM { () => { reg!(CLKCTRL!(), 0x00e0) }; }
macro_rules! FRAC { () => { reg!(CLKCTRL!(), 0x00f0) }; }
macro_rules! CLKSEQ { () => { reg!(CLKCTRL!(), 0x0110) }; }

const BP_CPU_INTERRUPT_WAIT: u32 = 12;
const BP_CLKSEQ_BYPASS_SAIF: u32 = 0;
const BP_CLKSEQ_BYPASS_SSP: u32 = 5;
const BP_SAIF_DIV_FRAC_EN: u32 = 16;
const BP_FRAC_IOFRAC: u32 = 24;

unsafe fn clk_misc_init() {
    let mut val: u32;

    /* Gate off cpu clock in WFI for power saving */
    writel_relaxed(1u32 << BP_CPU_INTERRUPT_WAIT, reg!(CPU!(), SET));

    /* Clear BYPASS for SAIF */
    writel_relaxed(1u32 << BP_CLKSEQ_BYPASS_SAIF, reg!(CLKSEQ!(), CLR));

    /* SAIF has to use frac div for functional operation */
    val = readl_relaxed(SAIF!());
    val |= 1u32 << BP_SAIF_DIV_FRAC_EN;
    writel_relaxed(val, SAIF!());

    /*
     * Source ssp clock from ref_io than ref_xtal,
     * as ref_xtal only provides 24 MHz as maximum.
     */
    writel_relaxed(1u32 << BP_CLKSEQ_BYPASS_SSP, reg!(CLKSEQ!(), CLR));

    /*
     * 480 MHz seems too high to be ssp clock source directly,
     * so set frac to get a 288 MHz ref_io.
     */
    writel_relaxed(0x3fu32 << BP_FRAC_IOFRAC, reg!(FRAC!(), CLR));
    writel_relaxed(30u32 << BP_FRAC_IOFRAC, reg!(FRAC!(), SET));
}

static sel_pll: [&str; 2] = ["pll", "ref_xtal"];
static sel_cpu: [&str; 2] = ["ref_cpu", "ref_xtal"];
static sel_pix: [&str; 2] = ["ref_pix", "ref_xtal"];
static sel_io: [&str; 2] = ["ref_io", "ref_xtal"];
static cpu_sels: [&str; 2] = ["cpu_pll", "cpu_xtal"];
static emi_sels: [&str; 2] = ["emi_pll", "emi_xtal"];

#[repr(usize)]
enum imx23_clk {
    ref_xtal, pll, ref_cpu, ref_emi, ref_pix, ref_io, saif_sel,
    lcdif_sel, gpmi_sel, ssp_sel, emi_sel, cpu, etm_sel, cpu_pll,
    cpu_xtal, hbus, xbus, lcdif_div, ssp_div, gpmi_div, emi_pll,
    emi_xtal, etm_div, saif_div, clk32k_div, rtc, adc, spdif_div,
    clk32k, dri, pwm, filt, uart, ssp, gpmi, spdif, emi, saif,
    lcdif, etm, usb, usb_phy, clk_max,
}

static mut clks: [*mut clk; clk_max as usize] = [core::ptr::null_mut(); clk_max as usize];
static mut clk_data: clk_onecell_data = clk_onecell_data { clks: core::ptr::null_mut(), clk_num: 0 };
static clks_init_on: [imx23_clk; 5] = [imx23_clk::cpu, imx23_clk::hbus, imx23_clk::xbus, imx23_clk::emi, imx23_clk::uart];

unsafe fn mx23_clocks_init(np: *mut device_node) {
    let dcnp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,imx23-digctl".as_ptr() as *const i8);
    digctrl = of_iomap(dcnp, 0);
    WARN_ON(digctrl.is_null());
    of_node_put(dcnp);
    clkctrl = of_iomap(np, 0);
    WARN_ON(clkctrl.is_null());
    clk_misc_init();

    clks[imx23_clk::ref_xtal as usize] = mxs_clk_fixed("ref_xtal", 24000000);
    clks[imx23_clk::pll as usize] = mxs_clk_pll("pll", "ref_xtal", PLLCTRL0!(), 16, 480000000);
    clks[imx23_clk::ref_cpu as usize] = mxs_clk_ref("ref_cpu", "pll", FRAC!(), 0);
    clks[imx23_clk::ref_emi as usize] = mxs_clk_ref("ref_emi", "pll", FRAC!(), 1);
    clks[imx23_clk::ref_pix as usize] = mxs_clk_ref("ref_pix", "pll", FRAC!(), 2);
    clks[imx23_clk::ref_io as usize] = mxs_clk_ref("ref_io", "pll", FRAC!(), 3);
    clks[imx23_clk::saif_sel as usize] = mxs_clk_mux("saif_sel", CLKSEQ!(), 0, 1, sel_pll.as_ptr(), sel_pll.len());
    clks[imx23_clk::lcdif_sel as usize] = mxs_clk_mux("lcdif_sel", CLKSEQ!(), 1, 1, sel_pix.as_ptr(), sel_pix.len());
    clks[imx23_clk::gpmi_sel as usize] = mxs_clk_mux("gpmi_sel", CLKSEQ!(), 4, 1, sel_io.as_ptr(), sel_io.len());
    clks[imx23_clk::ssp_sel as usize] = mxs_clk_mux("ssp_sel", CLKSEQ!(), 5, 1, sel_io.as_ptr(), sel_io.len());
    clks[imx23_clk::emi_sel as usize] = mxs_clk_mux("emi_sel", CLKSEQ!(), 6, 1, emi_sels.as_ptr(), emi_sels.len());
    clks[imx23_clk::cpu as usize] = mxs_clk_mux("cpu", CLKSEQ!(), 7, 1, cpu_sels.as_ptr(), cpu_sels.len());
    clks[imx23_clk::etm_sel as usize] = mxs_clk_mux("etm_sel", CLKSEQ!(), 8, 1, sel_cpu.as_ptr(), sel_cpu.len());
    clks[imx23_clk::cpu_pll as usize] = mxs_clk_div("cpu_pll", "ref_cpu", CPU!(), 0, 6, 28);
    clks[imx23_clk::cpu_xtal as usize] = mxs_clk_div("cpu_xtal", "ref_xtal", CPU!(), 16, 10, 29);
    clks[imx23_clk::hbus as usize] = mxs_clk_div("hbus", "cpu", HBUS!(), 0, 5, 29);
    clks[imx23_clk::xbus as usize] = mxs_clk_div("xbus", "ref_xtal", XBUS!(), 0, 10, 31);
    clks[imx23_clk::lcdif_div as usize] = mxs_clk_div("lcdif_div", "lcdif_sel", PIX!(), 0, 12, 29);
    clks[imx23_clk::ssp_div as usize] = mxs_clk_div("ssp_div", "ssp_sel", SSP!(), 0, 9, 29);
    clks[imx23_clk::gpmi_div as usize] = mxs_clk_div("gpmi_div", "gpmi_sel", GPMI!(), 0, 10, 29);
    clks[imx23_clk::emi_pll as usize] = mxs_clk_div("emi_pll", "ref_emi", EMI!(), 0, 6, 28);
    clks[imx23_clk::emi_xtal as usize] = mxs_clk_div("emi_xtal", "ref_xtal", EMI!(), 8, 4, 29);
    clks[imx23_clk::etm_div as usize] = mxs_clk_div("etm_div", "etm_sel", ETM!(), 0, 6, 29);
    clks[imx23_clk::saif_div as usize] = mxs_clk_frac("saif_div", "saif_sel", SAIF!(), 0, 16, 29);
    clks[imx23_clk::clk32k_div as usize] = mxs_clk_fixed_factor("clk32k_div", "ref_xtal", 1, 750);
    clks[imx23_clk::rtc as usize] = mxs_clk_fixed_factor("rtc", "ref_xtal", 1, 768);
    clks[imx23_clk::adc as usize] = mxs_clk_fixed_factor("adc", "clk32k", 1, 16);
    clks[imx23_clk::spdif_div as usize] = mxs_clk_fixed_factor("spdif_div", "pll", 1, 4);
    clks[imx23_clk::clk32k as usize] = mxs_clk_gate("clk32k", "clk32k_div", XTAL!(), 26);
    clks[imx23_clk::dri as usize] = mxs_clk_gate("dri", "ref_xtal", XTAL!(), 28);
    clks[imx23_clk::pwm as usize] = mxs_clk_gate("pwm", "ref_xtal", XTAL!(), 29);
    clks[imx23_clk::filt as usize] = mxs_clk_gate("filt", "ref_xtal", XTAL!(), 30);
    clks[imx23_clk::uart as usize] = mxs_clk_gate("uart", "ref_xtal", XTAL!(), 31);
    clks[imx23_clk::ssp as usize] = mxs_clk_gate("ssp", "ssp_div", SSP!(), 31);
    clks[imx23_clk::gpmi as usize] = mxs_clk_gate("gpmi", "gpmi_div", GPMI!(), 31);
    clks[imx23_clk::spdif as usize] = mxs_clk_gate("spdif", "spdif_div", SPDIF!(), 31);
    clks[imx23_clk::emi as usize] = mxs_clk_gate("emi", "emi_sel", EMI!(), 31);
    clks[imx23_clk::saif as usize] = mxs_clk_gate("saif", "saif_div", SAIF!(), 31);
    clks[imx23_clk::lcdif as usize] = mxs_clk_gate("lcdif", "lcdif_div", PIX!(), 31);
    clks[imx23_clk::etm as usize] = mxs_clk_gate("etm", "etm_div", ETM!(), 31);
    clks[imx23_clk::usb as usize] = mxs_clk_gate("usb", "usb_phy", DIGCTRL!(), 2);
    clks[imx23_clk::usb_phy as usize] = clk_register_gate(core::ptr::null_mut(), "usb_phy", "pll", 0, PLLCTRL0!(), 18, 0, &mxs_lock);

    let mut i = 0usize;
    while i < clks.len() {
        if IS_ERR(clks[i]) {
            pr_err("i.MX23 clk %d: register failed with %ld\n", i, PTR_ERR(clks[i]));
            return;
        }
        i += 1;
    }
    clk_data.clks = clks.as_mut_ptr();
    clk_data.clk_num = clks.len();
    of_clk_add_provider(np, of_clk_src_onecell_get, &mut clk_data);
    i = 0;
    while i < clks_init_on.len() {
        clk_prepare_enable(clks[clks_init_on[i] as usize]);
        i += 1;
    }
}

// CLK_OF_DECLARE(imx23_clkctrl, "fsl,imx23-clkctrl", mx23_clocks_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
