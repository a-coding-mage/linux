/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Copyright (C) 2019 Netronome Systems, Inc. */

// C header guard: _TLS_TRACE_H_, including TRACE_HEADER_MULTI_READ behavior.
// The Linux tracepoint macros and trace/define_trace.h are external facilities.

use core::ptr;

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[inline]
unsafe fn get_unaligned_be64(p: *const u8) -> u64 {
    u64::from_be(ptr::read_unaligned(p as *const u64))
}

#[repr(C)]
pub struct TlsDeviceOffloadSet {
    pub sk: *mut sock,
    pub rec_no: u64,
    pub dir: i32,
    pub tcp_seq: u32,
    pub ret: i32,
}

#[inline]
pub unsafe fn tls_device_offload_set(
    sk: *mut sock, dir: i32, tcp_seq: u32, rec_no: *const u8, ret: i32,
) -> TlsDeviceOffloadSet {
    TlsDeviceOffloadSet { sk, rec_no: get_unaligned_be64(rec_no), dir, tcp_seq, ret }
}

#[repr(C)]
pub struct TlsDeviceDecrypted {
    pub sk: *mut sock,
    pub rec_no: u64,
    pub tcp_seq: u32,
    pub rec_len: u32,
    pub encrypted: bool,
    pub decrypted: bool,
}

#[inline]
pub unsafe fn tls_device_decrypted(
    sk: *mut sock, tcp_seq: u32, rec_no: *const u8, rec_len: u32,
    encrypted: bool, decrypted: bool,
) -> TlsDeviceDecrypted {
    TlsDeviceDecrypted { sk, rec_no: get_unaligned_be64(rec_no), tcp_seq, rec_len, encrypted, decrypted }
}

#[repr(C)]
pub struct TlsDeviceRxResyncSend {
    pub sk: *mut sock,
    pub rec_no: u64,
    pub tcp_seq: u32,
    pub sync_type: i32,
}

#[inline]
pub unsafe fn tls_device_rx_resync_send(
    sk: *mut sock, tcp_seq: u32, rec_no: *const u8, sync_type: i32,
) -> TlsDeviceRxResyncSend {
    TlsDeviceRxResyncSend { sk, rec_no: get_unaligned_be64(rec_no), tcp_seq, sync_type }
}

#[repr(C)]
pub struct TlsDeviceRxResyncNhSchedule {
    pub sk: *mut sock,
}

#[inline]
pub fn tls_device_rx_resync_nh_schedule(sk: *mut sock) -> TlsDeviceRxResyncNhSchedule {
    TlsDeviceRxResyncNhSchedule { sk }
}

#[repr(C)]
pub struct TlsDeviceRxResyncNhDelay {
    pub sk: *mut sock,
    pub sock_data: u32,
    pub rec_len: u32,
}

#[inline]
pub fn tls_device_rx_resync_nh_delay(
    sk: *mut sock, sock_data: u32, rec_len: u32,
) -> TlsDeviceRxResyncNhDelay {
    TlsDeviceRxResyncNhDelay { sk, sock_data, rec_len }
}

#[repr(C)]
pub struct TlsDeviceTxResyncReq {
    pub sk: *mut sock,
    pub tcp_seq: u32,
    pub exp_tcp_seq: u32,
}

#[inline]
pub fn tls_device_tx_resync_req(
    sk: *mut sock, tcp_seq: u32, exp_tcp_seq: u32,
) -> TlsDeviceTxResyncReq {
    TlsDeviceTxResyncReq { sk, tcp_seq, exp_tcp_seq }
}

#[repr(C)]
pub struct TlsDeviceTxResyncSend {
    pub sk: *mut sock,
    pub rec_no: u64,
    pub tcp_seq: u32,
}

#[inline]
pub unsafe fn tls_device_tx_resync_send(
    sk: *mut sock, tcp_seq: u32, rec_no: *const u8,
) -> TlsDeviceTxResyncSend {
    TlsDeviceTxResyncSend { sk, rec_no: get_unaligned_be64(rec_no), tcp_seq }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
