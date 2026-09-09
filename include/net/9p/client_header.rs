/* SPDX-License-Identifier: GPL-2.0-only */
/* 9P Client Definitions */

pub const P9_ROW_MAXTAG: u32 = 255;
pub const DEFAULT_MSIZE: u32 = (128 * 1024) + P9_IOHDRSZ;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum p9_proto_versions {
    p9_proto_legacy,
    p9_proto_2000u,
    p9_proto_2000L,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum p9_trans_status {
    Connected,
    BeginDisconnect,
    Disconnected,
    Hung,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum p9_req_status_t {
    REQ_STATUS_ALLOC,
    REQ_STATUS_UNSENT,
    REQ_STATUS_SENT,
    REQ_STATUS_RCVD,
    REQ_STATUS_FLSHD,
    REQ_STATUS_ERROR,
}

#[repr(C)]
pub struct p9_req_t {
    pub status: core::ffi::c_int,
    pub t_err: core::ffi::c_int,
    pub refcount: refcount_t,
    pub wq: wait_queue_head_t,
    pub tc: p9_fcall,
    pub rc: p9_fcall,
    pub req_list: list_head,
}

#[repr(C)]
pub union p9_client_trans_opts {
    pub fd: p9_client_trans_opts_fd,
    pub tcp: p9_client_trans_opts_tcp,
}

#[repr(C)]
pub struct p9_client_trans_opts_fd { pub rfd: core::ffi::c_int, pub wfd: core::ffi::c_int }

#[repr(C)]
pub struct p9_client_trans_opts_tcp { pub port: u16, pub privport: bool }

#[repr(C)]
pub struct p9_client {
    pub lock: spinlock_t,
    pub msize: u32,
    pub proto_version: u8,
    pub trans_mod: *mut p9_trans_module,
    pub status: p9_trans_status,
    pub trans: *mut core::ffi::c_void,
    pub fcall_cache: *mut kmem_cache,
    pub trans_opts: p9_client_trans_opts,
    pub fids: idr,
    pub reqs: idr,
    pub name: [core::ffi::c_char; __NEW_UTS_LEN + 1],
}

#[repr(C)]
pub struct p9_client_opts {
    pub msize: u32,
    pub proto_version: u8,
    pub trans_mod: *mut p9_trans_module,
}

#[repr(C)]
pub struct p9_fd_opts { pub rfd: core::ffi::c_int, pub wfd: core::ffi::c_int, pub port: u16, pub privport: bool }

#[repr(C)]
pub struct p9_rdma_opts { pub port: i16, pub privport: bool, pub sq_depth: core::ffi::c_int, pub rq_depth: core::ffi::c_int, pub timeout: core::ffi::c_long }

#[repr(C)]
pub struct p9_session_opts {
    pub flags: u32,
    pub nodev: u8,
    pub debug: u16,
    pub afid: u32,
    pub cache: u32,
    pub ndentry_timeout_ms: u32,
    #[cfg(feature = "CONFIG_9P_FSCACHE")]
    pub cachetag: *mut core::ffi::c_char,
    pub uname: *mut core::ffi::c_char,
    pub aname: *mut core::ffi::c_char,
    pub dfltuid: kuid_t,
    pub dfltgid: kgid_t,
    pub uid: kuid_t,
    pub session_lock_timeout: core::ffi::c_long,
}

#[repr(C)]
pub struct v9fs_context {
    pub client_opts: p9_client_opts,
    pub fd_opts: p9_fd_opts,
    pub rdma_opts: p9_rdma_opts,
    pub session_opts: p9_session_opts,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fid_source { FID_FROM_OTHER, FID_FROM_INODE, FID_FROM_DENTRY }

#[repr(C)]
pub struct p9_fid {
    pub clnt: *mut p9_client,
    pub fid: u32,
    pub count: refcount_t,
    pub mode: core::ffi::c_int,
    pub qid: p9_qid,
    pub iounit: u32,
    pub uid: kuid_t,
    pub rdir: *mut core::ffi::c_void,
    pub dlist: hlist_node,
    pub ilist: hlist_node,
}

#[repr(C)]
pub struct p9_dirent { pub qid: p9_qid, pub d_off: u64, pub d_type: u8, pub d_name: [core::ffi::c_char; 256] }

pub struct iov_iter;
pub struct netfs_io_subrequest;

extern "C" {
    pub fn p9_show_client_options(m: *mut seq_file, clnt: *mut p9_client) -> core::ffi::c_int;
    pub fn p9_client_statfs(fid: *mut p9_fid, sb: *mut p9_rstatfs) -> core::ffi::c_int;
    pub fn p9_client_rename(fid: *mut p9_fid, newdirfid: *mut p9_fid, name: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn p9_client_renameat(olddirfid: *mut p9_fid, old_name: *const core::ffi::c_char, newdirfid: *mut p9_fid, new_name: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn p9_client_create(fc: *mut fs_context) -> *mut p9_client;
    pub fn p9_client_destroy(clnt: *mut p9_client);
    pub fn p9_client_disconnect(clnt: *mut p9_client);
    pub fn p9_client_begin_disconnect(clnt: *mut p9_client);
    pub fn p9_client_attach(clnt: *mut p9_client, afid: *mut p9_fid, uname: *const core::ffi::c_char, n_uname: kuid_t, aname: *const core::ffi::c_char) -> *mut p9_fid;
    pub fn p9_client_walk(oldfid: *mut p9_fid, nwname: u16, wnames: *const *const u8, clone: core::ffi::c_int) -> *mut p9_fid;
    pub fn p9_client_open(fid: *mut p9_fid, mode: core::ffi::c_int) -> core::ffi::c_int;
    pub fn p9_client_fcreate(fid: *mut p9_fid, name: *const core::ffi::c_char, perm: u32, mode: core::ffi::c_int, extension: *mut core::ffi::c_char) -> core::ffi::c_int;
    pub fn p9_client_link(fid: *mut p9_fid, oldfid: *mut p9_fid, newname: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn p9_client_symlink(fid: *mut p9_fid, name: *const core::ffi::c_char, symname: *const core::ffi::c_char, gid: kgid_t, qid: *mut p9_qid) -> core::ffi::c_int;
    pub fn p9_client_create_dotl(ofid: *mut p9_fid, name: *const core::ffi::c_char, flags: u32, mode: u32, gid: kgid_t, qid: *mut p9_qid) -> core::ffi::c_int;
    pub fn p9_client_clunk(fid: *mut p9_fid) -> core::ffi::c_int;
    pub fn p9_client_fsync(fid: *mut p9_fid, datasync: core::ffi::c_int) -> core::ffi::c_int;
    pub fn p9_client_remove(fid: *mut p9_fid) -> core::ffi::c_int;
    pub fn p9_client_unlinkat(dfid: *mut p9_fid, name: *const core::ffi::c_char, flags: core::ffi::c_int) -> core::ffi::c_int;
    pub fn p9_client_read(fid: *mut p9_fid, offset: u64, to: *mut iov_iter, err: *mut core::ffi::c_int) -> core::ffi::c_int;
    pub fn p9_client_read_once(fid: *mut p9_fid, offset: u64, to: *mut iov_iter, err: *mut core::ffi::c_int) -> core::ffi::c_int;
    pub fn p9_client_write(fid: *mut p9_fid, offset: u64, from: *mut iov_iter, err: *mut core::ffi::c_int) -> core::ffi::c_int;
    pub fn p9_client_write_subreq(subreq: *mut netfs_io_subrequest);
    pub fn p9_client_readdir(fid: *mut p9_fid, data: *mut core::ffi::c_char, count: u32, offset: u64) -> core::ffi::c_int;
    pub fn p9dirent_read(clnt: *mut p9_client, buf: *mut core::ffi::c_char, len: core::ffi::c_int, dirent: *mut p9_dirent) -> core::ffi::c_int;
    pub fn p9_client_stat(fid: *mut p9_fid) -> *mut p9_wstat;
    pub fn p9_client_wstat(fid: *mut p9_fid, wst: *mut p9_wstat) -> core::ffi::c_int;
    pub fn p9_client_setattr(fid: *mut p9_fid, attr: *mut p9_iattr_dotl) -> core::ffi::c_int;
    pub fn p9_client_getattr_dotl(fid: *mut p9_fid, request_mask: u64) -> *mut p9_stat_dotl;
    pub fn p9_client_mknod_dotl(oldfid: *mut p9_fid, name: *const core::ffi::c_char, mode: core::ffi::c_int, rdev: dev_t, gid: kgid_t, qid: *mut p9_qid) -> core::ffi::c_int;
    pub fn p9_client_mkdir_dotl(fid: *mut p9_fid, name: *const core::ffi::c_char, mode: core::ffi::c_int, gid: kgid_t, qid: *mut p9_qid) -> core::ffi::c_int;
    pub fn p9_client_lock_dotl(fid: *mut p9_fid, flock: *mut p9_flock, status: *mut u8) -> core::ffi::c_int;
    pub fn p9_client_getlock_dotl(fid: *mut p9_fid, fl: *mut p9_getlock) -> core::ffi::c_int;
    pub fn p9_fcall_fini(fc: *mut p9_fcall);
    pub fn p9_tag_lookup(c: *mut p9_client, tag: u16) -> *mut p9_req_t;
    pub fn p9_req_put(c: *mut p9_client, r: *mut p9_req_t) -> core::ffi::c_int;
    pub fn do_trace_9p_fid_get(fid: *mut p9_fid);
    pub fn do_trace_9p_fid_put(fid: *mut p9_fid);
    pub fn p9_client_cb(c: *mut p9_client, req: *mut p9_req_t, status: core::ffi::c_int);
    pub fn p9_parse_header(pdu: *mut p9_fcall, size: *mut i32, type_: *mut i8, tag: *mut i16, rewind: core::ffi::c_int) -> core::ffi::c_int;
    pub fn p9stat_read(clnt: *mut p9_client, buf: *mut core::ffi::c_char, len: core::ffi::c_int, st: *mut p9_wstat) -> core::ffi::c_int;
    pub fn p9stat_free(stbuf: *mut p9_wstat);
    pub fn p9_is_proto_dotu(clnt: *mut p9_client) -> core::ffi::c_int;
    pub fn p9_is_proto_dotl(clnt: *mut p9_client) -> core::ffi::c_int;
    pub fn p9_client_xattrwalk(file_fid: *mut p9_fid, attr_name: *const core::ffi::c_char, attr_size: *mut u64) -> *mut p9_fid;
    pub fn p9_client_xattrcreate(fid: *mut p9_fid, name: *const core::ffi::c_char, attr_size: u64, flags: core::ffi::c_int) -> core::ffi::c_int;
    pub fn p9_client_readlink(fid: *mut p9_fid, target: *mut *mut core::ffi::c_char) -> core::ffi::c_int;
    pub fn p9_client_init() -> core::ffi::c_int;
    pub fn p9_client_exit();
}

#[inline]
pub unsafe fn p9_req_get(r: *mut p9_req_t) { refcount_inc(&mut (*r).refcount); }

#[inline]
pub unsafe fn p9_req_try_get(r: *mut p9_req_t) -> core::ffi::c_int { refcount_inc_not_zero(&mut (*r).refcount) }

#[inline]
pub unsafe fn p9_fid_get(fid: *mut p9_fid) -> *mut p9_fid {
    // C tracepoint_enabled(9p_fid_ref)
    if tracepoint_enabled_9p_fid_ref() { do_trace_9p_fid_get(fid); }
    refcount_inc(&mut (*fid).count);
    fid
}

#[inline]
pub unsafe fn p9_fid_put(fid: *mut p9_fid) -> core::ffi::c_int {
    if fid.is_null() || IS_ERR(fid) { return 0; }
    // C tracepoint_enabled(9p_fid_ref)
    if tracepoint_enabled_9p_fid_ref() { do_trace_9p_fid_put(fid); }
    if !refcount_dec_and_test(&mut (*fid).count) { return 0; }
    p9_client_clunk(fid)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
