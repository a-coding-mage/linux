/* SPDX-License-Identifier: LGPL-2.1+ */
/* Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org> */

/* Dependencies from the original C header:
 * #include <netlink/netlink.h>
 * #include <netlink/genl/genl.h>
 * #include <netlink/genl/mngt.h>
 * #include <netlink/genl/ctrl.h>
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct thermal_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nl_msg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nl_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nl_cb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thermal_handler {
    pub done: c_int,
    pub error: c_int,
    pub ops: *mut thermal_ops,
    pub msg: *mut nl_msg,
    pub sk_event: *mut nl_sock,
    pub sk_sampling: *mut nl_sock,
    pub sk_cmd: *mut nl_sock,
    pub cb_cmd: *mut nl_cb,
    pub cb_event: *mut nl_cb,
    pub cb_sampling: *mut nl_cb,
}

#[repr(C)]
pub struct thermal_handler_param {
    pub th: *mut thermal_handler,
    pub arg: *mut c_void,
}

/*
 * Low level netlink
 */
unsafe extern "C" {
    pub fn nl_subscribe_thermal(
        nl_sock: *mut nl_sock,
        nl_cb: *mut nl_cb,
        group: *const c_char,
    ) -> c_int;

    pub fn nl_unsubscribe_thermal(
        nl_sock: *mut nl_sock,
        nl_cb: *mut nl_cb,
        group: *const c_char,
    ) -> c_int;

    pub fn nl_thermal_connect(nl_sock: *mut *mut nl_sock, nl_cb: *mut *mut nl_cb) -> c_int;

    pub fn nl_thermal_disconnect(nl_sock: *mut nl_sock, nl_cb: *mut nl_cb);

    pub fn nl_send_msg(
        sock: *mut nl_sock,
        nl_cb: *mut nl_cb,
        msg: *mut nl_msg,
        rx_handler: Option<unsafe extern "C" fn(*mut nl_msg, *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;
}
