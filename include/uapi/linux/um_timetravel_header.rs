/* SPDX-License-Identifier: BSD-3-Clause */
/* Copyright (C) 2019 - 2023 Intel Corporation */

/* Translated from uapi/linux/um_timetravel.h. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct um_timetravel_msg {
    pub op: u32,
    pub seq: u32,
    pub time: u64,
}

/* Max number of file descriptors that can be sent/received in a message. */
pub const UM_TIMETRAVEL_MAX_FDS: u32 = 2;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum um_timetravel_shared_mem_fds {
    UM_TIMETRAVEL_SHARED_MEMFD = 0,
    UM_TIMETRAVEL_SHARED_LOGFD = 1,
    UM_TIMETRAVEL_SHARED_MAX_FDS = 2,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum um_timetravel_start_ack {
    UM_TIMETRAVEL_START_ACK_ID = 0xffff,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum um_timetravel_ops {
    UM_TIMETRAVEL_ACK = 0,
    UM_TIMETRAVEL_START = 1,
    UM_TIMETRAVEL_REQUEST = 2,
    UM_TIMETRAVEL_WAIT = 3,
    UM_TIMETRAVEL_GET = 4,
    UM_TIMETRAVEL_UPDATE = 5,
    UM_TIMETRAVEL_RUN = 6,
    UM_TIMETRAVEL_FREE_UNTIL = 7,
    UM_TIMETRAVEL_GET_TOD = 8,
    UM_TIMETRAVEL_BROADCAST = 9,
}

/* Version of struct um_timetravel_schedshm. */
pub const UM_TIMETRAVEL_SCHEDSHM_VERSION: u32 = 2;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum um_timetravel_schedshm_cap {
    UM_TIMETRAVEL_SCHEDSHM_CAP_TIME_SHARE = 0x1,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum um_timetravel_schedshm_flags {
    UM_TIMETRAVEL_SCHEDSHM_FLAGS_REQ_RUN = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct um_timetravel_schedshm_client_fields {
    pub capa: u32,
    pub flags: u32,
    pub req_time: u64,
    pub name: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union um_timetravel_schedshm_client {
    pub fields: um_timetravel_schedshm_client_fields,
    pub reserve: [core::ffi::c_char; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct um_timetravel_schedshm_header_fields {
    pub version: u32,
    pub len: u32,
    pub free_until: u64,
    pub current_time: u64,
    pub running_id: u16,
    pub max_clients: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union um_timetravel_schedshm_header {
    pub fields: um_timetravel_schedshm_header_fields,
    pub hdr: [core::ffi::c_char; 4096],
}

#[repr(C)]
pub struct um_timetravel_schedshm {
    pub hdr: um_timetravel_schedshm_header,
    pub clients: [um_timetravel_schedshm_client; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
