/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Greybus Host Device
 *
 * Copyright 2014-2015 Google Inc.
 * Copyright 2014-2015 Linaro Ltd.
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left as external types for the translated header.

use core::ffi::c_void;

pub type gfp_t = usize;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ida {
    _private: [u8; 0],
}

pub struct gb_host_device;
pub struct gb_message;
pub struct gb_svc;

#[repr(C)]
pub struct gb_hd_driver {
    pub hd_priv_size: usize,

    pub cport_allocate:
        Option<unsafe extern "C" fn(hd: *mut gb_host_device, cport_id: i32, flags: usize) -> i32>,
    pub cport_release: Option<unsafe extern "C" fn(hd: *mut gb_host_device, cport_id: u16)>,
    pub cport_enable:
        Option<unsafe extern "C" fn(hd: *mut gb_host_device, cport_id: u16, flags: usize) -> i32>,
    pub cport_disable:
        Option<unsafe extern "C" fn(hd: *mut gb_host_device, cport_id: u16) -> i32>,
    pub cport_connected:
        Option<unsafe extern "C" fn(hd: *mut gb_host_device, cport_id: u16) -> i32>,
    pub cport_flush:
        Option<unsafe extern "C" fn(hd: *mut gb_host_device, cport_id: u16) -> i32>,
    pub cport_shutdown: Option<
        unsafe extern "C" fn(
            hd: *mut gb_host_device,
            cport_id: u16,
            phase: u8,
            timeout: u32,
        ) -> i32,
    >,
    pub cport_quiesce: Option<
        unsafe extern "C" fn(
            hd: *mut gb_host_device,
            cport_id: u16,
            peer_space: usize,
            timeout: u32,
        ) -> i32,
    >,
    pub cport_clear:
        Option<unsafe extern "C" fn(hd: *mut gb_host_device, cport_id: u16) -> i32>,

    pub message_send: Option<
        unsafe extern "C" fn(
            hd: *mut gb_host_device,
            dest_cport_id: u16,
            message: *mut gb_message,
            gfp_mask: gfp_t,
        ) -> i32,
    >,
    pub message_cancel: Option<unsafe extern "C" fn(message: *mut gb_message)>,
    pub latency_tag_enable:
        Option<unsafe extern "C" fn(hd: *mut gb_host_device, cport_id: u16) -> i32>,
    pub latency_tag_disable:
        Option<unsafe extern "C" fn(hd: *mut gb_host_device, cport_id: u16) -> i32>,
    pub output: Option<
        unsafe extern "C" fn(
            hd: *mut gb_host_device,
            req: *mut c_void,
            size: u16,
            cmd: u8,
            async_: bool,
        ) -> i32,
    >,
}

#[repr(C, align(8))]
pub struct gb_host_device {
    pub dev: device,
    pub bus_id: i32,
    pub driver: *const gb_hd_driver,

    pub modules: list_head,
    pub connections: list_head,
    pub cport_id_map: ida,

    /* Number of CPorts supported by the UniPro IP */
    pub num_cports: usize,

    /* Host device buffer constraints */
    pub buffer_size_max: usize,

    pub svc: *mut gb_svc,
    /* Private data for the host driver */
    pub hd_priv: [usize; 0],
}

// #define to_gb_host_device(d) container_of(d, struct gb_host_device, dev)
// The kernel container_of operation is provided by the surrounding bindings.

unsafe extern "C" {
    pub fn gb_hd_cport_reserve(hd: *mut gb_host_device, cport_id: u16) -> i32;
    pub fn gb_hd_cport_release_reserved(hd: *mut gb_host_device, cport_id: u16);
    pub fn gb_hd_cport_allocate(hd: *mut gb_host_device, cport_id: i32, flags: usize) -> i32;
    pub fn gb_hd_cport_release(hd: *mut gb_host_device, cport_id: u16);

    pub fn gb_hd_create(
        driver: *mut gb_hd_driver,
        parent: *mut device,
        buffer_size_max: usize,
        num_cports: usize,
    ) -> *mut gb_host_device;
    pub fn gb_hd_add(hd: *mut gb_host_device) -> i32;
    pub fn gb_hd_del(hd: *mut gb_host_device);
    pub fn gb_hd_shutdown(hd: *mut gb_host_device);
    pub fn gb_hd_put(hd: *mut gb_host_device);
    pub fn gb_hd_output(hd: *mut gb_host_device, req: *mut c_void, size: u16, cmd: u8, in_irq: bool)
        -> i32;

    pub fn gb_hd_init() -> i32;
    pub fn gb_hd_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
