// SPDX-License-Identifier: GPL-2.0-only
/*
 * Clk driver for NXP LPC18xx/LPC43xx Clock Generation Unit (CGU)
 *
 * Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
 */

// Linux kernel dependencies are supplied externally.

const LPC18XX_CGU_XTAL_OSC_CTRL: usize = 0x018;
const LPC18XX_CGU_PLL0USB_STAT: usize = 0x01c;
const LPC18XX_CGU_PLL0USB_CTRL: usize = 0x020;
const LPC18XX_CGU_PLL0USB_MDIV: usize = 0x024;
const LPC18XX_CGU_PLL0USB_NP_DIV: usize = 0x028;
const LPC18XX_CGU_PLL0AUDIO_STAT: usize = 0x02c;
const LPC18XX_CGU_PLL0AUDIO_CTRL: usize = 0x030;
const LPC18XX_CGU_PLL0AUDIO_MDIV: usize = 0x034;
const LPC18XX_CGU_PLL0AUDIO_NP_DIV: usize = 0x038;
const LPC18XX_CGU_PLL0AUDIO_FRAC: usize = 0x03c;
const LPC18XX_CGU_PLL1_STAT: usize = 0x040;
const LPC18XX_CGU_PLL1_CTRL: usize = 0x044;
const LPC18XX_PLL1_CTRL_FBSEL: u32 = 1 << 6;
const LPC18XX_PLL1_CTRL_DIRECT: u32 = 1 << 7;
const LPC18XX_CGU_PLL_CTRL_OFFSET: usize = 0x4;

const LPC18XX_PLL0_STAT_LOCK: u32 = 1 << 0;
const LPC18XX_PLL0_CTRL_PD: u32 = 1 << 0;
const LPC18XX_PLL0_CTRL_BYPASS: u32 = 1 << 1;
const LPC18XX_PLL0_CTRL_DIRECTI: u32 = 1 << 2;
const LPC18XX_PLL0_CTRL_DIRECTO: u32 = 1 << 3;
const LPC18XX_PLL0_CTRL_CLKEN: u32 = 1 << 4;
const LPC18XX_PLL0_MDIV_MDEC_MASK: u32 = 0x1ffff;
const LPC18XX_PLL0_MDIV_SELP_SHIFT: u32 = 17;
const LPC18XX_PLL0_MDIV_SELI_SHIFT: u32 = 22;
const LPC18XX_PLL0_MSEL_MAX: u32 = 1 << 15;
const LPC18XX_PLL0_NP_DIVS_1: u32 = 0x00302062;

#[repr(usize)]
enum ClockSource {
    CLK_SRC_OSC32,
    CLK_SRC_IRC,
    CLK_SRC_ENET_RX_CLK,
    CLK_SRC_ENET_TX_CLK,
    CLK_SRC_GP_CLKIN,
    CLK_SRC_RESERVED1,
    CLK_SRC_OSC,
    CLK_SRC_PLL0USB,
    CLK_SRC_PLL0AUDIO,
    CLK_SRC_PLL1,
    CLK_SRC_RESERVED2,
    CLK_SRC_RESERVED3,
    CLK_SRC_IDIVA,
    CLK_SRC_IDIVB,
    CLK_SRC_IDIVC,
    CLK_SRC_IDIVD,
    CLK_SRC_IDIVE,
    CLK_SRC_MAX,
}

static clk_src_names: [&str; ClockSource::CLK_SRC_MAX as usize] = [
    "osc32", "irc", "enet_rx_clk", "enet_tx_clk", "gp_clkin", "", "osc",
    "pll0usb", "pll0audio", "pll1", "", "", "idiva", "idivb", "idivc",
    "idivd", "idive",
];

static clk_base_names: [&str; BASE_CLK_MAX as usize] = [
    "base_safe_clk", "base_usb0_clk", "base_periph_clk", "base_usb1_clk",
    "base_cpu_clk", "base_spifi_clk", "base_spi_clk", "base_phy_rx_clk",
    "base_phy_tx_clk", "base_apb1_clk", "base_apb3_clk", "base_lcd_clk",
    "base_adchs_clk", "base_sdio_clk", "base_ssp0_clk", "base_ssp1_clk",
    "base_uart0_clk", "base_uart1_clk", "base_uart2_clk", "base_uart3_clk",
    "base_out_clk", "base_audio_clk", "base_cgu_out0_clk", "base_cgu_out1_clk",
];

static mut lpc18xx_cgu_pll0_src_ids: [u32; 12] = [0, 1, 2, 3, 4, 6, 9, 12, 13, 14, 15, 16];
static mut lpc18xx_cgu_pll1_src_ids: [u32; 13] = [0, 1, 2, 3, 4, 6, 7, 8, 12, 13, 14, 15, 16];
static mut lpc18xx_cgu_idiva_src_ids: [u32; 9] = [0, 1, 2, 3, 4, 6, 7, 8, 9];
static mut lpc18xx_cgu_idivbcde_src_ids: [u32; 9] = [0, 1, 2, 3, 4, 6, 8, 9, 12];
static mut lpc18xx_cgu_base_irc_src_ids: [u32; 1] = [1];
static mut lpc18xx_cgu_base_usb0_src_ids: [u32; 1] = [7];
static mut lpc18xx_cgu_base_common_src_ids: [u32; 13] = [0, 1, 2, 3, 4, 6, 8, 9, 12, 13, 14, 15, 16];
static mut lpc18xx_cgu_base_all_src_ids: [u32; 14] = [0, 1, 2, 3, 4, 6, 7, 8, 9, 12, 13, 14, 15, 16];

#[repr(C)]
struct lpc18xx_cgu_src_clk_div { clk_id: u8, n_parents: u8, div: clk_divider, mux: clk_mux, gate: clk_gate }

macro_rules! LPC1XX_CGU_SRC_CLK_DIV { ($id:expr, $width:expr, $table:ident) => {
    lpc18xx_cgu_src_clk_div { clk_id: $id, n_parents: lpc18xx_cgu_$table.len() as u8,
        div: clk_divider { shift: 2, width: $width, ..Default::default() },
        mux: clk_mux { mask: 0x1f, shift: 24, table: lpc18xx_cgu_$table.as_ptr(), ..Default::default() },
        gate: clk_gate { bit_idx: 0, flags: CLK_GATE_SET_TO_DISABLE, ..Default::default() } }
}; }

static mut lpc18xx_cgu_src_clk_divs: [lpc18xx_cgu_src_clk_div; 5] = [
    LPC1XX_CGU_SRC_CLK_DIV!(12, 2, idiva_src_ids),
    LPC1XX_CGU_SRC_CLK_DIV!(13, 4, idivbcde_src_ids),
    LPC1XX_CGU_SRC_CLK_DIV!(14, 4, idivbcde_src_ids),
    LPC1XX_CGU_SRC_CLK_DIV!(15, 4, idivbcde_src_ids),
    LPC1XX_CGU_SRC_CLK_DIV!(16, 8, idivbcde_src_ids),
];

#[repr(C)] struct lpc18xx_cgu_base_clk { clk_id: u8, n_parents: u8, mux: clk_mux, gate: clk_gate }
#[repr(C)] struct lpc18xx_pll { hw: clk_hw, reg: *mut core::ffi::c_void, flags: u8 }
#[repr(C)] struct lpc18xx_cgu_pll_clk { clk_id: u8, n_parents: u8, reg_offset: u8, mux: clk_mux, gate: clk_gate, pll: lpc18xx_pll, pll_ops: *const clk_ops }

/* PLL0 uses a special register value encoding. The compute functions below
 * are taken or derived from the LPC1850 user manual (section 12.6.3.3).
 */
unsafe fn lpc18xx_pll0_mdec2msel(mut x: u32) -> u32 {
    match x { 0x18003 => 1, 0x10003 => 2, _ => {
        let mut i = LPC18XX_PLL0_MSEL_MAX + 1;
        while x != 0x4000 && i > 0 { i -= 1; x = ((x ^ (x >> 14)) & 1) | ((x << 1) & 0x7fff); } i
    }}
}
unsafe fn lpc18xx_pll0_msel2mdec(msel: u32) -> u32 {
    match msel { 0 => 0, 1 => 0x18003, 2 => 0x10003, _ => { let mut x=0x4000; let mut i=msel; while i<=LPC18XX_PLL0_MSEL_MAX { x=(((x^(x>>1))&1)<<14)|((x>>1)&0xffff); i+=1; } x }}
}
fn lpc18xx_pll0_msel2seli(msel:u32)->u32 { if msel>16384 {1} else if msel>8192 {2} else if msel>2048 {4} else if msel>=501 {8} else if msel>=60 { let t=1024/(msel+9); if 1024==t*(msel+9) {t*4} else {(t+1)*4} } else {(msel&0x3c)+4} }
fn lpc18xx_pll0_msel2selp(msel:u32)->u32 { if msel<60 {(msel>>1)+1} else {31} }

unsafe fn lpc18xx_pll0_recalc_rate(hw:*mut clk_hw, parent_rate:usize)->usize { let pll=&*(hw as *mut lpc18xx_pll); let ctrl=readl(pll.reg.add(LPC18XX_CGU_PLL0USB_CTRL)); let mdiv=readl(pll.reg.add(LPC18XX_CGU_PLL0USB_MDIV)); let npdiv=readl(pll.reg.add(LPC18XX_CGU_PLL0USB_NP_DIV)); if ctrl&LPC18XX_PLL0_CTRL_BYPASS!=0{return parent_rate} if npdiv!=LPC18XX_PLL0_NP_DIVS_1 {pr_warn!("{}: pre/post dividers not supported", stringify!(lpc18xx_pll0_recalc_rate));return 0} let msel=lpc18xx_pll0_mdec2msel(mdiv&LPC18XX_PLL0_MDIV_MDEC_MASK); if msel!=0 {2*msel as usize*parent_rate} else {pr_warn!("{}: unable to calculate rate", stringify!(lpc18xx_pll0_recalc_rate));0} }

// The remaining registration wrappers retain the kernel's external clock API.
extern "C" {
    fn lpc18xx_cgu_register_source_clks(np:*mut device_node, base:*mut core::ffi::c_void);
    fn lpc18xx_cgu_register_base_clks(base:*mut core::ffi::c_void);
}

#[allow(non_snake_case)]
unsafe fn lpc18xx_cgu_init(np:*mut device_node) { let reg_base=of_iomap(np,0); if reg_base.is_null(){pr_warn!("lpc18xx_cgu_init: failed to map address range");return;} lpc18xx_cgu_register_source_clks(np,reg_base); lpc18xx_cgu_register_base_clks(reg_base); of_clk_add_provider(np,of_clk_src_onecell_get,&mut clk_base_data); }

/* Direct translations of the remaining kernel-facing operations. */
unsafe fn lpc18xx_pll0_determine_rate(_hw:*mut clk_hw, req:*mut clk_rate_request)->i32 { let r=&mut *req; if r.best_parent_rate<r.rate { return -22; } let m=(r.best_parent_rate+r.rate*2-1)/(r.rate*2); if m==0 || m>LPC18XX_PLL0_MSEL_MAX as usize { return -22; } r.rate=2*r.best_parent_rate*m; 0 }
unsafe fn lpc18xx_pll0_set_rate(hw:*mut clk_hw, rate:usize, parent_rate:usize)->i32 { let pll=&*(hw as *mut lpc18xx_pll); if parent_rate<rate{return -22;} let m=(parent_rate+rate*2-1)/(rate*2); if m==0||m>LPC18XX_PLL0_MSEL_MAX as usize{return -22;} let mut m=lpc18xx_pll0_msel2mdec(m as u32); m|=lpc18xx_pll0_msel2selp(m)<<LPC18XX_PLL0_MDIV_SELP_SHIFT; m|=lpc18xx_pll0_msel2seli(m)<<LPC18XX_PLL0_MDIV_SELI_SHIFT; let mut ctrl=readl(pll.reg.add(LPC18XX_CGU_PLL0USB_CTRL)); ctrl|=LPC18XX_PLL0_CTRL_PD; ctrl&=!(LPC18XX_PLL0_CTRL_BYPASS|LPC18XX_PLL0_CTRL_DIRECTI|LPC18XX_PLL0_CTRL_DIRECTO|LPC18XX_PLL0_CTRL_CLKEN); writel(ctrl,pll.reg.add(LPC18XX_CGU_PLL0USB_CTRL)); writel(m,pll.reg.add(LPC18XX_CGU_PLL0USB_MDIV)); writel(LPC18XX_PLL0_NP_DIVS_1,pll.reg.add(LPC18XX_CGU_PLL0USB_NP_DIV)); ctrl&=!LPC18XX_PLL0_CTRL_PD; writel(ctrl,pll.reg.add(LPC18XX_CGU_PLL0USB_CTRL)); let mut retry=3; while retry>=0 { udelay(10); if readl(pll.reg.add(LPC18XX_CGU_PLL0USB_STAT))&LPC18XX_PLL0_STAT_LOCK!=0 {ctrl|=LPC18XX_PLL0_CTRL_CLKEN;writel(ctrl,pll.reg.add(LPC18XX_CGU_PLL0USB_CTRL));return 0;} retry-=1;} -22 }
unsafe fn lpc18xx_pll1_recalc_rate(hw:*mut clk_hw,parent_rate:usize)->usize { let pll=&*(hw as *mut lpc18xx_pll); let c=readl(pll.reg.add(LPC18XX_CGU_PLL1_CTRL)); let m=((c>>16)&0xff)+1; let n=((c>>12)&3)+1; if c&(LPC18XX_PLL1_CTRL_DIRECT|LPC18XX_PLL1_CTRL_FBSEL)!=0 {m as usize*(parent_rate/n as usize)} else {let p=1usize<<((c>>8)&3); (m as usize/(2*p))*(parent_rate/n as usize)} }

/* External kernel declarations intentionally remain unresolved here. */
extern "C" { fn readl(addr:*mut core::ffi::c_void)->u32; fn writel(v:u32,addr:*mut core::ffi::c_void); fn udelay(us:u32); fn of_iomap(np:*mut device_node,index:i32)->*mut core::ffi::c_void; fn of_clk_add_provider(np:*mut device_node,get:unsafe extern "C" fn(),data:*mut core::ffi::c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
