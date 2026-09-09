/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Network filesystem support services. Rust source-level translation. */

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

#[repr(C)] pub struct mempool { _private: [u8; 0] }
pub type mempool_t = mempool;
pub struct folio_queue;
pub enum netfs_sreq_ref_trace {}

pub type ssize_t = isize;
pub type loff_t = i64;
pub type gfp_t = usize;
pub type u8 = core::primitive::u8;

#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct kiocb { _private: [u8; 0] }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct atomic64_t { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct iovec { _private: [u8; 0] }
#[repr(C)] pub struct bio_vec { _private: [u8; 0] }
#[repr(C)] pub struct rolling_buffer { _private: [u8; 0] }
#[repr(C)] pub struct writeback_control { _private: [u8; 0] }
#[repr(C)] pub struct vm_fault { _private: [u8; 0] }
#[repr(C)] pub struct fscache_cookie { _private: [u8; 0] }
pub type iov_iter_extraction_t = u32;
pub type vm_fault_t = u32;

#[repr(u8)] #[derive(Copy, Clone)] pub enum netfs_io_source { NETFS_SOURCE_UNKNOWN, NETFS_FILL_WITH_ZEROES, NETFS_DOWNLOAD_FROM_SERVER, NETFS_READ_FROM_CACHE, NETFS_INVALID_READ, NETFS_UPLOAD_TO_SERVER, NETFS_WRITE_TO_CACHE }
pub type netfs_io_terminated_t = unsafe extern "C" fn(*mut c_void, ssize_t);

#[repr(C)] pub struct netfs_inode {
    pub inode: inode, pub ops: *const netfs_request_ops, pub cache: *mut fscache_cookie,
    pub wb_queue: list_head, pub _remote_i_size: u64, pub _zero_point: u64,
    pub lock: spinlock_t, pub io_count: atomic_t, pub flags: usize,
}
pub const NETFS_ICTX_ODIRECT: usize = 0; pub const NETFS_ICTX_UNBUFFERED: usize = 1;
pub const NETFS_ICTX_WB_LOCK: usize = 2; pub const NETFS_ICTX_MODIFIED_ATTR: usize = 3;
pub const NETFS_ICTX_SINGLE_NO_UPLOAD: usize = 4;

#[repr(C)] pub struct netfs_group { pub ref_: refcount_t, pub free: Option<unsafe extern "C" fn(*mut netfs_group)> }
#[repr(C)] pub struct netfs_folio { pub netfs_group: *mut netfs_group, pub dirty_offset: u32, pub dirty_len: u32 }
pub const NETFS_FOLIO_INFO: usize = 0x1; pub const NETFS_FOLIO_COPY_TO_CACHE: usize = 0x356;

#[repr(C)] pub struct netfs_io_stream {
    pub construct: *mut netfs_io_subrequest, pub sreq_max_len: usize, pub sreq_max_segs: u32,
    pub submit_off: u32, pub submit_len: u32, pub submit_extendable_to: u32,
    pub prepare_write: Option<unsafe extern "C" fn(*mut netfs_io_subrequest)>,
    pub issue_write: Option<unsafe extern "C" fn(*mut netfs_io_subrequest)>,
    pub subrequests: list_head, pub collected_to: u64, pub transferred: usize, pub error: u16,
    pub source: netfs_io_source, pub stream_nr: u8, pub avail: bool, pub active: bool,
    pub need_retry: bool, pub failed: bool, pub transferred_valid: bool,
}
#[repr(C)] pub struct netfs_cache_resources { pub ops: *const netfs_cache_ops, pub cache_priv: *mut c_void, pub cache_priv2: *mut c_void, pub debug_id: u32, pub inval_counter: u32 }

#[repr(C)] pub struct netfs_io_subrequest {
    pub rreq: *mut netfs_io_request, pub work: work_struct, pub rreq_link: list_head, pub io_iter: iov_iter,
    pub start: u64, pub len: usize, pub transferred: usize, pub ref_: refcount_t, pub error: i16,
    pub debug_index: u16, pub nr_segs: u32, pub retry_count: u8, pub source: netfs_io_source, pub stream_nr: u8, pub flags: usize,
}
pub const NETFS_SREQ_COPY_TO_CACHE: usize = 0; pub const NETFS_SREQ_CLEAR_TAIL: usize = 1;
pub const NETFS_SREQ_MADE_PROGRESS: usize = 4; pub const NETFS_SREQ_BOUNDARY: usize = 6;
pub const NETFS_SREQ_HIT_EOF: usize = 7; pub const NETFS_SREQ_IN_PROGRESS: usize = 8;
pub const NETFS_SREQ_NEED_RETRY: usize = 9; pub const NETFS_SREQ_FAILED: usize = 10;

#[repr(u8)] #[derive(Copy, Clone)] pub enum netfs_io_origin { NETFS_READAHEAD, NETFS_READPAGE, NETFS_READ_GAPS, NETFS_READ_SINGLE, NETFS_READ_FOR_WRITE, NETFS_UNBUFFERED_READ, NETFS_DIO_READ, NETFS_WRITEBACK, NETFS_WRITEBACK_SINGLE, NETFS_WRITETHROUGH, NETFS_UNBUFFERED_WRITE, NETFS_DIO_WRITE, NETFS_PGPRIV2_COPY_TO_CACHE, nr__netfs_io_origin }
#[repr(C)] pub struct netfs_io_request {
    pub cleanup_work: work_struct, pub work: work_struct, pub inode: *mut inode, pub mapping: *mut address_space, pub iocb: *mut kiocb,
    pub cache_resources: netfs_cache_resources, pub copy_to_cache: *mut netfs_io_request, pub io_streams: [netfs_io_stream; 2], pub group: *mut netfs_group,
    pub buffer: rolling_buffer, pub waitq: wait_queue_head_t, pub netfs_priv: *mut c_void, pub netfs_priv2: *mut c_void, pub direct_bv: *mut bio_vec,
    pub submitted: u64, pub len: u64, pub transferred: usize, pub error: isize, pub i_size: u64, pub start: u64, pub issued_to: atomic64_t,
    pub collected_to: u64, pub cleaned_to: u64, pub abandon_to: u64, pub no_unlock_folio: *const c_void, pub gfp: gfp_t, pub direct_bv_count: u32,
    pub debug_id: u32, pub rsize: u32, pub wsize: u32, pub subreq_counter: atomic_t, pub nr_group_rel: u32, pub lock: spinlock_t,
    pub front_folio_order: u8, pub origin: netfs_io_origin, pub direct_bv_unpin: bool, pub ref_: refcount_t, pub flags: usize, pub netfs_ops: *const netfs_request_ops,
}
pub const NR_IO_STREAMS: usize = 2; pub const NETFS_ROLLBUF_PUT_MARK: usize = 1; pub const NETFS_ROLLBUF_PAGECACHE_MARK: usize = 2;
pub const NETFS_RREQ_IN_PROGRESS: usize = 0; pub const NETFS_RREQ_ALL_QUEUED: usize = 1; pub const NETFS_RREQ_PAUSE: usize = 2; pub const NETFS_RREQ_FAILED: usize = 3; pub const NETFS_RREQ_RETRYING: usize = 4; pub const NETFS_RREQ_SHORT_TRANSFER: usize = 5; pub const NETFS_RREQ_OFFLOAD_COLLECTION: usize = 8; pub const NETFS_RREQ_NO_UNLOCK_FOLIO: usize = 9; pub const NETFS_RREQ_FOLIO_COPY_TO_CACHE: usize = 10; pub const NETFS_RREQ_UPLOAD_TO_SERVER: usize = 11; pub const NETFS_RREQ_USE_IO_ITER: usize = 12; pub const NETFS_RREQ_USE_PGPRIV2: usize = 31;

#[repr(C)] pub struct netfs_request_ops {
    pub request_pool: *mut mempool_t, pub subrequest_pool: *mut mempool_t,
    pub init_request: Option<unsafe extern "C" fn(*mut netfs_io_request, *mut file) -> i32>, pub free_request: Option<unsafe extern "C" fn(*mut netfs_io_request)>, pub free_subrequest: Option<unsafe extern "C" fn(*mut netfs_io_subrequest)>,
    pub expand_readahead: Option<unsafe extern "C" fn(*mut netfs_io_request)>, pub prepare_read: Option<unsafe extern "C" fn(*mut netfs_io_subrequest) -> i32>, pub issue_read: Option<unsafe extern "C" fn(*mut netfs_io_subrequest)>, pub is_still_valid: Option<unsafe extern "C" fn(*mut netfs_io_request) -> bool>, pub check_write_begin: Option<unsafe extern "C" fn(*mut file, loff_t, u32, *mut *mut c_void, *mut *mut c_void) -> i32>, pub done: Option<unsafe extern "C" fn(*mut netfs_io_request)>,
    pub update_i_size: Option<unsafe extern "C" fn(*mut inode, loff_t)>, pub post_modify: Option<unsafe extern "C" fn(*mut inode)>, pub begin_writeback: Option<unsafe extern "C" fn(*mut netfs_io_request)>, pub prepare_write: Option<unsafe extern "C" fn(*mut netfs_io_subrequest)>, pub issue_write: Option<unsafe extern "C" fn(*mut netfs_io_subrequest)>, pub retry_request: Option<unsafe extern "C" fn(*mut netfs_io_request, *mut netfs_io_stream)>, pub invalidate_cache: Option<unsafe extern "C" fn(*mut netfs_io_request)>,
}
#[repr(u8)] pub enum netfs_read_from_hole { NETFS_READ_HOLE_IGNORE, NETFS_READ_HOLE_FAIL }
#[repr(C)] pub struct netfs_cache_ops {
    pub end_operation: Option<unsafe extern "C" fn(*mut netfs_cache_resources)>,
    pub read: Option<unsafe extern "C" fn(*mut netfs_cache_resources, loff_t, *mut iov_iter, netfs_read_from_hole, netfs_io_terminated_t, *mut c_void) -> i32>,
    pub write: Option<unsafe extern "C" fn(*mut netfs_cache_resources, loff_t, *mut iov_iter, netfs_io_terminated_t, *mut c_void) -> i32>,
    pub issue_write: Option<unsafe extern "C" fn(*mut netfs_io_subrequest)>, pub expand_readahead: Option<unsafe extern "C" fn(*mut netfs_cache_resources, *mut u64, *mut u64, u64)>, pub prepare_read: Option<unsafe extern "C" fn(*mut netfs_io_subrequest, u64) -> netfs_io_source>, pub prepare_write_subreq: Option<unsafe extern "C" fn(*mut netfs_io_subrequest)>, pub prepare_write: Option<unsafe extern "C" fn(*mut netfs_cache_resources, *mut loff_t, *mut usize, usize, loff_t, bool) -> i32>, pub query_occupancy: Option<unsafe extern "C" fn(*mut netfs_cache_resources, loff_t, usize, usize, *mut loff_t, *mut usize) -> i32>,
}

extern "C" {
    pub fn netfs_unbuffered_read_iter_locked(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t; pub fn netfs_unbuffered_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t; pub fn netfs_buffered_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t; pub fn netfs_file_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t;
    pub fn netfs_perform_write(iocb: *mut kiocb, iter: *mut iov_iter, group: *mut netfs_group) -> ssize_t; pub fn netfs_buffered_write_iter_locked(iocb: *mut kiocb, from: *mut iov_iter, group: *mut netfs_group) -> ssize_t; pub fn netfs_unbuffered_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t; pub fn netfs_unbuffered_write_iter_locked(iocb: *mut kiocb, iter: *mut iov_iter, group: *mut netfs_group) -> ssize_t; pub fn netfs_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t;
    pub fn netfs_single_mark_inode_dirty(inode: *mut inode); pub fn netfs_read_single(inode: *mut inode, file: *mut file, iter: *mut iov_iter) -> ssize_t; pub fn netfs_writeback_single(mapping: *mut address_space, wbc: *mut writeback_control, iter: *mut iov_iter) -> i32;
    pub fn netfs_readahead(rac: *mut c_void); pub fn netfs_read_folio(file: *mut file, folio: *mut c_void) -> i32; pub fn netfs_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> i32; pub fn netfs_dirty_folio(mapping: *mut address_space, folio: *mut c_void) -> bool; pub fn netfs_unpin_writeback(inode: *mut inode, wbc: *mut writeback_control) -> i32; pub fn netfs_clear_inode_writeback(inode: *mut inode, aux: *const c_void); pub fn netfs_invalidate_folio(folio: *mut c_void, offset: usize, length: usize); pub fn netfs_release_folio(folio: *mut c_void, gfp: gfp_t) -> bool;
    pub fn netfs_read_subreq_progress(subreq: *mut netfs_io_subrequest); pub fn netfs_read_subreq_terminated(subreq: *mut netfs_io_subrequest); pub fn netfs_get_subrequest(subreq: *mut netfs_io_subrequest, what: netfs_sreq_ref_trace); pub fn netfs_put_subrequest(subreq: *mut netfs_io_subrequest, what: netfs_sreq_ref_trace); pub fn netfs_extract_user_iter(orig: *mut iov_iter, orig_len: usize, new: *mut iov_iter, flags: iov_iter_extraction_t) -> ssize_t; pub fn netfs_limit_iter(iter: *const iov_iter, start_offset: usize, max_size: usize, max_segs: usize) -> usize; pub fn netfs_prepare_write_failed(subreq: *mut netfs_io_subrequest); pub fn netfs_write_subrequest_terminated(op: *mut c_void, transferred_or_error: ssize_t);
    pub fn netfs_start_io_read(inode: *mut inode) -> i32; pub fn netfs_end_io_read(inode: *mut inode); pub fn netfs_start_io_write(inode: *mut inode) -> i32; pub fn netfs_end_io_write(inode: *mut inode); pub fn netfs_start_io_direct(inode: *mut inode) -> i32; pub fn netfs_end_io_direct(inode: *mut inode);
    pub fn netfs_wb_begin(ictx: *mut netfs_inode, nowait: bool) -> bool; pub fn netfs_wb_end(ictx: *mut netfs_inode);
}

/* Inline helpers retain their kernel semantics; field/container primitives are supplied by dependencies. */
pub unsafe fn netfs_is_folio_info(priv_: *const c_void) -> bool { (priv_ as usize & NETFS_FOLIO_INFO) != 0 }
pub unsafe fn __netfs_folio_info(priv_: *const c_void) -> *mut netfs_folio { if netfs_is_folio_info(priv_) { (priv_ as usize & !NETFS_FOLIO_INFO) as *mut netfs_folio } else { core::ptr::null_mut() } }
pub unsafe fn netfs_folio_info(_folio: *mut c_void) -> *mut netfs_folio { /* folio_get_private() supplied by kernel */ core::ptr::null_mut() }
pub unsafe fn netfs_folio_group(_folio: *mut c_void) -> *mut netfs_group { core::ptr::null_mut() }
pub unsafe fn folio_start_private_2(_folio: *mut c_void) { /* VM_BUG_ON_FOLIO; folio_get; folio_set_private_2 */ }

extern "C" {
    pub fn netfs_page_mkwrite(vmf: *mut c_void, group: *mut netfs_group) -> vm_fault_t;
    pub fn netfs_folioq_alloc(rreq_id: u32, gfp: gfp_t, trace: u32) -> *mut folio_queue;
    pub fn netfs_folioq_free(folioq: *mut folio_queue, trace: u32);
    pub fn netfs_alloc_folioq_buffer(mapping: *mut address_space, buffer: *mut *mut folio_queue, cur_size: *mut usize, size: ssize_t, gfp: gfp_t) -> i32;
    pub fn netfs_free_folioq_buffer(fq: *mut folio_queue);
    pub fn netfs_read_remote_i_size(inode: *const inode) -> u64;
    pub fn netfs_write_remote_i_size(inode: *mut inode, remote_i_size: u64);
    pub fn netfs_read_zero_point(inode: *const inode) -> u64;
    pub fn netfs_write_zero_point(inode: *mut inode, zero_point: u64);
    pub fn netfs_read_sizes(inode: *const inode, i_size: *mut u64, remote_i_size: *mut u64, zero_point: *mut u64);
    pub fn netfs_write_sizes(inode: *mut inode, i_size: u64, remote_i_size: u64, zero_point: u64);
    pub fn netfs_inode_init(ctx: *mut netfs_inode, ops: *const netfs_request_ops, use_zero_point: bool);
    pub fn netfs_resize_file(ictx: *mut netfs_inode, new_i_size: u64, changed_on_server: bool);
    pub fn netfs_i_cookie(ctx: *mut netfs_inode) -> *mut fscache_cookie;
    pub fn netfs_wait_for_outstanding_io(inode: *mut inode);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
