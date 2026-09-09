/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  IPv6 IOAM Generic Netlink API
 *
 *  Author:
 *  Justin Iurman <justin.iurman@uliege.be>
 */

// C header guard: _UAPI_LINUX_IOAM6_GENL_H

pub const IOAM6_GENL_NAME: &str = "IOAM6";
pub const IOAM6_GENL_VERSION: i32 = 0x1;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ioam6Attr {
    IOAM6_ATTR_UNSPEC = 0,

    IOAM6_ATTR_NS_ID,
    IOAM6_ATTR_NS_DATA,
    IOAM6_ATTR_NS_DATA_WIDE,

    IOAM6_ATTR_SC_ID,
    IOAM6_ATTR_SC_DATA,
    IOAM6_ATTR_SC_NONE,

    IOAM6_ATTR_PAD,

    __IOAM6_ATTR_MAX,
}

pub const IOAM6_MAX_SCHEMA_DATA_LEN: usize = 255 * 4;
pub const IOAM6_ATTR_MAX: i32 = (__IOAM6_ATTR_MAX as i32) - 1;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ioam6Cmd {
    IOAM6_CMD_UNSPEC = 0,

    IOAM6_CMD_ADD_NAMESPACE,
    IOAM6_CMD_DEL_NAMESPACE,
    IOAM6_CMD_DUMP_NAMESPACES,

    IOAM6_CMD_ADD_SCHEMA,
    IOAM6_CMD_DEL_SCHEMA,
    IOAM6_CMD_DUMP_SCHEMAS,

    IOAM6_CMD_NS_SET_SCHEMA,

    __IOAM6_CMD_MAX,
}

pub const IOAM6_CMD_MAX: i32 = (__IOAM6_CMD_MAX as i32) - 1;

pub const IOAM6_GENL_EV_GRP_NAME: &str = "ioam6_events";

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ioam6EventType {
    IOAM6_EVENT_UNSPEC = 0,
    IOAM6_EVENT_TRACE,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ioam6EventAttr {
    IOAM6_EVENT_ATTR_UNSPEC = 0,

    IOAM6_EVENT_ATTR_TRACE_NAMESPACE,
    IOAM6_EVENT_ATTR_TRACE_NODELEN,
    IOAM6_EVENT_ATTR_TRACE_TYPE,
    IOAM6_EVENT_ATTR_TRACE_DATA,

    __IOAM6_EVENT_ATTR_MAX,
}

pub const IOAM6_EVENT_ATTR_MAX: i32 = (__IOAM6_EVENT_ATTR_MAX as i32) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
