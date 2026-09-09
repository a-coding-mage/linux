// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Sascha Hauer, Pengutronix <s.hauer@pengutronix.de>
 */

// Linux/kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_char;
use core::ptr;

const MX35_CCM_BASE_ADDR: usize = 0x53f80000;
const MX35_GPT1_BASE_ADDR: usize = 0x53f90000;
const MX35_INT_GPT: usize = NR_IRQS_LEGACY + 29;
const MXC_CCM_PDR0: usize = 0x04;
const MX35_CCM_PDR2: usize = 0x0c;
const MX35_CCM_PDR3: usize = 0x10;
const MX35_CCM_PDR4: usize = 0x14;
const MX35_CCM_MPCTL: usize = 0x1c;
const MX35_CCM_PPCTL: usize = 0x20;
const MX35_CCM_CGR0: usize = 0x2c;
const MX35_CCM_CGR1: usize = 0x30;
const MX35_CCM_CGR2: usize = 0x34;
const MX35_CCM_CGR3: usize = 0x38;

#[repr(C)]
struct ArmAhbDiv { arm: u8, ahb: u8, sel: u8 }

static mut CLK_CONSUMER: [ArmAhbDiv; 16] = [
    ArmAhbDiv { arm: 1, ahb: 4, sel: 0 }, ArmAhbDiv { arm: 1, ahb: 3, sel: 1 },
    ArmAhbDiv { arm: 2, ahb: 2, sel: 0 }, ArmAhbDiv { arm: 0, ahb: 0, sel: 0 },
    ArmAhbDiv { arm: 0, ahb: 0, sel: 0 }, ArmAhbDiv { arm: 0, ahb: 0, sel: 0 },
    ArmAhbDiv { arm: 4, ahb: 1, sel: 0 }, ArmAhbDiv { arm: 1, ahb: 5, sel: 0 },
    ArmAhbDiv { arm: 1, ahb: 8, sel: 0 }, ArmAhbDiv { arm: 1, ahb: 6, sel: 1 },
    ArmAhbDiv { arm: 2, ahb: 4, sel: 0 }, ArmAhbDiv { arm: 0, ahb: 0, sel: 0 },
    ArmAhbDiv { arm: 0, ahb: 0, sel: 0 }, ArmAhbDiv { arm: 0, ahb: 0, sel: 0 },
    ArmAhbDiv { arm: 4, ahb: 2, sel: 0 }, ArmAhbDiv { arm: 0, ahb: 0, sel: 0 },
];
static HSP_DIV_532: [i8; 4] = [4, 8, 3, 0];
static HSP_DIV_400: [i8; 4] = [3, 6, 3, 0];
static mut CLK_DATA: ClkOnecellData = ClkOnecellData { _private: [] };
static STD_SEL: [&'static [u8]; 2] = [b"ppll\0", b"arm\0"];
static IPG_PER_SEL: [&'static [u8]; 2] = [b"ahb_per_div\0", b"arm_per_div\0"];

// The following declarations are provided by the kernel clock subsystem.
extern "C" {
    static NR_IRQS_LEGACY: usize;
    static mut clk: [*mut Clk; clk_max as usize];
    fn ioremap(addr: usize, size: usize) -> *mut u8;
    fn __raw_readl(addr: *mut u8) -> u32;
    fn pr_err(fmt: *const c_char, ...);
    fn bug_on(cond: bool);
    fn imx_clk_fixed(name: *const c_char, rate: u32) -> *mut Clk;
    fn imx_clk_pllv1(kind: i32, name: *const c_char, parent: *const c_char, reg: *mut u8) -> *mut Clk;
    fn imx_clk_fixed_factor(name: *const c_char, parent: *const c_char, mult: u32, div: u32) -> *mut Clk;
    fn clk_get_rate(clk: *mut Clk) -> u64;
    fn imx_clk_divider(name: *const c_char, parent: *const c_char, reg: *mut u8, shift: u8, width: u8) -> *mut Clk;
    fn imx_clk_mux(name: *const c_char, reg: *mut u8, shift: u8, width: u8, parents: *const &'static [u8], count: usize) -> *mut Clk;
    fn imx_clk_gate2(name: *const c_char, parent: *const c_char, reg: *mut u8, shift: u8) -> *mut Clk;
    fn imx_check_clocks(clks: *mut *mut Clk, count: usize);
    fn clk_prepare_enable(clk: *mut Clk);
    fn imx_register_uart_clocks();
    fn imx_print_silicon_rev(name: *const c_char, rev: i32);
    fn mx35_revision() -> i32;
}

#[repr(C)] struct Clk { _private: [u8; 0] }
#[repr(C)] struct ClkOnecellData { _private: [u8; 0] }
const IMX_PLLV1_IMX35: i32 = 35;

#[allow(non_camel_case_types)]
#[repr(usize)]
enum mx35_clks {
    ckih, mpll, ppll, mpll_075, arm, hsp, hsp_div, hsp_sel, ahb, ipg,
    arm_per_div, ahb_per_div, ipg_per, uart_sel, uart_div, esdhc_sel,
    esdhc1_div, esdhc2_div, esdhc3_div, spdif_sel, spdif_div_pre,
    spdif_div_post, ssi_sel, ssi1_div_pre, ssi1_div_post, ssi2_div_pre,
    ssi2_div_post, usb_sel, usb_div, nfc_div, asrc_gate, pata_gate,
    audmux_gate, can1_gate, can2_gate, cspi1_gate, cspi2_gate, ect_gate,
    edio_gate, emi_gate, epit1_gate, epit2_gate, esai_gate, esdhc1_gate,
    esdhc2_gate, esdhc3_gate, fec_gate, gpio1_gate, gpio2_gate, gpio3_gate,
    gpt_gate, i2c1_gate, i2c2_gate, i2c3_gate, iomuxc_gate, ipu_gate,
    kpp_gate, mlb_gate, mshc_gate, owire_gate, pwm_gate, rngc_gate,
    rtc_gate, rtic_gate, scc_gate, sdma_gate, spba_gate, spdif_gate,
    ssi1_gate, ssi2_gate, uart1_gate, uart2_gate, uart3_gate, usbotg_gate,
    wdog_gate, max_gate, admux_gate, csi_gate, csi_div, csi_sel, iim_gate,
    gpu2d_gate, ckil, clk_max,
}

// Direct translation of the initialization routine; individual clock setup
// declarations and register fields are retained through the subsystem calls.
#[allow(non_snake_case)]
unsafe fn _mx35_clocks_init() {
    let base = ioremap(MX35_CCM_BASE_ADDR, 4096);
    bug_on(base.is_null());
    let pdr0 = __raw_readl(base.add(MXC_CCM_PDR0));
    let consumer_sel = ((pdr0 >> 16) & 0xf) as usize;
    let mut aad = &CLK_CONSUMER[consumer_sel];
    if aad.arm == 0 { pr_err(b"i.MX35 clk: illegal consumer mux selection 0x%x\n".as_ptr() as *const c_char, consumer_sel); aad = &CLK_CONSUMER[0]; }
    clk[mx35_clks::ckih as usize] = imx_clk_fixed(b"ckih\0".as_ptr() as *const c_char, 24000000);
    clk[mx35_clks::ckil as usize] = imx_clk_fixed(b"ckil\0".as_ptr() as *const c_char, 32768);
    clk[mx35_clks::mpll as usize] = imx_clk_pllv1(IMX_PLLV1_IMX35, b"mpll\0".as_ptr() as _, b"ckih\0".as_ptr() as _, base.add(MX35_CCM_MPCTL));
    clk[mx35_clks::ppll as usize] = imx_clk_pllv1(IMX_PLLV1_IMX35, b"ppll\0".as_ptr() as _, b"ckih\0".as_ptr() as _, base.add(MX35_CCM_PPCTL));
    clk[mx35_clks::mpll as usize] = imx_clk_fixed_factor(b"mpll_075\0".as_ptr() as _, b"mpll\0".as_ptr() as _, 3, 4);
    clk[mx35_clks::arm as usize] = if aad.sel != 0 { imx_clk_fixed_factor(b"arm\0".as_ptr() as _, b"mpll_075\0".as_ptr() as _, 1, aad.arm as u32) } else { imx_clk_fixed_factor(b"arm\0".as_ptr() as _, b"mpll\0".as_ptr() as _, 1, aad.arm as u32) };
    let hsp_div = if clk_get_rate(clk[mx35_clks::arm as usize]) > 400000000 { &HSP_DIV_532 } else { &HSP_DIV_400 };
    let mut hs = ((pdr0 >> 20) & 3) as usize;
    if hsp_div[hs] == 0 { pr_err(b"i.MX35 clk: illegal hsp clk selection 0x%x\n".as_ptr() as _, hs); hs = 0; }
    clk[mx35_clks::hsp as usize] = imx_clk_fixed_factor(b"hsp\0".as_ptr() as _, b"arm\0".as_ptr() as _, 1, hsp_div[hs] as u32);
    clk[mx35_clks::ahb as usize] = imx_clk_fixed_factor(b"ahb\0".as_ptr() as _, b"arm\0".as_ptr() as _, 1, aad.ahb as u32);
    clk[mx35_clks::ipg as usize] = imx_clk_fixed_factor(b"ipg\0".as_ptr() as _, b"ahb\0".as_ptr() as _, 1, 2);
    macro_rules! mux { ($i:ident,$n:literal,$r:expr,$s:expr,$w:expr,$p:ident) => { clk[mx35_clks::$i as usize] = imx_clk_mux(concat!($n,"\0").as_ptr() as _, $r, $s, $w, $p.as_ptr(), $p.len()); }; }
    macro_rules! div { ($i:ident,$n:literal,$p:literal,$r:expr,$s:expr,$w:expr) => { clk[mx35_clks::$i as usize] = imx_clk_divider(concat!($n,"\0").as_ptr() as _, concat!($p,"\0").as_ptr() as _, $r, $s, $w); }; }
    macro_rules! gate { ($i:ident,$n:literal,$p:literal,$r:expr,$s:expr) => { clk[mx35_clks::$i as usize] = imx_clk_gate2(concat!($n,"\0").as_ptr() as _, concat!($p,"\0").as_ptr() as _, $r, $s); }; }
    div!(arm_per_div,"arm_per_div","arm",base.add(MX35_CCM_PDR4),16,6);
    div!(ahb_per_div,"ahb_per_div","ahb",base.add(MXC_CCM_PDR0),12,3);
    mux!(ipg_per,"ipg_per",base.add(MXC_CCM_PDR0),26,1,IPG_PER_SEL);
    mux!(uart_sel,"uart_sel",base.add(MX35_CCM_PDR3),14,1,STD_SEL); div!(uart_div,"uart_div","uart_sel",base.add(MX35_CCM_PDR4),10,6);
    mux!(esdhc_sel,"esdhc_sel",base.add(MX35_CCM_PDR4),9,1,STD_SEL); div!(esdhc1_div,"esdhc1_div","esdhc_sel",base.add(MX35_CCM_PDR3),0,6); div!(esdhc2_div,"esdhc2_div","esdhc_sel",base.add(MX35_CCM_PDR3),8,6); div!(esdhc3_div,"esdhc3_div","esdhc_sel",base.add(MX35_CCM_PDR3),16,6);
    mux!(spdif_sel,"spdif_sel",base.add(MX35_CCM_PDR3),22,1,STD_SEL); div!(spdif_div_pre,"spdif_div_pre","spdif_sel",base.add(MX35_CCM_PDR3),29,3); /* divide by 1 not allowed */ div!(spdif_div_post,"spdif_div_post","spdif_div_pre",base.add(MX35_CCM_PDR3),23,6);
    mux!(ssi_sel,"ssi_sel",base.add(MX35_CCM_PDR2),6,1,STD_SEL); div!(ssi1_div_pre,"ssi1_div_pre","ssi_sel",base.add(MX35_CCM_PDR2),24,3); div!(ssi1_div_post,"ssi1_div_post","ssi1_div_pre",base.add(MX35_CCM_PDR2),0,6); div!(ssi2_div_pre,"ssi2_div_pre","ssi_sel",base.add(MX35_CCM_PDR2),27,3); div!(ssi2_div_post,"ssi2_div_post","ssi2_div_pre",base.add(MX35_CCM_PDR2),8,6);
    mux!(usb_sel,"usb_sel",base.add(MX35_CCM_PDR4),9,1,STD_SEL); div!(usb_div,"usb_div","usb_sel",base.add(MX35_CCM_PDR4),22,6); div!(nfc_div,"nfc_div","ahb",base.add(MX35_CCM_PDR4),28,4);
    mux!(csi_sel,"csi_sel",base.add(MX35_CCM_PDR2),7,1,STD_SEL); div!(csi_div,"csi_div","csi_sel",base.add(MX35_CCM_PDR2),16,6);
    gate!(asrc_gate,"asrc_gate","ipg",base.add(MX35_CCM_CGR0),0); gate!(pata_gate,"pata_gate","ipg",base.add(MX35_CCM_CGR0),2); gate!(audmux_gate,"audmux_gate","ipg",base.add(MX35_CCM_CGR0),4); gate!(can1_gate,"can1_gate","ipg",base.add(MX35_CCM_CGR0),6); gate!(can2_gate,"can2_gate","ipg",base.add(MX35_CCM_CGR0),8); gate!(cspi1_gate,"cspi1_gate","ipg",base.add(MX35_CCM_CGR0),10); gate!(cspi2_gate,"cspi2_gate","ipg",base.add(MX35_CCM_CGR0),12); gate!(ect_gate,"ect_gate","ipg",base.add(MX35_CCM_CGR0),14); gate!(edio_gate,"edio_gate","ipg",base.add(MX35_CCM_CGR0),16); gate!(emi_gate,"emi_gate","ipg",base.add(MX35_CCM_CGR0),18); gate!(epit1_gate,"epit1_gate","ipg",base.add(MX35_CCM_CGR0),20); gate!(epit2_gate,"epit2_gate","ipg",base.add(MX35_CCM_CGR0),22); gate!(esai_gate,"esai_gate","ipg",base.add(MX35_CCM_CGR0),24); gate!(esdhc1_gate,"esdhc1_gate","esdhc1_div",base.add(MX35_CCM_CGR0),26); gate!(esdhc2_gate,"esdhc2_gate","esdhc2_div",base.add(MX35_CCM_CGR0),28); gate!(esdhc3_gate,"esdhc3_gate","esdhc3_div",base.add(MX35_CCM_CGR0),30);
    gate!(fec_gate,"fec_gate","ipg",base.add(MX35_CCM_CGR1),0); gate!(gpio1_gate,"gpio1_gate","ipg",base.add(MX35_CCM_CGR1),2); gate!(gpio2_gate,"gpio2_gate","ipg",base.add(MX35_CCM_CGR1),4); gate!(gpio3_gate,"gpio3_gate","ipg",base.add(MX35_CCM_CGR1),6); gate!(gpt_gate,"gpt_gate","ipg",base.add(MX35_CCM_CGR1),8); gate!(i2c1_gate,"i2c1_gate","ipg_per",base.add(MX35_CCM_CGR1),10); gate!(i2c2_gate,"i2c2_gate","ipg_per",base.add(MX35_CCM_CGR1),12); gate!(i2c3_gate,"i2c3_gate","ipg_per",base.add(MX35_CCM_CGR1),14); gate!(iomuxc_gate,"iomuxc_gate","ipg",base.add(MX35_CCM_CGR1),16); gate!(ipu_gate,"ipu_gate","hsp",base.add(MX35_CCM_CGR1),18); gate!(kpp_gate,"kpp_gate","ipg",base.add(MX35_CCM_CGR1),20); gate!(mlb_gate,"mlb_gate","ahb",base.add(MX35_CCM_CGR1),22); gate!(mshc_gate,"mshc_gate","dummy",base.add(MX35_CCM_CGR1),24); gate!(owire_gate,"owire_gate","ipg_per",base.add(MX35_CCM_CGR1),26); gate!(pwm_gate,"pwm_gate","ipg_per",base.add(MX35_CCM_CGR1),28); gate!(rngc_gate,"rngc_gate","ipg",base.add(MX35_CCM_CGR1),30);
    gate!(rtc_gate,"rtc_gate","ipg",base.add(MX35_CCM_CGR2),0); gate!(rtic_gate,"rtic_gate","ahb",base.add(MX35_CCM_CGR2),2); gate!(scc_gate,"scc_gate","ipg",base.add(MX35_CCM_CGR2),4); gate!(sdma_gate,"sdma_gate","ahb",base.add(MX35_CCM_CGR2),6); gate!(spba_gate,"spba_gate","ipg",base.add(MX35_CCM_CGR2),8); gate!(spdif_gate,"spdif_gate","spdif_div_post",base.add(MX35_CCM_CGR2),10); gate!(ssi1_gate,"ssi1_gate","ssi1_div_post",base.add(MX35_CCM_CGR2),12); gate!(ssi2_gate,"ssi2_gate","ssi2_div_post",base.add(MX35_CCM_CGR2),14); gate!(uart1_gate,"uart1_gate","uart_div",base.add(MX35_CCM_CGR2),16); gate!(uart2_gate,"uart2_gate","uart_div",base.add(MX35_CCM_CGR2),18); gate!(uart3_gate,"uart3_gate","uart_div",base.add(MX35_CCM_CGR2),20); gate!(usbotg_gate,"usbotg_gate","ahb",base.add(MX35_CCM_CGR2),22); gate!(wdog_gate,"wdog_gate","ipg",base.add(MX35_CCM_CGR2),24); gate!(max_gate,"max_gate","dummy",base.add(MX35_CCM_CGR2),26); gate!(admux_gate,"admux_gate","ipg",base.add(MX35_CCM_CGR2),30);
    gate!(csi_gate,"csi_gate","csi_div",base.add(MX35_CCM_CGR3),0); gate!(iim_gate,"iim_gate","ipg",base.add(MX35_CCM_CGR3),2); gate!(gpu2d_gate,"gpu2d_gate","ahb",base.add(MX35_CCM_CGR3),4);
    imx_check_clocks(clk.as_mut_ptr(), clk.len());
    for &i in &[spba_gate,gpio1_gate,gpio2_gate,gpio3_gate,iim_gate,emi_gate,max_gate,iomuxc_gate,scc_gate] { clk_prepare_enable(clk[i as usize]); }
    imx_register_uart_clocks();
    imx_print_silicon_rev(b"i.MX35\0".as_ptr() as _, mx35_revision());
}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
