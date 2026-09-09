// SPDX-License-Identifier: GPL-2.0-only
/*
 * Synopsys AXS10X SDP I2S PLL clock driver
 *
 * Copyright (C) 2016 Synopsys
 */

// Linux kernel dependencies supplied by other translation units.

const PLL_IDIV_REG: usize = 0x0;
const PLL_FBDIV_REG: usize = 0x4;
const PLL_ODIV0_REG: usize = 0x8;
const PLL_ODIV1_REG: usize = 0xC;

#[repr(C)]
pub struct i2s_pll_cfg {
    pub rate: u32,
    pub idiv: u32,
    pub fbdiv: u32,
    pub odiv0: u32,
    pub odiv1: u32,
}

static I2S_PLL_CFG_27M: [i2s_pll_cfg; 9] = [
    i2s_pll_cfg { rate: 1024000, idiv: 0x104, fbdiv: 0x451, odiv0: 0x10E38, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 1411200, idiv: 0x104, fbdiv: 0x596, odiv0: 0x10D35, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 1536000, idiv: 0x208, fbdiv: 0xA28, odiv0: 0x10B2C, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 2048000, idiv: 0x82, fbdiv: 0x451, odiv0: 0x10E38, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 2822400, idiv: 0x82, fbdiv: 0x596, odiv0: 0x10D35, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 3072000, idiv: 0x104, fbdiv: 0xA28, odiv0: 0x10B2C, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 2116800, idiv: 0x82, fbdiv: 0x3CF, odiv0: 0x10C30, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 2304000, idiv: 0x104, fbdiv: 0x79E, odiv0: 0x10B2C, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 0, idiv: 0, fbdiv: 0, odiv0: 0, odiv1: 0 },
];

static I2S_PLL_CFG_28M: [i2s_pll_cfg; 9] = [
    i2s_pll_cfg { rate: 1024000, idiv: 0x82, fbdiv: 0x105, odiv0: 0x107DF, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 1411200, idiv: 0x28A, fbdiv: 0x1, odiv0: 0x10001, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 1536000, idiv: 0xA28, fbdiv: 0x187, odiv0: 0x10042, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 2048000, idiv: 0x41, fbdiv: 0x105, odiv0: 0x107DF, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 2822400, idiv: 0x145, fbdiv: 0x1, odiv0: 0x10001, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 3072000, idiv: 0x514, fbdiv: 0x187, odiv0: 0x10042, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 2116800, idiv: 0x514, fbdiv: 0x42, odiv0: 0x10001, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 2304000, idiv: 0x619, fbdiv: 0x82, odiv0: 0x10001, odiv1: 0x2000 },
    i2s_pll_cfg { rate: 0, idiv: 0, fbdiv: 0, odiv0: 0, odiv1: 0 },
];

#[repr(C)]
pub struct i2s_pll_clk {
    pub base: *mut core::ffi::c_void,
    pub hw: clk_hw,
    pub dev: *mut device,
}

#[repr(C)]
pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct device;
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct clk;
#[repr(C)] pub struct device_node { pub name: *const core::ffi::c_char }
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong, pub best_parent_rate: c_ulong }
#[repr(C)] pub struct clk_init_data {
    pub name: *const core::ffi::c_char,
    pub ops: *const clk_ops,
    pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: u8,
}
#[repr(C)] pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> i32>,
}
type c_ulong = usize;

unsafe fn i2s_pll_write(clk: *mut i2s_pll_clk, reg: usize, val: u32) {
    core::ptr::write_volatile(((*clk).base as *mut u8).add(reg) as *mut u32, val);
}
unsafe fn i2s_pll_read(clk: *mut i2s_pll_clk, reg: usize) -> u32 {
    core::ptr::read_volatile(((*clk).base as *const u8).add(reg) as *const u32)
}
unsafe fn to_i2s_pll_clk(hw: *mut clk_hw) -> *mut i2s_pll_clk {
    (hw as *mut u8).sub(core::mem::offset_of!(i2s_pll_clk, hw)) as *mut i2s_pll_clk
}
fn i2s_pll_get_value(val: u32) -> u32 { (val & 0x3F) + ((val >> 6) & 0x3F) }
fn i2s_pll_get_cfg(prate: c_ulong) -> *const i2s_pll_cfg {
    match prate { 27000000 => I2S_PLL_CFG_27M.as_ptr(), 28224000 => I2S_PLL_CFG_28M.as_ptr(), _ => core::ptr::null() }
}

unsafe extern "C" fn i2s_pll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let clk = to_i2s_pll_clk(hw);
    let idiv = i2s_pll_get_value(i2s_pll_read(clk, PLL_IDIV_REG));
    let fbdiv = i2s_pll_get_value(i2s_pll_read(clk, PLL_FBDIV_REG));
    let odiv = i2s_pll_get_value(i2s_pll_read(clk, PLL_ODIV0_REG));
    ((parent_rate / idiv as usize) * fbdiv as usize) / odiv as usize
}

unsafe extern "C" fn i2s_pll_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let pll_cfg = i2s_pll_get_cfg((*req).best_parent_rate);
    if pll_cfg.is_null() { return -22; }
    let mut i = 0;
    while (*pll_cfg.add(i)).rate != 0 {
        if (*pll_cfg.add(i)).rate as usize == (*req).rate { return 0; }
        i += 1;
    }
    -22
}

unsafe extern "C" fn i2s_pll_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> i32 {
    let clk = to_i2s_pll_clk(hw);
    let pll_cfg = i2s_pll_get_cfg(parent_rate);
    if pll_cfg.is_null() { return -22; }
    let mut i = 0;
    while (*pll_cfg.add(i)).rate != 0 {
        let cfg = &*pll_cfg.add(i);
        if cfg.rate as usize == rate {
            i2s_pll_write(clk, PLL_IDIV_REG, cfg.idiv);
            i2s_pll_write(clk, PLL_FBDIV_REG, cfg.fbdiv);
            i2s_pll_write(clk, PLL_ODIV0_REG, cfg.odiv0);
            i2s_pll_write(clk, PLL_ODIV1_REG, cfg.odiv1);
            return 0;
        }
        i += 1;
    }
    -22
}

static I2S_PLL_OPS: clk_ops = clk_ops {
    recalc_rate: Some(i2s_pll_recalc_rate),
    determine_rate: Some(i2s_pll_determine_rate),
    set_rate: Some(i2s_pll_set_rate),
};

// Platform-driver registration and kernel allocation/provider operations are external dependencies.
extern "C" {
    fn i2s_pll_clk_probe(pdev: *mut platform_device) -> i32;
    fn i2s_pll_clk_remove(pdev: *mut platform_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
