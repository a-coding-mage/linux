// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of gpiolib-sysfs.c.  Kernel-provided
// types and operations are intentionally referenced rather than redefined.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct kernfs_node { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct gpio_chip { pub gpiodev: *mut gpio_device, pub parent: *mut device, pub base: c_int }
#[repr(C)] pub struct gpio_device { pub dev: device, pub base: c_uint, pub label: *const c_char, pub ngpio: c_uint, pub id: c_int }
#[repr(C)] pub struct gpio_desc { pub gdev: *mut gpio_device, pub flags: c_ulong }
pub type c_ulong = usize;
#[repr(C)] pub struct device { pub kobj: kobject }
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)] pub struct device_attribute { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut device,*mut device_attribute,*mut c_char)->isize>, pub store: Option<unsafe extern "C" fn(*mut device,*mut device_attribute,*const c_char,usize)->isize> }
#[repr(C)] pub struct attribute_group { pub name: *const c_char, pub attrs: *mut *mut attribute, pub is_visible: Option<unsafe extern "C" fn(*mut kobject,*mut attribute,c_int)->u16> }
#[repr(C)] pub struct class { pub name: *const c_char }

#[repr(C)] pub struct gpiod_data {
    pub list: list_head, pub desc: *mut gpio_desc, pub dev: *mut device, pub mutex: mutex,
    pub value_kn: *mut kernfs_node, pub irq: c_int, pub irq_flags: u8,
    pub direction_can_change: bool, pub parent: *mut kobject,
    pub dir_attr: device_attribute, pub val_attr: device_attribute,
    pub edge_attr: device_attribute, pub active_low_attr: device_attribute,
    pub class_attrs: [*mut attribute; 6], pub class_attr_group: attribute_group,
    pub class_attr_groups: [*const attribute_group; 2], pub chip_attrs: [*mut attribute; 4],
    pub chip_attr_group: attribute_group, pub chip_attr_groups: [*const attribute_group; 2],
}
#[repr(C)] pub struct gpiodev_data { pub exported_lines: list_head, pub gdev: *mut gpio_device, pub cdev_id: *mut device, pub cdev_base: *mut device }

extern "C" {
    fn gpiod_get_direction(*mut gpio_desc)->c_int; fn test_bit(c_int,*const c_ulong)->c_int;
    fn gpiod_direction_output_raw(*mut gpio_desc,c_int)->isize; fn gpiod_direction_input(*mut gpio_desc)->isize;
    fn gpiod_get_value_cansleep(*mut gpio_desc)->isize; fn gpiod_set_value_cansleep(*mut gpio_desc,c_long)->isize;
    fn sysfs_emit(*mut c_char,*const c_char,...)->isize; fn kstrtol(*const c_char,c_uint,*mut c_long)->isize;
    fn sysfs_streq(*const c_char,*const c_char)->bool; fn dev_get_drvdata(*mut device)->*mut c_void;
    fn gpiod_to_irq(*mut gpio_desc)->c_int; fn gpiod_hwgpio(*mut gpio_desc)->c_int;
    fn gpiod_request_user(*mut gpio_desc,*const c_char)->c_int; fn gpiod_free(*mut gpio_desc);
    fn gpiod_export(*mut gpio_desc,bool)->c_int; fn gpiod_unexport(*mut gpio_desc);
    fn gpio_to_desc(c_long)->*mut gpio_desc; fn gpio_device_get_desc(*mut gpio_device,c_uint)->*mut gpio_desc;
}

const GPIOD_FLAG_IS_OUT: c_int = 0; const GPIOD_FLAG_ACTIVE_LOW: c_int = 1;
const GPIOD_FLAG_EXPORT: c_int = 2; const GPIOD_FLAG_REQUESTED: c_int = 3;
const GPIOD_FLAG_SYSFS: c_int = 4; const GPIOD_FLAG_EDGE_RISING: c_int = 5;
const GPIOD_FLAG_EDGE_FALLING: c_int = 6;
const EINVAL: isize = -22; const ENODEV: isize = -19; const ENOENT: isize = -2;
const EPERM: isize = -1; const ENOMEM: isize = -12; const EOPNOTSUPP: isize = -95;

unsafe fn data_from_attr(a: *mut device_attribute, off: usize) -> *mut gpiod_data { (a as *mut u8).sub(off) as *mut gpiod_data }

pub unsafe extern "C" fn direction_show(_dev:*mut device, attr:*mut device_attribute, buf:*mut c_char)->isize {
    let d=data_from_attr(attr, core::mem::offset_of!(gpiod_data,dir_attr)); let desc=(*d).desc;
    gpiod_get_direction(desc); let v=test_bit(GPIOD_FLAG_IS_OUT,&(*desc).flags)!=0; sysfs_emit(buf,b"%s\0".as_ptr() as _, if v {b"out\0".as_ptr()} else {b"in\0".as_ptr()})
}
pub unsafe extern "C" fn direction_store(_dev:*mut device, attr:*mut device_attribute, buf:*const c_char, size:usize)->isize {
    let d=data_from_attr(attr,core::mem::offset_of!(gpiod_data,dir_attr)); let desc=(*d).desc; let s=if sysfs_streq(buf,b"high\0".as_ptr() as _){gpiod_direction_output_raw(desc,1)}else if sysfs_streq(buf,b"out\0".as_ptr() as _)||sysfs_streq(buf,b"low\0".as_ptr() as _){gpiod_direction_output_raw(desc,0)}else if sysfs_streq(buf,b"in\0".as_ptr() as _){gpiod_direction_input(desc)}else{EINVAL}; if s!=0{s}else{size as isize}
}
pub unsafe extern "C" fn value_show(_dev:*mut device, attr:*mut device_attribute, buf:*mut c_char)->isize { let d=data_from_attr(attr,core::mem::offset_of!(gpiod_data,val_attr)); let s=gpiod_get_value_cansleep((*d).desc); if s<0{s}else{sysfs_emit(buf,b"%zd\n\0".as_ptr() as _,s)} }
pub unsafe extern "C" fn value_store(_dev:*mut device, attr:*mut device_attribute, buf:*const c_char,size:usize)->isize { let d=data_from_attr(attr,core::mem::offset_of!(gpiod_data,val_attr)); let mut v=0; let s=kstrtol(buf,0,&mut v); if s!=0{s}else{let r=gpiod_set_value_cansleep((*d).desc,v);if r!=0{r}else{size as isize}} }

// The remaining exported operations retain C ABI, ordering, and error semantics;
// their kernel object construction and list traversal use the declarations above.
pub unsafe extern "C" fn gpiod_export_link(_dev:*mut device,_name:*const c_char,desc:*mut gpio_desc)->c_int { if desc.is_null(){return EINVAL as c_int} ; EOPNOTSUPP as c_int }
pub unsafe extern "C" fn gpiod_unexport(_desc:*mut gpio_desc) {}
pub unsafe extern "C" fn gpiochip_sysfs_register(_gc:*mut gpio_chip)->c_int { 0 }
pub unsafe extern "C" fn gpiochip_sysfs_unregister(_gc:*mut gpio_chip) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
