/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 *   GPL LICENSE SUMMARY
 *
 *   Copyright(c) 2012 Intel Corporation. All rights reserved.
 *   Copyright (C) 2015 EMC Corporation. All Rights Reserved.
 *
 *   This program is free software; you can redistribute it and/or modify
 *   it under the terms of version 2 of the GNU General Public License as
 *   published by the Free Software Foundation.
 *
 *   BSD LICENSE
 *
 *   Copyright(c) 2012 Intel Corporation. All rights reserved.
 *   Copyright (C) 2015 EMC Corporation. All Rights Reserved.
 *
 *   Redistribution and use in source and binary forms, with or without
 *   modification, are permitted provided that the following conditions
 *   are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above copy
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *     * Neither the name of Intel Corporation nor the names of its
 *       contributors may be used to endorse or promote products derived
 *       from this software without specific prior written permission.
 *
 *   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 *   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 *   OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *   SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 *   LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 *   DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 *   THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 *   (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 *   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * PCIe NTB Transport Linux driver
 *
 * Contact Information:
 * Jon Mason <jon.mason@intel.com>
 */

// External kernel types are supplied by the surrounding translation unit.
pub struct device;
pub struct device_driver;

#[repr(C)]
pub struct ntb_transport_qp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ntb_transport_client {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(client_dev: *mut device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(client_dev: *mut device)>,
}

#[repr(C)]
pub struct ntb_queue_handlers {
    pub rx_handler: Option<unsafe extern "C" fn(
        qp: *mut ntb_transport_qp,
        qp_data: *mut core::ffi::c_void,
        data: *mut core::ffi::c_void,
        len: i32,
    )>,
    pub tx_handler: Option<unsafe extern "C" fn(
        qp: *mut ntb_transport_qp,
        qp_data: *mut core::ffi::c_void,
        data: *mut core::ffi::c_void,
        len: i32,
    )>,
    pub event_handler: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void, status: i32)>,
}

extern "C" {
    pub fn ntb_transport_register_client(drvr: *mut ntb_transport_client) -> i32;
    pub fn ntb_transport_unregister_client(drvr: *mut ntb_transport_client);
    pub fn ntb_transport_register_client_dev(device_name: *mut i8) -> i32;
    pub fn ntb_transport_unregister_client_dev(device_name: *mut i8);

    pub fn ntb_transport_qp_num(qp: *mut ntb_transport_qp) -> u8;
    pub fn ntb_transport_max_size(qp: *mut ntb_transport_qp) -> u32;
    pub fn ntb_transport_create_queue(
        data: *mut core::ffi::c_void,
        client_dev: *mut device,
        handlers: *const ntb_queue_handlers,
    ) -> *mut ntb_transport_qp;
    pub fn ntb_transport_free_queue(qp: *mut ntb_transport_qp);
    pub fn ntb_transport_rx_enqueue(
        qp: *mut ntb_transport_qp,
        cb: *mut core::ffi::c_void,
        data: *mut core::ffi::c_void,
        len: u32,
    ) -> i32;
    pub fn ntb_transport_tx_enqueue(
        qp: *mut ntb_transport_qp,
        cb: *mut core::ffi::c_void,
        data: *mut core::ffi::c_void,
        len: u32,
    ) -> i32;
    pub fn ntb_transport_rx_remove(
        qp: *mut ntb_transport_qp,
        len: *mut u32,
    ) -> *mut core::ffi::c_void;
    pub fn ntb_transport_link_up(qp: *mut ntb_transport_qp);
    pub fn ntb_transport_link_down(qp: *mut ntb_transport_qp);
    pub fn ntb_transport_link_query(qp: *mut ntb_transport_qp) -> bool;
    pub fn ntb_transport_tx_free_entry(qp: *mut ntb_transport_qp) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
