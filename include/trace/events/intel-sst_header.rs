/* SPDX-License-Identifier: GPL-2.0 */

// C tracepoint header translation.
// TRACE_SYSTEM is "intel-sst" and TRACE_SYSTEM_VAR is intel_sst.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SstIpcMsg {
    pub val: ::core::ffi::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SstIpcMailbox {
    pub offset: ::core::ffi::c_uint,
    pub val: ::core::ffi::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SstIpcMailboxInfo {
    pub size: ::core::ffi::c_uint,
}

// DECLARE_EVENT_CLASS(sst_ipc_msg, TP_PROTO(unsigned int val), ...)
// TP_fast_assign: __entry->val = val
// TP_printk: "0x%8.8x"
pub unsafe fn sst_ipc_msg(val: ::core::ffi::c_uint) -> SstIpcMsg {
    SstIpcMsg { val }
}

// DEFINE_EVENT(sst_ipc_msg, sst_ipc_msg_tx, TP_PROTO(unsigned int val), ...)
pub unsafe fn sst_ipc_msg_tx(val: ::core::ffi::c_uint) -> SstIpcMsg {
    sst_ipc_msg(val)
}

// DEFINE_EVENT(sst_ipc_msg, sst_ipc_msg_rx, TP_PROTO(unsigned int val), ...)
pub unsafe fn sst_ipc_msg_rx(val: ::core::ffi::c_uint) -> SstIpcMsg {
    sst_ipc_msg(val)
}

// DECLARE_EVENT_CLASS(sst_ipc_mailbox,
// TP_PROTO(unsigned int offset, unsigned int val), ...)
// TP_fast_assign: __entry->offset = offset; __entry->val = val
// TP_printk: " 0x%4.4x = 0x%8.8x"
pub unsafe fn sst_ipc_mailbox(
    offset: ::core::ffi::c_uint,
    val: ::core::ffi::c_uint,
) -> SstIpcMailbox {
    SstIpcMailbox { offset, val }
}

// DEFINE_EVENT(sst_ipc_mailbox, sst_ipc_inbox_rdata, ...)
pub unsafe fn sst_ipc_inbox_rdata(
    offset: ::core::ffi::c_uint,
    val: ::core::ffi::c_uint,
) -> SstIpcMailbox {
    sst_ipc_mailbox(offset, val)
}

// DEFINE_EVENT(sst_ipc_mailbox, sst_ipc_inbox_wdata, ...)
pub unsafe fn sst_ipc_inbox_wdata(
    offset: ::core::ffi::c_uint,
    val: ::core::ffi::c_uint,
) -> SstIpcMailbox {
    sst_ipc_mailbox(offset, val)
}

// DEFINE_EVENT(sst_ipc_mailbox, sst_ipc_outbox_rdata, ...)
pub unsafe fn sst_ipc_outbox_rdata(
    offset: ::core::ffi::c_uint,
    val: ::core::ffi::c_uint,
) -> SstIpcMailbox {
    sst_ipc_mailbox(offset, val)
}

// DEFINE_EVENT(sst_ipc_mailbox, sst_ipc_outbox_wdata, ...)
pub unsafe fn sst_ipc_outbox_wdata(
    offset: ::core::ffi::c_uint,
    val: ::core::ffi::c_uint,
) -> SstIpcMailbox {
    sst_ipc_mailbox(offset, val)
}

// DECLARE_EVENT_CLASS(sst_ipc_mailbox_info, TP_PROTO(unsigned int size), ...)
// TP_fast_assign: __entry->size = size
// TP_printk: "Mailbox bytes 0x%8.8x"
pub unsafe fn sst_ipc_mailbox_info(size: ::core::ffi::c_uint) -> SstIpcMailboxInfo {
    SstIpcMailboxInfo { size }
}

// DEFINE_EVENT(sst_ipc_mailbox_info, sst_ipc_inbox_read, ...)
pub unsafe fn sst_ipc_inbox_read(size: ::core::ffi::c_uint) -> SstIpcMailboxInfo {
    sst_ipc_mailbox_info(size)
}

// DEFINE_EVENT(sst_ipc_mailbox_info, sst_ipc_inbox_write, ...)
pub unsafe fn sst_ipc_inbox_write(size: ::core::ffi::c_uint) -> SstIpcMailboxInfo {
    sst_ipc_mailbox_info(size)
}

// DEFINE_EVENT(sst_ipc_mailbox_info, sst_ipc_outbox_read, ...)
pub unsafe fn sst_ipc_outbox_read(size: ::core::ffi::c_uint) -> SstIpcMailboxInfo {
    sst_ipc_mailbox_info(size)
}

// DEFINE_EVENT(sst_ipc_mailbox_info, sst_ipc_outbox_write, ...)
pub unsafe fn sst_ipc_outbox_write(size: ::core::ffi::c_uint) -> SstIpcMailboxInfo {
    sst_ipc_mailbox_info(size)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
