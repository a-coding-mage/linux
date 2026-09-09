/* SPDX-License-Identifier: GPL-2.0-or-later */
/* I2C message transfer tracepoints
 *
 * Copyright (C) 2013 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// TRACE_SYSTEM i2c
// The C header guard and TRACE_HEADER_MULTI_READ condition are preprocessor
// constructs; Rust's module/import system supplies the corresponding guard.

use core::ffi::c_int;

/* drivers/i2c/i2c-core-base.c */
unsafe extern "C" {
    pub fn i2c_transfer_trace_reg() -> c_int;
    pub fn i2c_transfer_trace_unreg();
}

/* __i2c_transfer() write request */
#[repr(C)]
pub struct I2cWriteEntry {
    pub adapter_nr: c_int,
    pub msg_nr: u16,
    pub addr: u16,
    pub flags: u16,
    pub len: u16,
    // __dynamic_array(__u8, buf, msg->len)
    pub buf: [u8; 0],
}

/* __i2c_transfer() read request */
#[repr(C)]
pub struct I2cReadEntry {
    pub adapter_nr: c_int,
    pub msg_nr: u16,
    pub addr: u16,
    pub flags: u16,
    pub len: u16,
}

/* __i2c_transfer() read reply */
#[repr(C)]
pub struct I2cReplyEntry {
    pub adapter_nr: c_int,
    pub msg_nr: u16,
    pub addr: u16,
    pub flags: u16,
    pub len: u16,
    // __dynamic_array(__u8, buf, msg->len)
    pub buf: [u8; 0],
}

/* __i2c_transfer() result */
#[repr(C)]
pub struct I2cResultEntry {
    pub adapter_nr: c_int,
    pub nr_msgs: u16,
    pub ret: i16,
}

// The following TRACE_EVENT_FN definitions preserve the C trace-event
// interfaces. Their TP_PROTO/TP_ARGS, assignments, and TP_printk formats are
// retained here as comments because the event machinery is supplied by the
// tracepoint dependency.
//
// i2c_write(const struct i2c_adapter *adap, const struct i2c_msg *msg, int num)
//   adapter_nr = adap->nr; msg_nr = num; addr = msg->addr;
//   flags = msg->flags; len = msg->len;
//   memcpy(buf, msg->buf, msg->len);
//   "i2c-%d #%u a=%03x f=%04x l=%u [%*phD]"
//   registration: i2c_transfer_trace_reg / i2c_transfer_trace_unreg
//
// i2c_read(const struct i2c_adapter *adap, const struct i2c_msg *msg, int num)
//   adapter_nr = adap->nr; msg_nr = num; addr = msg->addr;
//   flags = msg->flags; len = msg->len;
//   "i2c-%d #%u a=%03x f=%04x l=%u"
//   registration: i2c_transfer_trace_reg / i2c_transfer_trace_unreg
//
// i2c_reply(const struct i2c_adapter *adap, const struct i2c_msg *msg, int num)
//   adapter_nr = adap->nr; msg_nr = num; addr = msg->addr;
//   flags = msg->flags; len = msg->len;
//   memcpy(buf, msg->buf, msg->len);
//   "i2c-%d #%u a=%03x f=%04x l=%u [%*phD]"
//   registration: i2c_transfer_trace_reg / i2c_transfer_trace_unreg
//
// i2c_result(const struct i2c_adapter *adap, int num, int ret)
//   adapter_nr = adap->nr; nr_msgs = num; ret = ret;
//   "i2c-%d n=%u ret=%d"
//   registration: i2c_transfer_trace_reg / i2c_transfer_trace_unreg

// #include <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
