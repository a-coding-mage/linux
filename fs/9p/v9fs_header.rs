/* SPDX-License-Identifier: GPL-2.0-only */
/* V9FS definitions. */

// Kernel dependencies supplied by the surrounding translation unit:
// linux/backing-dev.h, linux/netfs.h, linux/fs_parser.h,
// net/9p/client.h, and net/9p/transport.h.

#[repr(u32)]
pub enum P9SessionFlags {
    V9FS_PROTO_2000U = 0x01,
    V9FS_PROTO_2000L = 0x02,
    V9FS_ACCESS_SINGLE = 0x04,
    V9FS_ACCESS_USER = 0x08,
    V9FS_ACCESS_CLIENT = 0x10,
    V9FS_POSIX_ACL = 0x20,
    V9FS_NO_XATTR = 0x40,
    V9FS_IGNORE_QV = 0x80,
    V9FS_DIRECT_IO = 0x100,
    V9FS_SYNC = 0x200,
    V9FS_NDENTRY_TIMEOUT_SET = 0x400,
}

pub const V9FS_ACCESS_ANY: u32 = V9FS_ACCESS_SINGLE as u32
    | V9FS_ACCESS_USER as u32
    | V9FS_ACCESS_CLIENT as u32;
pub const V9FS_ACCESS_MASK: u32 = V9FS_ACCESS_ANY;
pub const V9FS_ACL_MASK: u32 = V9FS_POSIX_ACL as u32;

#[repr(u8)]
pub enum P9CacheShortcuts {
    CACHE_SC_NONE = 0b00000000,
    CACHE_SC_READAHEAD = 0b00000001,
    CACHE_SC_MMAP = 0b00000101,
    CACHE_SC_LOOSE = 0b00001111,
    CACHE_SC_FSCACHE = 0b10001111,
}

#[repr(u8)]
pub enum P9CacheBits {
    CACHE_NONE = 0b00000000,
    CACHE_FILE = 0b00000001,
    CACHE_META = 0b00000010,
    CACHE_WRITEBACK = 0b00000100,
    CACHE_LOOSE = 0b00001000,
    CACHE_FSCACHE = 0b10000000,
}

#[repr(C)]
pub struct V9fsSessionInfo {
    pub flags: ::core::ffi::c_uint,
    pub nodev: ::core::ffi::c_uchar,
    pub debug: ::core::ffi::c_ushort,
    pub afid: ::core::ffi::c_uint,
    pub cache: ::core::ffi::c_uint,
    pub ndentry_timeout_ms: ::core::ffi::c_uint,
    #[cfg(CONFIG_9P_FSCACHE)]
    pub cachetag: *mut ::core::ffi::c_char,
    #[cfg(CONFIG_9P_FSCACHE)]
    pub fscache: *mut fscache_volume,
    pub uname: *mut ::core::ffi::c_char,
    pub aname: *mut ::core::ffi::c_char,
    pub maxdata: ::core::ffi::c_uint,
    pub dfltuid: kuid_t,
    pub dfltgid: kgid_t,
    pub uid: kuid_t,
    pub clnt: *mut p9_client,
    pub slist: list_head,
    pub rename_sem: rw_semaphore,
    pub session_lock_timeout: ::core::ffi::c_long,
}

pub const NDENTRY_TIMEOUT_NEVER: ::core::ffi::c_uint = !0;
pub const V9FS_INO_INVALID_ATTR: ::core::ffi::c_uint = 0x01;

#[repr(C)]
pub struct V9fsInode {
    pub netfs: netfs_inode,
    pub qid: p9_qid,
    pub cache_validity: ::core::ffi::c_uint,
    pub v_mutex: mutex,
}

#[inline]
pub unsafe fn V9FS_I(inode: *const inode) -> *mut V9fsInode {
    container_of!(inode, V9fsInode, netfs.inode)
}

#[inline]
pub unsafe fn v9fs_inode_cookie(v9inode: *mut V9fsInode) -> *mut fscache_cookie {
    #[cfg(CONFIG_9P_FSCACHE)]
    { return netfs_i_cookie(&mut (*v9inode).netfs); }
    #[cfg(not(CONFIG_9P_FSCACHE))]
    { let _ = v9inode; ::core::ptr::null_mut() }
}

#[inline]
pub unsafe fn v9fs_session_cache(v9ses: *mut V9fsSessionInfo) -> *mut fscache_volume {
    #[cfg(CONFIG_9P_FSCACHE)]
    { return (*v9ses).fscache; }
    #[cfg(not(CONFIG_9P_FSCACHE))]
    { let _ = v9ses; ::core::ptr::null_mut() }
}

extern "C" {
    pub static v9fs_param_spec: [fs_parameter_spec; 0];
    pub fn v9fs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> ::core::ffi::c_int;
    pub fn v9fs_show_options(m: *mut seq_file, root: *mut dentry) -> ::core::ffi::c_int;
    pub fn v9fs_session_init(v9ses: *mut V9fsSessionInfo, fc: *mut fs_context) -> *mut p9_fid;
    pub fn v9fs_session_close(v9ses: *mut V9fsSessionInfo);
    pub fn v9fs_session_cancel(v9ses: *mut V9fsSessionInfo);
    pub fn v9fs_session_begin_cancel(v9ses: *mut V9fsSessionInfo);
    pub fn v9fs_vfs_lookup(dir: *mut inode, dentry: *mut dentry, flags: ::core::ffi::c_uint) -> *mut dentry;
    pub fn v9fs_vfs_unlink(i: *mut inode, d: *mut dentry) -> ::core::ffi::c_int;
    pub fn v9fs_vfs_rmdir(i: *mut inode, d: *mut dentry) -> ::core::ffi::c_int;
    pub fn v9fs_vfs_rename(idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry, flags: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn v9fs_inode_from_fid(v9ses: *mut V9fsSessionInfo, fid: *mut p9_fid, sb: *mut super_block, new_: ::core::ffi::c_int) -> *mut inode;
    pub static v9fs_dir_inode_operations_dotl: inode_operations;
    pub static v9fs_file_inode_operations_dotl: inode_operations;
    pub static v9fs_symlink_inode_operations_dotl: inode_operations;
    pub static v9fs_req_ops: netfs_request_ops;
    pub fn v9fs_inode_from_fid_dotl(v9ses: *mut V9fsSessionInfo, fid: *mut p9_fid, sb: *mut super_block, new_: ::core::ffi::c_int) -> *mut inode;
}

pub const V9FS_PORT: ::core::ffi::c_uint = 564;
pub const V9FS_DEFUSER: &[u8] = b"nobody\0";
pub const V9FS_DEFANAME: &[u8] = b"\0";
// KUIDT_INIT(-2) and KGIDT_INIT(-2) are supplied by the kernel type layer.
pub const V9FS_DEFUID: i32 = -2;
pub const V9FS_DEFGID: i32 = -2;

#[inline]
pub unsafe fn v9fs_inode2v9ses(inode: *mut inode) -> *mut V9fsSessionInfo {
    (*(*inode).i_sb).s_fs_info as *mut V9fsSessionInfo
}

#[inline]
pub unsafe fn v9fs_dentry2v9ses(dentry: *const dentry) -> *mut V9fsSessionInfo {
    (*(*dentry).d_sb).s_fs_info as *mut V9fsSessionInfo
}

#[inline]
pub unsafe fn v9fs_proto_dotu(v9ses: *mut V9fsSessionInfo) -> ::core::ffi::c_int {
    ((*v9ses).flags & V9FS_PROTO_2000U as u32) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn v9fs_proto_dotl(v9ses: *mut V9fsSessionInfo) -> ::core::ffi::c_int {
    ((*v9ses).flags & V9FS_PROTO_2000L as u32) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn v9fs_get_inode_from_fid(v9ses: *mut V9fsSessionInfo, fid: *mut p9_fid, sb: *mut super_block) -> *mut inode {
    if v9fs_proto_dotl(v9ses) != 0 { v9fs_inode_from_fid_dotl(v9ses, fid, sb, 0) } else { v9fs_inode_from_fid(v9ses, fid, sb, 0) }
}

#[inline]
pub unsafe fn v9fs_get_new_inode_from_fid(v9ses: *mut V9fsSessionInfo, fid: *mut p9_fid, sb: *mut super_block) -> *mut inode {
    if v9fs_proto_dotl(v9ses) != 0 { v9fs_inode_from_fid_dotl(v9ses, fid, sb, 1) } else { v9fs_inode_from_fid(v9ses, fid, sb, 1) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
