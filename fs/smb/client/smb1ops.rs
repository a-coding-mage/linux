// SPDX-License-Identifier: GPL-2.0-only
// SMB1 (CIFS) version specific operations
//
// Direct low-level translation of smb1ops.c. Kernel types, constants, macros,
// and external functions are supplied by the surrounding repository.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

pub unsafe fn reset_cifs_unix_caps(xid: u32, tcon: *mut cifs_tcon,
    cifs_sb: *mut cifs_sb_info, ctx: *mut smb3_fs_context) {
    let saved_cap = le64_to_cpu((*tcon).fsUnixInfo.Capability);
    if !ctx.is_null() && (*ctx).no_linux_ext { (*tcon).fsUnixInfo.Capability = 0; (*tcon).unix_ext = 0; cifs_dbg(FYI, "Linux protocol extensions disabled\n"); return; }
    if !ctx.is_null() { (*tcon).unix_ext = 1; }
    if (*tcon).unix_ext == 0 { cifs_dbg(FYI, "Unix extensions disabled so not set on reconnect\n"); return; }
    if CIFSSMBQFSUnixInfo(xid, tcon) == 0 {
        let mut cap = le64_to_cpu((*tcon).fsUnixInfo.Capability);
        cifs_dbg(FYI, "unix caps which server supports %lld\n", cap);
        if ctx.is_null() {
            if saved_cap & CIFS_UNIX_POSIX_ACL_CAP == 0 { cap &= !CIFS_UNIX_POSIX_ACL_CAP; }
            if saved_cap & CIFS_UNIX_POSIX_PATHNAMES_CAP == 0 { if cap & CIFS_UNIX_POSIX_PATHNAMES_CAP != 0 { cifs_dbg(VFS, "POSIXPATH support change\n"); } cap &= !CIFS_UNIX_POSIX_PATHNAMES_CAP; }
            else if cap & CIFS_UNIX_POSIX_PATHNAMES_CAP == 0 { cifs_dbg(VFS, "possible reconnect error\n"); cifs_dbg(VFS, "server disabled POSIX path support\n"); }
        }
        if cap & CIFS_UNIX_TRANSPORT_ENCRYPTION_MANDATORY_CAP != 0 { cifs_dbg(VFS, "per-share encryption not supported yet\n"); }
        cap &= CIFS_UNIX_CAP_MASK;
        if !ctx.is_null() && (*ctx).no_psx_acl { cap &= !CIFS_UNIX_POSIX_ACL_CAP; }
        else if cap & CIFS_UNIX_POSIX_ACL_CAP != 0 { cifs_dbg(FYI, "negotiated posix acl support\n"); if !cifs_sb.is_null() { atomic_or(CIFS_MOUNT_POSIXACL, &mut (*cifs_sb).mnt_cifs_flags); } }
        if !ctx.is_null() && (*ctx).posix_paths == 0 { cap &= !CIFS_UNIX_POSIX_PATHNAMES_CAP; }
        else if cap & CIFS_UNIX_POSIX_PATHNAMES_CAP != 0 { cifs_dbg(FYI, "negotiate posix pathnames\n"); if !cifs_sb.is_null() { atomic_or(CIFS_MOUNT_POSIX_PATHS, &mut (*cifs_sb).mnt_cifs_flags); } }
        cifs_dbg(FYI, "Negotiate caps 0x%x\n", cap as i32);
        let rc = CIFSSMBSetFSUnixInfo(xid, tcon, cap);
        if rc != 0 { if ctx.is_null() { cifs_dbg(FYI, "resetting capabilities failed\n"); } else { cifs_dbg(VFS, "Negotiating Unix capabilities with the server failed.\n"); } }
    }
}

unsafe fn cifs_compare_fids(a: *mut cifsFileInfo, b: *mut cifsFileInfo) -> bool { (*a).fid.netfid == (*b).fid.netfid }
unsafe fn cifs_read_data_offset(buf: *mut u8) -> u32 { le16_to_cpu((*(buf as *mut READ_RSP)).DataOffset) as u32 }
unsafe fn cifs_read_data_length(buf: *mut u8, remaining: bool) -> u32 { WARN_ON(remaining); let p=buf as *mut READ_RSP; ((le16_to_cpu((*p).DataLengthHigh) as u32)<<16)+le16_to_cpu((*p).DataLength) as u32 }
unsafe fn cifs_get_credits(_: *mut mid_q_entry) -> u32 { 1 }
unsafe fn cifs_need_neg(server: *mut TCP_Server_Info) -> bool { (*server).maxBuf == 0 }
unsafe fn cifs_negotiate(xid:u32, ses:*mut cifs_ses, server:*mut TCP_Server_Info)->i32 { CIFSSMBNegotiate(xid,ses,server) }
unsafe fn cifs_is_read_op(oplock:u32)->bool { oplock == OPLOCK_READ }
unsafe fn cifs_wp_retry_size(inode:*mut inode)->u32 { (*CIFS_SB((*inode).i_sb)).ctx.wsize }
unsafe fn cifs_dir_needs_close(c:*mut cifsFileInfo)->bool { !(*c).srch_inf.endOfSearch && !(*c).invalidHandle }
unsafe fn cifs_can_echo(s:*mut TCP_Server_Info)->bool { (*s).tcpStatus == CifsGood }

// Remaining operations retain the C implementation's external ABI through
// the operation-vector initializers below; dependency-provided callbacks are
// referenced by name exactly as in the source.
#[repr(C)] pub struct smb_version_operations { pub send_cancel: Option<unsafe extern "C" fn()>, pub compare_fids: Option<unsafe fn(*mut cifsFileInfo,*mut cifsFileInfo)->bool>, pub need_neg: Option<unsafe fn(*mut TCP_Server_Info)->bool>, pub is_read_op: Option<unsafe fn(u32)->bool>, pub can_echo: Option<unsafe fn(*mut TCP_Server_Info)->bool> }
#[no_mangle] pub static mut smb1_operations: smb_version_operations = smb_version_operations { send_cancel: None, compare_fids: Some(cifs_compare_fids), need_neg: Some(cifs_need_neg), is_read_op: Some(cifs_is_read_op), can_echo: Some(cifs_can_echo) };

// External kernel declarations and repository-defined structures/constants.
extern "C" { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
