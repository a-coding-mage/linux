// SPDX-License-Identifier: GPL-2.0
/* Rust translation of devcoredump.c; kernel dependencies are supplied externally. */

use core::ffi::c_void;

// C kernel types and functions referenced here are supplied by other translation units.
#[repr(C)] pub struct class { pub name: *const i8, pub dev_release: Option<unsafe extern "C" fn(*mut device)>, pub dev_groups: *const *const attribute_group, pub class_groups: *const *const attribute_group }
#[repr(C)] pub struct device { pub class: *mut class, pub kobj: kobject }
#[repr(C)] pub struct kobject { pub sd: *mut c_void }
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct module;
#[repr(C)] pub struct file;
#[repr(C)] pub struct bin_attribute;
#[repr(C)] pub struct attribute { pub _private: *mut c_void }
#[repr(C)] pub struct attribute_group { pub bin_attrs: *const *const bin_attribute }
#[repr(C)] pub struct scatterlist;
#[repr(C)] pub struct atomic_t(pub i32);
pub type loff_t = i64; pub type gfp_t = u32; pub type ssize_t = isize;

static mut devcd_class: class = class { name: b"devcoredump\0".as_ptr() as *const i8, dev_release: Some(devcd_dev_release), dev_groups: unsafe { devcd_dev_groups.as_ptr() }, class_groups: unsafe { devcd_class_groups.as_ptr() } };
static mut devcd_disabled: bool = false;

#[repr(C)] struct devcd_entry {
    devcd_dev: device, data: *mut c_void, datalen: usize, mutex: mutex,
    init_completed: bool, deleted: bool, owner: *mut module,
    read: Option<unsafe extern "C" fn(*mut i8, loff_t, usize, *mut c_void, usize) -> ssize_t>,
    free: Option<unsafe extern "C" fn(*mut c_void)>, del_wk: delayed_work,
    failing_dev: *mut device,
}

unsafe fn dev_to_devcd(dev: *mut device) -> *mut devcd_entry { dev as *mut devcd_entry }

unsafe extern "C" fn devcd_dev_release(dev: *mut device) {
    let devcd = dev_to_devcd(dev); ((*devcd).free.unwrap())((*devcd).data); module_put((*devcd).owner);
    if !(*(*devcd).failing_dev).kobj.sd.is_null() { sysfs_delete_link(&mut (*(*devcd).failing_dev).kobj, &mut (*dev).kobj, b"devcoredump\0".as_ptr() as *const i8); }
    put_device((*devcd).failing_dev); kfree(devcd as *mut c_void);
}
unsafe fn __devcd_del(devcd: *mut devcd_entry) { (*devcd).deleted=true; device_del(&mut (*devcd).devcd_dev); put_device(&mut (*devcd).devcd_dev); }
unsafe extern "C" fn devcd_del(wk: *mut work_struct) { let d=wk as *mut devcd_entry; mutex_lock(&mut (*d).mutex); let i=(*d).init_completed; mutex_unlock(&mut (*d).mutex); if i { __devcd_del(d); } }
unsafe extern "C" fn devcd_data_read(_: *mut file, k: *mut kobject, _: *const bin_attribute, b:*mut i8, o:loff_t, c:usize)->ssize_t { let d=dev_to_devcd(k as *mut device); ((*d).read.unwrap())(b,o,c,(*d).data,(*d).datalen) }
unsafe extern "C" fn devcd_data_write(_: *mut file, k:*mut kobject, _: *const bin_attribute, _:*mut i8, _:loff_t, c:usize)->ssize_t { let d=dev_to_devcd(k as *mut device); if cancel_delayed_work(&mut (*d).del_wk)!=0 { schedule_delayed_work(&mut (*d).del_wk,0); } c as ssize_t }

static devcd_attr_data: bin_attribute = bin_attribute { _private: core::ptr::null_mut() };
static devcd_dev_bin_attrs: [*const bin_attribute;2] = [&devcd_attr_data, core::ptr::null()];
static devcd_dev_group: attribute_group = attribute_group { bin_attrs: devcd_dev_bin_attrs.as_ptr() };
static devcd_dev_groups: [*const attribute_group;2] = [&devcd_dev_group, core::ptr::null()];
static devcd_class_groups: [*const attribute_group;1] = [core::ptr::null()];

unsafe extern "C" fn devcd_free(dev:*mut device, _: *mut c_void)->i32 { let d=dev_to_devcd(dev); disable_delayed_work_sync(&mut (*d).del_wk); mutex_lock(&mut (*d).mutex); if !(*d).deleted { __devcd_del(d); } mutex_unlock(&mut (*d).mutex); 0 }
unsafe extern "C" fn disabled_show(_: *const class, _: *const c_void, b:*mut i8)->ssize_t { sysfs_emit(b,b"%d\n\0".as_ptr() as *const i8, devcd_disabled as i32) }
unsafe extern "C" fn disabled_store(_: *const class, _: *const c_void, b:*const i8, c:usize)->ssize_t { if simple_strtol(b,core::ptr::null_mut(),10)!=1 { return -22; } devcd_disabled=true; class_for_each_device(&mut devcd_class,core::ptr::null_mut(),core::ptr::null_mut(),Some(devcd_free)); c as ssize_t }

unsafe extern "C" fn devcd_readv(b:*mut i8,o:loff_t,c:usize,d:*mut c_void,l:usize)->ssize_t { memory_read_from_buffer(b,c,&mut (o as loff_t),d,l) }
unsafe extern "C" fn devcd_freev(d:*mut c_void){ vfree(d); }
pub unsafe extern "C" fn dev_coredumpv(dev:*mut device,data:*mut c_void,len:usize,g:gfp_t){ dev_coredumpm(dev,core::ptr::null_mut(),data,len,g,Some(devcd_readv),Some(devcd_freev)); }
unsafe extern "C" fn devcd_match_failing(dev:*mut device, f:*const c_void)->i32 { ( (*dev_to_devcd(dev)).failing_dev == f as *mut device) as i32 }
unsafe extern "C" fn devcd_free_sgtable(d:*mut c_void){ _devcd_free_sgtable(d); }
unsafe extern "C" fn devcd_read_from_sgtable(b:*mut i8,o:loff_t,mut n:usize,d:*mut c_void,l:usize)->ssize_t { if o>l {return -22;} if o+n as i64>l {n=l-o as usize;} sg_pcopy_to_buffer(d as *mut scatterlist,sg_nents(d as *mut scatterlist),b,n,o as usize) }
pub unsafe extern "C" fn dev_coredump_put(dev:*mut device){let e=class_find_device(&mut devcd_class,core::ptr::null_mut(),dev as *mut c_void,Some(devcd_match_failing));if !e.is_null(){devcd_free(e,core::ptr::null_mut());put_device(e);}}

// The remaining public framework entry point retains the C control flow and delegates to kernel APIs.
pub unsafe extern "C" fn dev_coredumpm_timeout(dev:*mut device, owner:*mut module, data:*mut c_void, datalen:usize, gfp:gfp_t, read:Option<unsafe extern "C" fn(*mut i8,loff_t,usize,*mut c_void,usize)->ssize_t>, free:Option<unsafe extern "C" fn(*mut c_void)>, timeout:usize) { if devcd_disabled {free.unwrap()(data);return;} let e=class_find_device(&mut devcd_class,core::ptr::null_mut(),dev as *mut c_void,Some(devcd_match_failing));if !e.is_null(){put_device(e);free.unwrap()(data);return;}if try_module_get(owner)==0 {free.unwrap()(data);return;}let d=kzalloc(core::mem::size_of::<devcd_entry>(),gfp) as *mut devcd_entry;if d.is_null(){module_put(owner);free.unwrap()(data);return;}(*d).owner=owner;(*d).data=data;(*d).datalen=datalen;(*d).read=read;(*d).free=free;(*d).failing_dev=get_device(dev);mutex_init(&mut (*d).mutex);device_initialize(&mut (*d).devcd_dev);(*d).devcd_dev.class=&mut devcd_class;mutex_lock(&mut (*d).mutex);INIT_DELAYED_WORK(&mut (*d).del_wk,Some(devcd_del));schedule_delayed_work(&mut (*d).del_wk,timeout);if device_add(&mut (*d).devcd_dev)!=0 {mutex_unlock(&mut (*d).mutex);cancel_delayed_work_sync(&mut (*d).del_wk);put_device(&mut (*d).devcd_dev);module_put(owner);free.unwrap()(data);return;}(*d).init_completed=true;mutex_unlock(&mut (*d).mutex);}
pub unsafe extern "C" fn dev_coredumpsg(dev:*mut device,t:*mut scatterlist,l:usize,g:gfp_t){dev_coredumpm(dev,core::ptr::null_mut(),t as *mut c_void,l,g,Some(devcd_read_from_sgtable),Some(devcd_free_sgtable));}

extern "C" { fn module_put(*mut module); fn try_module_get(*mut module)->i32; fn sysfs_delete_link(*mut kobject,*mut kobject,*const i8); fn put_device(*mut device); fn kfree(*mut c_void); fn device_del(*mut device); fn mutex_lock(*mut mutex); fn mutex_unlock(*mut mutex); fn cancel_delayed_work(*mut delayed_work)->i32; fn schedule_delayed_work(*mut delayed_work,usize); fn disable_delayed_work_sync(*mut delayed_work); fn sysfs_emit(*mut i8,*const i8,...)->ssize_t; fn simple_strtol(*const i8,*mut *mut i8,i32)->i64; fn class_for_each_device(*mut class,*mut c_void,*mut c_void,Option<unsafe extern "C" fn(*mut device,*mut c_void)->i32>); fn memory_read_from_buffer(*mut i8,usize,*mut loff_t,*mut c_void,usize)->ssize_t; fn vfree(*mut c_void); fn dev_coredumpm(*mut device,*mut module,*mut c_void,usize,gfp_t,Option<unsafe extern "C" fn(*mut i8,loff_t,usize,*mut c_void,usize)->ssize_t>,Option<unsafe extern "C" fn(*mut c_void)>); fn _devcd_free_sgtable(*mut c_void); fn sg_nents(*mut scatterlist)->i32; fn sg_pcopy_to_buffer(*mut scatterlist,i32,*mut i8,usize,usize)->ssize_t; fn class_find_device(*mut class,*mut c_void,*mut c_void,Option<unsafe extern "C" fn(*mut device,*const c_void)->i32>)->*mut device; fn kzalloc(usize,gfp_t)->*mut c_void; fn mutex_init(*mut mutex); fn device_initialize(*mut device); fn INIT_DELAYED_WORK(*mut delayed_work,Option<unsafe extern "C" fn(*mut work_struct)>); fn cancel_delayed_work_sync(*mut delayed_work); fn device_add(*mut device)->i32; fn get_device(*mut device)->*mut device; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
