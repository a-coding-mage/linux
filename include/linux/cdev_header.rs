/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/kobject.h, linux/kdev_t.h, linux/list.h, linux/device.h

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

// `dev_t` is supplied by linux/kdev_t.h.
pub type dev_t = u64;

#[repr(C)]
pub struct cdev {
    pub kobj: kobject,
    pub owner: *mut module,
    pub ops: *const file_operations,
    pub list: list_head,
    pub dev: dev_t,
    pub count: ::core::ffi::c_uint,
}

unsafe extern "C" {
    pub fn cdev_init(cdev: *mut cdev, fops: *const file_operations);

    pub fn cdev_alloc() -> *mut cdev;

    pub fn cdev_put(p: *mut cdev);

    pub fn cdev_add(cdev: *mut cdev, dev: dev_t, count: ::core::ffi::c_uint) -> ::core::ffi::c_int;

    pub fn cdev_set_parent(p: *mut cdev, kobj: *mut kobject);
    pub fn cdev_device_add(cdev: *mut cdev, dev: *mut device) -> ::core::ffi::c_int;
    pub fn cdev_device_del(cdev: *mut cdev, dev: *mut device);

    pub fn cdev_del(cdev: *mut cdev);

    pub fn cd_forget(inode: *mut inode);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
