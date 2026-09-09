/* SPDX-License-Identifier: MIT */
/*
 * Details of the "wire" protocol between Xen Store Daemon and client
 * library or guest kernel.
 * Copyright (C) 2005 Rusty Russell IBM Corporation
 */

#[repr(u32)]
pub enum xsd_sockmsg_type {
    XS_CONTROL = 0,
    XS_DIRECTORY,
    XS_READ,
    XS_GET_PERMS,
    XS_WATCH,
    XS_UNWATCH,
    XS_TRANSACTION_START,
    XS_TRANSACTION_END,
    XS_INTRODUCE,
    XS_RELEASE,
    XS_GET_DOMAIN_PATH,
    XS_WRITE,
    XS_MKDIR,
    XS_RM,
    XS_SET_PERMS,
    XS_WATCH_EVENT,
    XS_ERROR,
    XS_IS_DOMAIN_INTRODUCED,
    XS_RESUME,
    XS_SET_TARGET,
    /* XS_RESTRICT has been removed */
    XS_RESET_WATCHES = xsd_sockmsg_type::XS_SET_TARGET as u32 + 2,
    XS_DIRECTORY_PART,
    XS_TYPE_COUNT, // Number of valid types.
    XS_INVALID = 0xffff, // Guaranteed to remain an invalid type
}

pub const XS_DEBUG: xsd_sockmsg_type = xsd_sockmsg_type::XS_CONTROL;

pub const XS_WRITE_NONE: &str = "NONE";
pub const XS_WRITE_CREATE: &str = "CREATE";
pub const XS_WRITE_CREATE_EXCL: &str = "CREATE|EXCL";

/* We hand errors as strings, for portability. */
#[repr(C)]
pub struct xsd_errors {
    pub errnum: i32,
    pub errstring: *const core::ffi::c_char,
}

macro_rules! XSD_ERROR {
    ($x:ident) => {
        xsd_errors { errnum: $x, errstring: concat!(stringify!($x), "\0").as_ptr() as *const core::ffi::c_char }
    };
}

static xsd_errors_table: &[xsd_errors] = &[
    XSD_ERROR!(EINVAL),
    XSD_ERROR!(EACCES),
    XSD_ERROR!(EEXIST),
    XSD_ERROR!(EISDIR),
    XSD_ERROR!(ENOENT),
    XSD_ERROR!(ENOMEM),
    XSD_ERROR!(ENOSPC),
    XSD_ERROR!(EIO),
    XSD_ERROR!(ENOTEMPTY),
    XSD_ERROR!(ENOSYS),
    XSD_ERROR!(EROFS),
    XSD_ERROR!(EBUSY),
    XSD_ERROR!(EAGAIN),
    XSD_ERROR!(EISCONN),
    XSD_ERROR!(E2BIG),
];

#[repr(C)]
pub struct xsd_sockmsg {
    pub type_: u32,   // XS_???
    pub req_id: u32,  // Request identifier, echoed in daemon's response.
    pub tx_id: u32,   // Transaction id (0 if not related to a transaction).
    pub len: u32,     // Length of data following this.

    /* Generally followed by nul-terminated string(s). */
}

#[repr(u32)]
pub enum xs_watch_type {
    XS_WATCH_PATH = 0,
    XS_WATCH_TOKEN,
}

/* Inter-domain shared memory communications. */
pub const XENSTORE_RING_SIZE: usize = 1024;
pub type XENSTORE_RING_IDX = u32;
pub const fn MASK_XENSTORE_IDX(idx: XENSTORE_RING_IDX) -> XENSTORE_RING_IDX {
    idx & (XENSTORE_RING_SIZE as u32 - 1)
}

#[repr(C)]
pub struct xenstore_domain_interface {
    pub req: [core::ffi::c_char; XENSTORE_RING_SIZE], // Requests to xenstore daemon.
    pub rsp: [core::ffi::c_char; XENSTORE_RING_SIZE], // Replies and async watch events.
    pub req_cons: XENSTORE_RING_IDX,
    pub req_prod: XENSTORE_RING_IDX,
    pub rsp_cons: XENSTORE_RING_IDX,
    pub rsp_prod: XENSTORE_RING_IDX,
    pub server_features: u32, // Bitmap of features supported by the server
    pub connection: u32,
    pub error: u32,
}

/* Violating this is very bad.  See docs/misc/xenstore.txt. */
pub const XENSTORE_PAYLOAD_MAX: usize = 4096;

/* Violating these just gets you an error back */
pub const XENSTORE_ABS_PATH_MAX: usize = 3072;
pub const XENSTORE_REL_PATH_MAX: usize = 2048;

/* The ability to reconnect a ring */
pub const XENSTORE_SERVER_FEATURE_RECONNECTION: u32 = 1;
/* The presence of the "error" field in the ring page */
pub const XENSTORE_SERVER_FEATURE_ERROR: u32 = 2;

/* Valid values for the connection field */
pub const XENSTORE_CONNECTED: u32 = 0; // the steady-state
pub const XENSTORE_RECONNECT: u32 = 1; // guest has initiated a reconnect

/* Valid values for the error field */
pub const XENSTORE_ERROR_NONE: u32 = 0; // No error
pub const XENSTORE_ERROR_COMM: u32 = 1; // Communication problem
pub const XENSTORE_ERROR_RINGIDX: u32 = 2; // Invalid ring index
pub const XENSTORE_ERROR_PROTO: u32 = 3; // Protocol violation (payload too long)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
