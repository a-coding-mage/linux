// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2016-2022 HabanaLabs, Ltd.
// All Rights Reserved.

// Dependency declarations supplied by goyaP.h and the kernel are intentionally
// left as external types/functions.
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

type SizeT = usize;
type SsizeT = isize;

#[repr(C)]
pub struct HlDevice {
    pub asic_specific: *mut c_void,
    pub pdev: *mut c_void,
    pub high_pll: u32,
    pub dev: *mut c_void,
    pub fpriv_list_lock: c_void,
    pub is_compute_ctx_active: bool,
    pub asic_prop: AsicProperties,
}

#[repr(C)]
pub struct AsicProperties { pub cpucp_info: CpucpInfo }
#[repr(C)]
pub struct CpucpInfo { pub infineon_version: u32 }
#[repr(C)]
pub struct GoyaDevice {
    pub pm_mng_profile: c_int,
    pub curr_pll_profile: c_int,
    pub mme_clk: c_long,
    pub tpc_clk: c_long,
    pub ic_clk: c_long,
    pub goya_work: *mut GoyaWork,
}
#[repr(C)] pub struct GoyaWork { pub work_freq: c_void }
#[repr(C)] pub struct Device;
#[repr(C)] pub struct DeviceAttribute;
#[repr(C)] pub struct Attribute { _private: [u8; 0] }
#[repr(C)] pub struct AttributeGroup { pub attrs: *mut *mut Attribute }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HlPllFrequency { PLL_HIGH, PLL_LOW, PLL_LAST }

const PM_AUTO: c_int = 0;
const PM_MANUAL: c_int = 1;
const HL_GOYA_MME_PLL: c_int = 0;
const HL_GOYA_TPC_PLL: c_int = 1;
const HL_GOYA_IC_PLL: c_int = 2;
const GOYA_PLL_FREQ_LOW: c_ulong = 0;
const ENODEV: SsizeT = 19;
const EPERM: SsizeT = 1;
const EINVAL: SsizeT = 22;

extern "C" {
    fn hl_fw_set_frequency(hdev: *mut HlDevice, pll: c_int, value: c_ulong);
    fn hl_fw_get_frequency(hdev: *mut HlDevice, pll: c_int, curr: bool) -> c_long;
    fn hl_device_operational(hdev: *mut HlDevice, data: *mut c_void) -> bool;
    fn dev_get_drvdata(dev: *mut Device) -> *mut HlDevice;
    fn kstrtoul(buf: *const c_char, base: c_uint, value: *mut c_long) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn goya_set_frequency(hdev: *mut HlDevice, profile: HlPllFrequency);
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn flush_delayed_work(work: *mut c_void);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn le32_to_cpu(value: u32) -> u32;
}

pub unsafe fn goya_set_pll_profile(hdev: *mut HlDevice, freq: HlPllFrequency) {
    let goya = (*hdev).asic_specific as *mut GoyaDevice;
    if (*hdev).pdev.is_null() { return; }
    match freq {
        HlPllFrequency::PLL_HIGH => {
            hl_fw_set_frequency(hdev, HL_GOYA_MME_PLL, (*hdev).high_pll as c_ulong);
            hl_fw_set_frequency(hdev, HL_GOYA_TPC_PLL, (*hdev).high_pll as c_ulong);
            hl_fw_set_frequency(hdev, HL_GOYA_IC_PLL, (*hdev).high_pll as c_ulong);
        }
        HlPllFrequency::PLL_LOW => {
            hl_fw_set_frequency(hdev, HL_GOYA_MME_PLL, GOYA_PLL_FREQ_LOW);
            hl_fw_set_frequency(hdev, HL_GOYA_TPC_PLL, GOYA_PLL_FREQ_LOW);
            hl_fw_set_frequency(hdev, HL_GOYA_IC_PLL, GOYA_PLL_FREQ_LOW);
        }
        HlPllFrequency::PLL_LAST => {
            hl_fw_set_frequency(hdev, HL_GOYA_MME_PLL, (*goya).mme_clk as c_ulong);
            hl_fw_set_frequency(hdev, HL_GOYA_TPC_PLL, (*goya).tpc_clk as c_ulong);
            hl_fw_set_frequency(hdev, HL_GOYA_IC_PLL, (*goya).ic_clk as c_ulong);
        }
    }
}

unsafe fn clock_show(dev: *mut Device, buf: *mut c_char, pll: c_int, curr: bool) -> SsizeT {
    let hdev = dev_get_drvdata(dev);
    if !hl_device_operational(hdev, core::ptr::null_mut()) { return -ENODEV; }
    let value = hl_fw_get_frequency(hdev, pll, curr);
    if value < 0 { return value as SsizeT; }
    sprintf(buf, b"%lu\0".as_ptr() as *const c_char, value as c_ulong) as SsizeT
}

unsafe fn clock_store(dev: *mut Device, buf: *const c_char, count: SizeT, pll: c_int, field: *mut c_long) -> SsizeT {
    let hdev = dev_get_drvdata(dev);
    let goya = (*hdev).asic_specific as *mut GoyaDevice;
    let mut count = count as SsizeT;
    if !hl_device_operational(hdev, core::ptr::null_mut()) { return -ENODEV; }
    if (*goya).pm_mng_profile == PM_AUTO { return -EPERM; }
    let mut value = 0;
    if kstrtoul(buf, 0, &mut value) != 0 { return -EINVAL; }
    hl_fw_set_frequency(hdev, pll, value as c_ulong);
    *field = value;
    count
}

unsafe fn mme_clk_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut c_char) -> SsizeT { clock_show(dev, buf, HL_GOYA_MME_PLL, false) }
unsafe fn tpc_clk_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut c_char) -> SsizeT { clock_show(dev, buf, HL_GOYA_TPC_PLL, false) }
unsafe fn ic_clk_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut c_char) -> SsizeT { clock_show(dev, buf, HL_GOYA_IC_PLL, false) }
unsafe fn mme_clk_curr_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut c_char) -> SsizeT { clock_show(dev, buf, HL_GOYA_MME_PLL, true) }
unsafe fn tpc_clk_curr_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut c_char) -> SsizeT { clock_show(dev, buf, HL_GOYA_TPC_PLL, true) }
unsafe fn ic_clk_curr_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut c_char) -> SsizeT { clock_show(dev, buf, HL_GOYA_IC_PLL, true) }
unsafe fn mme_clk_store(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *const c_char, count: SizeT) -> SsizeT { let h=dev_get_drvdata(dev); clock_store(dev,buf,count,HL_GOYA_MME_PLL,&mut (*(h).asic_specific as *mut GoyaDevice).mme_clk) }
unsafe fn tpc_clk_store(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *const c_char, count: SizeT) -> SsizeT { let h=dev_get_drvdata(dev); clock_store(dev,buf,count,HL_GOYA_TPC_PLL,&mut (*(h).asic_specific as *mut GoyaDevice).tpc_clk) }
unsafe fn ic_clk_store(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *const c_char, count: SizeT) -> SsizeT { let h=dev_get_drvdata(dev); clock_store(dev,buf,count,HL_GOYA_IC_PLL,&mut (*(h).asic_specific as *mut GoyaDevice).ic_clk) }

unsafe fn pm_mng_profile_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut c_char) -> SsizeT {
    let hdev=dev_get_drvdata(dev); if !hl_device_operational(hdev, core::ptr::null_mut()) { return -ENODEV; }
    let goya=(*hdev).asic_specific as *mut GoyaDevice;
    let text=if (*goya).pm_mng_profile==PM_AUTO { b"auto\n\0" } else if (*goya).pm_mng_profile==PM_MANUAL { b"manual\n\0" } else { b"unknown\n\0" };
    sprintf(buf, b"%s\0".as_ptr() as *const c_char, text.as_ptr()) as SsizeT
}

unsafe fn high_pll_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut c_char) -> SsizeT { let h=dev_get_drvdata(dev); if !hl_device_operational(h,core::ptr::null_mut()){return -ENODEV;} sprintf(buf,b"%u\n\0".as_ptr() as *const c_char,(*h).high_pll) as SsizeT }

unsafe fn high_pll_store(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *const c_char, count: SizeT) -> SsizeT {
    let h=dev_get_drvdata(dev); let mut value=0; if !hl_device_operational(h,core::ptr::null_mut()){return -ENODEV;}
    if kstrtoul(buf,0,&mut value)!=0{return -EINVAL;} (*h).high_pll=value as u32; count as SsizeT
}

unsafe fn pm_mng_profile_store(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *const c_char, count: SizeT) -> SsizeT {
    let h=dev_get_drvdata(dev); let g=(*h).asic_specific as *mut GoyaDevice; let mut count=count as SsizeT;
    if !hl_device_operational(h,core::ptr::null_mut()){return -ENODEV;}
    mutex_lock(&mut (*h).fpriv_list_lock);
    if (*h).is_compute_ctx_active { dev_err((*h).dev,b"Can't change PM profile while compute context is opened on the device\n\0".as_ptr() as *const c_char); count=-EPERM; mutex_unlock(&mut (*h).fpriv_list_lock); return count; }
    // The kernel implementation compares the input prefix with "auto" and "manual".
    let is_auto=core::slice::from_raw_parts(buf as *const u8,4)==b"auto";
    let is_manual=core::slice::from_raw_parts(buf as *const u8,6)==b"manual";
    if is_auto { if (*g).pm_mng_profile==PM_MANUAL { (*g).curr_pll_profile=0; (*g).pm_mng_profile=PM_AUTO; goya_set_frequency(h,HlPllFrequency::PLL_LOW); } }
    else if is_manual { if (*g).pm_mng_profile==PM_AUTO { (*g).pm_mng_profile=PM_MANUAL; mutex_unlock(&mut (*h).fpriv_list_lock); if !(*g).goya_work.is_null(){flush_delayed_work(&mut (*(*g).goya_work).work_freq);} return count; } }
    else { dev_err((*h).dev,b"value should be auto or manual\n\0".as_ptr() as *const c_char); count=-EINVAL; }
    mutex_unlock(&mut (*h).fpriv_list_lock); count
}

unsafe fn infineon_ver_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut c_char) -> SsizeT {
    let h=dev_get_drvdata(dev); let v=le32_to_cpu((*h).asic_prop.cpucp_info.infineon_version); sprintf(buf,b"%#04x\n\0".as_ptr() as *const c_char,v) as SsizeT
}

// DEVICE_ATTR_* declarations and the attribute arrays are kernel sysfs metadata.
pub unsafe fn goya_add_device_attr(hdev: *mut HlDevice, dev_clk_attr_grp: *mut AttributeGroup, dev_vrm_attr_grp: *mut AttributeGroup) { let _=hdev; (*dev_clk_attr_grp).attrs=core::ptr::null_mut(); (*dev_vrm_attr_grp).attrs=core::ptr::null_mut(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
