// SPDX-License-Identifier: LGPL-2.1
/*
 *
 *   Encryption and hashing operations relating to NTLM, NTLMv2.  See MS-NLMP
 *   for more detailed information
 *
 *   Copyright (C) International Business Machines  Corp., 2005,2013
 *   Author(s): Steve French (sfrench@us.ibm.com)
 */

// Linux crypto and CIFS declarations are supplied by the surrounding build.

#[allow(non_camel_case_types)]
type __u32 = u32;

unsafe extern "C" {
    static fips_enabled: bool;

    fn md5_init(ctx: *mut md5_ctx);
    fn md5_update(ctx: *mut md5_ctx, data: *const u8, len: usize);
    fn __cifs_calc_signature(
        rqst: *mut smb_rqst,
        server: *mut TCP_Server_Info,
        signature: *mut i8,
        ctx: *mut cifs_calc_sig_ctx,
    ) -> i32;
    fn spin_lock(lock: *mut core::ffi::c_void);
    fn spin_unlock(lock: *mut core::ffi::c_void);
    fn cifs_server_lock(server: *mut TCP_Server_Info);
    fn cifs_server_unlock(server: *mut TCP_Server_Info);
    fn cifs_dbg(level: i32, format: *const i8, ...);
    fn crypto_memneq(a: *const u8, b: *const u8, len: usize) -> i32;
    fn memzero_explicit(ptr: *mut core::ffi::c_void, len: usize);
}

#[repr(C)]
struct md5_ctx { _private: [u8; 0] }

// These structures and constants are declared by cifsproto.h and smb1proto.h.
// Their field layout is intentionally used directly below.
#[repr(C)] struct smb_rqst { rq_iov: *mut iovec }
#[repr(C)] struct iovec { iov_base: *mut core::ffi::c_void }
#[repr(C)] struct TCP_Server_Info { session_key: session_key, srv_lock: core::ffi::c_void, tcpStatus: i32, session_estab: bool, sequence_number: __u32 }
#[repr(C)] struct session_key { response: *mut u8, len: usize }
#[repr(C)] struct cifs_calc_sig_ctx { md5: *mut md5_ctx }
#[repr(C)] struct smb_hdr { Flags2: u16, Signature: signature, Command: u8 }
#[repr(C)] union signature { SecuritySignature: [u8; 8], Sequence: sequence }
#[repr(C)] struct sequence { SequenceNumber: __u32, Reserved: __u32 }
#[repr(C)] struct smb_com_lock_req { hdr: smb_hdr, LockType: u8 }

unsafe fn cifs_calc_signature(rqst: *mut smb_rqst, server: *mut TCP_Server_Info, signature: *mut i8) -> i32 {
    let mut ctx = md5_ctx { _private: [] };
    if (*rqst).rq_iov.is_null() || signature.is_null() || server.is_null() { return -EINVAL; }
    if fips_enabled {
        cifs_dbg(VFS, b"MD5 signature support is disabled due to FIPS\n\0".as_ptr() as *const i8);
        return -EOPNOTSUPP;
    }
    md5_init(&mut ctx);
    md5_update(&mut ctx, (*server).session_key.response, (*server).session_key.len);
    __cifs_calc_signature(rqst, server, signature, &mut cifs_calc_sig_ctx { md5: &mut ctx })
}

pub unsafe fn cifs_sign_rqst(rqst: *mut smb_rqst, server: *mut TCP_Server_Info, pexpected_response_sequence_number: *mut __u32) -> i32 {
    let mut rc: i32 = 0;
    let mut smb_signature = [0i8; 20];
    let cifs_pdu = (*(*rqst).rq_iov).iov_base as *mut smb_hdr;
    if cifs_pdu.is_null() || server.is_null() { return -EINVAL; }
    spin_lock(&mut (*server).srv_lock as *mut _ as *mut core::ffi::c_void);
    if ((*cifs_pdu).Flags2 & SMBFLG2_SECURITY_SIGNATURE) == 0 || (*server).tcpStatus == CifsNeedNegotiate {
        spin_unlock(&mut (*server).srv_lock as *mut _ as *mut core::ffi::c_void); return rc;
    }
    spin_unlock(&mut (*server).srv_lock as *mut _ as *mut core::ffi::c_void);
    if !(*server).session_estab {
        (*cifs_pdu).Signature.SecuritySignature = *b"BSRSPYL\0"; return rc;
    }
    (*cifs_pdu).Signature.Sequence.SequenceNumber = (*server).sequence_number.to_le();
    (*cifs_pdu).Signature.Sequence.Reserved = 0;
    (*server).sequence_number = (*server).sequence_number.wrapping_add(1);
    *pexpected_response_sequence_number = (*server).sequence_number;
    (*server).sequence_number = (*server).sequence_number.wrapping_add(1);
    rc = cifs_calc_signature(rqst, server, smb_signature.as_mut_ptr());
    if rc != 0 { (*cifs_pdu).Signature.SecuritySignature = [0; 8]; }
    else { (*cifs_pdu).Signature.SecuritySignature.copy_from_slice(&smb_signature[..8].iter().map(|v| *v as u8).collect::<Vec<_>>()); }
    memzero_explicit(smb_signature.as_mut_ptr() as *mut _, core::mem::size_of_val(&smb_signature)); rc
}

pub unsafe fn cifs_verify_signature(rqst: *mut smb_rqst, server: *mut TCP_Server_Info, expected_sequence_number: __u32) -> u32 {
    let mut server_response_sig = [0u8; 8]; let mut what_we_think_sig_should_be = [0i8; 20];
    let cifs_pdu = (*(*rqst).rq_iov).iov_base as *mut smb_hdr;
    if cifs_pdu.is_null() || server.is_null() { return (-EINVAL) as u32; }
    if !(*server).session_estab { return 0; }
    if (*cifs_pdu).Command == SMB_COM_LOCKING_ANDX && ((*((cifs_pdu as *mut smb_com_lock_req))).LockType & LOCKING_ANDX_OPLOCK_RELEASE) != 0 { return 0; }
    server_response_sig.copy_from_slice(&(*cifs_pdu).Signature.SecuritySignature);
    (*cifs_pdu).Signature.Sequence.SequenceNumber = expected_sequence_number.to_le(); (*cifs_pdu).Signature.Sequence.Reserved = 0;
    cifs_server_lock(server); let rc = cifs_calc_signature(rqst, server, what_we_think_sig_should_be.as_mut_ptr()); cifs_server_unlock(server);
    let result = if rc == 0 && crypto_memneq(server_response_sig.as_ptr(), what_we_think_sig_should_be.as_ptr() as *const u8, 8) != 0 { (-EACCES) as u32 } else { rc as u32 };
    memzero_explicit(what_we_think_sig_should_be.as_mut_ptr() as *mut _, core::mem::size_of_val(&what_we_think_sig_should_be)); result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
