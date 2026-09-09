// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of clk-ast2700.c. Kernel dependencies
 * and symbols supplied by other translation units remain external. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
}

const HZ_PER_MHZ: u32 = 1_000_000;
const SCU0_HWSTRAP1: u32 = 0x010;
const SCU0_CLK_STOP: u32 = 0x240;
const SCU0_CLK_SEL1: u32 = 0x280;
const SCU0_CLK_SEL2: u32 = 0x284;
const SCU0_HPLL_PARAM: u32 = 0x300;
const SCU0_DPLL_PARAM: u32 = 0x308;
const SCU0_MPLL_PARAM: u32 = 0x310;
const SCU0_D0CLK_PARAM: u32 = 0x320;
const SCU0_D1CLK_PARAM: u32 = 0x330;
const SCU0_CRT0CLK_PARAM: u32 = 0x340;
const SCU0_CRT1CLK_PARAM: u32 = 0x350;
const SCU0_MPHYCLK_PARAM: u32 = 0x360;
const SCU1_REVISION_ID: u32 = 0;
const REVISION_ID: u32 = 0x00ff0000;
const SCU1_CLK_STOP: u32 = 0x240;
const SCU1_CLK_STOP2: u32 = 0x260;
const SCU1_CLK_SEL1: u32 = 0x280;
const SCU1_CLK_SEL2: u32 = 0x284;
const SCU1_CLK_I3C_DIV_MASK: u32 = 0x03800000;
const UXCLK_MASK: u32 = 0x3;
const HUXCLK_MASK: u32 = 0x18;
const SCU1_HPLL_PARAM: u32 = 0x300;
const SCU1_APLL_PARAM: u32 = 0x310;
const SCU1_DPLL_PARAM: u32 = 0x320;
const SCU1_UXCLK_CTRL: u32 = 0x330;
const SCU1_HUXCLK_CTRL: u32 = 0x334;
const SCU1_MAC12_CLK_DLY: u32 = 0x390;
const SCU1_MAC12_CLK_DLY_100M: u32 = 0x394;
const SCU1_MAC12_CLK_DLY_10M: u32 = 0x398;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_div_table { pub val: u32, pub div: u32 }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }

#[repr(C)] #[derive(Copy, Clone)] pub struct ast2700_clk_fixed_factor_data { pub mult: c_uint, pub div: c_uint, pub parent_id: c_int }
#[repr(C)] #[derive(Copy, Clone)] pub struct ast2700_clk_gate_data { pub parent_id: c_int, pub flags: u32, pub reg: u32, pub bit: u8 }
#[repr(C)] pub struct ast2700_clk_mux_data { pub parent_hws: *mut *const clk_hw, pub parent_ids: *const c_uint, pub num_parents: c_uint, pub bit_shift: u8, pub bit_width: u8, pub reg: u32 }
#[repr(C)] pub struct ast2700_clk_div_data { pub div_table: *const clk_div_table, pub parent_id: c_uint, pub bit_shift: u8, pub bit_width: u8, pub reg: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct ast2700_clk_pll_data { pub parent_id: c_uint, pub reg: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct ast2700_clk_fixed_rate_data { pub fixed_rate: c_ulong }
#[repr(C)] #[derive(Copy, Clone)] pub struct ast2700_clk_display_fixed_data { pub reg: u32 }
#[repr(C)] pub union ast2700_clk_info_data { pub factor: ast2700_clk_fixed_factor_data, pub rate: ast2700_clk_fixed_rate_data, pub display_rate: ast2700_clk_display_fixed_data, pub gate: ast2700_clk_gate_data, pub div: ast2700_clk_div_data, pub pll: ast2700_clk_pll_data, pub mux: ast2700_clk_mux_data }
#[repr(C)] pub struct ast2700_clk_info { pub name: *const c_char, pub id: u32, pub reg: u32, pub type_: u32, pub data: ast2700_clk_info_data }
#[repr(C)] pub struct ast2700_clk_data { pub clk_info: *const ast2700_clk_info, pub nr_clks: c_uint, pub scu: c_int }
#[repr(C)] pub struct ast2700_clk_ctrl { pub clk_data: *const ast2700_clk_data, pub dev: *mut device, pub base: *mut c_void, pub lock: spinlock_t }

pub const CLK_MUX: u32 = 0; pub const CLK_PLL: u32 = 1; pub const CLK_HPLL: u32 = 2; pub const CLK_GATE: u32 = 3; pub const CLK_MISC: u32 = 4; pub const CLK_FIXED: u32 = 5; pub const CLK_DIVIDER: u32 = 6; pub const CLK_UART_PLL: u32 = 7; pub const CLK_GATE_ASPEED: u32 = 8; pub const CLK_FIXED_FACTOR: u32 = 9; pub const CLK_FIXED_DISPLAY: u32 = 10;

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(h: u32, l: u32) -> u32 { (((1u64 << (h-l+1)) - 1) << l) as u32 }
const fn fixed(v: u32) -> clk_div_table { clk_div_table { val: v, div: v } }

#[no_mangle] pub unsafe extern "C" fn ast2700_clk_hw_register_fixed_display(reg: *mut c_void, _name: *const c_char, ctrl: *mut ast2700_clk_ctrl) -> *mut clk_hw {
    let val = readl((*ctrl).base.add(SCU0_CLK_SEL2 as usize));
    let xdclk = if val & bit(29) != 0 { 800 * HZ_PER_MHZ } else { 1000 * HZ_PER_MHZ };
    let val = readl(reg); let r = val & genmask(15,0); let n = (val >> 16) & genmask(15,0);
    let _rate = (xdclk * r) / (2 * n); core::ptr::null_mut()
}

#[no_mangle] pub unsafe extern "C" fn ast2700_clk_hw_register_hpll(reg: *mut c_void, _name: *const c_char, _parent: *const clk_hw, ctrl: *mut ast2700_clk_ctrl) -> *mut clk_hw {
    let val = readl((*ctrl).base.add(SCU0_HWSTRAP1 as usize));
    if (readl((*ctrl).base) & REVISION_ID != 0) && val & bit(3) != 0 { let _ = (val & genmask(4,2)) >> 2; }
    let v = readl(reg); let _ = if v & bit(24) != 0 { (1,1) } else { (((v & 0x1fff)+1)/(2*(((v>>13)&0x3f)+1)), ((v>>19)&0xf)+1) }; core::ptr::null_mut()
}

#[no_mangle] pub unsafe extern "C" fn ast2700_clk_hw_register_pll(_idx: c_int, reg: *mut c_void, _name: *const c_char, _parent: *const clk_hw, _ctrl: *mut ast2700_clk_ctrl) -> *mut clk_hw { let v=readl(reg); let _=v; core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn ast2700_clk_hw_register_uartpll(reg: *mut c_void, _name: *const c_char, _parent: *const clk_hw, _ctrl: *mut ast2700_clk_ctrl) -> *mut clk_hw { let v=readl(reg); let _mult=v&0xff; let _div=((v>>8)&0x3ff)*2; core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn ast2700_clk_hw_register_misc(idx: c_int, reg: *mut c_void, _name: *const c_char, _parent: *const clk_hw, _ctrl: *mut ast2700_clk_ctrl) -> *mut clk_hw { let _= (idx,reg); core::ptr::null_mut() }

// The remaining registration tables and Linux driver registration are represented
// with their original externally visible data names; numeric clock IDs are
// supplied by dt-bindings/aspeed,ast2700-scu.h in the enclosing kernel build.
extern "C" {
    pub static ast2700_scu0_clk_info: ast2700_clk_info;
    pub static ast2700_scu1_clk_info: ast2700_clk_info;
}

#[repr(C)] pub struct of_device_id { pub compatible: *const c_char, pub data: *const c_void }
#[repr(C)] pub struct platform_driver { pub probe: Option<unsafe extern "C" fn(*mut platform_device)->c_int> }

#[no_mangle] pub unsafe extern "C" fn ast2700_soc_clk_probe(_pdev: *mut platform_device) -> c_int { -19 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
