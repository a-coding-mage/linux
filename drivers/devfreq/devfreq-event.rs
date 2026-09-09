// SPDX-License-Identifier: GPL-2.0-only
/*
 * devfreq-event: a framework to provide raw data and events of devfreq devices
 *
 * Copyright (C) 2015 Samsung Electronics
 * Author: Chanwoo Choi <cw00.choi@samsung.com>
 *
 * This driver is based on drivers/devfreq/devfreq.c.
 */

// Declarations supplied by the corresponding Linux kernel headers are external
// dependencies of this translation.

static mut DEVFREQ_EVENT_ATTRS: [*mut attribute; 3] = [
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
];

static mut DEVFREQ_EVENT_CLASS: class = class {
    name: "devfreq-event",
    dev_groups: devfreq_event_groups,
};

/* The list of all devfreq event list */
static mut DEVFREQ_EVENT_LIST: list_head = LIST_HEAD_INIT(DEVFREQ_EVENT_LIST);
static mut DEVFREQ_EVENT_LIST_LOCK: mutex = DEFINE_MUTEX_INIT();

#[inline]
unsafe fn to_devfreq_event(dev: *mut device) -> *mut devfreq_event_dev {
    container_of!(dev, devfreq_event_dev, dev)
}

/**
 * devfreq_event_enable_edev() - Enable the devfreq-event dev and increase
 *                                      the enable_count of devfreq-event dev.
 * @edev       : the devfreq-event device
 *
 * Note that this function increase the enable_count and enable the
 * devfreq-event device. The devfreq-event device should be enabled before
 * using it by devfreq device.
 */
pub unsafe fn devfreq_event_enable_edev(edev: *mut devfreq_event_dev) -> i32 {
    let mut ret: i32 = 0;

    if edev.is_null() || (*edev).desc.is_null() {
        return -EINVAL;
    }

    mutex_lock(&mut (*edev).lock);
    if !(*edev).desc.is_null()
        && !(*(*edev).desc).ops.is_null()
        && !(*(*(*edev).desc).ops).enable.is_none()
        && (*edev).enable_count == 0
    {
        ret = ((*(*(*edev).desc).ops).enable.unwrap())(edev);
        if ret < 0 {
            mutex_unlock(&mut (*edev).lock);
            return ret;
        }
    }
    (*edev).enable_count += 1;
    mutex_unlock(&mut (*edev).lock);
    ret
}

/**
 * devfreq_event_disable_edev() - Disable the devfreq-event dev and decrease
 *                                      the enable_count of the devfreq-event dev.
 * @edev       : the devfreq-event device
 */
pub unsafe fn devfreq_event_disable_edev(edev: *mut devfreq_event_dev) -> i32 {
    let mut ret: i32 = 0;
    if edev.is_null() || (*edev).desc.is_null() { return -EINVAL; }
    mutex_lock(&mut (*edev).lock);
    if (*edev).enable_count <= 0 {
        dev_warn!(&(*edev).dev, "unbalanced enable_count\n");
        ret = -EIO;
        mutex_unlock(&mut (*edev).lock);
        return ret;
    }
    if !(*(*edev).desc).ops.is_null()
        && !(*(*(*edev).desc).ops).disable.is_none()
        && (*edev).enable_count == 1
    {
        ret = ((*(*(*edev).desc).ops).disable.unwrap())(edev);
        if ret < 0 { mutex_unlock(&mut (*edev).lock); return ret; }
    }
    (*edev).enable_count -= 1;
    mutex_unlock(&mut (*edev).lock);
    ret
}

/** Check whether devfreq-event dev is enabled or not. */
pub unsafe fn devfreq_event_is_enabled(edev: *mut devfreq_event_dev) -> bool {
    if edev.is_null() || (*edev).desc.is_null() { return false; }
    mutex_lock(&mut (*edev).lock);
    let enabled = (*edev).enable_count > 0;
    mutex_unlock(&mut (*edev).lock);
    enabled
}

pub unsafe fn devfreq_event_set_event(edev: *mut devfreq_event_dev) -> i32 {
    if edev.is_null() || (*edev).desc.is_null() { return -EINVAL; }
    if (*(*edev).desc).ops.is_null() || (*(*(*edev).desc).ops).set_event.is_none() { return -EINVAL; }
    if !devfreq_event_is_enabled(edev) { return -EPERM; }
    mutex_lock(&mut (*edev).lock);
    let ret = ((*(*(*edev).desc).ops).set_event.unwrap())(edev);
    mutex_unlock(&mut (*edev).lock);
    ret
}

pub unsafe fn devfreq_event_get_event(edev: *mut devfreq_event_dev, edata: *mut devfreq_event_data) -> i32 {
    if edev.is_null() || (*edev).desc.is_null() { return -EINVAL; }
    if (*(*edev).desc).ops.is_null() || (*(*(*edev).desc).ops).get_event.is_none() { return -EINVAL; }
    if !devfreq_event_is_enabled(edev) { return -EINVAL; }
    (*edata).total_count = 0;
    (*edata).load_count = 0;
    mutex_lock(&mut (*edev).lock);
    let ret = ((*(*(*edev).desc).ops).get_event.unwrap())(edev, edata);
    if ret < 0 { (*edata).total_count = 0; (*edata).load_count = 0; }
    mutex_unlock(&mut (*edev).lock);
    ret
}

pub unsafe fn devfreq_event_reset_event(edev: *mut devfreq_event_dev) -> i32 {
    if edev.is_null() || (*edev).desc.is_null() { return -EINVAL; }
    if !devfreq_event_is_enabled(edev) { return -EPERM; }
    mutex_lock(&mut (*edev).lock);
    let ret = if !(*(*edev).desc).ops.is_null() && !(*(*(*edev).desc).ops).reset.is_none() {
        ((*(*(*edev).desc).ops).reset.unwrap())(edev)
    } else { 0 };
    mutex_unlock(&mut (*edev).lock);
    ret
}

pub unsafe fn devfreq_event_get_edev_by_phandle(dev: *mut device, phandle_name: *const c_char, index: i32) -> *mut devfreq_event_dev {
    if (*dev).of_node.is_null() || phandle_name.is_null() { return ERR_PTR(-EINVAL); }
    let node = of_parse_phandle((*dev).of_node, phandle_name, index);
    if node.is_null() { return ERR_PTR(-ENODEV); }
    mutex_lock(&mut DEVFREQ_EVENT_LIST_LOCK);
    let mut edev: *mut devfreq_event_dev = core::ptr::null_mut();
    list_for_each_entry!(edev, &mut DEVFREQ_EVENT_LIST, node, {
        if !(*edev).dev.parent.is_null() && device_match_of_node((*edev).dev.parent, node) { break; }
    });
    if edev.is_null() || !of_node_name_eq(node, (*(*edev).desc).name) { edev = core::ptr::null_mut(); }
    mutex_unlock(&mut DEVFREQ_EVENT_LIST_LOCK);
    of_node_put(node);
    if edev.is_null() { ERR_PTR(-ENODEV) } else { edev }
}

pub unsafe fn devfreq_event_get_edev_count(dev: *mut device, phandle_name: *const c_char) -> i32 {
    if (*dev).of_node.is_null() || phandle_name.is_null() {
        dev_err!(dev, "device does not have a device node entry\n");
        return -EINVAL;
    }
    let count = of_property_count_elems_of_size((*dev).of_node, phandle_name, core::mem::size_of::<u32>());
    if count < 0 { dev_err!(dev, "failed to get the count of devfreq-event in %pOF node\n", (*dev).of_node); }
    count
}

unsafe fn devfreq_event_release_edev(dev: *mut device) {
    let edev = to_devfreq_event(dev);
    kfree(edev as *mut core::ffi::c_void);
}

pub unsafe fn devfreq_event_add_edev(dev: *mut device, desc: *mut devfreq_event_desc) -> *mut devfreq_event_dev {
    static mut EVENT_NO: atomic_t = ATOMIC_INIT(-1);
    if dev.is_null() || desc.is_null() || (*desc).name.is_null() || (*desc).ops.is_null()
        || (*(*desc).ops).set_event.is_none() || (*(*desc).ops).get_event.is_none() { return ERR_PTR(-EINVAL); }
    let edev = kzalloc_obj::<devfreq_event_dev>();
    if edev.is_null() { return ERR_PTR(-ENOMEM); }
    mutex_init(&mut (*edev).lock);
    (*edev).desc = desc;
    (*edev).enable_count = 0;
    (*edev).dev.parent = dev;
    (*edev).dev.class = &mut DEVFREQ_EVENT_CLASS;
    (*edev).dev.release = Some(devfreq_event_release_edev);
    dev_set_name(&mut (*edev).dev, "event%d", atomic_inc_return(&mut EVENT_NO));
    let ret = device_register(&mut (*edev).dev);
    if ret < 0 { put_device(&mut (*edev).dev); return ERR_PTR(ret); }
    dev_set_drvdata(&mut (*edev).dev, edev as *mut core::ffi::c_void);
    INIT_LIST_HEAD(&mut (*edev).node);
    mutex_lock(&mut DEVFREQ_EVENT_LIST_LOCK);
    list_add(&mut (*edev).node, &mut DEVFREQ_EVENT_LIST);
    mutex_unlock(&mut DEVFREQ_EVENT_LIST_LOCK);
    edev
}

pub unsafe fn devfreq_event_remove_edev(edev: *mut devfreq_event_dev) -> i32 {
    if edev.is_null() { return -EINVAL; }
    WARN_ON!((*edev).enable_count != 0);
    mutex_lock(&mut DEVFREQ_EVENT_LIST_LOCK);
    list_del(&mut (*edev).node);
    mutex_unlock(&mut DEVFREQ_EVENT_LIST_LOCK);
    device_unregister(&mut (*edev).dev);
    0
}

unsafe fn devm_devfreq_event_match(_dev: *mut device, res: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> i32 {
    let r = res as *mut *mut devfreq_event_dev;
    if r.is_null() || (*r).is_null() { WARN_ON!(true); return 0; }
    if *r == data as *mut devfreq_event_dev { 1 } else { 0 }
}
unsafe fn devm_devfreq_event_release(_dev: *mut device, res: *mut core::ffi::c_void) {
    devfreq_event_remove_edev(*(res as *mut *mut devfreq_event_dev));
}

pub unsafe fn devm_devfreq_event_add_edev(dev: *mut device, desc: *mut devfreq_event_desc) -> *mut devfreq_event_dev {
    let ptr = devres_alloc(Some(devm_devfreq_event_release), core::mem::size_of::<*mut devfreq_event_dev>(), GFP_KERNEL);
    if ptr.is_null() { return ERR_PTR(-ENOMEM); }
    let edev = devfreq_event_add_edev(dev, desc);
    if IS_ERR(edev) { devres_free(ptr); return ERR_PTR(-ENOMEM); }
    *(ptr as *mut *mut devfreq_event_dev) = edev;
    devres_add(dev, ptr);
    edev
}

pub unsafe fn devm_devfreq_event_remove_edev(dev: *mut device, edev: *mut devfreq_event_dev) {
    WARN_ON!(devres_release(dev, Some(devm_devfreq_event_release), Some(devm_devfreq_event_match), edev as *mut core::ffi::c_void));
}

unsafe fn name_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let edev = to_devfreq_event(dev);
    if edev.is_null() || (*edev).desc.is_null() { return -EINVAL as ssize_t; }
    sprintf!(buf, "%s\n", (*(*edev).desc).name)
}

unsafe fn enable_count_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let edev = to_devfreq_event(dev);
    if edev.is_null() || (*edev).desc.is_null() { return -EINVAL as ssize_t; }
    sprintf!(buf, "%d\n", (*edev).enable_count)
}

unsafe fn devfreq_event_init() -> i32 {
    let err = class_register(&mut DEVFREQ_EVENT_CLASS);
    if err != 0 { pr_err!("%s: couldn't create class\n", file!()); }
    err
}

// Equivalent of subsys_initcall(devfreq_event_init).
subsys_initcall!(devfreq_event_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
