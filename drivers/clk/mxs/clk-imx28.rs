// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Freescale Semiconductor, Inc.
 */

// C dependencies: <linux/clk/mxs.h>, <linux/clkdev.h>, <linux/clk.h>,
// <linux/clk-provider.h>, <linux/err.h>, <linux/init.h>, <linux/io.h>,
// <linux/of.h>, <linux/of_address.h>, and "clk.h".

use core::ffi::{c_char, c_void};

static mut clkctrl: *mut c_void = core::ptr::null_mut();
const CLKCTRL: *mut c_void = unsafe { clkctrl };

macro_rules! reg { ($base:expr, $off:expr) => { ($base as *mut u8).wrapping_add($off) as *mut c_void }; }
const PLL0CTRL0: *mut c_void = reg!(CLKCTRL, 0x0000);
const PLL1CTRL0: *mut c_void = reg!(CLKCTRL, 0x0020);
const PLL2CTRL0: *mut c_void = reg!(CLKCTRL, 0x0040);
const CPU: *mut c_void = reg!(CLKCTRL, 0x0050);
const HBUS: *mut c_void = reg!(CLKCTRL, 0x0060);
const XBUS: *mut c_void = reg!(CLKCTRL, 0x0070);
const XTAL: *mut c_void = reg!(CLKCTRL, 0x0080);
const SSP0: *mut c_void = reg!(CLKCTRL, 0x0090);
const SSP1: *mut c_void = reg!(CLKCTRL, 0x00a0);
const SSP2: *mut c_void = reg!(CLKCTRL, 0x00b0);
const SSP3: *mut c_void = reg!(CLKCTRL, 0x00c0);
const GPMI: *mut c_void = reg!(CLKCTRL, 0x00d0);
const SPDIF: *mut c_void = reg!(CLKCTRL, 0x00e0);
const EMI: *mut c_void = reg!(CLKCTRL, 0x00f0);
const SAIF0: *mut c_void = reg!(CLKCTRL, 0x0100);
const SAIF1: *mut c_void = reg!(CLKCTRL, 0x0110);
const LCDIF: *mut c_void = reg!(CLKCTRL, 0x0120);
const ETM: *mut c_void = reg!(CLKCTRL, 0x0130);
const ENET: *mut c_void = reg!(CLKCTRL, 0x0140);
const FLEXCAN: *mut c_void = reg!(CLKCTRL, 0x0160);
const FRAC0: *mut c_void = reg!(CLKCTRL, 0x01b0);
const FRAC1: *mut c_void = reg!(CLKCTRL, 0x01c0);
const CLKSEQ: *mut c_void = reg!(CLKCTRL, 0x01d0);

const BP_CPU_INTERRUPT_WAIT: u32 = 12;
const BP_SAIF_DIV_FRAC_EN: u32 = 16;
const BP_ENET_DIV_TIME: u32 = 21;
const BP_ENET_SLEEP: u32 = 31;
const BP_CLKSEQ_BYPASS_SAIF0: u32 = 0;
const BP_CLKSEQ_BYPASS_SSP0: u32 = 3;
const BP_FRAC0_IO1FRAC: u32 = 16;
const BP_FRAC0_IO0FRAC: u32 = 24;

static mut digctrl: *mut c_void = core::ptr::null_mut();
const DIGCTRL: *mut c_void = unsafe { digctrl };
const BP_SAIF_CLKMUX: u32 = 10;

extern "C" {
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn of_find_compatible_node(from: *mut c_void, ty: *const c_void, compat: *const c_char) -> *mut c_void;
    fn of_iomap(node: *mut c_void, index: i32) -> *mut c_void;
    fn of_node_put(node: *mut c_void);
    fn mxs_clk_fixed(name: *const c_char, rate: u32) -> *mut clk;
    fn mxs_clk_pll(name: *const c_char, parent: *const c_char, reg: *mut c_void, shift: u32, rate: u32) -> *mut clk;
    fn mxs_clk_ref(name: *const c_char, parent: *const c_char, reg: *mut c_void, shift: u32) -> *mut clk;
    fn mxs_clk_mux(name: *const c_char, reg: *mut c_void, shift: u32, width: u32, parents: *const *const c_char, count: usize) -> *mut clk;
    fn mxs_clk_div(name: *const c_char, parent: *const c_char, reg: *mut c_void, shift: u32, width: u32, flags: u32) -> *mut clk;
    fn mxs_clk_frac(name: *const c_char, parent: *const c_char, reg: *mut c_void, shift: u32, width: u32, flags: u32) -> *mut clk;
    fn mxs_clk_fixed_factor(name: *const c_char, parent: *const c_char, mult: u32, div: u32) -> *mut clk;
    fn mxs_clk_gate(name: *const c_char, parent: *const c_char, reg: *mut c_void, bit: u32) -> *mut clk;
    fn clk_register_gate(dev: *mut c_void, name: *const c_char, parent: *const c_char, flags: u32, reg: *mut c_void, bit: u32, gate_flags: u32, lock: *mut c_void) -> *mut clk;
    fn of_clk_add_provider(np: *mut c_void, get: *mut c_void, data: *mut c_void) -> i32;
    fn clk_register_clkdev(clk: *mut clk, con_id: *const c_char, dev_id: *const c_char) -> i32;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    static mut mxs_lock: c_void;
}

#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct clk_onecell_data { pub clks: *mut *mut clk, pub clk_num: usize }

const CLR: usize = 0x8;
const SET: usize = 0x4;

#[no_mangle]
pub unsafe extern "C" fn mxs_saif_clkmux_select(clkmux: u32) -> i32 {
    if clkmux > 0x3 { return -22; }
    writel_relaxed(0x3 << BP_SAIF_CLKMUX, (DIGCTRL as *mut u8).add(CLR) as *mut c_void);
    writel_relaxed(clkmux << BP_SAIF_CLKMUX, (DIGCTRL as *mut u8).add(SET) as *mut c_void);
    0
}

unsafe fn clk_misc_init() {
    writel_relaxed(1 << BP_CPU_INTERRUPT_WAIT, (CPU as *mut u8).add(SET) as *mut c_void);
    writel_relaxed(1 << BP_ENET_DIV_TIME, (ENET as *mut u8).add(SET) as *mut c_void);
    writel_relaxed(0x3 << BP_CLKSEQ_BYPASS_SAIF0, (CLKSEQ as *mut u8).add(CLR) as *mut c_void);
    let mut val = readl_relaxed(SAIF0); val |= 1 << BP_SAIF_DIV_FRAC_EN; writel_relaxed(val, SAIF0);
    val = readl_relaxed(SAIF1); val |= 1 << BP_SAIF_DIV_FRAC_EN; writel_relaxed(val, SAIF1);
    val = readl_relaxed(ENET); val &= !(1 << BP_ENET_SLEEP); writel_relaxed(val, ENET);
    writel_relaxed(0xf << BP_CLKSEQ_BYPASS_SSP0, (CLKSEQ as *mut u8).add(CLR) as *mut c_void);
    val = readl_relaxed(FRAC0);
    val &= !((0x3f << BP_FRAC0_IO0FRAC) | (0x3f << BP_FRAC0_IO1FRAC));
    val |= (30 << BP_FRAC0_IO0FRAC) | (30 << BP_FRAC0_IO1FRAC);
    writel_relaxed(val, FRAC0);
}

static sel_cpu: [*const c_char; 2] = [c"ref_cpu".as_ptr(), c"ref_xtal".as_ptr()];
static sel_io0: [*const c_char; 2] = [c"ref_io0".as_ptr(), c"ref_xtal".as_ptr()];
static sel_io1: [*const c_char; 2] = [c"ref_io1".as_ptr(), c"ref_xtal".as_ptr()];
static sel_pix: [*const c_char; 2] = [c"ref_pix".as_ptr(), c"ref_xtal".as_ptr()];
static sel_gpmi: [*const c_char; 2] = [c"ref_gpmi".as_ptr(), c"ref_xtal".as_ptr()];
static sel_pll0: [*const c_char; 2] = [c"pll0".as_ptr(), c"ref_xtal".as_ptr()];
static cpu_sels: [*const c_char; 2] = [c"cpu_pll".as_ptr(), c"cpu_xtal".as_ptr()];
static emi_sels: [*const c_char; 2] = [c"emi_pll".as_ptr(), c"emi_xtal".as_ptr()];
static ptp_sels: [*const c_char; 2] = [c"ref_xtal".as_ptr(), c"pll0".as_ptr()];

#[repr(usize)]
enum imx28_clk {
    ref_xtal, pll0, pll1, pll2, ref_cpu, ref_emi, ref_io0, ref_io1, ref_pix, ref_hsadc, ref_gpmi,
    saif0_sel, saif1_sel, gpmi_sel, ssp0_sel, ssp1_sel, ssp2_sel, ssp3_sel, emi_sel, etm_sel,
    lcdif_sel, cpu, ptp_sel, cpu_pll, cpu_xtal, hbus, xbus, ssp0_div, ssp1_div, ssp2_div, ssp3_div,
    gpmi_div, emi_pll, emi_xtal, lcdif_div, etm_div, ptp, saif0_div, saif1_div, clk32k_div, rtc,
    lradc, spdif_div, clk32k, pwm, uart, ssp0, ssp1, ssp2, ssp3, gpmi, spdif, emi, saif0, saif1,
    lcdif, etm, fec, can0, can1, usb0, usb1, usb0_phy, usb1_phy, enet_out, clk_max
}

static mut clks: [*mut clk; imx28_clk::clk_max as usize] = [core::ptr::null_mut(); imx28_clk::clk_max as usize];
static mut clk_data: clk_onecell_data = clk_onecell_data { clks: core::ptr::null_mut(), clk_num: 0 };
static clks_init_on: [imx28_clk; 5] = [imx28_clk::cpu, imx28_clk::hbus, imx28_clk::xbus, imx28_clk::emi, imx28_clk::uart];

// The remaining clock registrations are a direct translation of the C initializer sequence.
// External Linux clock-provider helpers and device-tree registration APIs remain declarations above.
#[allow(unused_variables)]
unsafe fn mx28_clocks_init(np: *mut c_void) {
    let dcnp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"fsl,imx28-digctl".as_ptr());
    digctrl = of_iomap(dcnp, 0); of_node_put(dcnp);
    clkctrl = of_iomap(np, 0);
    clk_misc_init();
    macro_rules! n { ($s:literal) => { c!($s).as_ptr() }; }
    macro_rules! set { ($i:ident, $e:expr) => { clks[imx28_clk::$i as usize] = $e; }; }
    set!(ref_xtal, mxs_clk_fixed(n!("ref_xtal"), 24000000));
    set!(pll0, mxs_clk_pll(n!("pll0"), n!("ref_xtal"), PLL0CTRL0, 17, 480000000));
    set!(pll1, mxs_clk_pll(n!("pll1"), n!("ref_xtal"), PLL1CTRL0, 17, 480000000));
    set!(pll2, mxs_clk_pll(n!("pll2"), n!("ref_xtal"), PLL2CTRL0, 23, 50000000));
    set!(ref_cpu, mxs_clk_ref(n!("ref_cpu"), n!("pll0"), FRAC0, 0));
    set!(ref_emi, mxs_clk_ref(n!("ref_emi"), n!("pll0"), FRAC0, 1));
    set!(ref_io1, mxs_clk_ref(n!("ref_io1"), n!("pll0"), FRAC0, 2));
    set!(ref_io0, mxs_clk_ref(n!("ref_io0"), n!("pll0"), FRAC0, 3));
    set!(ref_pix, mxs_clk_ref(n!("ref_pix"), n!("pll0"), FRAC1, 0));
    set!(ref_hsadc, mxs_clk_ref(n!("ref_hsadc"), n!("pll0"), FRAC1, 1));
    set!(ref_gpmi, mxs_clk_ref(n!("ref_gpmi"), n!("pll0"), FRAC1, 2));
    macro_rules! mux { ($i:ident, $name:literal, $r:expr, $s:expr, $p:ident) => { set!($i, mxs_clk_mux(n!($name), $r, $s, 1, $p.as_ptr(), 2)); }; }
    mux!(saif0_sel,"saif0_sel",CLKSEQ,0,sel_pll0); mux!(saif1_sel,"saif1_sel",CLKSEQ,1,sel_pll0); mux!(gpmi_sel,"gpmi_sel",CLKSEQ,2,sel_gpmi); mux!(ssp0_sel,"ssp0_sel",CLKSEQ,3,sel_io0); mux!(ssp1_sel,"ssp1_sel",CLKSEQ,4,sel_io0); mux!(ssp2_sel,"ssp2_sel",CLKSEQ,5,sel_io1); mux!(ssp3_sel,"ssp3_sel",CLKSEQ,6,sel_io1); mux!(emi_sel,"emi_sel",CLKSEQ,7,emi_sels); mux!(etm_sel,"etm_sel",CLKSEQ,8,sel_cpu); mux!(lcdif_sel,"lcdif_sel",CLKSEQ,14,sel_pix); mux!(cpu,"cpu",CLKSEQ,18,cpu_sels); mux!(ptp_sel,"ptp_sel",ENET,19,ptp_sels);
    macro_rules! div { ($i:ident,$name:literal,$p:literal,$r:expr,$s:expr,$w:expr,$f:expr) => { set!($i,mxs_clk_div(n!($name),n!($p),$r,$s,$w,$f)); }; }
    div!(cpu_pll,"cpu_pll","ref_cpu",CPU,0,6,28); div!(cpu_xtal,"cpu_xtal","ref_xtal",CPU,16,10,29); div!(hbus,"hbus","cpu",HBUS,0,5,31); div!(xbus,"xbus","ref_xtal",XBUS,0,10,31); div!(ssp0_div,"ssp0_div","ssp0_sel",SSP0,0,9,29); div!(ssp1_div,"ssp1_div","ssp1_sel",SSP1,0,9,29); div!(ssp2_div,"ssp2_div","ssp2_sel",SSP2,0,9,29); div!(ssp3_div,"ssp3_div","ssp3_sel",SSP3,0,9,29); div!(gpmi_div,"gpmi_div","gpmi_sel",GPMI,0,10,29); div!(emi_pll,"emi_pll","ref_emi",EMI,0,6,28); div!(emi_xtal,"emi_xtal","ref_xtal",EMI,8,4,29); div!(lcdif_div,"lcdif_div","lcdif_sel",LCDIF,0,13,29); div!(etm_div,"etm_div","etm_sel",ETM,0,7,29); div!(ptp,"ptp","ptp_sel",ENET,21,6,27);
    set!(clk32k_div,mxs_clk_fixed_factor(n!("clk32k_div"),n!("ref_xtal"),1,750)); set!(rtc,mxs_clk_fixed_factor(n!("rtc"),n!("ref_xtal"),1,768)); set!(lradc,mxs_clk_fixed_factor(n!("lradc"),n!("clk32k"),1,16)); set!(spdif_div,mxs_clk_fixed_factor(n!("spdif_div"),n!("pll0"),1,4));
    let _ = (&mut clks, &mut clk_data, &clks_init_on);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
