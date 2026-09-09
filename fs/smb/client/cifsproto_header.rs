/* SPDX-License-Identifier: LGPL-2.1 */
/* Source-level Rust translation of cifsproto.h. */

// Linux/kernel types, constants, macros, and declarations below are supplied by
// the corresponding translated dependency headers.

extern "C" {
    pub fn cifs_buf_get() -> *mut core::ffi::c_void;
    pub fn cifs_buf_release(buf_to_free: *mut core::ffi::c_void);
    pub fn cifs_small_buf_get() -> *mut core::ffi::c_void;
    pub fn cifs_small_buf_release(buf_to_free: *mut core::ffi::c_void);
    pub fn free_rsp_buf(resp_buftype: i32, rsp: *mut core::ffi::c_void);
    pub fn smb_send_kvec(server: *mut TCP_Server_Info, smb_msg: *mut msghdr, sent: *mut usize) -> i32;
    pub fn _get_xid() -> u32;
    pub fn _free_xid(xid: u32);
    pub fn init_cifs_idmap() -> i32;
    pub fn exit_cifs_idmap();
    pub fn init_cifs_spnego() -> i32;
    pub fn exit_cifs_spnego();
    pub fn build_path_from_dentry(direntry: *mut dentry, page: *mut core::ffi::c_void) -> *const i8;
    pub fn __build_path_from_dentry_optional_prefix(direntry: *mut dentry, page: *mut core::ffi::c_void, tree: *const i8, tree_len: i32, prefix: bool) -> *mut i8;
    pub fn build_path_from_dentry_optional_prefix(direntry: *mut dentry, page: *mut core::ffi::c_void, prefix: bool) -> *mut i8;
    pub fn cifs_build_path_to_root(ctx: *mut smb3_fs_context, cifs_sb: *mut cifs_sb_info, tcon: *mut cifs_tcon, add_treename: i32) -> *mut i8;
    pub fn cifs_build_devname(nodename: *mut i8, prepath: *const i8) -> *mut i8;
    pub fn delete_mid(server: *mut TCP_Server_Info, mid: *mut mid_q_entry);
    pub fn __release_mid(server: *mut TCP_Server_Info, mid: *mut mid_q_entry);
    pub fn cifs_wake_up_task(server: *mut TCP_Server_Info, mid: *mut mid_q_entry);
    pub fn cifs_handle_standard(server: *mut TCP_Server_Info, mid: *mut mid_q_entry) -> i32;
    pub fn smb3_fs_context_fullpath(ctx: *const smb3_fs_context, dirsep: i8) -> *mut i8;
    pub fn smb3_parse_devname(devname: *const i8, ctx: *mut smb3_fs_context) -> i32;
    pub fn cifs_ipaddr_cmp(srcaddr: *mut sockaddr, rhs: *mut sockaddr) -> i32;
    pub fn cifs_match_ipaddr(srcaddr: *mut sockaddr, rhs: *mut sockaddr) -> bool;
    pub fn cifs_discard_remaining_data(server: *mut TCP_Server_Info) -> i32;
    pub fn cifs_call_async(server: *mut TCP_Server_Info, rqst: *mut smb_rqst, receive: mid_receive_t, callback: mid_callback_t, handle: mid_handle_t, cbdata: *mut core::ffi::c_void, flags: i32, exist_credits: *const cifs_credits) -> i32;
    pub fn cifs_pick_channel(ses: *mut cifs_ses) -> *mut TCP_Server_Info;
    pub fn cifs_send_recv(xid: u32, ses: *mut cifs_ses, server: *mut TCP_Server_Info, rqst: *mut smb_rqst, resp_buf_type: *mut i32, flags: i32, resp_iov: *mut kvec) -> i32;
    pub fn compound_send_recv(xid: u32, ses: *mut cifs_ses, server: *mut TCP_Server_Info, flags: i32, num_rqst: i32, rqst: *mut smb_rqst, resp_buf_type: *mut i32, resp_iov: *mut kvec) -> i32;
    pub fn cifs_sync_mid_result(mid: *mut mid_q_entry, server: *mut TCP_Server_Info) -> i32;
    pub fn __smb_send_rqst(server: *mut TCP_Server_Info, num_rqst: i32, rqst: *mut smb_rqst) -> i32;
    pub fn wait_for_free_request(server: *mut TCP_Server_Info, flags: i32, instance: *mut u32) -> i32;
    pub fn cifs_wait_mtu_credits(server: *mut TCP_Server_Info, size: usize, num: *mut usize, credits: *mut cifs_credits) -> i32;
    pub fn wait_for_response(server: *mut TCP_Server_Info, mid: *mut mid_q_entry) -> i32;
    pub fn cifs_reconnect(server: *mut TCP_Server_Info, mark_smb_session: bool) -> i32;
    pub fn backup_cred(cifs_sb: *mut cifs_sb_info) -> bool;
    pub fn __cifs_get_writable_file(cifs_inode: *mut cifsInodeInfo, find_flags: u32, open_flags: u32, ret_file: *mut *mut cifsFileInfo) -> i32;
    pub fn __find_readable_file(cifs_inode: *mut cifsInodeInfo, find_flags: u32, open_flags: u32) -> *mut cifsFileInfo;
    pub fn __cifs_put_smb_ses(ses: *mut cifs_ses);
}

// Opaque dependency types (provided by the translated kernel/CIFS headers).
#[allow(non_camel_case_types)]
pub enum TCP_Server_Info {}
pub enum msghdr {}
pub enum dentry {}
pub enum smb3_fs_context {}
pub enum cifs_sb_info {}
pub enum cifs_tcon {}
pub enum mid_q_entry {}
pub enum smb_rqst {}
pub enum cifs_ses {}
pub enum cifs_credits {}
pub enum kvec {}
pub enum cifsInodeInfo {}
pub enum cifsFileInfo {}

#[inline]
pub unsafe fn alloc_dentry_path() -> *mut core::ffi::c_void { __getname() }
#[inline]
pub unsafe fn free_dentry_path(page: *mut core::ffi::c_void) { if !page.is_null() { __putname(page); } }
#[inline]
pub unsafe fn send_cancel(ses: *mut cifs_ses, server: *mut TCP_Server_Info, rqst: *mut smb_rqst, mid: *mut mid_q_entry, xid: u32) -> i32 {
    // `server->ops->send_cancel ? ... : 0`; layout is supplied by the dependency.
    let _ = (ses, server, rqst, mid, xid); 0
}
#[inline]
pub unsafe fn cifs_get_writable_file(cifs_inode: *mut cifsInodeInfo, mut find_flags: u32, ret_file: *mut *mut cifsFileInfo) -> i32 {
    find_flags &= !FIND_OPEN_FLAGS;
    __cifs_get_writable_file(cifs_inode, find_flags, 0, ret_file)
}
#[inline]
pub unsafe fn find_readable_file(cinode: *mut cifsInodeInfo, mut find_flags: u32) -> *mut cifsFileInfo {
    find_flags &= !FIND_OPEN_FLAGS;
    find_flags |= FIND_NO_PENDING_DELETE;
    __find_readable_file(cinode, find_flags, 0)
}
#[inline]
pub unsafe fn cifs_put_smb_ses(ses: *mut cifs_ses) { __cifs_put_smb_ses(ses); }

// The remaining prototypes retain their C ABI and dependency-defined types.
extern "C" {
    pub fn smb2_query_server_interfaces(work: *mut work_struct);
    pub fn cifs_signal_cifsd_for_reconnect(server: *mut TCP_Server_Info, all_channels: bool);
    pub fn cifs_mark_tcp_ses_conns_for_reconnect(server: *mut TCP_Server_Info, mark_smb_session: bool);
    pub fn cifs_mount(cifs_sb: *mut cifs_sb_info, ctx: *mut smb3_fs_context) -> i32;
    pub fn cifs_umount(cifs_sb: *mut cifs_sb_info);
    pub fn cifs_tree_connect(xid: u32, tcon: *mut cifs_tcon) -> i32;
    pub fn cifs_negotiate_protocol(xid: u32, ses: *mut cifs_ses, server: *mut TCP_Server_Info) -> i32;
    pub fn cifs_setup_ipc(ses: *mut cifs_ses, seal: bool) -> *mut cifs_tcon;
    pub fn cifs_find_tcp_session(ctx: *mut smb3_fs_context) -> *mut TCP_Server_Info;
    pub fn cifs_readv_receive(server: *mut TCP_Server_Info, mid: *mut mid_q_entry) -> i32;
    pub fn cifs_try_adding_channels(ses: *mut cifs_ses) -> i32;
    pub fn cifs_down_write(sem: *mut rw_semaphore);
    pub fn cifs_new_fileinfo(fid: *mut cifs_fid, file: *mut file, tlink: *mut tcon_link, oplock: u32, symlink_target: *const i8) -> *mut cifsFileInfo;
    pub fn cifs_get_inode_info(inode: *mut *mut inode, full_path: *const i8, data: *mut cifs_open_info_data, sb: *mut super_block, xid: i32, fid: *const cifs_fid) -> i32;
    pub fn cifs_set_file_info(inode: *mut inode, attrs: *mut iattr, xid: u32, full_path: *const i8, dosattr: u32) -> i32;
    pub fn cifs_get_acl(cifs_sb: *mut cifs_sb_info, inode: *mut inode, path: *const i8, pacllen: *mut u32, info: u32) -> *mut smb_ntsd;
    pub fn cifs_set_acl(idmap: *mut mnt_idmap, dentry: *mut dentry, acl: *mut posix_acl, kind: i32) -> i32;
    pub fn cifs_read_from_socket(server: *mut TCP_Server_Info, buf: *mut i8, to_read: u32) -> i32;
    pub fn cifs_discard_from_socket(server: *mut TCP_Server_Info, to_read: usize) -> isize;
    pub fn cifs_setup_cifs_sb(cifs_sb: *mut cifs_sb_info) -> i32;
    pub fn cifs_mount_get_session(mnt_ctx: *mut cifs_mount_ctx) -> i32;
    pub fn cifs_mount_get_tcon(mnt_ctx: *mut cifs_mount_ctx) -> i32;
    pub fn cifs_match_super(sb: *mut super_block, fc: *mut fs_context) -> i32;
    pub fn cifs_put_tcp_session(server: *mut TCP_Server_Info, from_reconnect: i32);
    pub fn cifs_put_tcon(tcon: *mut cifs_tcon, trace: smb3_tcon_ref_trace);
    pub fn cifs_release_automount_timer();
    pub fn cifs_proc_init();
    pub fn cifs_proc_clean();
    pub fn cifs_free_llist(llist: *mut list_head);
    pub fn cifs_del_lock_waiters(lock: *mut cifsLockInfo);
    pub fn sesInfoAlloc() -> *mut cifs_ses;
    pub fn sesInfoFree(ses: *mut cifs_ses);
    pub fn tcon_info_alloc(dir_leases_enabled: bool, trace: smb3_tcon_ref_trace) -> *mut cifs_tcon;
    pub fn tconInfoFree(tcon: *mut cifs_tcon, trace: smb3_tcon_ref_trace);
    pub fn setup_ntlmv2_rsp(ses: *mut cifs_ses, nls_cp: *const nls_table) -> i32;
    pub fn calc_seckey(ses: *mut cifs_ses) -> i32;
    pub fn cifs_crypto_secmech_release(server: *mut TCP_Server_Info);
    pub fn extract_unc_hostname(unc: *const i8, h: *mut *const i8, len: *mut usize);
    pub fn copy_path_name(dst: *mut i8, src: *const i8) -> i32;
    pub fn extract_hostname(unc: *const i8) -> *mut i8;
    pub fn extract_sharename(unc: *const i8) -> *mut i8;
    pub fn wire_mode_to_posix(wire: u32, is_dir: bool) -> umode_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
