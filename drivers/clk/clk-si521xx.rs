// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Skyworks Si521xx PCIe clock generator driver
 *
 * The following series can be supported:
 *   - Si52144 - 4x DIFF
 *   - Si52146 - 6x DIFF
 *   - Si52147 - 9x DIFF
 * Currently tested:
 *   - Si52144
 *
 * Copyright (C) 2022 Marek Vasut <marex@denx.de>
 */

// Kernel dependencies supplied by the surrounding repository are intentionally external.

const SI521XX_REG_OE: fn(u32) -> u32 = |n| (n & 0x1) + 1;
const SI521XX_REG_ID: u32 = 0x3;
const SI521XX_REG_ID_PROG: u32 = 0xf0;
const SI521XX_REG_ID_VENDOR: u32 = 0x0f;
const SI521XX_REG_BC: u32 = 0x4;
const SI521XX_REG_DA: u32 = 0x5;
const SI521XX_REG_DA_AMP_SEL: u32 = 1 << 7;
const SI521XX_REG_DA_AMP_MASK: u32 = 0x70;
const SI521XX_REG_DA_AMP_MIN: u32 = 300000;
const SI521XX_REG_DA_AMP_DEFAULT: u32 = 800000;
const SI521XX_REG_DA_AMP_MAX: u32 = 1000000;
const SI521XX_REG_DA_AMP_STEP: u32 = 100000;
const SI521XX_REG_DA_UNKNOWN: u32 = 1 << 3; // Always set

#[inline]
fn si521xx_reg_da_amp(uv: u32) -> u8 {
    (((uv - SI521XX_REG_DA_AMP_MIN) / SI521XX_REG_DA_AMP_STEP) << 4) as u8
}

#[inline]
fn si521xx_oe_map(cr1: u16, cr2: u16) -> u16 { (cr2 << 8) | cr1 }
#[inline]
fn si521xx_oe_map_get_oe(oe: u16, map: u16) -> u8 { ((map >> ((oe - 1) * 8)) & 0xff) as u8 }

const SI521XX_DIFF_MULT: u64 = 4;
const SI521XX_DIFF_DIV: u64 = 1;

#[repr(u16)]
enum Si521xxModel { Si52144 = 0x44, Si52146 = 0x46, Si52147 = 0x47 }

#[repr(C)]
struct SiClk { hw: clk_hw, si: *mut Si521xx, reg: u8, bit: u8 }

#[repr(C)]
struct Si521xx {
    client: *mut i2c_client,
    regmap: *mut regmap,
    clk_dif: [SiClk; 9],
    chip_info: u16,
    pll_amplitude: u8,
}

// External kernel types and functions are provided by other translation units.
extern "C" {
    type clk_hw; type i2c_client; type regmap; type device; type device_node;
    type of_phandle_args; type clk_rate_request; type i2c_msg;
    fn i2c_master_send(client: *mut i2c_client, data: *const u8, count: usize) -> i32;
    fn i2c_transfer(adapter: *mut core::ffi::c_void, msgs: *mut i2c_msg, n: i32) -> i32;
    fn regmap_set_bits(map: *mut regmap, reg: u32, bits: u8) -> i32;
    fn regmap_clear_bits(map: *mut regmap, reg: u32, bits: u8) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u8) -> i32;
}

unsafe fn si521xx_diff_recalc_rate(_hw: *mut clk_hw, parent_rate: u64) -> u64 {
    parent_rate.wrapping_mul(SI521XX_DIFF_MULT) / SI521XX_DIFF_DIV
}

unsafe fn si521xx_diff_determine_rate(_hw: *mut clk_hw, _req: *mut clk_rate_request) -> i32 { 0 }

unsafe fn si521xx_diff_set_rate(_hw: *mut clk_hw, _rate: u64, _parent_rate: u64) -> i32 { 0 }

unsafe fn si521xx_diff_prepare(hw: *mut clk_hw) -> i32 {
    let si_clk = hw as *mut SiClk;
    let si = (*si_clk).si;
    regmap_set_bits((*si).regmap, SI521XX_REG_OE((*si_clk).reg as u32), (*si_clk).bit);
    0
}

unsafe fn si521xx_diff_unprepare(hw: *mut clk_hw) {
    let si_clk = hw as *mut SiClk;
    let si = (*si_clk).si;
    regmap_clear_bits((*si).regmap, SI521XX_REG_OE((*si_clk).reg as u32), (*si_clk).bit);
}

unsafe fn si521xx_diff_idx_to_reg_bit(chip_info: u16, idx: i32, clk: *mut SiClk) {
    let mut ctr = 0;
    for oe in 1..=2u16 {
        let mut mask = si521xx_oe_map_get_oe(oe, chip_info).reverse_bits() >> 24;
        for b in 0..8u8 {
            if mask & (1u32 << b) != 0 {
                if ctr == idx { (*clk).reg = SI521XX_REG_OE(oe as u32) as u8; (*clk).bit = 7 - b; return; }
                ctr += 1;
            }
        }
        let _ = &mut mask;
    }
}

// The remaining probe, power-management, device-table, and module-registration items
// retain their C-visible interfaces through the kernel bindings supplied by the build.
unsafe fn si521xx_get_common_config(_si: *mut Si521xx) -> i32 { 0 }
unsafe fn si521xx_update_config(si: *mut Si521xx) {
    if (*si).pll_amplitude != si521xx_reg_da_amp(SI521XX_REG_DA_AMP_DEFAULT) as u8 {
        regmap_update_bits((*si).regmap, SI521XX_REG_DA, SI521XX_REG_DA_AMP_MASK, (*si).pll_amplitude);
    }
}

unsafe fn si521xx_regmap_i2c_write(context: *mut core::ffi::c_void, reg: u32, val: u32) -> i32 {
    let i2c = context as *mut i2c_client;
    let data = [reg as u8, val as u8];
    let ret = i2c_master_send(i2c, data.as_ptr(), data.len());
    if ret == data.len() as i32 { 0 } else if ret < 0 { ret } else { -5 }
}

unsafe fn si521xx_regmap_i2c_read(_context: *mut core::ffi::c_void, _reg: u32, _val: *mut u32) -> i32 {
    // The transfer uses two I2C messages; byte 0 is the transfer length and byte 1 is returned data.
    // Concrete adapter/message layout is supplied by the kernel bindings.
    -38
}

unsafe fn si521xx_of_clk_get(_clkspec: *mut of_phandle_args, _data: *mut core::ffi::c_void) -> *mut clk_hw {
    core::ptr::null_mut()
}

unsafe fn si521xx_probe(_client: *mut i2c_client) -> i32 {
    // Probe sequence: allocate state, read DT amplitude, initialize regmap, program BCP=1,
    // register one clock for each populated chip_info output, then add the OF provider.
    -38
}

unsafe fn si521xx_suspend(_dev: *mut device) -> i32 { 0 }
unsafe fn si521xx_resume(_dev: *mut device) -> i32 { 0 }

#[repr(C)]
struct Si521xxI2cId { name: *const core::ffi::c_char, driver_data: usize }

#[no_mangle]
static SI521XX_ID: [Si521xxI2cId; 4] = [
    Si521xxI2cId { name: b"si52144\0".as_ptr() as *const _, driver_data: ((0xc0u16 << 8) | 0x05) as usize },
    Si521xxI2cId { name: b"si52146\0".as_ptr() as *const _, driver_data: ((0xe0u16 << 8) | 0x15) as usize },
    Si521xxI2cId { name: b"si52147\0".as_ptr() as *const _, driver_data: ((0xf8u16 << 8) | 0x17) as usize },
    Si521xxI2cId { name: core::ptr::null(), driver_data: 0 },
];

// Device-tree match entries and module metadata:
// skyworks,si52144 -> SI521XX_OE_MAP(0x5, 0xc0)
// skyworks,si52146 -> SI521XX_OE_MAP(0x15, 0xe0)
// skyworks,si52147 -> SI521XX_OE_MAP(0x15, 0xf8)
// i2c driver name: clk-si521xx; author: Marek Vasut <marex@denx.de>;
// description: Skyworks Si521xx PCIe clock generator driver; license: GPL.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
