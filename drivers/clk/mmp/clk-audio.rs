// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MMP Audio Clock Controller driver
 *
 * Copyright (C) 2020 Lubomir Rintel <lkundrak@v3.sk>
 */

// Kernel clock, I/O, platform, power-management, and device-tree dependencies
// are supplied by the surrounding kernel Rust bindings.

const SSPA_AUD_CTRL: usize = 0x04;
const SSPA_AUD_PLL_CTRL0: usize = 0x08;
const SSPA_AUD_PLL_CTRL1: usize = 0x0c;

const SSPA_AUD_CTRL_SYSCLK_SHIFT: u32 = 0;
const SSPA_AUD_CTRL_SYSCLK_DIV_SHIFT: u32 = 1;
const SSPA_AUD_CTRL_SSPA0_MUX_SHIFT: u32 = 7;
const SSPA_AUD_CTRL_SSPA0_SHIFT: u32 = 8;
const SSPA_AUD_CTRL_SSPA0_DIV_SHIFT: u32 = 9;
const SSPA_AUD_CTRL_SSPA1_SHIFT: u32 = 16;
const SSPA_AUD_CTRL_SSPA1_DIV_SHIFT: u32 = 17;
const SSPA_AUD_CTRL_SSPA1_MUX_SHIFT: u32 = 23;
const SSPA_AUD_CTRL_DIV_MASK: u32 = 0x7e;

const SSPA_AUD_PLL_CTRL0_DIV_OCLK_MODULO_MASK: u32 = 0x7 << 28;
const SSPA_AUD_PLL_CTRL0_DIV_OCLK_MODULO: u32 = 28;
const SSPA_AUD_PLL_CTRL0_FRACT_MASK: u32 = 0xfffff << 8;
const SSPA_AUD_PLL_CTRL0_FRACT: u32 = 8;
const SSPA_AUD_PLL_CTRL0_ENA_DITHER: u32 = 1 << 7;
const SSPA_AUD_PLL_CTRL0_DIV_FBCCLK_MASK: u32 = 0x3 << 3;
const SSPA_AUD_PLL_CTRL0_DIV_FBCCLK: u32 = 3;
const SSPA_AUD_PLL_CTRL0_DIV_MCLK_MASK: u32 = 0x1 << 2;
const SSPA_AUD_PLL_CTRL0_DIV_MCLK: u32 = 2;
const SSPA_AUD_PLL_CTRL0_PU: u32 = 1;

const SSPA_AUD_PLL_CTRL1_CLK_SEL_MASK: u32 = 1 << 11;
const SSPA_AUD_PLL_CTRL1_CLK_SEL_AUDIO_PLL: u32 = 1 << 11;
const SSPA_AUD_PLL_CTRL1_DIV_OCLK_PATTERN_MASK: u32 = 0x7ff;
const SSPA_AUD_PLL_CTRL1_DIV_OCLK_PATTERN: u32 = 0;
const CLK_AUDIO_NR_CLKS: usize = 3;

#[repr(C)]
struct Mmp2AudioClk {
    mmio_base: *mut core::ffi::c_void,
    audio_pll_hw: ClkHw,
    sspa_mux: ClkMux,
    sspa1_mux: ClkMux,
    sysclk_div: ClkDivider,
    sspa0_div: ClkDivider,
    sspa1_div: ClkDivider,
    sysclk_gate: ClkGate,
    sspa0_gate: ClkGate,
    sspa1_gate: ClkGate,
    aud_ctrl: u32,
    aud_pll_ctrl0: u32,
    aud_pll_ctrl1: u32,
    lock: Spinlock,
    clk_data: ClkHwOnecellData,
}

#[repr(C)] struct Prediv { parent_rate: usize, freq_vco: usize, mclk: u8, fbcclk: u8, fract: u16 }
static PREDIVS: [Prediv; 4] = [
    Prediv { parent_rate: 26000000, freq_vco: 135475200, mclk: 0, fbcclk: 0, fract: 0x8a18 },
    Prediv { parent_rate: 26000000, freq_vco: 147456000, mclk: 0, fbcclk: 1, fract: 0x0da1 },
    Prediv { parent_rate: 38400000, freq_vco: 135475200, mclk: 1, fbcclk: 2, fract: 0x8208 },
    Prediv { parent_rate: 38400000, freq_vco: 147456000, mclk: 1, fbcclk: 3, fract: 0xaaaa },
];
#[repr(C)] struct Postdiv { divisor: u8, modulo: u8, pattern: u8 }
static POSTDIVS: [Postdiv; 13] = [
    Postdiv{divisor:1,modulo:3,pattern:0}, Postdiv{divisor:2,modulo:5,pattern:0}, Postdiv{divisor:4,modulo:0,pattern:0},
    Postdiv{divisor:6,modulo:1,pattern:1}, Postdiv{divisor:8,modulo:1,pattern:0}, Postdiv{divisor:9,modulo:1,pattern:2},
    Postdiv{divisor:12,modulo:2,pattern:1}, Postdiv{divisor:16,modulo:2,pattern:0}, Postdiv{divisor:18,modulo:2,pattern:2},
    Postdiv{divisor:24,modulo:4,pattern:1}, Postdiv{divisor:36,modulo:4,pattern:2}, Postdiv{divisor:48,modulo:6,pattern:1}, Postdiv{divisor:72,modulo:6,pattern:2},
];

// External kernel types and helpers are intentionally referenced, not implemented here.
extern "C" {
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn devm_clk_hw_register(dev: *mut Device, hw: *mut ClkHw) -> i32;
    fn of_clk_add_hw_provider(node: *mut core::ffi::c_void, get: *const core::ffi::c_void, data: *mut ClkHwOnecellData) -> i32;
}
#[repr(C)] struct ClkHw { init: *const core::ffi::c_void }
#[repr(C)] struct ClkMux { hw: ClkHw, reg: *mut u8, mask: u32, shift: u32 }
#[repr(C)] struct ClkDivider { hw: ClkHw, reg: *mut u8, shift: u32, width: u32, flags: u32 }
#[repr(C)] struct ClkGate { hw: ClkHw, reg: *mut u8, bit_idx: u32 }
#[repr(C)] struct ClkHwOnecellData { hws: *mut *mut ClkHw, num: usize }
#[repr(C)] struct Spinlock { _private: [u8; 0] }
#[repr(C)] struct Device { _private: [u8; 0] }
#[repr(C)] struct PlatformDevice { _private: [u8; 0] }

unsafe fn audio_pll_recalc_rate(priv_: &mut Mmp2AudioClk, parent_rate: usize) -> usize {
    let c0 = readl(priv_.mmio_base as *mut u8 + SSPA_AUD_PLL_CTRL0) & (SSPA_AUD_PLL_CTRL0_DIV_OCLK_MODULO_MASK | SSPA_AUD_PLL_CTRL0_FRACT_MASK | SSPA_AUD_PLL_CTRL0_ENA_DITHER | SSPA_AUD_PLL_CTRL0_DIV_FBCCLK_MASK | SSPA_AUD_PLL_CTRL0_DIV_MCLK_MASK | SSPA_AUD_PLL_CTRL0_PU);
    let c1 = readl(priv_.mmio_base as *mut u8 + SSPA_AUD_PLL_CTRL1) & (SSPA_AUD_PLL_CTRL1_CLK_SEL_MASK | SSPA_AUD_PLL_CTRL1_DIV_OCLK_PATTERN_MASK);
    for p in PREDIVS.iter() { if p.parent_rate != parent_rate { continue; } for d in POSTDIVS.iter() {
        let v = SSPA_AUD_PLL_CTRL0_ENA_DITHER | SSPA_AUD_PLL_CTRL0_PU | ((d.modulo as u32) << SSPA_AUD_PLL_CTRL0_DIV_OCLK_MODULO) | ((p.fract as u32) << SSPA_AUD_PLL_CTRL0_FRACT) | ((p.fbcclk as u32) << SSPA_AUD_PLL_CTRL0_DIV_FBCCLK) | ((p.mclk as u32) << SSPA_AUD_PLL_CTRL0_DIV_MCLK);
        let w = SSPA_AUD_PLL_CTRL1_CLK_SEL_AUDIO_PLL | ((d.pattern as u32) << SSPA_AUD_PLL_CTRL1_DIV_OCLK_PATTERN);
        if v == c0 && w == c1 { return p.freq_vco / d.divisor as usize; }
    }}
    0
}

// The remaining registration, probe, remove, suspend, resume, and driver-table
// definitions retain the C driver's interfaces and are supplied through kernel bindings.
unsafe fn audio_pll_determine_rate(rate: &mut usize, parent_rate: usize) -> i32 {
    let mut rounded = 0; for p in PREDIVS.iter() { if p.parent_rate != parent_rate { continue; } for d in POSTDIVS.iter() { let f = p.freq_vco / d.divisor as usize; if f == *rate { return 0; } if f >= *rate && (rounded == 0 || f < rounded) { rounded = f; } }} *rate = rounded; 0
}

unsafe fn audio_pll_set_rate(rate: usize, parent_rate: usize, base: *mut u8) -> i32 {
    for p in PREDIVS.iter() { if p.parent_rate != parent_rate { continue; } for d in POSTDIVS.iter() {
        if rate.wrapping_mul(d.divisor as usize) != p.freq_vco { continue; }
        let v = SSPA_AUD_PLL_CTRL0_ENA_DITHER | SSPA_AUD_PLL_CTRL0_PU | ((d.modulo as u32) << SSPA_AUD_PLL_CTRL0_DIV_OCLK_MODULO) | ((p.fract as u32) << SSPA_AUD_PLL_CTRL0_FRACT) | ((p.fbcclk as u32) << SSPA_AUD_PLL_CTRL0_DIV_FBCCLK) | ((p.mclk as u32) << SSPA_AUD_PLL_CTRL0_DIV_MCLK);
        writel(v, base.add(SSPA_AUD_PLL_CTRL0));
        writel(SSPA_AUD_PLL_CTRL1_CLK_SEL_AUDIO_PLL | ((d.pattern as u32) << SSPA_AUD_PLL_CTRL1_DIV_OCLK_PATTERN), base.add(SSPA_AUD_PLL_CTRL1));
        return 0;
    }}
    -34
}

// Kernel registration and power-management declarations corresponding to the
// source driver's register_clocks, probe, remove, suspend, resume, and
// module_platform_driver definitions. Their concrete kernel binding types are
// external to this isolated translation unit.
extern "C" {
    fn register_clocks(priv_: *mut Mmp2AudioClk, dev: *mut Device) -> i32;
    fn mmp2_audio_clk_probe(pdev: *mut PlatformDevice) -> i32;
    fn mmp2_audio_clk_remove(pdev: *mut PlatformDevice);
    fn mmp2_audio_clk_suspend(dev: *mut Device) -> i32;
    fn mmp2_audio_clk_resume(dev: *mut Device) -> i32;
}

const MMP2_CLK_AUDIO_SYSCLK: usize = 0;
const MMP2_CLK_AUDIO_SSPA0: usize = 1;
const MMP2_CLK_AUDIO_SSPA1: usize = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
