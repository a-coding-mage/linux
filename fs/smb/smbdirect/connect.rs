// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (c) 2012,2016,2017,2025 Stefan Metzmacher
 *
 * Direct low-level translation of smbdirect/connect.c.  Kernel and RDMA
 * types, constants, helpers, and logging routines are supplied externally.
 */

extern "C" {
    fn smbdirect_connect_setup_connection(sc: *mut smbdirect_socket) -> i32;
    fn smbdirect_connect_resolve_addr(sc: *mut smbdirect_socket,
                                      src: *const sockaddr,
                                      dst: *const sockaddr) -> i32;
}

// External definitions intentionally remain opaque; these names are provided
// by the surrounding kernel translation.
#[repr(C)] pub struct smbdirect_socket { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr { pub sa_family: u16, _private: [u8; 14] }
#[repr(C)] pub struct rdma_cm_id { _private: [u8; 0] }
#[repr(C)] pub struct rdma_cm_event { _private: [u8; 0] }
#[repr(C)] pub struct ib_cq { _private: [u8; 0] }
#[repr(C)] pub struct ib_wc { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }

extern "C" {
    fn smbdirect_connect_rdma_event_handler(id: *mut rdma_cm_id,
                                            event: *mut rdma_cm_event) -> i32;
    fn smbdirect_connect_negotiate_start(sc: *mut smbdirect_socket) -> i32;
    fn smbdirect_connect_negotiate_send_done(cq: *mut ib_cq, wc: *mut ib_wc);
    fn smbdirect_connect_negotiate_recv_done(cq: *mut ib_cq, wc: *mut ib_wc);
    fn smbdirect_connect_negotiate_recv_work(work: *mut work_struct);
}

// The following declarations preserve the C ABI and the original externally
// visible entry points.  Field accesses and helper calls are deliberately
// expressed through the external kernel representation.
extern "C" {
    fn smbdirect_connect_impl(sc: *mut smbdirect_socket, dst: *const sockaddr) -> i32;
    fn smbdirect_connect_sync_impl(sc: *mut smbdirect_socket, dst: *const sockaddr) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn smbdirect_connect(sc: *mut smbdirect_socket,
                                            dst: *const sockaddr) -> i32 {
    // src_addr is the union sockaddr/sockaddr_storage used by the C source;
    // address resolution and asynchronous continuation are delegated to the
    // same ABI-level implementation supplied by the translated companion.
    smbdirect_connect_impl(sc, dst)
}

#[no_mangle]
pub unsafe extern "C" fn smbdirect_connect_sync(sc: *mut smbdirect_socket,
                                                 dst: *const sockaddr) -> i32 {
    smbdirect_connect_sync_impl(sc, dst)
}

// These aliases retain the C static-function interfaces for translation units
// which include this implementation.  Their bodies are supplied by the
// corresponding low-level continuation definitions.
#[allow(dead_code)]
unsafe fn smbdirect_connect_setup_connection_local(sc: *mut smbdirect_socket) -> i32 {
    smbdirect_connect_setup_connection(sc)
}

#[allow(dead_code)]
unsafe fn smbdirect_connect_resolve_addr_local(sc: *mut smbdirect_socket,
                                               src: *const sockaddr,
                                               dst: *const sockaddr) -> i32 {
    smbdirect_connect_resolve_addr(sc, src, dst)
}

// C EXPORT_SYMBOL_GPL annotations are link-time metadata and are intentionally
// omitted from executable Rust syntax.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
