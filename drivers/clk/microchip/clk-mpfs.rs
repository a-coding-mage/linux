// SPDX-License-Identifier: GPL-2.0-only
/* PolarFire SoC MSS/core complex clock control */

use core::ffi::c_void;

// Linux kernel dependencies supplied by other translation units.
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_init_data { _private: [u8; 0] }
#[repr(C)] pub struct clk_div_table { pub val: u32, pub div: u32 }
#[repr(C)] pub struct clk_rate_request { _private: [u8; 0] }
#[repr(C)] pub struct clk_ops { _private: [u8; 0] }
#[repr(C)] pub struct clk_parent_data { pub index: u32 }
#[repr(C)] pub struct clk_divider { pub hw: clk_hw, pub reg: *mut c_void, pub shift: u8, pub width: u8, pub table: *const clk_div_table, pub flags: u32, pub lock: *mut c_void }
#[repr(C)] pub struct clk_hw_onecell_data { pub num: u32, pub hws: *mut *mut clk_hw }
#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub val_format_endian: u32, pub max_register: u32 }
#[repr(C)] pub struct of_device_id { pub compatible: *const u8 }
#[repr(C)] pub struct platform_driver { pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, pub name: *const u8, pub of_match_table: *const of_device_id }

type u32_ = u32;
const REG_MSSPLL_REF_CR: u32 = 0x08;
const REG_MSSPLL_POSTDIV01_CR: u32 = 0x10;
const REG_MSSPLL_POSTDIV23_CR: u32 = 0x14;
const REG_MSSPLL_SSCG_2_CR: u32 = 0x2c;
const REG_CLOCK_CONFIG_CR: u32 = 0x08;
const REG_RTC_CLOCK_CR: u32 = 0x0c;
const REG_SUBBLK_CLOCK_CR: u32 = 0x84;
const REG_SUBBLK_RESET_CR: u32 = 0x88;
const MSSPLL_FBDIV_SHIFT: u32 = 0;
const MSSPLL_FBDIV_WIDTH: u32 = 0x0c;
const MSSPLL_REFDIV_SHIFT: u32 = 0x08;
const MSSPLL_REFDIV_WIDTH: u32 = 0x06;
const MSSPLL_POSTDIV02_SHIFT: u32 = 0x08;
const MSSPLL_POSTDIV13_SHIFT: u32 = 0x18;
const MSSPLL_POSTDIV_WIDTH: u32 = 0x07;
const MSSPLL_FIXED_DIV: u32 = 4;
const CLK_MSSPLL_INTERNAL: u32 = 38;

#[repr(C)] pub struct mpfs_clock_data { pub dev: *mut device, pub regmap: *mut regmap, pub base: *mut c_void, pub msspll_base: *mut c_void, pub hw_data: clk_hw_onecell_data }
#[repr(C)] pub struct mpfs_msspll_hw_clock { pub base: *mut c_void, pub hw: clk_hw, pub init: clk_init_data, pub id: u32, pub reg_offset: u32, pub shift: u32, pub width: u32, pub flags: u32 }
#[repr(C)] pub struct mpfs_msspll_out_hw_clock { pub base: *mut c_void, pub output: clk_divider, pub init: clk_init_data, pub id: u32, pub reg_offset: u32 }
#[repr(C)] pub struct mpfs_cfg_clock { pub map: *mut regmap, pub table: *const clk_div_table, pub map_offset: u8, pub shift: u8, pub width: u8, pub flags: u8 }
#[repr(C)] pub struct mpfs_cfg_hw_clock { pub hw: clk_hw, pub cfg: mpfs_cfg_clock, pub id: u32 }
#[repr(C)] pub struct mpfs_periph_clock { pub map: *mut regmap, pub map_offset: u8, pub shift: u8 }
#[repr(C)] pub struct mpfs_periph_hw_clock { pub hw: clk_hw, pub periph: mpfs_periph_clock, pub id: u32 }

const CLK_MSSPLL0: u32 = 0; const CLK_MSSPLL1: u32 = 1; const CLK_MSSPLL2: u32 = 2; const CLK_MSSPLL3: u32 = 3;
const CLK_CPU: u32 = 4; const CLK_AXI: u32 = 5; const CLK_AHB: u32 = 6; const CLK_RTCREF: u32 = 7;
const CLK_ENVM: u32 = 8; const CLK_MAC0: u32 = 9; const CLK_MAC1: u32 = 10; const CLK_MMC: u32 = 11;
const CLK_TIMER: u32 = 12; const CLK_MMUART0: u32 = 13; const CLK_MMUART1: u32 = 14; const CLK_MMUART2: u32 = 15;
const CLK_MMUART3: u32 = 16; const CLK_MMUART4: u32 = 17; const CLK_SPI0: u32 = 18; const CLK_SPI1: u32 = 19;
const CLK_I2C0: u32 = 20; const CLK_I2C1: u32 = 21; const CLK_CAN0: u32 = 22; const CLK_CAN1: u32 = 23;
const CLK_USB: u32 = 24; const CLK_RTC: u32 = 25; const CLK_QSPI: u32 = 26; const CLK_GPIO0: u32 = 27;
const CLK_GPIO1: u32 = 28; const CLK_GPIO2: u32 = 29; const CLK_DDRC: u32 = 30; const CLK_FIC0: u32 = 31;
const CLK_FIC1: u32 = 32; const CLK_FIC2: u32 = 33; const CLK_FIC3: u32 = 34; const CLK_ATHENA: u32 = 35; const CLK_CFM: u32 = 36;

extern "C" {
    fn readl_relaxed(addr: *const c_void) -> u32;
    fn regmap_read(map: *mut regmap, offset: u32, val: *mut u32) -> i32;
    fn regmap_update_bits(map: *mut regmap, offset: u32, mask: u32, val: u32) -> i32;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const u8, ...) -> i32;
    fn divider_recalc_rate(hw: *mut clk_hw, prate: u64, val: u32, table: *const clk_div_table, flags: u8, width: u8) -> u64;
    fn divider_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request, table: *const clk_div_table, width: u8, flags: u32) -> i32;
    fn divider_get_val(rate: u64, prate: u64, table: *const clk_div_table, width: u8, flags: u32) -> i32;
    fn syscon_regmap_lookup_by_compatible(name: *const u8) -> *mut regmap;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, base: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn mpfs_reset_controller_register(dev: *mut device, map: *mut regmap) -> i32;
    fn devm_of_clk_add_hw_provider(dev: *mut device, get: *const c_void, data: *mut clk_hw_onecell_data) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

static MPFS_CLK_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, val_format_endian: 0, max_register: REG_SUBBLK_RESET_CR };
static MPFS_DIV_CPU_AXI_TABLE: [clk_div_table; 5] = [clk_div_table{val:0,div:1},clk_div_table{val:1,div:2},clk_div_table{val:2,div:4},clk_div_table{val:3,div:8},clk_div_table{val:0,div:0}];
static MPFS_DIV_AHB_TABLE: [clk_div_table; 4] = [clk_div_table{val:1,div:2},clk_div_table{val:2,div:4},clk_div_table{val:3,div:8},clk_div_table{val:0,div:0}];
static MPFS_DIV_RTCREF_TABLE: [clk_div_table; 3] = [clk_div_table{val:100,div:100},clk_div_table{val:125,div:125},clk_div_table{val:0,div:0}];

unsafe extern "C" fn mpfs_clk_msspll_recalc_rate(hw: *mut clk_hw, prate: u64) -> u64 {
    let h = &*(hw as *mut mpfs_msspll_hw_clock); let mult = (readl_relaxed(h.base.add(h.reg_offset as usize)) >> MSSPLL_FBDIV_SHIFT) & ((1u32 << MSSPLL_FBDIV_WIDTH)-1); let r = (readl_relaxed(h.base.add(REG_MSSPLL_REF_CR as usize)) >> MSSPLL_REFDIV_SHIFT) & ((1u32 << MSSPLL_REFDIV_WIDTH)-1); prate * mult as u64 / (r as u64 * MSSPLL_FIXED_DIV as u64)
}

unsafe fn mpfs_cfg_clk_recalc_rate(_hw: *mut clk_hw, prate: u64) -> u64 { prate }
unsafe fn mpfs_cfg_clk_determine_rate(_hw: *mut clk_hw, _req: *mut clk_rate_request) -> i32 { 0 }
unsafe fn mpfs_cfg_clk_set_rate(_hw: *mut clk_hw, _rate: u64, _prate: u64) -> i32 { 0 }
unsafe fn mpfs_periph_clk_enable(hw: *mut clk_hw) -> i32 {
    let p = &*(hw as *mut mpfs_periph_hw_clock); regmap_update_bits(p.periph.map, p.periph.map_offset as u32, 1u32 << p.periph.shift, 1u32 << p.periph.shift); 0
}
unsafe fn mpfs_periph_clk_disable(hw: *mut clk_hw) { let p = &*(hw as *mut mpfs_periph_hw_clock); regmap_update_bits(p.periph.map, p.periph.map_offset as u32, 1u32 << p.periph.shift, 0); }
unsafe fn mpfs_periph_clk_is_enabled(hw: *mut clk_hw) -> i32 { let p = &*(hw as *mut mpfs_periph_hw_clock); let mut v=0; regmap_read(p.periph.map,p.periph.map_offset as u32,&mut v); ((v & (1u32 << p.periph.shift)) != 0) as i32 }

unsafe fn mpfs_clk_register_mssplls(_dev: *mut device, hws: *mut mpfs_msspll_hw_clock, n: usize, data: *mut mpfs_clock_data) -> i32 { for i in 0..n { let h=&mut *hws.add(i); h.base=(*data).msspll_base; let r=devm_clk_hw_register(_dev,&mut h.hw); if r!=0{return r;} } 0 }
unsafe fn mpfs_clk_register_msspll_outs(_dev: *mut device, _hws: *mut mpfs_msspll_out_hw_clock, _n: usize, _data: *mut mpfs_clock_data) -> i32 { 0 }
unsafe fn mpfs_clk_register_cfgs(_dev: *mut device, _hws: *mut mpfs_cfg_hw_clock, _n: usize, _data: *mut mpfs_clock_data) -> i32 { 0 }
unsafe fn mpfs_clk_register_periphs(_dev: *mut device, _hws: *mut mpfs_periph_hw_clock, _n: usize, _data: *mut mpfs_clock_data) -> i32 { 0 }
unsafe fn mpfs_clk_syscon_probe(data: *mut mpfs_clock_data, pdev: *mut platform_device) -> i32 { (*data).regmap=syscon_regmap_lookup_by_compatible(b"microchip,mpfs-mss-top-sysreg\0".as_ptr()); (*data).msspll_base=devm_platform_ioremap_resource(pdev,0); 0 }
unsafe fn mpfs_clk_old_format_probe(data: *mut mpfs_clock_data, pdev: *mut platform_device) -> i32 { (*data).base=devm_platform_ioremap_resource(pdev,0); (*data).msspll_base=devm_platform_ioremap_resource(pdev,1); (*data).regmap=devm_regmap_init_mmio((*data).dev,(*data).base,&MPFS_CLK_REGMAP_CONFIG); mpfs_reset_controller_register((*data).dev,(*data).regmap) }
unsafe fn mpfs_clk_probe(_pdev: *mut platform_device) -> i32 { 0 }

#[no_mangle] pub unsafe extern "C" fn clk_mpfs_init() -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn clk_mpfs_exit() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
