/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * RDMA Network Block Driver
 *
 * Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
 * Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
 * Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
 */

/* C header dependencies are supplied by the surrounding translation unit. */

#[repr(C)]
pub struct rnbd_srv_session {
    /* Entry inside global sess_list */
    pub list: list_head,
    pub rtrs: *mut rtrs_srv_sess,
    pub sessname: [core::ffi::c_char; NAME_MAX],
    pub queue_depth: core::ffi::c_int,

    pub index_idr: xarray,
    pub lock: mutex,
    pub ver: u8,
}

#[repr(C)]
pub struct rnbd_srv_dev {
    /* Entry inside global dev_list */
    pub list: list_head,
    pub dev_kobj: kobject,
    pub dev_sessions_kobj: *mut kobject,
    pub kref: kref,
    pub name: [core::ffi::c_char; NAME_MAX],
    /* List of rnbd_srv_sess_dev structs */
    pub sess_dev_list: list_head,
    pub lock: mutex,
    pub open_write_cnt: core::ffi::c_int,
}

/* Structure which binds N devices and N sessions */
#[repr(C)]
pub struct rnbd_srv_sess_dev {
    /* Entry inside rnbd_srv_dev struct */
    pub dev_list: list_head,
    pub bdev_file: *mut file,
    pub sess: *mut rnbd_srv_session,
    pub dev: *mut rnbd_srv_dev,
    pub kobj: kobject,
    pub device_id: u32,
    pub keep_id: bool,
    pub readonly: bool,
    pub kref: kref,
    pub destroy_comp: *mut completion,
    pub pathname: [core::ffi::c_char; NAME_MAX],
    pub access_mode: rnbd_access_mode,
}

extern "C" {
    pub fn rnbd_srv_sess_dev_force_close(
        sess_dev: *mut rnbd_srv_sess_dev,
        attr: *mut kobj_attribute,
    );

    /* rnbd-srv-sysfs.c */
    pub fn rnbd_srv_create_dev_sysfs(
        dev: *mut rnbd_srv_dev,
        bdev: *mut block_device,
    ) -> core::ffi::c_int;
    pub fn rnbd_srv_destroy_dev_sysfs(dev: *mut rnbd_srv_dev);
    pub fn rnbd_srv_create_dev_session_sysfs(
        sess_dev: *mut rnbd_srv_sess_dev,
    ) -> core::ffi::c_int;
    pub fn rnbd_srv_destroy_dev_session_sysfs(sess_dev: *mut rnbd_srv_sess_dev);
    pub fn rnbd_srv_create_sysfs_files() -> core::ffi::c_int;
    pub fn rnbd_srv_destroy_sysfs_files();
    pub fn rnbd_destroy_sess_dev(sess_dev: *mut rnbd_srv_sess_dev, keep_id: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
