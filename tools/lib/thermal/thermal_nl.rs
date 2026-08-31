// SPDX-License-Identifier: LGPL-2.1+
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>

use std::cell::UnsafeCell;
use std::ffi::{c_char, c_int, c_void};
use std::ptr;

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
pub struct sockaddr_nl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nlmsgerr {
    pub error: c_int,
}

#[repr(C)]
pub struct nlattr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct genlmsghdr {
    _private: [u8; 0],
}

#[repr(C)]
struct handler_args {
    group: *const c_char,
    id: c_int,
}

// Constants and external symbols are supplied by libnl and thermal headers.
const THERMAL_ERROR: c_int = -1;
const THERMAL_SUCCESS: c_int = 0;
const ENOENT: c_int = 2;

const NL_OK: c_int = 0;
const NL_STOP: c_int = 5;
const NL_CB_DEFAULT: c_int = 0;
const NL_CB_CUSTOM: c_int = 1;
const NL_CB_VALID: c_int = 2;
const NL_CB_FINISH: c_int = 3;
const NL_CB_ACK: c_int = 4;
const NL_CB_SEQ_CHECK: c_int = 5;

const CTRL_CMD_GETFAMILY: c_int = 3;
const CTRL_ATTR_FAMILY_NAME: c_int = 2;
const CTRL_ATTR_MCAST_GROUPS: c_int = 7;
const CTRL_ATTR_MAX: usize = 7;
const CTRL_ATTR_MCAST_GRP_NAME: c_int = 1;
const CTRL_ATTR_MCAST_GRP_ID: c_int = 2;
const CTRL_ATTR_MCAST_GRP_MAX: usize = 2;

unsafe extern "C" {
    static THERMAL_GENL_FAMILY_NAME: *const c_char;

    fn nl_send_auto_complete(sock: *mut nl_sock, msg: *mut nl_msg) -> c_int;
    fn nl_cb_set(
        cb: *mut nl_cb,
        kind: c_int,
        mode: c_int,
        cb_func: Option<unsafe extern "C" fn(*mut nl_msg, *mut c_void) -> c_int>,
        arg: *mut c_void,
    ) -> c_int;
    fn nl_recvmsgs(sock: *mut nl_sock, cb: *mut nl_cb) -> c_int;
    fn nla_parse(
        tb: *mut *mut nlattr,
        maxtype: c_int,
        head: *mut nlattr,
        len: c_int,
        policy: *mut c_void,
    ) -> c_int;
    fn nlmsg_hdr(msg: *mut nl_msg) -> *mut c_void;
    fn nlmsg_data(nlh: *mut c_void) -> *mut c_void;
    fn genlmsg_attrdata(gnlh: *mut genlmsghdr, hdrlen: c_int) -> *mut nlattr;
    fn genlmsg_attrlen(gnlh: *mut genlmsghdr, hdrlen: c_int) -> c_int;
    fn nla_data(nla: *mut nlattr) -> *mut c_void;
    fn nla_len(nla: *mut nlattr) -> c_int;
    fn nla_get_u32(nla: *mut nlattr) -> u32;
    fn nla_ok(nla: *mut nlattr, remaining: c_int) -> c_int;
    fn nla_next(nla: *mut nlattr, remaining: *mut c_int) -> *mut nlattr;
    fn nlmsg_alloc() -> *mut nl_msg;
    fn genl_ctrl_resolve(sock: *mut nl_sock, name: *const c_char) -> c_int;
    fn genlmsg_put(
        msg: *mut nl_msg,
        port: u32,
        seq: u32,
        family: c_int,
        hdrlen: c_int,
        flags: c_int,
        cmd: u8,
        version: u8,
    ) -> *mut c_void;
    fn nla_put_string(msg: *mut nl_msg, attrtype: c_int, str: *const c_char) -> c_int;
    fn nlmsg_free(msg: *mut nl_msg);
    fn nl_cb_alloc(kind: c_int) -> *mut nl_cb;
    fn nl_socket_alloc() -> *mut nl_sock;
    fn genl_connect(sock: *mut nl_sock) -> c_int;
    fn nl_cb_err(
        cb: *mut nl_cb,
        mode: c_int,
        cb_func: Option<
            unsafe extern "C" fn(*mut sockaddr_nl, *mut nlmsgerr, *mut c_void) -> c_int,
        >,
        arg: *mut c_void,
    ) -> c_int;
    fn nl_socket_free(sock: *mut nl_sock);
    fn nl_cb_put(cb: *mut nl_cb);
    fn nl_close(sock: *mut nl_sock);
    fn nl_socket_drop_membership(sock: *mut nl_sock, group: c_int) -> c_int;
    fn nl_socket_add_membership(sock: *mut nl_sock, group: c_int) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

thread_local! {
    static ERR: UnsafeCell<c_int> = const { UnsafeCell::new(0) };
    static DONE: UnsafeCell<c_int> = const { UnsafeCell::new(0) };
}

unsafe fn tls_get(key: &'static std::thread::LocalKey<UnsafeCell<c_int>>) -> c_int {
    key.with(|value| unsafe { *value.get() })
}

unsafe fn tls_set(key: &'static std::thread::LocalKey<UnsafeCell<c_int>>, value: c_int) {
    key.with(|cell| unsafe {
        *cell.get() = value;
    });
}

unsafe extern "C" fn nl_seq_check_handler(_msg: *mut nl_msg, _arg: *mut c_void) -> c_int {
    NL_OK
}

unsafe extern "C" fn nl_error_handler(
    _nla: *mut sockaddr_nl,
    nl_err: *mut nlmsgerr,
    arg: *mut c_void,
) -> c_int {
    let ret = arg as *mut c_int;

    if !ret.is_null() {
        unsafe {
            *ret = (*nl_err).error;
        }
    }

    NL_STOP
}

unsafe extern "C" fn nl_finish_handler(_msg: *mut nl_msg, arg: *mut c_void) -> c_int {
    let ret = arg as *mut c_int;

    if !ret.is_null() {
        unsafe {
            *ret = 1;
        }
    }

    NL_OK
}

unsafe extern "C" fn nl_ack_handler(_msg: *mut nl_msg, arg: *mut c_void) -> c_int {
    let ret = arg as *mut c_int;

    if !ret.is_null() {
        unsafe {
            *ret = 1;
        }
    }

    NL_OK
}

#[no_mangle]
pub unsafe extern "C" fn nl_send_msg(
    sock: *mut nl_sock,
    cb: *mut nl_cb,
    msg: *mut nl_msg,
    rx_handler: Option<unsafe extern "C" fn(*mut nl_msg, *mut c_void) -> c_int>,
    data: *mut c_void,
) -> c_int {
    if rx_handler.is_none() {
        return THERMAL_ERROR;
    }

    let send_err = unsafe { nl_send_auto_complete(sock, msg) };
    unsafe {
        tls_set(&ERR, send_err);
    }
    if send_err < 0 {
        return send_err;
    }

    unsafe {
        nl_cb_set(cb, NL_CB_VALID, NL_CB_CUSTOM, rx_handler, data);
    }

    unsafe {
        tls_set(&ERR, 0);
        tls_set(&DONE, 0);
    }

    while unsafe { tls_get(&ERR) == 0 && tls_get(&DONE) == 0 } {
        unsafe {
            nl_recvmsgs(sock, cb);
        }
    }

    unsafe { tls_get(&ERR) }
}

unsafe extern "C" fn nl_family_handler(msg: *mut nl_msg, arg: *mut c_void) -> c_int {
    let grp = arg as *mut handler_args;
    let mut tb: [*mut nlattr; CTRL_ATTR_MAX + 1] = [ptr::null_mut(); CTRL_ATTR_MAX + 1];
    let gnlh = unsafe { nlmsg_data(nlmsg_hdr(msg)) as *mut genlmsghdr };
    let mut rem_mcgrp: c_int;

    unsafe {
        nla_parse(
            tb.as_mut_ptr(),
            CTRL_ATTR_MAX as c_int,
            genlmsg_attrdata(gnlh, 0),
            genlmsg_attrlen(gnlh, 0),
            ptr::null_mut(),
        );
    }

    if tb[CTRL_ATTR_MCAST_GROUPS as usize].is_null() {
        return THERMAL_ERROR;
    }

    let mut mcgrp = unsafe { nla_data(tb[CTRL_ATTR_MCAST_GROUPS as usize]) as *mut nlattr };
    rem_mcgrp = unsafe { nla_len(tb[CTRL_ATTR_MCAST_GROUPS as usize]) };

    while unsafe { nla_ok(mcgrp, rem_mcgrp) } != 0 {
        let mut tb_mcgrp: [*mut nlattr; CTRL_ATTR_MCAST_GRP_MAX + 1] =
            [ptr::null_mut(); CTRL_ATTR_MCAST_GRP_MAX + 1];

        unsafe {
            nla_parse(
                tb_mcgrp.as_mut_ptr(),
                CTRL_ATTR_MCAST_GRP_MAX as c_int,
                nla_data(mcgrp) as *mut nlattr,
                nla_len(mcgrp),
                ptr::null_mut(),
            );
        }

        if tb_mcgrp[CTRL_ATTR_MCAST_GRP_NAME as usize].is_null()
            || tb_mcgrp[CTRL_ATTR_MCAST_GRP_ID as usize].is_null()
        {
            mcgrp = unsafe { nla_next(mcgrp, &mut rem_mcgrp) };
            continue;
        }

        if unsafe {
            strncmp(
                nla_data(tb_mcgrp[CTRL_ATTR_MCAST_GRP_NAME as usize]) as *const c_char,
                (*grp).group,
                nla_len(tb_mcgrp[CTRL_ATTR_MCAST_GRP_NAME as usize]) as usize,
            )
        } != 0
        {
            mcgrp = unsafe { nla_next(mcgrp, &mut rem_mcgrp) };
            continue;
        }

        unsafe {
            (*grp).id = nla_get_u32(tb_mcgrp[CTRL_ATTR_MCAST_GRP_ID as usize]) as c_int;
        }

        break;
    }

    THERMAL_SUCCESS
}

unsafe fn nl_get_multicast_id(
    sock: *mut nl_sock,
    cb: *mut nl_cb,
    family: *const c_char,
    group: *const c_char,
) -> c_int {
    let msg: *mut nl_msg;
    let mut ret: c_int;
    let ctrlid: c_int;
    let mut grp = handler_args {
        group,
        id: -ENOENT,
    };

    msg = unsafe { nlmsg_alloc() };
    if msg.is_null() {
        return THERMAL_ERROR;
    }

    ctrlid = unsafe { genl_ctrl_resolve(sock, c"nlctrl".as_ptr()) };

    unsafe {
        genlmsg_put(
            msg,
            0,
            0,
            ctrlid,
            0,
            0,
            CTRL_CMD_GETFAMILY as u8,
            0,
        );
    }

    unsafe {
        nla_put_string(msg, CTRL_ATTR_FAMILY_NAME, family);
    }

    ret = unsafe {
        nl_send_msg(
            sock,
            cb,
            msg,
            Some(nl_family_handler),
            &mut grp as *mut handler_args as *mut c_void,
        )
    };
    if ret != 0 {
        unsafe {
            nlmsg_free(msg);
        }
        return ret;
    }

    ret = grp.id;

    unsafe {
        nlmsg_free(msg);
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn nl_thermal_connect(
    nl_sock: *mut *mut nl_sock,
    nl_cb: *mut *mut nl_cb,
) -> c_int {
    let cb: *mut nl_cb;
    let sock: *mut nl_sock;

    cb = unsafe { nl_cb_alloc(NL_CB_DEFAULT) };
    if cb.is_null() {
        return THERMAL_ERROR;
    }

    sock = unsafe { nl_socket_alloc() };
    if sock.is_null() {
        unsafe {
            nl_cb_put(cb);
        }
        return THERMAL_ERROR;
    }

    if unsafe { genl_connect(sock) } != 0 {
        unsafe {
            nl_socket_free(sock);
            nl_cb_put(cb);
        }
        return THERMAL_ERROR;
    }

    let err_arg = ERR.with(|cell| cell.get() as *mut c_void);
    let done_arg = DONE.with(|cell| cell.get() as *mut c_void);

    if unsafe { nl_cb_err(cb, NL_CB_CUSTOM, Some(nl_error_handler), err_arg) } != 0
        || unsafe { nl_cb_set(cb, NL_CB_FINISH, NL_CB_CUSTOM, Some(nl_finish_handler), done_arg) }
            != 0
        || unsafe { nl_cb_set(cb, NL_CB_ACK, NL_CB_CUSTOM, Some(nl_ack_handler), done_arg) } != 0
        || unsafe {
            nl_cb_set(
                cb,
                NL_CB_SEQ_CHECK,
                NL_CB_CUSTOM,
                Some(nl_seq_check_handler),
                done_arg,
            )
        } != 0
    {
        return THERMAL_ERROR;
    }

    unsafe {
        *nl_sock = sock;
        *nl_cb = cb;
    }

    THERMAL_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn nl_thermal_disconnect(nl_sock: *mut nl_sock, nl_cb: *mut nl_cb) {
    unsafe {
        nl_close(nl_sock);
        nl_socket_free(nl_sock);
        nl_cb_put(nl_cb);
    }
}

#[no_mangle]
pub unsafe extern "C" fn nl_unsubscribe_thermal(
    nl_sock: *mut nl_sock,
    nl_cb: *mut nl_cb,
    group: *const c_char,
) -> c_int {
    let mcid: c_int;

    mcid = unsafe { nl_get_multicast_id(nl_sock, nl_cb, THERMAL_GENL_FAMILY_NAME, group) };
    if mcid < 0 {
        return THERMAL_ERROR;
    }

    if unsafe { nl_socket_drop_membership(nl_sock, mcid) } != 0 {
        return THERMAL_ERROR;
    }

    THERMAL_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn nl_subscribe_thermal(
    nl_sock: *mut nl_sock,
    nl_cb: *mut nl_cb,
    group: *const c_char,
) -> c_int {
    let mcid: c_int;

    mcid = unsafe { nl_get_multicast_id(nl_sock, nl_cb, THERMAL_GENL_FAMILY_NAME, group) };
    if mcid < 0 {
        return THERMAL_ERROR;
    }

    if unsafe { nl_socket_add_membership(nl_sock, mcid) } != 0 {
        return THERMAL_ERROR;
    }

    THERMAL_SUCCESS
}
