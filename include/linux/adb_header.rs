/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for ADB (Apple Desktop Bus) support.
 */

use core::ffi::{c_char, c_int, c_uchar, c_void};

/* Declarations supplied by the corresponding Linux UAPI header. */

#[repr(C)]
pub struct adb_request {
    pub data: [c_uchar; 32],
    pub nbytes: c_int,
    pub reply: [c_uchar; 32],
    pub reply_len: c_int,
    pub reply_expected: c_uchar,
    pub sent: c_uchar,
    pub complete: c_uchar,
    pub done: Option<unsafe extern "C" fn(*mut adb_request)>,
    pub arg: *mut c_void,
    pub next: *mut adb_request,
}

#[repr(C)]
pub struct adb_ids {
    pub nids: c_int,
    pub id: [c_uchar; 16],
}

/* Structure which encapsulates a low-level ADB driver */
#[repr(C)]
pub struct adb_driver {
    pub name: [c_char; 16],
    pub probe: Option<unsafe extern "C" fn() -> c_int>,
    pub init: Option<unsafe extern "C" fn() -> c_int>,
    pub send_request:
        Option<unsafe extern "C" fn(*mut adb_request, c_int) -> c_int>,
    pub autopoll: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub poll: Option<unsafe extern "C" fn()>,
    pub reset_bus: Option<unsafe extern "C" fn() -> c_int>,
}

/* Values for adb_request flags */
pub const ADBREQ_REPLY: c_int = 1; /* expect reply */
pub const ADBREQ_SYNC: c_int = 2; /* poll until done */
pub const ADBREQ_NOSEND: c_int = 4; /* build the request, but don't send it */

/* Messages sent thru the client_list notifier. You should NOT stop
   the operation, at least not with this version */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum adb_message {
    ADB_MSG_POWERDOWN,
    ADB_MSG_PRE_RESET,
    ADB_MSG_POST_RESET,
}

/* Supplied by the notifier subsystem. */
#[repr(C)]
pub struct blocking_notifier_head {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut adb_client_list: blocking_notifier_head;

    pub fn adb_request(
        req: *mut adb_request,
        done: Option<unsafe extern "C" fn(*mut adb_request)>,
        flags: c_int,
        nbytes: c_int,
        ...,
    ) -> c_int;
    pub fn adb_register(
        default_id: c_int,
        handler_id: c_int,
        ids: *mut adb_ids,
        handler: Option<unsafe extern "C" fn(*mut c_uchar, c_int, c_int)>,
    ) -> c_int;
    pub fn adb_unregister(index: c_int) -> c_int;
    pub fn adb_poll();
    pub fn adb_input(data: *mut c_uchar, length: c_int, unused: c_int);
    pub fn adb_reset_bus() -> c_int;

    pub fn adb_try_handler_change(address: c_int, new_id: c_int) -> c_int;
    pub fn adb_get_infos(
        address: c_int,
        original_address: *mut c_int,
        handler_id: *mut c_int,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
