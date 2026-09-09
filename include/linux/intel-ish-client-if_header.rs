/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Intel ISH client Interface definitions
 *
 * Copyright (c) 2019, Intel Corporation.
 */

use core::ffi::{c_char, c_void};

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct device_driver {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ishtp_device_id {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}
#[repr(C)]
pub struct guid_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

pub enum ishtp_cl_device {}
pub enum ishtp_device {}
pub enum ishtp_cl {}
pub enum ishtp_fw_client {}

pub type ishtp_print_log = unsafe extern "C" fn(*mut ishtp_device, *const c_char, ...);

#[repr(C)]
#[derive(Copy, Clone)]
pub enum cl_state {
    ISHTP_CL_INITIALIZING = 0,
    ISHTP_CL_CONNECTING,
    ISHTP_CL_CONNECTED,
    ISHTP_CL_DISCONNECTING,
    ISHTP_CL_DISCONNECTED,
}

#[repr(C)]
pub struct ishtp_cl_driver {
    pub driver: device_driver,
    pub name: *const c_char,
    pub id: *const ishtp_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut ishtp_cl_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut ishtp_cl_device)>,
    pub reset: Option<unsafe extern "C" fn(*mut ishtp_cl_device) -> i32>,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct ishtp_msg_data {
    pub size: u32,
    pub data: *mut u8,
}

#[repr(C)]
pub struct ishtp_cl_rb {
    pub list: list_head,
    pub cl: *mut ishtp_cl,
    pub buffer: ishtp_msg_data,
    pub buf_idx: usize,
    pub read_time: usize,
}

extern "C" {
    pub fn ishtp_cl_driver_register(driver: *mut ishtp_cl_driver, owner: *mut module) -> i32;
    pub fn ishtp_cl_driver_unregister(driver: *mut ishtp_cl_driver);
    pub fn ishtp_register_event_cb(device: *mut ishtp_cl_device, read_cb: Option<unsafe extern "C" fn(*mut ishtp_cl_device)>) -> i32;
    pub fn ishtp_device(cl_device: *mut ishtp_cl_device) -> *mut device;
    pub fn ishtp_wait_resume(dev: *mut ishtp_device) -> bool;
    pub fn ishtp_trace_callback(cl_device: *mut ishtp_cl_device) -> ishtp_print_log;
    pub fn ishtp_get_pci_device(cl_device: *mut ishtp_cl_device) -> *mut device;
    pub fn ishtp_get_workqueue(cl_device: *mut ishtp_cl_device) -> *mut workqueue_struct;
    pub fn ishtp_cl_allocate(cl_device: *mut ishtp_cl_device) -> *mut ishtp_cl;
    pub fn ishtp_cl_free(cl: *mut ishtp_cl);
    pub fn ishtp_cl_link(cl: *mut ishtp_cl) -> i32;
    pub fn ishtp_cl_unlink(cl: *mut ishtp_cl);
    pub fn ishtp_cl_disconnect(cl: *mut ishtp_cl) -> i32;
    pub fn ishtp_cl_connect(cl: *mut ishtp_cl) -> i32;
    pub fn ishtp_cl_establish_connection(cl: *mut ishtp_cl, uuid: *const guid_t, tx_size: i32, rx_size: i32, reset: bool) -> i32;
    pub fn ishtp_cl_destroy_connection(cl: *mut ishtp_cl, reset: bool);
    pub fn ishtp_cl_send(cl: *mut ishtp_cl, buf: *mut u8, length: usize) -> i32;
    pub fn ishtp_cl_flush_queues(cl: *mut ishtp_cl) -> i32;
    pub fn ishtp_cl_io_rb_recycle(rb: *mut ishtp_cl_rb) -> i32;
    pub fn ishtp_cl_rx_get_rb(cl: *mut ishtp_cl) -> *mut ishtp_cl_rb;
    pub fn ishtp_get_client_data(cl: *mut ishtp_cl) -> *mut c_void;
    pub fn ishtp_set_client_data(cl: *mut ishtp_cl, data: *mut c_void);
    pub fn ishtp_get_ishtp_device(cl: *mut ishtp_cl) -> *mut ishtp_device;
    pub fn ishtp_set_tx_ring_size(cl: *mut ishtp_cl, size: i32);
    pub fn ishtp_set_rx_ring_size(cl: *mut ishtp_cl, size: i32);
    pub fn ishtp_set_connection_state(cl: *mut ishtp_cl, state: i32);
    pub fn ishtp_get_connection_state(cl: *mut ishtp_cl) -> i32;
    pub fn ishtp_cl_set_fw_client_id(cl: *mut ishtp_cl, fw_client_id: i32);
    pub fn ishtp_put_device(cl_dev: *mut ishtp_cl_device);
    pub fn ishtp_get_device(cl_dev: *mut ishtp_cl_device);
    pub fn ishtp_set_drvdata(cl_device: *mut ishtp_cl_device, data: *mut c_void);
    pub fn ishtp_get_drvdata(cl_device: *mut ishtp_cl_device) -> *mut c_void;
    pub fn ishtp_dev_to_cl_device(dev: *mut device) -> *mut ishtp_cl_device;
    pub fn ishtp_fw_cl_get_client(dev: *mut ishtp_device, uuid: *const guid_t) -> *mut ishtp_fw_client;
    pub fn ishtp_get_fw_client_id(fw_client: *mut ishtp_fw_client) -> i32;
    pub fn ish_hw_reset(dev: *mut ishtp_device) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
