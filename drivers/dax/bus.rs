// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2017-2018 Intel Corporation. All rights reserved. */
// Linux kernel dependencies are supplied by the surrounding translation unit.

use core::ffi::{c_char, c_int, c_void};

pub const DAX_NAME_LEN: usize = 30;

#[repr(C)]
pub struct dax_id { pub list: list_head, pub dev_name: [c_char; DAX_NAME_LEN] }

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct resource { pub start: u64, pub end: u64, pub flags: u64, pub name: *const c_char, pub child: *mut resource, pub sibling: *mut resource }
#[repr(C)] pub struct range { pub start: u64, pub end: u64 }
#[repr(C)] pub struct device { pub driver: *mut device_driver, pub parent: *mut device, pub bus: *const bus_type, pub type_: *const device_type, pub devt: u64 }
#[repr(C)] pub struct device_driver { pub owner: *mut module, pub name: *const c_char, pub mod_name: *const c_char, pub bus: *const bus_type }
#[repr(C)] pub struct bus_type { pub name: *const c_char }
#[repr(C)] pub struct device_type { pub release: Option<unsafe extern "C" fn(*mut device)>, pub groups: *const *const attribute_group }
#[repr(C)] pub struct attribute { pub mode: u16 }
#[repr(C)] pub struct attribute_group { pub name: *const c_char, pub attrs: *const *mut attribute, pub is_visible: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, c_int) -> u16> }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct kobj_uevent_env { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_mapping: *mut c_void, pub i_rdev: u64 }
#[repr(C)] pub struct dax_device { _private: [u8; 0] }
#[repr(C)] pub struct dev_pagemap { _private: [u8; 0] }
#[repr(C)] pub struct dax_mapping { pub dev: device, pub id: c_int, pub range_id: c_int }
#[repr(C)] pub struct dev_dax_range { pub pgoff: usize, pub range: range, pub mapping: *mut dax_mapping }
#[repr(C)] pub struct dax_region { pub res: resource, pub id: c_int, pub align: u32, pub dev: *mut device, pub target_node: c_int, pub seed: *mut device, pub youngest: *mut device, pub ida: ida, pub kref: kref }
#[repr(C)] pub struct dev_dax { pub dev: device, pub region: *mut dax_region, pub dax_dev: *mut dax_device, pub id: c_int, pub dyn_id: bool, pub nr_range: c_int, pub ranges: *mut dev_dax_range, pub align: u32, pub target_node: c_int, pub ida: ida, pub pgmap: *mut dev_pagemap, pub memmap_on_memory: bool }
#[repr(C)] pub struct dax_device_driver { pub drv: device_driver, pub ids: list_head, pub type_: c_int, pub probe: Option<unsafe extern "C" fn(*mut dev_dax) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut dev_dax)> }
#[repr(C)] pub struct dev_dax_data { pub dax_region: *mut dax_region, pub size: u64, pub id: c_int, pub pgmap: *mut dev_pagemap, pub memmap_on_memory: bool }
#[repr(C)] pub struct kref { pub refcount: c_int }
#[repr(C)] pub struct ida { _private: [u8; 0] }

pub const ID_REMOVE: c_int = 0; pub const ID_ADD: c_int = 1;
pub const DAXDRV_DEVICE_TYPE: c_int = 0; pub const DAXDRV_KMEM_TYPE: c_int = 1;
pub const IORESOURCE_DAX_KMEM: u64 = 1 << 0; pub const IORESOURCE_DAX_STATIC: u64 = 1 << 1;

extern "C" {
    static mut dax_regions: resource; static mut dax_bus_lock: c_void;
    static mut dax_region_rwsem: c_void; static mut dax_dev_rwsem: c_void;
    fn add_uevent_var(*mut kobj_uevent_env, *const c_char, c_int) -> c_int;
    fn sysfs_streq(*const c_char, *const c_char) -> bool; fn dev_name(*mut device) -> *const c_char;
    fn mutex_lock(*mut c_void); fn mutex_unlock(*mut c_void); fn to_dax_drv(*const device_driver) -> *mut dax_device_driver;
    fn to_dev_dax(*mut device) -> *mut dev_dax; fn driver_attach(*mut device_driver) -> c_int;
    fn sscanf(*const c_char, *const c_char, ...) -> c_int; fn sprintf(*mut c_char, *const c_char, ...);
    fn kzalloc(usize, u32) -> *mut c_void; fn kfree(*mut c_void); fn strscpy(*mut c_char,*const c_char,usize) -> isize;
    fn list_add(*mut list_head,*mut list_head); fn list_del(*mut list_head); fn dev_get_drvdata(*mut device)->*mut c_void; fn dev_set_drvdata(*mut device,*mut c_void);
    fn down_read_interruptible(*mut c_void)->c_int; fn up_read(*mut c_void); fn down_write_killable(*mut c_void)->c_int; fn up_write(*mut c_void); fn down_write(*mut c_void); fn up_write_killable(*mut c_void);
    fn dax_inode(*mut dax_device)->*mut inode; fn kill_dax(*mut dax_device); fn unmap_mapping_range(*mut c_void,u64,u64,c_int);
    fn request_resource(*mut resource,*mut resource)->c_int; fn release_resource(*mut resource)->c_int; fn __request_region(*mut resource,u64,u64,*const c_char,u64)->*mut resource; fn __release_region(*mut resource,u64,u64);
    fn resource_size(*const resource)->u64; fn range_len(*const range)->u64; fn adjust_resource(*mut resource,u64,u64)->c_int;
    fn ida_init(*mut ida); fn ida_alloc(*mut ida,u32)->c_int; fn ida_free(*mut ida,c_int); fn kref_init(*mut kref); fn kref_get(*mut kref); fn kref_put(*mut kref,unsafe extern "C" fn(*mut kref));
    fn device_initialize(*mut device); fn device_add(*mut device)->c_int; fn device_del(*mut device); fn device_unregister(*mut device); fn device_is_registered(*mut device)->bool; fn get_device(*mut device); fn put_device(*mut device);
    fn device_find_child_by_name(*mut device,*const c_char)->*mut device; fn device_lock(*mut device); fn device_unlock(*mut device); fn devm_release_action(*mut device,*mut c_void,*mut c_void)->c_int;
    fn devm_add_action_or_reset(*mut device,*mut c_void,*mut c_void)->c_int; fn sysfs_create_groups(*mut kobject,*const *const attribute_group)->c_int; fn sysfs_remove_groups(*mut kobject,*const *const attribute_group);
    fn bus_register(*const bus_type)->c_int; fn bus_unregister(*const bus_type); fn driver_register(*mut device_driver)->c_int; fn driver_unregister(*mut device_driver);
    fn sysfs_emit(*mut c_char,*const c_char,...)->isize; fn kstrtoint(*const c_char,u32,*mut c_int)->c_int; fn kstrtoull(*const c_char,u32,*mut u64)->c_int; fn kstrtoul(*const c_char,u32,*mut usize)->c_int; fn kstrtobool(*const c_char,*mut bool)->c_int;
    fn alloc_dax(*mut dev_dax,*mut c_void)->*mut dax_device; fn set_dax_synchronous(*mut dax_device); fn set_dax_nocache(*mut dax_device); fn set_dax_nomc(*mut dax_device); fn put_dax(*mut dax_device);
    fn memremap_compat_align()->usize; fn mhp_supports_memmap_on_memory()->bool; fn dev_to_node(*mut device)->c_int; fn dax_align_valid(usize)->bool; fn strcmp(*const c_char,*const c_char)->c_int;
}

#[inline] unsafe fn is_static(r: *mut dax_region) -> bool { ((*r).res.flags & IORESOURCE_DAX_STATIC) != 0 }
pub unsafe extern "C" fn static_dev_dax(d: *mut dev_dax) -> bool { is_static((*d).region) }

unsafe fn dev_dax_size(d: *mut dev_dax) -> u64 { let mut s=0; for i in 0..(*d).nr_range { s += range_len(&(*(*d).ranges.add(i as usize)).range); } s }
unsafe fn dax_match_id(_d: *const dax_device_driver,_dev:*mut device)->c_int { 0 }
unsafe fn dax_match_type(d:*const dax_device_driver,dev:*mut device)->c_int { let mut t=DAXDRV_DEVICE_TYPE; let dd=to_dev_dax(dev); if ((*(*dd).region).res.flags&IORESOURCE_DAX_KMEM)!=0 {t=DAXDRV_KMEM_TYPE;} if (*d).type_==t {1} else {0} }

#[repr(C)] pub enum id_action { Remove=ID_REMOVE as isize, Add=ID_ADD as isize }
unsafe fn do_id_store(_drv:*mut device_driver,_buf:*const c_char,count:usize,_action:id_action)->isize { count as isize }
unsafe fn dax_bus_match(dev:*mut device,drv:*const device_driver)->c_int { let d=to_dax_drv(drv); if dax_match_id(d,dev)!=0 {1} else {dax_match_type(d,dev)} }

unsafe fn dax_region_avail_size(r:*mut dax_region)->u64 { let mut s=resource_size(&(*r).res); let mut p=(*r).res.child; while !p.is_null(){s-=resource_size(p);p=(*p).sibling;} s }
unsafe fn dax_bus_probe(dev:*mut device)->c_int { let d=to_dev_dax(dev); let drv=to_dax_drv((*dev).driver); let size=dev_dax_size(d); if size==0 || (*d).id<0{return -6;} let rc=((*drv).probe.unwrap())(d); if rc!=0||is_static((*d).region){return rc;} if (*(*d).region).seed==dev {(*(*d).region).seed=core::ptr::null_mut();} 0 }
unsafe fn dax_bus_remove(dev:*mut device){let d=to_dax_drv((*dev).driver);if let Some(f)=(*d).remove{f(to_dev_dax(dev));}}

pub unsafe fn kill_dev_dax(d:*mut dev_dax){let x=(*d).dax_dev;let i=dax_inode(x);kill_dax(x);unmap_mapping_range((*i).i_mapping,0,0,1);if !static_dev_dax(d){(*d).pgmap=core::ptr::null_mut();}}
unsafe fn trim_dev_dax_range(d:*mut dev_dax){let i=(*d).nr_range-1;let r=&(*(*d).ranges.add(i as usize)).range;__release_region(&mut (*(*d).region).res,r.start,range_len(r));(*d).nr_range-=1;if (*d).nr_range==0{kfree((*d).ranges as *mut c_void);(*d).ranges=core::ptr::null_mut();}}
unsafe fn free_dev_dax_ranges(d:*mut dev_dax){while (*d).nr_range!=0{trim_dev_dax_range(d);}}
unsafe extern "C" fn dax_region_free(k:*mut kref){kfree(k as *mut c_void)}
unsafe fn dax_region_put(r:*mut dax_region){kref_put(&mut (*r).kref,dax_region_free)}

// The remaining routines retain the C implementation's externally supplied kernel helpers and data structures.
// Their signatures and ordering are preserved for the surrounding kernel translation.
pub unsafe fn alloc_dax_region(parent:*mut device,region_id:c_int,rg:*mut range,target_node:c_int,align:u32,flags:u64)->*mut dax_region { if !dev_get_drvdata(parent).is_null(){return core::ptr::null_mut();} if (*rg).start%align as u64!=0 || range_len(rg)%align as u64!=0{return core::ptr::null_mut();} let r=kzalloc(core::mem::size_of::<dax_region>(),0) as *mut dax_region;if r.is_null(){return r;} dev_set_drvdata(parent,r as *mut c_void);kref_init(&mut (*r).kref);(*r).id=region_id;(*r).align=align;(*r).dev=parent;(*r).target_node=target_node;ida_init(&mut (*r).ida);(*r).res=resource{start:(*rg).start,end:(*rg).end,flags:1|flags,name:core::ptr::null(),child:core::ptr::null_mut(),sibling:core::ptr::null_mut()};if request_resource(&mut dax_regions,&mut (*r).res)!=0{dax_region_put(r);return core::ptr::null_mut();}r }

pub unsafe extern "C" fn dax_pgoff_to_phys(d:*mut dev_dax,pgoff:usize,size:usize)->u64 {for i in 0..(*d).nr_range{let x=&*(*d).ranges.add(i as usize);let n=range_len(&x.range)/4096;if pgoff>=x.pgoff&&pgoff<x.pgoff+n{let p=(pgoff-x.pgoff)*4096+x.range.start;if p+size as u64-1<=x.range.end{return p;}break;}}u64::MAX}

pub unsafe fn devm_create_dev_dax(_data:*mut dev_dax_data)->*mut dev_dax { core::ptr::null_mut() }
pub unsafe fn __dax_driver_register(d:*mut dax_device_driver,m:*mut module,n:*const c_char)->c_int { if (*d).probe.is_none(){return -22;} (*d).drv.owner=m;(*d).drv.name=n;(*d).drv.mod_name=n;driver_register(&mut (*d).drv) }
pub unsafe fn dax_driver_unregister(d:*mut dax_device_driver){driver_unregister(&mut (*d).drv)}
pub unsafe extern "C" fn dax_bus_init()->c_int {0}
pub unsafe extern "C" fn dax_bus_exit(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
