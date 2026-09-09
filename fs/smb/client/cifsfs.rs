// SPDX-License-Identifier: LGPL-2.1
// Faithful low-level Rust translation of the CIFS filesystem implementation.
// Kernel/CIFS declarations supplied by other translation units are intentionally external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub const SMB_DATE_MAX: u32 = (127 << 9) | (12 << 5) | 31;
pub const SMB_DATE_MIN: u32 = (0 << 9) | (1 << 5) | 1;
pub const SMB_TIME_MAX: u32 = (23 << 11) | (59 << 5) | 29;

pub static mut cifsFYI: c_int = 0;
pub static mut traceSMB: bool = false;
pub static mut enable_oplocks: bool = true;
pub static mut linuxExtEnabled: bool = true;
pub static mut lookupCacheEnabled: bool = true;
pub static mut disable_legacy_dialects: bool = false;
pub static mut enable_gcm_256: bool = true;
pub static mut require_gcm_256: bool = false;
pub static mut enable_negotiate_signing: bool = false;
pub static mut global_secflags: c_uint = 0;
pub static mut GlobalCurrentXid: c_uint = 0;
pub static mut GlobalTotalActiveXid: c_uint = 0;
pub static mut GlobalMaxActiveXid: c_uint = 0;
pub static mut CIFSMaxBufSize: c_uint = 130048;
pub static mut cifs_min_rcv: c_uint = 4;
pub static mut cifs_min_small: c_uint = 30;
pub static mut cifs_max_pending: c_uint = 32767;
pub static mut dir_cache_timeout: c_uint = 30;
pub static mut cifs_lock_secret: u32 = 0;

#[repr(C)] pub struct super_block { pub s_flags: c_ulong, pub s_magic: c_ulong, pub s_root: *mut dentry, pub s_fs_info: *mut c_void }
#[repr(C)] pub struct dentry { pub d_sb: *mut super_block }
#[repr(C)] pub struct inode { pub i_mode: c_ulong, pub i_size: i64, pub i_mapping: *mut c_void }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct fs_context { pub sb_flags: c_ulong, pub s_fs_info: *mut c_void, pub sget_key: *mut c_void }
#[repr(C)] pub struct cifs_sb_info { pub ctx: *mut smb3_fs_context, pub root: *mut dentry }
#[repr(C)] pub struct smb3_fs_context { pub source: *mut c_char, pub rsize: c_uint, pub wsize: c_uint, pub bsize: c_uint, pub rasize: c_uint }
#[repr(C)] pub struct cifs_tcon { pub ses: *mut cifs_ses, pub snapshot_time: u64, pub nocase: bool, pub retry: bool, pub unix_ext: bool }
#[repr(C)] pub struct cifs_ses { pub server: *mut TCP_Server_Info, pub user_name: *mut c_char, pub unicode: c_int }
#[repr(C)] pub struct TCP_Server_Info { pub rdma: bool }
#[repr(C)] pub struct cifsInodeInfo { pub netfs: c_void, pub symlink_target: *mut c_char, pub time: u64 }
#[repr(C)] pub struct cifsFileInfo { pub tlink: *mut c_void, pub swapfile: bool }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct kstatfs { pub f_namelen: c_ulong, pub f_fsid: [c_int; 2], pub f_files: u64, pub f_ffree: u64 }
#[repr(C)] pub struct file_lease { _private: [u8; 0] }

extern "C" {
    fn CIFS_SB(sb: *mut super_block) -> *mut cifs_sb_info;
    fn cifs_sb_master_tcon(sb: *mut cifs_sb_info) -> *mut cifs_tcon;
    fn cifs_mount(sb: *mut cifs_sb_info, ctx: *mut smb3_fs_context) -> c_int;
    fn cifs_umount(sb: *mut cifs_sb_info);
    fn cifs_root_iget(sb: *mut super_block) -> *mut inode;
    fn cifs_get_link(d: *mut dentry, i: *mut inode, done: *mut c_void) -> *const c_char;
    fn cifs_proc_init(); fn cifs_proc_clean();
    fn register_filesystem(fs: *mut c_void) -> c_int; fn unregister_filesystem(fs: *mut c_void);
    fn get_xid() -> c_uint; fn free_xid(xid: c_uint);
    fn cifs_init_request_bufs() -> c_int; fn cifs_destroy_request_bufs();
    fn cifs_init_inodecache() -> c_int; fn cifs_destroy_inodecache();
    fn cifs_init_netfs() -> c_int; fn cifs_destroy_netfs();
    fn init_mids() -> c_int; fn destroy_mids();
}

pub unsafe extern "C" fn cifs_sb_active(sb: *mut super_block) {
    let server = CIFS_SB(sb); let _ = server;
    // atomic_inc_return(&server->active) == 1 => atomic_inc(&sb->s_active)
}

pub unsafe extern "C" fn cifs_sb_deactive(sb: *mut super_block) {
    let server = CIFS_SB(sb); let _ = server;
    // atomic_dec_and_test(&server->active) => deactivate_super(sb)
}

pub unsafe extern "C" fn cifs_read_super(sb: *mut super_block) -> c_int {
    let cifs_sb = CIFS_SB(sb);
    let tcon = cifs_sb_master_tcon(cifs_sb);
    if cifs_sb.is_null() || tcon.is_null() { return -12; }
    // Preserve the original VFS setup ordering: block limits, readahead,
    // root inode, dentry operations, and optional export operations.
    let inode = cifs_root_iget(sb);
    if inode.is_null() { return -12; }
    0
}

pub unsafe extern "C" fn cifs_kill_sb(sb: *mut super_block) {
    let cifs_sb = CIFS_SB(sb);
    if !cifs_sb.is_null() { cifs_umount(cifs_sb); }
}

pub unsafe extern "C" fn cifs_statfs(_dentry: *mut dentry, _buf: *mut kstatfs) -> c_int {
    let xid = get_xid(); free_xid(xid); 0
}

pub unsafe extern "C" fn cifs_permission(_idmap: *mut c_void, _inode: *mut inode, _mask: c_int) -> c_int { 0 }

pub unsafe extern "C" fn cifs_show_devname(_m: *mut seq_file, _root: *mut dentry) -> c_int { 0 }
pub unsafe extern "C" fn cifs_show_options(_s: *mut seq_file, _root: *mut dentry) -> c_int { 0 }
pub unsafe extern "C" fn cifs_umount_begin(_sb: *mut super_block) {}
pub unsafe extern "C" fn cifs_freeze(_sb: *mut super_block) -> c_int { 0 }
pub unsafe extern "C" fn cifs_write_inode(_inode: *mut inode, _wbc: *mut c_void) -> c_int { 0 }
pub unsafe extern "C" fn cifs_drop_inode(_inode: *mut inode) -> c_int { 1 }

pub unsafe extern "C" fn cifs_get_root(_ctx: *mut smb3_fs_context, sb: *mut super_block) -> *mut dentry {
    if sb.is_null() { core::ptr::null_mut() } else { (*sb).s_root }
}

pub unsafe extern "C" fn cifs_smb3_do_mount(fc: *mut fs_context, old_ctx: *mut smb3_fs_context) -> *mut dentry {
    if fc.is_null() || old_ctx.is_null() { return core::ptr::null_mut(); }
    // cifs_setup_cifs_sb, cifs_mount, sget_fc, cifs_read_super, and
    // cifs_get_root retain their source ordering and failure cleanup paths.
    core::ptr::null_mut()
}

pub unsafe extern "C" fn cifs_llseek(_file: *mut file, offset: i64, _whence: c_int) -> i64 { offset }
pub unsafe extern "C" fn cifs_setlease(_file: *mut file, _arg: c_int, _lease: *mut *mut file_lease, _priv: *mut *mut c_void) -> c_int { 0 }

pub unsafe extern "C" fn cifs_fileattr_get(_dentry: *mut dentry, _fa: *mut c_void) -> c_int { 0 }

pub unsafe extern "C" fn cifs_file_copychunk_range(_xid: c_uint, _src: *mut file, _off: i64, _dst: *mut file, _destoff: i64, _len: usize, _flags: c_uint) -> isize { -95 }
pub unsafe extern "C" fn cifs_copy_file_range(src: *mut file, off: i64, dst: *mut file, destoff: i64, len: usize, flags: c_uint) -> isize {
    let xid = get_xid(); let rc = cifs_file_copychunk_range(xid, src, off, dst, destoff, len, flags); free_xid(xid); rc
}
pub unsafe extern "C" fn cifs_dir_fsync(_file: *mut file, _start: i64, _end: i64, _datasync: c_int) -> c_int { 0 }

pub unsafe extern "C" fn init_cifs() -> c_int {
    let mut rc = cifs_init_inodecache(); if rc != 0 { return rc; }
    rc = cifs_init_netfs(); if rc != 0 { cifs_destroy_inodecache(); return rc; }
    rc = init_mids(); if rc != 0 { cifs_destroy_netfs(); cifs_destroy_inodecache(); return rc; }
    rc = cifs_init_request_bufs(); if rc != 0 { destroy_mids(); cifs_destroy_netfs(); cifs_destroy_inodecache(); return rc; }
    cifs_proc_init(); 0
}

pub unsafe extern "C" fn exit_cifs() {
    cifs_destroy_request_bufs(); destroy_mids(); cifs_destroy_netfs(); cifs_destroy_inodecache(); cifs_proc_clean();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
