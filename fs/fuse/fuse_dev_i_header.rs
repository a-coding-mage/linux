/* SPDX-License-Identifier: GPL-2.0 */
// Direct Rust translation of fuse_dev_i.h. Included kernel types are external dependencies.

pub const FUSE_INT_REQ_BIT: u64 = 1u64 << 0;
pub const FUSE_REQ_ID_STEP: u64 = 1u64 << 1;

pub const FUSE_PQ_HASH_BITS: usize = 8;
pub const FUSE_PQ_HASH_SIZE: usize = 1usize << FUSE_PQ_HASH_BITS;

#[repr(C)]
pub enum fuse_req_flag {
    FR_ISREPLY,
    FR_FORCE,
    FR_BACKGROUND,
    FR_WAITING,
    FR_ABORTED,
    FR_INTERRUPTED,
    FR_LOCKED,
    FR_PENDING,
    FR_SENT,
    FR_FINISHED,
    FR_PRIVATE,
    FR_ASYNC,
    FR_URING,
    FR_SYNC_WAKEUP,
}

#[repr(C)]
pub struct fuse_req {
    pub list: list_head,
    pub intr_entry: list_head,
    pub args: *mut fuse_args,
    pub count: refcount_t,
    pub flags: c_ulong,
    pub in_: fuse_req_in,
    pub out: fuse_req_out,
    pub waitq: wait_queue_head_t,
    #[cfg(feature = "CONFIG_VIRTIO_FS")]
    pub argbuf: *mut c_void,
    pub chan: *mut fuse_chan,
    #[cfg(feature = "CONFIG_FUSE_IO_URING")]
    pub ring_entry: *mut c_void,
    #[cfg(feature = "CONFIG_FUSE_IO_URING")]
    pub ring_queue: *mut c_void,
    pub create_time: c_ulong,
}

#[repr(C)] pub struct fuse_req_in { pub h: fuse_in_header }
#[repr(C)] pub struct fuse_req_out { pub h: fuse_out_header }

#[repr(C)]
pub struct fuse_forget_link { pub forget_one: fuse_forget_one, pub next: *mut fuse_forget_link }

#[repr(C)]
pub struct fuse_iqueue_ops {
    pub send_forget: Option<unsafe extern "C" fn(*mut fuse_iqueue, *mut fuse_forget_link)>,
    pub send_interrupt: Option<unsafe extern "C" fn(*mut fuse_iqueue, *mut fuse_req)>,
    pub send_req: Option<unsafe extern "C" fn(*mut fuse_iqueue, *mut fuse_req)>,
    pub release: Option<unsafe extern "C" fn(*mut fuse_iqueue)>,
}

#[repr(C)]
pub struct fuse_iqueue {
    pub connected: c_uint,
    pub lock: spinlock_t,
    pub waitq: wait_queue_head_t,
    pub reqctr: u64,
    pub pending: list_head,
    pub interrupts: list_head,
    pub forget_list_head: fuse_forget_link,
    pub forget_list_tail: *mut fuse_forget_link,
    pub forget_batch: c_int,
    pub fasync: *mut fasync_struct,
    pub ops: *const fuse_iqueue_ops,
    pub priv_: *mut c_void,
}

#[repr(C)]
pub struct fuse_chan {
    pub lock: spinlock_t,
    pub conn: *mut fuse_conn,
    pub iq: fuse_iqueue,
    pub devices: list_head,
    pub max_background: c_uint,
    pub num_background: c_uint,
    pub active_background: c_uint,
    pub bg_queue: list_head,
    pub bg_lock: spinlock_t,
    pub initialized: c_int,
    pub blocked: c_int,
    pub blocked_waitq: wait_queue_head_t,
    pub connected: c_uint,
    pub num_waiting: atomic_t,
    pub no_interrupt: bool,
    pub io_uring: c_uint,
    pub minor: c_uint,
    pub max_write: c_uint,
    pub max_pages: c_uint,
    pub pq_prealloc: *mut list_head,
    pub abort_with_err: bool,
    #[cfg(feature = "CONFIG_FUSE_IO_URING")]
    pub ring: *mut fuse_ring,
    pub timeout: fuse_chan_timeout,
}

#[repr(C)] pub struct fuse_chan_timeout { pub work: delayed_work, pub req_timeout: c_uint }

#[repr(C)]
pub struct fuse_pqueue { pub connected: c_uint, pub lock: spinlock_t, pub processing: *mut list_head, pub io: list_head }

#[repr(C)]
pub struct fuse_dev { pub ref_: refcount_t, pub sync_init: bool, pub chan: *mut fuse_chan, pub pq: fuse_pqueue, pub entry: list_head }

#[repr(C)]
pub struct fuse_copy_state {
    pub req: *mut fuse_req,
    pub iter: *mut iov_iter,
    pub pipebufs: *mut pipe_buffer,
    pub currbuf: *mut pipe_buffer,
    pub pipe: *mut pipe_inode_info,
    pub nr_segs: c_ulong,
    pub pg: *mut page,
    pub len: c_uint,
    pub offset: c_uint,
    pub write: bool,
    pub move_folios: bool,
    pub is_uring: bool,
    pub skip_folio_copy: bool,
    pub ring: fuse_copy_ring,
}
#[repr(C)] pub struct fuse_copy_ring { pub copied_sz: c_uint }

pub const FUSE_DEV_CHAN_DISCONNECTED: *mut fuse_chan = 1usize as *mut fuse_chan;

pub unsafe extern "C" fn fuse_dev_chan_get(fud: *mut fuse_dev) -> *mut fuse_chan {
    // Pairs with xchg() in fuse_dev_install().
    smp_load_acquire(&(*fud).chan)
}
pub unsafe extern "C" fn fuse_file_to_fud(file: *mut file) -> *mut fuse_dev { (*file).private_data as *mut fuse_dev }
pub unsafe extern "C" fn __fuse_get_dev(file: *mut file) -> *mut fuse_dev {
    let fud = fuse_file_to_fud(file);
    if fuse_dev_chan_get(fud).is_null() { core::ptr::null_mut() } else { fud }
}

extern "C" {
    pub fn fuse_iqueue_init(fiq: *mut fuse_iqueue, ops: *const fuse_iqueue_ops, priv_: *mut c_void);
    pub fn fuse_get_dev(file: *mut file) -> *mut fuse_dev;
    pub fn fuse_req_hash(unique: u64) -> c_uint;
    pub fn fuse_request_find(fpq: *mut fuse_pqueue, unique: u64) -> *mut fuse_req;
    pub fn fuse_dev_end_requests(head: *mut list_head);
    pub fn fuse_request_bg_finish(fch: *mut fuse_chan, req: *mut fuse_req);
    pub fn fuse_copy_init(cs: *mut fuse_copy_state, write: bool, iter: *mut iov_iter);
    pub fn fuse_len_args(numargs: c_uint, args: *mut fuse_arg) -> c_uint;
    pub fn fuse_copy_args(cs: *mut fuse_copy_state, numargs: c_uint, argpages: c_uint, args: *mut fuse_arg, zeroing: c_int) -> c_int;
    pub fn fuse_copy_out_args(cs: *mut fuse_copy_state, args: *mut fuse_args, nbytes: c_uint) -> c_int;
    pub fn fuse_dev_queue_forget(fiq: *mut fuse_iqueue, forget: *mut fuse_forget_link);
    pub fn fuse_dev_queue_interrupt(fiq: *mut fuse_iqueue, req: *mut fuse_req);
    pub fn fuse_remove_pending_req(req: *mut fuse_req, lock: *mut spinlock_t) -> bool;
    pub fn fuse_request_expired(fch: *mut fuse_chan, list: *mut list_head) -> bool;
    pub fn fuse_request_assign_unique(fiq: *mut fuse_iqueue, req: *mut fuse_req);
    pub fn fuse_get_unique(fiq: *mut fuse_iqueue) -> u64;
    pub fn fuse_dev_alloc_install(fch: *mut fuse_chan) -> *mut fuse_dev;
    pub fn fuse_dev_alloc() -> *mut fuse_dev;
    pub fn fuse_dev_release(inode: *mut inode, file: *mut file) -> c_int;
    pub fn fuse_pqueue_alloc() -> *mut list_head;
    pub fn fuse_pqueue_init(fpq: *mut fuse_pqueue);
    pub fn fuse_request_end(req: *mut fuse_req);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
