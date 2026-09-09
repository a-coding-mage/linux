// SPDX-License-Identifier: GPL-2.0
/*
 * class.c - basic device class management
 *
 * Copyright (c) 2002-3 Patrick Mochel
 * Copyright (c) 2002-3 Open Source Development Labs
 * Copyright (c) 2003-2004 Greg Kroah-Hartman
 * Copyright (c) 2003-2004 IBM Corp.
 */

// Linux kernel dependencies are supplied by other translated units.

static mut CLASS_KSET: *mut kset = core::ptr::null_mut();

unsafe fn class_to_subsys(class: *const class) -> *mut subsys_private {
    let mut sp: *mut subsys_private = core::ptr::null_mut();
    if class.is_null() || CLASS_KSET.is_null() { return core::ptr::null_mut(); }
    spin_lock(&mut (*CLASS_KSET).list_lock);
    if !list_empty(&(*CLASS_KSET).list) {
        let mut kobj = list_first_entry(&(*CLASS_KSET).list, kobject, entry);
        while !kobj.is_null() {
            let kset = container_of!(kobj, kset, kobj);
            sp = container_of_const!(kset, subsys_private, subsys);
            if (*sp).class == class { break; }
            kobj = list_next_entry(kobj, entry);
            if kobj == list_first_entry(&(*CLASS_KSET).list, kobject, entry) { sp = core::ptr::null_mut(); break; }
        }
    }
    sp = subsys_get(sp);
    spin_unlock(&mut (*CLASS_KSET).list_lock);
    sp
}

unsafe fn class_attr_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> ssize_t {
    let class_attr = container_of!(attr, class_attribute, attr);
    let cp = to_subsys_private(kobj);
    let mut ret: ssize_t = -EIO;
    if !(*class_attr).show.is_none() { ret = (*class_attr).show.unwrap()((*cp).class, class_attr, buf); }
    ret
}

unsafe fn class_attr_store(kobj: *mut kobject, attr: *mut attribute, buf: *const c_char, count: usize) -> ssize_t {
    let class_attr = container_of!(attr, class_attribute, attr);
    let cp = to_subsys_private(kobj);
    let mut ret: ssize_t = -EIO;
    if !(*class_attr).store.is_none() { ret = (*class_attr).store.unwrap()((*cp).class, class_attr, buf, count); }
    ret
}

unsafe fn class_release(kobj: *mut kobject) {
    let cp = to_subsys_private(kobj); let class = (*cp).class;
    pr_debug!("class '{}': release.\n", (*class).name);
    if !(*class).class_release.is_none() { (*class).class_release.unwrap()(class); }
    else { pr_debug!("class '{}' does not have a release() function, be careful\n", (*class).name); }
    lockdep_unregister_key(&mut (*cp).lock_key); kfree(cp as *mut c_void);
}

unsafe fn class_child_ns_type(kobj: *const kobject) -> *const kobj_ns_type_operations {
    let cp = to_subsys_private(kobj as *mut kobject); (*(*cp).class).ns_type
}

static CLASS_SYSFS_OPS: sysfs_ops = sysfs_ops { show: Some(class_attr_show), store: Some(class_attr_store) };
static CLASS_KTYPE: kobj_type = kobj_type { sysfs_ops: &CLASS_SYSFS_OPS, release: Some(class_release), child_ns_type: Some(class_child_ns_type) };

pub unsafe fn class_create_file_ns(cls: *const class, attr: *const class_attribute, ns: *const ns_common) -> c_int {
    let sp = class_to_subsys(cls); if sp.is_null() { return -EINVAL; }
    let error = sysfs_create_file_ns(&mut (*sp).subsys.kobj, &(*attr).attr, ns); subsys_put(sp); error
}
pub unsafe fn class_remove_file_ns(cls: *const class, attr: *const class_attribute, ns: *const ns_common) {
    let sp = class_to_subsys(cls); if sp.is_null() { return; }
    sysfs_remove_file_ns(&mut (*sp).subsys.kobj, &(*attr).attr, ns); subsys_put(sp);
}

unsafe fn klist_class_to_dev(n: *mut klist_node) -> *mut device { (*to_device_private_class(n)).device }
unsafe fn klist_class_dev_get(n: *mut klist_node) { get_device(klist_class_to_dev(n)); }
unsafe fn klist_class_dev_put(n: *mut klist_node) { put_device(klist_class_to_dev(n)); }

pub unsafe fn class_register(cls: *const class) -> c_int {
    if (!(*cls).ns_type.is_null()) && (*cls).namespace.is_null() { return -EINVAL; }
    if (*cls).ns_type.is_null() && !(*cls).namespace.is_null() { return -EINVAL; }
    let cp = kzalloc_obj::<subsys_private>(); if cp.is_null() { return -ENOMEM; }
    klist_init(&mut (*cp).klist_devices, Some(klist_class_dev_get), Some(klist_class_dev_put));
    INIT_LIST_HEAD(&mut (*cp).interfaces); kset_init(&mut (*cp).glue_dirs);
    let key = &mut (*cp).lock_key; lockdep_register_key(key); __mutex_init(&mut (*cp).mutex, "subsys mutex", key);
    let mut error = kobject_set_name(&mut (*cp).subsys.kobj, "%s", (*cls).name);
    if error != 0 { lockdep_unregister_key(key); kfree(cp as *mut c_void); return error; }
    (*cp).subsys.kobj.kset = CLASS_KSET; (*cp).subsys.kobj.ktype = &CLASS_KTYPE; (*cp).class = cls;
    error = kset_register(&mut (*cp).subsys);
    if error != 0 { lockdep_unregister_key(key); kfree(cp as *mut c_void); return error; }
    error = sysfs_create_groups(&mut (*cp).subsys.kobj, (*cls).class_groups);
    if error != 0 { kobject_del(&mut (*cp).subsys.kobj); kfree_const((*cp).subsys.kobj.name); lockdep_unregister_key(key); kfree(cp as *mut c_void); }
    error
}

pub unsafe fn class_unregister(cls: *const class) { let sp = class_to_subsys(cls); if sp.is_null() { return; } sysfs_remove_groups(&mut (*sp).subsys.kobj, (*cls).class_groups); kset_unregister(&mut (*sp).subsys); subsys_put(sp); }

unsafe fn class_create_release(cls: *const class) { pr_debug!("{} called for {}\n", __func__, (*cls).name); kfree(cls as *mut c_void); }

pub unsafe fn class_create(name: *const c_char) -> *mut class {
    let cls = kzalloc_obj::<class>(); if cls.is_null() { return ERR_PTR(-ENOMEM); }
    (*cls).name = name; (*cls).class_release = Some(class_create_release);
    let retval = class_register(cls); if retval != 0 { kfree(cls as *mut c_void); return ERR_PTR(retval); } cls
}
pub unsafe fn class_destroy(cls: *const class) { if IS_ERR_OR_NULL(cls) { return; } class_unregister(cls); }

pub unsafe fn class_dev_iter_init(iter: *mut class_dev_iter, class: *const class, start: *const device, ty: *const device_type) {
    let sp = class_to_subsys(class); core::ptr::write_bytes(iter, 0, 1); if sp.is_null() { return; }
    let start_knode = if !start.is_null() { &mut (*(*start).p).knode_class } else { core::ptr::null_mut() };
    klist_iter_init_node(&mut (*sp).klist_devices, &mut (*iter).ki, start_knode); (*iter).type_ = ty; (*iter).sp = sp;
}
pub unsafe fn class_dev_iter_next(iter: *mut class_dev_iter) -> *mut device {
    if (*iter).sp.is_null() { return core::ptr::null_mut(); }
    loop { let knode = klist_next(&mut (*iter).ki); if knode.is_null() { return core::ptr::null_mut(); } let dev = klist_class_to_dev(knode); if (*iter).type_.is_null() || (*iter).type_ == (*dev).type_ { return dev; } }
}
pub unsafe fn class_dev_iter_exit(iter: *mut class_dev_iter) { klist_iter_exit(&mut (*iter).ki); subsys_put((*iter).sp); }

pub unsafe fn class_for_each_device(class: *const class, start: *const device, data: *mut c_void, fn_: device_iter_t) -> c_int {
    let sp = class_to_subsys(class); if class.is_null() || sp.is_null() { return -EINVAL; }
    let mut iter = core::mem::zeroed::<class_dev_iter>(); class_dev_iter_init(&mut iter, class, start, core::ptr::null()); let mut error = 0;
    loop { let dev = class_dev_iter_next(&mut iter); if dev.is_null() { break; } error = fn_.unwrap()(dev, data); if error != 0 { break; } }
    class_dev_iter_exit(&mut iter); subsys_put(sp); error
}

pub unsafe fn class_find_device(class: *const class, start: *const device, data: *const c_void, match_: device_match_t) -> *mut device {
    let sp = class_to_subsys(class); if class.is_null() || sp.is_null() { return core::ptr::null_mut(); }
    let mut iter = core::mem::zeroed::<class_dev_iter>(); class_dev_iter_init(&mut iter, class, start, core::ptr::null()); let mut dev;
    loop { dev = class_dev_iter_next(&mut iter); if dev.is_null() { break; } if match_.unwrap()(dev, data) { get_device(dev); break; } }
    class_dev_iter_exit(&mut iter); subsys_put(sp); dev
}

pub unsafe fn class_interface_register(i: *mut class_interface) -> c_int {
    if i.is_null() || (*i).class.is_null() { return -ENODEV; } let parent = (*i).class; let sp = class_to_subsys(parent); if sp.is_null() { return -EINVAL; }
    mutex_lock(&mut (*sp).mutex); list_add_tail(&mut (*i).node, &mut (*sp).interfaces);
    if let Some(add) = (*i).add_dev { let mut it = core::mem::zeroed(); class_dev_iter_init(&mut it, parent, core::ptr::null(), core::ptr::null()); loop { let d = class_dev_iter_next(&mut it); if d.is_null() { break; } add(d); } class_dev_iter_exit(&mut it); }
    mutex_unlock(&mut (*sp).mutex); 0
}
pub unsafe fn class_interface_unregister(i: *mut class_interface) { let parent = (*i).class; if parent.is_null() { return; } let sp = class_to_subsys(parent); if sp.is_null() { return; } mutex_lock(&mut (*sp).mutex); list_del_init(&mut (*i).node); if let Some(remove) = (*i).remove_dev { let mut it = core::mem::zeroed(); class_dev_iter_init(&mut it, parent, core::ptr::null(), core::ptr::null()); loop { let d = class_dev_iter_next(&mut it); if d.is_null() { break; } remove(d); } class_dev_iter_exit(&mut it); } mutex_unlock(&mut (*sp).mutex); subsys_put(sp); subsys_put(sp); }

pub unsafe fn show_class_attr_string(_class: *const class, attr: *const class_attribute, buf: *mut c_char) -> ssize_t { let cs = container_of!(attr, class_attribute_string, attr); sysfs_emit(buf, "%s\n", (*cs).str_) }

#[repr(C)] pub struct class_compat { pub kobj: *mut kobject }
pub unsafe fn class_compat_register(name: *const c_char) -> *mut class_compat { let cls = kmalloc_obj::<class_compat>(); if cls.is_null() { return core::ptr::null_mut(); } (*cls).kobj = kobject_create_and_add(name, &mut (*CLASS_KSET).kobj); if (*cls).kobj.is_null() { kfree(cls as *mut c_void); return core::ptr::null_mut(); } cls }
pub unsafe fn class_compat_unregister(cls: *mut class_compat) { kobject_put((*cls).kobj); kfree(cls as *mut c_void); }
pub unsafe fn class_compat_create_link(cls: *mut class_compat, dev: *mut device) -> c_int { sysfs_create_link((*cls).kobj, &mut (*dev).kobj, dev_name(dev)) }
pub unsafe fn class_compat_remove_link(cls: *mut class_compat, dev: *mut device) { sysfs_remove_link((*cls).kobj, dev_name(dev)); }
pub unsafe fn class_is_registered(class: *const class) -> bool { let sp = class_to_subsys(class); if !sp.is_null() { subsys_put(sp); true } else { false } }
pub unsafe fn classes_init() -> c_int { CLASS_KSET = kset_create_and_add("class", core::ptr::null(), core::ptr::null()); if CLASS_KSET.is_null() { -ENOMEM } else { 0 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
