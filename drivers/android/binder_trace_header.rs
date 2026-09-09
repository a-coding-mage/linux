/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Google, Inc.
 *
 * Rust translation of binder_trace.h.  The Linux tracepoint generator and
 * the definitions of the binder structures are supplied by other files.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_char;

// C forward declarations.  Their fields are defined by the binder sources.
#[repr(C)]
pub struct binder_buffer {
    pub debug_id: ::core::ffi::c_int,
    pub data_size: usize,
    pub offsets_size: usize,
    pub extra_buffers_size: usize,
}

#[repr(C)]
pub struct binder_node {
    pub debug_id: ::core::ffi::c_int,
    pub ptr: binder_uintptr_t,
}

#[repr(C)]
pub struct binder_proc;

#[repr(C)]
pub struct binder_alloc {
    pub pid: ::core::ffi::c_int,
    pub vm_start: usize,
}

#[repr(C)]
pub struct binder_ref_data {
    pub debug_id: ::core::ffi::c_int,
    pub desc: u32,
}

#[repr(C)]
pub struct binder_thread {
    pub pid: ::core::ffi::c_int,
}

#[repr(C)]
pub struct binder_transaction {
    pub debug_id: ::core::ffi::c_int,
    pub code: u32,
    pub flags: u32,
    pub to_proc: *mut binder_proc_with_pid,
    pub to_thread: *mut binder_thread,
    pub from_pid: ::core::ffi::c_int,
    pub from_tid: ::core::ffi::c_int,
    pub is_reply: bool,
}

// The binder sources provide the actual process layout used by transactions.
#[repr(C)]
pub struct binder_proc_with_pid {
    pub pid: ::core::ffi::c_int,
}

pub type binder_uintptr_t = usize;

// The following marker types correspond to TRACE_EVENT/DEFINE_EVENT instances.
// Their generated tracepoint implementations are provided externally.
pub struct binder_ioctl;
pub struct binder_function_return_class;
pub struct binder_ioctl_done;
pub struct binder_write_done;
pub struct binder_read_done;
pub struct binder_wait_for_work;
pub struct binder_txn_latency_free;
pub struct binder_transaction_received;
pub struct binder_transaction_node_to_ref;
pub struct binder_transaction_ref_to_node;
pub struct binder_transaction_ref_to_ref;
pub struct binder_transaction_fd_send;
pub struct binder_transaction_fd_recv;
pub struct binder_buffer_class;
pub struct binder_transaction_alloc_buf;
pub struct binder_transaction_buffer_release;
pub struct binder_transaction_failed_buffer_release;
pub struct binder_transaction_update_buffer_release;
pub struct binder_update_page_range;
pub struct binder_lru_page_class;
pub struct binder_alloc_lru_start;
pub struct binder_alloc_lru_end;
pub struct binder_free_lru_start;
pub struct binder_free_lru_end;
pub struct binder_alloc_page_start;
pub struct binder_alloc_page_end;
pub struct binder_unmap_user_start;
pub struct binder_unmap_user_end;
pub struct binder_unmap_kernel_start;
pub struct binder_unmap_kernel_end;
pub struct binder_command;
pub struct binder_return;
pub struct binder_netlink_report;

// C tracepoint event signatures, retained as declarations for generated users:
// binder_ioctl(unsigned int cmd, unsigned long arg)
// binder_ioctl_done(int ret), binder_write_done(int ret), binder_read_done(int ret)
// binder_wait_for_work(bool proc_work, bool transaction_stack, bool thread_todo)
// binder_txn_latency_free(struct binder_transaction *t, int from_proc,
//                         int from_thread, int to_proc, int to_thread)
// binder_transaction(bool reply, struct binder_transaction *t,
//                    struct binder_node *target_node)
// binder_transaction_received(struct binder_transaction *t)
// binder_transaction_node_to_ref(struct binder_transaction *t,
//                                struct binder_node *node,
//                                struct binder_ref_data *rdata)
// binder_transaction_ref_to_node(struct binder_transaction *t,
//                                struct binder_node *node,
//                                struct binder_ref_data *rdata)
// binder_transaction_ref_to_ref(struct binder_transaction *t,
//                               struct binder_node *node,
//                               struct binder_ref_data *src_ref,
//                               struct binder_ref_data *dest_ref)
// binder_transaction_fd_send/recv(struct binder_transaction *t, int fd, size_t offset)
// binder_transaction_alloc_buf/buffer_release/failed_buffer_release/
// binder_transaction_update_buffer_release(struct binder_buffer *buffer)
// binder_update_page_range(struct binder_alloc *alloc, bool allocate,
//                          unsigned long start, unsigned long end)
// binder_alloc_lru_start/end, binder_free_lru_start/end,
// binder_alloc_page_start/end, binder_unmap_user_start/end,
// binder_unmap_kernel_start/end(const struct binder_alloc *alloc,
//                               size_t page_index)
// binder_command(uint32_t cmd), binder_return(uint32_t cmd)
// binder_netlink_report(const char *context, struct binder_transaction *t,
//                       u32 data_size, u32 error)

// TRACE_INCLUDE_PATH ., TRACE_INCLUDE_FILE binder_trace, and trace/define_trace.h
// are build-time Linux tracepoint-generator directives with no standalone Rust
// executable equivalent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
