// SPDX-License-Identifier: GPL-2.0
/*
 * Ceph fs string constants
 */
// linux/module.h and linux/ceph/types.h dependencies are supplied externally.

use core::ffi::c_char;

pub const fn ceph_mds_state_name(s: i32) -> *const c_char {
    match s {
        /* down and out */
        CEPH_MDS_STATE_DNE => b"down:dne\0".as_ptr() as *const c_char,
        CEPH_MDS_STATE_STOPPED => b"down:stopped\0".as_ptr() as *const c_char,
        /* up and out */
        CEPH_MDS_STATE_BOOT => b"up:boot\0".as_ptr() as *const c_char,
        CEPH_MDS_STATE_STANDBY => b"up:standby\0".as_ptr() as *const c_char,
        CEPH_MDS_STATE_STANDBY_REPLAY => b"up:standby-replay\0".as_ptr() as *const c_char,
        CEPH_MDS_STATE_REPLAYONCE => b"up:oneshot-replay\0".as_ptr() as *const c_char,
        CEPH_MDS_STATE_CREATING => b"up:creating\0".as_ptr() as *const c_char,
        CEPH_MDS_STATE_STARTING => b"up:starting\0".as_ptr() as *const c_char,
        /* up and in */
        CEPH_MDS_STATE_REPLAY => b"up:replay\0".as_ptr() as *const c_char,
        CEPH_MDS_STATE_RESOLVE => b"up:resolve\0".as_ptr() as *const c_char,
        CEPH_MDS_STATE_RECONNECT => b"up:reconnect\0".as_ptr() as *const c_char,
        CEPH_MDS_STATE_REJOIN => b"up:rejoin\0".as_ptr() as *const c_char,
        CEPH_MDS_STATE_CLIENTREPLAY => b"up:clientreplay\0".as_ptr() as *const c_char,
        CEPH_MDS_STATE_ACTIVE => b"up:active\0".as_ptr() as *const c_char,
        CEPH_MDS_STATE_STOPPING => b"up:stopping\0".as_ptr() as *const c_char,
        _ => b"???\0".as_ptr() as *const c_char,
    }
}

pub const fn ceph_session_op_name(op: i32) -> *const c_char {
    match op {
        CEPH_SESSION_REQUEST_OPEN => b"request_open\0".as_ptr() as *const c_char,
        CEPH_SESSION_OPEN => b"open\0".as_ptr() as *const c_char,
        CEPH_SESSION_REQUEST_CLOSE => b"request_close\0".as_ptr() as *const c_char,
        CEPH_SESSION_CLOSE => b"close\0".as_ptr() as *const c_char,
        CEPH_SESSION_REQUEST_RENEWCAPS => b"request_renewcaps\0".as_ptr() as *const c_char,
        CEPH_SESSION_RENEWCAPS => b"renewcaps\0".as_ptr() as *const c_char,
        CEPH_SESSION_STALE => b"stale\0".as_ptr() as *const c_char,
        CEPH_SESSION_RECALL_STATE => b"recall_state\0".as_ptr() as *const c_char,
        CEPH_SESSION_FLUSHMSG => b"flushmsg\0".as_ptr() as *const c_char,
        CEPH_SESSION_FLUSHMSG_ACK => b"flushmsg_ack\0".as_ptr() as *const c_char,
        CEPH_SESSION_FORCE_RO => b"force_ro\0".as_ptr() as *const c_char,
        CEPH_SESSION_REJECT => b"reject\0".as_ptr() as *const c_char,
        CEPH_SESSION_REQUEST_FLUSH_MDLOG => b"flush_mdlog\0".as_ptr() as *const c_char,
        _ => b"???\0".as_ptr() as *const c_char,
    }
}

pub const fn ceph_mds_op_name(op: i32) -> *const c_char {
    match op {
        CEPH_MDS_OP_LOOKUP => b"lookup\0", CEPH_MDS_OP_LOOKUPHASH => b"lookuphash\0",
        CEPH_MDS_OP_LOOKUPPARENT => b"lookupparent\0", CEPH_MDS_OP_LOOKUPINO => b"lookupino\0",
        CEPH_MDS_OP_LOOKUPNAME => b"lookupname\0", CEPH_MDS_OP_GETATTR => b"getattr\0",
        CEPH_MDS_OP_GETVXATTR => b"getvxattr\0", CEPH_MDS_OP_SETXATTR => b"setxattr\0",
        CEPH_MDS_OP_SETATTR => b"setattr\0", CEPH_MDS_OP_RMXATTR => b"rmxattr\0",
        CEPH_MDS_OP_SETLAYOUT => b"setlayou\0", CEPH_MDS_OP_SETDIRLAYOUT => b"setdirlayout\0",
        CEPH_MDS_OP_READDIR => b"readdir\0", CEPH_MDS_OP_MKNOD => b"mknod\0",
        CEPH_MDS_OP_LINK => b"link\0", CEPH_MDS_OP_UNLINK => b"unlink\0",
        CEPH_MDS_OP_RENAME => b"rename\0", CEPH_MDS_OP_MKDIR => b"mkdir\0",
        CEPH_MDS_OP_RMDIR => b"rmdir\0", CEPH_MDS_OP_SYMLINK => b"symlink\0",
        CEPH_MDS_OP_CREATE => b"create\0", CEPH_MDS_OP_OPEN => b"open\0",
        CEPH_MDS_OP_LOOKUPSNAP => b"lookupsnap\0", CEPH_MDS_OP_LSSNAP => b"lssnap\0",
        CEPH_MDS_OP_MKSNAP => b"mksnap\0", CEPH_MDS_OP_RMSNAP => b"rmsnap\0",
        CEPH_MDS_OP_RENAMESNAP => b"renamesnap\0", CEPH_MDS_OP_SETFILELOCK => b"setfilelock\0",
        CEPH_MDS_OP_GETFILELOCK => b"getfilelock\0", _ => b"???\0",
    }.as_ptr() as *const c_char
}

pub const fn ceph_cap_op_name(op: i32) -> *const c_char {
    match op {
        CEPH_CAP_OP_GRANT => b"grant\0", CEPH_CAP_OP_REVOKE => b"revoke\0",
        CEPH_CAP_OP_TRUNC => b"trunc\0", CEPH_CAP_OP_EXPORT => b"export\0",
        CEPH_CAP_OP_IMPORT => b"import\0", CEPH_CAP_OP_UPDATE => b"update\0",
        CEPH_CAP_OP_DROP => b"drop\0", CEPH_CAP_OP_FLUSH => b"flush\0",
        CEPH_CAP_OP_FLUSH_ACK => b"flush_ack\0", CEPH_CAP_OP_FLUSHSNAP => b"flushsnap\0",
        CEPH_CAP_OP_FLUSHSNAP_ACK => b"flushsnap_ack\0", CEPH_CAP_OP_RELEASE => b"release\0",
        CEPH_CAP_OP_RENEW => b"renew\0", _ => b"???\0",
    }.as_ptr() as *const c_char
}

pub const fn ceph_lease_op_name(o: i32) -> *const c_char {
    match o {
        CEPH_MDS_LEASE_REVOKE => b"revoke\0", CEPH_MDS_LEASE_RELEASE => b"release\0",
        CEPH_MDS_LEASE_RENEW => b"renew\0", CEPH_MDS_LEASE_REVOKE_ACK => b"revoke_ack\0",
        _ => b"???\0",
    }.as_ptr() as *const c_char
}

pub const fn ceph_snap_op_name(o: i32) -> *const c_char {
    match o {
        CEPH_SNAP_OP_UPDATE => b"update\0", CEPH_SNAP_OP_CREATE => b"create\0",
        CEPH_SNAP_OP_DESTROY => b"destroy\0", CEPH_SNAP_OP_SPLIT => b"split\0",
        _ => b"???\0",
    }.as_ptr() as *const c_char
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
