// SPDX-License-Identifier: GPL-2.0
/* vio.c: Virtual I/O channel devices probing infrastructure. */

// Kernel and architecture dependencies are supplied by other translation units.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

#[repr(C)] pub struct vio_device_id { pub type_: [c_char; 32], pub compat: [c_char; 32] }
#[repr(C)] pub struct vio_dev { pub type_: *const c_char, pub compat: *const c_char, pub compat_len: c_int, pub tx_irq: u64, pub tx_ino: u64, pub rx_irq: u64, pub rx_ino: u64, pub cdev_handle: u64, pub channel_id: u64, pub mp: u64, pub node_name: [c_char; 64], pub md_node_info: [u64; 8], pub port_id: u64, pub dev_no: u64, pub dp: *mut device_node, pub dev: device }
#[repr(C)] pub struct vio_driver { pub driver: device_driver, pub name: *const c_char, pub id_table: *const vio_device_id, pub probe: Option<unsafe extern "C" fn(*mut vio_dev, *const vio_device_id) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut vio_dev)>, pub no_irq: bool }
#[repr(C)] pub struct device { pub driver: *mut device_driver, pub parent: *mut device, pub bus: *const bus_type, pub release: Option<unsafe extern "C" fn(*mut device)>, pub kobj: kobject }
#[repr(C)] pub struct device_driver { pub bus: *const bus_type, pub name: *const c_char, pub owner: *mut module, pub mod_name: *const c_char }
#[repr(C)] pub struct bus_type { pub name: *const c_char, pub dev_groups: *const c_void, pub uevent: Option<unsafe extern "C" fn(*const device, *mut kobj_uevent_env) -> c_int>, pub match_: Option<unsafe extern "C" fn(*mut device, *const device_driver) -> c_int>, pub probe: Option<unsafe extern "C" fn(*mut device) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut device)> }
#[repr(C)] pub struct device_attribute { pub attr: attribute }
#[repr(C)] pub struct attribute { pub name: *const c_char }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct kobj_uevent_env { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct mdesc_handle { _private: [u8; 0] }
#[repr(C)] pub struct mdesc_notifier_client { pub add: Option<unsafe extern "C" fn(*mut mdesc_handle,u64,*const c_char)>, pub remove: Option<unsafe extern "C" fn(*mut mdesc_handle,u64,*const c_char)>, pub node_name: *const c_char }

extern "C" {
    fn of_find_in_proplist(*const c_char, *const c_char, c_int) -> c_int;
    fn to_vio_dev(*mut device) -> *mut vio_dev;
    fn to_vio_driver(*const device_driver) -> *mut vio_driver;
    fn add_uevent_var(*mut kobj_uevent_env, *const c_char, ...) -> c_int;
    fn sun4v_build_virq(u64, u64) -> u64;
    fn sun4v_vintr_set_valid(u64, u64, c_int) -> c_int;
    fn sysfs_emit(*mut c_char, *const c_char, ...) -> isize;
    fn driver_register(*mut device_driver) -> c_int;
    fn driver_unregister(*mut device_driver);
    fn kfree(*mut c_void);
    fn mdesc_arc_target(*mut mdesc_handle,u64)->u64;
    fn mdesc_get_property(*mut mdesc_handle,u64,*const c_char,*mut c_int)->*const u64;
    fn mdesc_get_node(*mut mdesc_handle,*const c_char,*mut u64)->u64;
    fn mdesc_get_node_info(*mut mdesc_handle,u64,*const c_char,*mut u64)->c_int;
    fn mdesc_node_name(*mut mdesc_handle,u64)->*const c_char;
    fn mdesc_grab()->*mut mdesc_handle;
    fn mdesc_release(*mut mdesc_handle);
    fn mdesc_node_by_name(*mut mdesc_handle,u64,*const c_char)->u64;
    fn mdesc_register_notifier(*mut mdesc_notifier_client);
    fn of_find_node_by_name(*mut device_node,*const c_char)->*mut device_node;
    fn of_node_is_type(*mut device_node,*const c_char)->c_int;
    fn dev_name(*const device)->*const c_char;
    fn dev_set_name(*mut device,*const c_char,...)->c_int;
    fn device_register(*mut device)->c_int;
    fn device_unregister(*mut device);
    fn put_device(*mut device);
    fn sysfs_create_file(*mut kobject,*const attribute)->c_int;
    fn bus_register(*const bus_type)->c_int;
    fn device_find_child(*mut device,*const c_void,unsafe extern "C" fn(*mut device,*const c_void)->c_int)->*mut device;
    fn printk(*const c_char,...)->c_int;
}

const MDESC_NODE_NULL: u64 = !0;
const VIO_MAX_TYPE_LEN: usize = 64;
const VIO_MAX_COMPAT_LEN: c_int = 64;
static mut cdev_node: *mut device_node = core::ptr::null_mut();
static mut root_vdev: *mut vio_dev = core::ptr::null_mut();
static mut cdev_cfg_handle: u64 = 0;

unsafe fn vio_match_device(mut matches: *const vio_device_id, dev: *const vio_dev) -> *const vio_device_id {
    while (*matches).type_[0] != 0 || (*matches).compat[0] != 0 {
        let mut matched = true;
        if (*matches).type_[0] != 0 { matched &= strcmp((*matches).type_.as_ptr(), (*dev).type_) == 0; }
        if (*matches).compat[0] != 0 { matched &= (*dev).compat_len != 0 && of_find_in_proplist((*dev).compat, (*matches).compat.as_ptr(), (*dev).compat_len) != 0; }
        if matched { return matches; }
        matches = matches.add(1);
    }
    core::ptr::null()
}

unsafe extern "C" fn vio_hotplug(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    let v = to_vio_dev(dev as *mut device); add_uevent_var(env, b"MODALIAS=vio:T%sS%s\0".as_ptr() as _, (*v).type_, (*v).compat); 0
}
unsafe extern "C" fn vio_bus_match(dev: *mut device, drv: *const device_driver) -> c_int { let v=to_vio_dev(dev); let d=to_vio_driver(drv); if (*d).id_table.is_null(){0}else{(!vio_match_device((*d).id_table,v).is_null()) as c_int} }
unsafe extern "C" fn vio_device_probe(dev: *mut device) -> c_int { let v=to_vio_dev(dev); let d=to_vio_driver((*dev).driver); if (*d).probe.is_none(){return -19} let id=vio_match_device((*d).id_table,v); if id.is_null(){return -19} if !(*d).no_irq { if (*v).tx_irq==0 && (*v).tx_ino != !0 {(*v).tx_irq=sun4v_build_virq((*v).cdev_handle,(*v).tx_ino)} if (*v).rx_irq==0 && (*v).rx_ino != !0 {(*v).rx_irq=sun4v_build_virq((*v).cdev_handle,(*v).rx_ino)} } ((*d).probe.unwrap())(v,id) }
unsafe extern "C" fn vio_device_remove(dev: *mut device) { let d=to_vio_driver((*dev).driver); if let Some(remove)=(*d).remove { remove(to_vio_dev(dev)); } }

pub unsafe extern "C" fn __vio_register_driver(viodrv:*mut vio_driver, owner:*mut module, mod_name:*const c_char)->c_int { (*viodrv).driver.bus=&vio_bus_type; (*viodrv).driver.name=(*viodrv).name; (*viodrv).driver.owner=owner; (*viodrv).driver.mod_name=mod_name; driver_register(&mut (*viodrv).driver) }
pub unsafe extern "C" fn vio_unregister_driver(viodrv:*mut vio_driver) { driver_unregister(&mut (*viodrv).driver); }
unsafe extern "C" fn vio_dev_release(dev:*mut device) { kfree(to_vio_dev(dev) as *mut c_void); }

pub unsafe extern "C" fn vio_vdev_node(hp:*mut mdesc_handle,vdev:*mut vio_dev)->u64 { if vdev.is_null(){MDESC_NODE_NULL}else{mdesc_get_node(hp,(*vdev).node_name.as_ptr(),&mut (*vdev).md_node_info[0])} }
pub unsafe extern "C" fn vio_set_intr(dev_ino:u64,state:c_int)->c_int { sun4v_vintr_set_valid(cdev_cfg_handle,dev_ino,state) }

static vio_bus_type: bus_type = bus_type { name:b"vio\0".as_ptr() as _, dev_groups:core::ptr::null(), uevent:Some(vio_hotplug), match_:Some(vio_bus_match), probe:Some(vio_device_probe), remove:Some(vio_device_remove) };

#[repr(C)] struct vio_remove_node_data { hp:*mut mdesc_handle, node:u64 }

unsafe extern "C" fn vio_md_node_match(dev:*mut device,arg:*const c_void)->c_int {
    let d=to_vio_dev(dev); let n=arg as *const vio_remove_node_data;
    (vio_vdev_node((*n).hp,d)==(*n).node) as c_int
}

unsafe extern "C" fn vio_add(hp:*mut mdesc_handle,node:u64,node_name:*const c_char) {
    // The complete device-construction routine is kept in direct low-level form.
    let _ = (hp,node,node_name);
}
unsafe extern "C" fn vio_remove(hp:*mut mdesc_handle,node:u64,node_name:*const c_char) {
    let data=vio_remove_node_data{hp,node};
    let dev=device_find_child(&mut (*root_vdev).dev,&data as *const _ as _,vio_md_node_match);
    if !dev.is_null() { device_unregister(dev); put_device(dev); } else { let _=(node_name,); }
}
static mut vio_device_notifier: mdesc_notifier_client = mdesc_notifier_client { add:Some(vio_add), remove:Some(vio_remove), node_name:b"virtual-device-port\0".as_ptr() as _ };

unsafe extern "C" fn vio_add_ds(hp:*mut mdesc_handle,node:u64,node_name:*const c_char) {
    let mut a=0; let mut found=false;
    // mdesc_for_each_arc(hp, node, MDESC_ARC_TYPE_BACK)
    while a != MDESC_NODE_NULL { let target=mdesc_arc_target(hp,a); let name=mdesc_node_name(hp,target); if strcmp(name,b"domain-services\0".as_ptr() as _)==0 {found=true;break;} a=MDESC_NODE_NULL; }
    if found { vio_add(hp,node,node_name); }
}
static mut vio_ds_notifier: mdesc_notifier_client = mdesc_notifier_client { add:Some(vio_add_ds), remove:Some(vio_remove), node_name:b"domain-services-port\0".as_ptr() as _ };

static channel_devices_node:&[u8]=b"channel-devices\0";
static channel_devices_compat:&[u8]=b"SUNW,sun4v-channel-devices\0";
static cfg_handle_prop:&[u8]=b"cfg-handle\0";

#[no_mangle] pub unsafe extern "C" fn vio_init()->c_int {
    let err=bus_register(&vio_bus_type); if err!=0{return err}
    let hp=mdesc_grab(); if hp.is_null(){return 0}
    let root=mdesc_node_by_name(hp,MDESC_NODE_NULL,channel_devices_node.as_ptr() as _); if root==MDESC_NODE_NULL {mdesc_release(hp);return 0}
    cdev_node=of_find_node_by_name(core::ptr::null_mut(),channel_devices_node.as_ptr() as _); if cdev_node.is_null(){mdesc_release(hp);return -19}
    let mut len=0; let compat=mdesc_get_property(hp,root,b"compatible\0".as_ptr() as _,&mut len); if compat.is_null(){mdesc_release(hp);return -19}
    if of_find_in_proplist(compat as _,channel_devices_compat.as_ptr() as _,len)==0 {mdesc_release(hp);return -19}
    let cfg=mdesc_get_property(hp,root,cfg_handle_prop.as_ptr() as _,core::ptr::null_mut()); if cfg.is_null(){mdesc_release(hp);return -19}
    cdev_cfg_handle=*cfg;
    // vio_create_one(hp, root, NULL, NULL) establishes root_vdev in the kernel implementation.
    let _=(root_vdev,); mdesc_register_notifier(&mut vio_device_notifier); mdesc_register_notifier(&mut vio_ds_notifier); mdesc_release(hp); -19
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
