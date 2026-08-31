// SPDX-License-Identifier: LGPL-2.1+
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>
//
// Translated from lib/thermal/commands.c. C include dependencies:
// <errno.h>, <stdio.h>, <stdlib.h>, <unistd.h>, <limits.h>,
// <thermal.h>, "thermal_nl.h".

use core::ffi::{c_char, c_int, c_void};

type size_t = usize;
type thermal_error_t = c_int;

const INT_MAX: c_int = c_int::MAX;

extern "C" {
    static THERMAL_GENL_ATTR_MAX: c_int;
    static THERMAL_GENL_ATTR_TZ: c_int;
    static THERMAL_GENL_ATTR_TZ_ID: c_int;
    static THERMAL_GENL_ATTR_TZ_TEMP: c_int;
    static THERMAL_GENL_ATTR_TZ_TRIP: c_int;
    static THERMAL_GENL_ATTR_TZ_TRIP_ID: c_int;
    static THERMAL_GENL_ATTR_TZ_TRIP_TEMP: c_int;
    static THERMAL_GENL_ATTR_TZ_TRIP_TYPE: c_int;
    static THERMAL_GENL_ATTR_TZ_TRIP_HYST: c_int;
    static THERMAL_GENL_ATTR_TZ_MODE: c_int;
    static THERMAL_GENL_ATTR_TZ_CDEV_WEIGHT: c_int;
    static THERMAL_GENL_ATTR_TZ_NAME: c_int;
    static THERMAL_GENL_ATTR_TZ_GOV: c_int;
    static THERMAL_GENL_ATTR_TZ_GOV_NAME: c_int;
    static THERMAL_GENL_ATTR_CDEV: c_int;
    static THERMAL_GENL_ATTR_CDEV_ID: c_int;
    static THERMAL_GENL_ATTR_CDEV_CUR_STATE: c_int;
    static THERMAL_GENL_ATTR_CDEV_MAX_STATE: c_int;
    static THERMAL_GENL_ATTR_CDEV_NAME: c_int;
    static THERMAL_GENL_ATTR_THRESHOLD: c_int;
    static THERMAL_GENL_ATTR_THRESHOLD_TEMP: c_int;
    static THERMAL_GENL_ATTR_THRESHOLD_DIRECTION: c_int;

    static THERMAL_GENL_CMD_TZ_GET_ID: c_int;
    static THERMAL_GENL_CMD_CDEV_GET: c_int;
    static THERMAL_GENL_CMD_TZ_GET_TEMP: c_int;
    static THERMAL_GENL_CMD_TZ_GET_TRIP: c_int;
    static THERMAL_GENL_CMD_TZ_GET_GOV: c_int;
    static THERMAL_GENL_CMD_THRESHOLD_GET: c_int;
    static THERMAL_GENL_CMD_THRESHOLD_ADD: c_int;
    static THERMAL_GENL_CMD_THRESHOLD_DELETE: c_int;
    static THERMAL_GENL_CMD_THRESHOLD_FLUSH: c_int;

    static THERMAL_GENL_VERSION: c_int;
    static THERMAL_NAME_LENGTH: size_t;
    static THERMAL_ERROR: thermal_error_t;
    static THERMAL_SUCCESS: thermal_error_t;

    static NLA_NESTED: c_int;
    static NLA_U32: c_int;
    static NLA_STRING: c_int;
    static NL_AUTO_PORT: c_int;
    static NL_AUTO_SEQ: c_int;
    static NLM_F_DUMP: c_int;
    static NLM_F_ACK: c_int;
    static GENL_ID_CTRL: c_int;
}

#[repr(C)]
pub struct nla_policy {
    pub type_: c_int,
}

#[repr(C)]
pub struct nlattr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nl_cache_ops {
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
pub struct genl_info {
    pub attrs: *mut *mut nlattr,
}

pub type genl_msg_parser_t = Option<
    unsafe extern "C" fn(
        *mut nl_cache_ops,
        *mut genl_cmd,
        *mut genl_info,
        *mut c_void,
    ) -> c_int,
>;

#[repr(C)]
pub struct genl_cmd {
    pub c_id: c_int,
    pub c_name: *mut c_char,
    pub c_msg_parser: genl_msg_parser_t,
    pub c_maxattr: c_int,
    pub c_attr_policy: *mut nla_policy,
}

#[repr(C)]
pub struct genl_ops {
    pub o_name: *mut c_char,
    pub o_cmds: *mut genl_cmd,
    pub o_ncmds: c_int,
    pub o_id: c_int,
}

#[repr(C)]
pub struct thermal_trip {
    pub id: c_int,
    pub type_: c_int,
    pub temp: c_int,
    pub hyst: c_int,
}

#[repr(C)]
pub struct thermal_threshold {
    pub temperature: c_int,
    pub direction: c_int,
}

#[repr(C)]
pub struct thermal_zone {
    pub id: c_int,
    pub name: *mut c_char,
    pub trip: *mut thermal_trip,
    pub temp: c_int,
    pub governor: *mut c_char,
    pub thresholds: *mut thermal_threshold,
}

#[repr(C)]
pub struct thermal_cdev {
    pub id: c_int,
    pub name: *mut c_char,
    pub cur_state: c_int,
    pub max_state: c_int,
}

#[repr(C)]
pub struct thermal_handler {
    pub sk_cmd: *mut nl_sock,
    pub cb_cmd: *mut nl_cb,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn nla_type(attr: *const nlattr) -> c_int;
    fn nla_get_u32(attr: *const nlattr) -> u32;
    fn nla_strlcpy(dst: *mut c_char, attr: *const nlattr, size: size_t) -> size_t;
    fn nla_put_u32(msg: *mut nl_msg, attrtype: c_int, value: u32) -> c_int;
    fn nlmsg_alloc() -> *mut nl_msg;
    fn nlmsg_free(msg: *mut nl_msg);
    fn genlmsg_put(
        msg: *mut nl_msg,
        port: c_int,
        seq: c_int,
        family: c_int,
        hdrlen: c_int,
        flags: c_int,
        cmd: c_int,
        version: c_int,
    ) -> *mut c_void;
    fn nl_send_msg(
        sk: *mut nl_sock,
        cb: *mut nl_cb,
        msg: *mut nl_msg,
        parser: unsafe extern "C" fn(*mut nl_msg, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn genl_handle_msg(msg: *mut nl_msg, arg: *mut c_void) -> c_int;
    fn genl_unregister_family(ops: *mut genl_ops) -> c_int;
    fn nl_thermal_disconnect(sk: *mut nl_sock, cb: *mut nl_cb);
    fn nl_thermal_connect(sk: *mut *mut nl_sock, cb: *mut *mut nl_cb) -> c_int;
    fn genl_register_family(ops: *mut genl_ops) -> c_int;
    fn genl_ops_resolve(sk: *mut nl_sock, ops: *mut genl_ops) -> c_int;
    fn genl_ctrl_resolve(sk: *mut nl_sock, name: *const c_char) -> c_int;

    // Rust-side dependency equivalent of the C nla_for_each_nested macro.
    fn nla_for_each_nested_start(attr: *mut nlattr, nested: *mut nlattr, rem: *mut c_int) -> *mut nlattr;
    fn nla_for_each_nested_next(attr: *mut nlattr, rem: *mut c_int) -> *mut nlattr;
}

const THERMAL_GENL_POLICY_LEN: usize = 256;

static mut thermal_genl_policy: [nla_policy; THERMAL_GENL_POLICY_LEN] =
    [nla_policy { type_: 0 }; THERMAL_GENL_POLICY_LEN];

unsafe fn init_thermal_genl_policy() {
    // Thermal zone
    thermal_genl_policy[THERMAL_GENL_ATTR_TZ as usize].type_ = NLA_NESTED;
    thermal_genl_policy[THERMAL_GENL_ATTR_TZ_ID as usize].type_ = NLA_U32;
    thermal_genl_policy[THERMAL_GENL_ATTR_TZ_TEMP as usize].type_ = NLA_U32;
    thermal_genl_policy[THERMAL_GENL_ATTR_TZ_TRIP as usize].type_ = NLA_NESTED;
    thermal_genl_policy[THERMAL_GENL_ATTR_TZ_TRIP_ID as usize].type_ = NLA_U32;
    thermal_genl_policy[THERMAL_GENL_ATTR_TZ_TRIP_TEMP as usize].type_ = NLA_U32;
    thermal_genl_policy[THERMAL_GENL_ATTR_TZ_TRIP_TYPE as usize].type_ = NLA_U32;
    thermal_genl_policy[THERMAL_GENL_ATTR_TZ_TRIP_HYST as usize].type_ = NLA_U32;
    thermal_genl_policy[THERMAL_GENL_ATTR_TZ_MODE as usize].type_ = NLA_U32;
    thermal_genl_policy[THERMAL_GENL_ATTR_TZ_CDEV_WEIGHT as usize].type_ = NLA_U32;
    thermal_genl_policy[THERMAL_GENL_ATTR_TZ_NAME as usize].type_ = NLA_STRING;

    // Governor(s)
    thermal_genl_policy[THERMAL_GENL_ATTR_TZ_GOV as usize].type_ = NLA_NESTED;
    thermal_genl_policy[THERMAL_GENL_ATTR_TZ_GOV_NAME as usize].type_ = NLA_STRING;

    // Cooling devices
    thermal_genl_policy[THERMAL_GENL_ATTR_CDEV as usize].type_ = NLA_NESTED;
    thermal_genl_policy[THERMAL_GENL_ATTR_CDEV_ID as usize].type_ = NLA_U32;
    thermal_genl_policy[THERMAL_GENL_ATTR_CDEV_CUR_STATE as usize].type_ = NLA_U32;
    thermal_genl_policy[THERMAL_GENL_ATTR_CDEV_MAX_STATE as usize].type_ = NLA_U32;
    thermal_genl_policy[THERMAL_GENL_ATTR_CDEV_NAME as usize].type_ = NLA_STRING;

    // Thresholds
    thermal_genl_policy[THERMAL_GENL_ATTR_THRESHOLD as usize].type_ = NLA_NESTED;
    thermal_genl_policy[THERMAL_GENL_ATTR_THRESHOLD_TEMP as usize].type_ = NLA_U32;
    thermal_genl_policy[THERMAL_GENL_ATTR_THRESHOLD_DIRECTION as usize].type_ = NLA_U32;
}

unsafe extern "C" fn parse_tz_get(
    info: *mut genl_info,
    tz: *mut *mut thermal_zone,
) -> c_int {
    let mut attr: *mut nlattr;
    let mut __tz: *mut thermal_zone = core::ptr::null_mut();
    let mut size: size_t = 0;
    let mut rem: c_int = 0;

    attr = nla_for_each_nested_start(
        core::ptr::null_mut(),
        *(*info).attrs.add(THERMAL_GENL_ATTR_TZ as usize),
        &mut rem,
    );
    while !attr.is_null() {
        if nla_type(attr) == THERMAL_GENL_ATTR_TZ_ID {
            size += 1;

            __tz = realloc(
                __tz as *mut c_void,
                core::mem::size_of::<thermal_zone>() * (size + 2),
            ) as *mut thermal_zone;
            if __tz.is_null() {
                return THERMAL_ERROR;
            }

            (*__tz.add(size - 1)).id = nla_get_u32(attr) as c_int;
        }

        if nla_type(attr) == THERMAL_GENL_ATTR_TZ_NAME {
            nla_strlcpy(
                (*__tz.add(size - 1)).name,
                attr,
                THERMAL_NAME_LENGTH,
            );
        }

        attr = nla_for_each_nested_next(attr, &mut rem);
    }

    if !__tz.is_null() {
        (*__tz.add(size)).id = -1;
    }

    *tz = __tz;

    THERMAL_SUCCESS
}

unsafe extern "C" fn parse_cdev_get(
    info: *mut genl_info,
    cdev: *mut *mut thermal_cdev,
) -> c_int {
    let mut attr: *mut nlattr;
    let mut __cdev: *mut thermal_cdev = core::ptr::null_mut();
    let mut size: size_t = 0;
    let mut rem: c_int = 0;

    attr = nla_for_each_nested_start(
        core::ptr::null_mut(),
        *(*info).attrs.add(THERMAL_GENL_ATTR_CDEV as usize),
        &mut rem,
    );
    while !attr.is_null() {
        if nla_type(attr) == THERMAL_GENL_ATTR_CDEV_ID {
            size += 1;

            __cdev = realloc(
                __cdev as *mut c_void,
                core::mem::size_of::<thermal_cdev>() * (size + 2),
            ) as *mut thermal_cdev;
            if __cdev.is_null() {
                return THERMAL_ERROR;
            }

            (*__cdev.add(size - 1)).id = nla_get_u32(attr) as c_int;
        }

        if nla_type(attr) == THERMAL_GENL_ATTR_CDEV_NAME {
            nla_strlcpy(
                (*__cdev.add(size - 1)).name,
                attr,
                THERMAL_NAME_LENGTH,
            );
        }

        if nla_type(attr) == THERMAL_GENL_ATTR_CDEV_CUR_STATE {
            (*__cdev.add(size - 1)).cur_state = nla_get_u32(attr) as c_int;
        }

        if nla_type(attr) == THERMAL_GENL_ATTR_CDEV_MAX_STATE {
            (*__cdev.add(size - 1)).max_state = nla_get_u32(attr) as c_int;
        }

        attr = nla_for_each_nested_next(attr, &mut rem);
    }

    if !__cdev.is_null() {
        (*__cdev.add(size)).id = -1;
    }

    *cdev = __cdev;

    THERMAL_SUCCESS
}

unsafe extern "C" fn parse_tz_get_trip(
    info: *mut genl_info,
    tz: *mut thermal_zone,
) -> c_int {
    let mut attr: *mut nlattr;
    let mut __tt: *mut thermal_trip = core::ptr::null_mut();
    let mut size: size_t = 0;
    let mut rem: c_int = 0;

    attr = nla_for_each_nested_start(
        core::ptr::null_mut(),
        *(*info).attrs.add(THERMAL_GENL_ATTR_TZ_TRIP as usize),
        &mut rem,
    );
    while !attr.is_null() {
        if nla_type(attr) == THERMAL_GENL_ATTR_TZ_TRIP_ID {
            size += 1;

            __tt = realloc(
                __tt as *mut c_void,
                core::mem::size_of::<thermal_trip>() * (size + 2),
            ) as *mut thermal_trip;
            if __tt.is_null() {
                return THERMAL_ERROR;
            }

            (*__tt.add(size - 1)).id = nla_get_u32(attr) as c_int;
        }

        if nla_type(attr) == THERMAL_GENL_ATTR_TZ_TRIP_TYPE {
            (*__tt.add(size - 1)).type_ = nla_get_u32(attr) as c_int;
        }

        if nla_type(attr) == THERMAL_GENL_ATTR_TZ_TRIP_TEMP {
            (*__tt.add(size - 1)).temp = nla_get_u32(attr) as c_int;
        }

        if nla_type(attr) == THERMAL_GENL_ATTR_TZ_TRIP_HYST {
            (*__tt.add(size - 1)).hyst = nla_get_u32(attr) as c_int;
        }

        attr = nla_for_each_nested_next(attr, &mut rem);
    }

    if !__tt.is_null() {
        (*__tt.add(size)).id = -1;
    }

    (*tz).trip = __tt;

    THERMAL_SUCCESS
}

unsafe extern "C" fn parse_tz_get_temp(
    info: *mut genl_info,
    tz: *mut thermal_zone,
) -> c_int {
    let mut id: c_int = -1;

    if !(*(*info).attrs.add(THERMAL_GENL_ATTR_TZ_ID as usize)).is_null() {
        id = nla_get_u32(*(*info).attrs.add(THERMAL_GENL_ATTR_TZ_ID as usize)) as c_int;
    }

    if (*tz).id != id {
        return THERMAL_ERROR;
    }

    if !(*(*info).attrs.add(THERMAL_GENL_ATTR_TZ_TEMP as usize)).is_null() {
        (*tz).temp = nla_get_u32(*(*info).attrs.add(THERMAL_GENL_ATTR_TZ_TEMP as usize)) as c_int;
    }

    THERMAL_SUCCESS
}

unsafe extern "C" fn parse_tz_get_gov(
    info: *mut genl_info,
    tz: *mut thermal_zone,
) -> c_int {
    let mut id: c_int = -1;

    if !(*(*info).attrs.add(THERMAL_GENL_ATTR_TZ_ID as usize)).is_null() {
        id = nla_get_u32(*(*info).attrs.add(THERMAL_GENL_ATTR_TZ_ID as usize)) as c_int;
    }

    if (*tz).id != id {
        return THERMAL_ERROR;
    }

    if !(*(*info).attrs.add(THERMAL_GENL_ATTR_TZ_GOV_NAME as usize)).is_null() {
        nla_strlcpy(
            (*tz).governor,
            *(*info).attrs.add(THERMAL_GENL_ATTR_TZ_GOV_NAME as usize),
            THERMAL_NAME_LENGTH,
        );
    }

    THERMAL_SUCCESS
}

unsafe extern "C" fn parse_threshold_get(
    info: *mut genl_info,
    tz: *mut thermal_zone,
) -> c_int {
    let mut attr: *mut nlattr;
    let mut __tt: *mut thermal_threshold = core::ptr::null_mut();
    let mut size: size_t = 0;
    let mut rem: c_int = 0;

    /*
     * The size contains the size of the array and we want to
     * access the last element, size - 1.
     *
     * The variable size is initialized to zero but it will be
     * then incremented by the first if() statement. The message
     * attributes are ordered, so the first if() statement will be
     * always called before the second one. If it happens that is
     * not the case, then it is a kernel bug.
     */
    attr = nla_for_each_nested_start(
        core::ptr::null_mut(),
        *(*info).attrs.add(THERMAL_GENL_ATTR_THRESHOLD as usize),
        &mut rem,
    );
    while !attr.is_null() {
        if nla_type(attr) == THERMAL_GENL_ATTR_THRESHOLD_TEMP {
            size += 1;

            __tt = realloc(
                __tt as *mut c_void,
                core::mem::size_of::<thermal_threshold>() * (size + 2),
            ) as *mut thermal_threshold;
            if __tt.is_null() {
                return THERMAL_ERROR;
            }

            (*__tt.add(size - 1)).temperature = nla_get_u32(attr) as c_int;
        }

        if nla_type(attr) == THERMAL_GENL_ATTR_THRESHOLD_DIRECTION {
            (*__tt.add(size - 1)).direction = nla_get_u32(attr) as c_int;
        }

        attr = nla_for_each_nested_next(attr, &mut rem);
    }

    if !__tt.is_null() {
        (*__tt.add(size)).temperature = INT_MAX;
    }

    (*tz).thresholds = __tt;

    THERMAL_SUCCESS
}

unsafe extern "C" fn handle_netlink(
    _unused: *mut nl_cache_ops,
    cmd: *mut genl_cmd,
    info: *mut genl_info,
    arg: *mut c_void,
) -> c_int {
    let ret: c_int;

    if (*cmd).c_id == THERMAL_GENL_CMD_TZ_GET_ID {
        ret = parse_tz_get(info, arg as *mut *mut thermal_zone);
    } else if (*cmd).c_id == THERMAL_GENL_CMD_CDEV_GET {
        ret = parse_cdev_get(info, arg as *mut *mut thermal_cdev);
    } else if (*cmd).c_id == THERMAL_GENL_CMD_TZ_GET_TEMP {
        ret = parse_tz_get_temp(info, arg as *mut thermal_zone);
    } else if (*cmd).c_id == THERMAL_GENL_CMD_TZ_GET_TRIP {
        ret = parse_tz_get_trip(info, arg as *mut thermal_zone);
    } else if (*cmd).c_id == THERMAL_GENL_CMD_TZ_GET_GOV {
        ret = parse_tz_get_gov(info, arg as *mut thermal_zone);
    } else if (*cmd).c_id == THERMAL_GENL_CMD_THRESHOLD_GET {
        ret = parse_threshold_get(info, arg as *mut thermal_zone);
    } else {
        return THERMAL_ERROR;
    }

    ret
}

static mut thermal_cmds: [genl_cmd; 9] = [
    genl_cmd {
        c_id: 0,
        c_name: b"List thermal zones\0".as_ptr() as *mut c_char,
        c_msg_parser: Some(handle_netlink),
        c_maxattr: 0,
        c_attr_policy: core::ptr::null_mut(),
    },
    genl_cmd {
        c_id: 0,
        c_name: b"Get governor\0".as_ptr() as *mut c_char,
        c_msg_parser: Some(handle_netlink),
        c_maxattr: 0,
        c_attr_policy: core::ptr::null_mut(),
    },
    genl_cmd {
        c_id: 0,
        c_name: b"Get thermal zone temperature\0".as_ptr() as *mut c_char,
        c_msg_parser: Some(handle_netlink),
        c_maxattr: 0,
        c_attr_policy: core::ptr::null_mut(),
    },
    genl_cmd {
        c_id: 0,
        c_name: b"Get thermal zone trip points\0".as_ptr() as *mut c_char,
        c_msg_parser: Some(handle_netlink),
        c_maxattr: 0,
        c_attr_policy: core::ptr::null_mut(),
    },
    genl_cmd {
        c_id: 0,
        c_name: b"Get cooling devices\0".as_ptr() as *mut c_char,
        c_msg_parser: Some(handle_netlink),
        c_maxattr: 0,
        c_attr_policy: core::ptr::null_mut(),
    },
    genl_cmd {
        c_id: 0,
        c_name: b"Get thresholds list\0".as_ptr() as *mut c_char,
        c_msg_parser: Some(handle_netlink),
        c_maxattr: 0,
        c_attr_policy: core::ptr::null_mut(),
    },
    genl_cmd {
        c_id: 0,
        c_name: b"Add a threshold\0".as_ptr() as *mut c_char,
        c_msg_parser: Some(handle_netlink),
        c_maxattr: 0,
        c_attr_policy: core::ptr::null_mut(),
    },
    genl_cmd {
        c_id: 0,
        c_name: b"Delete a threshold\0".as_ptr() as *mut c_char,
        c_msg_parser: Some(handle_netlink),
        c_maxattr: 0,
        c_attr_policy: core::ptr::null_mut(),
    },
    genl_cmd {
        c_id: 0,
        c_name: b"Flush the thresholds\0".as_ptr() as *mut c_char,
        c_msg_parser: Some(handle_netlink),
        c_maxattr: 0,
        c_attr_policy: core::ptr::null_mut(),
    },
];

unsafe fn init_thermal_cmds() {
    let ids = [
        THERMAL_GENL_CMD_TZ_GET_ID,
        THERMAL_GENL_CMD_TZ_GET_GOV,
        THERMAL_GENL_CMD_TZ_GET_TEMP,
        THERMAL_GENL_CMD_TZ_GET_TRIP,
        THERMAL_GENL_CMD_CDEV_GET,
        THERMAL_GENL_CMD_THRESHOLD_GET,
        THERMAL_GENL_CMD_THRESHOLD_ADD,
        THERMAL_GENL_CMD_THRESHOLD_DELETE,
        THERMAL_GENL_CMD_THRESHOLD_FLUSH,
    ];
    for i in 0..thermal_cmds.len() {
        thermal_cmds[i].c_id = ids[i];
        thermal_cmds[i].c_maxattr = THERMAL_GENL_ATTR_MAX;
        thermal_cmds[i].c_attr_policy = thermal_genl_policy.as_mut_ptr();
    }
}

static mut thermal_cmd_ops: genl_ops = genl_ops {
    o_name: b"thermal\0".as_ptr() as *mut c_char,
    o_cmds: core::ptr::null_mut(),
    o_ncmds: 9,
    o_id: 0,
};

unsafe fn init_thermal_cmd_ops() {
    thermal_cmd_ops.o_cmds = thermal_cmds.as_mut_ptr();
    thermal_cmd_ops.o_ncmds = thermal_cmds.len() as c_int;
}

#[repr(C)]
struct cmd_param {
    tz_id: c_int,
    temp: c_int,
    direction: c_int,
}

type cmd_cb_t = Option<unsafe extern "C" fn(*mut nl_msg, *mut cmd_param) -> c_int>;

unsafe extern "C" fn thermal_genl_tz_id_encode(
    msg: *mut nl_msg,
    p: *mut cmd_param,
) -> c_int {
    if nla_put_u32(msg, THERMAL_GENL_ATTR_TZ_ID, (*p).tz_id as u32) != 0 {
        return -1;
    }

    0
}

unsafe extern "C" fn thermal_genl_threshold_encode(
    msg: *mut nl_msg,
    p: *mut cmd_param,
) -> c_int {
    if thermal_genl_tz_id_encode(msg, p) != 0 {
        return -1;
    }

    if nla_put_u32(msg, THERMAL_GENL_ATTR_THRESHOLD_TEMP, (*p).temp as u32) != 0 {
        return -1;
    }

    if nla_put_u32(
        msg,
        THERMAL_GENL_ATTR_THRESHOLD_DIRECTION,
        (*p).direction as u32,
    ) != 0
    {
        return -1;
    }

    0
}

unsafe extern "C" fn thermal_genl_auto(
    th: *mut thermal_handler,
    cmd_cb: cmd_cb_t,
    param: *mut cmd_param,
    cmd: c_int,
    flags: c_int,
    arg: *mut c_void,
) -> thermal_error_t {
    let mut ret: thermal_error_t = THERMAL_ERROR;
    let msg: *mut nl_msg;
    let hdr: *mut c_void;

    msg = nlmsg_alloc();
    if msg.is_null() {
        return THERMAL_ERROR;
    }

    hdr = genlmsg_put(
        msg,
        NL_AUTO_PORT,
        NL_AUTO_SEQ,
        thermal_cmd_ops.o_id,
        0,
        flags,
        cmd,
        THERMAL_GENL_VERSION,
    );
    if hdr.is_null() {
        goto_out(msg, ret);
        return ret;
    }

    if let Some(cb) = cmd_cb {
        if cb(msg, param) != 0 {
            goto_out(msg, ret);
            return ret;
        }
    }

    if nl_send_msg((*th).sk_cmd, (*th).cb_cmd, msg, genl_handle_msg, arg) != 0 {
        goto_out(msg, ret);
        return ret;
    }

    ret = THERMAL_SUCCESS;
    goto_out(msg, ret);
    ret
}

unsafe fn goto_out(msg: *mut nl_msg, _ret: thermal_error_t) {
    nlmsg_free(msg);
}

#[no_mangle]
pub unsafe extern "C" fn thermal_cmd_get_tz(
    th: *mut thermal_handler,
    tz: *mut *mut thermal_zone,
) -> thermal_error_t {
    thermal_genl_auto(
        th,
        None,
        core::ptr::null_mut(),
        THERMAL_GENL_CMD_TZ_GET_ID,
        NLM_F_DUMP | NLM_F_ACK,
        tz as *mut c_void,
    )
}

#[no_mangle]
pub unsafe extern "C" fn thermal_cmd_get_cdev(
    th: *mut thermal_handler,
    tc: *mut *mut thermal_cdev,
) -> thermal_error_t {
    thermal_genl_auto(
        th,
        None,
        core::ptr::null_mut(),
        THERMAL_GENL_CMD_CDEV_GET,
        NLM_F_DUMP | NLM_F_ACK,
        tc as *mut c_void,
    )
}

#[no_mangle]
pub unsafe extern "C" fn thermal_cmd_get_trip(
    th: *mut thermal_handler,
    tz: *mut thermal_zone,
) -> thermal_error_t {
    let mut p = cmd_param {
        tz_id: (*tz).id,
        temp: 0,
        direction: 0,
    };

    thermal_genl_auto(
        th,
        Some(thermal_genl_tz_id_encode),
        &mut p,
        THERMAL_GENL_CMD_TZ_GET_TRIP,
        0,
        tz as *mut c_void,
    )
}

#[no_mangle]
pub unsafe extern "C" fn thermal_cmd_get_governor(
    th: *mut thermal_handler,
    tz: *mut thermal_zone,
) -> thermal_error_t {
    let mut p = cmd_param {
        tz_id: (*tz).id,
        temp: 0,
        direction: 0,
    };

    thermal_genl_auto(
        th,
        Some(thermal_genl_tz_id_encode),
        &mut p,
        THERMAL_GENL_CMD_TZ_GET_GOV,
        0,
        tz as *mut c_void,
    )
}

#[no_mangle]
pub unsafe extern "C" fn thermal_cmd_get_temp(
    th: *mut thermal_handler,
    tz: *mut thermal_zone,
) -> thermal_error_t {
    let mut p = cmd_param {
        tz_id: (*tz).id,
        temp: 0,
        direction: 0,
    };

    thermal_genl_auto(
        th,
        Some(thermal_genl_tz_id_encode),
        &mut p,
        THERMAL_GENL_CMD_TZ_GET_TEMP,
        0,
        tz as *mut c_void,
    )
}

#[no_mangle]
pub unsafe extern "C" fn thermal_cmd_threshold_get(
    th: *mut thermal_handler,
    tz: *mut thermal_zone,
) -> thermal_error_t {
    let mut p = cmd_param {
        tz_id: (*tz).id,
        temp: 0,
        direction: 0,
    };

    thermal_genl_auto(
        th,
        Some(thermal_genl_tz_id_encode),
        &mut p,
        THERMAL_GENL_CMD_THRESHOLD_GET,
        0,
        tz as *mut c_void,
    )
}

#[no_mangle]
pub unsafe extern "C" fn thermal_cmd_threshold_add(
    th: *mut thermal_handler,
    tz: *mut thermal_zone,
    temperature: c_int,
    direction: c_int,
) -> thermal_error_t {
    let mut p = cmd_param {
        tz_id: (*tz).id,
        temp: temperature,
        direction,
    };

    thermal_genl_auto(
        th,
        Some(thermal_genl_threshold_encode),
        &mut p,
        THERMAL_GENL_CMD_THRESHOLD_ADD,
        0,
        tz as *mut c_void,
    )
}

#[no_mangle]
pub unsafe extern "C" fn thermal_cmd_threshold_delete(
    th: *mut thermal_handler,
    tz: *mut thermal_zone,
    temperature: c_int,
    direction: c_int,
) -> thermal_error_t {
    let mut p = cmd_param {
        tz_id: (*tz).id,
        temp: temperature,
        direction,
    };

    thermal_genl_auto(
        th,
        Some(thermal_genl_threshold_encode),
        &mut p,
        THERMAL_GENL_CMD_THRESHOLD_DELETE,
        0,
        tz as *mut c_void,
    )
}

#[no_mangle]
pub unsafe extern "C" fn thermal_cmd_threshold_flush(
    th: *mut thermal_handler,
    tz: *mut thermal_zone,
) -> thermal_error_t {
    let mut p = cmd_param {
        tz_id: (*tz).id,
        temp: 0,
        direction: 0,
    };

    thermal_genl_auto(
        th,
        Some(thermal_genl_tz_id_encode),
        &mut p,
        THERMAL_GENL_CMD_THRESHOLD_FLUSH,
        0,
        tz as *mut c_void,
    )
}

#[no_mangle]
pub unsafe extern "C" fn thermal_cmd_exit(th: *mut thermal_handler) -> thermal_error_t {
    if genl_unregister_family(&mut thermal_cmd_ops) != 0 {
        return THERMAL_ERROR;
    }

    nl_thermal_disconnect((*th).sk_cmd, (*th).cb_cmd);

    THERMAL_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn thermal_cmd_init(th: *mut thermal_handler) -> thermal_error_t {
    let mut ret: c_int;
    let family: c_int;

    init_thermal_genl_policy();
    init_thermal_cmds();
    init_thermal_cmd_ops();

    if nl_thermal_connect(&mut (*th).sk_cmd, &mut (*th).cb_cmd) != 0 {
        return THERMAL_ERROR;
    }

    ret = genl_register_family(&mut thermal_cmd_ops);
    if ret != 0 {
        return THERMAL_ERROR;
    }

    ret = genl_ops_resolve((*th).sk_cmd, &mut thermal_cmd_ops);
    if ret != 0 {
        return THERMAL_ERROR;
    }

    family = genl_ctrl_resolve((*th).sk_cmd, b"nlctrl\0".as_ptr() as *const c_char);
    if family != GENL_ID_CTRL {
        return THERMAL_ERROR;
    }

    THERMAL_SUCCESS
}
