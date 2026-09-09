/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright © 2003-2010 David Woodhouse <dwmw2@infradead.org>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct hd_geometry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtd_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gendisk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct request_queue {
    _private: [u8; 0],
}

#[repr(C)]
pub struct blk_mq_tag_set {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

// These types are supplied by the surrounding kernel translation.
pub type list_head = [usize; 2];
pub type mutex = [usize; 5];
pub type kref = [usize; 1];
pub type spinlock_t = [usize; 4];

#[repr(C)]
pub struct mtd_blktrans_dev {
    pub tr: *mut mtd_blktrans_ops,
    pub list: list_head,
    pub mtd: *mut mtd_info,
    pub lock: mutex,
    pub devnum: core::ffi::c_int,
    pub bg_stop: bool,
    pub size: usize,
    pub readonly: core::ffi::c_int,
    pub open: core::ffi::c_int,
    pub r#ref: kref,
    pub disk: *mut gendisk,
    pub disk_attributes: *mut attribute_group,
    pub rq: *mut request_queue,
    pub rq_list: list_head,
    pub tag_set: *mut blk_mq_tag_set,
    pub queue_lock: spinlock_t,
    pub priv_: *mut core::ffi::c_void,
    pub writable: bool,
}

#[repr(C)]
pub struct mtd_blktrans_ops {
    pub name: *mut core::ffi::c_char,
    pub major: core::ffi::c_int,
    pub part_bits: core::ffi::c_int,
    pub blksize: core::ffi::c_int,
    pub blkshift: core::ffi::c_int,

    /* Access functions */
    pub readsect: Option<unsafe extern "C" fn(
        dev: *mut mtd_blktrans_dev,
        block: usize,
        buffer: *mut core::ffi::c_char,
    ) -> core::ffi::c_int>,
    pub writesect: Option<unsafe extern "C" fn(
        dev: *mut mtd_blktrans_dev,
        block: usize,
        buffer: *mut core::ffi::c_char,
    ) -> core::ffi::c_int>,
    pub discard: Option<unsafe extern "C" fn(
        dev: *mut mtd_blktrans_dev,
        block: usize,
        nr_blocks: u32,
    ) -> core::ffi::c_int>,
    pub background: Option<unsafe extern "C" fn(dev: *mut mtd_blktrans_dev)>,

    /* Block layer ioctls */
    pub getgeo: Option<unsafe extern "C" fn(
        dev: *mut mtd_blktrans_dev,
        geo: *mut hd_geometry,
    ) -> core::ffi::c_int>,
    pub flush: Option<unsafe extern "C" fn(dev: *mut mtd_blktrans_dev) -> core::ffi::c_int>,

    /* Called with mtd_table_mutex held; no race with add/remove */
    pub open: Option<unsafe extern "C" fn(dev: *mut mtd_blktrans_dev) -> core::ffi::c_int>,
    pub release: Option<unsafe extern "C" fn(dev: *mut mtd_blktrans_dev)>,

    /* Called on {de,}registration and on subsequent addition/removal
       of devices, with mtd_table_mutex held. */
    pub add_mtd: Option<unsafe extern "C" fn(
        tr: *mut mtd_blktrans_ops,
        mtd: *mut mtd_info,
    )>,
    pub remove_dev: Option<unsafe extern "C" fn(dev: *mut mtd_blktrans_dev)>,

    pub devs: list_head,
    pub list: list_head,
    pub owner: *mut module,
}

unsafe extern "C" {
    pub fn register_mtd_blktrans(tr: *mut mtd_blktrans_ops) -> core::ffi::c_int;
    pub fn deregister_mtd_blktrans(tr: *mut mtd_blktrans_ops) -> core::ffi::c_int;
    pub fn add_mtd_blktrans_dev(dev: *mut mtd_blktrans_dev) -> core::ffi::c_int;
    pub fn del_mtd_blktrans_dev(dev: *mut mtd_blktrans_dev) -> core::ffi::c_int;
    pub fn mtd_blktrans_cease_background(dev: *mut mtd_blktrans_dev) -> core::ffi::c_int;
}

/**
 * module_mtd_blktrans() - Helper macro for registering a mtd blktrans driver
 * @__mtd_blktrans: mtd_blktrans_ops struct
 *
 * Helper macro for mtd blktrans drivers which do not do anything special in
 * module init/exit. This eliminates a lot of boilerplate. Each module may only
 * use this macro once, and calling it replaces module_init() and module_exit()
 */
#[macro_export]
macro_rules! module_mtd_blktrans {
    ($__mtd_blktrans:expr) => {
        module_driver!($::__mtd_blktrans, register_mtd_blktrans, deregister_mtd_blktrans);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
