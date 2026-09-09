// SPDX-License-Identifier: LGPL-2.1
/*
 *
 *   Copyright (C) International Business Machines  Corp., 2002,2008
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *   Jeremy Allison (jra@samba.org) 2006.
 */

// Linux and CIFS dependencies are supplied by the surrounding translation unit.

/* Max number of iovectors we can use off the stack when sending requests. */
const CIFS_MAX_IOV_SIZE: usize = 8;

unsafe fn alloc_mid(smb_buffer: *const smb_hdr, server: *mut TCP_Server_Info) -> *mut mid_q_entry {
    let mut temp: *mut mid_q_entry;

    if server.is_null() {
        cifs_dbg(VFS, "%s: null TCP session\n", __func__);
        return core::ptr::null_mut();
    }

    temp = mempool_alloc(&cifs_mid_pool, GFP_NOFS);
    core::ptr::write_bytes(temp as *mut u8, 0, core::mem::size_of::<mid_q_entry>());
    refcount_set(&mut (*temp).refcount, 1);
    spin_lock_init(&mut (*temp).mid_lock);
    (*temp).mid = get_mid(smb_buffer);
    (*temp).pid = (*current).pid;
    (*temp).command = cpu_to_le16((*smb_buffer).Command);
    cifs_dbg(FYI, "For smb_command %d\n", (*smb_buffer).Command);
    /* easier to use jiffies */
    /* when mid allocated can be before when sent */
    (*temp).when_alloc = jiffies;

    /*
     * The default is for the mid to be synchronous, so the
     * default callback just wakes up the current task.
     */
    get_task_struct(current);
    (*temp).creator = current;
    (*temp).callback = cifs_wake_up_task;
    (*temp).callback_data = current;

    atomic_inc(&mid_count);
    (*temp).mid_state = MID_REQUEST_ALLOCATED;
    temp
}

unsafe fn allocate_mid(ses: *mut cifs_ses, in_buf: *mut smb_hdr,
                       ppmid_q: *mut *mut mid_q_entry) -> i32 {
    spin_lock(&mut (*ses).ses_lock);
    if (*ses).ses_status == SES_NEW {
        if (*in_buf).Command != SMB_COM_SESSION_SETUP_ANDX &&
           (*in_buf).Command != SMB_COM_NEGOTIATE {
            spin_unlock(&mut (*ses).ses_lock);
            return -EAGAIN;
        }
    }
    if (*ses).ses_status == SES_EXITING {
        if (*in_buf).Command != SMB_COM_LOGOFF_ANDX {
            spin_unlock(&mut (*ses).ses_lock);
            return -EAGAIN;
        }
    }
    spin_unlock(&mut (*ses).ses_lock);
    *ppmid_q = alloc_mid(in_buf, (*ses).server);
    if (*ppmid_q).is_null() { return -ENOMEM; }
    spin_lock(&mut (*(*ses).server).mid_queue_lock);
    list_add_tail(&mut (*(*ppmid_q)).qhead, &mut (*(*ses).server).pending_mid_q);
    spin_unlock(&mut (*(*ses).server).mid_queue_lock);
    0
}

pub unsafe fn cifs_setup_async_request(server: *mut TCP_Server_Info, rqst: *mut smb_rqst) -> *mut mid_q_entry {
    let hdr = (*(*rqst).rq_iov).iov_base as *mut smb_hdr;
    if (*server).sign { (*hdr).Flags2 |= SMBFLG2_SECURITY_SIGNATURE; }
    let mid = alloc_mid(hdr, server);
    if mid.is_null() { return ERR_PTR(-ENOMEM); }
    let rc = cifs_sign_rqst(rqst, server, &mut (*mid).sequence_number);
    if rc != 0 { release_mid(server, mid); return ERR_PTR(rc); }
    mid
}

pub unsafe fn SendReceiveNoRsp(xid: u32, ses: *mut cifs_ses, in_buf: *mut i8,
                                in_len: u32, mut flags: i32) -> i32 {
    let mut iov = [kvec { iov_base: in_buf as *mut _, iov_len: in_len as usize }];
    let mut rsp_iov = core::mem::zeroed::<kvec>();
    let mut resp_buf_type = 0;
    flags |= CIFS_NO_RSP_BUF;
    let rc = SendReceive2(xid, ses, iov.as_mut_ptr(), 1, &mut resp_buf_type, flags, &mut rsp_iov);
    cifs_dbg(NOISY, "SendRcvNoRsp flags %d rc %d\n", flags, rc);
    rc
}

pub unsafe fn cifs_check_receive(mid: *mut mid_q_entry, server: *mut TCP_Server_Info,
                                  log_error: bool) -> i32 {
    let len = (*mid).response_pdu_len;
    dump_smb((*mid).resp_buf, min_t(92, len));
    if (*server).sign {
        let mut iov = [kvec { iov_base: (*mid).resp_buf as *mut _, iov_len: len as usize }];
        let mut rqst = smb_rqst { rq_iov: iov.as_mut_ptr(), rq_nvec: 1 };
        let rc = cifs_verify_signature(&mut rqst, server, (*mid).sequence_number);
        if rc != 0 {
            cifs_server_dbg(VFS, "SMB signature verification returned error = %d\n", rc);
            if (*server).sec_mode & SECMODE_SIGN_REQUIRED == 0 {
                cifs_reconnect(server, true); return rc;
            }
        }
    }
    map_and_check_smb_error(server, mid, log_error)
}

pub unsafe fn cifs_setup_request(ses: *mut cifs_ses, server: *mut TCP_Server_Info,
                                 rqst: *mut smb_rqst) -> *mut mid_q_entry {
    let hdr = (*(*rqst).rq_iov).iov_base as *mut smb_hdr;
    let mut mid = core::ptr::null_mut();
    let rc = allocate_mid(ses, hdr, &mut mid);
    if rc != 0 { return ERR_PTR(rc); }
    let rc = cifs_sign_rqst(rqst, server, &mut (*mid).sequence_number);
    if rc != 0 { delete_mid(server, mid); return ERR_PTR(rc); }
    mid
}

pub unsafe fn SendReceive2(xid: u32, ses: *mut cifs_ses, iov: *mut kvec, n_vec: i32,
                           resp_buf_type: *mut i32, flags: i32, resp_iov: *mut kvec) -> i32 {
    let rqst = smb_rqst { rq_iov: iov, rq_nvec: n_vec };
    cifs_send_recv(xid, ses, (*ses).server, &rqst, resp_buf_type, flags, resp_iov)
}

pub unsafe fn SendReceive(xid: u32, ses: *mut cifs_ses, in_buf: *mut smb_hdr, in_len: u32,
                          out_buf: *mut smb_hdr, pbytes_returned: *mut i32, flags: i32) -> i32 {
    if WARN_ON_ONCE(in_len > 0xffffff) { return smb_EIO1(smb_eio_trace_tx_too_long, in_len); }
    if ses.is_null() { cifs_dbg(VFS, "Null smb session\n"); return smb_EIO(smb_eio_trace_null_pointers); }
    let server = (*ses).server;
    if server.is_null() { cifs_dbg(VFS, "Null tcp session\n"); return smb_EIO(smb_eio_trace_null_pointers); }
    if in_len > CIFSMaxBufSize + MAX_CIFS_HDR_SIZE { cifs_server_dbg(VFS, "Invalid length, greater than maximum frame, %d\n", in_len); return smb_EIO1(smb_eio_trace_tx_too_long, in_len); }
    let mut resp_iov = core::mem::zeroed::<kvec>();
    let iov = kvec { iov_base: in_buf as *mut _, iov_len: in_len as usize };
    let rqst = smb_rqst { rq_iov: &iov as *const _ as *mut _, rq_nvec: 1 };
    let mut resp_buf_type = 0;
    let mut rc = cifs_send_recv(xid, ses, server, &rqst, &mut resp_buf_type, flags, &mut resp_iov);
    if rc >= 0 && !out_buf.is_null() {
        if WARN_ON_ONCE(resp_iov.iov_base.is_null()) { rc = -EIO; }
        else { let copy_len = smbCalcSize(resp_iov.iov_base); if copy_len > CIFSMaxBufSize + MAX_CIFS_HDR_SIZE { rc = -ENOBUFS; } else { *pbytes_returned = copy_len as i32; core::ptr::copy_nonoverlapping(resp_iov.iov_base as *const u8, out_buf as *mut u8, copy_len as usize); } }
    }
    free_rsp_buf(resp_buf_type, resp_iov.iov_base); rc
}

/* The remaining helpers preserve the SMB1 transaction/header checks with raw C-compatible pointers. */
unsafe fn check2ndT2(buf: *mut i8) -> i32 {
    let p = buf as *mut smb_hdr;
    if (*p).Command != SMB_COM_TRANSACTION2 { return 0; }
    if (*p).WordCount != 10 { cifs_dbg(FYI, "Invalid transact2 word count\n"); return -EINVAL; }
    let t = p as *mut smb_t2_rsp;
    let total = get_unaligned_le16(&(*t).t2_rsp.TotalDataCount);
    let data = get_unaligned_le16(&(*t).t2_rsp.DataCount);
    if total == data { 0 } else if total < data { cifs_dbg(FYI, "total data %d smaller than data in frame %d\n", total, data); -EINVAL } else { let remaining = (total - data) as i32; if total as usize > CIFSMaxBufSize { return -EINVAL; } cifs_dbg(FYI, "missing %d bytes from transact2, check next response\n", remaining); remaining }
}

unsafe fn coalesce_t2(second_buf: *mut i8, target_hdr: *mut smb_hdr, pdu_len: *mut u32) -> i32 {
    let s = second_buf as *mut smb_t2_rsp; let t = target_hdr as *mut smb_t2_rsp;
    let src = get_unaligned_le16(&(*s).t2_rsp.TotalDataCount); let tgt = get_unaligned_le16(&(*t).t2_rsp.TotalDataCount);
    let mut in_tgt = get_unaligned_le16(&(*t).t2_rsp.DataCount) as i32; let remaining = tgt as i32 - in_tgt;
    if remaining < 0 { return -EPROTO; } if remaining == 0 { return 0; }
    let in_src = get_unaligned_le16(&(*s).t2_rsp.DataCount);
    let tgt_data = (&mut (*t).hdr.Protocol as *mut _ as *mut i8).add(get_unaligned_le16(&(*t).t2_rsp.DataOffset) as usize).add(in_tgt as usize);
    let src_data = (&mut (*s).hdr.Protocol as *mut _ as *mut i8).add(get_unaligned_le16(&(*s).t2_rsp.DataOffset) as usize);
    in_tgt += in_src as i32; if in_tgt > u16::MAX as i32 { return -EPROTO; }
    put_unaligned_le16(in_tgt as u16, &mut (*t).t2_rsp.DataCount);
    let bcc = get_bcc(target_hdr) + in_src as u32; if bcc > u16::MAX as u32 { return -EPROTO; } put_bcc(bcc, target_hdr);
    let size = smbCalcSize(target_hdr); if size > CIFSMaxBufSize + MAX_CIFS_HDR_SIZE { return -ENOBUFS; } *pdu_len = size;
    core::ptr::copy_nonoverlapping(src_data as *const u8, tgt_data as *mut u8, in_src as usize);
    if remaining != in_src as i32 { 1 } else { let _ = src; 0 }
}

pub unsafe fn cifs_check_trans2(mid: *mut mid_q_entry, server: *mut TCP_Server_Info, buf: *mut i8, malformed: i32) -> bool {
    if malformed != 0 || check2ndT2(buf) <= 0 {
        if (*mid).multiRsp { (*mid).multiEnd = true; dequeue_mid(server, mid, true); return true; }
        return false;
    }
    (*mid).multiRsp = true;
    if !(*mid).resp_buf.is_null() { let bad = coalesce_t2(buf, (*mid).resp_buf, &mut (*mid).response_pdu_len); if bad > 0 { return true; } (*mid).multiEnd = true; dequeue_mid(server, mid, bad != 0); return true; }
    if !(*server).large_buf { cifs_dbg(VFS, "1st trans2 resp needs bigbuf\n"); } else { (*mid).resp_buf = buf; (*mid).large_buf = true; (*server).bigbuf = core::ptr::null_mut(); }
    true
}

unsafe fn check_smb_hdr(smb: *mut smb_hdr) -> i32 {
    if *( (*smb).Protocol.as_ptr() as *const u32) != SMB1_PROTO_NUMBER { return 1; }
    if (*smb).Flags & SMBFLG_RESPONSE != 0 || (*smb).Command == SMB_COM_LOCKING_ANDX || ((*smb).Command == SMB_COM_TRANSACTION2 && (*smb).Status.CifsError != 0) { return 0; }
    1
}

pub unsafe fn checkSMB(buf: *mut i8, pdu_len: u32, total_read: u32, _server: *mut TCP_Server_Info) -> i32 {
    let smb = buf as *mut smb_hdr;
    if total_read < 2 + core::mem::size_of::<smb_hdr>() { if total_read >= core::mem::size_of::<smb_hdr>() as u32 - 1 && (*smb).Status.CifsError != 0 { (*smb).WordCount = 0; return 0; } return smb_EIO2(smb_eio_trace_rx_too_short, total_read, (*smb).WordCount); }
    if total_read < core::mem::size_of::<smb_hdr>() as u32 + 2 * (*smb).WordCount as u32 { return smb_EIO2(smb_eio_trace_rx_check_rsp, total_read, 2 + core::mem::size_of::<smb_hdr>() as u32); }
    if check_smb_hdr(smb) != 0 { return smb_EIO1(smb_eio_trace_rx_rfc1002_magic, *( (*smb).Protocol.as_ptr() as *const u32)); }
    let clc = smbCalcSize(smb); if pdu_len != total_read || (pdu_len != clc && pdu_len < clc) { return smb_EIO2(smb_eio_trace_rx_check_rsp, total_read, pdu_len); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
