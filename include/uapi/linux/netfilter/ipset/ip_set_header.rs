/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright (C) 2000-2002 Joakim Axelsson <gozem@linux.nu>
 *                         Patrick Schaaf <bof@bof.de>
 *                         Martin Josefsson <gandalf@wlug.westbo.se>
 * Copyright (C) 2003-2011 Jozsef Kadlecsik <kadlec@netfilter.org>
 */

/* Dependency: linux/types.h supplies the kernel integer types. */

/* The protocol versions */
pub const IPSET_PROTOCOL: u32 = 7;
pub const IPSET_PROTOCOL_MIN: u32 = 6;
pub const IPSET_MAXNAMELEN: usize = 32;
pub const IPSET_MAX_COMMENT_SIZE: usize = 255;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipset_cmd {
    IPSET_CMD_NONE,
    IPSET_CMD_PROTOCOL,
    IPSET_CMD_CREATE,
    IPSET_CMD_DESTROY,
    IPSET_CMD_FLUSH,
    IPSET_CMD_RENAME,
    IPSET_CMD_SWAP,
    IPSET_CMD_LIST,
    IPSET_CMD_SAVE,
    IPSET_CMD_ADD,
    IPSET_CMD_DEL,
    IPSET_CMD_TEST,
    IPSET_CMD_HEADER,
    IPSET_CMD_TYPE,
    IPSET_CMD_GET_BYNAME,
    IPSET_CMD_GET_BYINDEX,
    IPSET_MSG_MAX,
    IPSET_CMD_RESTORE = IPSET_MSG_MAX as isize,
    IPSET_CMD_HELP,
    IPSET_CMD_VERSION,
    IPSET_CMD_QUIT,
    IPSET_CMD_MAX,
    IPSET_CMD_COMMIT = IPSET_CMD_MAX as isize,
}

pub const IPSET_ATTR_UNSPEC: u32 = 0;
pub const IPSET_ATTR_PROTOCOL: u32 = 1;
pub const IPSET_ATTR_SETNAME: u32 = 2;
pub const IPSET_ATTR_TYPENAME: u32 = 3;
pub const IPSET_ATTR_SETNAME2: u32 = IPSET_ATTR_TYPENAME;
pub const IPSET_ATTR_REVISION: u32 = 4;
pub const IPSET_ATTR_FAMILY: u32 = 5;
pub const IPSET_ATTR_FLAGS: u32 = 6;
pub const IPSET_ATTR_DATA: u32 = 7;
pub const IPSET_ATTR_ADT: u32 = 8;
pub const IPSET_ATTR_LINENO: u32 = 9;
pub const IPSET_ATTR_PROTOCOL_MIN: u32 = 10;
pub const IPSET_ATTR_REVISION_MIN: u32 = IPSET_ATTR_PROTOCOL_MIN;
pub const IPSET_ATTR_INDEX: u32 = 11;
pub const __IPSET_ATTR_CMD_MAX: u32 = 12;
pub const IPSET_ATTR_CMD_MAX: u32 = __IPSET_ATTR_CMD_MAX - 1;

pub const IPSET_ATTR_IP: u32 = IPSET_ATTR_UNSPEC + 1;
pub const IPSET_ATTR_IP_FROM: u32 = IPSET_ATTR_IP;
pub const IPSET_ATTR_IP_TO: u32 = 2;
pub const IPSET_ATTR_CIDR: u32 = 3;
pub const IPSET_ATTR_PORT: u32 = 4;
pub const IPSET_ATTR_PORT_FROM: u32 = IPSET_ATTR_PORT;
pub const IPSET_ATTR_PORT_TO: u32 = 5;
pub const IPSET_ATTR_TIMEOUT: u32 = 6;
pub const IPSET_ATTR_PROTO: u32 = 7;
pub const IPSET_ATTR_CADT_FLAGS: u32 = 8;
pub const IPSET_ATTR_CADT_LINENO: u32 = IPSET_ATTR_LINENO;
pub const IPSET_ATTR_MARK: u32 = 10;
pub const IPSET_ATTR_MARKMASK: u32 = 11;
pub const IPSET_ATTR_BITMASK: u32 = 12;
pub const IPSET_ATTR_CADT_MAX: u32 = 16;
pub const IPSET_ATTR_INITVAL: u32 = 17;
pub const IPSET_ATTR_HASHSIZE: u32 = 18;
pub const IPSET_ATTR_MAXELEM: u32 = 19;
pub const IPSET_ATTR_NETMASK: u32 = 20;
pub const IPSET_ATTR_BUCKETSIZE: u32 = 21;
pub const IPSET_ATTR_RESIZE: u32 = 22;
pub const IPSET_ATTR_SIZE: u32 = 23;
pub const IPSET_ATTR_ELEMENTS: u32 = 24;
pub const IPSET_ATTR_REFERENCES: u32 = 25;
pub const IPSET_ATTR_MEMSIZE: u32 = 26;
pub const __IPSET_ATTR_CREATE_MAX: u32 = 27;
pub const IPSET_ATTR_CREATE_MAX: u32 = __IPSET_ATTR_CREATE_MAX - 1;

pub const IPSET_ATTR_ETHER: u32 = IPSET_ATTR_CADT_MAX + 1;
pub const IPSET_ATTR_NAME: u32 = 18;
pub const IPSET_ATTR_NAMEREF: u32 = 19;
pub const IPSET_ATTR_IP2: u32 = 20;
pub const IPSET_ATTR_CIDR2: u32 = 21;
pub const IPSET_ATTR_IP2_TO: u32 = 22;
pub const IPSET_ATTR_IFACE: u32 = 23;
pub const IPSET_ATTR_BYTES: u32 = 24;
pub const IPSET_ATTR_PACKETS: u32 = 25;
pub const IPSET_ATTR_COMMENT: u32 = 26;
pub const IPSET_ATTR_SKBMARK: u32 = 27;
pub const IPSET_ATTR_SKBPRIO: u32 = 28;
pub const IPSET_ATTR_SKBQUEUE: u32 = 29;
pub const IPSET_ATTR_PAD: u32 = 30;
pub const __IPSET_ATTR_ADT_MAX: u32 = 31;
pub const IPSET_ATTR_ADT_MAX: u32 = __IPSET_ATTR_ADT_MAX - 1;

pub const IPSET_ATTR_IPADDR_IPV4: u32 = 1;
pub const IPSET_ATTR_IPADDR_IPV6: u32 = 2;
pub const __IPSET_ATTR_IPADDR_MAX: u32 = 3;
pub const IPSET_ATTR_IPADDR_MAX: u32 = __IPSET_ATTR_IPADDR_MAX - 1;

#[repr(C)]
pub enum ipset_errno {
    IPSET_ERR_PRIVATE = 4096,
    IPSET_ERR_PROTOCOL,
    IPSET_ERR_FIND_TYPE,
    IPSET_ERR_MAX_SETS,
    IPSET_ERR_BUSY,
    IPSET_ERR_EXIST_SETNAME2,
    IPSET_ERR_TYPE_MISMATCH,
    IPSET_ERR_EXIST,
    IPSET_ERR_INVALID_CIDR,
    IPSET_ERR_INVALID_NETMASK,
    IPSET_ERR_INVALID_FAMILY,
    IPSET_ERR_TIMEOUT,
    IPSET_ERR_REFERENCED,
    IPSET_ERR_IPADDR_IPV4,
    IPSET_ERR_IPADDR_IPV6,
    IPSET_ERR_COUNTER,
    IPSET_ERR_COMMENT,
    IPSET_ERR_INVALID_MARKMASK,
    IPSET_ERR_SKBINFO,
    IPSET_ERR_BITMASK_NETMASK_EXCL,
    IPSET_ERR_TYPE_SPECIFIC = 4352,
}

pub const IPSET_FLAG_BIT_EXIST: u32 = 0;
pub const IPSET_FLAG_EXIST: u32 = 1 << IPSET_FLAG_BIT_EXIST;
pub const IPSET_FLAG_BIT_LIST_SETNAME: u32 = 1;
pub const IPSET_FLAG_LIST_SETNAME: u32 = 1 << IPSET_FLAG_BIT_LIST_SETNAME;
pub const IPSET_FLAG_BIT_LIST_HEADER: u32 = 2;
pub const IPSET_FLAG_LIST_HEADER: u32 = 1 << IPSET_FLAG_BIT_LIST_HEADER;
pub const IPSET_FLAG_BIT_SKIP_COUNTER_UPDATE: u32 = 3;
pub const IPSET_FLAG_SKIP_COUNTER_UPDATE: u32 = 1 << IPSET_FLAG_BIT_SKIP_COUNTER_UPDATE;
pub const IPSET_FLAG_BIT_SKIP_SUBCOUNTER_UPDATE: u32 = 4;
pub const IPSET_FLAG_SKIP_SUBCOUNTER_UPDATE: u32 = 1 << IPSET_FLAG_BIT_SKIP_SUBCOUNTER_UPDATE;
pub const IPSET_FLAG_BIT_MATCH_COUNTERS: u32 = 5;
pub const IPSET_FLAG_MATCH_COUNTERS: u32 = 1 << IPSET_FLAG_BIT_MATCH_COUNTERS;
pub const IPSET_FLAG_BIT_RETURN_NOMATCH: u32 = 7;
pub const IPSET_FLAG_RETURN_NOMATCH: u32 = 1 << IPSET_FLAG_BIT_RETURN_NOMATCH;
pub const IPSET_FLAG_BIT_MAP_SKBMARK: u32 = 8;
pub const IPSET_FLAG_MAP_SKBMARK: u32 = 1 << IPSET_FLAG_BIT_MAP_SKBMARK;
pub const IPSET_FLAG_BIT_MAP_SKBPRIO: u32 = 9;
pub const IPSET_FLAG_MAP_SKBPRIO: u32 = 1 << IPSET_FLAG_BIT_MAP_SKBPRIO;
pub const IPSET_FLAG_BIT_MAP_SKBQUEUE: u32 = 10;
pub const IPSET_FLAG_MAP_SKBQUEUE: u32 = 1 << IPSET_FLAG_BIT_MAP_SKBQUEUE;
pub const IPSET_FLAG_CMD_MAX: u32 = 15;

pub const IPSET_FLAG_BIT_BEFORE: u32 = 0;
pub const IPSET_FLAG_BEFORE: u32 = 1;
pub const IPSET_FLAG_BIT_PHYSDEV: u32 = 1;
pub const IPSET_FLAG_PHYSDEV: u32 = 1 << 1;
pub const IPSET_FLAG_BIT_NOMATCH: u32 = 2;
pub const IPSET_FLAG_NOMATCH: u32 = 1 << 2;
pub const IPSET_FLAG_BIT_WITH_COUNTERS: u32 = 3;
pub const IPSET_FLAG_WITH_COUNTERS: u32 = 1 << 3;
pub const IPSET_FLAG_BIT_WITH_COMMENT: u32 = 4;
pub const IPSET_FLAG_WITH_COMMENT: u32 = 1 << 4;
pub const IPSET_FLAG_BIT_WITH_FORCEADD: u32 = 5;
pub const IPSET_FLAG_WITH_FORCEADD: u32 = 1 << 5;
pub const IPSET_FLAG_BIT_WITH_SKBINFO: u32 = 6;
pub const IPSET_FLAG_WITH_SKBINFO: u32 = 1 << 6;
pub const IPSET_FLAG_BIT_IFACE_WILDCARD: u32 = 7;
pub const IPSET_FLAG_IFACE_WILDCARD: u32 = 1 << 7;
pub const IPSET_FLAG_CADT_MAX: u32 = 15;

pub const IPSET_CREATE_FLAG_BIT_FORCEADD: u32 = 0;
pub const IPSET_CREATE_FLAG_FORCEADD: u32 = 1;
pub const IPSET_CREATE_FLAG_BIT_BUCKETSIZE: u32 = 1;
pub const IPSET_CREATE_FLAG_BUCKETSIZE: u32 = 1 << 1;
pub const IPSET_CREATE_FLAG_BIT_MAX: u32 = 7;

#[repr(C)]
pub enum ipset_adt { IPSET_ADD, IPSET_DEL, IPSET_TEST, IPSET_ADT_MAX, IPSET_CREATE = 3, IPSET_CADT_MAX }

pub type ip_set_id_t = u16;
pub const IPSET_INVALID_ID: ip_set_id_t = 65535;

pub const IPSET_DIM_ZERO: u32 = 0;
pub const IPSET_DIM_ONE: u32 = 1;
pub const IPSET_DIM_TWO: u32 = 2;
pub const IPSET_DIM_THREE: u32 = 3;
pub const IPSET_DIM_MAX: u32 = 6;
pub const IPSET_BIT_RETURN_NOMATCH: u32 = 7;
pub const IPSET_INV_MATCH: u32 = 1 << IPSET_DIM_ZERO;
pub const IPSET_DIM_ONE_SRC: u32 = 1 << IPSET_DIM_ONE;
pub const IPSET_DIM_TWO_SRC: u32 = 1 << IPSET_DIM_TWO;
pub const IPSET_DIM_THREE_SRC: u32 = 1 << IPSET_DIM_THREE;
pub const IPSET_RETURN_NOMATCH: u32 = 1 << IPSET_BIT_RETURN_NOMATCH;
pub const IPSET_COUNTER_NONE: u32 = 0;
pub const IPSET_COUNTER_EQ: u32 = 1;
pub const IPSET_COUNTER_NE: u32 = 2;
pub const IPSET_COUNTER_LT: u32 = 3;
pub const IPSET_COUNTER_GT: u32 = 4;

#[repr(C)]
pub struct ip_set_counter_match0 { pub op: u8, pub value: u64 }
#[repr(C, align(8))]
pub struct ip_set_counter_match { pub value: u64, pub op: u8 }

pub const SO_IP_SET: u32 = 83;

#[repr(C)]
pub union ip_set_name_index { pub name: [core::ffi::c_char; IPSET_MAXNAMELEN], pub index: ip_set_id_t }

pub const IP_SET_OP_GET_BYNAME: u32 = 0x00000006;
#[repr(C)]
pub struct ip_set_req_get_set { pub op: u32, pub version: u32, pub set: ip_set_name_index }
pub const IP_SET_OP_GET_BYINDEX: u32 = 0x00000007;
pub const IP_SET_OP_GET_FNAME: u32 = 0x00000008;
#[repr(C)]
pub struct ip_set_req_get_set_family { pub op: u32, pub version: u32, pub family: u32, pub set: ip_set_name_index }
pub const IP_SET_OP_VERSION: u32 = 0x00000100;
#[repr(C)]
pub struct ip_set_req_version { pub op: u32, pub version: u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
