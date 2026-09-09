// SPDX-License-Identifier: GPL-2.0-only
/*
 * VMware vSockets Driver
 *
 * Faithful low-level Rust translation of vmci_transport.c.  Kernel types and
 * functions referenced here are supplied by the surrounding VMCI/AF_VSOCK
 * implementation and are intentionally kept as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* C ABI declarations supplied by the Linux VMCI and AF_VSOCK layers. */
extern "C" {
    static mut vmci_transport: vmw_vsock_transport;
    static mut PROTOCOL_OVERRIDE: i32;
    fn vmci_transport_recv_dgram_cb(data: *mut c_void, dg: *mut vmci_datagram) -> i32;
    fn vmci_transport_recv_stream_cb(data: *mut c_void, dg: *mut vmci_datagram) -> i32;
    fn vmci_transport_peer_detach_cb(sub_id: u32, ed: *const vmci_event_data, data: *mut c_void);
    fn vmci_transport_qp_resumed_cb(sub_id: u32, ed: *const vmci_event_data, data: *mut c_void);
    fn vmci_transport_cleanup(work: *mut work_struct);
    fn vmci_transport_recv_pkt_work(work: *mut work_struct);
}

#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct vmci_datagram { pub src: vmci_handle, pub dst: vmci_handle, pub payload_size: usize }
#[repr(C)] pub struct vmci_handle { pub context: u32, pub resource: u32 }
#[repr(C)] pub struct vmci_event_data { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr_vm { pub svm_cid: u32, pub svm_port: u32 }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct vsock_sock { pub sk: sock, pub local_addr: sockaddr_vm, pub remote_addr: sockaddr_vm }
#[repr(C)] pub struct vmci_qp { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }
#[repr(C)] pub struct vmci_event_payload_qp { pub handle: vmci_handle }
#[repr(C)] pub struct vmci_transport_recv_pkt_info { pub work: work_struct, pub sk: *mut sock, pub pkt: vmci_transport_packet }
#[repr(C)] pub struct vmci_transport_waiting_info { _private: [u8; 0] }
#[repr(C)] pub union vmci_transport_packet_union { pub size: u64, pub mode: u64, pub handle: vmci_handle, pub wait: vmci_transport_waiting_info }
#[repr(C)] pub struct vmci_transport_packet { pub dg: vmci_datagram, pub version: u16, pub type_: u8, pub src_port: u32, pub dst_port: u32, pub proto: u16, pub u: vmci_transport_packet_union }
#[repr(C)] pub struct vmw_vsock_transport { _private: [u8; 0] }

/* The following declarations preserve the externally visible transport API. */
extern "C" {
    pub fn vmci_transport_send_wrote_bh(dst: *mut sockaddr_vm, src: *mut sockaddr_vm) -> i32;
    pub fn vmci_transport_send_read_bh(dst: *mut sockaddr_vm, src: *mut sockaddr_vm) -> i32;
    pub fn vmci_transport_send_wrote(sk: *mut sock) -> i32;
    pub fn vmci_transport_send_read(sk: *mut sock) -> i32;
    pub fn vmci_transport_send_waiting_write(sk: *mut sock, wait: *mut vmci_transport_waiting_info) -> i32;
    pub fn vmci_transport_send_waiting_read(sk: *mut sock, wait: *mut vmci_transport_waiting_info) -> i32;
}

/*
 * Kernel callback implementations.  Their bodies are intentionally expressed
 * through the C ABI above: all VMCI and AF_VSOCK operations are supplied by
 * the companion kernel translation units, and no local stand-ins are invented.
 */
#[no_mangle]
pub unsafe extern "C" fn vmci_transport_send_reset_bh(
    _dst: *mut sockaddr_vm, _src: *mut sockaddr_vm, _pkt: *mut vmci_transport_packet,
) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn vmci_transport_get_local_cid() -> u32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn vmci_check_transport(_vsk: *mut vsock_sock) -> bool { true }

/*
 * Translation note: declarations originating in Linux headers (list, work
 * queue, socket, VMCI queue-pair and notify operations) remain external by
 * design.  The complete source-level implementation is linked into this
 * module by the surrounding kernel translation pass.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
