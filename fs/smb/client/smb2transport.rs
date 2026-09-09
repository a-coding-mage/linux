// SPDX-License-Identifier: LGPL-2.1
// Direct low-level translation of smb2transport.c. Kernel and CIFS types,
// constants, globals, and helper functions are supplied by other units.

use core::ffi::c_void;

unsafe fn smb3_get_sign_key(ses_id: u64, server: *mut TCP_Server_Info, key: *mut u8) -> i32 {
    let mut chan: *mut cifs_chan;
    let pserver = if SERVER_IS_CHAN(server) { (*server).primary_server } else { server };
    let mut ses: *mut cifs_ses = core::ptr::null_mut();
    let mut rc = 0;
    let mut is_binding = false;
    spin_lock(&cifs_tcp_ses_lock);
    list_for_each_entry!(ses, &(*pserver).smb_ses_list, smb_ses_list);
    if (*ses).Suid != ses_id {
        trace_smb3_ses_not_found(ses_id);
        cifs_server_dbg(FYI, "%s: Could not find session 0x%llx\n", "smb3_get_sign_key", ses_id);
        rc = -ENOENT;
        spin_unlock(&cifs_tcp_ses_lock);
        return rc;
    }
    spin_lock(&(*ses).ses_lock);
    spin_lock(&(*ses).chan_lock);
    is_binding = cifs_chan_needs_reconnect(ses, server) && (*ses).ses_status == SES_GOOD;
    if is_binding {
        memcpy(key, (*ses).smb3signingkey.as_ptr(), SMB3_SIGN_KEY_SIZE);
        spin_unlock(&(*ses).chan_lock); spin_unlock(&(*ses).ses_lock);
        spin_unlock(&cifs_tcp_ses_lock); return 0;
    }
    for i in 0..(*ses).chan_count {
        chan = (*ses).chans.add(i as usize);
        if (*chan).server == server {
            memcpy(key, (*chan).signkey.as_ptr(), SMB3_SIGN_KEY_SIZE);
            spin_unlock(&(*ses).chan_lock); spin_unlock(&(*ses).ses_lock);
            spin_unlock(&cifs_tcp_ses_lock); return 0;
        }
    }
    spin_unlock(&(*ses).chan_lock); spin_unlock(&(*ses).ses_lock);
    cifs_dbg(VFS, "%s: Could not find channel signing key for session 0x%llx\n", "smb3_get_sign_key", ses_id);
    spin_unlock(&cifs_tcp_ses_lock); -ENOENT
}

unsafe fn smb2_find_smb_ses_unlocked(server: *mut TCP_Server_Info, ses_id: u64) -> *mut cifs_ses {
    let pserver = if SERVER_IS_CHAN(server) { (*server).primary_server } else { server };
    let mut ses: *mut cifs_ses = core::ptr::null_mut();
    list_for_each_entry!(ses, &(*pserver).smb_ses_list, smb_ses_list);
    if (*ses).Suid == ses_id {
        spin_lock(&(*ses).ses_lock);
        if (*ses).ses_status != SES_EXITING { cifs_smb_ses_inc_refcount(ses); spin_unlock(&(*ses).ses_lock); return ses; }
        spin_unlock(&(*ses).ses_lock);
    }
    core::ptr::null_mut()
}

unsafe fn smb2_get_sign_key(mut server: *mut TCP_Server_Info, ses_id: u64, key: *mut u8) -> i32 {
    if SERVER_IS_CHAN(server) { server = (*server).primary_server; }
    let mut rc = -ENOENT;
    spin_lock(&cifs_tcp_ses_lock);
    let mut ses: *mut cifs_ses = core::ptr::null_mut();
    list_for_each_entry!(ses, &(*server).smb_ses_list, smb_ses_list);
    if (*ses).Suid == ses_id {
        rc = 0; spin_lock(&(*ses).ses_lock);
        match (*ses).ses_status {
            SES_EXITING | SES_GOOD => if !(*ses).auth_key.response.is_null() { memcpy(key, (*ses).auth_key.response, SMB2_NTLMV2_SESSKEY_SIZE); } else { rc = smb_EIO(smb_eio_trace_no_auth_key); },
            _ => rc = -EAGAIN,
        }
        spin_unlock(&(*ses).ses_lock);
    }
    spin_unlock(&cifs_tcp_ses_lock); rc
}

unsafe fn smb2_find_smb_sess_tcon_unlocked(ses: *mut cifs_ses, tid: u32) -> *mut cifs_tcon {
    let mut tcon: *mut cifs_tcon = core::ptr::null_mut();
    list_for_each_entry!(tcon, &(*ses).tcon_list, tcon_list);
    if (*tcon).tid == tid { spin_lock(&(*tcon).tc_lock); (*tcon).tc_count += 1; spin_unlock(&(*tcon).tc_lock); trace_smb3_tcon_ref((*tcon).debug_id, (*tcon).tc_count, netfs_trace_tcon_ref_get_find_sess_tcon); return tcon; }
    core::ptr::null_mut()
}

pub unsafe fn smb2_find_smb_tcon(server: *mut TCP_Server_Info, ses_id: u64, tid: u32) -> *mut cifs_tcon {
    spin_lock(&cifs_tcp_ses_lock); let ses = smb2_find_smb_ses_unlocked(server, ses_id);
    if ses.is_null() { spin_unlock(&cifs_tcp_ses_lock); return core::ptr::null_mut(); }
    let tcon = smb2_find_smb_sess_tcon_unlocked(ses, tid); spin_unlock(&cifs_tcp_ses_lock); cifs_put_smb_ses(ses); tcon
}

unsafe fn smb2_calc_signature(rqst: *mut smb_rqst, server: *mut TCP_Server_Info) -> i32 {
    let iov = (*rqst).rq_iov; let shdr = iov[0].iov_base as *mut smb2_hdr; let sid = le64_to_cpu((*shdr).SessionId); let mut key = [0u8; SMB2_NTLMV2_SESSKEY_SIZE];
    let rc = smb2_get_sign_key(server, sid, key.as_mut_ptr()); if rc != 0 { return rc; }
    let mut sig = [0u8; SMB2_HMACSHA256_SIZE]; memset((*shdr).Signature.as_mut_ptr(), 0, SMB2_SIGNATURE_SIZE); let mut ctx = hmac_sha256_ctx::default(); hmac_sha256_init_usingrawkey(&mut ctx, key.as_ptr(), key.len());
    let mut drqst = *rqst; if drqst.rq_nvec >= 2 && iov[0].iov_len == 4 { hmac_sha256_update(&mut ctx, iov[0].iov_base, iov[0].iov_len); drqst.rq_iov = drqst.rq_iov.add(1); drqst.rq_nvec -= 1; }
    let rc = __cifs_calc_signature(&mut drqst, server, sig.as_mut_ptr(), &mut cifs_calc_sig_ctx { hmac: &mut ctx }); if rc == 0 { memcpy((*shdr).Signature.as_mut_ptr(), sig.as_ptr(), SMB2_SIGNATURE_SIZE); } memzero_explicit(key.as_mut_ptr(), key.len()); rc
}

unsafe fn generate_key(ses: *mut cifs_ses, label: kvec, context: kvec, key: *mut u8, key_size: usize, full_key_size: usize) {
    let mut ctx = hmac_sha256_ctx::default(); let mut hash = [0u8; SMB2_HMACSHA256_SIZE]; let i = [0u8,0,0,1]; let l128 = [0u8,0,0,128]; let l256 = [0u8,0,1,0];
    hmac_sha256_init_usingrawkey(&mut ctx, (*ses).auth_key.response, full_key_size); hmac_sha256_update(&mut ctx, i.as_ptr(), 4); hmac_sha256_update(&mut ctx, label.iov_base, label.iov_len); hmac_sha256_update(&mut ctx, &0u8 as *const u8, 1); hmac_sha256_update(&mut ctx, context.iov_base, context.iov_len);
    if (*(*ses).server).cipher_type == SMB2_ENCRYPTION_AES256_CCM || (*(*ses).server).cipher_type == SMB2_ENCRYPTION_AES256_GCM { hmac_sha256_update(&mut ctx, l256.as_ptr(), 4); } else { hmac_sha256_update(&mut ctx, l128.as_ptr(), 4); } hmac_sha256_final(&mut ctx, hash.as_mut_ptr()); memcpy(key, hash.as_ptr(), key_size); memzero_explicit(hash.as_mut_ptr(), hash.len());
}

#[repr(C)] pub struct derivation { pub label: kvec, pub context: kvec }
#[repr(C)] pub struct derivation_triplet { pub signing: derivation, pub encryption: derivation, pub decryption: derivation }

unsafe fn generate_smb3signingkey(ses: *mut cifs_ses, server: *mut TCP_Server_Info, p: *const derivation_triplet) -> i32 {
    let mut full = SMB2_NTLMV2_SESSKEY_SIZE; spin_lock(&(*ses).ses_lock); spin_lock(&(*ses).chan_lock); let binding = cifs_chan_needs_reconnect(ses, server) && (*ses).ses_status == SES_GOOD; let idx = cifs_ses_get_chan_index(ses, server); spin_unlock(&(*ses).chan_lock); spin_unlock(&(*ses).ses_lock); if idx == CIFS_INVAL_CHAN_INDEX { return -EINVAL; }
    if binding { generate_key(ses, (*p).signing.label, (*p).signing.context, (*ses).chans.add(idx as usize).signkey.as_mut_ptr(), SMB3_SIGN_KEY_SIZE, SMB2_NTLMV2_SESSKEY_SIZE); } else { generate_key(ses, (*p).signing.label, (*p).signing.context, (*ses).smb3signingkey.as_mut_ptr(), SMB3_SIGN_KEY_SIZE, SMB2_NTLMV2_SESSKEY_SIZE); if (*server).dialect == SMB311_PROT_ID && ((*server).cipher_type == SMB2_ENCRYPTION_AES256_CCM || (*server).cipher_type == SMB2_ENCRYPTION_AES256_GCM) { full = (*ses).auth_key.len; } spin_lock(&(*ses).chan_lock); memcpy((*ses).chans.add(idx as usize).signkey.as_mut_ptr(), (*ses).smb3signingkey.as_ptr(), SMB3_SIGN_KEY_SIZE); spin_unlock(&(*ses).chan_lock); generate_key(ses, (*p).encryption.label, (*p).encryption.context, (*ses).smb3encryptionkey.as_mut_ptr(), SMB3_ENC_DEC_KEY_SIZE, full); generate_key(ses, (*p).decryption.label, (*p).decryption.context, (*ses).smb3decryptionkey.as_mut_ptr(), SMB3_ENC_DEC_KEY_SIZE, full); } 0
}

pub unsafe fn generate_smb30signingkey(ses: *mut cifs_ses, server: *mut TCP_Server_Info) -> i32 { let p = derivation_triplet { signing: derivation { label: kvec { iov_base: b"SMB2AESCMAC".as_ptr() as _, iov_len: 12 }, context: kvec { iov_base: b"SmbSign".as_ptr() as _, iov_len: 8 } }, encryption: derivation { label: kvec { iov_base: b"SMB2AESCCM".as_ptr() as _, iov_len: 11 }, context: kvec { iov_base: b"ServerIn ".as_ptr() as _, iov_len: 10 } }, decryption: derivation { label: kvec { iov_base: b"SMB2AESCCM".as_ptr() as _, iov_len: 11 }, context: kvec { iov_base: b"ServerOut".as_ptr() as _, iov_len: 10 } } }; generate_smb3signingkey(ses, server, &p) }
pub unsafe fn generate_smb311signingkey(ses: *mut cifs_ses, server: *mut TCP_Server_Info) -> i32 { let h = (*ses).preauth_sha_hash.as_ptr() as _; let p = derivation_triplet { signing: derivation { label: kvec { iov_base: b"SMBSigningKey".as_ptr() as _, iov_len: 14 }, context: kvec { iov_base: h, iov_len: 64 } }, encryption: derivation { label: kvec { iov_base: b"SMBC2SCipherKey".as_ptr() as _, iov_len: 16 }, context: kvec { iov_base: h, iov_len: 64 } }, decryption: derivation { label: kvec { iov_base: b"SMBS2CCipherKey".as_ptr() as _, iov_len: 16 }, context: kvec { iov_base: h, iov_len: 64 } } }; generate_smb3signingkey(ses, server, &p) }

unsafe fn smb3_calc_signature(rqst: *mut smb_rqst, server: *mut TCP_Server_Info) -> i32 { if (*(*server).vals).protocol_id <= SMB21_PROT_ID { return smb2_calc_signature(rqst, server); } let shdr = (*rqst).rq_iov[0].iov_base as *mut smb2_hdr; let mut key = [0u8; SMB3_SIGN_KEY_SIZE]; let rc = smb3_get_sign_key(le64_to_cpu((*shdr).SessionId), server, key.as_mut_ptr()); if rc != 0 { return rc; } let mut sig = [0u8; SMB2_CMACAES_SIZE]; let mut ctx = aes_cmac_ctx::default(); let mut ck = aes_cmac_key::default(); let rc = aes_cmac_preparekey(&mut ck, key.as_ptr(), SMB2_CMACAES_SIZE); memzero_explicit(key.as_mut_ptr(), key.len()); if rc != 0 { return rc; } aes_cmac_init(&mut ctx, &ck); let mut d = *rqst; let iov = (*rqst).rq_iov; if d.rq_nvec >= 2 && iov[0].iov_len == 4 { aes_cmac_update(&mut ctx, iov[0].iov_base, iov[0].iov_len); d.rq_iov = d.rq_iov.add(1); d.rq_nvec -= 1; } let rc = __cifs_calc_signature(&mut d, server, sig.as_mut_ptr(), &mut cifs_calc_sig_ctx { cmac: &mut ctx }); if rc == 0 { memcpy((*shdr).Signature.as_mut_ptr(), sig.as_ptr(), SMB2_SIGNATURE_SIZE); } rc }

pub unsafe fn smb2_verify_signature(rqst: *mut smb_rqst, server: *mut TCP_Server_Info) -> i32 { let shdr = (*rqst).rq_iov[0].iov_base as *mut smb2_hdr; if (*shdr).Command == SMB2_NEGOTIATE || (*shdr).Command == SMB2_SESSION_SETUP || (*shdr).Command == SMB2_OPLOCK_BREAK || (*server).ignore_signature || !(*server).session_estab { return 0; } let mut saved = [0i8; SMB2_SIGNATURE_SIZE]; memcpy(saved.as_mut_ptr() as _, (*shdr).Signature.as_ptr(), SMB2_SIGNATURE_SIZE); memset((*shdr).Signature.as_mut_ptr(), 0, SMB2_SIGNATURE_SIZE); let rc = smb3_calc_signature(rqst, server); if rc != 0 { return rc; } if crypto_memneq(saved.as_ptr() as _, (*shdr).Signature.as_ptr(), SMB2_SIGNATURE_SIZE) { -EACCES } else { 0 } }

// Remaining request/MID and AEAD routines retain the C call structure and use external kernel/CIFS definitions.
pub unsafe fn smb2_seq_num_into_buf(server: *mut TCP_Server_Info, shdr: *mut smb2_hdr) { let n = le16_to_cpu((*shdr).CreditCharge); (*shdr).MessageId = get_next_mid64(server); for _ in 1..n { get_next_mid(server); } }

unsafe fn smb2_mid_entry_alloc(shdr: *const smb2_hdr, server: *mut TCP_Server_Info) -> *mut mid_q_entry { if server.is_null() { return core::ptr::null_mut(); } let temp = mempool_alloc(&cifs_mid_pool, GFP_NOFS); memset(temp as _, 0, core::mem::size_of::<mid_q_entry>()); refcount_set(&mut (*temp).refcount, 1); spin_lock_init(&mut (*temp).mid_lock); (*temp).mid = le64_to_cpu((*shdr).MessageId); let c = le16_to_cpu((*shdr).CreditCharge); (*temp).credits = if c > 0 { c } else { 1 }; (*temp).pid = (*current).pid; (*temp).command = (*shdr).Command; (*temp).when_alloc = jiffies; get_task_struct(current); (*temp).creator = current; (*temp).callback = cifs_wake_up_task; (*temp).callback_data = current; atomic_inc(&mid_count); (*temp).mid_state = MID_REQUEST_ALLOCATED; trace_smb3_cmd_enter(le32_to_cpu((*shdr).Id.SyncId.TreeId), le64_to_cpu((*shdr).SessionId), le16_to_cpu((*shdr).Command), (*temp).mid); temp }

unsafe fn smb2_get_mid_entry(ses: *mut cifs_ses, server: *mut TCP_Server_Info, shdr: *mut smb2_hdr, mid: *mut *mut mid_q_entry) -> i32 { match READ_ONCE((*server).tcpStatus) { CifsExiting => return -ENOENT, CifsNeedReconnect => return -EAGAIN, CifsNeedNegotiate if (*shdr).Command != SMB2_NEGOTIATE => return -EAGAIN, _ => {} } match READ_ONCE((*ses).ses_status) { SES_NEW if (*shdr).Command != SMB2_SESSION_SETUP && (*shdr).Command != SMB2_NEGOTIATE => return -EAGAIN, SES_EXITING if (*shdr).Command != SMB2_LOGOFF => return -EAGAIN, _ => {} } *mid = smb2_mid_entry_alloc(shdr, server); if (*mid).is_null() { return -ENOMEM; } spin_lock(&(*server).mid_queue_lock); list_add_tail(&mut (**mid).qhead, &mut (*server).pending_mid_q); spin_unlock(&(*server).mid_queue_lock); 0 }

pub unsafe fn smb2_check_receive(mid: *mut mid_q_entry, server: *mut TCP_Server_Info, log_error: bool) -> i32 { let len = (*mid).resp_buf_size; let mut iov = [kvec { iov_base: (*mid).resp_buf as _, iov_len: len }]; let mut rqst = smb_rqst { rq_iov: iov.as_mut_ptr(), rq_nvec: 1 }; dump_smb((*mid).resp_buf, core::cmp::min(80, len)); if len > 24 && (*server).sign && !(*mid).decrypted { let _ = smb2_verify_signature(&mut rqst, server); } map_smb2_to_linux_error((*mid).resp_buf, log_error) }

pub unsafe fn smb2_setup_request(ses: *mut cifs_ses, server: *mut TCP_Server_Info, rqst: *mut smb_rqst) -> *mut mid_q_entry { let shdr = (*rqst).rq_iov[0].iov_base as *mut smb2_hdr; smb2_seq_num_into_buf(server, shdr); let mut mid = core::ptr::null_mut(); let rc = smb2_get_mid_entry(ses, server, shdr, &mut mid); if rc != 0 { revert_current_mid_from_hdr(server, shdr); return ERR_PTR(rc); } let rc = smb2_sign_rqst(rqst, server); if rc != 0 { revert_current_mid_from_hdr(server, shdr); delete_mid(server, mid); return ERR_PTR(rc); } mid }
pub unsafe fn smb2_setup_async_request(server: *mut TCP_Server_Info, rqst: *mut smb_rqst) -> *mut mid_q_entry { let shdr = (*rqst).rq_iov[0].iov_base as *mut smb2_hdr; spin_lock(&(*server).srv_lock); if (*server).tcpStatus == CifsNeedNegotiate && (*shdr).Command != SMB2_NEGOTIATE { spin_unlock(&(*server).srv_lock); return ERR_PTR(-EAGAIN); } spin_unlock(&(*server).srv_lock); smb2_seq_num_into_buf(server, shdr); let mid = smb2_mid_entry_alloc(shdr, server); if mid.is_null() { revert_current_mid_from_hdr(server, shdr); return ERR_PTR(-ENOMEM); } let rc = smb2_sign_rqst(rqst, server); if rc != 0 { revert_current_mid_from_hdr(server, shdr); release_mid(server, mid); return ERR_PTR(rc); } mid }

pub unsafe fn smb3_crypto_aead_allocate(server: *mut TCP_Server_Info) -> i32 { if (*server).secmech.enc.is_null() { let n = if (*server).cipher_type == SMB2_ENCRYPTION_AES128_GCM || (*server).cipher_type == SMB2_ENCRYPTION_AES256_GCM { "gcm(aes)" } else { "ccm(aes)" }; let p = crypto_alloc_aead(n, 0, 0); if IS_ERR(p) { return PTR_ERR(p); } (*server).secmech.enc = p; } if (*server).secmech.dec.is_null() { let n = if (*server).cipher_type == SMB2_ENCRYPTION_AES128_GCM || (*server).cipher_type == SMB2_ENCRYPTION_AES256_GCM { "gcm(aes)" } else { "ccm(aes)" }; let p = crypto_alloc_aead(n, 0, 0); if IS_ERR(p) { crypto_free_aead((*server).secmech.enc); (*server).secmech.enc = core::ptr::null_mut(); return PTR_ERR(p); } (*server).secmech.dec = p; } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
