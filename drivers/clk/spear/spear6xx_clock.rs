// SPDX-License-Identifier: GPL-2.0-only
/*
 * SPEAr6xx machines clock framework source file
 *
 * Copyright (C) 2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

// C dependencies supplied by the surrounding kernel translation unit.

static mut _lock: spinlock_t = spinlock_t::new();

const MCTR_CLK_SHIFT: u32 = 28;
const MCTR_CLK_MASK: u32 = 3;
const HCLK_RATIO_SHIFT: u32 = 10;
const HCLK_RATIO_MASK: u32 = 2;
const PCLK_RATIO_SHIFT: u32 = 8;
const PCLK_RATIO_MASK: u32 = 2;
const CLCD_CLK_SHIFT: u32 = 2;
const CLCD_CLK_MASK: u32 = 2;
const UART_CLK_SHIFT: u32 = 4;
const UART_CLK_MASK: u32 = 1;
const FIRDA_CLK_SHIFT: u32 = 5;
const FIRDA_CLK_MASK: u32 = 2;
const GPT0_CLK_SHIFT: u32 = 8;
const GPT1_CLK_SHIFT: u32 = 10;
const GPT2_CLK_SHIFT: u32 = 11;
const GPT3_CLK_SHIFT: u32 = 12;
const GPT_CLK_MASK: u32 = 1;
const UART0_CLK_ENB: u32 = 3;
const UART1_CLK_ENB: u32 = 4;
const SSP0_CLK_ENB: u32 = 5;
const SSP1_CLK_ENB: u32 = 6;
const I2C_CLK_ENB: u32 = 7;
const JPEG_CLK_ENB: u32 = 8;
const FSMC_CLK_ENB: u32 = 9;
const FIRDA_CLK_ENB: u32 = 10;
const GPT2_CLK_ENB: u32 = 11;
const GPT3_CLK_ENB: u32 = 12;
const GPIO2_CLK_ENB: u32 = 13;
const SSP2_CLK_ENB: u32 = 14;
const ADC_CLK_ENB: u32 = 15;
const GPT1_CLK_ENB: u32 = 11;
const RTC_CLK_ENB: u32 = 17;
const GPIO1_CLK_ENB: u32 = 18;
const DMA_CLK_ENB: u32 = 19;
const SMI_CLK_ENB: u32 = 21;
const CLCD_CLK_ENB: u32 = 22;
const GMAC_CLK_ENB: u32 = 23;
const USBD_CLK_ENB: u32 = 24;
const USBH0_CLK_ENB: u32 = 25;
const USBH1_CLK_ENB: u32 = 26;

static mut pll_rtbl: [pll_rate_tbl; 3] = [
    pll_rate_tbl { mode: 0, m: 0x53, n: 0x0F, p: 0x1 },
    pll_rate_tbl { mode: 0, m: 0x85, n: 0x0F, p: 0x1 },
    pll_rate_tbl { mode: 0, m: 0xA6, n: 0x0F, p: 0x1 },
];

static mut aux_rtbl: [aux_rate_tbl; 4] = [
    aux_rate_tbl { xscale: 2, yscale: 27, eq: 0 },
    aux_rate_tbl { xscale: 2, yscale: 8, eq: 0 },
    aux_rate_tbl { xscale: 2, yscale: 4, eq: 0 },
    aux_rate_tbl { xscale: 1, yscale: 2, eq: 1 },
];

static mut gpt_rtbl: [gpt_rate_tbl; 3] = [
    gpt_rate_tbl { mscale: 4, nscale: 0 },
    gpt_rate_tbl { mscale: 2, nscale: 0 },
    gpt_rate_tbl { mscale: 1, nscale: 0 },
];

static clcd_parents: [&'static CStr; 2] = [c"pll3_clk", c"clcd_syn_gclk"];
static firda_parents: [&'static CStr; 2] = [c"pll3_clk", c"firda_syn_gclk"];
static uart_parents: [&'static CStr; 2] = [c"pll3_clk", c"uart_syn_gclk"];
static gpt0_1_parents: [&'static CStr; 2] = [c"pll3_clk", c"gpt0_1_syn_clk"];
static gpt2_parents: [&'static CStr; 2] = [c"pll3_clk", c"gpt2_syn_clk"];
static gpt3_parents: [&'static CStr; 2] = [c"pll3_clk", c"gpt3_syn_clk"];
static ddr_parents: [&'static CStr; 4] = [c"ahb_clk", c"ahbmult2_clk", c"none", c"pll2_clk"];

pub unsafe fn spear6xx_clk_init(misc_base: *mut core::ffi::c_void) {
    let mut clk: *mut clk;
    let mut clk1: *mut clk;
    macro_rules! reg { ($e:expr) => { clk = $e; }; }
    macro_rules! dev { ($name:expr, $dev:expr) => { clk_register_clkdev(clk, $name, $dev); }; }
    let pll1_ctr = misc_base.add(0x008);
    let pll1_frq = misc_base.add(0x00C);
    let pll2_ctr = misc_base.add(0x014);
    let pll2_frq = misc_base.add(0x018);
    let pll_clk_cfg = misc_base.add(0x020);
    let core_clk_cfg = misc_base.add(0x024);
    let perip_clk_cfg = misc_base.add(0x028);
    let perip1_clk_enb = misc_base.add(0x02C);
    let prsc0_clk_cfg = misc_base.add(0x044);
    let prsc1_clk_cfg = misc_base.add(0x048);
    let prsc2_clk_cfg = misc_base.add(0x04C);
    let clcd_clk_synt = misc_base.add(0x05C);
    let firda_clk_synt = misc_base.add(0x060);
    let uart_clk_synt = misc_base.add(0x064);

    reg!(clk_register_fixed_rate(core::ptr::null_mut(), c"osc_32k_clk", core::ptr::null(), 0, 32000)); dev!(c"osc_32k_clk", core::ptr::null());
    reg!(clk_register_fixed_rate(core::ptr::null_mut(), c"osc_30m_clk", core::ptr::null(), 0, 30000000)); dev!(c"osc_30m_clk", core::ptr::null());
    reg!(clk_register_gate(core::ptr::null_mut(), c"rtc_spear", c"osc_32k_clk", 0, perip1_clk_enb, RTC_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"rtc-spear");
    reg!(clk_register_fixed_rate(core::ptr::null_mut(), c"pll3_clk", c"osc_24m_clk", 0, 48000000)); dev!(c"pll3_clk", core::ptr::null());
    reg!(clk_register_vco_pll(c"vco1_clk", c"pll1_clk", core::ptr::null(), c"osc_30m_clk", 0, pll1_ctr, pll1_frq, pll_rtbl.as_mut_ptr(), 3, &_lock, &mut clk1, core::ptr::null_mut())); dev!(c"vco1_clk", core::ptr::null()); clk_register_clkdev(clk1, c"pll1_clk", core::ptr::null());
    reg!(clk_register_vco_pll(c"vco2_clk", c"pll2_clk", core::ptr::null(), c"osc_30m_clk", 0, pll2_ctr, pll2_frq, pll_rtbl.as_mut_ptr(), 3, &_lock, &mut clk1, core::ptr::null_mut())); dev!(c"vco2_clk", core::ptr::null()); clk_register_clkdev(clk1, c"pll2_clk", core::ptr::null());
    reg!(clk_register_fixed_factor(core::ptr::null_mut(), c"wdt_clk", c"osc_30m_clk", 0, 1, 1)); dev!(core::ptr::null(), c"fc880000.wdt");
    reg!(clk_register_fixed_factor(core::ptr::null_mut(), c"cpu_clk", c"pll1_clk", CLK_SET_RATE_PARENT, 1, 1)); dev!(c"cpu_clk", core::ptr::null());
    reg!(clk_register_divider(core::ptr::null_mut(), c"ahb_clk", c"pll1_clk", CLK_SET_RATE_PARENT, core_clk_cfg, HCLK_RATIO_SHIFT, HCLK_RATIO_MASK, 0, &_lock)); dev!(c"ahb_clk", core::ptr::null());
    reg!(clk_register_aux(c"uart_syn_clk", c"uart_syn_gclk", c"pll1_clk", 0, uart_clk_synt, core::ptr::null(), aux_rtbl.as_mut_ptr(), 4, &_lock, &mut clk1)); dev!(c"uart_syn_clk", core::ptr::null()); clk_register_clkdev(clk1, c"uart_syn_gclk", core::ptr::null());
    reg!(clk_register_mux(core::ptr::null_mut(), c"uart_mclk", uart_parents.as_ptr(), 2, CLK_SET_RATE_NO_REPARENT, perip_clk_cfg, UART_CLK_SHIFT, UART_CLK_MASK, 0, &_lock)); dev!(c"uart_mclk", core::ptr::null());
    reg!(clk_register_gate(core::ptr::null_mut(), c"uart0", c"uart_mclk", 0, perip1_clk_enb, UART0_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"d0000000.serial");
    reg!(clk_register_gate(core::ptr::null_mut(), c"uart1", c"uart_mclk", 0, perip1_clk_enb, UART1_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"d0080000.serial");

    // Remaining registrations retain the source ordering and use the same kernel clock APIs.
    reg!(clk_register_aux(c"firda_syn_clk", c"firda_syn_gclk", c"pll1_clk", 0, firda_clk_synt, core::ptr::null(), aux_rtbl.as_mut_ptr(), 4, &_lock, &mut clk1)); dev!(c"firda_syn_clk", core::ptr::null()); clk_register_clkdev(clk1, c"firda_syn_gclk", core::ptr::null());
    reg!(clk_register_mux(core::ptr::null_mut(), c"firda_mclk", firda_parents.as_ptr(), 2, CLK_SET_RATE_NO_REPARENT, perip_clk_cfg, FIRDA_CLK_SHIFT, FIRDA_CLK_MASK, 0, &_lock)); dev!(c"firda_mclk", core::ptr::null());
    reg!(clk_register_gate(core::ptr::null_mut(), c"firda_clk", c"firda_mclk", 0, perip1_clk_enb, FIRDA_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"firda");
    reg!(clk_register_aux(c"clcd_syn_clk", c"clcd_syn_gclk", c"pll1_clk", 0, clcd_clk_synt, core::ptr::null(), aux_rtbl.as_mut_ptr(), 4, &_lock, &mut clk1)); dev!(c"clcd_syn_clk", core::ptr::null()); clk_register_clkdev(clk1, c"clcd_syn_gclk", core::ptr::null());
    reg!(clk_register_mux(core::ptr::null_mut(), c"clcd_mclk", clcd_parents.as_ptr(), 2, CLK_SET_RATE_NO_REPARENT, perip_clk_cfg, CLCD_CLK_SHIFT, CLCD_CLK_MASK, 0, &_lock)); dev!(c"clcd_mclk", core::ptr::null());
    reg!(clk_register_gate(core::ptr::null_mut(), c"clcd_clk", c"clcd_mclk", 0, perip1_clk_enb, CLCD_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"fc200000.clcd");
    reg!(clk_register_gpt(c"gpt0_1_syn_clk", c"pll1_clk", 0, prsc0_clk_cfg, gpt_rtbl.as_mut_ptr(), 3, &_lock)); dev!(c"gpt0_1_syn_clk", core::ptr::null());
    reg!(clk_register_mux(core::ptr::null_mut(), c"gpt0_mclk", gpt0_1_parents.as_ptr(), 2, CLK_SET_RATE_NO_REPARENT, perip_clk_cfg, GPT0_CLK_SHIFT, GPT_CLK_MASK, 0, &_lock)); dev!(core::ptr::null(), c"gpt0");
    reg!(clk_register_mux(core::ptr::null_mut(), c"gpt1_mclk", gpt0_1_parents.as_ptr(), 2, CLK_SET_RATE_NO_REPARENT, perip_clk_cfg, GPT1_CLK_SHIFT, GPT_CLK_MASK, 0, &_lock)); dev!(c"gpt1_mclk", core::ptr::null());
    reg!(clk_register_gate(core::ptr::null_mut(), c"gpt1_clk", c"gpt1_mclk", 0, perip1_clk_enb, GPT1_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"gpt1");
    reg!(clk_register_gpt(c"gpt2_syn_clk", c"pll1_clk", 0, prsc1_clk_cfg, gpt_rtbl.as_mut_ptr(), 3, &_lock)); dev!(c"gpt2_syn_clk", core::ptr::null());
    reg!(clk_register_mux(core::ptr::null_mut(), c"gpt2_mclk", gpt2_parents.as_ptr(), 2, CLK_SET_RATE_NO_REPARENT, perip_clk_cfg, GPT2_CLK_SHIFT, GPT_CLK_MASK, 0, &_lock)); dev!(c"gpt2_mclk", core::ptr::null());
    reg!(clk_register_gate(core::ptr::null_mut(), c"gpt2_clk", c"gpt2_mclk", 0, perip1_clk_enb, GPT2_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"gpt2");
    reg!(clk_register_gpt(c"gpt3_syn_clk", c"pll1_clk", 0, prsc2_clk_cfg, gpt_rtbl.as_mut_ptr(), 3, &_lock)); dev!(c"gpt3_syn_clk", core::ptr::null());
    reg!(clk_register_mux(core::ptr::null_mut(), c"gpt3_mclk", gpt3_parents.as_ptr(), 2, CLK_SET_RATE_NO_REPARENT, perip_clk_cfg, GPT3_CLK_SHIFT, GPT_CLK_MASK, 0, &_lock)); dev!(c"gpt3_mclk", core::ptr::null());
    reg!(clk_register_gate(core::ptr::null_mut(), c"gpt3_clk", c"gpt3_mclk", 0, perip1_clk_enb, GPT3_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"gpt3");
    reg!(clk_register_gate(core::ptr::null_mut(), c"usbh0_clk", c"pll3_clk", 0, perip1_clk_enb, USBH0_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"e1800000.ehci"); dev!(core::ptr::null(), c"e1900000.ohci");
    reg!(clk_register_gate(core::ptr::null_mut(), c"usbh1_clk", c"pll3_clk", 0, perip1_clk_enb, USBH1_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"e2000000.ehci"); dev!(core::ptr::null(), c"e2100000.ohci");
    reg!(clk_register_gate(core::ptr::null_mut(), c"usbd_clk", c"pll3_clk", 0, perip1_clk_enb, USBD_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"designware_udc");
    reg!(clk_register_fixed_factor(core::ptr::null_mut(), c"ahbmult2_clk", c"ahb_clk", 0, 2, 1)); dev!(c"ahbmult2_clk", core::ptr::null());
    reg!(clk_register_mux(core::ptr::null_mut(), c"ddr_clk", ddr_parents.as_ptr(), 4, CLK_SET_RATE_NO_REPARENT, pll_clk_cfg, MCTR_CLK_SHIFT, MCTR_CLK_MASK, 0, &_lock)); dev!(c"ddr_clk", core::ptr::null());
    reg!(clk_register_divider(core::ptr::null_mut(), c"apb_clk", c"ahb_clk", CLK_SET_RATE_PARENT, core_clk_cfg, PCLK_RATIO_SHIFT, PCLK_RATIO_MASK, 0, &_lock)); dev!(c"apb_clk", core::ptr::null());
    reg!(clk_register_gate(core::ptr::null_mut(), c"dma_clk", c"ahb_clk", 0, perip1_clk_enb, DMA_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"fc400000.dma");
    reg!(clk_register_gate(core::ptr::null_mut(), c"fsmc_clk", c"ahb_clk", 0, perip1_clk_enb, FSMC_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"d1800000.flash");
    reg!(clk_register_gate(core::ptr::null_mut(), c"gmac_clk", c"ahb_clk", 0, perip1_clk_enb, GMAC_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"e0800000.ethernet");
    reg!(clk_register_gate(core::ptr::null_mut(), c"i2c_clk", c"ahb_clk", 0, perip1_clk_enb, I2C_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"d0200000.i2c");
    reg!(clk_register_gate(core::ptr::null_mut(), c"jpeg_clk", c"ahb_clk", 0, perip1_clk_enb, JPEG_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"jpeg");
    reg!(clk_register_gate(core::ptr::null_mut(), c"smi_clk", c"ahb_clk", 0, perip1_clk_enb, SMI_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"fc000000.flash");
    reg!(clk_register_gate(core::ptr::null_mut(), c"adc_clk", c"apb_clk", 0, perip1_clk_enb, ADC_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"d820b000.adc");
    reg!(clk_register_fixed_factor(core::ptr::null_mut(), c"gpio0_clk", c"apb_clk", 0, 1, 1)); dev!(core::ptr::null(), c"f0100000.gpio");
    reg!(clk_register_gate(core::ptr::null_mut(), c"gpio1_clk", c"apb_clk", 0, perip1_clk_enb, GPIO1_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"fc980000.gpio");
    reg!(clk_register_gate(core::ptr::null_mut(), c"gpio2_clk", c"apb_clk", 0, perip1_clk_enb, GPIO2_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"d8100000.gpio");
    reg!(clk_register_gate(core::ptr::null_mut(), c"ssp0_clk", c"apb_clk", 0, perip1_clk_enb, SSP0_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"d0100000.spi");
    reg!(clk_register_gate(core::ptr::null_mut(), c"ssp1_clk", c"apb_clk", 0, perip1_clk_enb, SSP1_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"d0180000.spi");
    reg!(clk_register_gate(core::ptr::null_mut(), c"ssp2_clk", c"apb_clk", 0, perip1_clk_enb, SSP2_CLK_ENB, 0, &_lock)); dev!(core::ptr::null(), c"d8180000.spi");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
