// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024, Advanced Micro Devices, Inc.
 */

// Declarations supplied by the corresponding kernel and driver dependencies
// are intentionally left external to this translation unit.

use core::ffi::c_void;
use core::mem::size_of;

extern "C" {
    fn memcpy_fromio(dst: *mut c_void, src: *const c_void, count: usize);
    fn print_hex_dump_debug(
        prefix_str: *const i8,
        prefix_type: u32,
        rowsize: u32,
        groupsize: u32,
        buf: *const c_void,
        len: usize,
        ascii: bool,
    );
    fn complete(comp: *mut completion);
    fn wait_for_completion_timeout(comp: *mut completion, timeout: u64) -> i64;
    fn xdna_mailbox_send_msg(
        chann: *mut mailbox_channel,
        msg: *mut xdna_mailbox_msg,
        timeout: u32,
    ) -> i32;
    fn msecs_to_jiffies(msecs: u32) -> u64;
    fn xdna_err(xdna: *mut amdxdna_dev, fmt: *const i8, ...);
}

// These types, constants, and fields are defined by the included driver headers.
#[allow(non_camel_case_types)]
enum amdxdna_dev {}
#[allow(non_camel_case_types)]
enum mailbox_channel {}
#[allow(non_camel_case_types)]
enum completion {}
#[allow(non_camel_case_types)]
struct xdna_notify {
    data: *mut c_void,
    size: usize,
    error: i32,
    comp: completion,
}
#[allow(non_camel_case_types)]
struct xdna_mailbox_msg {
    handle: *mut c_void,
}

const EINVAL: i32 = 22;
const ETIME: i32 = 62;

extern "C" {
    static TX_TIMEOUT: u32;
    static RX_TIMEOUT: u32;
}

#[no_mangle]
pub unsafe extern "C" fn xdna_msg_cb(
    handle: *mut c_void,
    data: *mut c_void,
    size: usize,
) -> i32 {
    let cb_arg = handle as *mut xdna_notify;
    let mut ret: i32;

    if data.is_null() {
        ret = (*cb_arg).error;
        complete(&mut (*cb_arg).comp);
        return ret;
    }

    if (*cb_arg).size != size {
        (*cb_arg).error = -EINVAL;
        ret = (*cb_arg).error;
        complete(&mut (*cb_arg).comp);
        return ret;
    }

    memcpy_fromio((*cb_arg).data, data as *const c_void, (*cb_arg).size);
    print_hex_dump_debug(
        b"resp data: \0".as_ptr() as *const i8,
        1, // DUMP_PREFIX_OFFSET
        16,
        4,
        (*cb_arg).data,
        (*cb_arg).size,
        true,
    );
    ret = (*cb_arg).error;
    complete(&mut (*cb_arg).comp);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn xdna_send_msg_wait(
    xdna: *mut amdxdna_dev,
    chann: *mut mailbox_channel,
    msg: *mut xdna_mailbox_msg,
) -> i32 {
    let hdl = (*msg).handle as *mut xdna_notify;
    let mut ret = xdna_mailbox_send_msg(chann, msg, TX_TIMEOUT);
    if ret != 0 {
        xdna_err(xdna, b"Send message failed, ret %d\0".as_ptr() as *const i8, ret);
        return ret;
    }

    ret = wait_for_completion_timeout(&mut (*hdl).comp, msecs_to_jiffies(RX_TIMEOUT)) as i32;
    if ret == 0 {
        xdna_err(xdna, b"Wait for completion timeout\0".as_ptr() as *const i8);
        return -ETIME;
    }

    (*hdl).error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
