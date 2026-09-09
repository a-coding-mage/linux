/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2013-2016, Intel Corporation. All rights reserved.
 */

// C dependencies supplied by other translation units:
// linux/device.h, linux/uuid.h, linux/device-id/mei_cl.h

use core::ffi::c_void;

pub struct mei_cl_device;
pub struct mei_device;
pub struct scatterlist;
pub struct mei_me_client;
pub struct mei_cl;
pub struct list_head;
pub struct device;
pub struct work_struct;
pub struct device_driver;
pub struct module;
pub struct mei_cl_device_id;
pub struct uuid_le;

pub type u8 = ::core::ffi::c_uchar;
pub type u32 = ::core::ffi::c_uint;
pub type size_t = usize;
pub type ssize_t = isize;
pub type c_ulong = ::core::ffi::c_ulong;

pub const MEI_CL_NAME_SIZE: usize = 32; // Supplied by linux/device-id/mei_cl.h.

pub type mei_cldev_cb_t = Option<unsafe extern "C" fn(cldev: *mut mei_cl_device)>;

/**
 * struct mei_cl_device - MEI device handle
 * An mei_cl_device pointer is returned from mei_add_device()
 * and links MEI bus clients to their actual ME host client pointer.
 * Drivers for MEI devices will get an mei_cl_device pointer
 * when being probed and shall use it for doing ME bus I/O.
 *
 * @bus_list: device on the bus list
 * @bus: parent mei device
 * @dev: linux driver model device pointer
 * @me_cl: me client
 * @cl: mei client
 * @name: device name
 * @rx_work: async work to execute Rx event callback
 * @rx_cb: Drivers register this callback to get asynchronous ME
 *\tRx buffer pending notifications.
 * @notif_work: async work to execute FW notify event callback
 * @notif_cb: Drivers register this callback to get asynchronous ME
 *\tFW notification pending notifications.
 *
 * @do_match: whether the device can be matched with a driver
 * @is_added: device is already scanned
 * @priv_data: client private data
 */
#[repr(C)]
pub struct mei_cl_device {
    pub bus_list: list_head,
    pub bus: *mut mei_device,
    pub dev: device,
    pub me_cl: *mut mei_me_client,
    pub cl: *mut mei_cl,
    pub name: [u8; MEI_CL_NAME_SIZE],
    pub rx_work: work_struct,
    pub rx_cb: mei_cldev_cb_t,
    pub notif_work: work_struct,
    pub notif_cb: mei_cldev_cb_t,
    pub do_match: u32,
    pub is_added: u32,
    pub priv_data: *mut c_void,
}

// C: #define to_mei_cl_device(d) container_of(d, struct mei_cl_device, dev)

#[repr(C)]
pub struct mei_cl_driver {
    pub driver: device_driver,
    pub name: *const ::core::ffi::c_char,
    pub id_table: *const mei_cl_device_id,
    pub probe: Option<unsafe extern "C" fn(
        cldev: *mut mei_cl_device,
        id: *const mei_cl_device_id,
    ) -> i32>,
    pub remove: Option<unsafe extern "C" fn(cldev: *mut mei_cl_device)>,
}

extern "C" {
    pub fn __mei_cldev_driver_register(
        cldrv: *mut mei_cl_driver,
        owner: *mut module,
    ) -> i32;
    pub fn mei_cldev_driver_unregister(cldrv: *mut mei_cl_driver);

    pub fn mei_cldev_send(cldev: *mut mei_cl_device, buf: *const u8, length: size_t) -> ssize_t;
    pub fn mei_cldev_send_timeout(
        cldev: *mut mei_cl_device, buf: *const u8, length: size_t, timeout: c_ulong,
    ) -> ssize_t;
    pub fn mei_cldev_recv(cldev: *mut mei_cl_device, buf: *mut u8, length: size_t) -> ssize_t;
    pub fn mei_cldev_recv_timeout(
        cldev: *mut mei_cl_device, buf: *mut u8, length: size_t, timeout: c_ulong,
    ) -> ssize_t;
    pub fn mei_cldev_send_vtag(
        cldev: *mut mei_cl_device, buf: *const u8, length: size_t, vtag: u8,
    ) -> ssize_t;
    pub fn mei_cldev_send_vtag_timeout(
        cldev: *mut mei_cl_device, buf: *const u8, length: size_t, vtag: u8, timeout: c_ulong,
    ) -> ssize_t;
    pub fn mei_cldev_recv_vtag(
        cldev: *mut mei_cl_device, buf: *mut u8, length: size_t, vtag: *mut u8,
    ) -> ssize_t;
    pub fn mei_cldev_recv_vtag_timeout(
        cldev: *mut mei_cl_device, buf: *mut u8, length: size_t, vtag: *mut u8, timeout: c_ulong,
    ) -> ssize_t;
    pub fn mei_cldev_register_rx_cb(cldev: *mut mei_cl_device, rx_cb: mei_cldev_cb_t) -> i32;
    pub fn mei_cldev_register_notif_cb(cldev: *mut mei_cl_device, notif_cb: mei_cldev_cb_t) -> i32;
    pub fn mei_cldev_uuid(cldev: *const mei_cl_device) -> *const uuid_le;
    pub fn mei_cldev_ver(cldev: *const mei_cl_device) -> u8;
    pub fn mei_cldev_mtu(cldev: *const mei_cl_device) -> size_t;
    pub fn mei_cldev_get_drvdata(cldev: *const mei_cl_device) -> *mut c_void;
    pub fn mei_cldev_set_drvdata(cldev: *mut mei_cl_device, data: *mut c_void);
    pub fn mei_cldev_enable(cldev: *mut mei_cl_device) -> i32;
    pub fn mei_cldev_disable(cldev: *mut mei_cl_device) -> i32;
    pub fn mei_cldev_enabled(cldev: *const mei_cl_device) -> bool;
    pub fn mei_cldev_send_gsc_command(
        cldev: *mut mei_cl_device,
        client_id: u8,
        fence_id: u32,
        sg_in: *mut scatterlist,
        total_in_len: size_t,
        sg_out: *mut scatterlist,
    ) -> ssize_t;
    pub fn mei_cldev_dma_map(cldev: *mut mei_cl_device, buffer_id: u8, size: size_t) -> *mut c_void;
    pub fn mei_cldev_dma_unmap(cldev: *mut mei_cl_device) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
