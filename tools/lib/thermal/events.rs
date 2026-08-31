// SPDX-License-Identifier: LGPL-2.1+
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>

use core::ffi::{c_char, c_int, c_void};

// Dependencies from <linux/netlink.h>, <thermal.h>, and "thermal_nl.h" are
// expected to be supplied by the translated repository.

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
pub struct nl_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nl_cb {
    _private: [u8; 0],
}

pub type thermal_error_t = c_int;

#[repr(C)]
pub struct thermal_handler_param {
    pub th: *mut thermal_handler,
    pub arg: *mut c_void,
}

#[repr(C)]
pub struct thermal_handler {
    pub ops: *mut thermal_ops,
    pub sk_event: *mut nl_sock,
    pub cb_event: *mut nl_cb,
}

#[repr(C)]
pub struct thermal_ops {
    pub events: thermal_events_ops,
}

#[repr(C)]
pub struct thermal_events_ops {
    pub tz_create: Option<unsafe extern "C" fn(*const c_char, u32, *mut c_void) -> thermal_error_t>,
    pub tz_delete: Option<unsafe extern "C" fn(u32, *mut c_void) -> thermal_error_t>,
    pub tz_enable: Option<unsafe extern "C" fn(u32, *mut c_void) -> thermal_error_t>,
    pub tz_disable: Option<unsafe extern "C" fn(u32, *mut c_void) -> thermal_error_t>,
    pub trip_change: Option<unsafe extern "C" fn(u32, u32, u32, u32, u32, *mut c_void) -> thermal_error_t>,
    pub trip_add: Option<unsafe extern "C" fn(u32, u32, u32, u32, u32, *mut c_void) -> thermal_error_t>,
    pub trip_delete: Option<unsafe extern "C" fn(u32, u32, *mut c_void) -> thermal_error_t>,
    pub trip_high: Option<unsafe extern "C" fn(u32, u32, u32, *mut c_void) -> thermal_error_t>,
    pub trip_low: Option<unsafe extern "C" fn(u32, u32, u32, *mut c_void) -> thermal_error_t>,
    pub cdev_add: Option<unsafe extern "C" fn(*const c_char, u32, u32, *mut c_void) -> thermal_error_t>,
    pub cdev_delete: Option<unsafe extern "C" fn(u32, *mut c_void) -> thermal_error_t>,
    pub cdev_update: Option<unsafe extern "C" fn(u32, u32, *mut c_void) -> thermal_error_t>,
    pub gov_change: Option<unsafe extern "C" fn(u32, *const c_char, *mut c_void) -> thermal_error_t>,
    pub threshold_add: Option<unsafe extern "C" fn(u32, u32, u32, *mut c_void) -> thermal_error_t>,
    pub threshold_delete: Option<unsafe extern "C" fn(u32, u32, u32, *mut c_void) -> thermal_error_t>,
    pub threshold_flush: Option<unsafe extern "C" fn(u32, *mut c_void) -> thermal_error_t>,
    pub threshold_up: Option<unsafe extern "C" fn(u32, u32, u32, *mut c_void) -> thermal_error_t>,
    pub threshold_down: Option<unsafe extern "C" fn(u32, u32, u32, *mut c_void) -> thermal_error_t>,
}

unsafe extern "C" {
    fn nlmsg_hdr(n: *mut nl_msg) -> *mut nlmsghdr;
    fn genlmsg_hdr(nlh: *mut nlmsghdr) -> *mut genlmsghdr;
    fn genlmsg_parse(
        nlh: *mut nlmsghdr,
        hdrlen: c_int,
        tb: *mut *mut nlattr,
        maxtype: c_int,
        policy: *mut c_void,
    ) -> c_int;
    fn nla_get_string(nla: *mut nlattr) -> *const c_char;
    fn nla_get_u32(nla: *mut nlattr) -> u32;
    fn nl_cb_set(
        cb: *mut nl_cb,
        kind: c_int,
        mode: c_int,
        func: Option<unsafe extern "C" fn(*mut nl_msg, *mut c_void) -> c_int>,
        arg: *mut c_void,
    ) -> c_int;
    fn nl_recvmsgs(sk: *mut nl_sock, cb: *mut nl_cb) -> c_int;
    fn nl_socket_get_fd(sk: *mut nl_sock) -> c_int;
    fn nl_unsubscribe_thermal(sk: *mut nl_sock, cb: *mut nl_cb, group: *const c_char) -> c_int;
    fn nl_thermal_disconnect(sk: *mut nl_sock, cb: *mut nl_cb);
    fn nl_thermal_connect(sk: *mut *mut nl_sock, cb: *mut *mut nl_cb) -> c_int;
    fn nl_subscribe_thermal(sk: *mut nl_sock, cb: *mut nl_cb, group: *const c_char) -> c_int;
}

unsafe extern "C" {
    static THERMAL_GENL_EVENT_GROUP_NAME: *const c_char;
}

unsafe extern "C" {
    static __THERMAL_GENL_EVENT_MAX: usize;
    static THERMAL_GENL_ATTR_MAX: usize;
    static THERMAL_SUCCESS: thermal_error_t;
    static THERMAL_ERROR: thermal_error_t;
    static NL_CB_VALID: c_int;
    static NL_CB_CUSTOM: c_int;
    static THERMAL_GENL_EVENT_TZ_CREATE: usize;
    static THERMAL_GENL_EVENT_TZ_DELETE: usize;
    static THERMAL_GENL_EVENT_TZ_ENABLE: usize;
    static THERMAL_GENL_EVENT_TZ_DISABLE: usize;
    static THERMAL_GENL_EVENT_TZ_TRIP_CHANGE: usize;
    static THERMAL_GENL_EVENT_TZ_TRIP_ADD: usize;
    static THERMAL_GENL_EVENT_TZ_TRIP_DELETE: usize;
    static THERMAL_GENL_EVENT_TZ_TRIP_UP: usize;
    static THERMAL_GENL_EVENT_TZ_TRIP_DOWN: usize;
    static THERMAL_GENL_EVENT_CDEV_ADD: usize;
    static THERMAL_GENL_EVENT_CDEV_DELETE: usize;
    static THERMAL_GENL_EVENT_CDEV_STATE_UPDATE: usize;
    static THERMAL_GENL_EVENT_TZ_GOV_CHANGE: usize;
    static THERMAL_GENL_EVENT_THRESHOLD_ADD: usize;
    static THERMAL_GENL_EVENT_THRESHOLD_DELETE: usize;
    static THERMAL_GENL_EVENT_THRESHOLD_FLUSH: usize;
    static THERMAL_GENL_EVENT_THRESHOLD_UP: usize;
    static THERMAL_GENL_EVENT_THRESHOLD_DOWN: usize;
    static THERMAL_GENL_ATTR_TZ_NAME: usize;
    static THERMAL_GENL_ATTR_TZ_ID: usize;
    static THERMAL_GENL_ATTR_TZ_TRIP_ID: usize;
    static THERMAL_GENL_ATTR_TZ_TRIP_TYPE: usize;
    static THERMAL_GENL_ATTR_TZ_TRIP_TEMP: usize;
    static THERMAL_GENL_ATTR_TZ_TRIP_HYST: usize;
    static THERMAL_GENL_ATTR_TZ_TEMP: usize;
    static THERMAL_GENL_ATTR_CDEV_NAME: usize;
    static THERMAL_GENL_ATTR_CDEV_ID: usize;
    static THERMAL_GENL_ATTR_CDEV_MAX_STATE: usize;
    static THERMAL_GENL_ATTR_CDEV_CUR_STATE: usize;
    static THERMAL_GENL_ATTR_GOV_NAME: usize;
    static THERMAL_GENL_ATTR_THRESHOLD_TEMP: usize;
    static THERMAL_GENL_ATTR_THRESHOLD_DIRECTION: usize;
    static THERMAL_GENL_ATTR_TZ_PREV_TEMP: usize;
}

/*
 * Optimization: fill this array to tell which event we do want to pay
 * attention to. That happens at init time with the ops
 * structure. Each ops will enable the event and the general handler
 * will be able to discard the event if there is not ops associated
 * with it.
 */
static mut enabled_ops: [c_int; __THERMAL_GENL_EVENT_MAX] = [0; __THERMAL_GENL_EVENT_MAX];

unsafe extern "C" fn handle_thermal_event(n: *mut nl_msg, mut arg: *mut c_void) -> c_int {
    let nlh: *mut nlmsghdr = nlmsg_hdr(n);
    let genlhdr: *mut genlmsghdr = genlmsg_hdr(nlh);
    let mut attrs: [*mut nlattr; THERMAL_GENL_ATTR_MAX + 1] =
        [core::ptr::null_mut(); THERMAL_GENL_ATTR_MAX + 1];
    let thp: *mut thermal_handler_param = arg as *mut thermal_handler_param;
    let ops: *mut thermal_events_ops = &mut (*(*(*thp).th).ops).events;

    genlmsg_parse(
        nlh,
        0,
        attrs.as_mut_ptr(),
        THERMAL_GENL_ATTR_MAX as c_int,
        core::ptr::null_mut(),
    );

    arg = (*thp).arg;

    /*
     * This is an event we don't care of, bail out.
     */
    if enabled_ops[(*genlhdr).cmd as usize] == 0 {
        return THERMAL_SUCCESS;
    }

    match (*genlhdr).cmd as usize {
        x if x == THERMAL_GENL_EVENT_TZ_CREATE => {
            ((*ops).tz_create.unwrap())(
                nla_get_string(attrs[THERMAL_GENL_ATTR_TZ_NAME]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]),
                arg,
            )
        }

        x if x == THERMAL_GENL_EVENT_TZ_DELETE => {
            ((*ops).tz_delete.unwrap())(nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]), arg)
        }

        x if x == THERMAL_GENL_EVENT_TZ_ENABLE => {
            ((*ops).tz_enable.unwrap())(nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]), arg)
        }

        x if x == THERMAL_GENL_EVENT_TZ_DISABLE => {
            ((*ops).tz_disable.unwrap())(nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]), arg)
        }

        x if x == THERMAL_GENL_EVENT_TZ_TRIP_CHANGE => {
            ((*ops).trip_change.unwrap())(
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TRIP_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TRIP_TYPE]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TRIP_TEMP]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TRIP_HYST]),
                arg,
            )
        }

        x if x == THERMAL_GENL_EVENT_TZ_TRIP_ADD => {
            ((*ops).trip_add.unwrap())(
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TRIP_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TRIP_TYPE]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TRIP_TEMP]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TRIP_HYST]),
                arg,
            )
        }

        x if x == THERMAL_GENL_EVENT_TZ_TRIP_DELETE => {
            ((*ops).trip_delete.unwrap())(
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TRIP_ID]),
                arg,
            )
        }

        x if x == THERMAL_GENL_EVENT_TZ_TRIP_UP => {
            ((*ops).trip_high.unwrap())(
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TRIP_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TEMP]),
                arg,
            )
        }

        x if x == THERMAL_GENL_EVENT_TZ_TRIP_DOWN => {
            ((*ops).trip_low.unwrap())(
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TRIP_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TEMP]),
                arg,
            )
        }

        x if x == THERMAL_GENL_EVENT_CDEV_ADD => {
            ((*ops).cdev_add.unwrap())(
                nla_get_string(attrs[THERMAL_GENL_ATTR_CDEV_NAME]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_CDEV_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_CDEV_MAX_STATE]),
                arg,
            )
        }

        x if x == THERMAL_GENL_EVENT_CDEV_DELETE => {
            ((*ops).cdev_delete.unwrap())(nla_get_u32(attrs[THERMAL_GENL_ATTR_CDEV_ID]), arg)
        }

        x if x == THERMAL_GENL_EVENT_CDEV_STATE_UPDATE => {
            ((*ops).cdev_update.unwrap())(
                nla_get_u32(attrs[THERMAL_GENL_ATTR_CDEV_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_CDEV_CUR_STATE]),
                arg,
            )
        }

        x if x == THERMAL_GENL_EVENT_TZ_GOV_CHANGE => {
            ((*ops).gov_change.unwrap())(
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]),
                nla_get_string(attrs[THERMAL_GENL_ATTR_GOV_NAME]),
                arg,
            )
        }

        x if x == THERMAL_GENL_EVENT_THRESHOLD_ADD => {
            ((*ops).threshold_add.unwrap())(
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_THRESHOLD_TEMP]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_THRESHOLD_DIRECTION]),
                arg,
            )
        }

        x if x == THERMAL_GENL_EVENT_THRESHOLD_DELETE => {
            ((*ops).threshold_delete.unwrap())(
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_THRESHOLD_TEMP]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_THRESHOLD_DIRECTION]),
                arg,
            )
        }

        x if x == THERMAL_GENL_EVENT_THRESHOLD_FLUSH => {
            ((*ops).threshold_flush.unwrap())(nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]), arg)
        }

        x if x == THERMAL_GENL_EVENT_THRESHOLD_UP => {
            ((*ops).threshold_up.unwrap())(
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TEMP]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_PREV_TEMP]),
                arg,
            )
        }

        x if x == THERMAL_GENL_EVENT_THRESHOLD_DOWN => {
            ((*ops).threshold_down.unwrap())(
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TEMP]),
                nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_PREV_TEMP]),
                arg,
            )
        }

        _ => -1,
    }
}

unsafe fn thermal_events_ops_init(ops: *mut thermal_events_ops) {
    enabled_ops[THERMAL_GENL_EVENT_TZ_CREATE] = ((*ops).tz_create.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_TZ_DELETE] = ((*ops).tz_delete.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_TZ_DISABLE] = ((*ops).tz_disable.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_TZ_ENABLE] = ((*ops).tz_enable.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_TZ_TRIP_UP] = ((*ops).trip_high.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_TZ_TRIP_DOWN] = ((*ops).trip_low.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_TZ_TRIP_CHANGE] = ((*ops).trip_change.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_TZ_TRIP_ADD] = ((*ops).trip_add.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_TZ_TRIP_DELETE] = ((*ops).trip_delete.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_CDEV_ADD] = ((*ops).cdev_add.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_CDEV_DELETE] = ((*ops).cdev_delete.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_CDEV_STATE_UPDATE] = ((*ops).cdev_update.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_TZ_GOV_CHANGE] = ((*ops).gov_change.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_THRESHOLD_ADD] = ((*ops).threshold_add.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_THRESHOLD_DELETE] =
        ((*ops).threshold_delete.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_THRESHOLD_FLUSH] = ((*ops).threshold_flush.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_THRESHOLD_UP] = ((*ops).threshold_up.is_some()) as c_int;
    enabled_ops[THERMAL_GENL_EVENT_THRESHOLD_DOWN] = ((*ops).threshold_down.is_some()) as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn thermal_events_handle(
    th: *mut thermal_handler,
    arg: *mut c_void,
) -> thermal_error_t {
    let mut thp: thermal_handler_param = thermal_handler_param { th, arg };

    if th.is_null() {
        return THERMAL_ERROR;
    }

    if nl_cb_set(
        (*th).cb_event,
        NL_CB_VALID,
        NL_CB_CUSTOM,
        Some(handle_thermal_event),
        &mut thp as *mut thermal_handler_param as *mut c_void,
    ) != 0
    {
        return THERMAL_ERROR;
    }

    nl_recvmsgs((*th).sk_event, (*th).cb_event)
}

#[no_mangle]
pub unsafe extern "C" fn thermal_events_fd(th: *mut thermal_handler) -> c_int {
    if th.is_null() {
        return -1;
    }

    nl_socket_get_fd((*th).sk_event)
}

#[no_mangle]
pub unsafe extern "C" fn thermal_events_exit(th: *mut thermal_handler) -> thermal_error_t {
    if nl_unsubscribe_thermal((*th).sk_event, (*th).cb_event, THERMAL_GENL_EVENT_GROUP_NAME) != 0 {
        return THERMAL_ERROR;
    }

    nl_thermal_disconnect((*th).sk_event, (*th).cb_event);

    THERMAL_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn thermal_events_init(th: *mut thermal_handler) -> thermal_error_t {
    thermal_events_ops_init(&mut (*(*th).ops).events);

    if nl_thermal_connect(&mut (*th).sk_event, &mut (*th).cb_event) != 0 {
        return THERMAL_ERROR;
    }

    if nl_subscribe_thermal((*th).sk_event, (*th).cb_event, THERMAL_GENL_EVENT_GROUP_NAME) != 0 {
        return THERMAL_ERROR;
    }

    THERMAL_SUCCESS
}
