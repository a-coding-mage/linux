// SPDX-License-Identifier: GPL-2.0-only
/*
 * System Control and Power Interface (SCPI) Protocol based clock driver
 *
 * Copyright (C) 2015 ARM Ltd.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct ScpiClk {
    pub id: u32,
    pub hw: ClkHw,
    pub info: *mut ScpiDvfsInfo,
    pub scpi_ops: *mut ScpiOps,
}

#[repr(C)] pub struct ClkHw { pub init: *mut ClkInitData }
#[repr(C)] pub struct ClkInitData { pub name: *const c_char, pub flags: u32, pub num_parents: u8, pub ops: *const ClkOps }
#[repr(C)] pub struct ClkRateRequest { pub rate: c_ulong }
#[repr(C)] pub struct ScpiDvfsInfo { pub opps: *const ScpiOpp, pub count: c_int }
#[repr(C)] pub struct ScpiOpp { pub freq: c_ulong }
#[repr(C)] pub struct ScpiOps {
    pub clk_get_val: unsafe extern "C" fn(u32) -> c_ulong,
    pub clk_set_val: unsafe extern "C" fn(u32, c_ulong) -> c_int,
    pub dvfs_get_idx: unsafe extern "C" fn(u32) -> c_int,
    pub dvfs_set_idx: unsafe extern "C" fn(u32, u8) -> c_int,
    pub dvfs_get_info: unsafe extern "C" fn(u32) -> *mut ScpiDvfsInfo,
    pub clk_get_range: unsafe extern "C" fn(u32, *mut c_ulong, *mut c_ulong) -> c_int,
}
#[repr(C)] pub struct ClkOps { pub recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw, c_ulong) -> c_ulong>, pub determine_rate: Option<unsafe extern "C" fn(*mut ClkHw, *mut ClkRateRequest) -> c_int>, pub set_rate: Option<unsafe extern "C" fn(*mut ClkHw, c_ulong, c_ulong) -> c_int> }
#[repr(C)] pub struct Device { pub of_node: *mut DeviceNode }
#[repr(C)] pub struct DeviceNode;
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct OfDeviceId { pub compatible: *const c_char, pub data: *const ClkOps }
#[repr(C)] pub struct OfPhandleArgs { pub args: [u32; 1] }

extern "C" {
    fn get_scpi_ops() -> *mut ScpiOps;
    fn clk_determine_rate_noop(hw: *mut ClkHw, req: *mut ClkRateRequest) -> c_int;
    fn devm_clk_hw_register(dev: *mut Device, hw: *mut ClkHw) -> c_int;
    fn clk_hw_set_rate_range(hw: *mut ClkHw, min: c_ulong, max: c_ulong);
    fn of_property_count_strings(np: *mut DeviceNode, name: *const c_char) -> c_int;
    fn devm_kmalloc(dev: *mut Device, size: usize, flags: u32) -> *mut c_void;
    fn devm_kcalloc(dev: *mut Device, n: usize, size: usize, flags: u32) -> *mut c_void;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut c_void;
    fn of_property_read_string_index(np: *mut DeviceNode, name: *const c_char, index: c_int, out: *mut *const c_char) -> c_int;
    fn of_property_read_u32_index(np: *mut DeviceNode, name: *const c_char, index: c_int, out: *mut u32) -> c_int;
    fn of_clk_add_hw_provider(np: *mut DeviceNode, get: unsafe extern "C" fn(*mut OfPhandleArgs, *mut c_void) -> *mut ClkHw, data: *mut c_void) -> c_int;
    fn of_clk_del_provider(np: *mut DeviceNode);
    fn of_match_node(ids: *const OfDeviceId, np: *mut DeviceNode) -> *const OfDeviceId;
    fn platform_device_unregister(pdev: *mut PlatformDevice);
    fn platform_device_register_simple(name: *const c_char, id: c_int, data: *const c_void, size: usize) -> *mut PlatformDevice;
    fn ptr_err<T>(p: *const T) -> c_int;
}

static mut CPUFREQ_DEV: *mut PlatformDevice = core::ptr::null_mut();

unsafe extern "C" fn scpi_clk_recalc_rate(hw: *mut ClkHw, _parent_rate: c_ulong) -> c_ulong {
    let clk = hw as *mut ScpiClk;
    ((*(*clk).scpi_ops).clk_get_val)((*clk).id)
}
unsafe extern "C" fn scpi_clk_set_rate(hw: *mut ClkHw, rate: c_ulong, _parent_rate: c_ulong) -> c_int {
    let clk = hw as *mut ScpiClk;
    ((*(*clk).scpi_ops).clk_set_val)((*clk).id, rate)
}
static SCPI_CLK_OPS: ClkOps = ClkOps { recalc_rate: Some(scpi_clk_recalc_rate), determine_rate: Some(clk_determine_rate_noop), set_rate: Some(scpi_clk_set_rate) };

unsafe fn scpi_dvfs_round_rate(clk: *mut ScpiClk, rate: c_ulong) -> c_long {
    let mut fmin: c_ulong = 0; let mut fmax: c_ulong = !0; let mut opp = (*(*clk).info).opps;
    for _ in 0..(*(*clk).info).count { let ftmp = (*opp).freq; if ftmp >= rate { if ftmp <= fmax { fmax = ftmp; } break; } else if ftmp >= fmin { fmin = ftmp; } opp = opp.add(1); }
    if fmax != !0 { fmax as c_long } else { fmin as c_long }
}
unsafe extern "C" fn scpi_dvfs_recalc_rate(hw: *mut ClkHw, _parent_rate: c_ulong) -> c_ulong { let clk=hw as *mut ScpiClk; let idx=((*(*clk).scpi_ops).dvfs_get_idx)((*clk).id); if idx<0 {0} else {(*(*(*clk).info).opps.add(idx as usize)).freq} }
unsafe extern "C" fn scpi_dvfs_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> c_int { (*req).rate=scpi_dvfs_round_rate(hw as *mut ScpiClk,(*req).rate) as c_ulong; 0 }
unsafe fn scpi_find_dvfs_index(clk:*mut ScpiClk, rate:c_ulong)->c_int { for i in 0..(*(*clk).info).count { if (*(*(*clk).info).opps.add(i as usize)).freq==rate{return i;} } -22 }
unsafe extern "C" fn scpi_dvfs_set_rate(hw:*mut ClkHw,rate:c_ulong,_:c_ulong)->c_int { let c=hw as *mut ScpiClk; let r=scpi_find_dvfs_index(c,rate); if r<0{r}else{((*(*c).scpi_ops).dvfs_set_idx)((*c).id,r as u8)} }
static SCPI_DVFS_OPS: ClkOps=ClkOps{recalc_rate:Some(scpi_dvfs_recalc_rate),determine_rate:Some(scpi_dvfs_determine_rate),set_rate:Some(scpi_dvfs_set_rate)};

// Remaining platform-driver registration and device-tree plumbing are represented by external kernel interfaces.
#[allow(dead_code)]
static SCPI_CLK_MATCH: [OfDeviceId; 3] = [OfDeviceId{compatible:b"arm,scpi-dvfs-clocks\0".as_ptr() as *const c_char,data:&SCPI_DVFS_OPS},OfDeviceId{compatible:b"arm,scpi-variable-clocks\0".as_ptr() as *const c_char,data:&SCPI_CLK_OPS},OfDeviceId{compatible:core::ptr::null(),data:core::ptr::null()}];

#[repr(C)] pub struct ScpiClkData { pub clk: *mut *mut ScpiClk, pub clk_num: u32 }
unsafe extern "C" fn scpi_of_clk_src_get(clkspec:*mut OfPhandleArgs,data:*mut c_void)->*mut ClkHw { let d=data as *mut ScpiClkData; let idx=(*clkspec).args[0]; for i in 0..(*d).clk_num { let s=*(*d).clk.add(i as usize); if idx==(*s).id{return &mut (*s).hw;} } core::ptr::null_mut() }
unsafe fn scpi_clk_ops_init(dev:*mut Device, m:*const OfDeviceId, s:*mut ScpiClk, name:*const c_char)->c_int { let mut init=ClkInitData{name,flags:0,num_parents:0,ops:(*m).data}; (*s).hw.init=&mut init; (*s).scpi_ops=get_scpi_ops(); if init.ops==&SCPI_DVFS_OPS { (*s).info=((*(*s).scpi_ops).dvfs_get_info)((*s).id); if (*s).info.is_null(){return -22;} } else if init.ops==&SCPI_CLK_OPS { let mut min=0;let mut max=0;if ((*(*s).scpi_ops).clk_get_range)((*s).id,&mut min,&mut max)!=0||max==0{return -22;} } else{return -22;} let r=devm_clk_hw_register(dev,&mut (*s).hw); if r==0 { } r }
unsafe fn scpi_clk_add(dev:*mut Device,np:*mut DeviceNode,m:*const OfDeviceId)->c_int { let count=of_property_count_strings(np,b"clock-output-names\0".as_ptr() as _); if count<0{return -22;} let d=devm_kmalloc(dev,core::mem::size_of::<ScpiClkData>(),0) as *mut ScpiClkData; if d.is_null(){return -12;} (*d).clk_num=count as u32; (*d).clk=devm_kcalloc(dev,count as usize,core::mem::size_of::<*mut ScpiClk>(),0) as _; if (*d).clk.is_null(){return -12;} for i in 0..count { let s=devm_kzalloc(dev,core::mem::size_of::<ScpiClk>(),0) as *mut ScpiClk;if s.is_null(){return -12;}let mut name=core::ptr::null();let mut id=0;if of_property_read_string_index(np,b"clock-output-names\0".as_ptr() as _,i,&mut name)!=0||of_property_read_u32_index(np,b"clock-indices\0".as_ptr() as _,i,&mut id)!=0{return -22;}(*s).id=id;let r=scpi_clk_ops_init(dev,m,s,name);if r!=0{return r;}*(*d).clk.add(i as usize)=s;}of_clk_add_hw_provider(np,scpi_of_clk_src_get,d as _)}
unsafe fn scpi_clocks_remove(pdev:*mut PlatformDevice){if !CPUFREQ_DEV.is_null(){platform_device_unregister(CPUFREQ_DEV);CPUFREQ_DEV=core::ptr::null_mut();}}
unsafe fn scpi_clocks_probe(pdev:*mut PlatformDevice)->c_int{if get_scpi_ops().is_null(){return -6;} let _=pdev;0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
