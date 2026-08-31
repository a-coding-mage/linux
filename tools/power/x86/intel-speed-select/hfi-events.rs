// SPDX-License-Identifier: GPL-2.0
/*
 * Intel Speed Select -- Read HFI events for OOB
 * Copyright (c) 2022 Intel Corporation.
 */

/*
 * This file incorporates work covered by the following copyright and
 * permission notice:

 * WPA Supplicant - driver interaction with Linux nl80211/cfg80211
 * Copyright (c) 2003-2008, Jouni Malinen <j@w1.fi>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * Alternatively, this software may be distributed under the terms of
 * BSD license.
 *
 * Requires
 * libnl-genl-3-dev
 *
 * For Fedora/CenOS
 * dnf install libnl3-devel
 * For Ubuntu
 * apt install libnl-3-dev libnl-genl-3-dev
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct nl_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nl_cb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nl_msg {
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
    pub cmd: u8,
    pub version: u8,
    pub reserved: u16,
}

#[repr(C)]
pub struct nlmsghdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct isst_id {
    _private: [u8; 0],
}

#[repr(C)]
struct hfi_event_data {
    nl_handle: *mut nl_sock,
    nl_cb: *mut nl_cb,
}

static mut drv: hfi_event_data = hfi_event_data {
    nl_handle: core::ptr::null_mut(),
    nl_cb: core::ptr::null_mut(),
};

const NL_STOP: c_int = 5;
const NL_SKIP: c_int = 0;
const NL_OK: c_int = 1;
const NL_CB_CUSTOM: c_int = 3;
const NL_CB_DEFAULT: c_int = 0;
const NL_CB_VALID: c_int = 4;
const NL_CB_FINISH: c_int = 2;
const NL_CB_ACK: c_int = 3;
const NL_CB_SEQ_CHECK: c_int = 0;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const CTRL_ATTR_MAX: usize = 10;
const CTRL_ATTR_MCAST_GROUPS: usize = 7;
const CTRL_ATTR_MCAST_GRP_MAX: usize = 2;
const CTRL_ATTR_MCAST_GRP_NAME: usize = 1;
const CTRL_ATTR_MCAST_GRP_ID: usize = 2;
const CTRL_CMD_GETFAMILY: c_int = 3;
const CTRL_ATTR_FAMILY_NAME: c_int = 2;
const THERMAL_GENL_ATTR_MAX: usize = 10;
const THERMAL_GENL_ATTR_CPU_CAPABILITY: usize = 6;
const THERMAL_GENL_EVENT_CPU_CAPABILITY_CHANGE: u8 = 5;

unsafe extern "C" {
    static THERMAL_GENL_FAMILY_NAME: *const c_char;
    static THERMAL_GENL_EVENT_GROUP_NAME: *const c_char;
    static mut stderr: *mut c_void;

    fn nl_cb_clone(cb: *mut nl_cb) -> *mut nl_cb;
    fn nl_send_auto_complete(sk: *mut nl_sock, msg: *mut nl_msg) -> c_int;
    fn nl_cb_err(
        cb: *mut nl_cb,
        kind: c_int,
        func: unsafe extern "C" fn(*mut sockaddr_nl, *mut nlmsgerr, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn nl_cb_set(
        cb: *mut nl_cb,
        kind: c_int,
        kind2: c_int,
        func: unsafe extern "C" fn(*mut nl_msg, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn nl_recvmsgs(sk: *mut nl_sock, cb: *mut nl_cb) -> c_int;
    fn nl_cb_put(cb: *mut nl_cb);
    fn nlmsg_free(msg: *mut nl_msg);
    fn nlmsg_hdr(msg: *mut nl_msg) -> *mut nlmsghdr;
    fn nlmsg_data(nlh: *mut nlmsghdr) -> *mut c_void;
    fn genlmsg_hdr(nlh: *mut nlmsghdr) -> *mut genlmsghdr;
    fn genlmsg_attrdata(gnlh: *mut genlmsghdr, hdrlen: c_int) -> *mut nlattr;
    fn genlmsg_attrlen(gnlh: *mut genlmsghdr, hdrlen: c_int) -> c_int;
    fn genlmsg_parse(
        nlh: *mut nlmsghdr,
        hdrlen: c_int,
        tb: *mut *mut nlattr,
        maxtype: c_int,
        policy: *mut c_void,
    ) -> c_int;
    fn nla_parse(
        tb: *mut *mut nlattr,
        maxtype: c_int,
        head: *mut nlattr,
        len: c_int,
        policy: *mut c_void,
    ) -> c_int;
    fn nla_data(nla: *mut nlattr) -> *mut c_void;
    fn nla_len(nla: *mut nlattr) -> c_int;
    fn nla_get_u32(nla: *mut nlattr) -> u32;
    fn nlmsg_alloc() -> *mut nl_msg;
    fn genl_ctrl_resolve(sk: *mut nl_sock, name: *const c_char) -> c_int;
    fn genlmsg_put(
        msg: *mut nl_msg,
        port: u32,
        seq: u32,
        family: c_int,
        hdrlen: c_int,
        flags: c_int,
        cmd: c_int,
        version: c_int,
    ) -> *mut c_void;
    fn nla_put_string(msg: *mut nl_msg, attrtype: c_int, str_: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn set_isst_id(id: *mut isst_id, cpu: c_int);
    fn process_level_change(id: *mut isst_id);
    fn nl_socket_alloc() -> *mut nl_sock;
    fn genl_connect(sk: *mut nl_sock) -> c_int;
    fn nl_cb_alloc(kind: c_int) -> *mut nl_cb;
    fn nl_socket_add_membership(sk: *mut nl_sock, group: c_int) -> c_int;
    fn nl_socket_free(sk: *mut nl_sock);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn debug_printf(format: *const c_char, ...) -> c_int;
    fn __cpuid(level: c_uint, eax: *mut c_uint, ebx: *mut c_uint, ecx: *mut c_uint, edx: *mut c_uint);
}

unsafe extern "C" fn ack_handler(_msg: *mut nl_msg, arg: *mut c_void) -> c_int {
    let err = arg as *mut c_int;
    *err = 0;
    NL_STOP
}

unsafe extern "C" fn finish_handler(_msg: *mut nl_msg, arg: *mut c_void) -> c_int {
    let ret = arg as *mut c_int;
    *ret = 0;
    NL_SKIP
}

unsafe extern "C" fn error_handler(
    _nla: *mut sockaddr_nl,
    err: *mut nlmsgerr,
    arg: *mut c_void,
) -> c_int {
    let ret = arg as *mut c_int;
    *ret = (*err).error;
    NL_SKIP
}

unsafe extern "C" fn seq_check_handler(_msg: *mut nl_msg, _arg: *mut c_void) -> c_int {
    NL_OK
}

unsafe fn send_and_recv_msgs(
    drv: *mut hfi_event_data,
    msg: *mut nl_msg,
    valid_handler: Option<unsafe extern "C" fn(*mut nl_msg, *mut c_void) -> c_int>,
    valid_data: *mut c_void,
) -> c_int {
    let cb: *mut nl_cb;
    let mut err: c_int = -ENOMEM;

    cb = nl_cb_clone((*drv).nl_cb);
    if cb.is_null() {
        nl_cb_put(cb);
        nlmsg_free(msg);
        return err;
    }

    err = nl_send_auto_complete((*drv).nl_handle, msg);
    if err >= 0 {
        err = 1;

        nl_cb_err(cb, NL_CB_CUSTOM, error_handler, &mut err as *mut c_int as *mut c_void);
        nl_cb_set(cb, NL_CB_FINISH, NL_CB_CUSTOM, finish_handler, &mut err as *mut c_int as *mut c_void);
        nl_cb_set(cb, NL_CB_ACK, NL_CB_CUSTOM, ack_handler, &mut err as *mut c_int as *mut c_void);

        if let Some(handler) = valid_handler {
            nl_cb_set(cb, NL_CB_VALID, NL_CB_CUSTOM, handler, valid_data);
        }

        while err > 0 {
            nl_recvmsgs((*drv).nl_handle, cb);
        }
    }

    nl_cb_put(cb);
    nlmsg_free(msg);
    err
}

#[repr(C)]
struct family_data {
    group: *const c_char,
    id: c_int,
}

unsafe extern "C" fn family_handler(msg: *mut nl_msg, arg: *mut c_void) -> c_int {
    let res = arg as *mut family_data;
    let mut tb: [*mut nlattr; CTRL_ATTR_MAX + 1] = [core::ptr::null_mut(); CTRL_ATTR_MAX + 1];
    let gnlh = nlmsg_data(nlmsg_hdr(msg)) as *mut genlmsghdr;
    let mut mcgrp: *mut nlattr;
    let mut i: c_int;

    nla_parse(
        tb.as_mut_ptr(),
        CTRL_ATTR_MAX as c_int,
        genlmsg_attrdata(gnlh, 0),
        genlmsg_attrlen(gnlh, 0),
        core::ptr::null_mut(),
    );
    if tb[CTRL_ATTR_MCAST_GROUPS].is_null() {
        return NL_SKIP;
    }

    /*
     * C source uses nla_for_each_nested(mcgrp, tb[CTRL_ATTR_MCAST_GROUPS], i).
     * The exact iterator expansion is supplied by libnl headers.
     */
    mcgrp = core::ptr::null_mut();
    i = 0;
    while !mcgrp.is_null() {
        let mut tb2: [*mut nlattr; CTRL_ATTR_MCAST_GRP_MAX + 1] =
            [core::ptr::null_mut(); CTRL_ATTR_MCAST_GRP_MAX + 1];
        nla_parse(
            tb2.as_mut_ptr(),
            CTRL_ATTR_MCAST_GRP_MAX as c_int,
            nla_data(mcgrp) as *mut nlattr,
            nla_len(mcgrp),
            core::ptr::null_mut(),
        );
        if tb2[CTRL_ATTR_MCAST_GRP_NAME].is_null()
            || tb2[CTRL_ATTR_MCAST_GRP_ID].is_null()
            || strncmp(
                nla_data(tb2[CTRL_ATTR_MCAST_GRP_NAME]) as *const c_char,
                (*res).group,
                nla_len(tb2[CTRL_ATTR_MCAST_GRP_NAME]) as usize,
            ) != 0
        {
            i += 1;
            continue;
        }
        (*res).id = nla_get_u32(tb2[CTRL_ATTR_MCAST_GRP_ID]) as c_int;
        break;
    }

    0
}

unsafe fn nl_get_multicast_id(
    drv: *mut hfi_event_data,
    family: *const c_char,
    group: *const c_char,
) -> c_int {
    let mut msg: *mut nl_msg;
    let mut ret: c_int = -1;
    let mut res = family_data { group, id: -ENOENT };

    msg = nlmsg_alloc();
    if msg.is_null() {
        return -ENOMEM;
    }
    genlmsg_put(
        msg,
        0,
        0,
        genl_ctrl_resolve((*drv).nl_handle, c"nlctrl".as_ptr()),
        0,
        0,
        CTRL_CMD_GETFAMILY,
        0,
    );
    if nla_put_string(msg, CTRL_ATTR_FAMILY_NAME, family) < 0 {
        nlmsg_free(msg);
        return ret;
    }

    ret = send_and_recv_msgs(
        drv,
        msg,
        Some(family_handler),
        &mut res as *mut family_data as *mut c_void,
    );
    msg = core::ptr::null_mut();
    if ret == 0 {
        ret = res.id;
    }

    nlmsg_free(msg);
    ret
}

#[repr(C)]
struct perf_cap {
    cpu: c_int,
    perf: c_int,
    eff: c_int,
}

unsafe fn process_hfi_event(perf_cap: *mut perf_cap) {
    let mut id = core::mem::MaybeUninit::<isst_id>::uninit();

    set_isst_id(id.as_mut_ptr(), (*perf_cap).cpu);
    process_level_change(id.as_mut_ptr());
}

unsafe extern "C" fn handle_event(n: *mut nl_msg, _arg: *mut c_void) -> c_int {
    let nlh = nlmsg_hdr(n);
    let genlhdr = genlmsg_hdr(nlh);
    let mut attrs: [*mut nlattr; THERMAL_GENL_ATTR_MAX + 1] =
        [core::ptr::null_mut(); THERMAL_GENL_ATTR_MAX + 1];
    let ret: c_int;
    let mut perf_cap = perf_cap {
        cpu: 0,
        perf: 0,
        eff: 0,
    };

    ret = genlmsg_parse(
        nlh,
        0,
        attrs.as_mut_ptr(),
        THERMAL_GENL_ATTR_MAX as c_int,
        core::ptr::null_mut(),
    );

    debug_printf(c"Received event %d parse_rer:%d\n".as_ptr(), (*genlhdr).cmd as c_int, ret);
    if (*genlhdr).cmd == THERMAL_GENL_EVENT_CPU_CAPABILITY_CHANGE {
        let mut cap: *mut nlattr;
        let mut j: c_int;
        let mut index: c_int = 0;

        debug_printf(c"THERMAL_GENL_EVENT_CPU_CAPABILITY_CHANGE\n".as_ptr());
        /*
         * C source uses nla_for_each_nested(cap, attrs[THERMAL_GENL_ATTR_CPU_CAPABILITY], j).
         * The exact iterator expansion is supplied by libnl headers.
         */
        cap = core::ptr::null_mut();
        j = 0;
        while !cap.is_null() {
            match index {
                0 => {
                    perf_cap.cpu = nla_get_u32(cap) as c_int;
                }
                1 => {
                    perf_cap.perf = nla_get_u32(cap) as c_int;
                }
                2 => {
                    perf_cap.eff = nla_get_u32(cap) as c_int;
                }
                _ => {}
            }
            index += 1;
            if index == 3 {
                index = 0;
                process_hfi_event(&mut perf_cap as *mut perf_cap);
            }
            j += 1;
        }
    }

    0
}

static mut _hfi_exit: c_int = 0;

unsafe fn check_hf_suport() -> c_int {
    let mut eax: c_uint = 0;
    let mut ebx: c_uint = 0;
    let mut ecx: c_uint = 0;
    let mut edx: c_uint = 0;

    __cpuid(6, &mut eax, &mut ebx, &mut ecx, &mut edx);
    if eax & (1u32 << 19) != 0 {
        return 1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hfi_main() -> c_int {
    let sock: *mut nl_sock;
    let cb: *mut nl_cb;
    let mut err: c_int = 0;
    let mcast_id: c_int;

    if check_hf_suport() == 0 {
        fprintf(stderr, c"CPU Doesn't support HFI\n".as_ptr());
        return -1;
    }

    sock = nl_socket_alloc();
    if sock.is_null() {
        fprintf(stderr, c"nl_socket_alloc failed\n".as_ptr());
        return -1;
    }

    if genl_connect(sock) != 0 {
        fprintf(stderr, c"genl_connect(sk_event) failed\n".as_ptr());
        nl_socket_free(sock);
        return -1;
    }

    drv.nl_handle = sock;
    cb = nl_cb_alloc(NL_CB_DEFAULT);
    drv.nl_cb = cb;
    if drv.nl_cb.is_null() {
        printf(c"Failed to allocate netlink callbacks".as_ptr());
        nl_socket_free(sock);
        return -1;
    }

    mcast_id = nl_get_multicast_id(
        &mut drv as *mut hfi_event_data,
        THERMAL_GENL_FAMILY_NAME,
        THERMAL_GENL_EVENT_GROUP_NAME,
    );
    if mcast_id < 0 {
        fprintf(stderr, c"nl_get_multicast_id failed\n".as_ptr());
        nl_socket_free(sock);
        return -1;
    }

    if nl_socket_add_membership(sock, mcast_id) != 0 {
        fprintf(stderr, c"nl_socket_add_membership failed".as_ptr());
        nl_socket_free(sock);
        return -1;
    }

    nl_cb_set(cb, NL_CB_SEQ_CHECK, NL_CB_CUSTOM, seq_check_handler, core::ptr::null_mut());
    nl_cb_set(cb, NL_CB_VALID, NL_CB_CUSTOM, handle_event, core::ptr::null_mut());

    debug_printf(c"hfi is initialized\n".as_ptr());

    while _hfi_exit == 0 && err == 0 {
        err = nl_recvmsgs(sock, cb);
        debug_printf(c"nl_recv_message err:%d\n".as_ptr(), err);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hfi_exit() {
    _hfi_exit = 1;
}
