/* SPDX-License-Identifier: LGPL-2.1 */
/* Rust translation of smb2proto.h. Included Linux/kernel types are supplied externally. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
extern "C" {
    pub fn map_smb2_to_linux_error(buf: *mut core::ffi::c_char, log_err: bool) -> core::ffi::c_int;
    pub fn smb2_init_maperror() -> core::ffi::c_int;

    #[cfg(feature = "CONFIG_SMB_KUNIT_TESTS")]
    pub fn smb2_get_err_map_test(smb2_status: u32) -> *const status_to_posix_error;
    #[cfg(feature = "CONFIG_SMB_KUNIT_TESTS")]
    pub static mut smb2_error_map_table_test: *const status_to_posix_error;
    #[cfg(feature = "CONFIG_SMB_KUNIT_TESTS")]
    pub static mut smb2_error_map_num: u32;

    pub fn smb2_check_message(buf: *mut core::ffi::c_char, pdu_len: u32, len: u32, server: *mut TCP_Server_Info) -> core::ffi::c_int;
    pub fn smb2_calc_size(buf: *mut core::ffi::c_void) -> u32;
    pub fn smb2_get_data_area_len(off: *mut core::ffi::c_int, len: *mut core::ffi::c_int, shdr: *mut smb2_hdr) -> *mut le16;
    pub fn cifs_convert_path_to_utf16(from: *const core::ffi::c_char, cifs_sb: *mut cifs_sb_info) -> *mut le16;
    pub fn smb2_verify_signature(rqst: *mut smb_rqst, server: *mut TCP_Server_Info) -> core::ffi::c_int;
    pub fn smb2_check_receive(mid: *mut mid_q_entry, server: *mut TCP_Server_Info, log_error: bool) -> core::ffi::c_int;
    pub fn smb2_setup_request(ses: *mut cifs_ses, server: *mut TCP_Server_Info, rqst: *mut smb_rqst) -> *mut mid_q_entry;
    pub fn smb2_setup_async_request(server: *mut TCP_Server_Info, rqst: *mut smb_rqst) -> *mut mid_q_entry;
    pub fn smb2_find_smb_tcon(server: *mut TCP_Server_Info, ses_id: u64, tid: u32) -> *mut cifs_tcon;
    pub fn smb2_get_lease_state(cinode: *mut cifsInodeInfo, oplock: u32) -> le32;
    pub fn smb2_is_valid_oplock_break(buffer: *mut core::ffi::c_char, server: *mut TCP_Server_Info) -> bool;
    pub fn smb3_handle_read_data(server: *mut TCP_Server_Info, mid: *mut mid_q_entry) -> core::ffi::c_int;
    pub fn smb2_create_reparse_inode(data: *mut cifs_open_info_data, sb: *mut super_block, xid: u32, tcon: *mut cifs_tcon, full_path: *const core::ffi::c_char, directory: bool, reparse_iov: *mut kvec, xattr_iov: *mut kvec) -> *mut inode;
    pub fn smb2_query_reparse_point(xid: u32, tcon: *mut cifs_tcon, cifs_sb: *mut cifs_sb_info, full_path: *const core::ffi::c_char, tag: *mut u32, rsp: *mut kvec, rsp_buftype: *mut core::ffi::c_int) -> core::ffi::c_int;
    pub fn smb2_query_path_info(xid: u32, tcon: *mut cifs_tcon, cifs_sb: *mut cifs_sb_info, full_path: *const core::ffi::c_char, data: *mut cifs_open_info_data) -> core::ffi::c_int;
    pub fn smb2_set_path_size(xid: u32, tcon: *mut cifs_tcon, full_path: *const core::ffi::c_char, size: u64, cifs_sb: *mut cifs_sb_info, set_alloc: bool, dentry: *mut dentry) -> core::ffi::c_int;
    pub fn smb2_set_file_info(inode: *mut inode, full_path: *const core::ffi::c_char, buf: *mut FILE_BASIC_INFO, xid: u32) -> core::ffi::c_int;
    pub fn smb311_posix_mkdir(xid: u32, inode: *mut inode, mode: umode_t, tcon: *mut cifs_tcon, full_path: *const core::ffi::c_char, cifs_sb: *mut cifs_sb_info) -> core::ffi::c_int;
    pub fn smb2_mkdir(xid: u32, parent_inode: *mut inode, mode: umode_t, tcon: *mut cifs_tcon, name: *const core::ffi::c_char, cifs_sb: *mut cifs_sb_info) -> core::ffi::c_int;
    pub fn smb2_mkdir_setinfo(inode: *mut inode, name: *const core::ffi::c_char, cifs_sb: *mut cifs_sb_info, tcon: *mut cifs_tcon, xid: u32);
    pub fn smb2_rmdir(xid: u32, tcon: *mut cifs_tcon, name: *const core::ffi::c_char, cifs_sb: *mut cifs_sb_info) -> core::ffi::c_int;
    pub fn smb2_unlink(xid: u32, tcon: *mut cifs_tcon, name: *const core::ffi::c_char, cifs_sb: *mut cifs_sb_info, dentry: *mut dentry) -> core::ffi::c_int;
    pub fn smb2_rename_path(xid: u32, tcon: *mut cifs_tcon, source_dentry: *mut dentry, from_name: *const core::ffi::c_char, to_name: *const core::ffi::c_char, cifs_sb: *mut cifs_sb_info) -> core::ffi::c_int;
    pub fn smb2_create_hardlink(xid: u32, tcon: *mut cifs_tcon, source_dentry: *mut dentry, from_name: *const core::ffi::c_char, to_name: *const core::ffi::c_char, cifs_sb: *mut cifs_sb_info) -> core::ffi::c_int;
    pub fn smb3_create_mf_symlink(xid: u32, tcon: *mut cifs_tcon, cifs_sb: *mut cifs_sb_info, path: *const u8, pbuf: *mut core::ffi::c_char, pbytes_written: *mut u32) -> core::ffi::c_int;
    pub fn smb3_query_mf_symlink(xid: u32, tcon: *mut cifs_tcon, cifs_sb: *mut cifs_sb_info, path: *const u8, pbuf: *mut core::ffi::c_char, pbytes_read: *mut u32) -> core::ffi::c_int;
    pub fn smb2_fix_symlink_target_type(target: *mut *mut core::ffi::c_char, directory: bool, cifs_sb: *mut cifs_sb_info) -> core::ffi::c_int;
    pub fn smb2_parse_native_symlink(target: *mut *mut core::ffi::c_char, buf: *const core::ffi::c_char, len: u32, relative: bool, full_path: *const core::ffi::c_char, cifs_sb: *mut cifs_sb_info) -> core::ffi::c_int;
    pub fn smb2_parse_symlink_response(cifs_sb: *mut cifs_sb_info, iov: *const kvec, full_path: *const core::ffi::c_char, path: *mut *mut core::ffi::c_char) -> core::ffi::c_int;
    pub fn smb2_open_file(xid: u32, oparms: *mut cifs_open_parms, oplock: *mut u32, buf: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn smb2_unlock_range(cfile: *mut cifsFileInfo, flock: *mut file_lock, xid: u32) -> core::ffi::c_int;
    pub fn smb2_push_mandatory_locks(cfile: *mut cifsFileInfo) -> core::ffi::c_int;
    pub fn smb2_reconnect_server(work: *mut work_struct);
    pub fn smb3_crypto_aead_allocate(server: *mut TCP_Server_Info) -> core::ffi::c_int;
    pub fn smb_rqst_len(server: *mut TCP_Server_Info, rqst: *mut smb_rqst) -> usize;
    pub fn smb2_set_next_command(tcon: *mut cifs_tcon, rqst: *mut smb_rqst);
    pub fn smb2_set_related(rqst: *mut smb_rqst);
    pub fn smb2_set_replay(server: *mut TCP_Server_Info, rqst: *mut smb_rqst);
    pub fn smb2_should_replay(tcon: *mut cifs_tcon, pretries: *mut core::ffi::c_int, pcur_sleep: *mut core::ffi::c_int) -> bool;

    /* SMB2 worker functions. */
    pub fn SMB2_negotiate(xid: u32, ses: *mut cifs_ses, server: *mut TCP_Server_Info) -> core::ffi::c_int;
    pub fn SMB2_sess_setup(xid: u32, ses: *mut cifs_ses, server: *mut TCP_Server_Info, nls_cp: *const nls_table) -> core::ffi::c_int;
    pub fn SMB2_logoff(xid: u32, ses: *mut cifs_ses) -> core::ffi::c_int;
    pub fn SMB2_tcon(xid: u32, ses: *mut cifs_ses, tree: *const core::ffi::c_char, tcon: *mut cifs_tcon, cp: *const nls_table) -> core::ffi::c_int;
    pub fn SMB2_tdis(xid: u32, tcon: *mut cifs_tcon) -> core::ffi::c_int;
    pub fn SMB2_open(xid: u32, oparms: *mut cifs_open_parms, path: *mut le16, oplock: *mut u8, buf: *mut cifs_open_info_data, posix: *mut create_posix_rsp, err_iov: *mut kvec, buftype: *mut core::ffi::c_int) -> core::ffi::c_int;
    pub fn SMB2_open_init(tcon: *mut cifs_tcon, server: *mut TCP_Server_Info, rqst: *mut smb_rqst, oplock: *mut u8, oparms: *mut cifs_open_parms, path: *mut le16) -> core::ffi::c_int;
    pub fn SMB2_open_free(rqst: *mut smb_rqst);
    pub fn SMB2_ioctl(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, opcode: u32, in_data: *mut core::ffi::c_char, indatalen: u32, max_out_data_len: u32, out_data: *mut *mut core::ffi::c_char, plen: *mut u32) -> core::ffi::c_int;
    pub fn SMB2_ioctl_init(tcon: *mut cifs_tcon, server: *mut TCP_Server_Info, rqst: *mut smb_rqst, persistent_fid: u64, volatile_fid: u64, opcode: u32, in_data: *mut core::ffi::c_char, indatalen: u32, max_response_size: u32) -> core::ffi::c_int;
    pub fn SMB2_ioctl_free(rqst: *mut smb_rqst);
    pub fn SMB2_change_notify(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, watch_tree: bool, completion_filter: u32, max_out_data_len: u32, out_data: *mut *mut core::ffi::c_char, plen: *mut u32) -> core::ffi::c_int;
    pub fn __SMB2_close(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, pbuf: *mut smb2_file_network_open_info) -> core::ffi::c_int;
    pub fn SMB2_close(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64) -> core::ffi::c_int;
    pub fn SMB2_close_init(tcon: *mut cifs_tcon, server: *mut TCP_Server_Info, rqst: *mut smb_rqst, persistent_fid: u64, volatile_fid: u64, query_attrs: bool) -> core::ffi::c_int;
    pub fn SMB2_close_free(rqst: *mut smb_rqst);
    pub fn SMB2_flush(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64) -> core::ffi::c_int;
    pub fn SMB2_flush_init(xid: u32, rqst: *mut smb_rqst, tcon: *mut cifs_tcon, server: *mut TCP_Server_Info, persistent_fid: u64, volatile_fid: u64) -> core::ffi::c_int;
    pub fn SMB2_flush_free(rqst: *mut smb_rqst);
    pub fn SMB2_query_info(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, data: *mut smb2_file_all_info) -> core::ffi::c_int;
    pub fn SMB2_query_info_init(tcon: *mut cifs_tcon, server: *mut TCP_Server_Info, rqst: *mut smb_rqst, persistent_fid: u64, volatile_fid: u64, info_class: u8, info_type: u8, additional_info: u32, output_len: usize, input_len: usize, input: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn SMB2_query_info_free(rqst: *mut smb_rqst);
    pub fn SMB2_query_acl(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, data: *mut *mut core::ffi::c_void, plen: *mut u32, extra_info: u32) -> core::ffi::c_int;
    pub fn SMB2_get_srv_num(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, uniqueid: *mut le64) -> core::ffi::c_int;
    pub fn smb2_async_readv(rdata: *mut cifs_io_subrequest) -> core::ffi::c_int;
    pub fn SMB2_read(xid: u32, io_parms: *mut cifs_io_parms, nbytes: *mut u32, buf: *mut *mut core::ffi::c_char, buf_type: *mut core::ffi::c_int) -> core::ffi::c_int;
    pub fn smb2_async_writev(wdata: *mut cifs_io_subrequest);
    pub fn SMB2_write(xid: u32, io_parms: *mut cifs_io_parms, nbytes: *mut u32, iov: *mut kvec, n_vec: core::ffi::c_int) -> core::ffi::c_int;
    pub fn SMB2_echo(server: *mut TCP_Server_Info) -> core::ffi::c_int;
    pub fn SMB2_query_directory(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, index: core::ffi::c_int, srch_inf: *mut cifs_search_info) -> core::ffi::c_int;
    pub fn SMB2_query_directory_init(xid: u32, tcon: *mut cifs_tcon, server: *mut TCP_Server_Info, rqst: *mut smb_rqst, persistent_fid: u64, volatile_fid: u64, index: core::ffi::c_int, info_level: core::ffi::c_int) -> core::ffi::c_int;
    pub fn SMB2_query_directory_free(rqst: *mut smb_rqst);
    pub fn SMB2_set_eof(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, pid: u32, new_eof: i64) -> core::ffi::c_int;
    pub fn SMB2_set_allocation(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, pid: u32, allocation_size: i64) -> core::ffi::c_int;
    pub fn SMB2_set_info_init(tcon: *mut cifs_tcon, server: *mut TCP_Server_Info, rqst: *mut smb_rqst, persistent_fid: u64, volatile_fid: u64, pid: u32, info_class: u8, info_type: u8, additional_info: u32, data: *mut *mut core::ffi::c_void, size: *mut u32) -> core::ffi::c_int;
    pub fn SMB2_set_info_free(rqst: *mut smb_rqst);
    pub fn SMB2_set_acl(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, pnntsd: *mut smb_ntsd, pacllen: core::ffi::c_int, aclflag: core::ffi::c_int) -> core::ffi::c_int;
    pub fn SMB2_set_ea(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, buf: *mut smb2_file_full_ea_info, len: core::ffi::c_int) -> core::ffi::c_int;
    pub fn SMB2_set_compression(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, compression_state: u16) -> core::ffi::c_int;
    pub fn SMB2_oplock_break(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, oplock_level: u8) -> core::ffi::c_int;
    pub fn smb2_handle_cancelled_close(tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64) -> core::ffi::c_int;
    pub fn smb2_handle_cancelled_mid(mid: *mut mid_q_entry, server: *mut TCP_Server_Info) -> core::ffi::c_int;
    pub fn smb2_cancelled_close_fid(work: *mut work_struct);
    pub fn SMB311_posix_qfs_info(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, fsdata: *mut kstatfs) -> core::ffi::c_int;
    pub fn SMB2_QFS_attr(xid: u32, tcon: *mut cifs_tcon, persistent_fid: u64, volatile_fid: u64, level: core::ffi::c_int) -> core::ffi::c_int;
    pub fn SMB2_lock(xid: u32, tcon: *mut cifs_tcon, persist_fid: u64, volatile_fid: u64, pid: u32, length: u64, offset: u64, lock_flags: u32, wait: bool) -> core::ffi::c_int;
    pub fn smb2_lockv(xid: u32, tcon: *mut cifs_tcon, persist_fid: u64, volatile_fid: u64, pid: u32, num_lock: u32, buf: *mut smb2_lock_element) -> core::ffi::c_int;
    pub fn SMB2_lease_break(xid: u32, tcon: *mut cifs_tcon, lease_key: *mut u8, lease_state: le32) -> core::ffi::c_int;
    pub fn smb3_validate_negotiate(xid: u32, tcon: *mut cifs_tcon) -> core::ffi::c_int;
    pub fn smb2_select_sectype(server: *mut TCP_Server_Info, requested: securityEnum) -> securityEnum;
    pub fn smb2_parse_contexts(server: *mut TCP_Server_Info, rsp_iov: *mut kvec, epoch: *mut u16, lease_key: *mut core::ffi::c_char, oplock: *mut u8, buf: *mut smb2_file_all_info, posix: *mut create_posix_rsp) -> core::ffi::c_int;
    pub fn smb3_encryption_required(tcon: *const cifs_tcon) -> core::ffi::c_int;
    pub fn smb2_validate_iov(offset: u32, buffer_length: u32, iov: *mut kvec, min_buf_size: u32) -> core::ffi::c_int;
    pub fn smb2_validate_and_copy_iov(offset: u32, buffer_length: u32, iov: *mut kvec, minbufsize: u32, data: *mut core::ffi::c_char) -> core::ffi::c_int;
    pub fn smb2_copy_fs_info_to_kstatfs(pfs_inf: *mut smb2_fs_full_size_info, kst: *mut kstatfs);
    pub fn smb311_update_preauth_hash(ses: *mut cifs_ses, server: *mut TCP_Server_Info, iov: *mut kvec, nvec: core::ffi::c_int);
    pub fn smb2_query_info_compound(xid: u32, tcon: *mut cifs_tcon, path: *const core::ffi::c_char, desired_access: u32, class: u32, type_: u32, output_len: u32, rsp: *mut kvec, buftype: *mut core::ffi::c_int, cifs_sb: *mut cifs_sb_info) -> core::ffi::c_int;
    pub fn posix_info_parse(beg: *const core::ffi::c_void, end: *const core::ffi::c_void, out: *mut smb2_posix_info_parsed) -> core::ffi::c_int;
    pub fn posix_info_sid_size(beg: *const core::ffi::c_void, end: *const core::ffi::c_void) -> core::ffi::c_int;
    pub fn smb2_rename_pending_delete(full_path: *const core::ffi::c_char, dentry: *mut dentry, xid: u32) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
