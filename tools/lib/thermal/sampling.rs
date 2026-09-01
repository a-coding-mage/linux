// SPDX-License-Identifier: LGPL-2.1+
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>
//
// Translated from lib/thermal/sampling.c. C dependencies:
// errno.h, stdio.h, stdlib.h, unistd.h, thermal.h, thermal_nl.h

use core::ffi::{c_int, c_void};
use core::ptr;

extern "C" {
    fn nlmsg_hdr(n: *mut nl_msg) -> *mut nlmsghdr;
    fn genlmsg_hdr(nlh: *mut nlmsghdr) -> *mut genlmsghdr;
    fn genlmsg_parse(
        nlh: *mut nlmsghdr,
        hdrlen: c_int,
        tb: *mut *mut nlattr,
        maxtype: c_int,
        policy: *mut nla_policy,
    ) -> c_int;
    fn nla_get_u32(nla: *const nlattr) -> u32;
    fn nl_cb_set(
        cb: *mut nl_cb,
        type_: c_int,
        kind: c_int,
        func: Option<unsafe extern "C" fn(*mut nl_msg, *mut c_void) -> c_int>,
        arg: *mut c_void,
    ) -> c_int;
    fn nl_recvmsgs(sk: *mut nl_sock, cb: *mut nl_cb) -> thermal_error_t;
    fn nl_socket_get_fd(sk: *mut nl_sock) -> c_int;
    fn nl_unsubscribe_thermal(
        sk: *mut nl_sock,
        cb: *mut nl_cb,
        group_name: *const c_char,
    ) -> c_int;
    fn nl_thermal_disconnect(sk: *mut nl_sock, cb: *mut nl_cb);
    fn nl_thermal_connect(sk: *mut *mut nl_sock, cb: *mut *mut nl_cb) -> c_int;
    fn nl_subscribe_thermal(sk: *mut nl_sock, cb: *mut nl_cb, group_name: *const c_char) -> c_int;
}

pub type c_char = i8;
pub type thermal_error_t = c_int;

// Opaque external C types supplied by the translated headers.
#[repr(C)]
pub struct nl_msg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nlmsghdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct genlmsghdr {
    pub cmd: u8,
}

#[repr(C)]
pub struct nlattr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nla_policy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nl_cb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nl_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thermal_handler {
    pub ops: *mut thermal_ops,
    pub cb_sampling: *mut nl_cb,
    pub sk_sampling: *mut nl_sock,
}

#[repr(C)]
pub struct thermal_ops {
    pub sampling: thermal_sampling_ops,
}

#[repr(C)]
pub struct thermal_sampling_ops {
    pub tz_temp: Option<unsafe extern "C" fn(u32, u32, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct thermal_handler_param {
    pub th: *mut thermal_handler,
    pub arg: *mut c_void,
}

extern "C" {
    static THERMAL_GENL_SAMPLING_GROUP_NAME: *const c_char;
}

pub const THERMAL_ERROR: thermal_error_t = -1;
pub const THERMAL_SUCCESS: thermal_error_t = 0;

pub const NL_CB_VALID: c_int = 1;
pub const NL_CB_CUSTOM: c_int = 1;

pub const THERMAL_GENL_ATTR_MAX: usize = 16;
pub const THERMAL_GENL_ATTR_TZ_ID: usize = 1;
pub const THERMAL_GENL_ATTR_TZ_TEMP: usize = 2;
pub const THERMAL_GENL_SAMPLING_TEMP: u8 = 1;

unsafe extern "C" fn handle_thermal_sample(n: *mut nl_msg, mut arg: *mut c_void) -> c_int {
    let nlh: *mut nlmsghdr = nlmsg_hdr(n);
    let genlhdr: *mut genlmsghdr = genlmsg_hdr(nlh);
    let mut attrs: [*mut nlattr; THERMAL_GENL_ATTR_MAX + 1] =
        [ptr::null_mut(); THERMAL_GENL_ATTR_MAX + 1];
    let thp: *mut thermal_handler_param = arg as *mut thermal_handler_param;
    let th: *mut thermal_handler = (*thp).th;

    arg = (*thp).arg;

    genlmsg_parse(
        nlh,
        0,
        attrs.as_mut_ptr(),
        THERMAL_GENL_ATTR_MAX as c_int,
        ptr::null_mut(),
    );

    match (*genlhdr).cmd {
        THERMAL_GENL_SAMPLING_TEMP => {
            let tz_temp = (*(*th).ops).sampling.tz_temp;
            return tz_temp.unwrap_unchecked()(
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TEMP]),
                arg,
            );
        }
        _ => {
            return THERMAL_ERROR;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn thermal_sampling_handle(
    th: *mut thermal_handler,
    arg: *mut c_void,
) -> thermal_error_t {
    let mut thp: thermal_handler_param = thermal_handler_param { th, arg };

    if th.is_null() {
        return THERMAL_ERROR;
    }

    if nl_cb_set(
        (*th).cb_sampling,
        NL_CB_VALID,
        NL_CB_CUSTOM,
        Some(handle_thermal_sample),
        &mut thp as *mut thermal_handler_param as *mut c_void,
    ) != 0
    {
        return THERMAL_ERROR;
    }

    return nl_recvmsgs((*th).sk_sampling, (*th).cb_sampling);
}

#[no_mangle]
pub unsafe extern "C" fn thermal_sampling_fd(th: *mut thermal_handler) -> c_int {
    if th.is_null() {
        return -1;
    }

    return nl_socket_get_fd((*th).sk_sampling);
}

#[no_mangle]
pub unsafe extern "C" fn thermal_sampling_exit(th: *mut thermal_handler) -> thermal_error_t {
    if nl_unsubscribe_thermal(
        (*th).sk_sampling,
        (*th).cb_sampling,
        THERMAL_GENL_SAMPLING_GROUP_NAME,
    ) != 0
    {
        return THERMAL_ERROR;
    }

    nl_thermal_disconnect((*th).sk_sampling, (*th).cb_sampling);

    return THERMAL_SUCCESS;
}

#[no_mangle]
pub unsafe extern "C" fn thermal_sampling_init(th: *mut thermal_handler) -> thermal_error_t {
    if nl_thermal_connect(&mut (*th).sk_sampling, &mut (*th).cb_sampling) != 0 {
        return THERMAL_ERROR;
    }

    if nl_subscribe_thermal(
        (*th).sk_sampling,
        (*th).cb_sampling,
        THERMAL_GENL_SAMPLING_GROUP_NAME,
    ) != 0
    {
        return THERMAL_ERROR;
    }

    return THERMAL_SUCCESS;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
