/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Greybus operations
 *
 * Copyright 2014 Google Inc.
 * Copyright 2014 Linaro Ltd.
 */

/* Linux header dependencies are supplied by the surrounding translation. */

pub const GB_OPERATION_TIMEOUT_DEFAULT: u32 = 1000;
pub const GB_MESSAGE_TYPE_RESPONSE: u8 = 0x80;

#[repr(C)]
pub enum gb_operation_result {
    GB_OP_SUCCESS = 0x00,
    GB_OP_INTERRUPTED = 0x01,
    GB_OP_TIMEOUT = 0x02,
    GB_OP_NO_MEMORY = 0x03,
    GB_OP_PROTOCOL_BAD = 0x04,
    GB_OP_OVERFLOW = 0x05,
    GB_OP_INVALID = 0x06,
    GB_OP_RETRY = 0x07,
    GB_OP_NONEXISTENT = 0x08,
    GB_OP_UNKNOWN_ERROR = 0xfe,
    GB_OP_MALFUNCTION = 0xff,
}

pub const GB_OPERATION_MESSAGE_SIZE_MIN: usize = core::mem::size_of::<gb_operation_msg_hdr>();
pub const GB_OPERATION_MESSAGE_SIZE_MAX: u16 = u16::MAX;

/*
 * Protocol code should only examine the payload and payload_size fields, and
 * host-controller drivers may use the hcpriv field. All other fields are
 * intended to be private to the operations core code.
 */
#[repr(C)]
pub struct gb_message {
    pub operation: *mut gb_operation,
    pub header: *mut gb_operation_msg_hdr,
    pub payload: *mut core::ffi::c_void,
    pub payload_size: usize,
    pub buffer: *mut core::ffi::c_void,
    pub hcpriv: *mut core::ffi::c_void,
}

pub const GB_OPERATION_FLAG_INCOMING: usize = 1usize << 0;
pub const GB_OPERATION_FLAG_UNIDIRECTIONAL: usize = 1usize << 1;
pub const GB_OPERATION_FLAG_SHORT_RESPONSE: usize = 1usize << 2;
pub const GB_OPERATION_FLAG_CORE: usize = 1usize << 3;
pub const GB_OPERATION_FLAG_USER_MASK: usize =
    GB_OPERATION_FLAG_SHORT_RESPONSE | GB_OPERATION_FLAG_UNIDIRECTIONAL;

/*
 * A Greybus operation is a remote procedure call performed over a
 * connection between two UniPro interfaces.
 *
 * Every operation consists of a request message sent to the other
 * end of the connection coupled with a reply message returned to
 * the sender. Every operation has a type, whose interpretation is
 * dependent on the protocol associated with the connection.
 *
 * Only four things in an operation structure are intended to be
 * directly usable by protocol handlers: the operation's connection
 * pointer; the operation type; the request message payload (and
 * size); and the response message payload (and size). Note that a
 * message with a 0-byte payload has a null message payload pointer.
 *
 * In addition, every operation has a result, which is an errno
 * value. Protocol handlers access the operation result using
 * gb_operation_result().
 */
pub type gb_operation_callback = Option<unsafe extern "C" fn(*mut gb_operation)>;

#[repr(C)]
pub struct gb_operation {
    pub connection: *mut gb_connection,
    pub request: *mut gb_message,
    pub response: *mut gb_message,
    pub flags: libc::c_ulong,
    pub type_: u8,
    pub id: u16,
    pub r#errno: libc::c_int,
    pub work: work_struct,
    pub callback: gb_operation_callback,
    pub completion: completion,
    pub timer: timer_list,
    pub kref: kref,
    pub waiters: atomic_t,
    pub active: libc::c_int,
    pub links: list_head,
    pub private: *mut core::ffi::c_void,
}

#[inline]
pub unsafe fn gb_operation_is_incoming(operation: *mut gb_operation) -> bool {
    ((*operation).flags & GB_OPERATION_FLAG_INCOMING as libc::c_ulong) != 0
}

#[inline]
pub unsafe fn gb_operation_is_unidirectional(operation: *mut gb_operation) -> bool {
    ((*operation).flags & GB_OPERATION_FLAG_UNIDIRECTIONAL as libc::c_ulong) != 0
}

#[inline]
pub unsafe fn gb_operation_short_response_allowed(operation: *mut gb_operation) -> bool {
    ((*operation).flags & GB_OPERATION_FLAG_SHORT_RESPONSE as libc::c_ulong) != 0
}

#[inline]
pub unsafe fn gb_operation_is_core(operation: *mut gb_operation) -> bool {
    ((*operation).flags & GB_OPERATION_FLAG_CORE as libc::c_ulong) != 0
}

extern "C" {
    pub fn gb_connection_recv(connection: *mut gb_connection, data: *mut core::ffi::c_void, size: usize);
    pub fn gb_operation_result(operation: *mut gb_operation) -> libc::c_int;
    pub fn gb_operation_get_payload_size_max(connection: *mut gb_connection) -> usize;
    pub fn gb_operation_create_flags(connection: *mut gb_connection, type_: u8, request_size: usize, response_size: usize, flags: libc::c_ulong, gfp: gfp_t) -> *mut gb_operation;
    pub fn gb_operation_create_core(connection: *mut gb_connection, type_: u8, request_size: usize, response_size: usize, flags: libc::c_ulong, gfp: gfp_t) -> *mut gb_operation;
    pub fn gb_operation_get(operation: *mut gb_operation);
    pub fn gb_operation_put(operation: *mut gb_operation);
    pub fn gb_operation_response_alloc(operation: *mut gb_operation, response_size: usize, gfp: gfp_t) -> bool;
    pub fn gb_operation_request_send(operation: *mut gb_operation, callback: gb_operation_callback, timeout: u32, gfp: gfp_t) -> libc::c_int;
    pub fn gb_operation_request_send_sync_timeout(operation: *mut gb_operation, timeout: u32) -> libc::c_int;
    pub fn gb_operation_cancel(operation: *mut gb_operation, errno: libc::c_int);
    pub fn gb_operation_cancel_incoming(operation: *mut gb_operation, errno: libc::c_int);
    pub fn greybus_message_sent(hd: *mut gb_host_device, message: *mut gb_message, status: libc::c_int);
    pub fn gb_operation_sync_timeout(connection: *mut gb_connection, type_: libc::c_int, request: *mut core::ffi::c_void, request_size: libc::c_int, response: *mut core::ffi::c_void, response_size: libc::c_int, timeout: u32) -> libc::c_int;
    pub fn gb_operation_unidirectional_timeout(connection: *mut gb_connection, type_: libc::c_int, request: *mut core::ffi::c_void, request_size: libc::c_int, timeout: u32) -> libc::c_int;
    pub fn gb_operation_init() -> libc::c_int;
    pub fn gb_operation_exit();
}

#[inline]
pub unsafe fn gb_operation_create(connection: *mut gb_connection, type_: u8, request_size: usize, response_size: usize, gfp: gfp_t) -> *mut gb_operation {
    gb_operation_create_flags(connection, type_, request_size, response_size, 0, gfp)
}

#[inline]
pub unsafe fn gb_operation_request_send_sync(operation: *mut gb_operation) -> libc::c_int {
    gb_operation_request_send_sync_timeout(operation, GB_OPERATION_TIMEOUT_DEFAULT)
}

#[inline]
pub unsafe fn gb_operation_sync(connection: *mut gb_connection, type_: libc::c_int, request: *mut core::ffi::c_void, request_size: libc::c_int, response: *mut core::ffi::c_void, response_size: libc::c_int) -> libc::c_int {
    gb_operation_sync_timeout(connection, type_, request, request_size, response, response_size, GB_OPERATION_TIMEOUT_DEFAULT)
}

#[inline]
pub unsafe fn gb_operation_unidirectional(connection: *mut gb_connection, type_: libc::c_int, request: *mut core::ffi::c_void, request_size: libc::c_int) -> libc::c_int {
    gb_operation_unidirectional_timeout(connection, type_, request, request_size, GB_OPERATION_TIMEOUT_DEFAULT)
}

#[inline]
pub unsafe fn gb_operation_get_data(operation: *mut gb_operation) -> *mut core::ffi::c_void {
    (*operation).private
}

#[inline]
pub unsafe fn gb_operation_set_data(operation: *mut gb_operation, data: *mut core::ffi::c_void) {
    (*operation).private = data;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
