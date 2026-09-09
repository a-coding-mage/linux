/* SPDX-License-Identifier: LGPL-2.1 */
/* Source translation of smb1proto.h. */

#[repr(C)]
pub struct cifs_unix_set_info_args {
    pub ctime: u64,
    pub atime: u64,
    pub mtime: u64,
    pub mode: u64,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub device: dev_t,
}

/* CONFIG_CIFS_ALLOW_INSECURE_LEGACY declarations. */
extern "C" {
    pub fn small_smb_init_no_tc(smb_command: i32, wct: i32, ses: *mut cifs_ses, request_buf: *mut *mut core::ffi::c_void) -> i32;
    pub fn CIFSSMBNegotiate(xid: u32, ses: *mut cifs_ses, server: *mut TCP_Server_Info) -> i32;
    pub fn CIFSTCon(xid: u32, ses: *mut cifs_ses, tree: *const i8, tcon: *mut cifs_tcon, nls_codepage: *const nls_table) -> i32;
    pub fn CIFSSMBTDis(xid: u32, tcon: *mut cifs_tcon) -> i32;
    pub fn CIFSSMBEcho(server: *mut TCP_Server_Info) -> i32;
    pub fn CIFSSMBLogoff(xid: u32, ses: *mut cifs_ses) -> i32;
    pub fn CIFSPOSIXDelFile(xid: u32, tcon: *mut cifs_tcon, fileName: *const i8, r#type: u16, nls_codepage: *const nls_table, remap: i32) -> i32;
    pub fn CIFSSMBDelFile(xid: u32, tcon: *mut cifs_tcon, name: *const i8, cifs_sb: *mut cifs_sb_info, dentry: *mut dentry) -> i32;
    pub fn CIFSSMBRmDir(xid: u32, tcon: *mut cifs_tcon, name: *const i8, cifs_sb: *mut cifs_sb_info) -> i32;
    pub fn CIFSSMBMkDir(xid: u32, inode: *mut inode, mode: umode_t, tcon: *mut cifs_tcon, name: *const i8, cifs_sb: *mut cifs_sb_info) -> i32;
    pub fn CIFSPOSIXCreate(xid: u32, tcon: *mut cifs_tcon, posix_flags: u32, mode: u64, netfid: *mut u16, pRetData: *mut FILE_UNIX_BASIC_INFO, pOplock: *mut u32, name: *const i8, nls_codepage: *const nls_table, remap: i32) -> i32;
    pub fn SMBLegacyOpen(xid: u32, tcon: *mut cifs_tcon, fileName: *const i8, openDisposition: i32, access_flags: i32, create_options: i32, netfid: *mut u16, pOplock: *mut i32, pfile_info: *mut FILE_ALL_INFO, nls_codepage: *const nls_table, remap: i32) -> i32;
    pub fn CIFS_open(xid: u32, oparms: *mut cifs_open_parms, oplock: *mut i32, buf: *mut FILE_ALL_INFO) -> i32;
    pub fn cifs_async_readv(rdata: *mut cifs_io_subrequest) -> i32;
    pub fn CIFSSMBRead(xid: u32, io_parms: *mut cifs_io_parms, nbytes: *mut u32, buf: *mut *mut i8, pbuf_type: *mut i32) -> i32;
    pub fn CIFSSMBWrite(xid: u32, io_parms: *mut cifs_io_parms, nbytes: *mut u32, buf: *const i8) -> i32;
    pub fn cifs_async_writev(wdata: *mut cifs_io_subrequest);
    pub fn CIFSSMBWrite2(xid: u32, io_parms: *mut cifs_io_parms, nbytes: *mut u32, iov: *mut kvec, n_vec: i32) -> i32;
    pub fn cifs_lockv(xid: u32, tcon: *mut cifs_tcon, netfid: u16, lock_type: u8, num_unlock: u32, num_lock: u32, buf: *mut LOCKING_ANDX_RANGE) -> i32;
    pub fn CIFSSMBLock(xid: u32, tcon: *mut cifs_tcon, smb_file_id: u16, netpid: u32, len: u64, offset: u64, numUnlock: u32, numLock: u32, lockType: u8, waitFlag: bool, oplock_level: u8) -> i32;
    pub fn CIFSSMBClose(xid: u32, tcon: *mut cifs_tcon, smb_file_id: i32) -> i32;
    pub fn CIFSSMBFlush(xid: u32, tcon: *mut cifs_tcon, smb_file_id: i32) -> i32;
    pub fn CIFSSMBRename(xid: u32, tcon: *mut cifs_tcon, source_dentry: *mut dentry, from_name: *const i8, to_name: *const i8, cifs_sb: *mut cifs_sb_info) -> i32;
    pub fn CIFSSMBRenameOpenFile(xid: u32, pTcon: *mut cifs_tcon, netfid: i32, target_name: *const i8, nls_codepage: *const nls_table, remap: i32) -> i32;
    pub fn CIFSUnixCreateSymLink(xid: u32, tcon: *mut cifs_tcon, fromName: *const i8, toName: *const i8, nls_codepage: *const nls_table, remap: i32) -> i32;
    pub fn CIFSUnixCreateHardLink(xid: u32, tcon: *mut cifs_tcon, fromName: *const i8, toName: *const i8, nls_codepage: *const nls_table, remap: i32) -> i32;
    pub fn CIFSCreateHardLink(xid: u32, tcon: *mut cifs_tcon, source_dentry: *mut dentry, from_name: *const i8, to_name: *const i8, cifs_sb: *mut cifs_sb_info) -> i32;
    pub fn CIFSSMBUnixQuerySymLink(xid: u32, tcon: *mut cifs_tcon, searchName: *const u8, symlinkinfo: *mut *mut i8, nls_codepage: *const nls_table, remap: i32) -> i32;
    pub fn cifs_query_reparse_point(xid: u32, tcon: *mut cifs_tcon, cifs_sb: *mut cifs_sb_info, full_path: *const i8, tag: *mut u32, rsp: *mut kvec, rsp_buftype: *mut i32) -> i32;
    pub fn CIFSSMB_set_compression(xid: u32, tcon: *mut cifs_tcon, fid: u16, compression_state: u16) -> i32;
    pub fn CIFSGetExtAttr(xid: u32, tcon: *mut cifs_tcon, netfid: i32, pExtAttrBits: *mut u64, pMask: *mut u64) -> i32;
    pub fn CIFSSMBQFileInfo(xid: u32, tcon: *mut cifs_tcon, netfid: u16, pFindData: *mut FILE_ALL_INFO) -> i32;
    pub fn CIFSSMBQFSInfo(xid: u32, tcon: *mut cifs_tcon, FSData: *mut kstatfs) -> i32;
    pub fn CIFSSMBSetFSUnixInfo(xid: u32, tcon: *mut cifs_tcon, cap: u64) -> i32;
    pub fn CIFSSMBUnixSetFileInfo(xid: u32, tcon: *mut cifs_tcon, args: *const cifs_unix_set_info_args, fid: u16, pid_of_opener: u32) -> i32;
    pub fn CIFSSMBUnixSetPathInfo(xid: u32, tcon: *mut cifs_tcon, file_name: *const i8, args: *const cifs_unix_set_info_args, nls_codepage: *const nls_table, remap: i32) -> i32;
    pub fn cifs_dump_detail(buf: *mut core::ffi::c_void, buf_len: usize, server: *mut TCP_Server_Info);
    pub fn cifs_sign_rqst(rqst: *mut smb_rqst, server: *mut TCP_Server_Info, pexpected_response_sequence_number: *mut u32) -> i32;
    pub fn cifs_verify_signature(rqst: *mut smb_rqst, server: *mut TCP_Server_Info, expected_sequence_number: u32) -> i32;
    pub fn map_smb_to_linux_error(buf: *mut i8, logErr: bool) -> i32;
    pub fn smb1_init_maperror() -> i32;
    pub fn header_assemble(buffer: *mut smb_hdr, smb_command: i8, treeCon: *const cifs_tcon, word_count: i32) -> u32;
    pub fn is_valid_oplock_break(buffer: *mut i8, srv: *mut TCP_Server_Info) -> bool;
    pub fn smbCalcSize(buf: *mut core::ffi::c_void) -> u32;
    pub fn reset_cifs_unix_caps(xid: u32, tcon: *mut cifs_tcon, cifs_sb: *mut cifs_sb_info, ctx: *mut smb3_fs_context);
    pub fn CIFS_SessSetup(xid: u32, ses: *mut cifs_ses, server: *mut TCP_Server_Info, nls_cp: *const nls_table) -> i32;
    pub fn cifs_setup_async_request(server: *mut TCP_Server_Info, rqst: *mut smb_rqst) -> *mut mid_q_entry;
    pub fn SendReceiveNoRsp(xid: u32, ses: *mut cifs_ses, in_buf: *mut i8, in_len: u32, flags: i32) -> i32;
    pub fn cifs_setup_request(ses: *mut cifs_ses, server: *mut TCP_Server_Info, rqst: *mut smb_rqst) -> *mut mid_q_entry;
    pub fn checkSMB(buf: *mut i8, pdu_len: u32, total_read: u32, server: *mut TCP_Server_Info) -> i32;
}

#[inline]
pub unsafe fn get_mid(smb: *const smb_hdr) -> u16 { (*smb).Mid.to_le() }
#[inline]
pub unsafe fn compare_mid(mid: u16, smb: *const smb_hdr) -> bool { mid == (*smb).Mid.to_le() }
#[inline]
pub unsafe fn GETU16(var: *const u8) -> u16 { *(var as *const u16) }
#[inline]
pub unsafe fn GETU32(var: *const u8) -> u32 { *(var as *const u32) }
#[inline]
pub unsafe fn BCC(smb: *mut smb_hdr) -> *mut u8 { (smb as *mut u8).add(core::mem::size_of::<smb_hdr>() + 2 * (*smb).WordCount as usize) }
#[inline]
pub unsafe fn pByteArea(smb_var: *mut smb_hdr) -> *mut u8 { BCC(smb_var).add(2) }
#[inline]
pub unsafe fn get_bcc(hdr: *mut smb_hdr) -> u16 { u16::from_le(*(BCC(hdr) as *const u16)) }
#[inline]
pub unsafe fn put_bcc(count: u16, hdr: *mut smb_hdr) { (BCC(hdr) as *mut u16).write_unaligned(count.to_le()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
