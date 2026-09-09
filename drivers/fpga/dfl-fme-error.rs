// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for FPGA Management Engine Error Management
 *
 * Copyright 2019 Intel Corporation, Inc.
 *
 * Authors:
 *   Kang Luwei <luwei.kang@intel.com>
 *   Xiao Guangrong <guangrong.xiao@linux.intel.com>
 *   Wu Hao <hao.wu@intel.com>
 *   Joseph Grecco <joe.grecco@intel.com>
 *   Enno Luebbers <enno.luebbers@intel.com>
 *   Tim Whisonant <tim.whisonant@intel.com>
 *   Ananda Ravuri <ananda.ravuri@intel.com>
 *   Mitchel, Henry <henry.mitchel@intel.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const FME_ERROR_MASK: usize = 0x8;
const FME_ERROR: usize = 0x10;
const MBP_ERROR: u64 = 1u64 << 6;
const PCIE0_ERROR_MASK: usize = 0x18;
const PCIE0_ERROR: usize = 0x20;
const PCIE1_ERROR_MASK: usize = 0x28;
const PCIE1_ERROR: usize = 0x30;
const FME_FIRST_ERROR: usize = 0x38;
const FME_NEXT_ERROR: usize = 0x40;
const RAS_NONFAT_ERROR_MASK: usize = 0x48;
const RAS_NONFAT_ERROR: usize = 0x50;
const RAS_CATFAT_ERROR_MASK: usize = 0x58;
const RAS_CATFAT_ERROR: usize = 0x60;
const RAS_ERROR_INJECT: usize = 0x68;
const INJECT_ERROR_MASK: u64 = 0x7;
const ERROR_MASK: u64 = u64::MAX;

extern "C" {
    fn to_dfl_feature_dev_data(dev: *mut device) -> *mut dfl_feature_dev_data;
    fn dfl_get_feature_ioaddr_by_id(fdata: *mut dfl_feature_dev_data, id: u64) -> *mut u8;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn readq(addr: *mut u8) -> u64;
    fn writeq(value: u64, addr: *mut u8);
    fn kstrtou64(buf: *const i8, base: u32, out: *mut u64) -> i32;
    fn kstrtou8(buf: *const i8, base: u32, out: *mut u8) -> i32;
    fn dfl_feature_revision(base: *mut u8) -> u32;
    fn dfl_get_feature_by_id(fdata: *mut dfl_feature_dev_data, id: u64) -> *mut dfl_feature;
    fn kobj_to_dev(kobj: *mut kobject) -> *mut device;
    fn dfl_feature_ioctl_get_num_irqs(pdev: *mut platform_device, feature: *mut dfl_feature, arg: usize) -> i64;
    fn dfl_feature_ioctl_set_irq(pdev: *mut platform_device, feature: *mut dfl_feature, arg: usize) -> i64;
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> isize;
    fn dev_dbg(dev: *mut device, fmt: *const i8, ...);
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { pub attr: attribute }
#[repr(C)] pub struct attribute { pub mode: u16 }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct dfl_feature_dev_data { pub lock: mutex }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct dfl_feature { _private: [u8; 0] }
#[repr(C)] pub struct dfl_feature_id { pub id: u64 }
#[repr(C)] pub struct dfl_feature_ops {
    pub init: Option<unsafe extern "C" fn(*mut platform_device, *mut dfl_feature) -> i32>,
    pub uinit: Option<unsafe extern "C" fn(*mut platform_device, *mut dfl_feature)>,
    pub ioctl: Option<unsafe extern "C" fn(*mut platform_device, *mut dfl_feature, u32, usize) -> i64>,
}
#[repr(C)] pub struct attribute_group {
    pub name: *const i8,
    pub attrs: *mut *mut attribute,
    pub is_visible: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, i32) -> u16>,
}

extern "C" {
    static dev_attr_pcie0_errors: device_attribute;
    static dev_attr_pcie1_errors: device_attribute;
    static dev_attr_nonfatal_errors: device_attribute;
    static dev_attr_catfatal_errors: device_attribute;
    static dev_attr_inject_errors: device_attribute;
    static dev_attr_fme_errors: device_attribute;
    static dev_attr_first_error: device_attribute;
    static dev_attr_next_error: device_attribute;
    static FME_FEATURE_ID_GLOBAL_ERR: u64;
    static DFL_FPGA_FME_ERR_GET_IRQ_NUM: u32;
    static DFL_FPGA_FME_ERR_SET_IRQ: u32;
}

unsafe fn show_hex(buf: *mut i8, value: u64) -> isize {
    sprintf(buf, b"0x%llx\0".as_ptr() as *const i8, value)
}

#[no_mangle]
pub unsafe extern "C" fn pcie0_errors_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize {
    let fdata = to_dfl_feature_dev_data(dev);
    let base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    mutex_lock(&mut (*fdata).lock); let value = readq(base.add(PCIE0_ERROR)); mutex_unlock(&mut (*fdata).lock);
    show_hex(buf, value)
}

#[no_mangle]
pub unsafe extern "C" fn pcie0_errors_store(dev: *mut device, _attr: *mut device_attribute, buf: *const i8, count: usize) -> isize {
    let fdata = to_dfl_feature_dev_data(dev); let mut val = 0u64;
    if kstrtou64(buf, 0, &mut val) != 0 { return -22; }
    let base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR); mutex_lock(&mut (*fdata).lock);
    writeq(ERROR_MASK, base.add(PCIE0_ERROR_MASK)); let v = readq(base.add(PCIE0_ERROR)); let ret = if val == v { writeq(v, base.add(PCIE0_ERROR)); 0 } else { -22 };
    writeq(0, base.add(PCIE0_ERROR_MASK)); mutex_unlock(&mut (*fdata).lock); if ret != 0 { ret } else { count as isize }
}

#[no_mangle]
pub unsafe extern "C" fn pcie1_errors_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize {
    let fdata = to_dfl_feature_dev_data(dev); let base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    mutex_lock(&mut (*fdata).lock); let value = readq(base.add(PCIE1_ERROR)); mutex_unlock(&mut (*fdata).lock); show_hex(buf, value)
}

#[no_mangle]
pub unsafe extern "C" fn pcie1_errors_store(dev: *mut device, _attr: *mut device_attribute, buf: *const i8, count: usize) -> isize {
    let fdata = to_dfl_feature_dev_data(dev); let mut val = 0u64; if kstrtou64(buf, 0, &mut val) != 0 { return -22; }
    let base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR); mutex_lock(&mut (*fdata).lock); writeq(ERROR_MASK, base.add(PCIE1_ERROR_MASK)); let v = readq(base.add(PCIE1_ERROR)); let ret = if val == v { writeq(v, base.add(PCIE1_ERROR)); 0 } else { -22 }; writeq(0, base.add(PCIE1_ERROR_MASK)); mutex_unlock(&mut (*fdata).lock); if ret != 0 { ret } else { count as isize }
}

unsafe fn simple_show(dev: *mut device, buf: *mut i8, offset: usize) -> isize { let fdata = to_dfl_feature_dev_data(dev); let base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR); show_hex(buf, readq(base.add(offset))) }
pub unsafe extern "C" fn nonfatal_errors_show(d: *mut device, _: *mut device_attribute, b: *mut i8) -> isize { simple_show(d,b,RAS_NONFAT_ERROR) }
pub unsafe extern "C" fn catfatal_errors_show(d: *mut device, _: *mut device_attribute, b: *mut i8) -> isize { simple_show(d,b,RAS_CATFAT_ERROR) }
pub unsafe extern "C" fn first_error_show(d: *mut device, _: *mut device_attribute, b: *mut i8) -> isize { simple_show(d,b,FME_FIRST_ERROR) }
pub unsafe extern "C" fn next_error_show(d: *mut device, _: *mut device_attribute, b: *mut i8) -> isize { simple_show(d,b,FME_NEXT_ERROR) }

pub unsafe extern "C" fn inject_errors_show(dev: *mut device, _: *mut device_attribute, buf: *mut i8) -> isize { let f=to_dfl_feature_dev_data(dev); let b=dfl_get_feature_ioaddr_by_id(f,FME_FEATURE_ID_GLOBAL_ERR); mutex_lock(&mut (*f).lock); let v=readq(b.add(RAS_ERROR_INJECT)); mutex_unlock(&mut (*f).lock); show_hex(buf,(v & INJECT_ERROR_MASK)) }
pub unsafe extern "C" fn inject_errors_store(dev: *mut device, _: *mut device_attribute, buf: *const i8, count: usize) -> isize { let f=to_dfl_feature_dev_data(dev); let mut e=0u8; if kstrtou8(buf,0,&mut e)!=0 || (e as u64 & !INJECT_ERROR_MASK)!=0{return -22}; let b=dfl_get_feature_ioaddr_by_id(f,FME_FEATURE_ID_GLOBAL_ERR); mutex_lock(&mut (*f).lock); let mut v=readq(b.add(RAS_ERROR_INJECT)); v &= !INJECT_ERROR_MASK; v |= e as u64; writeq(v,b.add(RAS_ERROR_INJECT)); mutex_unlock(&mut (*f).lock); count as isize }

pub unsafe extern "C" fn fme_errors_show(d:*mut device,_:*mut device_attribute,b:*mut i8)->isize{let f=to_dfl_feature_dev_data(d);let x=dfl_get_feature_ioaddr_by_id(f,FME_FEATURE_ID_GLOBAL_ERR);mutex_lock(&mut(*f).lock);let v=readq(x.add(FME_ERROR));mutex_unlock(&mut(*f).lock);show_hex(b,v)}
pub unsafe extern "C" fn fme_errors_store(d:*mut device,_:*mut device_attribute,s:*const i8,n:usize)->isize{let f=to_dfl_feature_dev_data(d);let mut val=0; if kstrtou64(s,0,&mut val)!=0{return -22};let x=dfl_get_feature_ioaddr_by_id(f,FME_FEATURE_ID_GLOBAL_ERR);mutex_lock(&mut(*f).lock);writeq(ERROR_MASK,x.add(FME_ERROR_MASK));let v=readq(x.add(FME_ERROR));let r=if val==v{writeq(v,x.add(FME_ERROR));0}else{-22};writeq(if dfl_feature_revision(x)!=0{0}else{MBP_ERROR},x.add(FME_ERROR_MASK));mutex_unlock(&mut(*f).lock);if r!=0{r}else{n as isize}}

pub unsafe extern "C" fn fme_global_err_attrs_visible(kobj:*mut kobject, attr:*mut attribute, _:i32)->u16 {
    let dev=kobj_to_dev(kobj); let f=to_dfl_feature_dev_data(dev);
    if dfl_get_feature_by_id(f,FME_FEATURE_ID_GLOBAL_ERR).is_null(){0}else{(*attr).mode}
}

#[no_mangle] pub static mut fme_global_err_attrs:[*mut attribute;9]=[
    &dev_attr_pcie0_errors.attr as *const attribute as *mut attribute,
    &dev_attr_pcie1_errors.attr as *const attribute as *mut attribute,
    &dev_attr_nonfatal_errors.attr as *const attribute as *mut attribute,
    &dev_attr_catfatal_errors.attr as *const attribute as *mut attribute,
    &dev_attr_inject_errors.attr as *const attribute as *mut attribute,
    &dev_attr_fme_errors.attr as *const attribute as *mut attribute,
    &dev_attr_first_error.attr as *const attribute as *mut attribute,
    &dev_attr_next_error.attr as *const attribute as *mut attribute,
    core::ptr::null_mut(),
];
#[no_mangle] pub static fme_global_err_group:attribute_group=attribute_group{name:b"errors\0".as_ptr() as *const i8,attrs:fme_global_err_attrs.as_ptr() as *mut *mut attribute,is_visible:Some(fme_global_err_attrs_visible)};

unsafe fn fme_err_mask(dev:*mut device, mask:bool){let f=to_dfl_feature_dev_data(dev);let b=dfl_get_feature_ioaddr_by_id(f,FME_FEATURE_ID_GLOBAL_ERR);mutex_lock(&mut(*f).lock);let fe=if mask{ERROR_MASK}else{0};writeq(if dfl_feature_revision(b)!=0{fe}else{if mask{ERROR_MASK}else{MBP_ERROR}},b.add(FME_ERROR_MASK));for o in [PCIE0_ERROR_MASK,PCIE1_ERROR_MASK,RAS_NONFAT_ERROR_MASK,RAS_CATFAT_ERROR_MASK]{writeq(fe,b.add(o));}mutex_unlock(&mut(*f).lock)}
pub unsafe extern "C" fn fme_global_err_init(p:*mut platform_device,_:*mut dfl_feature)->i32{fme_err_mask(&mut(*p).dev,false);0}
pub unsafe extern "C" fn fme_global_err_uinit(p:*mut platform_device,_:*mut dfl_feature){fme_err_mask(&mut(*p).dev,true)}
pub unsafe extern "C" fn fme_global_error_ioctl(p:*mut platform_device,f:*mut dfl_feature,c:u32,a:usize)->i64{if c==DFL_FPGA_FME_ERR_GET_IRQ_NUM{dfl_feature_ioctl_get_num_irqs(p,f,a)}else if c==DFL_FPGA_FME_ERR_SET_IRQ{dfl_feature_ioctl_set_irq(p,f,a)}else{-19}}

#[no_mangle] pub static mut fme_global_err_id_table:[dfl_feature_id;2]=[dfl_feature_id{id:0},dfl_feature_id{id:0}];
#[no_mangle] pub static fme_global_err_ops:dfl_feature_ops=dfl_feature_ops{init:Some(fme_global_err_init),uinit:Some(fme_global_err_uinit),ioctl:Some(fme_global_error_ioctl)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
