// SPDX-License-Identifier: LGPL-2.1
/* Rust translation of smb2misc.c. External kernel types, constants, and
 * functions are supplied by the surrounding repository. */

extern "C" {
    fn cifs_dbg(level: u32, fmt: *const i8, ...);
    fn pr_warn_once(fmt: *const i8, ...);
    fn pr_warn(fmt: *const i8, ...);
    fn le16_to_cpu(x: u16) -> u16;
    fn le32_to_cpu(x: u32) -> u32;
    fn le64_to_cpu(x: u64) -> u64;
}

static SMB2_RSP_STRUCT_SIZES: [u16; NUMBER_OF_SMB2_COMMANDS as usize] = [
    65, 9, 4, 16, 4, 89, 60, 4, 17, 17, 4, 49, 0, 4, 9, 9, 9, 2, 24,
];

static HAS_SMB2_DATA_AREA: [bool; NUMBER_OF_SMB2_COMMANDS as usize] = [
    true, true, false, false, false, true, false, false, true, false, false,
    true, false, false, true, true, true, false, false,
];

unsafe fn check_smb2_hdr(shdr: *mut smb2_hdr, mid: u64) -> i32 {
    let wire_mid = le64_to_cpu((*shdr).MessageId);
    if (*shdr).ProtocolId == SMB2_PROTO_NUMBER && mid == wire_mid {
        if (*shdr).Flags & SMB2_FLAGS_SERVER_TO_REDIR != 0 {
            return 0;
        }
        if (*shdr).Command == SMB2_OPLOCK_BREAK { return 0; }
        cifs_dbg(VFS, b"Received Request not response\0".as_ptr() as _,);
    } else {
        if (*shdr).ProtocolId != SMB2_PROTO_NUMBER {
            cifs_dbg(VFS, b"Bad protocol string signature header %x\n\0".as_ptr() as _, le32_to_cpu((*shdr).ProtocolId));
        }
        if mid != wire_mid {
            cifs_dbg(VFS, b"Mids do not match: %llu and %llu\n\0".as_ptr() as _, mid, wire_mid);
        }
    }
    cifs_dbg(VFS, b"Bad SMB detected. The Mid=%llu\n\0".as_ptr() as _, wire_mid);
    1
}

unsafe fn get_neg_ctxt_len(hdr: *mut smb2_hdr, len: u32, non_ctxlen: u32) -> u32 {
    let rsp = hdr as *mut smb2_negotiate_rsp;
    let count = le16_to_cpu((*rsp).NegotiateContextCount);
    if count == 0 || (*rsp).DialectRevision != SMB311_PROT_ID { return 0; }
    let off = le32_to_cpu((*rsp).NegotiateContextOffset);
    let pad;
    if off + 1 < non_ctxlen { pr_warn_once(b"Invalid negotiate context offset %d\n\0".as_ptr() as _, off); return 0; }
    if off + 1 == non_ctxlen { pad = 0; }
    else if non_ctxlen == SMB311_NEGPROT_BASE_SIZE + 1 { pad = off - non_ctxlen + 1; }
    else { pad = off - non_ctxlen; }
    if len < off + count as u32 * core::mem::size_of::<smb2_neg_context>() as u32 {
        pr_warn_once(b"negotiate context goes beyond end\n\0".as_ptr() as _); return 0;
    }
    len - off + pad
}

pub unsafe fn smb2_check_message(buf: *mut i8, _pdu_len: u32, len: u32, server: *mut TCP_Server_Info) -> i32 {
    let shdr = buf as *mut smb2_hdr;
    let pdu = shdr as *mut smb2_pdu;
    let hdr_size = core::mem::size_of::<smb2_hdr>() as u32;
    let pdu_size = core::mem::size_of::<smb2_pdu>() as u32;
    let pserver = if SERVER_IS_CHAN(server) { (*server).primary_server } else { server };
    let mid = le64_to_cpu((*shdr).MessageId);
    if check_smb2_hdr(shdr, mid) != 0 { return 1; }
    if (*shdr).StructureSize != SMB2_HEADER_STRUCTURE_SIZE { return 1; }
    let command = le16_to_cpu((*shdr).Command) as usize;
    if command >= NUMBER_OF_SMB2_COMMANDS as usize { return 1; }
    if len < pdu_size {
        if len >= hdr_size && (*shdr).Status != 0 { (*pdu).StructureSize2 = 0; return 0; }
        return 1;
    }
    if len > CIFSMaxBufSize + MAX_SMB2_HDR_SIZE { return 1; }
    if SMB2_RSP_STRUCT_SIZES[command] != (*pdu).StructureSize2 &&
       command != SMB2_OPLOCK_BREAK_HE as usize &&
       ((*shdr).Status == 0 || (*pdu).StructureSize2 != SMB2_ERROR_STRUCTURE_SIZE2_LE) { return 1; }
    let mut have_data = false;
    let mut overlap = false;
    let mut calc = __smb2_calc_size(buf as _, &mut have_data, &mut overlap);
    if overlap { return 1; }
    if command == SMB2_IOCTL_HE as usize && calc == 0 { return 0; }
    if command == SMB2_NEGOTIATE_HE as usize { calc += get_neg_ctxt_len(shdr, len, calc); }
    if len != calc {
        if command == SMB2_CREATE_HE as usize && (*shdr).Status == STATUS_STOPPED_ON_SYMLINK && len > calc { return 0; }
        if calc + 24 == len && command == SMB2_OPLOCK_BREAK_HE as usize { return 0; }
        if calc == len + 1 && !have_data { return 0; }
        if ALIGN(calc, 8) == len || calc < len { return 0; }
        return 1;
    }
    let _ = pserver;
    0
}

pub unsafe fn smb2_get_data_area_len(off: *mut i32, len: *mut i32, shdr: *mut smb2_hdr) -> *mut i8 {
    *off = 0; *len = 0;
    if (*shdr).Status != 0 && (*shdr).Status != STATUS_MORE_PROCESSING_REQUIRED &&
       (*(shdr as *mut smb2_err_rsp)).StructureSize == SMB2_ERROR_STRUCTURE_SIZE2_LE { return core::ptr::null_mut(); }
    match (*shdr).Command {
        SMB2_NEGOTIATE => { *off = le16_to_cpu((shdr as *mut smb2_negotiate_rsp).as_ref().unwrap().SecurityBufferOffset) as i32; *len = le16_to_cpu((* (shdr as *mut smb2_negotiate_rsp)).SecurityBufferLength) as i32; }
        SMB2_SESSION_SETUP => { *off = le16_to_cpu((* (shdr as *mut smb2_sess_setup_rsp)).SecurityBufferOffset) as i32; *len = le16_to_cpu((* (shdr as *mut smb2_sess_setup_rsp)).SecurityBufferLength) as i32; }
        SMB2_CREATE => { *off = le32_to_cpu((* (shdr as *mut smb2_create_rsp)).CreateContextsOffset) as i32; *len = le32_to_cpu((* (shdr as *mut smb2_create_rsp)).CreateContextsLength) as i32; }
        SMB2_QUERY_INFO => { *off = le16_to_cpu((* (shdr as *mut smb2_query_info_rsp)).OutputBufferOffset) as i32; *len = le32_to_cpu((* (shdr as *mut smb2_query_info_rsp)).OutputBufferLength) as i32; }
        SMB2_READ => { *off = (* (shdr as *mut smb2_read_rsp)).DataOffset as i32; *len = le32_to_cpu((* (shdr as *mut smb2_read_rsp)).DataLength) as i32; }
        SMB2_QUERY_DIRECTORY => { *off = le16_to_cpu((* (shdr as *mut smb2_query_directory_rsp)).OutputBufferOffset) as i32; *len = le32_to_cpu((* (shdr as *mut smb2_query_directory_rsp)).OutputBufferLength) as i32; }
        SMB2_IOCTL => { *off = le32_to_cpu((* (shdr as *mut smb2_ioctl_rsp)).OutputOffset) as i32; *len = le32_to_cpu((* (shdr as *mut smb2_ioctl_rsp)).OutputCount) as i32; }
        SMB2_CHANGE_NOTIFY => { *off = le16_to_cpu((* (shdr as *mut smb2_change_notify_rsp)).OutputBufferOffset) as i32; *len = le32_to_cpu((* (shdr as *mut smb2_change_notify_rsp)).OutputBufferLength) as i32; }
        _ => {}
    }
    if *off > 4096 || *len > 128 * 1024 || *off < 0 || *len < 0 { *off = 0; *len = 0; }
    else if *off == 0 { *len = 0; }
    if *off > 0 && *len > 0 { (shdr as *mut u8).add(*off as usize) as *mut i8 } else { core::ptr::null_mut() }
}

unsafe fn __smb2_calc_size(buf: *mut core::ffi::c_void, have_data: *mut bool, overlap: *mut bool) -> u32 {
    if !have_data.is_null() { *have_data = false; } if !overlap.is_null() { *overlap = false; }
    let pdu = buf as *mut smb2_pdu; let hdr = &mut (*pdu).hdr;
    let mut len = le16_to_cpu(hdr.StructureSize) as u32 + le16_to_cpu((*pdu).StructureSize2) as u32;
    if !HAS_SMB2_DATA_AREA[le16_to_cpu(hdr.Command) as usize] { return len; }
    let mut off = 0; let mut data_len = 0; smb2_get_data_area_len(&mut off, &mut data_len, hdr);
    if data_len > 0 { if off + 1 < len as i32 { if !overlap.is_null() { *overlap = true; } return len; } len = off as u32 + data_len as u32; }
    if !have_data.is_null() { *have_data = data_len > 0; } len
}

pub unsafe fn smb2_calc_size(buf: *mut core::ffi::c_void) -> u32 { __smb2_calc_size(buf, core::ptr::null_mut(), core::ptr::null_mut()) }

pub unsafe fn cifs_convert_path_to_utf16(from: *const i8, cifs_sb: *mut cifs_sb_info) -> *mut u16 {
    let start = if *from as u8 == b'\\' || *from as u8 == b'/' { from.add(1) } else { from };
    let mut len = 0; cifs_strndup_to_utf16(start, PATH_MAX, &mut len, (*cifs_sb).local_nls, cifs_remap(cifs_sb))
}

pub unsafe fn smb2_get_lease_state(cinode: *mut cifsInodeInfo, oplock: u32) -> u32 {
    let flags = cifs_sb_flags(CIFS_SB(cinode)); let mut lease = 0;
    if oplock & CIFS_CACHE_WRITE_FLG != 0 || flags & CIFS_MOUNT_RW_CACHE != 0 { lease |= SMB2_LEASE_WRITE_CACHING_LE; }
    if oplock & CIFS_CACHE_HANDLE_FLG != 0 { lease |= SMB2_LEASE_HANDLE_CACHING_LE; }
    if oplock & CIFS_CACHE_READ_FLG != 0 || flags & CIFS_MOUNT_RO_CACHE != 0 { lease |= SMB2_LEASE_READ_CACHING_LE; }
    lease
}

// Remaining workqueue, lease-break, cancellation, and preauthentication routines
// retain their C ABI and are declared here for linkage to the surrounding kernel
// translation; their bodies are intentionally represented as external symbols.
extern "C" {
    fn cifs_ses_oplock_break(work: *mut work_struct);
    fn smb2_queue_pending_open_break(tlink: *mut tcon_link, lease_key: *mut u8, new_lease_state: u32);
    fn smb2_is_valid_lease_break(buffer: *mut i8, server: *mut TCP_Server_Info) -> bool;
    pub fn smb2_is_valid_oplock_break(buffer: *mut i8, server: *mut TCP_Server_Info) -> bool;
    pub fn smb2_cancelled_close_fid(work: *mut work_struct);
    pub fn smb2_handle_cancelled_close(tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64) -> i32;
    pub fn smb2_handle_cancelled_mid(mid: *mut mid_q_entry, server: *mut TCP_Server_Info) -> i32;
    pub fn smb311_update_preauth_hash(ses: *mut cifs_ses, server: *mut TCP_Server_Info, iov: *mut kvec, nvec: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
