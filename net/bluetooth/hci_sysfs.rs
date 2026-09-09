// SPDX-License-Identifier: GPL-2.0
/* Bluetooth HCI driver model support. */

// Kernel dependencies supplied by the surrounding translation unit.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct class {
    pub name: *const c_char,
}

#[repr(C)]
pub struct device_type {
    pub name: *const c_char,
    pub release: Option<unsafe extern "C" fn(*mut device)>,
    pub groups: *const *const attribute_group,
}

#[repr(C)]
pub struct device {
    pub type_: *const device_type,
    pub class: *const class,
    pub parent: *mut device,
}

#[repr(C)]
pub struct hci_conn {
    pub hdev: *mut hci_dev,
    pub dev: device,
    pub handle: u16,
}

#[repr(C)]
pub struct hci_dev {
    pub dev: device,
    pub name: [c_char; 16],
    pub reset: Option<unsafe extern "C" fn(*mut hci_dev)>,
    pub srcu: srcu_struct,
}

#[repr(C)]
pub struct srcu_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_attribute;

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute_group;

const HCI_UNREGISTER: c_int = 0;
const DPM_ORDER_DEV_LAST: c_int = 0;

extern "C" {
    static THIS_MODULE: c_void;
    static mut dev_attr_reset: device_attribute;
    static mut bt_host_groups: *const *const attribute_group;

    fn kfree(ptr: *mut c_void);
    fn to_hci_conn(dev: *mut device) -> *mut hci_conn;
    fn to_hci_dev(dev: *mut device) -> *mut hci_dev;
    fn hci_dev_test_flag(hdev: *mut hci_dev, flag: c_int) -> bool;
    fn hci_release_dev(hdev: *mut hci_dev);
    fn cleanup_srcu_struct(srcu: *mut srcu_struct);
    fn module_put(module: *const c_void);
    fn __module_get(module: *const c_void);
    fn device_initialize(dev: *mut device);
    fn device_is_registered(dev: *mut device) -> bool;
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...);
    fn device_add(dev: *mut device) -> c_int;
    fn put_device(dev: *mut device);
    fn device_find_any_child(dev: *mut device) -> *mut device;
    fn device_move(dev: *mut device, new_parent: *mut device, order: c_int) -> c_int;
    fn device_unregister(dev: *mut device);
    fn class_register(class: *const class) -> c_int;
    fn class_unregister(class: *const class);
    fn bt_dev_dbg(hdev: *mut hci_dev, fmt: *const c_char, ...);
    fn bt_dev_err(hdev: *mut hci_dev, fmt: *const c_char, ...);
}

static BT_CLASS: class = class {
    name: b"bluetooth\0".as_ptr() as *const c_char,
};

unsafe extern "C" fn bt_link_release(dev: *mut device) {
    let conn = to_hci_conn(dev);
    kfree(conn as *mut c_void);
}

static BT_LINK: device_type = device_type {
    name: b"link\0".as_ptr() as *const c_char,
    release: Some(bt_link_release),
    groups: core::ptr::null(),
};

#[no_mangle]
pub unsafe extern "C" fn hci_conn_init_sysfs(conn: *mut hci_conn) {
    let hdev = (*conn).hdev;

    bt_dev_dbg(hdev, b"conn %p\0".as_ptr() as *const c_char, conn);

    (*conn).dev.type_ = &BT_LINK;
    (*conn).dev.class = &BT_CLASS;
    (*conn).dev.parent = &mut (*hdev).dev;

    device_initialize(&mut (*conn).dev);
}

#[no_mangle]
pub unsafe extern "C" fn hci_conn_add_sysfs(conn: *mut hci_conn) {
    let hdev = (*conn).hdev;

    bt_dev_dbg(hdev, b"conn %p\0".as_ptr() as *const c_char, conn);

    if device_is_registered(&mut (*conn).dev) {
        return;
    }

    dev_set_name(
        &mut (*conn).dev,
        b"%s:%d\0".as_ptr() as *const c_char,
        (*hdev).name.as_ptr(),
        (*conn).handle as c_int,
    );

    if device_add(&mut (*conn).dev) < 0 {
        bt_dev_err(hdev, b"failed to register connection device\0".as_ptr() as *const c_char);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hci_conn_del_sysfs(conn: *mut hci_conn) {
    let hdev = (*conn).hdev;

    bt_dev_dbg(hdev, b"conn %p\0".as_ptr() as *const c_char, conn);

    if !device_is_registered(&mut (*conn).dev) {
        // If device_add() has not succeeded, use only put_device() to drop the reference count.
        put_device(&mut (*conn).dev);
        return;
    }

    // If there are devices using the connection as parent reset it to NULL before unregistering.
    loop {
        let dev = device_find_any_child(&mut (*conn).dev);
        if dev.is_null() {
            break;
        }
        device_move(dev, core::ptr::null_mut(), DPM_ORDER_DEV_LAST);
        put_device(dev);
    }

    device_unregister(&mut (*conn).dev);
}

unsafe extern "C" fn bt_host_release(dev: *mut device) {
    let hdev = to_hci_dev(dev);

    if hci_dev_test_flag(hdev, HCI_UNREGISTER) {
        hci_release_dev(hdev);
    } else {
        cleanup_srcu_struct(&mut (*hdev).srcu);
        kfree(hdev as *mut c_void);
    }
    module_put(&THIS_MODULE);
}

unsafe extern "C" fn reset_store(dev: *mut device, _attr: *mut device_attribute, _buf: *const c_char, count: usize) -> isize {
    let hdev = to_hci_dev(dev);

    if let Some(reset) = (*hdev).reset {
        reset(hdev);
    }

    count as isize
}

static BT_HOST: device_type = device_type {
    name: b"host\0".as_ptr() as *const c_char,
    release: Some(bt_host_release),
    groups: unsafe { &bt_host_groups as *const _ },
};

#[no_mangle]
pub unsafe extern "C" fn hci_init_sysfs(hdev: *mut hci_dev) {
    let dev = &mut (*hdev).dev;

    dev.type_ = &BT_HOST;
    dev.class = &BT_CLASS;

    __module_get(&THIS_MODULE);
    device_initialize(dev);
}

#[no_mangle]
pub unsafe extern "C" fn bt_sysfs_init() -> c_int {
    class_register(&BT_CLASS)
}

#[no_mangle]
pub unsafe extern "C" fn bt_sysfs_cleanup() {
    class_unregister(&BT_CLASS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
