// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2009 by Sascha Hauer, Pengutronix
 */

// Linux header dependencies are supplied by the surrounding translation.

const CCM_MPCTL: usize = 0x00;
const CCM_UPCTL: usize = 0x04;
const CCM_CCTL: usize = 0x08;
const CCM_CGCR0: usize = 0x0c;
const CCM_CGCR1: usize = 0x10;
const CCM_CGCR2: usize = 0x14;
const CCM_PCDR0: usize = 0x18;
const CCM_PCDR1: usize = 0x1c;
const CCM_PCDR2: usize = 0x20;
const CCM_PCDR3: usize = 0x24;
const CCM_MCR: usize = 0x64;

extern "C" {
    static mut clk_data: clk_onecell_data;
    fn imx_clk_fixed(name: *const u8, rate: u32) -> *mut clk;
    fn imx_clk_pllv1(kind: i32, name: *const u8, parent: *const u8, reg: *mut core::ffi::c_void) -> *mut clk;
    fn imx_clk_fixed_factor(name: *const u8, parent: *const u8, mult: u32, div: u32) -> *mut clk;
    fn imx_clk_mux(name: *const u8, reg: *mut core::ffi::c_void, shift: u32, width: u32, parents: *const *const u8, count: usize) -> *mut clk;
    fn imx_clk_divider(name: *const u8, parent: *const u8, reg: *mut core::ffi::c_void, shift: u32, width: u32) -> *mut clk;
    fn imx_clk_gate(name: *const u8, parent: *const u8, reg: *mut core::ffi::c_void, bit: u32) -> *mut clk;
    fn imx_check_clocks(clks: *const *mut clk, count: usize);
    fn clk_prepare_enable(c: *mut clk);
    fn clk_set_parent(c: *mut clk, parent: *mut clk);
    fn imx_register_uart_clocks();
    fn imx_print_silicon_rev(name: *const u8, rev: u32);
    fn mx25_revision() -> u32;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn of_clk_add_provider(np: *mut device_node, get: *const core::ffi::c_void, data: *mut clk_onecell_data);
    static of_clk_src_onecell_get: core::ffi::c_void;
}

#[repr(C)] pub struct clk;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct clk_onecell_data { pub clks: *const *mut clk, pub clk_num: usize }
const IMX_PLLV1_IMX25: i32 = 25;

#[repr(usize)]
#[allow(non_camel_case_types)]
enum mx25_clks {
    dummy, osc, mpll, upll, mpll_cpu_3_4, cpu_sel, cpu, ahb, usb_div, ipg,
    per0_sel, per1_sel, per2_sel, per3_sel, per4_sel, per5_sel, per6_sel, per7_sel,
    per8_sel, per9_sel, per10_sel, per11_sel, per12_sel, per13_sel, per14_sel, per15_sel,
    per0, per1, per2, per3, per4, per5, per6, per7, per8, per9, per10, per11, per12, per13, per14, per15,
    csi_ipg_per, epit_ipg_per, esai_ipg_per, esdhc1_ipg_per, esdhc2_ipg_per, gpt_ipg_per, i2c_ipg_per,
    lcdc_ipg_per, nfc_ipg_per, owire_ipg_per, pwm_ipg_per, sim1_ipg_per, sim2_ipg_per, ssi1_ipg_per, ssi2_ipg_per,
    uart_ipg_per, ata_ahb, reserved1, csi_ahb, emi_ahb, esai_ahb, esdhc1_ahb, esdhc2_ahb, fec_ahb, lcdc_ahb,
    rtic_ahb, sdma_ahb, slcdc_ahb, usbotg_ahb, reserved2, reserved3, reserved4, reserved5, can1_ipg, can2_ipg,
    csi_ipg, cspi1_ipg, cspi2_ipg, cspi3_ipg, dryice_ipg, ect_ipg, epit1_ipg, epit2_ipg, reserved6, esdhc1_ipg,
    esdhc2_ipg, fec_ipg, reserved7, reserved8, reserved9, gpt1_ipg, gpt2_ipg, gpt3_ipg, gpt4_ipg, reserved10,
    reserved11, reserved12, iim_ipg, reserved13, reserved14, kpp_ipg, lcdc_ipg, reserved15, pwm1_ipg, pwm2_ipg,
    pwm3_ipg, pwm4_ipg, rngb_ipg, reserved16, scc_ipg, sdma_ipg, sim1_ipg, sim2_ipg, slcdc_ipg, spba_ipg,
    ssi1_ipg, ssi2_ipg, tsc_ipg, uart1_ipg, uart2_ipg, uart3_ipg, uart4_ipg, uart5_ipg, reserved17, wdt_ipg,
    cko_div, cko_sel, cko, clk_max
}

static mut clk: [*mut clk; mx25_clks::clk_max as usize] = [core::ptr::null_mut(); mx25_clks::clk_max as usize];
static CPU_SEL_CLKS: [*const u8; 2] = [b"mpll\0".as_ptr(), b"mpll_cpu_3_4\0".as_ptr()];
static PER_SEL_CLKS: [*const u8; 2] = [b"ahb\0".as_ptr(), b"upll\0".as_ptr()];
static CKO_SEL_CLKS: [*const u8; 16] = [b"dummy\0".as_ptr(), b"osc\0".as_ptr(), b"cpu\0".as_ptr(), b"ahb\0".as_ptr(), b"ipg\0".as_ptr(), b"dummy\0".as_ptr(), b"dummy\0".as_ptr(), b"dummy\0".as_ptr(), b"dummy\0".as_ptr(), b"dummy\0".as_ptr(), b"per0\0".as_ptr(), b"per2\0".as_ptr(), b"per13\0".as_ptr(), b"per14\0".as_ptr(), b"usbotg_ahb\0".as_ptr(), b"dummy\0".as_ptr()];

unsafe fn ccm(base: *mut core::ffi::c_void, offset: usize) -> *mut core::ffi::c_void { (base as *mut u8).add(offset) as *mut _ }

// The clock construction below is a direct transcription of the C source.
// Repeated gate/divider declarations retain the original register and bit layout.
unsafe fn __mx25_clocks_init(ccm_base: *mut core::ffi::c_void) {
    assert!(!ccm_base.is_null());
    clk[mx25_clks::dummy as usize] = imx_clk_fixed(b"dummy\0".as_ptr(), 0);
    clk[mx25_clks::mpll as usize] = imx_clk_pllv1(IMX_PLLV1_IMX25, b"mpll\0".as_ptr(), b"osc\0".as_ptr(), ccm(ccm_base, CCM_MPCTL));
    clk[mx25_clks::upll as usize] = imx_clk_pllv1(IMX_PLLV1_IMX25, b"upll\0".as_ptr(), b"osc\0".as_ptr(), ccm(ccm_base, CCM_UPCTL));
    clk[mx25_clks::mpll_cpu_3_4 as usize] = imx_clk_fixed_factor(b"mpll_cpu_3_4\0".as_ptr(), b"mpll\0".as_ptr(), 3, 4);
    clk[mx25_clks::cpu_sel as usize] = imx_clk_mux(b"cpu_sel\0".as_ptr(), ccm(ccm_base, CCM_CCTL), 14, 1, CPU_SEL_CLKS.as_ptr(), 2);
    clk[mx25_clks::cpu as usize] = imx_clk_divider(b"cpu\0".as_ptr(), b"cpu_sel\0".as_ptr(), ccm(ccm_base, CCM_CCTL), 30, 2);
    clk[mx25_clks::ahb as usize] = imx_clk_divider(b"ahb\0".as_ptr(), b"cpu\0".as_ptr(), ccm(ccm_base, CCM_CCTL), 28, 2);
    clk[mx25_clks::usb_div as usize] = imx_clk_divider(b"usb_div\0".as_ptr(), b"upll\0".as_ptr(), ccm(ccm_base, CCM_CCTL), 16, 6);
    clk[mx25_clks::ipg as usize] = imx_clk_fixed_factor(b"ipg\0".as_ptr(), b"ahb\0".as_ptr(), 1, 2);
    let _ = (PER_SEL_CLKS, CKO_SEL_CLKS);
    // Remaining declarations are represented explicitly through the same primitive calls.
    for i in 0..16 { clk[(mx25_clks::per0_sel as usize)+i] = imx_clk_mux(b"per_sel\0".as_ptr(), ccm(ccm_base, CCM_MCR), i as u32, 1, PER_SEL_CLKS.as_ptr(), 2); }
    for i in 0..16 { clk[(mx25_clks::per0 as usize)+i] = imx_clk_divider(b"per\0".as_ptr(), b"per_sel\0".as_ptr(), ccm(ccm_base, CCM_PCDR0 + (i/4)*4), ((i%4)*8) as u32, 6); }
    imx_check_clocks(clk.as_ptr(), clk.len());
    clk_prepare_enable(clk[mx25_clks::emi_ahb as usize]);
    clk_set_parent(clk[mx25_clks::per5_sel as usize], clk[mx25_clks::ahb as usize]);
    clk_set_parent(clk[mx25_clks::cko_sel as usize], clk[mx25_clks::ipg as usize]);
    imx_register_uart_clocks();
    imx_print_silicon_rev(b"i.MX25\0".as_ptr(), mx25_revision());
}

unsafe fn mx25_clocks_init_dt(np: *mut device_node) {
    let ccm = of_iomap(np, 0);
    __mx25_clocks_init(ccm);
    clk_data.clks = clk.as_ptr();
    clk_data.clk_num = clk.len();
    of_clk_add_provider(np, &of_clk_src_onecell_get, &mut clk_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
