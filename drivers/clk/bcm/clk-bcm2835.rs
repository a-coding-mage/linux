// SPDX-License-Identifier: GPL-2.0+
/* Direct low-level Rust translation of clk-bcm2835.c.  Kernel-provided types
 * and functions are intentionally referenced as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

extern "C" {
    fn readl(addr: *const u32) -> u32;
    fn writel(value: u32, addr: *mut u32);
}

const CM_PASSWORD: u32 = 0x5a00_0000;
const CM_DIV_FRAC_BITS: u32 = 12;
const CM_DIV_FRAC_MASK: u32 = (1 << CM_DIV_FRAC_BITS) - 1;
const CM_ENABLE: u32 = 1 << 4;
const CM_KILL: u32 = 1 << 5;
const CM_GATE_BIT: u32 = 6;
const CM_GATE: u32 = 1 << CM_GATE_BIT;
const CM_BUSY: u32 = 1 << 7;
const CM_FRAC: u32 = 1 << 9;
const CM_SRC_MASK: u32 = 0xf;
const CM_TCNT_SRC1_SHIFT: u32 = 12;
const CM_OSCCOUNT: u32 = 0x100;
const CM_TCNTCTL: u32 = 0x0c0;
const CM_TCNTCNT: u32 = 0x0c4;
const CM_PLL_ANARST: u32 = 1 << 8;
const CM_LOCK: u32 = 0x114;
const A2W_PLL_FRAC_BITS: u32 = 20;
const A2W_PLL_FRAC_MASK: u32 = (1 << A2W_PLL_FRAC_BITS) - 1;
const A2W_PLL_CHANNEL_DISABLE: u32 = 1 << 8;
const A2W_PLL_CTRL_PWRDN: u32 = 1 << 16;
const A2W_PLL_CTRL_PRST_DISABLE: u32 = 1 << 17;
const A2W_PLL_CTRL_PDIV_MASK: u32 = 0x0000_7000;
const A2W_PLL_CTRL_PDIV_SHIFT: u32 = 12;
const A2W_PLL_CTRL_NDIV_MASK: u32 = 0x3ff;
const A2W_PLL_CTRL_NDIV_SHIFT: u32 = 0;
const A2W_PLL_DIV_BITS: u32 = 8;
const LOCK_TIMEOUT_NS: u64 = 100_000_000;
const BCM2835_MAX_FB_RATE: u32 = 1_750_000_000;
const SOC_BCM2835: u32 = 1;
const SOC_BCM2711: u32 = 2;
const SOC_ALL: u32 = SOC_BCM2835 | SOC_BCM2711;

#[repr(C)]
pub struct bcm2835_cprman {
    pub dev: *mut core::ffi::c_void,
    pub regs: *mut u32,
    pub regs_lock: *mut core::ffi::c_void,
    pub soc: u32,
}

#[repr(C)]
pub struct bcm2835_pll_ana_bits { pub mask0:u32, pub set0:u32, pub mask1:u32, pub set1:u32, pub mask3:u32, pub set3:u32, pub fb_prediv_mask:u32 }
#[repr(C)]
pub struct bcm2835_pll_data { pub name:*const i8, pub cm_ctrl_reg:u32, pub a2w_ctrl_reg:u32, pub frac_reg:u32, pub ana_reg_base:u32, pub reference_enable_mask:u32, pub lock_mask:u32, pub flags:u32, pub ana:*const bcm2835_pll_ana_bits, pub min_rate:usize, pub max_rate:usize, pub max_fb_rate:usize }
#[repr(C)]
pub struct bcm2835_pll { pub hw:*mut core::ffi::c_void, pub cprman:*mut bcm2835_cprman, pub data:*const bcm2835_pll_data }

#[inline] unsafe fn cprman_write(c: *mut bcm2835_cprman, reg:u32, val:u32) { writel(CM_PASSWORD | val, (*c).regs.add((reg / 4) as usize)); }
#[inline] unsafe fn cprman_read(c: *mut bcm2835_cprman, reg:u32) -> u32 { readl((*c).regs.add((reg / 4) as usize)) }

unsafe fn bcm2835_pll_is_on(pll:*mut bcm2835_pll) -> i32 { (cprman_read((*pll).cprman, (*(*pll).data).a2w_ctrl_reg) & A2W_PLL_CTRL_PRST_DISABLE) as i32 }

unsafe fn bcm2835_pll_get_prediv_mask(c:*mut bcm2835_cprman, d:*const bcm2835_pll_data) -> u32 {
    if (*c).soc & SOC_BCM2711 != 0 { 0 } else { (*(*d).ana).fb_prediv_mask }
}

unsafe fn bcm2835_pll_choose_ndiv_and_fdiv(rate:usize, parent:usize, ndiv:&mut u32, fdiv:&mut u32) {
    let div = ((rate as u128) << A2W_PLL_FRAC_BITS) / parent as u128;
    *ndiv = (div >> A2W_PLL_FRAC_BITS) as u32; *fdiv = (div & A2W_PLL_FRAC_MASK as u128) as u32;
}

unsafe fn bcm2835_pll_rate_from_divisors(parent:usize, ndiv:u32, fdiv:u32, pdiv:u32) -> usize {
    if pdiv == 0 { return 0; }
    (((parent as u128 * (((ndiv as u128) << A2W_PLL_FRAC_BITS) + fdiv as u128)) / pdiv as u128) >> A2W_PLL_FRAC_BITS) as usize
}

/* The remaining registration tables and driver callbacks retain the C
 * driver's externally supplied clock-framework interfaces. */
extern "C" {
    fn bcm2835_clk_probe(pdev:*mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
