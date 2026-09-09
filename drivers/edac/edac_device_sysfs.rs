/* Rust translation of edac_device_sysfs.c. External kernel types and symbols
 * are intentionally left as dependencies supplied by the surrounding tree. */

const EDAC_DEVICE_SYMLINK: *const u8 = b"device\0".as_ptr();

#[repr(C)]
pub struct ctl_info_attribute { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut edac_device_ctl_info, *mut u8) -> isize>, pub store: Option<unsafe extern "C" fn(*mut edac_device_ctl_info, *const u8, usize) -> isize> }

unsafe extern "C" {
    fn sprintf(data: *mut u8, fmt: *const u8, ...) -> isize;
    fn simple_strtoul(data: *const u8, end: *mut *mut u8, base: u32) -> u64;
    fn edac_device_reset_delay_period(ctl: *mut edac_device_ctl_info, value: u64);
    fn edac_get_sysfs_subsys() -> *const bus_type;
    fn memset(dst: *mut core::ffi::c_void, value: i32, size: usize) -> *mut core::ffi::c_void;
    fn try_module_get(owner: *mut module) -> bool;
    fn module_put(owner: *mut module);
    fn bus_get_dev_root(bus: *const bus_type) -> *mut device;
    fn put_device(dev: *mut device);
    fn kobject_init_and_add(kobj: *mut kobject, typ: *mut kobj_type, parent: *mut kobject, fmt: *const u8, ...) -> i32;
    fn kobject_uevent(kobj: *mut kobject, event: i32) -> i32;
    fn kobject_put(kobj: *mut kobject);
    fn kobject_get(kobj: *mut kobject) -> *mut kobject;
    fn kobject_name(kobj: *mut kobject) -> *const u8;
    fn sysfs_create_file(kobj: *mut kobject, attr: *const attribute) -> i32;
    fn sysfs_remove_file(kobj: *mut kobject, attr: *const attribute);
    fn sysfs_create_link(kobj: *mut kobject, target: *mut kobject, name: *const u8) -> i32;
    fn sysfs_remove_link(kobj: *mut kobject, name: *const u8);
    fn __edac_device_free_ctl_info(dev: *mut edac_device_ctl_info);
    fn edac_dbg(level: i32, fmt: *const u8, ...);
}

#[repr(C)] pub struct attribute { pub name: *const u8, pub mode: u16 }
#[repr(C)] pub struct sysfs_ops { pub show: Option<unsafe extern "C" fn(*mut kobject,*mut attribute,*mut u8)->isize>, pub store: Option<unsafe extern "C" fn(*mut kobject,*mut attribute,*const u8,usize)->isize> }
#[repr(C)] pub struct kobject { pub parent: *mut kobject }
#[repr(C)] pub struct kobj_type { pub release: Option<unsafe extern "C" fn(*mut kobject)>, pub sysfs_ops: *const sysfs_ops, pub default_groups: *const attribute_group }
#[repr(C)] pub struct attribute_group;
#[repr(C)] pub struct module;
#[repr(C)] pub struct bus_type;
#[repr(C)] pub struct device { pub kobj: kobject }
#[repr(C)] pub struct counters { pub ue_count: u32, pub ce_count: u32 }
#[repr(C)] pub struct edac_device_ctl_info { pub kobj:kobject, pub log_ue:u32, pub log_ce:u32, pub panic_on_ue:u32, pub poll_msec:u32, pub owner:*mut module, pub edac_subsys:*const bus_type, pub dev_idx:i32, pub name:*const u8, pub dev:*mut device, pub instances:*mut edac_device_instance, pub nr_instances:i32, pub sysfs_attributes:*const edac_dev_sysfs_attribute }
#[repr(C)] pub struct edac_device_instance { pub kobj:kobject, pub ctl:*mut edac_device_ctl_info, pub counters:counters, pub name:*const u8, pub blocks:*mut edac_device_block, pub nr_blocks:i32 }
#[repr(C)] pub struct edac_device_block { pub kobj:kobject, pub instance:*mut edac_device_instance, pub counters:counters, pub name:*const u8, pub block_attributes:*mut edac_dev_sysfs_block_attribute, pub nr_attribs:i32 }
#[repr(C)] pub struct edac_dev_sysfs_attribute { pub attr:attribute }
#[repr(C)] pub struct edac_dev_sysfs_block_attribute { pub attr:attribute, pub show:Option<unsafe extern "C" fn(*mut kobject,*mut attribute,*mut u8)->isize> }

unsafe extern "C" fn edac_device_ctl_log_ue_show(c:*mut edac_device_ctl_info,d:*mut u8)->isize { sprintf(d,b"%u\n\0".as_ptr(),(*c).log_ue) }
unsafe extern "C" fn edac_device_ctl_log_ue_store(c:*mut edac_device_ctl_info,d:*const u8,n:usize)->isize { (*c).log_ue=(simple_strtoul(d,core::ptr::null_mut(),0)!=0) as u32;n as isize }
unsafe extern "C" fn edac_device_ctl_log_ce_show(c:*mut edac_device_ctl_info,d:*mut u8)->isize { sprintf(d,b"%u\n\0".as_ptr(),(*c).log_ce) }
unsafe extern "C" fn edac_device_ctl_log_ce_store(c:*mut edac_device_ctl_info,d:*const u8,n:usize)->isize { (*c).log_ce=(simple_strtoul(d,core::ptr::null_mut(),0)!=0) as u32;n as isize }
unsafe extern "C" fn edac_device_ctl_panic_on_ue_show(c:*mut edac_device_ctl_info,d:*mut u8)->isize { sprintf(d,b"%u\n\0".as_ptr(),(*c).panic_on_ue) }
unsafe extern "C" fn edac_device_ctl_panic_on_ue_store(c:*mut edac_device_ctl_info,d:*const u8,n:usize)->isize { (*c).panic_on_ue=(simple_strtoul(d,core::ptr::null_mut(),0)!=0) as u32;n as isize }
unsafe extern "C" fn edac_device_ctl_poll_msec_show(c:*mut edac_device_ctl_info,d:*mut u8)->isize { sprintf(d,b"%u\n\0".as_ptr(),(*c).poll_msec) }
unsafe extern "C" fn edac_device_ctl_poll_msec_store(c:*mut edac_device_ctl_info,d:*const u8,n:usize)->isize { edac_device_reset_delay_period(c,simple_strtoul(d,core::ptr::null_mut(),0));n as isize }

unsafe extern "C" fn edac_dev_ctl_info_show(_: *mut kobject, _: *mut attribute, _: *mut u8)->isize { -5 }
unsafe extern "C" fn edac_dev_ctl_info_store(_: *mut kobject, _: *mut attribute, _: *const u8, _: usize)->isize { -5 }
static DEVICE_CTL_INFO_OPS:sysfs_ops=sysfs_ops{show:Some(edac_dev_ctl_info_show),store:Some(edac_dev_ctl_info_store)};
static mut KTYPE_DEVICE_CTRL:kobj_type=kobj_type{release:Some(edac_device_ctrl_master_release),sysfs_ops:&DEVICE_CTL_INFO_OPS,default_groups:core::ptr::null()};

unsafe extern "C" fn edac_device_ctrl_master_release(k:*mut kobject){ let d=k as *mut edac_device_ctl_info; edac_dbg(4,b"control index=%d\n\0".as_ptr(),(*d).dev_idx);module_put((*d).owner);__edac_device_free_ctl_info(d); }

pub unsafe extern "C" fn edac_device_register_sysfs_main_kobj(d:*mut edac_device_ctl_info)->i32 { (*d).edac_subsys=edac_get_sysfs_subsys();memset(&mut (*d).kobj as *mut _ as *mut _,0,core::mem::size_of::<kobject>()); if !try_module_get((*d).owner){return -19}; let root=bus_get_dev_root((*d).edac_subsys); let mut e=-19; if !root.is_null(){e=kobject_init_and_add(&mut (*d).kobj,&mut KTYPE_DEVICE_CTRL,&mut (*root).kobj,b"%s\0".as_ptr(),(*d).name);put_device(root)} if e!=0{kobject_put(&mut (*d).kobj);module_put((*d).owner)}else{kobject_uevent(&mut (*d).kobj,0)} e }
pub unsafe extern "C" fn edac_device_unregister_sysfs_main_kobj(d:*mut edac_device_ctl_info){kobject_put(&mut (*d).kobj)}

unsafe extern "C" fn instance_ue_count_show(i:*mut edac_device_instance,d:*mut u8)->isize{sprintf(d,b"%u\n\0".as_ptr(),(*i).counters.ue_count)}
unsafe extern "C" fn instance_ce_count_show(i:*mut edac_device_instance,d:*mut u8)->isize{sprintf(d,b"%u\n\0".as_ptr(),(*i).counters.ce_count)}
unsafe extern "C" fn edac_device_ctrl_instance_release(k:*mut kobject){kobject_put(&mut (*(k as *mut edac_device_instance)).ctl.as_mut().unwrap().kobj)}
unsafe extern "C" fn edac_dev_instance_show(_: *mut kobject, _: *mut attribute, _: *mut u8)->isize{-5}
unsafe extern "C" fn edac_dev_instance_store(_: *mut kobject, _: *mut attribute, _: *const u8, _: usize)->isize{-5}
static DEVICE_INSTANCE_OPS:sysfs_ops=sysfs_ops{show:Some(edac_dev_instance_show),store:Some(edac_dev_instance_store)};
static mut KTYPE_INSTANCE_CTRL:kobj_type=kobj_type{release:Some(edac_device_ctrl_instance_release),sysfs_ops:&DEVICE_INSTANCE_OPS,default_groups:core::ptr::null()};

unsafe extern "C" fn block_ue_count_show(k:*mut kobject,_:*mut attribute,d:*mut u8)->isize{sprintf(d,b"%u\n\0".as_ptr(),(*(k as *mut edac_device_block)).counters.ue_count)}
unsafe extern "C" fn block_ce_count_show(k:*mut kobject,_:*mut attribute,d:*mut u8)->isize{sprintf(d,b"%u\n\0".as_ptr(),(*(k as *mut edac_device_block)).counters.ce_count)}
unsafe extern "C" fn edac_device_ctrl_block_release(k:*mut kobject){kobject_put(&mut (*(k as *mut edac_device_block)).instance.as_mut().unwrap().instance.as_mut().unwrap().ctl.as_mut().unwrap().kobj)}
unsafe extern "C" fn edac_dev_block_show(_: *mut kobject, _: *mut attribute, _: *mut u8)->isize{-5}
static DEVICE_BLOCK_OPS:sysfs_ops=sysfs_ops{show:Some(edac_dev_block_show),store:None};
static mut KTYPE_BLOCK_CTRL:kobj_type=kobj_type{release:Some(edac_device_ctrl_block_release),sysfs_ops:&DEVICE_BLOCK_OPS,default_groups:core::ptr::null()};

unsafe fn edac_device_create_block(d:*mut edac_device_ctl_info,i:*mut edac_device_instance,b:*mut edac_device_block)->i32{memset(&mut (*b).kobj as *mut _ as *mut _,0,core::mem::size_of::<kobject>());let main=kobject_get(&mut (*d).kobj);if main.is_null(){return -19}let mut e=kobject_init_and_add(&mut (*b).kobj,&mut KTYPE_BLOCK_CTRL,&mut (*i).kobj,b"%s\0".as_ptr(),(*b).name);if e!=0{kobject_put(main);return -19}let mut a=(*b).block_attributes;for _ in 0..(*b).nr_attribs{e=sysfs_create_file(&mut (*b).kobj,&(*a).attr);if e!=0{ kobject_put(&mut (*b).kobj);return e}a=a.add(1)}kobject_uevent(&mut (*b).kobj,0);0}
unsafe fn edac_device_delete_block(_: *mut edac_device_ctl_info,b:*mut edac_device_block){let mut a=(*b).block_attributes;for _ in 0..(*b).nr_attribs{sysfs_remove_file(&mut (*b).kobj,&(*a).attr);a=a.add(1)}kobject_put(&mut (*b).kobj)}
unsafe fn edac_device_create_instance(d:*mut edac_device_ctl_info,idx:i32)->i32{let i=(*d).instances.add(idx as usize);memset(&mut (*i).kobj as *mut _ as *mut _,0,core::mem::size_of::<kobject>());(*i).ctl=d;let main=kobject_get(&mut (*d).kobj);if main.is_null(){return -19}let mut e=kobject_init_and_add(&mut (*i).kobj,&mut KTYPE_INSTANCE_CTRL,&mut (*d).kobj,b"%s\0".as_ptr(),(*i).name);if e!=0{kobject_put(main);return e}for n in 0..(*i).nr_blocks{e=edac_device_create_block(d,i,(*i).blocks.add(n as usize));if e!=0{for j in 0..n{edac_device_delete_block(d,(*i).blocks.add(j as usize))}kobject_put(&mut (*i).kobj);return e}}kobject_uevent(&mut (*i).kobj,0);0}
unsafe fn edac_device_delete_instance(d:*mut edac_device_ctl_info,idx:i32){let i=(*d).instances.add(idx as usize);for n in 0..(*i).nr_blocks{edac_device_delete_block(d,(*i).blocks.add(n as usize))}kobject_put(&mut (*i).kobj)}
unsafe fn edac_device_create_instances(d:*mut edac_device_ctl_info)->i32{for i in 0..(*d).nr_instances{let e=edac_device_create_instance(d,i);if e!=0{for j in 0..i{edac_device_delete_instance(d,j)}return e}}0}
unsafe fn edac_device_delete_instances(d:*mut edac_device_ctl_info){for i in 0..(*d).nr_instances{edac_device_delete_instance(d,i)}}
unsafe fn edac_device_add_main_sysfs_attributes(d:*mut edac_device_ctl_info)->i32{let mut a=(*d).sysfs_attributes;if !a.is_null(){while !(*a).attr.name.is_null(){let e=sysfs_create_file(&mut (*d).kobj,&(*a).attr);if e!=0{return e}a=a.add(1)}}0}
unsafe fn edac_device_remove_main_sysfs_attributes(d:*mut edac_device_ctl_info){let mut a=(*d).sysfs_attributes;if !a.is_null(){while !(*a).attr.name.is_null(){sysfs_remove_file(&mut (*d).kobj,&(*a).attr);a=a.add(1)}}}
pub unsafe extern "C" fn edac_device_create_sysfs(d:*mut edac_device_ctl_info)->i32{let mut e=edac_device_add_main_sysfs_attributes(d);if e!=0{return e}e=sysfs_create_link(&mut (*d).kobj,&mut (*(*d).dev).kobj,EDAC_DEVICE_SYMLINK);if e!=0{edac_device_remove_main_sysfs_attributes(d);return e}e=edac_device_create_instances(d);if e!=0{sysfs_remove_link(&mut (*d).kobj,EDAC_DEVICE_SYMLINK);edac_device_remove_main_sysfs_attributes(d)}e}
pub unsafe extern "C" fn edac_device_remove_sysfs(d:*mut edac_device_ctl_info){edac_device_remove_main_sysfs_attributes(d);sysfs_remove_link(&mut (*d).kobj,EDAC_DEVICE_SYMLINK);edac_device_delete_instances(d)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
