/*
 * dvb_ca.h: generic DVB functions for EN50221 CA interfaces
 *
 * Copyright (C) 2004 Andrew de Quincey
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either version 2.1
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

// C dependencies: linux/list.h, linux/dvb/ca.h, and media/dvbdev.h.

pub const DVB_CA_EN50221_POLL_CAM_PRESENT: ::core::ffi::c_int = 1;
pub const DVB_CA_EN50221_POLL_CAM_CHANGED: ::core::ffi::c_int = 2;
pub const DVB_CA_EN50221_POLL_CAM_READY: ::core::ffi::c_int = 4;

pub const DVB_CA_EN50221_FLAG_IRQ_CAMCHANGE: ::core::ffi::c_int = 1;
pub const DVB_CA_EN50221_FLAG_IRQ_FR: ::core::ffi::c_int = 2;
pub const DVB_CA_EN50221_FLAG_IRQ_DA: ::core::ffi::c_int = 4;

pub const DVB_CA_EN50221_CAMCHANGE_REMOVED: ::core::ffi::c_int = 0;
pub const DVB_CA_EN50221_CAMCHANGE_INSERTED: ::core::ffi::c_int = 1;

/**
 * struct dvb_ca_en50221 - Structure describing a CA interface
 *
 * NOTE: the read_*, write_* and poll_slot_status functions will be
 * called for different slots concurrently and need to use locks where
 * and if appropriate. There will be no concurrent access to one slot.
 */
#[repr(C)]
pub struct dvb_ca_en50221 {
    pub owner: *mut module,

    pub read_attribute_mem: Option<unsafe extern "C" fn(
        ca: *mut dvb_ca_en50221,
        slot: ::core::ffi::c_int,
        address: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
    pub write_attribute_mem: Option<unsafe extern "C" fn(
        ca: *mut dvb_ca_en50221,
        slot: ::core::ffi::c_int,
        address: ::core::ffi::c_int,
        value: u8,
    ) -> ::core::ffi::c_int>,

    pub read_cam_control: Option<unsafe extern "C" fn(
        ca: *mut dvb_ca_en50221,
        slot: ::core::ffi::c_int,
        address: u8,
    ) -> ::core::ffi::c_int>,
    pub write_cam_control: Option<unsafe extern "C" fn(
        ca: *mut dvb_ca_en50221,
        slot: ::core::ffi::c_int,
        address: u8,
        value: u8,
    ) -> ::core::ffi::c_int>,

    pub read_data: Option<unsafe extern "C" fn(
        ca: *mut dvb_ca_en50221,
        slot: ::core::ffi::c_int,
        ebuf: *mut u8,
        ecount: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
    pub write_data: Option<unsafe extern "C" fn(
        ca: *mut dvb_ca_en50221,
        slot: ::core::ffi::c_int,
        ebuf: *mut u8,
        ecount: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,

    pub slot_reset: Option<unsafe extern "C" fn(
        ca: *mut dvb_ca_en50221,
        slot: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
    pub slot_shutdown: Option<unsafe extern "C" fn(
        ca: *mut dvb_ca_en50221,
        slot: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
    pub slot_ts_enable: Option<unsafe extern "C" fn(
        ca: *mut dvb_ca_en50221,
        slot: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,

    pub poll_slot_status: Option<unsafe extern "C" fn(
        ca: *mut dvb_ca_en50221,
        slot: ::core::ffi::c_int,
        open: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,

    pub data: *mut ::core::ffi::c_void,
    pub private: *mut ::core::ffi::c_void,
}

// External types supplied by the included kernel headers.
pub enum module {}
pub enum dvb_adapter {}

/* Functions for reporting IRQ events. */
unsafe extern "C" {
    pub fn dvb_ca_en50221_camchange_irq(
        pubca: *mut dvb_ca_en50221,
        slot: ::core::ffi::c_int,
        change_type: ::core::ffi::c_int,
    );

    pub fn dvb_ca_en50221_camready_irq(
        pubca: *mut dvb_ca_en50221,
        slot: ::core::ffi::c_int,
    );

    pub fn dvb_ca_en50221_frda_irq(
        ca: *mut dvb_ca_en50221,
        slot: ::core::ffi::c_int,
    );

    /* Initialisation/shutdown functions. */
    pub fn dvb_ca_en50221_init(
        dvb_adapter: *mut dvb_adapter,
        ca: *mut dvb_ca_en50221,
        flags: ::core::ffi::c_int,
        slot_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn dvb_ca_en50221_release(ca: *mut dvb_ca_en50221);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
