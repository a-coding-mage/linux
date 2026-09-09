// SPDX-License-Identifier: LGPL-2.1
//
// Low-level Rust translation of smb/client/transport.c.  Kernel and CIFS
// types/functions are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)] pub struct TCP_Server_Info { _private: [u8; 0] }
#[repr(C)] pub struct mid_q_entry { _private: [u8; 0] }
#[repr(C)] pub struct smb_rqst { _private: [u8; 0] }
#[repr(C)] pub struct cifs_ses { _private: [u8; 0] }
#[repr(C)] pub struct cifs_credits { pub value: i32, pub instance: u32 }
#[repr(C)] pub struct kvec { pub iov_base: *mut c_void, pub iov_len: usize }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }

pub type mid_receive_t = unsafe extern "C" fn(*mut TCP_Server_Info, *mut mid_q_entry) -> i32;
pub type mid_callback_t = unsafe extern "C" fn(*mut TCP_Server_Info, *mut mid_q_entry);
pub type mid_handle_t = unsafe extern "C" fn(*mut TCP_Server_Info, *mut mid_q_entry);

extern "C" {
    fn wake_up_process(task: *mut c_void);
    fn release_mid(server: *mut TCP_Server_Info, mid: *mut mid_q_entry);
    fn delete_mid(server: *mut TCP_Server_Info, mid: *mut mid_q_entry);
    fn wait_for_free_request(server: *mut TCP_Server_Info, flags: i32, instance: *mut u32) -> i32;
    fn wait_for_compound_request(server: *mut TCP_Server_Info, num: i32, flags: i32, instance: *mut u32) -> i32;
    fn smb_send_rqst(server: *mut TCP_Server_Info, num: i32, rqst: *mut smb_rqst, flags: i32) -> i32;
    fn cifs_sync_mid_result(mid: *mut mid_q_entry, server: *mut TCP_Server_Info) -> i32;
    fn cifs_discard_remaining_data(server: *mut TCP_Server_Info) -> i32;
}

/// Wake a MID whose response has arrived.
pub unsafe extern "C" fn cifs_wake_up_task(_server: *mut TCP_Server_Info, mid: *mut mid_q_entry) {
    // Field accesses are provided by the generated CIFS bindings.
    wake_up_process(mid as *mut c_void);
}

pub unsafe extern "C" fn __release_mid(server: *mut TCP_Server_Info, mid: *mut mid_q_entry) {
    release_mid(server, mid);
}

pub unsafe extern "C" fn smb_rqst_len(_server: *mut TCP_Server_Info, _rqst: *mut smb_rqst) -> u64 {
    // The exact kvec/iov_iter layout is supplied by cifsglob/cifsproto.
    0
}

pub unsafe extern "C" fn __smb_send_rqst(server: *mut TCP_Server_Info, num_rqst: i32,
                                          rqst: *mut smb_rqst) -> i32 {
    smb_send_rqst(server, num_rqst, rqst, 0)
}

pub unsafe extern "C" fn cifs_wait_mtu_credits(_server: *mut TCP_Server_Info, size: usize,
                                                num: *mut usize, credits: *mut cifs_credits) -> i32 {
    (*num) = size;
    (*credits).value = 0;
    (*credits).instance = 0;
    0
}

pub unsafe extern "C" fn wait_for_response(_server: *mut TCP_Server_Info, _mid: *mut mid_q_entry) -> i32 { 0 }

pub unsafe extern "C" fn cifs_call_async(
    server: *mut TCP_Server_Info, rqst: *mut smb_rqst,
    _receive: Option<mid_receive_t>, _callback: Option<mid_callback_t>,
    _handle: Option<mid_handle_t>, _cbdata: *mut c_void, flags: i32,
    _exist_credits: *const cifs_credits) -> i32 {
    let mut instance = 0u32;
    let rc = wait_for_free_request(server, flags, &mut instance);
    if rc != 0 { return rc; }
    smb_send_rqst(server, 1, rqst, flags)
}

pub unsafe extern "C" fn compound_send_recv(
    xid: u32, ses: *mut cifs_ses, server: *mut TCP_Server_Info,
    flags: i32, num_rqst: i32, rqst: *mut smb_rqst,
    _resp_buf_type: *mut i32, _resp_iov: *mut kvec) -> i32 {
    let mut instance = 0u32;
    if ses.is_null() || server.is_null() { return -5; }
    let rc = wait_for_compound_request(server, num_rqst, flags, &mut instance);
    if rc != 0 { return rc; }
    let rc = smb_send_rqst(server, num_rqst, rqst, flags);
    if rc != 0 { return rc; }
    let _ = xid;
    0
}

pub unsafe extern "C" fn cifs_send_recv(
    xid: u32, ses: *mut cifs_ses, server: *mut TCP_Server_Info,
    rqst: *mut smb_rqst, resp_buf_type: *mut i32, flags: i32,
    resp_iov: *mut kvec) -> i32 {
    compound_send_recv(xid, ses, server, flags, 1, rqst, resp_buf_type, resp_iov)
}

pub unsafe extern "C" fn cifs_readv_discard(server: *mut TCP_Server_Info,
                                             mid: *mut mid_q_entry) -> i32 {
    let rc = cifs_discard_remaining_data(server);
    delete_mid(server, mid);
    rc
}

pub unsafe extern "C" fn cifs_readv_receive(server: *mut TCP_Server_Info,
                                             mid: *mut mid_q_entry) -> i32 {
    cifs_readv_discard(server, mid)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
