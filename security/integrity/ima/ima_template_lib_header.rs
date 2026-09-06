/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 Politecnico di Torino, Italy
 *                    TORSEC group -- https://security.polito.it
 *
 * Author: Roberto Sassu <roberto.sassu@polito.it>
 *
 * File: ima_template_lib.h
 *      Header for the library of supported template fields.
 */

// Dependency: linux/seq_file.h
// Dependency: ima.h

use std::ffi::{c_char, c_int, c_ulong, c_void};

pub const ENFORCE_FIELDS: u32 = 0x00000001;
pub const ENFORCE_BUFEND: u32 = 0x00000002;

// Opaque types from external dependencies
#[repr(C)]
pub struct seq_file;

#[repr(C)]
pub enum ima_show_type {
    _Unused = 0,
}

#[repr(C)]
pub struct ima_field_data;

#[repr(C)]
pub struct ima_event_data;

extern "C" {
    pub fn ima_show_template_digest(
        m: *mut seq_file,
        show: ima_show_type,
        field_data: *mut ima_field_data,
    );

    pub fn ima_show_template_digest_ng(
        m: *mut seq_file,
        show: ima_show_type,
        field_data: *mut ima_field_data,
    );

    pub fn ima_show_template_digest_ngv2(
        m: *mut seq_file,
        show: ima_show_type,
        field_data: *mut ima_field_data,
    );

    pub fn ima_show_template_string(
        m: *mut seq_file,
        show: ima_show_type,
        field_data: *mut ima_field_data,
    );

    pub fn ima_show_template_sig(
        m: *mut seq_file,
        show: ima_show_type,
        field_data: *mut ima_field_data,
    );

    pub fn ima_show_template_buf(
        m: *mut seq_file,
        show: ima_show_type,
        field_data: *mut ima_field_data,
    );

    pub fn ima_show_template_uint(
        m: *mut seq_file,
        show: ima_show_type,
        field_data: *mut ima_field_data,
    );

    pub fn ima_parse_buf(
        bufstartp: *mut c_void,
        bufendp: *mut c_void,
        bufcurp: *mut *mut c_void,
        maxfields: c_int,
        fields: *mut ima_field_data,
        curfields: *mut c_int,
        len_mask: *mut c_ulong,
        enforce_mask: c_int,
        bufname: *mut c_char,
    ) -> c_int;

    pub fn ima_eventdigest_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventname_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventdigest_ng_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventdigest_ngv2_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventdigest_modsig_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventname_ng_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventsig_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventbuf_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventmodsig_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventevmsig_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventinodeuid_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventinodegid_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventinodemode_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventinodexattrnames_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventinodexattrlengths_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;

    pub fn ima_eventinodexattrvalues_init(
        event_data: *mut ima_event_data,
        field_data: *mut ima_field_data,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
