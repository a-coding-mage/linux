// SPDX-License-Identifier: GPL-2.0
/*
 * attribute_container.c - implementation of a simple container for classes
 *
 * Copyright (c) 2005 - James Bottomley <James.Bottomley@steeleye.com>
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct InternalContainer {
    node: klist_node,
    cont: *mut attribute_container,
    classdev: device,
}

unsafe extern "C" {
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn spin_lock(lock: *mut spinlock);
    fn spin_unlock(lock: *mut spinlock);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn klist_init(list: *mut klist, get: Option<unsafe extern "C" fn(*mut klist_node)>, put: Option<unsafe extern "C" fn(*mut klist_node)>);
    fn klist_add_tail(node: *mut klist_node, list: *mut klist);
    fn klist_del(node: *mut klist_node);
    fn klist_iter_init(list: *mut klist, iter: *mut klist_iter);
    fn klist_next(iter: *mut klist_iter) -> *mut klist_node;
    fn klist_iter_exit(iter: *mut klist_iter);
    fn device_initialize(dev: *mut device);
    fn dev_set_name(dev: *mut device, fmt: *const u8, ...) -> i32;
    fn device_add(dev: *mut device) -> i32;
    fn device_del(dev: *mut device);
    fn device_unregister(dev: *mut device);
    fn sysfs_create_group(kobj: *mut kobject, grp: *mut attribute_group) -> i32;
    fn sysfs_remove_group(kobj: *mut kobject, grp: *mut attribute_group);
    fn sysfs_attr_init(attr: *mut attribute);
    fn device_create_file(dev: *mut device, attr: *mut device_attribute) -> i32;
    fn device_remove_file(dev: *mut device, attr: *mut device_attribute);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn attribute_container_no_classdevs(cont: *mut attribute_container) -> bool;
    fn attribute_container_add_class_device(dev: *mut device) -> i32;
    fn attribute_container_remove_attrs(dev: *mut device);
}

unsafe extern "C" fn internal_container_klist_get(n: *mut klist_node) {
    let ic = container_of!(n, InternalContainer, node);
    get_device(&mut (*ic).classdev);
}

unsafe extern "C" fn internal_container_klist_put(n: *mut klist_node) {
    let ic = container_of!(n, InternalContainer, node);
    put_device(&mut (*ic).classdev);
}

pub unsafe extern "C" fn attribute_container_classdev_to_container(classdev: *mut device) -> *mut attribute_container {
    let ic = container_of!(classdev, InternalContainer, classdev);
    (*ic).cont
}

static mut ATTRIBUTE_CONTAINER_LIST: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut ATTRIBUTE_CONTAINER_MUTEX: mutex = mutex { __dummy: 0 };

pub unsafe extern "C" fn attribute_container_register(cont: *mut attribute_container) {
    INIT_LIST_HEAD!(&mut (*cont).node);
    klist_init(&mut (*cont).containers, Some(internal_container_klist_get), Some(internal_container_klist_put));
    mutex_lock(&mut ATTRIBUTE_CONTAINER_MUTEX);
    list_add_tail(&mut (*cont).node, &mut ATTRIBUTE_CONTAINER_LIST);
    mutex_unlock(&mut ATTRIBUTE_CONTAINER_MUTEX);
}

pub unsafe extern "C" fn attribute_container_unregister(cont: *mut attribute_container) -> i32 {
    let mut retval = -EBUSY;
    mutex_lock(&mut ATTRIBUTE_CONTAINER_MUTEX);
    spin_lock(&mut (*cont).containers.k_lock);
    if list_empty(&(*cont).containers.k_list) {
        retval = 0;
        list_del(&mut (*cont).node);
    }
    spin_unlock(&mut (*cont).containers.k_lock);
    mutex_unlock(&mut ATTRIBUTE_CONTAINER_MUTEX);
    retval
}

unsafe extern "C" fn attribute_container_release(classdev: *mut device) {
    let ic = container_of!(classdev, InternalContainer, classdev);
    let dev = (*classdev).parent;
    kfree(ic as *mut core::ffi::c_void);
    put_device(dev);
}

pub unsafe extern "C" fn attribute_container_add_device(dev: *mut device, fn_: Option<unsafe extern "C" fn(*mut attribute_container, *mut device, *mut device)>) {
    mutex_lock(&mut ATTRIBUTE_CONTAINER_MUTEX);
    list_for_each_entry!(cont, &mut ATTRIBUTE_CONTAINER_LIST, node, {
        if attribute_container_no_classdevs(cont) || !((*cont).match_)(cont, dev) { continue; }
        let ic = kzalloc_obj!(InternalContainer);
        if ic.is_null() { dev_err!(dev, "failed to allocate class container\n"); continue; }
        (*ic).cont = cont;
        device_initialize(&mut (*ic).classdev);
        (*ic).classdev.parent = get_device(dev);
        (*ic).classdev.class = (*cont).class;
        (*(*cont).class).dev_release = Some(attribute_container_release);
        dev_set_name(&mut (*ic).classdev, b"%s\0".as_ptr(), dev_name!(dev));
        if let Some(f) = fn_ { f(cont, dev, &mut (*ic).classdev); } else { attribute_container_add_class_device(&mut (*ic).classdev); }
        klist_add_tail(&mut (*ic).node, &mut (*cont).containers);
    });
    mutex_unlock(&mut ATTRIBUTE_CONTAINER_MUTEX);
}

pub unsafe extern "C" fn attribute_container_remove_device(dev: *mut device, fn_: Option<unsafe extern "C" fn(*mut attribute_container, *mut device, *mut device)>) {
    mutex_lock(&mut ATTRIBUTE_CONTAINER_MUTEX);
    list_for_each_entry!(cont, &mut ATTRIBUTE_CONTAINER_LIST, node, {
        if attribute_container_no_classdevs(cont) || !((*cont).match_)(cont, dev) { continue; }
        klist_for_each_entry!(ic, &mut (*cont).containers, node, iter, {
            if dev != (*ic).classdev.parent { continue; }
            klist_del(&mut (*ic).node);
            if let Some(f) = fn_ { f(cont, dev, &mut (*ic).classdev); } else { attribute_container_remove_attrs(&mut (*ic).classdev); device_unregister(&mut (*ic).classdev); }
        });
    });
    mutex_unlock(&mut ATTRIBUTE_CONTAINER_MUTEX);
}

unsafe fn do_attribute_container_device_trigger_safe(dev: *mut device, cont: *mut attribute_container, fn_: unsafe extern "C" fn(*mut attribute_container, *mut device, *mut device) -> i32, undo: Option<unsafe extern "C" fn(*mut attribute_container, *mut device, *mut device) -> i32>) -> i32 {
    if attribute_container_no_classdevs(cont) { return fn_(cont, dev, core::ptr::null_mut()); }
    let mut ret = 0;
    let mut failed: *mut InternalContainer = core::ptr::null_mut();
    klist_for_each_entry!(ic, &mut (*cont).containers, node, iter, {
        if dev == (*ic).classdev.parent {
            ret = fn_(cont, dev, &mut (*ic).classdev);
            if ret != 0 { failed = ic; klist_iter_exit(iter); break; }
        }
    });
    if ret == 0 { return 0; }
    let Some(undo_fn) = undo else { return ret; };
    klist_for_each_entry!(ic, &mut (*cont).containers, node, iter, {
        if ic == failed { klist_iter_exit(iter); break; }
        if dev == (*ic).classdev.parent { undo_fn(cont, dev, &mut (*ic).classdev); }
    });
    ret
}

pub unsafe extern "C" fn attribute_container_device_trigger_safe(dev: *mut device, fn_: unsafe extern "C" fn(*mut attribute_container, *mut device, *mut device) -> i32, undo: Option<unsafe extern "C" fn(*mut attribute_container, *mut device, *mut device) -> i32>) -> i32 {
    let mut ret = 0;
    let mut failed: *mut attribute_container = core::ptr::null_mut();
    mutex_lock(&mut ATTRIBUTE_CONTAINER_MUTEX);
    list_for_each_entry!(cont, &mut ATTRIBUTE_CONTAINER_LIST, node, {
        if !((*cont).match_)(cont, dev) { continue; }
        ret = do_attribute_container_device_trigger_safe(dev, cont, fn_, undo);
        if ret != 0 { failed = cont; break; }
    });
    if ret != 0 && undo.is_some() {
        list_for_each_entry!(cont, &mut ATTRIBUTE_CONTAINER_LIST, node, {
            if failed == cont { break; }
            if !((*cont).match_)(cont, dev) { continue; }
            do_attribute_container_device_trigger_safe(dev, cont, undo.unwrap(), None);
        });
    }
    mutex_unlock(&mut ATTRIBUTE_CONTAINER_MUTEX);
    ret
}

// The remaining routines retain the original kernel helper calls and iterator structure.
pub unsafe extern "C" fn attribute_container_device_trigger(dev: *mut device, fn_: unsafe extern "C" fn(*mut attribute_container, *mut device, *mut device) ) {
    mutex_lock(&mut ATTRIBUTE_CONTAINER_MUTEX);
    list_for_each_entry!(cont, &mut ATTRIBUTE_CONTAINER_LIST, node, {
        if !((*cont).match_)(cont, dev) { continue; }
        if attribute_container_no_classdevs(cont) { fn_(cont, dev, core::ptr::null_mut()); continue; }
        klist_for_each_entry!(ic, &mut (*cont).containers, node, iter, { if dev == (*ic).classdev.parent { fn_(cont, dev, &mut (*ic).classdev); } });
    });
    mutex_unlock(&mut ATTRIBUTE_CONTAINER_MUTEX);
}

pub unsafe extern "C" fn attribute_container_add_attrs(classdev: *mut device) -> i32 {
    let cont = attribute_container_classdev_to_container(classdev);
    let attrs = (*cont).attrs;
    BUG_ON!(attrs != core::ptr::null_mut() && (*cont).grp != core::ptr::null_mut());
    if attrs.is_null() && (*cont).grp.is_null() { return 0; }
    if !(*cont).grp.is_null() { return sysfs_create_group(&mut (*classdev).kobj, (*cont).grp); }
    let mut i = 0;
    while !(*attrs.add(i)).is_null() { sysfs_attr_init(&mut (*(*attrs.add(i))).attr); let error = device_create_file(classdev, *attrs.add(i)); if error != 0 { return error; } i += 1; }
    0
}

pub unsafe extern "C" fn attribute_container_add_class_device(classdev: *mut device) -> i32 { let mut error = device_add(classdev); if error != 0 { return error; } error = attribute_container_add_attrs(classdev); if error != 0 { device_del(classdev); } error }

pub unsafe extern "C" fn attribute_container_remove_attrs(classdev: *mut device) {
    let cont = attribute_container_classdev_to_container(classdev); let attrs = (*cont).attrs;
    if attrs.is_null() && (*cont).grp.is_null() { return; }
    if !(*cont).grp.is_null() { sysfs_remove_group(&mut (*classdev).kobj, (*cont).grp); return; }
    let mut i = 0; while !(*attrs.add(i)).is_null() { device_remove_file(classdev, *attrs.add(i)); i += 1; }
}

pub unsafe extern "C" fn attribute_container_class_device_del(classdev: *mut device) { attribute_container_remove_attrs(classdev); device_del(classdev); }

pub unsafe extern "C" fn attribute_container_find_class_device(cont: *mut attribute_container, dev: *mut device) -> *mut device {
    let mut result = core::ptr::null_mut();
    klist_for_each_entry!(ic, &mut (*cont).containers, node, iter, { if (*ic).classdev.parent == dev { result = &mut (*ic).classdev; klist_iter_exit(iter); break; } });
    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
