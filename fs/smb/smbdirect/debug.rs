// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2017, Microsoft Corporation.
 *   Copyright (c) 2025, Stefan Metzmacher
 */

// Declarations supplied by the corresponding kernel/internal headers are kept
// as external Rust items here.

use core::ffi::{c_char, c_uint, c_ulonglong, c_void};
use core::sync::atomic::{AtomicU32, Ordering};

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smbdirect_socket_parameters {
    pub recv_credit_max: c_uint,
    pub send_credit_target: c_uint,
    pub max_send_size: c_uint,
    pub max_fragmented_recv_size: c_uint,
    pub max_fragmented_send_size: c_uint,
    pub max_recv_size: c_uint,
    pub keepalive_interval_msec: c_uint,
    pub max_read_write_size: c_uint,
    pub responder_resources: c_uint,
    pub max_frmr_depth: c_uint,
}

#[repr(C)]
pub struct smbdirect_socket_statistics {
    pub get_receive_buffer: c_ulonglong,
    pub put_receive_buffer: c_ulonglong,
    pub send_empty: c_ulonglong,
    pub enqueue_reassembly_queue: c_ulonglong,
    pub dequeue_reassembly_queue: c_ulonglong,
}

#[repr(C)]
pub struct smbdirect_reassembly {
    pub data_length: c_uint,
    pub queue_length: c_uint,
}

#[repr(C)]
pub struct smbdirect_credits {
    pub count: AtomicU32,
    pub target: c_uint,
}

#[repr(C)]
pub struct smbdirect_send_io {
    pub credits: smbdirect_credits,
    pub pending: smbdirect_credits,
}

#[repr(C)]
pub struct smbdirect_recv_io {
    pub credits: smbdirect_credits,
    pub reassembly: smbdirect_reassembly,
}

#[repr(C)]
pub struct smbdirect_mr_io {
    pub type_: c_uint,
    pub ready: smbdirect_credits,
    pub used: smbdirect_credits,
}

#[repr(C)]
pub struct smbdirect_socket {
    pub parameters: smbdirect_socket_parameters,
    pub status: c_uint,
    pub statistics: smbdirect_socket_statistics,
    pub recv_io: smbdirect_recv_io,
    pub send_io: smbdirect_send_io,
    pub mr_io: smbdirect_mr_io,
}

extern "C" {
    fn seq_puts(m: *mut seq_file, s: *const c_char);
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn smbdirect_socket_status_string(status: c_uint) -> *const c_char;
}

// The build configuration supplies this protocol constant.
pub const SMBDIRECT_V1: c_uint = 1;

#[inline]
unsafe fn atomic_read(value: *const AtomicU32) -> c_uint {
    (*value).load(Ordering::Relaxed)
}

#[no_mangle]
pub unsafe extern "C" fn smbdirect_connection_legacy_debug_proc_show(
    sc: *mut smbdirect_socket,
    rdma_readwrite_threshold: c_uint,
    m: *mut seq_file,
) {
    if sc.is_null() {
        return;
    }
    let sp = &(*sc).parameters;

    seq_puts(m, b"\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"SMBDirect protocol version: 0x%x \0".as_ptr() as *const c_char, SMBDIRECT_V1);
    seq_printf(m, b"transport status: %s (%u)\0".as_ptr() as *const c_char,
               smbdirect_socket_status_string((*sc).status), (*sc).status);

    seq_puts(m, b"\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"Conn receive_credit_max: %u \0".as_ptr() as *const c_char, sp.recv_credit_max);
    seq_printf(m, b"send_credit_target: %u max_send_size: %u\0".as_ptr() as *const c_char,
               sp.send_credit_target, sp.max_send_size);

    seq_puts(m, b"\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"Conn max_fragmented_recv_size: %u \0".as_ptr() as *const c_char, sp.max_fragmented_recv_size);
    seq_printf(m, b"max_fragmented_send_size: %u max_receive_size:%u\0".as_ptr() as *const c_char,
               sp.max_fragmented_send_size, sp.max_recv_size);

    seq_puts(m, b"\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"Conn keep_alive_interval: %u \0".as_ptr() as *const c_char, sp.keepalive_interval_msec / 1000);
    seq_printf(m, b"max_readwrite_size: %u rdma_readwrite_threshold: %u\0".as_ptr() as *const c_char,
               sp.max_read_write_size, rdma_readwrite_threshold);

    seq_puts(m, b"\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"Debug count_get_receive_buffer: %llu \0".as_ptr() as *const c_char, (*sc).statistics.get_receive_buffer);
    seq_printf(m, b"count_put_receive_buffer: %llu count_send_empty: %llu\0".as_ptr() as *const c_char,
               (*sc).statistics.put_receive_buffer, (*sc).statistics.send_empty);

    seq_puts(m, b"\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"Read Queue count_enqueue_reassembly_queue: %llu \0".as_ptr() as *const c_char, (*sc).statistics.enqueue_reassembly_queue);
    seq_printf(m, b"count_dequeue_reassembly_queue: %llu \0".as_ptr() as *const c_char, (*sc).statistics.dequeue_reassembly_queue);
    seq_printf(m, b"reassembly_data_length: %u \0".as_ptr() as *const c_char, (*sc).recv_io.reassembly.data_length);
    seq_printf(m, b"reassembly_queue_length: %u\0".as_ptr() as *const c_char, (*sc).recv_io.reassembly.queue_length);

    seq_puts(m, b"\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"Current Credits send_credits: %u \0".as_ptr() as *const c_char, atomic_read(&(*sc).send_io.credits.count));
    seq_printf(m, b"receive_credits: %u receive_credit_target: %u\0".as_ptr() as *const c_char,
               atomic_read(&(*sc).recv_io.credits.count), (*sc).recv_io.credits.target);

    seq_puts(m, b"\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"Pending send_pending: %u \0".as_ptr() as *const c_char, atomic_read(&(*sc).send_io.pending.count));

    seq_puts(m, b"\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"MR responder_resources: %u \0".as_ptr() as *const c_char, sp.responder_resources);
    seq_printf(m, b"max_frmr_depth: %u mr_type: 0x%x\0".as_ptr() as *const c_char,
               sp.max_frmr_depth, (*sc).mr_io.type_);

    seq_puts(m, b"\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"MR mr_ready_count: %u mr_used_count: %u\0".as_ptr() as *const c_char,
               atomic_read(&(*sc).mr_io.ready.count), atomic_read(&(*sc).mr_io.used.count));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
