// SPDX-License-Identifier: GPL-2.0
/*
 * module.c - module sysfs fun for drivers
 */

use core::ffi::{c_char, c_int, c_void};

// Types and functions supplied by the surrounding kernel sources.
#[repr(C)]
pub struct Kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BusType {
    pub name: *const c_char,
}

#[repr(C)]
pub struct DevicePrivate {
    pub kobj: Kobject,
    pub mkobj: *mut ModuleKobject,
}

#[repr(C)]
pub struct DeviceDriver {
    pub bus: *const BusType,
    pub name: *const c_char,
    pub mod_name: *const c_char,
    pub p: *mut DevicePrivate,
    pub owner: *mut Module,
}

#[repr(C)]
pub struct ModuleKobject {
    pub kobj: Kobject,
    pub drivers_dir: *mut Kobject,
}

#[repr(C)]
pub struct Module {
    pub mkobj: ModuleKobject,
}

extern "C" {
    static mut drivers_dir_mutex: c_void;

    fn kasprintf(flags: usize, fmt: *const c_char, ...) -> *mut c_char;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn kobject_create_and_add(name: *const c_char, parent: *mut Kobject) -> *mut Kobject;
    fn lookup_or_create_module_kobject(name: *const c_char) -> *mut ModuleKobject;
    fn kobject_put(kobj: *mut Kobject);
    fn sysfs_create_link(parent: *mut Kobject, target: *mut Kobject, name: *const c_char) -> c_int;
    fn sysfs_remove_link(parent: *mut Kobject, name: *const c_char);
    fn kfree(ptr: *mut c_void);
}

const GFP_KERNEL: usize = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

unsafe fn make_driver_name(drv: *const DeviceDriver) -> *mut c_char {
    kasprintf(
        GFP_KERNEL,
        b"%s:%s\0".as_ptr() as *const c_char,
        (*(*drv).bus).name,
        (*drv).name,
    )
}

unsafe fn module_create_drivers_dir(mk: *mut ModuleKobject) {
    mutex_lock(&mut drivers_dir_mutex as *mut c_void);
    if !mk.is_null() && (*mk).drivers_dir.is_null() {
        (*mk).drivers_dir = kobject_create_and_add(
            b"drivers\0".as_ptr() as *const c_char,
            &mut (*mk).kobj,
        );
    }
    mutex_unlock(&mut drivers_dir_mutex as *mut c_void);
}

pub unsafe fn module_add_driver(mod_: *mut Module, drv: *const DeviceDriver) -> c_int {
    let mut driver_name: *mut c_char;
    let mut mk: *mut ModuleKobject = core::ptr::null_mut();
    let mut ret: c_int;

    if drv.is_null() {
        return 0;
    }

    if !mod_.is_null() {
        mk = &mut (*mod_).mkobj;
    } else if !(*drv).mod_name.is_null() {
        // Lookup or create built-in module entry in /sys/module.
        mk = lookup_or_create_module_kobject((*drv).mod_name);
        if !mk.is_null() {
            // remember our module structure
            (*(*drv).p).mkobj = mk;
            // lookup_or_create_module_kobject took a reference
            kobject_put(&mut (*mk).kobj);
        }
    }

    if mk.is_null() {
        return 0;
    }

    ret = sysfs_create_link(&mut (*(*drv).p).kobj, &mut (*mk).kobj, b"module\0".as_ptr() as *const c_char);
    if ret != 0 {
        return ret;
    }

    driver_name = make_driver_name(drv);
    if driver_name.is_null() {
        ret = -ENOMEM;
        return goto_out_remove_kobj(mk, drv, ret);
    }

    module_create_drivers_dir(mk);
    if (*mk).drivers_dir.is_null() {
        ret = -EINVAL;
        return goto_out_free_driver_name(mk, drv, driver_name, ret);
    }

    ret = sysfs_create_link((*mk).drivers_dir, &mut (*(*drv).p).kobj, driver_name);
    if ret != 0 {
        sysfs_remove_link((*mk).drivers_dir, driver_name);
        kfree(driver_name as *mut c_void);
        sysfs_remove_link(&mut (*(*drv).p).kobj, b"module\0".as_ptr() as *const c_char);
        return ret;
    }

    kfree(driver_name as *mut c_void);
    return 0;

    fn goto_out_remove_kobj(mk: *mut ModuleKobject, drv: *const DeviceDriver, ret: c_int) -> c_int {
        unsafe { sysfs_remove_link(&mut (*(*drv).p).kobj, b"module\0".as_ptr() as *const c_char); }
        let _ = mk;
        ret
    }
    fn goto_out_free_driver_name(mk: *mut ModuleKobject, drv: *const DeviceDriver, name: *mut c_char, ret: c_int) -> c_int {
        unsafe { kfree(name as *mut c_void); }
        goto_out_remove_kobj(mk, drv, ret)
    }
}

pub unsafe fn module_remove_driver(drv: *const DeviceDriver) {
    if drv.is_null() {
        return;
    }

    sysfs_remove_link(&mut (*(*drv).p).kobj, b"module\0".as_ptr() as *const c_char);

    let mk = if !(*drv).owner.is_null() {
        &mut (*(*drv).owner).mkobj
    } else {
        (*(*drv).p).mkobj
    };
    if !mk.is_null() && !(*mk).drivers_dir.is_null() {
        let driver_name = make_driver_name(drv);
        if !driver_name.is_null() {
            sysfs_remove_link((*mk).drivers_dir, driver_name);
            kfree(driver_name as *mut c_void);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
