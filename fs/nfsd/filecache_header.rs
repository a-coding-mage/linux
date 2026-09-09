// Translated from filecache.h. The original include supplies the dependent
// kernel types and declarations used below.

/*
 * Limit the time that the list_lru_one lock is held during
 * an LRU scan.
 */
pub const NFSD_FILE_GC_BATCH: usize = 16usize;

/*
 * This is the fsnotify_mark container that nfsd attaches to the files that it
 * is holding open. Note that we have a separate refcount here aside from the
 * one in the fsnotify_mark. We only want a single fsnotify_mark attached to
 * the inode, and for each nfsd_file to hold a reference to it.
 *
 * The fsnotify_mark is itself refcounted, but that's not sufficient to tell us
 * how to put that reference. If there are still outstanding nfsd_files that
 * reference the mark, then we would want to call fsnotify_put_mark on it.
 * If there were not, then we'd need to call fsnotify_destroy_mark. Since we
 * can't really tell the difference, we use the nfm_mark to keep track of how
 * many nfsd_files hold references to the mark. When that counter goes to zero
 * then we know to call fsnotify_destroy_mark on it.
 */
#[repr(C)]
pub struct nfsd_file_mark {
    pub nfm_mark: fsnotify_mark,
    pub nfm_ref: refcount_t,
    /* serializes nfsd_fsnotify_recalc_mask() against itself */
    pub nfm_recalc_mutex: mutex,
}

/*
 * A representation of a file that has been opened by knfsd. These are hashed
 * in the hashtable by inode pointer value. Note that this object doesn't
 * hold a reference to the inode by itself, so the nf_inode pointer should
 * never be dereferenced, only used for comparison.
 */
#[repr(C)]
pub struct nfsd_file {
    pub nf_rlist: rhlist_head,
    pub nf_inode: *mut core::ffi::c_void,
    pub nf_file: *mut file,
    pub nf_cred: *const cred,
    pub nf_net: *mut net,
    pub nf_flags: core::ffi::c_ulong,
    pub nf_ref: refcount_t,
    pub nf_may: u8,

    pub nf_mark: *mut nfsd_file_mark,
    pub nf_lru: list_head,
    pub nf_gc: list_head,
    pub nf_rcu: rcu_head,
    pub nf_birthtime: ktime_t,

    pub nf_dio_mem_align: u32,
    pub nf_dio_offset_align: u32,
    pub nf_dio_read_offset_align: u32,
}

pub const NFSD_FILE_HASHED: u32 = 0;
pub const NFSD_FILE_PENDING: u32 = 1;
pub const NFSD_FILE_REFERENCED: u32 = 2;
pub const NFSD_FILE_GC: u32 = 3;
pub const NFSD_FILE_RECENT: u32 = 4;

unsafe extern "C" {
    pub fn nfsd_file_cache_init() -> core::ffi::c_int;
    pub fn nfsd_file_cache_purge(net: *mut net);
    pub fn nfsd_file_cache_shutdown();
    pub fn nfsd_file_cache_start_net(net: *mut net) -> core::ffi::c_int;
    pub fn nfsd_file_cache_shutdown_net(net: *mut net);
    pub fn nfsd_file_put(nf: *mut nfsd_file);
    pub fn nfsd_file_put_local(nf: *mut *mut nfsd_file) -> *mut net;
    pub fn nfsd_file_get(nf: *mut nfsd_file) -> *mut nfsd_file;
    pub fn nfsd_file_file(nf: *mut nfsd_file) -> *mut file;
    pub fn nfsd_file_close_inode_sync(inode: *mut inode);
    pub fn nfsd_file_close_export(net: *mut net, path: *const path);
    pub fn nfsd_file_net_dispose(nn: *mut nfsd_net);
    pub fn nfsd_file_is_cached(inode: *mut inode) -> bool;
    pub fn nfsd_file_acquire_gc(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
        may_flags: core::ffi::c_uint, nfp: *mut *mut nfsd_file) -> __be32;
    pub fn nfsd_file_acquire(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
        may_flags: core::ffi::c_uint, nfp: *mut *mut nfsd_file) -> __be32;
    pub fn nfsd_file_acquire_opened(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
        may_flags: core::ffi::c_uint, file: *mut file,
        nfp: *mut *mut nfsd_file) -> __be32;
    pub fn nfsd_file_acquire_local(net: *mut net, cred: *mut svc_cred,
        client: *mut auth_domain, fhp: *mut svc_fh,
        may_flags: core::ffi::c_uint, pnf: *mut *mut nfsd_file) -> __be32;
    pub fn nfsd_file_acquire_dir(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
        pnf: *mut *mut nfsd_file) -> __be32;
    pub fn nfsd_file_cache_stats_show(m: *mut seq_file, v: *mut core::ffi::c_void)
        -> core::ffi::c_int;
    pub fn nfsd_fsnotify_recalc_mask(nf: *mut nfsd_file);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
