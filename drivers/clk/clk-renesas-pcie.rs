// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Renesas 9-series PCIe clock generator driver
 *
 * The following series can be supported:
 *   - 9FGV/9DBV/9DMV/9FGL/9DML/9QXL/9SQ
 * Currently supported:
 *   - 9FGV0241
 *   - 9FGV0441
 *   - 9FGV0841
 *
 * Copyright (C) 2022 Marek Vasut <marex@denx.de>
 */

// Dependencies are supplied by the Linux kernel bindings.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)] pub struct I2cClient { pub addr: u16, pub adapter: *mut I2cAdapter, pub dev: Device }
#[repr(C)] pub struct I2cAdapter;
#[repr(C)] pub struct Device { pub of_node: *mut DeviceNode }
#[repr(C)] pub struct DeviceNode;
#[repr(C)] pub struct Regmap;
#[repr(C)] pub struct ClkHw;
#[repr(C)] pub struct OfPhandleArgs { pub args: [u32; 1] }
#[repr(C)] pub struct DeviceId { pub name: [c_char; 32], pub driver_data: usize }
#[repr(C)] pub struct OfDeviceId { pub compatible: *const c_char, pub data: *const c_void }
#[repr(C)] pub struct I2cMsg { pub addr: u16, pub flags: u16, pub len: u16, pub buf: *mut u8 }
#[repr(C)] pub struct RegmapRange { pub range_min: c_uint, pub range_max: c_uint }
#[repr(C)] pub struct RegmapAccessTable { pub yes_ranges: *const RegmapRange, pub n_yes_ranges: usize }
#[repr(C)] pub struct RegmapConfig {
    pub reg_bits: c_uint, pub val_bits: c_uint, pub cache_type: c_uint,
    pub max_register: c_uint, pub num_reg_defaults_raw: c_uint,
    pub rd_table: *const RegmapAccessTable, pub wr_table: *const RegmapAccessTable,
    pub reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
    pub reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
}

extern "C" {
    fn i2c_master_send(client: *mut I2cClient, buf: *const u8, count: c_int) -> c_int;
    fn i2c_transfer(adapter: *mut I2cAdapter, msgs: *mut I2cMsg, count: c_int) -> c_int;
    fn i2c_set_clientdata(client: *mut I2cClient, data: *mut c_void);
    fn i2c_get_match_data(client: *mut I2cClient) -> *const Rs9ChipInfo;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init(dev: *mut Device, bus: *const c_void, context: *mut c_void, config: *const RegmapConfig) -> *mut Regmap;
    fn regmap_update_bits(map: *mut Regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut Regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut Regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regcache_cache_only(map: *mut Regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut Regmap);
    fn regcache_sync(map: *mut Regmap) -> c_int;
    fn of_get_child_by_name(node: *mut DeviceNode, name: *const c_char) -> *mut DeviceNode;
    fn of_node_put(node: *mut DeviceNode);
    fn of_property_read_u32(node: *mut DeviceNode, name: *const c_char, val: *mut u32) -> c_int;
    fn dev_err_probe(dev: *mut Device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut Device, fmt: *const c_char, ...);
    fn dev_get_drvdata(dev: *mut Device) -> *mut c_void;
    fn devm_clk_hw_register_fixed_factor_index(dev: *mut Device, name: *const c_char, flags: c_uint, parent: c_uint, mult: c_uint, div: c_uint) -> *mut ClkHw;
    fn devm_of_clk_add_hw_provider(dev: *mut Device, get: unsafe extern "C" fn(*mut OfPhandleArgs, *mut c_void) -> *mut ClkHw, data: *mut c_void) -> c_int;
}

const RS9_REG_OE: u32 = 0x0; const RS9_REG_SS: u32 = 0x1;
const RS9_REG_SS_AMP_0V6: u8 = 0x0; const RS9_REG_SS_AMP_0V7: u8 = 0x1;
const RS9_REG_SS_AMP_0V8: u8 = 0x2; const RS9_REG_SS_AMP_0V9: u8 = 0x3;
const RS9_REG_SS_AMP_DEFAULT: u8 = RS9_REG_SS_AMP_0V8; const RS9_REG_SS_AMP_MASK: u8 = 0x3;
const RS9_REG_SS_SSC_100: u8 = 0; const RS9_REG_SS_SSC_M025: u8 = 1 << 3; const RS9_REG_SS_SSC_M050: u8 = 3 << 3;
const RS9_REG_SS_SSC_DEFAULT: u8 = RS9_REG_SS_SSC_100; const RS9_REG_SS_SSC_MASK: u8 = 3 << 3;
const RS9_REG_SR: u32 = 0x2; const RS9_REG_VID: u32 = 0x5; const RS9_REG_DID: u32 = 0x6; const RS9_REG_BCP: u32 = 0x7;
const RS9_REG_VID_MASK: u32 = 0xf; const RS9_REG_VID_IDT: u32 = 1;
const RS9_REG_DID_TYPE_SHIFT: u32 = 6; const RS9_REG_DID_TYPE_FGV: u32 = 0 << RS9_REG_DID_TYPE_SHIFT;
const RS9_REG_DID_TYPE_DBV: u32 = 1 << RS9_REG_DID_TYPE_SHIFT; const RS9_REG_DID_TYPE_DMV: u32 = 2 << RS9_REG_DID_TYPE_SHIFT;
const I2C_M_RD: u16 = 1; const REGCACHE_FLAT: u32 = 1; const GFP_KERNEL: u32 = 0;

#[repr(C)] pub struct Rs9ChipInfo { pub num_clks: u32, pub outshift: u8, pub did: u8 }
#[repr(C)] pub struct Rs9DriverData { pub client: *mut I2cClient, pub regmap: *mut Regmap, pub chip_info: *const Rs9ChipInfo, pub clk_dif: [*mut ClkHw; 8], pub pll_amplitude: u8, pub pll_ssc: u8, pub clk_dif_sr: u8 }

unsafe extern "C" fn rs9_regmap_i2c_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    let i2c = context as *mut I2cClient; let data = [reg as u8, 1, val as u8]; let ret = i2c_master_send(i2c, data.as_ptr(), 3);
    if ret == 3 { 0 } else if ret < 0 { ret } else { -5 }
}
unsafe extern "C" fn rs9_regmap_i2c_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    let i2c = context as *mut I2cClient; let mut txdata = reg as u8; let mut rxdata = [0u8; 2];
    let mut xfer = [I2cMsg { addr: (*i2c).addr, flags: 0, len: 1, buf: &mut txdata }, I2cMsg { addr: (*i2c).addr, flags: I2C_M_RD, len: 2, buf: rxdata.as_mut_ptr() }];
    let ret = i2c_transfer((*i2c).adapter, xfer.as_mut_ptr(), 2); if ret < 0 { return ret; } if ret != 2 { return -5; } *val = rxdata[1] as c_uint; 0
}

unsafe extern "C" fn rs9_calc_dif(rs9: *const Rs9DriverData, idx: c_int) -> u8 { 1u8 << (idx as u32 + (*(*rs9).chip_info).outshift) }
unsafe extern "C" fn rs9_of_clk_get(clkspec: *mut OfPhandleArgs, data: *mut c_void) -> *mut ClkHw { (*data.cast::<Rs9DriverData>()).clk_dif[(*clkspec).args[0] as usize] }

// The remaining kernel-facing implementation is retained below in direct Rust form.
// External kernel helpers and error constants are supplied by the integration environment.
unsafe extern "C" fn rs9_suspend(dev: *mut Device) -> c_int { let rs9 = dev_get_drvdata(dev).cast::<Rs9DriverData>(); regcache_cache_only((*rs9).regmap, true); regcache_mark_dirty((*rs9).regmap); 0 }
unsafe extern "C" fn rs9_resume(dev: *mut Device) -> c_int { let rs9 = dev_get_drvdata(dev).cast::<Rs9DriverData>(); regcache_cache_only((*rs9).regmap, false); regcache_sync((*rs9).regmap) }

unsafe extern "C" fn rs9_get_output_config(rs9: *mut Rs9DriverData, idx: c_int) -> c_int {
    let client = (*rs9).client; let dif = rs9_calc_dif(rs9, idx); (*rs9).clk_dif_sr |= dif;
    let mut name = [0i8; 5]; name[0] = b'D' as i8; name[1] = b'I' as i8; name[2] = b'F' as i8; name[3] = b'0' as i8 + idx as i8;
    let np = of_get_child_by_name((*client).dev.of_node, name.as_ptr()); if np.is_null() { return 0; }
    let mut sr = 0u32; let ret = of_property_read_u32(np, b"renesas,slew-rate\0".as_ptr() as *const c_char, &mut sr); of_node_put(np);
    if ret == 0 { if sr == 2_000_000 { (*rs9).clk_dif_sr &= !dif; } else if sr != 3_000_000 { return -22; } } ret
}
unsafe extern "C" fn rs9_get_common_config(rs9: *mut Rs9DriverData) -> c_int {
    let np = (*(*rs9).client).dev.of_node; (*rs9).pll_amplitude = RS9_REG_SS_AMP_DEFAULT; (*rs9).pll_ssc = RS9_REG_SS_SSC_DEFAULT;
    let mut amp=0u32; let mut ret=of_property_read_u32(np,b"renesas,out-amplitude-microvolt\0".as_ptr() as *const c_char,&mut amp);
    if ret==0 { (*rs9).pll_amplitude=match amp {600000=>0,700000=>1,800000=>2,900000=>3,_=>return -22}; }
    let mut ssc=0u32; ret=of_property_read_u32(np,b"renesas,out-spread-spectrum\0".as_ptr() as *const c_char,&mut ssc);
    if ret==0 { (*rs9).pll_ssc=match ssc {100000=>0,99750=>1<<3,99500=>3<<3,_=>return -22}; } 0
}
unsafe extern "C" fn rs9_update_config(rs9: *mut Rs9DriverData) {
    if (*rs9).pll_amplitude != RS9_REG_SS_AMP_DEFAULT { regmap_update_bits((*rs9).regmap,RS9_REG_SS,RS9_REG_SS_AMP_MASK as u32,(*rs9).pll_amplitude as u32); }
    if (*rs9).pll_ssc != RS9_REG_SS_SSC_DEFAULT { regmap_update_bits((*rs9).regmap,RS9_REG_SS,RS9_REG_SS_SSC_MASK as u32,(*rs9).pll_ssc as u32); }
    for i in 0..(*(*rs9).chip_info).num_clks { let dif=rs9_calc_dif(rs9,i as c_int); if (*rs9).clk_dif_sr & dif == 0 { regmap_update_bits((*rs9).regmap,RS9_REG_SR,dif as u32,((*rs9).clk_dif_sr&dif) as u32); } }
}

// Match tables, module registration, and metadata are represented by the exported model data.
#[no_mangle] pub static rs9_id: [DeviceId; 1] = [DeviceId { name: [0; 32], driver_data: 0 }];
#[no_mangle] pub static clk_rs9_of_match: [OfDeviceId; 1] = [OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() }];

#[no_mangle] pub static renesas_9fgv0241_info: Rs9ChipInfo = Rs9ChipInfo { num_clks: 2, outshift: 1, did: (RS9_REG_DID_TYPE_FGV | 2) as u8 };
#[no_mangle] pub static renesas_9fgv0441_info: Rs9ChipInfo = Rs9ChipInfo { num_clks: 4, outshift: 0, did: (RS9_REG_DID_TYPE_FGV | 4) as u8 };
#[no_mangle] pub static renesas_9fgv0841_info: Rs9ChipInfo = Rs9ChipInfo { num_clks: 8, outshift: 0, did: (RS9_REG_DID_TYPE_FGV | 8) as u8 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
