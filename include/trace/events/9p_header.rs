/* SPDX-License-Identifier: GPL-2.0 */

// `TRACE_SYSTEM` is 9p in the C header.  The Linux tracepoint include and
// trace generation directives have no direct Rust equivalent.

/// 9P message names used by the tracepoint symbolic printer.
pub const P9_MSG_T: &[(i32, &str)] = &[
    (P9_TLERROR, "P9_TLERROR"),
    (P9_RLERROR, "P9_RLERROR"),
    (P9_TSTATFS, "P9_TSTATFS"),
    (P9_RSTATFS, "P9_RSTATFS"),
    (P9_TLOPEN, "P9_TLOPEN"),
    (P9_RLOPEN, "P9_RLOPEN"),
    (P9_TLCREATE, "P9_TLCREATE"),
    (P9_RLCREATE, "P9_RLCREATE"),
    (P9_TSYMLINK, "P9_TSYMLINK"),
    (P9_RSYMLINK, "P9_RSYMLINK"),
    (P9_TMKNOD, "P9_TMKNOD"),
    (P9_RMKNOD, "P9_RMKNOD"),
    (P9_TRENAME, "P9_TRENAME"),
    (P9_RRENAME, "P9_RRENAME"),
    (P9_TREADLINK, "P9_TREADLINK"),
    (P9_RREADLINK, "P9_RREADLINK"),
    (P9_TGETATTR, "P9_TGETATTR"),
    (P9_RGETATTR, "P9_RGETATTR"),
    (P9_TSETATTR, "P9_TSETATTR"),
    (P9_RSETATTR, "P9_RSETATTR"),
    (P9_TXATTRWALK, "P9_TXATTRWALK"),
    (P9_RXATTRWALK, "P9_RXATTRWALK"),
    (P9_TXATTRCREATE, "P9_TXATTRCREATE"),
    (P9_RXATTRCREATE, "P9_RXATTRCREATE"),
    (P9_TREADDIR, "P9_TREADDIR"),
    (P9_RREADDIR, "P9_RREADDIR"),
    (P9_TFSYNC, "P9_TFSYNC"),
    (P9_RFSYNC, "P9_RFSYNC"),
    (P9_TLOCK, "P9_TLOCK"),
    (P9_RLOCK, "P9_RLOCK"),
    (P9_TGETLOCK, "P9_TGETLOCK"),
    (P9_RGETLOCK, "P9_RGETLOCK"),
    (P9_TLINK, "P9_TLINK"),
    (P9_RLINK, "P9_RLINK"),
    (P9_TMKDIR, "P9_TMKDIR"),
    (P9_RMKDIR, "P9_RMKDIR"),
    (P9_TRENAMEAT, "P9_TRENAMEAT"),
    (P9_RRENAMEAT, "P9_RRENAMEAT"),
    (P9_TUNLINKAT, "P9_TUNLINKAT"),
    (P9_RUNLINKAT, "P9_RUNLINKAT"),
    (P9_TVERSION, "P9_TVERSION"),
    (P9_RVERSION, "P9_RVERSION"),
    (P9_TAUTH, "P9_TAUTH"),
    (P9_RAUTH, "P9_RAUTH"),
    (P9_TATTACH, "P9_TATTACH"),
    (P9_RATTACH, "P9_RATTACH"),
    (P9_TERROR, "P9_TERROR"),
    (P9_RERROR, "P9_RERROR"),
    (P9_TFLUSH, "P9_TFLUSH"),
    (P9_RFLUSH, "P9_RFLUSH"),
    (P9_TWALK, "P9_TWALK"),
    (P9_RWALK, "P9_RWALK"),
    (P9_TOPEN, "P9_TOPEN"),
    (P9_ROPEN, "P9_ROPEN"),
    (P9_TCREATE, "P9_TCREATE"),
    (P9_RCREATE, "P9_RCREATE"),
    (P9_TREAD, "P9_TREAD"),
    (P9_RREAD, "P9_RREAD"),
    (P9_TWRITE, "P9_TWRITE"),
    (P9_RWRITE, "P9_RWRITE"),
    (P9_TCLUNK, "P9_TCLUNK"),
    (P9_RCLUNK, "P9_RCLUNK"),
    (P9_TREMOVE, "P9_TREMOVE"),
    (P9_RREMOVE, "P9_RREMOVE"),
    (P9_TSTAT, "P9_TSTAT"),
    (P9_RSTAT, "P9_RSTAT"),
    (P9_TWSTAT, "P9_TWSTAT"),
    (P9_RWSTAT, "P9_RWSTAT"),
];

pub const P9_FID_REFTYPE: &[(u8, &str)] = &[
    (P9_FID_REF_CREATE as u8, "create "),
    (P9_FID_REF_GET as u8, "get    "),
    (P9_FID_REF_PUT as u8, "put    "),
    (P9_FID_REF_DESTROY as u8, "destroy"),
];

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum P9FidRefType {
    P9_FID_REF_CREATE,
    P9_FID_REF_GET,
    P9_FID_REF_PUT,
    P9_FID_REF_DESTROY,
}

pub const P9_PROTO_DUMP_SZ: usize = 32;

// The C TRACE_EVENT declarations are kernel tracepoint registrations. Their
// field layouts and printf expressions are retained here as Rust-facing
// records; the referenced kernel types and trace backend remain external.

#[repr(C)]
pub struct P9ClientReqEntry {
    pub clnt: *mut core::ffi::c_void,
    pub r#type: u8,
    pub tag: u32,
}

#[repr(C)]
pub struct P9ClientResEntry {
    pub clnt: *mut core::ffi::c_void,
    pub r#type: u8,
    pub tag: u32,
    pub err: u32,
}

#[repr(C)]
pub struct P9FidRefEntry {
    pub fid: i32,
    pub refcount: i32,
    pub r#type: u8,
}

// 9p_protocol_dump contains a dynamically sized byte array `line` whose
// length is min(pdu->capacity, P9_PROTO_DUMP_SZ), copied from pdu->sdata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
