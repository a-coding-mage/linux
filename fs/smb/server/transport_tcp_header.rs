/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

use core::ffi::{c_char, c_int};

// Opaque types declared by dependencies in the original header.
#[repr(C)]
pub struct interface {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksmbd_transport {
    _private: [u8; 0],
}

pub unsafe extern "C" {
    pub fn ksmbd_tcp_set_interfaces(ifc_list: *mut c_char, ifc_list_sz: c_int) -> c_int;
    pub fn ksmbd_find_netdev_name_iface_list(
        netdev_name: *mut c_char,
    ) -> *mut interface;
    pub fn ksmbd_free_transport(kt: *mut ksmbd_transport);
    pub fn ksmbd_tcp_init() -> c_int;
    pub fn ksmbd_tcp_destroy();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
