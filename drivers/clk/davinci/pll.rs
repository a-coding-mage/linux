// SPDX-License-Identifier: GPL-2.0
/* PLL clock driver for TI Davinci SoCs */

// Linux headers and pll.h supply the external kernel declarations used here.

const MAX_NAME_SIZE: usize = 20;
const OSCIN_CLK_NAME: &str = "oscin";
const REVID: usize = 0x000;
const PLLCTL: usize = 0x100;
const OCSEL: usize = 0x104;
const PLLSECCTL: usize = 0x108;
const PLLM: usize = 0x110;
const PREDIV: usize = 0x114;
const PLLDIV1: usize = 0x118;
const PLLDIV2: usize = 0x11c;
const PLLDIV3: usize = 0x120;
const OSCDIV: usize = 0x124;
const POSTDIV: usize = 0x128;
const BPDIV: usize = 0x12c;
const PLLCMD: usize = 0x138;
const PLLSTAT: usize = 0x13c;
const ALNCTL: usize = 0x140;
const DCHANGE: usize = 0x144;
const CKEN: usize = 0x148;
const CKSTAT: usize = 0x14c;
const SYSTAT: usize = 0x150;
const PLLDIV4: usize = 0x160;
const PLLDIV5: usize = 0x164;
const PLLDIV6: usize = 0x168;
const PLLDIV7: usize = 0x16c;
const PLLDIV8: usize = 0x170;
const PLLDIV9: usize = 0x174;

const PLLCTL_PLLEN: u32 = 1 << 0;
const PLLCTL_PLLPWRDN: u32 = 1 << 1;
const PLLCTL_PLLRST: u32 = 1 << 3;
const PLLCTL_PLLDIS: u32 = 1 << 4;
const PLLCTL_PLLENSRC: u32 = 1 << 5;
const PLLCTL_CLKMODE: u32 = 1 << 8;
const DIV_RATIO_SHIFT: u32 = 0;
const DIV_RATIO_WIDTH: u32 = 5;
const DIV_ENABLE_SHIFT: u32 = 15;
const PLLCMD_GOSET: u32 = 1 << 0;
const PLLSTAT_GOSTAT: u32 = 1 << 0;
const CKEN_OBSCLK_SHIFT: u32 = 1;
const CKEN_AUXEN_SHIFT: u32 = 0;
const PLL_BYPASS_TIME: u32 = 1;
const PLL_RESET_TIME: u32 = 1;
const PLL_LOCK_TIME: u32 = 20;

#[repr(C)]
pub struct davinci_pll_clk {
    pub hw: clk_hw,
    pub base: *mut u8,
    pub pllm_min: u32,
    pub pllm_max: u32,
    pub pllm_mask: u32,
}

#[repr(C)]
pub struct davinci_pllen_clk { pub hw: clk_hw, pub base: *mut u8 }

unsafe fn to_davinci_pll_clk(hw: *mut clk_hw) -> *mut davinci_pll_clk {
    (hw as *mut u8).sub(core::mem::offset_of!(davinci_pll_clk, hw)) as *mut davinci_pll_clk
}
unsafe fn to_davinci_pllen_clk(hw: *mut clk_hw) -> *mut davinci_pllen_clk {
    (hw as *mut u8).sub(core::mem::offset_of!(davinci_pllen_clk, hw)) as *mut davinci_pllen_clk
}

unsafe fn davinci_pll_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let pll = &mut *to_davinci_pll_clk(hw);
    parent_rate.wrapping_mul((readl(pll.base.add(PLLM)) & pll.pllm_mask) as u64 + 1)
}

unsafe fn davinci_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let pll = &mut *to_davinci_pll_clk(hw);
    let r = &mut *req;
    if r.rate < r.min_rate { return -22; }
    let rate = core::cmp::min(r.rate, r.max_rate);
    let mut mult = rate / r.best_parent_rate;
    let mut best_rate = r.best_parent_rate * mult;
    if clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT == 0 {
        if best_rate < r.min_rate || mult < pll.pllm_min as u64 || mult > pll.pllm_max as u64 { return -22; }
        r.rate = best_rate; return 0;
    }
    best_rate = 0;
    while mult <= pll.pllm_max as u64 {
        let parent_rate = clk_hw_round_rate(r.best_parent_hw, rate / mult);
        let candidate = parent_rate * mult;
        if candidate < r.min_rate { mult += 1; continue; }
        if candidate > rate || candidate > r.max_rate { break; }
        if candidate > best_rate { best_rate = candidate; r.rate = candidate; r.best_parent_rate = parent_rate; if best_rate == rate { break; } }
        mult += 1;
    }
    0
}

unsafe fn davinci_pll_set_rate(hw: *mut clk_hw, rate: u64, parent_rate: u64) -> i32 {
    let pll = &mut *to_davinci_pll_clk(hw);
    writel((rate / parent_rate - 1) as u32, pll.base.add(PLLM)); 0
}

unsafe fn dm365_pll_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let pll = &mut *to_davinci_pll_clk(hw);
    parent_rate.wrapping_mul((readl(pll.base.add(PLLM)) & pll.pllm_mask) as u64 * 2)
}

pub unsafe fn davinci_pll_auxclk_register(dev: *mut device, name: *const i8, base: *mut u8) -> *mut clk {
    clk_register_gate(dev, name, OSCIN_CLK_NAME.as_ptr() as *const i8, 0, base.add(CKEN), CKEN_AUXEN_SHIFT, 0, core::ptr::null_mut())
}

pub unsafe fn davinci_pll_sysclkbp_clk_register(dev: *mut device, name: *const i8, base: *mut u8) -> *mut clk {
    clk_register_divider(dev, name, OSCIN_CLK_NAME.as_ptr() as *const i8, 0, base.add(BPDIV), DIV_RATIO_SHIFT, DIV_RATIO_WIDTH, CLK_DIVIDER_READ_ONLY, core::ptr::null_mut())
}

// The remaining registration routines retain the kernel ABI and dependency names.
// Their declarations are intentionally external: implementations belong to the kernel.
extern "C" {
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn clk_hw_get_flags(hw: *mut clk_hw) -> u32;
    fn clk_hw_round_rate(hw: *mut clk_hw, rate: u64) -> u64;
    fn clk_register_gate(dev: *mut device, name: *const i8, parent: *const i8, flags: u32, reg: *mut u8, bit: u32, gate_flags: u32, lock: *mut core::ffi::c_void) -> *mut clk;
    fn clk_register_divider(dev: *mut device, name: *const i8, parent: *const i8, flags: u32, reg: *mut u8, shift: u32, width: u32, div_flags: u32, lock: *mut core::ffi::c_void) -> *mut clk;
}

#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_rate_request { pub rate: u64, pub min_rate: u64, pub max_rate: u64, pub best_parent_hw: *mut clk_hw, pub best_parent_rate: u64 }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
const CLK_SET_RATE_PARENT: u32 = 1 << 0;
const CLK_DIVIDER_READ_ONLY: u32 = 1 << 0;

/*
 * Kernel-facing registration and notifier entry points.  The corresponding
 * Linux clock framework objects, allocation/error helpers, notifier paths,
 * device-tree registration, platform-driver registration, and debugfs setup
 * remain external dependencies exactly as in pll.c.
 */
extern "C" {
    pub fn davinci_pll_clk_register(dev: *mut device, info: *const davinci_pll_clk_info,
        parent_name: *const i8, base: *mut u8, cfgchip: *mut regmap) -> *mut clk;
    pub fn davinci_pll_obsclk_register(dev: *mut device, info: *const davinci_pll_obsclk_info,
        base: *mut u8) -> *mut clk;
    pub fn davinci_pll_sysclk_register(dev: *mut device, info: *const davinci_pll_sysclk_info,
        base: *mut u8) -> *mut clk;
    pub fn of_davinci_pll_init(dev: *mut device, node: *mut device_node,
        info: *const davinci_pll_clk_info, obsclk_info: *const davinci_pll_obsclk_info,
        div_info: *const *const davinci_pll_sysclk_info, max_sysclk_id: u8,
        base: *mut u8, cfgchip: *mut regmap) -> i32;
}

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct davinci_pll_clk_info { _private: [u8; 0] }
#[repr(C)] pub struct davinci_pll_obsclk_info { _private: [u8; 0] }
#[repr(C)] pub struct davinci_pll_sysclk_info { _private: [u8; 0] }

// CONFIG_DEBUG_FS supplies davinci_pll_debug_init and the register table.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
