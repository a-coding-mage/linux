/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * VMware vSockets Driver
 *
 * Copyright (C) 2013 VMware, Inc. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, matching the original header includes.

/* If the packet format changes in a release then this should change too. */
pub const VMCI_TRANSPORT_PACKET_VERSION: u32 = 1;

/* The resource ID on which control packets are sent. */
pub const VMCI_TRANSPORT_PACKET_RID: u32 = 1;

/* The resource ID on which control packets are sent to the hypervisor. */
pub const VMCI_TRANSPORT_HYPERVISOR_PACKET_RID: u32 = 15;

pub const VSOCK_PROTO_INVALID: u32 = 0;
pub const VSOCK_PROTO_PKT_ON_NOTIFY: u32 = 1 << 0;
pub const VSOCK_PROTO_ALL_SUPPORTED: u32 = VSOCK_PROTO_PKT_ON_NOTIFY;

#[macro_export]
macro_rules! vmci_trans {
    ($vsk:expr) => {
        unsafe { &mut *((*$vsk).trans as *mut $crate::vmci_transport) }
    };
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum vmci_transport_packet_type {
    VMCI_TRANSPORT_PACKET_TYPE_INVALID = 0,
    VMCI_TRANSPORT_PACKET_TYPE_REQUEST,
    VMCI_TRANSPORT_PACKET_TYPE_NEGOTIATE,
    VMCI_TRANSPORT_PACKET_TYPE_OFFER,
    VMCI_TRANSPORT_PACKET_TYPE_ATTACH,
    VMCI_TRANSPORT_PACKET_TYPE_WROTE,
    VMCI_TRANSPORT_PACKET_TYPE_READ,
    VMCI_TRANSPORT_PACKET_TYPE_RST,
    VMCI_TRANSPORT_PACKET_TYPE_SHUTDOWN,
    VMCI_TRANSPORT_PACKET_TYPE_WAITING_WRITE,
    VMCI_TRANSPORT_PACKET_TYPE_WAITING_READ,
    VMCI_TRANSPORT_PACKET_TYPE_REQUEST2,
    VMCI_TRANSPORT_PACKET_TYPE_NEGOTIATE2,
    VMCI_TRANSPORT_PACKET_TYPE_MAX,
}

#[repr(C)]
pub struct vmci_transport_waiting_info {
    pub generation: u64,
    pub offset: u64,
}

/* Control packet type for STREAM sockets.  DGRAMs have no control packets nor
 * special packet header for data packets, they are just raw VMCI DGRAM
 * messages.  For STREAMs, control packets are sent over the control channel
 * while data is written and read directly from queue pairs with no packet
 * format.
 */
#[repr(C)]
pub struct vmci_transport_packet {
    pub dg: vmci_datagram,
    pub version: u8,
    pub type_: u8,
    pub proto: u16,
    pub src_port: u32,
    pub dst_port: u32,
    pub _reserved2: u32,
    pub u: vmci_transport_packet__u,
}

#[repr(C)]
pub union vmci_transport_packet__u {
    pub size: u64,
    pub mode: u64,
    pub handle: vmci_handle,
    pub wait: vmci_transport_waiting_info,
}

#[repr(C)]
pub struct vmci_transport_notify_pkt {
    pub write_notify_window: u64,
    pub write_notify_min_window: u64,
    pub peer_waiting_read: bool,
    pub peer_waiting_write: bool,
    pub peer_waiting_write_detected: bool,
    pub sent_waiting_read: bool,
    pub sent_waiting_write: bool,
    pub peer_waiting_read_info: vmci_transport_waiting_info,
    pub peer_waiting_write_info: vmci_transport_waiting_info,
    pub produce_q_generation: u64,
    pub consume_q_generation: u64,
}

#[repr(C)]
pub struct vmci_transport_notify_pkt_q_state {
    pub write_notify_window: u64,
    pub write_notify_min_window: u64,
    pub peer_waiting_write: bool,
    pub peer_waiting_write_detected: bool,
}

#[repr(C)]
pub union vmci_transport_notify {
    pub pkt: vmci_transport_notify_pkt,
    pub pkt_q_state: vmci_transport_notify_pkt_q_state,
}

/* Our transport-specific data. */
#[repr(C)]
pub struct vmci_transport {
    /* For DGRAMs. */
    pub dg_handle: vmci_handle,
    /* For STREAMs. */
    pub qp_handle: vmci_handle,
    pub qpair: *mut vmci_qp,
    pub produce_size: u64,
    pub consume_size: u64,
    pub detach_sub_id: u32,
    pub notify: vmci_transport_notify,
    pub notify_ops: *const vmci_transport_notify_ops,
    pub elem: list_head,
    pub sk: *mut sock,
    pub lock: spinlock_t, /* protects sk. */
}

extern "C" {
    pub fn vmci_transport_send_wrote_bh(dst: *mut sockaddr_vm, src: *mut sockaddr_vm) -> i32;
    pub fn vmci_transport_send_read_bh(dst: *mut sockaddr_vm, src: *mut sockaddr_vm) -> i32;
    pub fn vmci_transport_send_wrote(sk: *mut sock) -> i32;
    pub fn vmci_transport_send_read(sk: *mut sock) -> i32;
    pub fn vmci_transport_send_waiting_write(
        sk: *mut sock,
        wait: *mut vmci_transport_waiting_info,
    ) -> i32;
    pub fn vmci_transport_send_waiting_read(
        sk: *mut sock,
        wait: *mut vmci_transport_waiting_info,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
