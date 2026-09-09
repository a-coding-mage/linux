/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2025, Google LLC.
 * Pasha Tatashin <pasha.tatashin@soleen.com>
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/bug.h, linux/compiler.h, linux/kho/abi/luo.h, linux/list.h,
// linux/mutex.h, linux/refcount.h, linux/rwsem.h, linux/types.h, and
// uapi/linux/liveupdate.h.

use core::ffi::c_ulong;

#[repr(C)]
pub struct liveupdate_file_op_args {
    pub handler: *mut liveupdate_file_handler,
    pub retrieve_status: i32,
    pub file: *mut file,
    pub serialized_data: u64,
    pub private_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct liveupdate_file_ops {
    pub can_preserve: Option<unsafe extern "C" fn(*mut liveupdate_file_handler, *mut file) -> bool>,
    pub preserve: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args) -> i32>,
    pub unpreserve: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args)>,
    pub freeze: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args) -> i32>,
    pub unfreeze: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args)>,
    pub retrieve: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args) -> i32>,
    pub can_finish: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args) -> bool>,
    pub finish: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args)>,
    pub get_id: Option<unsafe extern "C" fn(*mut file) -> c_ulong>,
    pub owner: *mut module,
}

#[repr(C)]
pub struct liveupdate_file_handler {
    pub ops: *const liveupdate_file_ops,
    pub compatible: [core::ffi::c_char; LIVEUPDATE_HNDL_COMPAT_LENGTH],
    pub list: list_head,
    pub flb_list: list_head,
}

#[repr(C)]
pub struct liveupdate_flb_op_args {
    pub flb: *mut liveupdate_flb,
    pub data: u64,
    pub obj: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct liveupdate_flb_ops {
    pub preserve: Option<unsafe extern "C" fn(*mut liveupdate_flb_op_args) -> i32>,
    pub unpreserve: Option<unsafe extern "C" fn(*mut liveupdate_flb_op_args)>,
    pub retrieve: Option<unsafe extern "C" fn(*mut liveupdate_flb_op_args) -> i32>,
    pub finish: Option<unsafe extern "C" fn(*mut liveupdate_flb_op_args)>,
    pub owner: *mut module,
}

#[repr(C)]
pub struct luo_flb_private_state {
    pub count: refcount_t,
    pub data: u64,
    pub obj: *mut core::ffi::c_void,
    pub lock: mutex,
    pub finished: bool,
    pub retrieve_status: i32,
}

#[repr(C)]
pub struct luo_flb_private {
    pub list: list_head,
    pub outgoing: luo_flb_private_state,
    pub incoming: luo_flb_private_state,
    pub users: i32,
    pub initialized: bool,
}

#[repr(C)]
pub struct liveupdate_flb {
    pub ops: *const liveupdate_flb_ops,
    pub compatible: [core::ffi::c_char; LIVEUPDATE_FLB_COMPAT_LENGTH],
    pub private: luo_flb_private,
}

// The CONFIG_LIVEUPDATE branch is selected by the kernel build configuration.
#[cfg(CONFIG_LIVEUPDATE)]
extern "C" {
    pub fn liveupdate_enabled() -> bool;
    pub fn liveupdate_reboot() -> i32;
    pub fn liveupdate_register_file_handler(fh: *mut liveupdate_file_handler) -> i32;
    pub fn liveupdate_unregister_file_handler(fh: *mut liveupdate_file_handler);
    pub fn liveupdate_register_flb(fh: *mut liveupdate_file_handler, flb: *mut liveupdate_flb) -> i32;
    pub fn liveupdate_unregister_flb(fh: *mut liveupdate_file_handler, flb: *mut liveupdate_flb);
    pub fn liveupdate_flb_get_incoming(flb: *mut liveupdate_flb, objp: *mut *mut core::ffi::c_void) -> i32;
    pub fn liveupdate_flb_put_incoming(flb: *mut liveupdate_flb);
    pub fn liveupdate_flb_get_outgoing(flb: *mut liveupdate_flb, objp: *mut *mut core::ffi::c_void) -> i32;
    pub fn liveupdate_flb_put_outgoing(flb: *mut liveupdate_flb);
}

#[cfg(not(CONFIG_LIVEUPDATE))]
pub unsafe fn liveupdate_enabled() -> bool { false }
#[cfg(not(CONFIG_LIVEUPDATE))]
pub unsafe fn liveupdate_reboot() -> i32 { 0 }
#[cfg(not(CONFIG_LIVEUPDATE))]
pub unsafe fn liveupdate_register_file_handler(_: *mut liveupdate_file_handler) -> i32 { -EOPNOTSUPP }
#[cfg(not(CONFIG_LIVEUPDATE))]
pub unsafe fn liveupdate_unregister_file_handler(_: *mut liveupdate_file_handler) {}
#[cfg(not(CONFIG_LIVEUPDATE))]
pub unsafe fn liveupdate_register_flb(_: *mut liveupdate_file_handler, _: *mut liveupdate_flb) -> i32 { -EOPNOTSUPP }
#[cfg(not(CONFIG_LIVEUPDATE))]
pub unsafe fn liveupdate_unregister_flb(_: *mut liveupdate_file_handler, _: *mut liveupdate_flb) {}
#[cfg(not(CONFIG_LIVEUPDATE))]
pub unsafe fn liveupdate_flb_get_incoming(_: *mut liveupdate_flb, _: *mut *mut core::ffi::c_void) -> i32 { -EOPNOTSUPP }
#[cfg(not(CONFIG_LIVEUPDATE))]
pub unsafe fn liveupdate_flb_put_incoming(_: *mut liveupdate_flb) {}
#[cfg(not(CONFIG_LIVEUPDATE))]
pub unsafe fn liveupdate_flb_get_outgoing(_: *mut liveupdate_flb, _: *mut *mut core::ffi::c_void) -> i32 { -EOPNOTSUPP }
#[cfg(not(CONFIG_LIVEUPDATE))]
pub unsafe fn liveupdate_flb_put_outgoing(_: *mut liveupdate_flb) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
