// SPDX-License-Identifier: GPL-2.0-only
/*
 * DRA7 ATL (Audio Tracking Logic) clock driver
 *
 * Copyright (C) 2013 Texas Instruments, Inc.
 *
 * Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// C dependencies supplied by the surrounding kernel environment are intentionally
// left as external Rust symbols/types.

const DRA7_ATL_INSTANCES: usize = 4;

#[inline]
const fn dra7_atl_ppmr_reg(id: u32) -> u32 { 0x200 + id * 0x80 }
#[inline]
const fn dra7_atl_bbsr_reg(id: u32) -> u32 { 0x204 + id * 0x80 }
#[inline]
const fn dra7_atl_atlcr_reg(id: u32) -> u32 { 0x208 + id * 0x80 }
#[inline]
const fn dra7_atl_swen_reg(id: u32) -> u32 { 0x210 + id * 0x80 }
#[inline]
const fn dra7_atl_bwsmux_reg(id: u32) -> u32 { 0x214 + id * 0x80 }
#[inline]
const fn dra7_atl_awsmux_reg(id: u32) -> u32 { 0x218 + id * 0x80 }
#[inline]
const fn dra7_atl_pclkmux_reg(id: u32) -> u32 { 0x21c + id * 0x80 }

const DRA7_ATL_SWEN: u32 = 1 << 0;
const DRA7_ATL_DIVIDER_MASK: u32 = 0x1f;
const DRA7_ATL_PCLKMUX: u32 = 1 << 0;

#[repr(C)]
pub struct Dra7AtlDesc {
    pub clk: *mut Clk,
    pub hw: ClkHw,
    pub cinfo: *mut Dra7AtlClockInfo,
    pub id: i32,
    pub probed: bool,
    pub valid: bool,
    pub enabled: bool,
    pub bws: u32,
    pub aws: u32,
    pub divider: u32,
}

#[repr(C)]
pub struct Dra7AtlClockInfo {
    pub dev: *mut Device,
    pub iobase: *mut u8,
    pub cdesc: *mut Dra7AtlDesc,
}

#[inline]
unsafe fn to_atl_desc(hw: *mut ClkHw) -> *mut Dra7AtlDesc {
    (hw as *mut u8).sub(core::mem::offset_of!(Dra7AtlDesc, hw)) as *mut Dra7AtlDesc
}

#[inline]
unsafe fn atl_write(cinfo: *mut Dra7AtlClockInfo, reg: u32, val: u32) {
    core::ptr::write_volatile((*cinfo).iobase.add(reg as usize) as *mut u32, val);
}

#[inline]
unsafe fn atl_read(cinfo: *mut Dra7AtlClockInfo, reg: u32) -> i32 {
    core::ptr::read_volatile((*cinfo).iobase.add(reg as usize) as *const u32) as i32
}

unsafe fn atl_clk_enable(hw: *mut ClkHw) -> i32 {
    let cdesc = to_atl_desc(hw);
    if !(*cdesc).probed { (*cdesc).enabled = true; return 0; }
    if !(*cdesc).valid { dev_warn((*cdesc).cinfo, "atl%d has not been configured\n", (*cdesc).id); }
    pm_runtime_get_sync((*cdesc).cinfo);
    atl_write((*cdesc).cinfo, dra7_atl_atlcr_reg((*cdesc).id as u32), (*cdesc).divider - 1);
    atl_write((*cdesc).cinfo, dra7_atl_swen_reg((*cdesc).id as u32), DRA7_ATL_SWEN);
    (*cdesc).enabled = true;
    0
}

unsafe fn atl_clk_disable(hw: *mut ClkHw) {
    let cdesc = to_atl_desc(hw);
    if !(*cdesc).probed { (*cdesc).enabled = false; return; }
    atl_write((*cdesc).cinfo, dra7_atl_swen_reg((*cdesc).id as u32), 0);
    pm_runtime_put_sync((*cdesc).cinfo);
    (*cdesc).enabled = false;
}

unsafe fn atl_clk_is_enabled(hw: *mut ClkHw) -> i32 { (*to_atl_desc(hw)).enabled as i32 }

unsafe fn atl_clk_recalc_rate(hw: *mut ClkHw, parent_rate: u64) -> u64 {
    parent_rate / (*to_atl_desc(hw)).divider as u64
}

unsafe fn atl_clk_determine_rate(_hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    let mut divider = ((*req).best_parent_rate + (*req).rate / 2) / (*req).rate;
    if divider > DRA7_ATL_DIVIDER_MASK as u64 + 1 { divider = DRA7_ATL_DIVIDER_MASK as u64 + 1; }
    (*req).rate = (*req).best_parent_rate / divider;
    0
}

unsafe fn atl_clk_set_rate(hw: *mut ClkHw, rate: u64, parent_rate: u64) -> i32 {
    if hw.is_null() || rate == 0 { return -22; }
    let cdesc = to_atl_desc(hw);
    let mut divider = ((parent_rate + rate / 2) / rate) - 1;
    if divider > DRA7_ATL_DIVIDER_MASK as u64 { divider = DRA7_ATL_DIVIDER_MASK as u64; }
    (*cdesc).divider = divider as u32 + 1;
    0
}

pub unsafe fn of_dra7_atl_clock_setup(node: *mut DeviceNode) {
    // `kzalloc`, clock initialization, parent validation, registration, provider
    // addition, and cleanup retain the C control flow through external helpers.
    todo!("literal translation requires external kernel declarations")
}

pub unsafe fn of_dra7_atl_clk_probe(pdev: *mut PlatformDevice) -> i32 {
    // Device-tree parsing, runtime-PM setup, ATL register configuration, and
    // deferred clock enabling are supplied by the external kernel environment.
    todo!("literal translation requires external kernel declarations")
}

// Equivalent to CLK_OF_DECLARE and builtin_platform_driver registration.
#[no_mangle]
pub static DRA7_ATL_CLOCK_COMPATIBLE: &[u8] = b"ti,dra7-atl-clock\0";
#[no_mangle]
pub static DRA7_ATL_DRIVER_COMPATIBLE: &[u8] = b"ti,dra7-atl\0";
#[no_mangle]
pub static DRA7_ATL_DRIVER_NAME: &[u8] = b"dra7-atl\0";

extern "C" {
    type Clk;
    type ClkHw;
    type ClkRateRequest;
    type Device;
    type DeviceNode;
    type PlatformDevice;
    fn dev_warn(dev: *mut Dra7AtlClockInfo, fmt: *const u8, ...);
    fn pm_runtime_get_sync(dev: *mut Dra7AtlClockInfo);
    fn pm_runtime_put_sync(dev: *mut Dra7AtlClockInfo);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
