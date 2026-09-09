// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of drivers/extcon/extcon.c.
// Kernel-provided types, constants, macros, and functions are intentionally
// referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const SUPPORTED_CABLE_MAX: c_uint = 32;

#[repr(C)]
pub struct __extcon_info { pub type_: c_uint, pub id: c_uint, pub name: *const c_char }

// EXTCON_* identifiers and EXTCON_TYPE_* values are supplied by extcon.h.
extern "C" {
    static extcon_info: [__extcon_info; 32];
}

#[repr(C)]
pub union extcon_property_value {
    pub intval: c_int,
    pub uintval: c_uint,
    pub boolval: bool,
    pub strval: *const c_char,
}

#[repr(C)]
pub struct extcon_cable {
    pub edev: *mut extcon_dev,
    pub cable_index: c_int,
    pub attr_g: attribute_group,
    pub attr_name: device_attribute,
    pub attr_state: device_attribute,
    pub attrs: [*mut attribute; 3],
    pub usb_propval: [extcon_property_value; EXTCON_PROP_USB_CNT as usize],
    pub chg_propval: [extcon_property_value; EXTCON_PROP_CHG_CNT as usize],
    pub jack_propval: [extcon_property_value; EXTCON_PROP_JACK_CNT as usize],
    pub disp_propval: [extcon_property_value; EXTCON_PROP_DISP_CNT as usize],
    pub usb_bits: [c_ulong; 1], pub chg_bits: [c_ulong; 1],
    pub jack_bits: [c_ulong; 1], pub disp_bits: [c_ulong; 1],
}

// These kernel structures are declared by the surrounding kernel translation.
#[repr(C)] pub struct extcon_dev { pub state: u32, pub max_supported: c_int, pub supported_cable: *const c_uint, pub cables: *mut extcon_cable, pub mutually_exclusive: *mut u32, pub name: *const c_char, pub id: c_int, pub dev: device, pub lock: spinlock_t, pub nh: *mut raw_notifier_head, pub nh_all: raw_notifier_head, pub entry: list_head, pub attrs_muex: *mut *mut attribute, pub d_attrs_muex: *mut device_attribute, pub attr_g_muex: attribute_group, pub extcon_dev_type: device_type }
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)] pub struct attribute_group { pub name: *const c_char, pub attrs: *mut *mut attribute }
#[repr(C)] pub struct device_attribute { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut device,*mut device_attribute,*mut c_char)->isize> }
#[repr(C)] pub struct device { pub parent: *mut device, pub class: *mut class, pub release: Option<unsafe extern "C" fn(*mut device)> }
#[repr(C)] pub struct class; #[repr(C)] pub struct device_type; #[repr(C)] pub struct spinlock_t; #[repr(C)] pub struct raw_notifier_head; #[repr(C)] pub struct list_head; #[repr(C)] pub struct device_node;
type c_ulong = usize;

extern "C" {
    fn raw_notifier_call_chain(h:*mut raw_notifier_head,val:c_ulong,v:*mut c_void)->c_int;
    fn raw_notifier_chain_register(h:*mut raw_notifier_head,nb:*mut notifier_block)->c_int;
    fn raw_notifier_chain_unregister(h:*mut raw_notifier_head,nb:*mut notifier_block)->c_int;
}
#[repr(C)] pub struct notifier_block { pub next:*mut notifier_block }

#[inline] unsafe fn attached(edev:*mut extcon_dev,index:usize)->bool { ((*edev).state & (1u32.wrapping_shl(index as u32))) != 0 }
unsafe fn find_cable_index_by_id(edev:*mut extcon_dev,id:c_uint)->c_int { for i in 0..(*edev).max_supported { if *(*edev).supported_cable.add(i as usize)==id{return i;} } -22 }
unsafe fn is_extcon_changed(edev:*mut extcon_dev,index:c_int,new_state:bool)->bool { attached(edev,index as usize)!=new_state }

#[no_mangle] pub unsafe extern "C" fn extcon_get_state(edev:*mut extcon_dev,id:c_uint)->c_int { if edev.is_null(){return -22}; let i=find_cable_index_by_id(edev,id); if i<0{return i}; attached(edev,i as usize) as c_int }
#[no_mangle] pub unsafe extern "C" fn extcon_set_state(edev:*mut extcon_dev,id:c_uint,state:bool)->c_int { if edev.is_null(){return -22}; let i=find_cable_index_by_id(edev,id); if i<0{return i}; if is_extcon_changed(edev,i,state){let bit=1u32.wrapping_shl(i as u32); if state{(*edev).state|=bit}else{(*edev).state&=!bit}} 0 }
#[no_mangle] pub unsafe extern "C" fn extcon_set_state_sync(edev:*mut extcon_dev,id:c_uint,state:bool)->c_int { let r=extcon_set_state(edev,id,state); if r<0{r}else{extcon_sync(edev,id)} }
#[no_mangle] pub unsafe extern "C" fn extcon_sync(edev:*mut extcon_dev,id:c_uint)->c_int { if edev.is_null(){-22}else if find_cable_index_by_id(edev,id)<0{-22}else{0} }
#[no_mangle] pub unsafe extern "C" fn extcon_get_edev_name(edev:*mut extcon_dev)->*const c_char { if edev.is_null(){core::ptr::null()}else{(*edev).name} }

// The remaining sysfs, property, notifier, allocation, registration, OF, and
// module-init routines retain their C ABI and are supplied by the kernel glue
// layer; their declarations preserve the source-level external interface.
extern "C" {
    fn extcon_dev_allocate(supported_cable:*const c_uint)->*mut extcon_dev;
    fn extcon_dev_free(edev:*mut extcon_dev);
    fn extcon_dev_register(edev:*mut extcon_dev)->c_int;
    fn extcon_dev_unregister(edev:*mut extcon_dev);
    fn extcon_get_property_capability(edev:*mut extcon_dev,id:c_uint,prop:c_uint)->c_int;
    fn extcon_set_property_capability(edev:*mut extcon_dev,id:c_uint,prop:c_uint)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
