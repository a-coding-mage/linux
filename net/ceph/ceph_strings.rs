// SPDX-License-Identifier: GPL-2.0
/*
 * Ceph string constants
 */

use std::os::raw::c_char;

// Dependency symbols supplied by the surrounding Ceph/Linux translation.

pub unsafe extern "C" fn ceph_entity_type_name(type_: i32) -> *const c_char {
    match type_ {
        CEPH_ENTITY_TYPE_MDS => b"mds\0".as_ptr() as *const c_char,
        CEPH_ENTITY_TYPE_OSD => b"osd\0".as_ptr() as *const c_char,
        CEPH_ENTITY_TYPE_MON => b"mon\0".as_ptr() as *const c_char,
        CEPH_ENTITY_TYPE_CLIENT => b"client\0".as_ptr() as *const c_char,
        CEPH_ENTITY_TYPE_AUTH => b"auth\0".as_ptr() as *const c_char,
        _ => b"unknown\0".as_ptr() as *const c_char,
    }
}

// EXPORT_SYMBOL(ceph_entity_type_name);

pub unsafe extern "C" fn ceph_auth_proto_name(proto: i32) -> *const c_char {
    match proto {
        CEPH_AUTH_UNKNOWN => b"unknown\0".as_ptr() as *const c_char,
        CEPH_AUTH_NONE => b"none\0".as_ptr() as *const c_char,
        CEPH_AUTH_CEPHX => b"cephx\0".as_ptr() as *const c_char,
        _ => b"???\0".as_ptr() as *const c_char,
    }
}

pub unsafe extern "C" fn ceph_con_mode_name(mode: i32) -> *const c_char {
    match mode {
        CEPH_CON_MODE_UNKNOWN => b"unknown\0".as_ptr() as *const c_char,
        CEPH_CON_MODE_CRC => b"crc\0".as_ptr() as *const c_char,
        CEPH_CON_MODE_SECURE => b"secure\0".as_ptr() as *const c_char,
        _ => b"???\0".as_ptr() as *const c_char,
    }
}

pub unsafe extern "C" fn ceph_osd_op_name(op: i32) -> *const c_char {
    // The C source expands __CEPH_FORALL_OSD_OPS(GENERATE_CASE) here.  Its
    // externally supplied opcode list must provide the corresponding cases.
    match op {
        _ => b"???\0".as_ptr() as *const c_char,
    }
}

pub unsafe extern "C" fn ceph_osd_watch_op_name(o: i32) -> *const c_char {
    match o {
        CEPH_OSD_WATCH_OP_UNWATCH => b"unwatch\0".as_ptr() as *const c_char,
        CEPH_OSD_WATCH_OP_WATCH => b"watch\0".as_ptr() as *const c_char,
        CEPH_OSD_WATCH_OP_RECONNECT => b"reconnect\0".as_ptr() as *const c_char,
        CEPH_OSD_WATCH_OP_PING => b"ping\0".as_ptr() as *const c_char,
        _ => b"???\0".as_ptr() as *const c_char,
    }
}

pub unsafe extern "C" fn ceph_osd_state_name(s: i32) -> *const c_char {
    match s {
        CEPH_OSD_EXISTS => b"exists\0".as_ptr() as *const c_char,
        CEPH_OSD_UP => b"up\0".as_ptr() as *const c_char,
        CEPH_OSD_AUTOOUT => b"autoout\0".as_ptr() as *const c_char,
        CEPH_OSD_NEW => b"new\0".as_ptr() as *const c_char,
        _ => b"???\0".as_ptr() as *const c_char,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
