/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2018-2021 Intel Corporation */

/* Dependencies supplied by the corresponding Linux headers are referenced by name. */

/*
 * Currently we don't support any PECI command over 32 bytes.
 */
pub const PECI_REQUEST_MAX_BUF_SIZE: usize = 32;

pub struct peci_controller;
pub struct peci_request;

/**
 * struct peci_controller_ops - PECI controller specific methods
 * @xfer: PECI transfer function
 *
 * PECI controllers may have different hardware interfaces - the drivers
 * implementing PECI controllers can use this structure to abstract away those
 * differences by exposing a common interface for PECI core.
 */
#[repr(C)]
pub struct peci_controller_ops {
    pub xfer: Option<unsafe extern "C" fn(
        controller: *mut peci_controller,
        addr: u8,
        req: *mut peci_request,
    ) -> i32>,
}

/**
 * struct peci_controller - PECI controller
 * @dev: device object to register PECI controller to the device model
 * @ops: pointer to device specific controller operations
 * @bus_lock: lock used to protect multiple callers
 * @id: PECI controller ID
 *
 * PECI controllers usually connect to their drivers using non-PECI bus,
 * such as the platform bus.
 * Each PECI controller can communicate with one or more PECI devices.
 */
#[repr(C)]
pub struct peci_controller {
    pub dev: device,
    pub ops: *const peci_controller_ops,
    pub bus_lock: mutex, /* held for the duration of xfer */
    pub id: u8,
}

unsafe extern "C" {
    pub fn devm_peci_controller_add(
        parent: *mut device,
        ops: *const peci_controller_ops,
    ) -> *mut peci_controller;
}

pub unsafe fn to_peci_controller(d: *mut core::ffi::c_void) -> *mut peci_controller {
    (d as *mut u8).sub(core::mem::offset_of!(peci_controller, dev)) as *mut peci_controller
}

/**
 * struct peci_device - PECI device
 * @dev: device object to register PECI device to the device model
 * @info: PECI device characteristics
 * @info.x86_vfm: device vendor-family-model
 * @info.peci_revision: PECI revision supported by the PECI device
 * @info.socket_id: the socket ID represented by the PECI device
 * @addr: address used on the PECI bus connected to the parent controller
 * @deleted: indicates that PECI device was already deleted
 *
 * A peci_device identifies a single device (i.e. CPU) connected to a PECI bus.
 * The behaviour exposed to the rest of the system is defined by the PECI driver
 * managing the device.
 */
#[repr(C)]
pub struct peci_device {
    pub dev: device,
    pub info: peci_device_info,
    pub addr: u8,
    pub deleted: bool,
}

#[repr(C)]
pub struct peci_device_info {
    pub x86_vfm: u32,
    pub peci_revision: u8,
    pub socket_id: u8,
}

pub unsafe fn to_peci_device(d: *mut device) -> *mut peci_device {
    (d as *mut u8).sub(core::mem::offset_of!(peci_device, dev)) as *mut peci_device
}

/**
 * struct peci_request - PECI request
 * @device: PECI device to which the request is sent
 * @tx: TX buffer specific data
 * @tx.buf: TX buffer
 * @tx.len: transfer data length in bytes
 * @rx: RX buffer specific data
 * @rx.buf: RX buffer
 * @rx.len: received data length in bytes
 *
 * A peci_request represents a request issued by PECI originator (TX) and
 * a response received from PECI responder (RX).
 */
#[repr(C)]
pub struct peci_request {
    pub device: *mut peci_device,
    pub rx: peci_request_buffer,
    pub tx: peci_request_buffer,
}

#[repr(C)]
pub struct peci_request_buffer {
    pub buf: [u8; PECI_REQUEST_MAX_BUF_SIZE],
    pub len: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
