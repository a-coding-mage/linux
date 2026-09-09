// SPDX-License-Identifier: GPL-2.0
/*
 * For transports using message passing.
 *
 * Derived from shm.c.
 *
 * Copyright (C) 2019-2024 ARM Ltd.
 * Copyright (C) 2020-2021 OpenSynergy GmbH
 */

/* The SCMI specification requires all parameters, message headers, return
 * arguments or any protocol data to be expressed in little endian format only.
 */
#[repr(C)]
struct ScmiMsgPayld {
    msg_header: u32,
    msg_payload: [u32; 0],
}

/* External types and helpers are supplied by common.h and other dependencies. */

unsafe fn msg_command_size(xfer: *mut ScmiXfer) -> usize {
    core::mem::size_of::<ScmiMsgPayld>() + (*xfer).tx.len
}

unsafe fn msg_response_size(xfer: *mut ScmiXfer) -> usize {
    core::mem::size_of::<ScmiMsgPayld>() + core::mem::size_of::<u32>() + (*xfer).rx.len
}

unsafe fn msg_tx_prepare(msg: *mut ScmiMsgPayld, xfer: *mut ScmiXfer) {
    (*msg).msg_header = pack_scmi_header(&(*xfer).hdr).to_le();
    if !(*xfer).tx.buf.is_null() {
        core::ptr::copy_nonoverlapping(
            (*xfer).tx.buf as *const u8,
            (*msg).msg_payload.as_mut_ptr() as *mut u8,
            (*xfer).tx.len,
        );
    }
}

unsafe fn msg_read_header(msg: *mut ScmiMsgPayld) -> u32 {
    u32::from_le((*msg).msg_header)
}

unsafe fn msg_fetch_response(msg: *mut ScmiMsgPayld, len: usize, xfer: *mut ScmiXfer) {
    let prefix_len = core::mem::size_of::<ScmiMsgPayld>() + core::mem::size_of::<u32>();

    (*xfer).hdr.status = u32::from_le(*(*msg).msg_payload.as_ptr());
    (*xfer).rx.len = core::cmp::min(
        (*xfer).rx.len,
        if len >= prefix_len { len - prefix_len } else { 0 },
    );

    /* Take a copy to the rx buffer.. */
    core::ptr::copy_nonoverlapping(
        (*msg).msg_payload.as_ptr().add(1) as *const u8,
        (*xfer).rx.buf as *mut u8,
        (*xfer).rx.len,
    );
}

unsafe fn msg_fetch_notification(
    msg: *mut ScmiMsgPayld,
    len: usize,
    max_len: usize,
    xfer: *mut ScmiXfer,
) {
    (*xfer).rx.len = core::cmp::min(
        max_len,
        if len >= core::mem::size_of::<ScmiMsgPayld>() {
            len - core::mem::size_of::<ScmiMsgPayld>()
        } else {
            0
        },
    );

    /* Take a copy to the rx buffer.. */
    core::ptr::copy_nonoverlapping(
        (*msg).msg_payload.as_ptr() as *const u8,
        (*xfer).rx.buf as *mut u8,
        (*xfer).rx.len,
    );
}

static SCMI_MSG_OPS: ScmiMessageOperations = ScmiMessageOperations {
    tx_prepare: msg_tx_prepare,
    command_size: msg_command_size,
    response_size: msg_response_size,
    read_header: msg_read_header,
    fetch_response: msg_fetch_response,
    fetch_notification: msg_fetch_notification,
};

pub unsafe fn scmi_message_operations_get() -> *const ScmiMessageOperations {
    &SCMI_MSG_OPS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
