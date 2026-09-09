// SPDX-License-Identifier: GPL-2.0-only

/*
 * Copyright (C) 2020 Google Corporation
 */

// Forward declarations supplied by other translation units.
pub struct sock;
pub struct hci_dev;

extern "C" {
    pub fn read_def_system_config(
        sk: *mut sock,
        hdev: *mut hci_dev,
        data: *mut core::ffi::c_void,
        data_len: u16,
    ) -> core::ffi::c_int;

    pub fn set_def_system_config(
        sk: *mut sock,
        hdev: *mut hci_dev,
        data: *mut core::ffi::c_void,
        data_len: u16,
    ) -> core::ffi::c_int;

    pub fn read_def_runtime_config(
        sk: *mut sock,
        hdev: *mut hci_dev,
        data: *mut core::ffi::c_void,
        data_len: u16,
    ) -> core::ffi::c_int;

    pub fn set_def_runtime_config(
        sk: *mut sock,
        hdev: *mut hci_dev,
        data: *mut core::ffi::c_void,
        data_len: u16,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
