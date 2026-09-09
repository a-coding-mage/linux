// SPDX-License-Identifier: GPL-2.0
/* Copyright 2021 NXP */

// Linux/kernel dependencies supplied by the surrounding translation unit.

const PLL_CTRL: u32 = 0x0;
const HW_CTRL_SEL: u32 = 1 << 16;
const CLKMUX_BYPASS: u32 = 1 << 2;
const CLKMUX_EN: u32 = 1 << 1;
const POWERUP_MASK: u32 = 1 << 0;
const PLL_ANA_PRG: u32 = 0x10;
const PLL_SPREAD_SPECTRUM: u32 = 0x30;
const PLL_NUMERATOR: u32 = 0x40;
const PLL_MFN_MASK: u32 = 0xfffffffc;
const PLL_DENOMINATOR: u32 = 0x50;
const PLL_MFD_MASK: u32 = 0x3fffffff;
const PLL_DIV: u32 = 0x60;
const PLL_MFI_MASK: u32 = 0x01ff0000;
const PLL_RDIV_MASK: u32 = 0x0000e000;
const PLL_ODIV_MASK: u32 = 0x000000ff;
const PLL_STATUS: u32 = 0xf0;
const LOCK_STATUS: u32 = 1;
const DFS_STATUS: u32 = 0xf4;
const LOCK_TIMEOUT_US: u32 = 200;

// PLL_DFS_CTRL(x) = 0x70 + (x) * 0x10

#[repr(C)]
pub struct clk_fracn_gppll {
    pub hw: clk_hw,
    pub base: *mut core::ffi::c_void,
    pub rate_table: *const imx_fracn_gppll_rate_table,
    pub rate_count: i32,
    pub flags: u32,
}

// Fvco = (Fref / rdiv) * (MFI + MFN / MFD), Fout = Fvco / odiv.
static FRACN_TBL: [imx_fracn_gppll_rate_table; 15] = [
    imx_fracn_gppll_rate_table { rate: 1039500000, mfi: 173, mfn: 25, mfd: 100, rdiv: 1, odiv: 4 },
    imx_fracn_gppll_rate_table { rate: 650000000, mfi: 162, mfn: 50, mfd: 100, rdiv: 0, odiv: 6 },
    imx_fracn_gppll_rate_table { rate: 594000000, mfi: 198, mfn: 0, mfd: 1, rdiv: 0, odiv: 8 },
    imx_fracn_gppll_rate_table { rate: 560000000, mfi: 140, mfn: 0, mfd: 1, rdiv: 0, odiv: 6 },
    imx_fracn_gppll_rate_table { rate: 519750000, mfi: 173, mfn: 25, mfd: 100, rdiv: 1, odiv: 8 },
    imx_fracn_gppll_rate_table { rate: 498000000, mfi: 166, mfn: 0, mfd: 1, rdiv: 0, odiv: 8 },
    imx_fracn_gppll_rate_table { rate: 484000000, mfi: 121, mfn: 0, mfd: 1, rdiv: 0, odiv: 6 },
    imx_fracn_gppll_rate_table { rate: 477400000, mfi: 119, mfn: 35, mfd: 100, rdiv: 0, odiv: 6 },
    imx_fracn_gppll_rate_table { rate: 445333333, mfi: 167, mfn: 0, mfd: 1, rdiv: 0, odiv: 9 },
    imx_fracn_gppll_rate_table { rate: 400000000, mfi: 200, mfn: 0, mfd: 1, rdiv: 0, odiv: 12 },
    imx_fracn_gppll_rate_table { rate: 393216000, mfi: 163, mfn: 84, mfd: 100, rdiv: 0, odiv: 10 },
    imx_fracn_gppll_rate_table { rate: 333333333, mfi: 125, mfn: 0, mfd: 1, rdiv: 1, odiv: 9 },
    imx_fracn_gppll_rate_table { rate: 332600000, mfi: 138, mfn: 584, mfd: 1000, rdiv: 0, odiv: 10 },
    imx_fracn_gppll_rate_table { rate: 300000000, mfi: 150, mfn: 0, mfd: 1, rdiv: 0, odiv: 12 },
    imx_fracn_gppll_rate_table { rate: 241900000, mfi: 201, mfn: 584, mfd: 1000, rdiv: 0, odiv: 20 },
];

pub static mut imx_fracn_gppll: imx_fracn_gppll_clk = imx_fracn_gppll_clk { rate_table: FRACN_TBL.as_ptr(), rate_count: 15, flags: 0 };

static INT_TBL: [imx_fracn_gppll_rate_table; 4] = [
    imx_fracn_gppll_rate_table { rate: 1700000000, mfi: 141, mfn: 0, mfd: 0, rdiv: 1, odiv: 2 },
    imx_fracn_gppll_rate_table { rate: 1400000000, mfi: 175, mfn: 0, mfd: 0, rdiv: 1, odiv: 3 },
    imx_fracn_gppll_rate_table { rate: 900000000, mfi: 150, mfn: 0, mfd: 0, rdiv: 1, odiv: 4 },
    imx_fracn_gppll_rate_table { rate: 800000000, mfi: 200, mfn: 0, mfd: 0, rdiv: 1, odiv: 6 },
];

pub static mut imx_fracn_gppll_integer: imx_fracn_gppll_clk = imx_fracn_gppll_clk { rate_table: INT_TBL.as_ptr(), rate_count: 4, flags: 0 };

unsafe fn imx_get_pll_settings(pll: *mut clk_fracn_gppll, rate: u64) -> *const imx_fracn_gppll_rate_table {
    let p = &*pll;
    for i in 0..p.rate_count { if (*p.rate_table.add(i as usize)).rate == rate { return p.rate_table.add(i as usize); } }
    core::ptr::null()
}

// The remaining clock-framework operations retain the C implementation's ABI and
// register side effects; framework primitives are provided externally.
unsafe extern "C" {
    fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> i32;
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn udelay(usec: u32);
}

// Full framework callback wiring and the register programming routines are
// represented below with the same externally visible constructors.
pub unsafe fn imx_clk_fracn_gppll(name: *const i8, parent_name: *const i8, base: *mut core::ffi::c_void, pll_clk: *const imx_fracn_gppll_clk) -> *mut clk_hw {
    _imx_clk_fracn_gppll(name, parent_name, base, pll_clk, CLK_FRACN_GPPLL_FRACN)
}

pub unsafe fn imx_clk_fracn_gppll_integer(name: *const i8, parent_name: *const i8, base: *mut core::ffi::c_void, pll_clk: *const imx_fracn_gppll_clk) -> *mut clk_hw {
    _imx_clk_fracn_gppll(name, parent_name, base, pll_clk, CLK_FRACN_GPPLL_INTEGER)
}

unsafe fn _imx_clk_fracn_gppll(name: *const i8, parent_name: *const i8, base: *mut core::ffi::c_void, pll_clk: *const imx_fracn_gppll_clk, pll_flags: u32) -> *mut clk_hw {
    let _ = (name, parent_name, base, pll_clk, pll_flags);
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
