// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2026 Intel Corporation */

// Declarations supplied by the surrounding kernel/QAT translation units.
use core::ffi::{c_char, c_int, c_uint, c_void};

type ssize_t = isize;
type size_t = usize;

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct device_attribute {
    pub attr: attribute,
}
#[repr(C)]
pub struct attribute;
#[repr(C)]
pub struct attribute_group {
    pub attrs: *mut *mut attribute,
    pub name: *const c_char,
}
#[repr(C)]
pub struct pci_dev;
#[repr(C)]
pub struct adf_accel_dev;
#[repr(C)]
pub struct adf_hw_device_data;
#[repr(C)]
pub struct adf_kpt_interface_data;
#[repr(C)]
pub struct adf_kpt_hw_data;

extern "C" {
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn adf_devmgr_pci_to_accel_dev(dev: *mut pci_dev) -> *mut adf_accel_dev;
    fn adf_dev_started(accel_dev: *mut adf_accel_dev) -> bool;
    fn adf_get_service_enabled(accel_dev: *mut adf_accel_dev) -> c_int;
    fn kstrtobool(buf: *const c_char, value: *mut bool) -> c_int;
    fn kstrtouint(buf: *const c_char, base: c_uint, value: *mut c_uint) -> c_int;
    fn sysfs_emit(buf: *mut c_char, format: *const c_char, ...) -> ssize_t;
    fn dev_info(dev: *mut device, format: *const c_char, ...);
    fn dev_err(dev: *mut device, format: *const c_char, ...);
    fn devm_device_add_group(dev: *mut device, group: *const attribute_group) -> c_int;
}

// These accessors correspond to the C macros GET_DEV, GET_HW_DATA,
// GET_KPT_USER_DATA, and GET_KPT_CFG_DATA supplied by the QAT headers.
extern "C" {
    fn GET_DEV(accel_dev: *mut adf_accel_dev) -> *mut device;
    fn GET_HW_DATA(accel_dev: *mut adf_accel_dev) -> *mut adf_hw_device_data;
    fn GET_KPT_USER_DATA(accel_dev: *mut adf_accel_dev) -> *mut adf_kpt_interface_data;
    fn GET_KPT_CFG_DATA(accel_dev: *mut adf_accel_dev) -> *mut adf_kpt_hw_data;
}

const EINVAL: ssize_t = -22;
const SVC_ASYM: c_int = 1;

extern "C" {
    fn adf_hw_get_accel_cap(accel_dev: *mut adf_accel_dev) -> usize;
}

#[inline]
unsafe fn enable_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return EINVAL; }
    let user_data = GET_KPT_USER_DATA(accel_dev);
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, (*user_data).enable as c_int)
}

#[inline]
unsafe fn enable_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return EINVAL; }
    if adf_dev_started(accel_dev) { dev_info(dev, b"Device qat_dev%d must be down before enabling KPT\n\0".as_ptr() as *const c_char, (*accel_dev).accel_id); return EINVAL; }
    if adf_get_service_enabled(accel_dev) != SVC_ASYM { dev_info(dev, b"KPT can only be enabled when the asymmetric service is enabled\n\0".as_ptr() as *const c_char); return EINVAL; }
    let hw_data = GET_HW_DATA(accel_dev);
    (*hw_data).accel_capabilities_mask = adf_hw_get_accel_cap(accel_dev);
    if (*hw_data).accel_capabilities_mask == 0 { return EINVAL; }
    let mut enable = false;
    let ret = kstrtobool(buf, &mut enable);
    if ret != 0 { return ret as ssize_t; }
    (*GET_KPT_USER_DATA(accel_dev)).enable = enable;
    count as ssize_t
}

#[inline]
unsafe fn swk_shared_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return EINVAL; }
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, (*GET_KPT_USER_DATA(accel_dev)).swk_shared as c_int)
}

#[inline]
unsafe fn swk_shared_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return EINVAL; }
    if adf_dev_started(accel_dev) { dev_info(dev, b"Device qat_dev%d must be down before setting swk_shared\n\0".as_ptr() as *const c_char, (*accel_dev).accel_id); return EINVAL; }
    let mut value = false;
    let ret = kstrtobool(buf, &mut value);
    if ret != 0 { return ret as ssize_t; }
    (*GET_KPT_USER_DATA(accel_dev)).swk_shared = value;
    count as ssize_t
}

unsafe fn swk_max_ttl_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let a=adf_devmgr_pci_to_accel_dev(to_pci_dev(dev)); if a.is_null(){return EINVAL;} sysfs_emit(buf,b"%u\n\0".as_ptr() as *const c_char,(*GET_KPT_USER_DATA(a)).swk_max_ttl) }
unsafe fn swk_max_ttl_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t { let a=adf_devmgr_pci_to_accel_dev(to_pci_dev(dev)); if a.is_null(){return EINVAL;} if adf_dev_started(a){dev_info(dev,b"Device qat_dev%d must be down before setting swk_max_ttl\n\0".as_ptr() as *const c_char,(*a).accel_id);return EINVAL;} let mut v=0; let r=kstrtouint(buf,10,&mut v); if r!=0{return r as ssize_t;} let k=GET_KPT_CFG_DATA(a); if v>(*k).max_swk_ttl{dev_info(dev,b"Configuration value is out of range (%u - %u)\n\0".as_ptr() as *const c_char,0,(*k).max_swk_ttl);return EINVAL;} (*k).user_input.swk_max_ttl=v; count as ssize_t }
unsafe fn swk_cnt_per_fn_show(dev:*mut device,_:*mut device_attribute,buf:*mut c_char)->ssize_t{let a=adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));if a.is_null(){return EINVAL;}sysfs_emit(buf,b"%u\n\0".as_ptr() as *const c_char,(*GET_KPT_USER_DATA(a)).swk_cnt_per_fn)}
unsafe fn swk_cnt_per_fn_store(dev:*mut device,_:*mut device_attribute,buf:*const c_char,count:size_t)->ssize_t{let a=adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));if a.is_null(){return EINVAL;}if adf_dev_started(a){dev_info(dev,b"Device qat_dev%d must be down before setting swk_cnt_per_fn\n\0".as_ptr() as *const c_char,(*a).accel_id);return EINVAL;}let mut v=0;let r=kstrtouint(buf,10,&mut v);if r!=0{return r as ssize_t;}let k=GET_KPT_CFG_DATA(a);if v>(*k).max_swk_cnt_per_fn_pasid{dev_info(dev,b"swk_cnt_per_fn: value out of range (0 - %u)\n\0".as_ptr() as *const c_char,(*k).max_swk_cnt_per_fn_pasid);return EINVAL;}(*k).user_input.swk_cnt_per_fn=v;count as ssize_t}
unsafe fn swk_cnt_per_pasid_show(dev:*mut device,_:*mut device_attribute,buf:*mut c_char)->ssize_t{let a=adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));if a.is_null(){return EINVAL;}sysfs_emit(buf,b"%u\n\0".as_ptr() as *const c_char,(*GET_KPT_USER_DATA(a)).swk_cnt_per_pasid)}
unsafe fn swk_cnt_per_pasid_store(dev:*mut device,_:*mut device_attribute,buf:*const c_char,count:size_t)->ssize_t{let a=adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));if a.is_null(){return EINVAL;}if adf_dev_started(a){dev_info(dev,b"Device qat_dev%d must be down before setting swk_cnt_per_pasid\n\0".as_ptr() as *const c_char,(*a).accel_id);return EINVAL;}let mut v=0;let r=kstrtouint(buf,10,&mut v);if r!=0{return r as ssize_t;}let k=GET_KPT_CFG_DATA(a);if v>(*k).max_swk_cnt_per_fn_pasid{dev_info(dev,b"swk_cnt_per_pasid: value out of range (0 - %u)\n\0".as_ptr() as *const c_char,(*k).max_swk_cnt_per_fn_pasid);return EINVAL;}(*k).user_input.swk_cnt_per_pasid=v;count as ssize_t}

pub unsafe fn adf_sysfs_init_kpt(accel_dev: *mut adf_accel_dev) -> c_int {
    let ret = devm_device_add_group(GET_DEV(accel_dev), &qat_kpt_group);
    if ret != 0 { dev_err(GET_DEV(accel_dev), b"Failed to create qat_kpt attribute group\n\0".as_ptr() as *const c_char); return ret; }
    0
}

// DEVICE_ATTR_RW declarations and the attribute array are provided by the kernel
// sysfs binding layer in the final repository integration.
extern "C" {
    static qat_kpt_group: attribute_group;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
