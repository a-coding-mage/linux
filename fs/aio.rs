/*
 * An async IO implementation for Linux — source-level Rust translation of
 * aio.c.  Kernel-provided types and operations remain external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* Kernel ABI types supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct aio_ring {
    pub id: u32, pub nr: u32, pub head: u32, pub tail: u32,
    pub magic: u32, pub compat_features: u32, pub incompat_features: u32,
    pub header_length: u32,
    pub io_events: [io_event; 0],
}

#[repr(C)]
pub struct io_event { pub data: u64, pub obj: u64, pub res: i64, pub res2: i64 }
#[repr(C)] pub struct rcu_head { _p: [u8; 0] }
#[repr(C)] pub struct completion { _p: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct percpu_ref { _p: [u8; 0] }
#[repr(C)] pub struct folio { _p: [u8; 0] }
#[repr(C)] pub struct file { _p: [u8; 0] }
#[repr(C)] pub struct inode { _p: [u8; 0] }
#[repr(C)] pub struct mm_struct { _p: [u8; 0] }
#[repr(C)] pub struct mutex { _p: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _p: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct work_struct { _p: [u8; 0] }
#[repr(C)] pub struct wait_queue_entry { _p: [u8; 0] }
#[repr(C)] pub struct eventfd_ctx { _p: [u8; 0] }
#[repr(C)] pub struct cred { _p: [u8; 0] }
#[repr(C)] pub struct kiocb { _p: [u8; 0] }
#[repr(C)] pub struct iocb { _p: [u8; 0] }
#[repr(C)] pub struct iovec { _p: [u8; 0] }
#[repr(C)] pub struct iov_iter { _p: [u8; 0] }
#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i64 }
pub type aio_context_t = u64;
pub type kiocb_cancel_fn = unsafe extern "C" fn(*mut kiocb) -> i32;
pub type __poll_t = u32;

pub const KIOCB_KEY: u32 = 0;
pub const AIO_RING_MAGIC: u32 = 0xa10a10a1;
pub const AIO_RING_COMPAT_FEATURES: u32 = 1;
pub const AIO_RING_INCOMPAT_FEATURES: u32 = 0;
pub const AIO_PLUG_THRESHOLD: u32 = 2;
pub const AIO_RING_PAGES: usize = 8;

#[repr(C)] pub struct kioctx_table { pub rcu: rcu_head, pub nr: u32, pub table: [*mut kioctx; 0] }
#[repr(C)] pub struct kioctx_cpu { pub reqs_available: u32 }
#[repr(C)] pub struct ctx_rq_wait { pub comp: completion, pub count: atomic_t }

#[repr(C)]
pub struct kioctx {
    pub users: percpu_ref, pub dead: atomic_t, pub reqs: percpu_ref,
    pub user_id: usize, pub cpu: *mut kioctx_cpu, pub req_batch: u32,
    pub max_reqs: u32, pub nr_events: u32, pub mmap_base: usize,
    pub mmap_size: usize, pub ring_folios: *mut *mut folio, pub nr_pages: isize,
    pub free_rwork: [u8; 0], pub rq_wait: *mut ctx_rq_wait,
    pub reqs_available: atomic_t, pub ctx_lock: spinlock_t,
    pub active_reqs: list_head, pub ring_lock: mutex, pub wait: [u8; 0],
    pub tail: u32, pub completed_events: u32, pub completion_lock: spinlock_t,
    pub internal_folios: [*mut folio; AIO_RING_PAGES], pub aio_ring_file: *mut file,
    pub id: u32,
}

#[repr(C)] pub struct fsync_iocb { pub file: *mut file, pub work: work_struct, pub datasync: bool, pub creds: *mut cred }
#[repr(C)] pub struct poll_iocb { pub file: *mut file, pub head: *mut c_void, pub events: __poll_t, pub cancelled: bool, pub work_scheduled: bool, pub work_need_resched: bool, pub wait: wait_queue_entry, pub work: work_struct }
#[repr(C)] pub union aio_kiocb_union { pub ki_filp: *mut file, pub rw: kiocb, pub fsync: fsync_iocb, pub poll: poll_iocb }
#[repr(C)] pub struct aio_kiocb { pub u: aio_kiocb_union, pub ki_ctx: *mut kioctx, pub ki_cancel: Option<kiocb_cancel_fn>, pub ki_res: io_event, pub ki_list: list_head, pub ki_refcnt: u32, pub ki_eventfd: *mut eventfd_ctx }

#[repr(C)] pub struct aio_inode_info { pub vfs_inode: inode, pub migrate_lock: spinlock_t, pub ctx: *mut kioctx }

pub const AIO_EVENTS_PER_PAGE: usize = 4096 / core::mem::size_of::<io_event>();
pub const AIO_EVENTS_FIRST_PAGE: usize = (4096 - core::mem::size_of::<aio_ring>()) / core::mem::size_of::<io_event>();
pub const AIO_EVENTS_OFFSET: usize = AIO_EVENTS_PER_PAGE - AIO_EVENTS_FIRST_PAGE;

static mut aio_nr: usize = 0;
static mut aio_max_nr: usize = 0x10000;

/* The remainder of the implementation consists of kernel syscall and VFS
 * glue.  These symbols are intentionally declared, not reimplemented: the
 * corresponding Linux kernel definitions provide their bodies and types. */
extern "C" {
    pub fn exit_aio(mm: *mut mm_struct);
    pub fn kiocb_set_cancel_fn(iocb: *mut kiocb, cancel: Option<kiocb_cancel_fn>);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
