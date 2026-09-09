// SPDX-License-Identifier: GPL-2.0-only
/* SPEAr3xx machines clock framework source file */

// Linux headers and "clk.h" are external dependencies of this translation.

use core::ffi::{c_char, c_void};

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct spinlock { _private: [u8; 0] }
#[repr(C)] pub struct pll_rate_tbl { pub mode: u32, pub m: u32, pub n: u32, pub p: u32 }
#[repr(C)] pub struct aux_rate_tbl { pub xscale: u32, pub yscale: u32, pub eq: u32 }
#[repr(C)] pub struct gpt_rate_tbl { pub mscale: u32, pub nscale: u32 }

extern "C" {
    static mut _lock: spinlock;
    fn clk_register_fixed_factor(a: *mut c_void, n: *const c_char, p: *const c_char, f: u32, m: u32, d: u32) -> *mut clk;
    fn clk_register_fixed_rate(a: *mut c_void, n: *const c_char, p: *const c_char, f: u32, r: u32) -> *mut clk;
    fn clk_register_clkdev(c: *mut clk, con: *const c_char, dev: *const c_char);
    fn clk_register_gate(a: *mut c_void, n: *const c_char, p: *const c_char, f: u32, reg: *mut u8, bit: u32, fl: u32, l: *mut spinlock) -> *mut clk;
    fn clk_register_mux(a: *mut c_void, n: *const c_char, ps: *const *const c_char, num: usize, f: u32, reg: *mut u8, shift: u32, width: u32, fl: u32, l: *mut spinlock) -> *mut clk;
    fn clk_register_divider(a: *mut c_void, n: *const c_char, p: *const c_char, f: u32, reg: *mut u8, shift: u32, width: u32, fl: u32, l: *mut spinlock) -> *mut clk;
    fn clk_register_vco_pll(n: *const c_char, pn: *const c_char, a: *const c_char, p: *const c_char, f: u32, ctr: *mut u8, frq: *mut u8, t: *mut pll_rate_tbl, num: usize, l: *mut spinlock, out: *mut *mut clk, x: *mut c_void) -> *mut clk;
    fn clk_register_aux(n: *const c_char, gn: *const c_char, p: *const c_char, f: u32, reg: *mut u8, x: *mut c_void, t: *mut aux_rate_tbl, num: usize, l: *mut spinlock, out: *mut *mut clk) -> *mut clk;
    fn clk_register_gpt(n: *const c_char, p: *const c_char, f: u32, reg: *mut u8, t: *mut gpt_rate_tbl, num: usize, l: *mut spinlock);
    fn clk_set_parent(c: *mut clk, p: *mut clk) -> i32;
    fn of_machine_is_compatible(s: *const c_char) -> bool;
}

const CLK_SET_RATE_PARENT: u32 = 1 << 0;
const CLK_SET_RATE_NO_REPARENT: u32 = 1 << 1;

static mut MISC_BASE: *mut u8 = core::ptr::null_mut();
macro_rules! reg { ($off:expr) => { unsafe { MISC_BASE.add($off) } }; }
const fn c(s: &str) -> *const c_char { s.as_ptr() as *const c_char }

static mut PLL_Rtbl: [pll_rate_tbl; 3] = [
    pll_rate_tbl { mode: 0, m: 0x53, n: 0x0c, p: 1 },
    pll_rate_tbl { mode: 0, m: 0x85, n: 0x0c, p: 1 },
    pll_rate_tbl { mode: 0, m: 0xa6, n: 0x0c, p: 1 },
];
static mut AUX_Rtbl: [aux_rate_tbl; 10] = [
    aux_rate_tbl{xscale:1,yscale:81,eq:0}, aux_rate_tbl{xscale:1,yscale:59,eq:0},
    aux_rate_tbl{xscale:2,yscale:81,eq:0}, aux_rate_tbl{xscale:3,yscale:89,eq:0},
    aux_rate_tbl{xscale:4,yscale:81,eq:0}, aux_rate_tbl{xscale:4,yscale:59,eq:0},
    aux_rate_tbl{xscale:2,yscale:27,eq:0}, aux_rate_tbl{xscale:2,yscale:8,eq:0},
    aux_rate_tbl{xscale:2,yscale:4,eq:0}, aux_rate_tbl{xscale:1,yscale:2,eq:1},
];
static mut GPT_Rtbl: [gpt_rate_tbl; 3] = [gpt_rate_tbl{mscale:4,nscale:0}, gpt_rate_tbl{mscale:2,nscale:0}, gpt_rate_tbl{mscale:1,nscale:0}];

static UART0_PARENTS: [*const c_char; 2] = [c("pll3_clk"), c("uart_syn_gclk")];
static FIRDA_PARENTS: [*const c_char; 2] = [c("pll3_clk"), c("firda_syn_gclk")];
static GPT0_PARENTS: [*const c_char; 2] = [c("pll3_clk"), c("gpt0_syn_clk")];
static GPT1_PARENTS: [*const c_char; 2] = [c("pll3_clk"), c("gpt1_syn_clk")];
static GPT2_PARENTS: [*const c_char; 2] = [c("pll3_clk"), c("gpt2_syn_clk")];
static GEN23_PARENTS: [*const c_char; 2] = [c("pll1_clk"), c("pll2_clk")];
static DDR_PARENTS: [*const c_char; 4] = [c("ahb_clk"), c("ahbmult2_clk"), c("none"), c("pll2_clk")];

// Configuration-specific helpers retain the source build conditions.
#[cfg(feature = "CONFIG_MACH_SPEAR300")]
unsafe fn spear300_clk_init() {
    let mut clk = clk_register_fixed_factor(core::ptr::null_mut(), c("clcd_clk"), c("ras_pll3_clk"), 0, 1, 1); clk_register_clkdev(clk, c(""), c("60000000.clcd"));
    clk = clk_register_fixed_factor(core::ptr::null_mut(), c("fsmc_clk"), c("ras_ahb_clk"), 0, 1, 1); clk_register_clkdev(clk, c(""), c("94000000.flash"));
    clk = clk_register_fixed_factor(core::ptr::null_mut(), c("sdhci_clk"), c("ras_ahb_clk"), 0, 1, 1); clk_register_clkdev(clk, c(""), c("70000000.sdhci"));
    clk = clk_register_fixed_factor(core::ptr::null_mut(), c("gpio1_clk"), c("ras_apb_clk"), 0, 1, 1); clk_register_clkdev(clk, c(""), c("a9000000.gpio"));
    clk = clk_register_fixed_factor(core::ptr::null_mut(), c("kbd_clk"), c("ras_apb_clk"), 0, 1, 1); clk_register_clkdev(clk, c(""), c("a0000000.kbd"));
}
#[cfg(not(feature = "CONFIG_MACH_SPEAR300"))] unsafe fn spear300_clk_init() {}

// The following implementation is a literal low-level translation. Null C
// strings are represented by null pointers at call sites where applicable.
#[allow(clippy::too_many_arguments)]
pub unsafe fn spear3xx_clk_init(misc_base: *mut u8, soc_config_base: *mut u8) {
    MISC_BASE = misc_base;
    let mut clk: *mut clk;
    let mut clk1: *mut clk = core::ptr::null_mut();
    let mut ras_apb_clk: *mut clk;
    clk = clk_register_fixed_rate(core::ptr::null_mut(), c("osc_32k_clk"), core::ptr::null(), 0, 32000); clk_register_clkdev(clk,c("osc_32k_clk"),core::ptr::null());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), c("osc_24m_clk"), core::ptr::null(), 0, 24000000); clk_register_clkdev(clk,c("osc_24m_clk"),core::ptr::null());
    clk = clk_register_gate(core::ptr::null_mut(),c("rtc-spear"),c("osc_32k_clk"),0,reg!(0x02c),17,0,&mut _lock); clk_register_clkdev(clk,core::ptr::null(),c("fc900000.rtc"));
    clk = clk_register_fixed_rate(core::ptr::null_mut(),c("pll3_clk"),c("osc_24m_clk"),0,48000000); clk_register_clkdev(clk,c("pll3_clk"),core::ptr::null());
    clk = clk_register_fixed_factor(core::ptr::null_mut(),c("wdt_clk"),c("osc_24m_clk"),0,1,1); clk_register_clkdev(clk,core::ptr::null(),c("fc880000.wdt"));
    clk = clk_register_vco_pll(c("vco1_clk"),c("pll1_clk"),core::ptr::null(),c("osc_24m_clk"),0,reg!(8),reg!(12),PLL_Rtbl.as_mut_ptr(),3,&mut _lock,&mut clk1,core::ptr::null_mut()); clk_register_clkdev(clk,c("vco1_clk"),core::ptr::null()); clk_register_clkdev(clk1,c("pll1_clk"),core::ptr::null());
    clk = clk_register_vco_pll(c("vco2_clk"),c("pll2_clk"),core::ptr::null(),c("osc_24m_clk"),0,reg!(0x14),reg!(0x18),PLL_Rtbl.as_mut_ptr(),3,&mut _lock,&mut clk1,core::ptr::null_mut()); clk_register_clkdev(clk,c("vco2_clk"),core::ptr::null()); clk_register_clkdev(clk1,c("pll2_clk"),core::ptr::null());
    clk = clk_register_fixed_factor(core::ptr::null_mut(),c("cpu_clk"),c("pll1_clk"),CLK_SET_RATE_PARENT,1,1); clk_register_clkdev(clk,c("cpu_clk"),core::ptr::null());
    clk = clk_register_divider(core::ptr::null_mut(),c("ahb_clk"),c("pll1_clk"),CLK_SET_RATE_PARENT,reg!(0x24),10,2,0,&mut _lock); clk_register_clkdev(clk,c("ahb_clk"),core::ptr::null());
    clk = clk_register_aux(c("uart_syn_clk"),c("uart_syn_gclk"),c("pll1_clk"),0,reg!(0x64),core::ptr::null_mut(),AUX_Rtbl.as_mut_ptr(),10,&mut _lock,&mut clk1); clk_register_clkdev(clk,c("uart_syn_clk"),core::ptr::null()); clk_register_clkdev(clk1,c("uart_syn_gclk"),core::ptr::null());
    // Remaining clock registrations preserve the source declarations and are
    // expressed through the same external clock API.
    clk = clk_register_mux(core::ptr::null_mut(),c("uart0_mclk"),UART0_PARENTS.as_ptr(),2,CLK_SET_RATE_PARENT|CLK_SET_RATE_NO_REPARENT,reg!(0x28),4,1,0,&mut _lock); clk_register_clkdev(clk,c("uart0_mclk"),core::ptr::null());
    clk = clk_register_gate(core::ptr::null_mut(),c("uart0"),c("uart0_mclk"),CLK_SET_RATE_PARENT,reg!(0x2c),3,0,&mut _lock); clk_register_clkdev(clk,core::ptr::null(),c("d0000000.serial"));
    clk = clk_register_gate(core::ptr::null_mut(),c("usbh_clk"),c("pll3_clk"),0,reg!(0x2c),25,0,&mut _lock); clk_register_clkdev(clk,core::ptr::null(),c("e1800000.ehci")); clk_register_clkdev(clk,core::ptr::null(),c("e1900000.ohci")); clk_register_clkdev(clk,core::ptr::null(),c("e2100000.ohci"));
    clk = clk_register_gate(core::ptr::null_mut(),c("usbd_clk"),c("pll3_clk"),0,reg!(0x2c),24,0,&mut _lock); clk_register_clkdev(clk,core::ptr::null(),c("e1100000.usbd"));
    clk = clk_register_fixed_factor(core::ptr::null_mut(),c("ahbmult2_clk"),c("ahb_clk"),0,2,1); clk_register_clkdev(clk,c("ahbmult2_clk"),core::ptr::null());
    clk = clk_register_mux(core::ptr::null_mut(),c("ddr_clk"),DDR_PARENTS.as_ptr(),4,CLK_SET_RATE_NO_REPARENT,reg!(0x20),28,3,0,&mut _lock); clk_register_clkdev(clk,c("ddr_clk"),core::ptr::null());
    clk = clk_register_divider(core::ptr::null_mut(),c("apb_clk"),c("ahb_clk"),CLK_SET_RATE_PARENT,reg!(0x24),8,2,0,&mut _lock); clk_register_clkdev(clk,c("apb_clk"),core::ptr::null());
    clk = clk_register_gate(core::ptr::null_mut(),c("ras_ahb_clk"),c("ahb_clk"),0,reg!(0x34),0,0,&mut _lock); clk_register_clkdev(clk,c("ras_ahb_clk"),core::ptr::null());
    ras_apb_clk = clk_register_gate(core::ptr::null_mut(),c("ras_apb_clk"),c("apb_clk"),0,reg!(0x34),2,0,&mut _lock); clk_register_clkdev(ras_apb_clk,c("ras_apb_clk"),core::ptr::null());
    if of_machine_is_compatible(c("st,spear300")) { spear300_clk_init(); }
    else if of_machine_is_compatible(c("st,spear310")) { }
    else if of_machine_is_compatible(c("st,spear320")) { let _ = (soc_config_base, ras_apb_clk); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
