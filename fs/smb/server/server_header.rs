/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

/* C header dependency: #include "smbacl.h" */

use core::ffi::{c_char, c_int, c_short, c_ulong, c_uint, c_ushort};

/*
 * Server state type
 */
pub const SERVER_STATE_STARTING_UP: c_uint = 0;
pub const SERVER_STATE_RUNNING: c_uint = 1;
pub const SERVER_STATE_RESETTING: c_uint = 2;
pub const SERVER_STATE_SHUTTING_DOWN: c_uint = 3;

/*
 * Server global config string index
 */
pub const SERVER_CONF_NETBIOS_NAME: usize = 0;
pub const SERVER_CONF_SERVER_STRING: usize = 1;
pub const SERVER_CONF_WORK_GROUP: usize = 2;

/* Types supplied by other translated dependencies. */
#[repr(C)]
pub struct smb_sid {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksmbd_server_config {
    pub flags: c_uint,
    pub state: c_uint,
    pub signing: c_short,
    pub enforced_signing: c_short,
    pub min_protocol: c_short,
    pub max_protocol: c_short,
    pub tcp_port: c_ushort,
    pub ipc_timeout: c_ushort,
    pub ipc_last_active: c_ulong,
    pub deadtime: c_ulong,
    pub share_fake_fscaps: c_uint,
    pub domain_sid: smb_sid,
    pub auth_mechs: c_uint,
    pub max_connections: c_uint,
    pub max_inflight_req: c_uint,
    pub max_ip_connections: c_uint,
    pub conf: [*mut c_char; SERVER_CONF_WORK_GROUP + 1],
    pub dh_task: *mut task_struct,
    pub bind_interfaces_only: bool,
    /* AAPL model string for Finder icon, e.g. "Xserve" */
    pub aapl_model: [c_char; 32],
}

extern "C" {
    pub static mut server_conf: ksmbd_server_config;

    pub fn ksmbd_set_netbios_name(v: *mut c_char) -> c_int;
    pub fn ksmbd_set_server_string(v: *mut c_char) -> c_int;
    pub fn ksmbd_set_work_group(v: *mut c_char) -> c_int;

    pub fn ksmbd_netbios_name() -> *mut c_char;
    pub fn ksmbd_server_string() -> *mut c_char;
    pub fn ksmbd_work_group() -> *mut c_char;

    pub fn server_queue_ctrl_init_work() -> c_int;
    pub fn server_queue_ctrl_reset_work() -> c_int;
}

#[inline]
pub unsafe fn ksmbd_server_running() -> c_int {
    (core::ptr::read_volatile(core::ptr::addr_of!(server_conf.state)) == SERVER_STATE_RUNNING)
        as c_int
}

#[inline]
pub unsafe fn ksmbd_server_configurable() -> c_int {
    (core::ptr::read_volatile(core::ptr::addr_of!(server_conf.state)) < SERVER_STATE_RESETTING)
        as c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
