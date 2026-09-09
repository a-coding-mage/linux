// SPDX-License-Identifier: GPL-2.0
/*
 *  DIBS - Direct Internal Buffer Sharing
 *
 *  Implementation of the DIBS class module
 *
 *  Copyright IBM Corp. 2025
 */

// Kernel dependencies supplied by the surrounding build.
use crate::dibs_loopback::*;

// MODULE_DESCRIPTION!("Direct Internal Buffer Sharing class");
// MODULE_LICENSE!("GPL");

static DIBS_CLASS: class = class { name: "dibs" };

/* use an array rather a list for fast mapping: */
static mut CLIENTS: [*mut dibs_client; MAX_DIBS_CLIENTS] = [core::ptr::null_mut(); MAX_DIBS_CLIENTS];
static mut MAX_CLIENT: u8 = 0;
static mut CLIENTS_LOCK: mutex = DEFINE_MUTEX!();

#[repr(C)]
struct dibs_dev_list {
    list: list_head,
    mutex: mutex, /* protects dibs device list */
}

static mut DIBS_DEV_LIST: dibs_dev_list = dibs_dev_list {
    list: LIST_HEAD_INIT!(),
    mutex: __MUTEX_INITIALIZER!(),
};

unsafe fn dibs_setup_forwarding(client: *mut dibs_client, dibs: *mut dibs_dev) {
    let mut flags: ulong = 0;

    spin_lock_irqsave(&mut (*dibs).lock, &mut flags);
    (*dibs).subs[(*client).id as usize] = client;
    spin_unlock_irqrestore(&mut (*dibs).lock, flags);
}

pub unsafe extern "C" fn dibs_register_client(client: *mut dibs_client) -> c_int {
    let mut dibs: *mut dibs_dev;
    let mut i: c_int;
    let mut rc: c_int = -ENOSPC;

    mutex_lock(&mut DIBS_DEV_LIST.mutex);
    mutex_lock(&mut CLIENTS_LOCK);
    i = 0;
    while i < MAX_DIBS_CLIENTS as c_int {
        if CLIENTS[i as usize].is_null() {
            CLIENTS[i as usize] = client;
            (*client).id = i;
            if i == MAX_CLIENT as c_int { MAX_CLIENT += 1; }
            rc = 0;
            break;
        }
        i += 1;
    }
    mutex_unlock(&mut CLIENTS_LOCK);

    if i < MAX_DIBS_CLIENTS as c_int {
        /* initialize with all devices that we got so far */
        list_for_each_entry!(dibs, &mut DIBS_DEV_LIST.list, list, {
            (*dibs).priv_[i as usize] = core::ptr::null_mut();
            ((*client).ops).add_dev(dibs);
            dibs_setup_forwarding(client, dibs);
        });
    }
    mutex_unlock(&mut DIBS_DEV_LIST.mutex);
    rc
}

pub unsafe extern "C" fn dibs_unregister_client(client: *mut dibs_client) -> c_int {
    let mut dibs: *mut dibs_dev;
    let mut flags: ulong = 0;
    let mut max_dmbs: c_int;
    let mut rc: c_int = 0;

    mutex_lock(&mut DIBS_DEV_LIST.mutex);
    list_for_each_entry!(dibs, &mut DIBS_DEV_LIST.list, list, {
        spin_lock_irqsave(&mut (*dibs).lock, &mut flags);
        max_dmbs = ((*dibs).ops).max_dmbs();
        for i in 0..max_dmbs {
            if (*dibs).dmb_clientid_arr[i as usize] == (*client).id {
                WARN!(1, "{}: attempt to unregister '{}' with registered dmb(s)\n", module_path!(), (*client).name);
                rc = -EBUSY;
                spin_unlock_irqrestore(&mut (*dibs).lock, flags);
                mutex_unlock(&mut DIBS_DEV_LIST.mutex);
                return rc;
            }
        }
        (*dibs).subs[(*client).id as usize] = core::ptr::null_mut();
        spin_unlock_irqrestore(&mut (*dibs).lock, flags);
        ((*CLIENTS[(*client).id as usize]).ops).del_dev(dibs);
        (*dibs).priv_[(*client).id as usize] = core::ptr::null_mut();
    });

    mutex_lock(&mut CLIENTS_LOCK);
    CLIENTS[(*client).id as usize] = core::ptr::null_mut();
    if (*client).id + 1 == MAX_CLIENT { MAX_CLIENT -= 1; }
    mutex_unlock(&mut CLIENTS_LOCK);
    mutex_unlock(&mut DIBS_DEV_LIST.mutex);
    rc
}

unsafe extern "C" fn dibs_dev_release(dev: *mut device) {
    let dibs = container_of!(dev, dibs_dev, dev);
    kfree((*dibs).dmb_clientid_arr);
    kfree(dibs);
}

pub unsafe extern "C" fn dibs_dev_alloc() -> *mut dibs_dev {
    let dibs = kzalloc_obj!(dibs_dev);
    if dibs.is_null() { return dibs; }
    spin_lock_init(&mut (*dibs).lock);
    (*dibs).dev.release = Some(dibs_dev_release);
    (*dibs).dev.class = &DIBS_CLASS;
    device_initialize(&mut (*dibs).dev);
    dibs
}

unsafe extern "C" fn gid_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let dibs = container_of!(dev, dibs_dev, dev);
    sysfs_emit!(buf, "%pUb\n", &(*dibs).gid)
}
static DEVICE_ATTR_RO_GID: device_attribute = DEVICE_ATTR_RO!(gid);

unsafe extern "C" fn fabric_id_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let dibs = container_of!(dev, dibs_dev, dev);
    let fabric_id: u16 = ((*dibs).ops).get_fabric_id(dibs);
    sysfs_emit!(buf, "0x%04x\n", fabric_id)
}
static DEVICE_ATTR_RO_FABRIC_ID: device_attribute = DEVICE_ATTR_RO!(fabric_id);

static mut DIBS_DEV_ATTRS: [*mut attribute; 3] = [
    &DEVICE_ATTR_RO_GID.attr as *const _ as *mut _,
    &DEVICE_ATTR_RO_FABRIC_ID.attr as *const _ as *mut _,
    core::ptr::null_mut(),
];
static DIBS_DEV_ATTR_GROUP: attribute_group = attribute_group { attrs: DIBS_DEV_ATTRS.as_ptr() as *mut _ };

pub unsafe extern "C" fn dibs_dev_add(dibs: *mut dibs_dev) -> c_int {
    let max_dmbs = ((*dibs).ops).max_dmbs();
    (*dibs).dmb_clientid_arr = kzalloc(max_dmbs as usize, GFP_KERNEL);
    if (*dibs).dmb_clientid_arr.is_null() { return -ENOMEM; }
    memset((*dibs).dmb_clientid_arr, NO_DIBS_CLIENT, max_dmbs as usize);
    let mut ret = device_add(&mut (*dibs).dev);
    if ret != 0 { return ret; }
    ret = sysfs_create_group(&mut (*dibs).dev.kobj, &DIBS_DEV_ATTR_GROUP);
    if ret != 0 {
        dev_err!(&(*dibs).dev, "sysfs_create_group failed for dibs_dev\n");
        device_del(&mut (*dibs).dev);
        return ret;
    }
    mutex_lock(&mut DIBS_DEV_LIST.mutex);
    mutex_lock(&mut CLIENTS_LOCK);
    for i in 0..MAX_CLIENT as usize {
        if !CLIENTS[i].is_null() {
            ((*CLIENTS[i]).ops).add_dev(dibs);
            dibs_setup_forwarding(CLIENTS[i], dibs);
        }
    }
    mutex_unlock(&mut CLIENTS_LOCK);
    list_add(&mut (*dibs).list, &mut DIBS_DEV_LIST.list);
    mutex_unlock(&mut DIBS_DEV_LIST.mutex);
    0
}

pub unsafe extern "C" fn dibs_dev_del(dibs: *mut dibs_dev) {
    let mut flags: ulong = 0;
    sysfs_remove_group(&mut (*dibs).dev.kobj, &DIBS_DEV_ATTR_GROUP);
    spin_lock_irqsave(&mut (*dibs).lock, &mut flags);
    for i in 0..MAX_DIBS_CLIENTS { (*dibs).subs[i] = core::ptr::null_mut(); }
    spin_unlock_irqrestore(&mut (*dibs).lock, flags);
    mutex_lock(&mut DIBS_DEV_LIST.mutex);
    mutex_lock(&mut CLIENTS_LOCK);
    for i in 0..MAX_CLIENT as usize { if !CLIENTS[i].is_null() { ((*CLIENTS[i]).ops).del_dev(dibs); } }
    mutex_unlock(&mut CLIENTS_LOCK);
    list_del_init(&mut (*dibs).list);
    mutex_unlock(&mut DIBS_DEV_LIST.mutex);
    device_del(&mut (*dibs).dev);
}

unsafe extern "C" fn dibs_init() -> c_int {
    let rc = class_register(&DIBS_CLASS);
    if rc != 0 { return rc; }
    let rc = dibs_loopback_init();
    if rc != 0 { pr_err!("{} fails with {}\n", "dibs_init", rc); }
    rc
}

unsafe extern "C" fn dibs_exit() {
    dibs_loopback_exit();
    class_unregister(&DIBS_CLASS);
}

// subsys_initcall!(dibs_init);
// module_exit!(dibs_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
