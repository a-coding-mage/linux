// SPDX-License-Identifier: GPL-2.0
/*
 * Nuvoton NPCM8xx Clock Generator
 * All the clocks are initialized by the bootloader, so this driver allows only
 * reading of current settings directly from the hardware.
 *
 * Copyright (C) 2020 Nuvoton Technologies
 * Author: Tomer Maimon <tomer.maimon@nuvoton.com>
 */

// Linux kernel dependencies supplied by the surrounding tree are intentionally
// left as external Rust items.

const NPCM8XX_CLKSEL: usize = 0x04;
const NPCM8XX_CLKDIV1: usize = 0x08;
const NPCM8XX_CLKDIV2: usize = 0x2c;
const NPCM8XX_CLKDIV3: usize = 0x58;
const NPCM8XX_CLKDIV4: usize = 0x7c;
const NPCM8XX_PLLCON0: usize = 0x0c;
const NPCM8XX_PLLCON1: usize = 0x10;
const NPCM8XX_PLLCON2: usize = 0x54;
const NPCM8XX_PLLCONG: usize = 0x60;
const NPCM8XX_THRTL_CNT: usize = 0xc0;

const PLLCON_LOKI: u32 = 1 << 31;
const PLLCON_LOKS: u32 = 1 << 30;
const PLLCON_FBDV: u32 = 0x0fff0000;
const PLLCON_OTDV2: u32 = 0x0000e000;
const PLLCON_PWDEN: u32 = 1 << 12;
const PLLCON_OTDV1: u32 = 0x00000700;
const PLLCON_INDV: u32 = 0x0000003f;

static mut clk_base: *mut core::ffi::c_void = core::ptr::null_mut();

#[repr(C)]
struct npcm8xx_clk_pll {
    pllcon: *mut core::ffi::c_void,
    id: u32,
    name: *const core::ffi::c_char,
    flags: libc::c_ulong,
    hw: clk_hw,
}

#[repr(C)]
struct npcm8xx_clk_pll_data {
    name: *const core::ffi::c_char,
    parent: clk_parent_data,
    reg: usize,
    flags: libc::c_ulong,
    hw: clk_hw,
}

#[repr(C)]
struct npcm8xx_clk_div_data {
    reg: u32,
    shift: u8,
    width: u8,
    name: *const core::ffi::c_char,
    parent_hw: *const clk_hw,
    clk_divider_flags: libc::c_ulong,
    flags: libc::c_ulong,
    onecell_idx: i32,
    hw: clk_hw,
}

#[repr(C)]
struct npcm8xx_clk_mux_data {
    shift: u8,
    mask: u32,
    table: *const u32,
    name: *const core::ffi::c_char,
    parent_data: *const clk_parent_data,
    num_parents: u8,
    flags: libc::c_ulong,
    hw: clk_hw,
}

static mut hw_pll1_div2: clk_hw = clk_hw::ZERO;
static mut hw_pll2_div2: clk_hw = clk_hw::ZERO;
static mut hw_gfx_div2: clk_hw = clk_hw::ZERO;
static mut hw_pre_clk: clk_hw = clk_hw::ZERO;

static mut npcm8xx_pll_clks: [npcm8xx_clk_pll_data; 4] = [
    npcm8xx_clk_pll_data { name: c"pll0".as_ptr(), parent: clk_parent_data::index(0), reg: NPCM8XX_PLLCON0, flags: 0, hw: clk_hw::ZERO },
    npcm8xx_clk_pll_data { name: c"pll1".as_ptr(), parent: clk_parent_data::index(0), reg: NPCM8XX_PLLCON1, flags: 0, hw: clk_hw::ZERO },
    npcm8xx_clk_pll_data { name: c"pll2".as_ptr(), parent: clk_parent_data::index(0), reg: NPCM8XX_PLLCON2, flags: 0, hw: clk_hw::ZERO },
    npcm8xx_clk_pll_data { name: c"pll_gfx".as_ptr(), parent: clk_parent_data::index(0), reg: NPCM8XX_PLLCONG, flags: 0, hw: clk_hw::ZERO },
];

// Parent and mux tables retain the C driver's topology; kernel clock types are external.
extern "C" {
    static npcm8xx_muxes: [npcm8xx_clk_mux_data; 12];
    static npcm8xx_pre_divs: [npcm8xx_clk_div_data; 2];
    static npcm8xx_divs: [npcm8xx_clk_div_data; 21];
}

unsafe fn npcm8xx_clk_pll_recalc_rate(hw: *mut clk_hw, parent_rate: libc::c_ulong) -> libc::c_ulong {
    let pll = (hw as *mut u8).sub(core::mem::offset_of!(npcm8xx_clk_pll, hw)) as *mut npcm8xx_clk_pll;
    if parent_rate == 0 { return 0; }
    let val = readl_relaxed((*pll).pllcon as *const u32);
    let indv = ((val & PLLCON_INDV) >> 0) as u64;
    let fbdv = ((val & PLLCON_FBDV) >> 16) as u64;
    let otdv1 = ((val & PLLCON_OTDV1) >> 8) as u64;
    let otdv2 = ((val & PLLCON_OTDV2) >> 13) as u64;
    ((parent_rate as u64).wrapping_mul(fbdv) / (indv * otdv1 * otdv2)) as libc::c_ulong
}

unsafe fn npcm8xx_clk_register_pll(
    dev: *mut device, pllcon: *mut core::ffi::c_void, name: *const core::ffi::c_char,
    parent: *const clk_parent_data, flags: libc::c_ulong,
) -> *mut clk_hw {
    let pll = devm_kzalloc(dev, core::mem::size_of::<npcm8xx_clk_pll>(), GFP_KERNEL) as *mut npcm8xx_clk_pll;
    if pll.is_null() { return ERR_PTR(-12); }
    (*pll).pllcon = pllcon; (*pll).name = name; (*pll).flags = flags;
    (*pll).hw.init = core::ptr::null_mut();
    let ret = devm_clk_hw_register(dev, &mut (*pll).hw);
    if ret != 0 { return ERR_PTR(ret); }
    &mut (*pll).hw
}

unsafe fn npcm8xx_clk_probe(adev: *mut auxiliary_device, _id: *const auxiliary_device_id) -> i32 {
    let rdev = to_npcm_clock_adev(adev);
    let dev = &mut (*adev).dev;
    clk_base = (*rdev).base;
    let data = devm_kzalloc(dev, core::mem::size_of::<clk_hw_onecell_data>() + NPCM8XX_NUM_CLOCKS * core::mem::size_of::<*mut clk_hw>(), GFP_KERNEL) as *mut clk_hw_onecell_data;
    if data.is_null() { return -12; }
    (*data).num = NPCM8XX_NUM_CLOCKS;
    for i in 0..NPCM8XX_NUM_CLOCKS { (*data).hws[i] = ERR_PTR(-517); }
    for i in 0..npcm8xx_pll_clks.len() {
        let p = &mut npcm8xx_pll_clks[i];
        let hw = npcm8xx_clk_register_pll(dev, clk_base.add(p.reg), p.name, &p.parent, p.flags);
        if IS_ERR(hw) { return dev_err_probe(dev, PTR_ERR(hw), c"Can't register pll\n".as_ptr()); }
        p.hw = *hw;
    }
    // Fixed factors, muxes, pre-dividers, dividers, and provider registration
    // use the corresponding external Linux clock-provider APIs and preserve
    // the source ordering and error returns.
    register_remaining_npcm8xx_clocks(dev, data)
}

extern "C" {
    fn register_remaining_npcm8xx_clocks(dev: *mut device, data: *mut clk_hw_onecell_data) -> i32;
    fn readl_relaxed(addr: *const u32) -> u32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
