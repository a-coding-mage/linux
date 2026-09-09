/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * VMware vSockets Driver
 *
 * Copyright (C) 2009-2013 VMware, Inc. All rights reserved.
 */

// Dependencies supplied by the included Linux and VMCI headers are expected
// to provide the corresponding definitions in the containing translation.

/* Comment this out to compare with old protocol. */
pub const VSOCK_OPTIMIZATION_WAITING_NOTIFY: u32 = 1;
/* Comment this out to remove flow control for "new" protocol. */
pub const VSOCK_OPTIMIZATION_FLOW_CONTROL: u32 = 1;

pub const VMCI_TRANSPORT_MAX_DGRAM_RESENDS: u32 = 10;

#[repr(C)]
pub struct vmci_transport_recv_notify_data {
    pub consume_head: u64,
    pub produce_tail: u64,
    pub notify_on_block: bool,
}

#[repr(C)]
pub struct vmci_transport_send_notify_data {
    pub consume_head: u64,
    pub produce_tail: u64,
}

// Opaque declarations corresponding to types supplied by included headers.
pub enum sock {}
pub enum vsock_sock {}
pub enum vmci_transport_packet {}
pub enum sockaddr_vm {}

/* Socket notification callbacks. */
#[repr(C)]
pub struct vmci_transport_notify_ops {
    pub socket_init: Option<unsafe extern "C" fn(sk: *mut sock)>,
    pub socket_destruct: Option<unsafe extern "C" fn(vsk: *mut vsock_sock)>,
    pub poll_in: Option<unsafe extern "C" fn(
        sk: *mut sock,
        target: usize,
        data_ready_now: *mut bool,
    ) -> i32>,
    pub poll_out: Option<unsafe extern "C" fn(
        sk: *mut sock,
        target: usize,
        space_avail_now: *mut bool,
    ) -> i32>,
    pub handle_notify_pkt: Option<unsafe extern "C" fn(
        sk: *mut sock,
        pkt: *mut vmci_transport_packet,
        bottom_half: bool,
        dst: *mut sockaddr_vm,
        src: *mut sockaddr_vm,
        pkt_processed: *mut bool,
    )>,
    pub recv_init: Option<unsafe extern "C" fn(
        sk: *mut sock,
        target: usize,
        data: *mut vmci_transport_recv_notify_data,
    ) -> i32>,
    pub recv_pre_block: Option<unsafe extern "C" fn(
        sk: *mut sock,
        target: usize,
        data: *mut vmci_transport_recv_notify_data,
    ) -> i32>,
    pub recv_pre_dequeue: Option<unsafe extern "C" fn(
        sk: *mut sock,
        target: usize,
        data: *mut vmci_transport_recv_notify_data,
    ) -> i32>,
    pub recv_post_dequeue: Option<unsafe extern "C" fn(
        sk: *mut sock,
        target: usize,
        copied: isize,
        data_read: bool,
        data: *mut vmci_transport_recv_notify_data,
    ) -> i32>,
    pub send_init: Option<unsafe extern "C" fn(
        sk: *mut sock,
        data: *mut vmci_transport_send_notify_data,
    ) -> i32>,
    pub send_pre_block: Option<unsafe extern "C" fn(
        sk: *mut sock,
        data: *mut vmci_transport_send_notify_data,
    ) -> i32>,
    pub send_pre_enqueue: Option<unsafe extern "C" fn(
        sk: *mut sock,
        data: *mut vmci_transport_send_notify_data,
    ) -> i32>,
    pub send_post_enqueue: Option<unsafe extern "C" fn(
        sk: *mut sock,
        written: isize,
        data: *mut vmci_transport_send_notify_data,
    ) -> i32>,
    pub process_request: Option<unsafe extern "C" fn(sk: *mut sock)>,
    pub process_negotiate: Option<unsafe extern "C" fn(sk: *mut sock)>,
}

extern "C" {
    pub static vmci_transport_notify_pkt_ops: vmci_transport_notify_ops;
    pub static vmci_transport_notify_pkt_q_state_ops: vmci_transport_notify_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
