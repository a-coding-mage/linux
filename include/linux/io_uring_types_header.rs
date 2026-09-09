// Translated from io_uring_types.h. Kernel dependencies are supplied by the surrounding crate.

#[repr(C)]
pub struct iou_loop_params { _private: [u8; 0] }
#[repr(C)]
pub struct io_uring_bpf_ops { _private: [u8; 0] }

pub const IOU_F_TWQ_LAZY_WAKE: u32 = 1;
pub const IOU_F_TWQ_IN_WAKE: u32 = 2;

pub const IO_URING_F_COMPLETE_DEFER: u32 = 1;
pub const IO_URING_F_UNLOCKED: u32 = 2;
pub const IO_URING_F_MULTISHOT: u32 = 4;
pub const IO_URING_F_IOWQ: u32 = 8;
pub const IO_URING_F_INLINE: u32 = 16;
pub const IO_URING_F_NONBLOCK: i32 = i32::MIN;
pub const IO_URING_F_SQE128: u32 = 1 << 8;
pub const IO_URING_F_CQE32: u32 = 1 << 9;
pub const IO_URING_F_IOPOLL: u32 = 1 << 10;
pub const IO_URING_F_CANCEL: u32 = 1 << 11;
pub const IO_URING_F_COMPAT: u32 = 1 << 12;

pub const IO_BUF_DEST: u32 = 1 << ITER_DEST;
pub const IO_BUF_SOURCE: u32 = 1 << ITER_SOURCE;
pub const IO_RINGFD_REG_MAX: usize = 16;

#[repr(C)] pub struct io_wq_work_node { pub next: *mut io_wq_work_node }
#[repr(C)] pub struct io_wq_work_list { pub first: *mut io_wq_work_node, pub last: *mut io_wq_work_node }
#[repr(C)] pub struct mpscq { pub tail: *mut llist_node, pub stub: llist_node }
#[repr(C)] pub struct io_wq_work { pub list: io_wq_work_node, pub flags: atomic_t, pub cancel_seq: i32 }
#[repr(C)] pub struct io_rsrc_data { pub nr: u32, pub nodes: *mut *mut io_rsrc_node }
#[repr(C)] pub struct io_file_table { pub data: io_rsrc_data, pub bitmap: *mut libc::c_ulong, pub alloc_hint: u32 }
#[repr(C)] pub struct io_hash_bucket { pub list: hlist_head }
#[repr(C)] pub struct io_hash_table { pub hbs: *mut io_hash_bucket, pub hash_bits: libc::c_uint }
#[repr(C)] pub struct io_mapped_region { pub pages: *mut *mut page, pub ptr: *mut libc::c_void, pub nr_pages: libc::c_uint, pub flags: libc::c_uint }

#[repr(C)] pub union io_br_sel_union { pub addr: *mut libc::c_void, pub val: isize }
#[repr(C)] pub struct io_br_sel { pub buf_list: *mut io_buffer_list, pub value: io_br_sel_union }

#[repr(C)] pub struct io_uring_task {
    pub cached_refs: i32, pub last: *const io_ring_ctx, pub task: *mut task_struct, pub io_wq: *mut io_wq,
    pub task_head: *mut llist_node, pub registered_rings: [*mut file; IO_RINGFD_REG_MAX],
    pub xa: xarray, pub wait: wait_queue_head, pub in_cancel: atomic_t, pub inflight_tracked: atomic_t,
    pub inflight: percpu_counter, pub fallback_work: work_struct,
    pub task_list: mpscq, pub task_work: callback_head,
}
#[repr(C)] pub union iou_vec_union { pub iovec: *mut iovec, pub bvec: *mut bio_vec }
#[repr(C)] pub struct iou_vec { pub value: iou_vec_union, pub nr: libc::c_uint }
#[repr(C)] pub struct io_uring { pub head: u32, pub tail: u32 }

#[repr(C)] pub struct io_rings {
    pub sq: io_uring, pub cq: io_uring, pub sq_ring_mask: u32, pub cq_ring_mask: u32,
    pub sq_ring_entries: u32, pub cq_ring_entries: u32, pub sq_dropped: u32,
    pub sq_flags: atomic_t, pub cq_flags: u32, pub cq_overflow: u32,
    pub cqes: [io_uring_cqe; 0],
}

#[repr(C)] pub struct io_bpf_filter { _private: [u8; 0] }
#[repr(C)] pub struct io_bpf_filters { pub refs: refcount_t, pub lock: spinlock_t, pub filters: *mut *mut io_bpf_filter, pub rcu_head: rcu_head }
#[repr(C)] pub struct io_restriction {
    pub register_op: [libc::c_ulong; 1], pub sqe_op: [libc::c_ulong; 1], pub bpf_filters: *mut io_bpf_filters,
    pub bpf_filters_cow: bool, pub sqe_flags_allowed: u8, pub sqe_flags_required: u8,
    pub op_registered: bool, pub reg_registered: bool,
}
#[repr(C)] pub struct io_submit_link { pub head: *mut io_kiocb, pub last: *mut io_kiocb }
#[repr(C)] pub struct io_submit_state {
    pub free_list: io_wq_work_node, pub compl_reqs: io_wq_work_list, pub link: io_submit_link,
    pub plug_started: bool, pub need_plug: bool, pub cq_flush: bool, pub submit_nr: u16, pub plug: blk_plug,
}
#[repr(C)] pub struct io_alloc_cache { pub entries: *mut *mut libc::c_void, pub nr_cached: u32, pub max_cached: u32, pub elem_size: u32, pub init_clear: u32 }

pub const IO_RING_F_DRAIN_NEXT: u32 = 1 << 0;
pub const IO_RING_F_OP_RESTRICTED: u32 = 1 << 1;
pub const IO_RING_F_REG_RESTRICTED: u32 = 1 << 2;
pub const IO_RING_F_OFF_TIMEOUT_USED: u32 = 1 << 3;
pub const IO_RING_F_DRAIN_ACTIVE: u32 = 1 << 4;
pub const IO_RING_F_HAS_EVFD: u32 = 1 << 5;
pub const IO_RING_F_TASK_COMPLETE: u32 = 1 << 6;
pub const IO_RING_F_LOCKLESS_CQ: u32 = 1 << 7;
pub const IO_RING_F_SYSCALL_IOPOLL: u32 = 1 << 8;
pub const IO_RING_F_POLL_ACTIVATED: u32 = 1 << 9;
pub const IO_RING_F_DRAIN_DISABLED: u32 = 1 << 10;
pub const IO_RING_F_COMPAT: u32 = 1 << 11;
pub const IO_RING_F_IOWQ_LIMITS_SET: u32 = 1 << 12;

#[repr(C)] pub struct iou_ctx {}
#[repr(C)] pub struct io_ring_ctx {
    pub flags: u32, pub int_flags: u32, pub submitter_task: *mut task_struct, pub rings: *mut io_rings,
    pub bpf_filters: *mut *mut io_bpf_filter, pub refs: percpu_ref, pub clockid: clockid_t,
    pub clock_offset: tk_offsets, pub notify_method: task_work_notify_mode, pub sq_thread_idle: libc::c_uint,
    pub uring_lock: mutex, pub sq_array: *mut u32, pub sq_sqes: *mut io_uring_sqe, pub cached_sq_head: libc::c_uint,
    pub sq_entries: libc::c_uint, pub cancel_seq: atomic_t, pub poll_multi_queue: bool, pub iopoll_list: list_head,
    pub work_head: *mut llist_node, pub file_table: io_file_table, pub buf_table: io_rsrc_data,
    pub node_cache: io_alloc_cache, pub imu_cache: io_alloc_cache, pub submit_state: io_submit_state,
    pub io_bl_xa: xarray, pub cancel_table: io_hash_table, pub apoll_cache: io_alloc_cache,
    pub netmsg_cache: io_alloc_cache, pub rw_cache: io_alloc_cache, pub cmd_cache: io_alloc_cache,
    pub loop_step: Option<unsafe extern "C" fn(*mut iou_ctx, *mut iou_loop_params) -> i32>,
    pub cancelable_uring_cmd: hlist_head, pub hybrid_poll_time: u64,
    pub cqe_cached: *mut io_uring_cqe, pub cqe_sentinel: *mut io_uring_cqe, pub cached_cq_tail: libc::c_uint,
    pub cq_entries: libc::c_uint, pub io_ev_fd: *mut io_ev_fd, pub cq_wait_arg: *mut libc::c_void, pub cq_wait_size: usize,
    pub rings_rcu: *mut io_rings, pub work_list: mpscq, pub check_cq: libc::c_ulong, pub cq_wait_nr: atomic_t,
    pub cq_timeouts: atomic_t, pub cq_wait: wait_queue_head, pub timeout_lock: raw_spinlock_t,
    pub timeout_list: list_head, pub ltimeout_list: list_head, pub cq_last_tm_flush: libc::c_uint,
    pub completion_lock: spinlock_t, pub cq_overflow_list: list_head, pub waitid_list: hlist_head,
    // CONFIG_FUTEX: futex_list and futex_cache are present when enabled.
    pub sq_creds: *const cred, pub sq_data: *mut io_sq_data, pub sqo_sq_wait: wait_queue_head, pub sqd_list: list_head,
    pub file_alloc_start: u32, pub file_alloc_end: u32, pub poll_wq: wait_queue_head, pub restrictions: io_restriction,
    pub zcrx_ctxs: xarray, pub hpage_acct: xarray, pub pers_next: u32, pub personalities: xarray,
    pub hash_map: *mut io_wq_hash, pub user: *mut user_struct, pub mm_account: *mut mm_struct,
    pub tctx_list: list_head, pub tctx_lock: mutex, pub exit_work: work_struct, pub ref_comp: completion,
    pub iowq_limits: [u32; 2], pub poll_wq_task_work: callback_head, pub defer_list: list_head, pub nr_drained: libc::c_uint,
    pub nr_req_allocated: libc::c_uint, pub bpf_ops: *mut io_uring_bpf_ops, pub mmap_lock: mutex,
    pub sq_region: io_mapped_region, pub ring_region: io_mapped_region, pub param_region: io_mapped_region,
    pub kcov_handle: kcov_common_handle_id,
}

#[repr(C)] pub struct io_tw_state { pub cancel: bool }
pub type io_tw_token_t = io_tw_state;
pub type io_req_flags_t = u64;
pub const fn io_req_flag(bitno: u32) -> io_req_flags_t { 1u64 << bitno }

pub const REQ_F_FIXED_FILE_BIT: u32 = IOSQE_FIXED_FILE_BIT; pub const REQ_F_IO_DRAIN_BIT: u32 = IOSQE_IO_DRAIN_BIT;
pub const REQ_F_LINK_BIT: u32 = IOSQE_IO_LINK_BIT; pub const REQ_F_HARDLINK_BIT: u32 = IOSQE_IO_HARDLINK_BIT;
pub const REQ_F_FORCE_ASYNC_BIT: u32 = IOSQE_ASYNC_BIT; pub const REQ_F_BUFFER_SELECT_BIT: u32 = IOSQE_BUFFER_SELECT_BIT;
pub const REQ_F_CQE_SKIP_BIT: u32 = IOSQE_CQE_SKIP_SUCCESS_BIT;
pub const REQ_F_FAIL_BIT: u32 = 8;
// Sequential enum values from REQ_F_INFLIGHT_BIT through __REQ_F_LAST_BIT.
pub const REQ_F_INFLIGHT_BIT: u32 = 9; pub const REQ_F_CUR_POS_BIT: u32 = 10; pub const REQ_F_NOWAIT_BIT: u32 = 11;
pub const REQ_F_LINK_TIMEOUT_BIT: u32 = 12; pub const REQ_F_NEED_CLEANUP_BIT: u32 = 13; pub const REQ_F_POLLED_BIT: u32 = 14;
pub const REQ_F_HYBRID_IOPOLL_STATE_BIT: u32 = 15; pub const REQ_F_BUFFER_SELECTED_BIT: u32 = 16; pub const REQ_F_BUFFER_RING_BIT: u32 = 17;
pub const REQ_F_REISSUE_BIT: u32 = 18; pub const REQ_F_CREDS_BIT: u32 = 19; pub const REQ_F_REFCOUNT_BIT: u32 = 20;
pub const REQ_F_ARM_LTIMEOUT_BIT: u32 = 21; pub const REQ_F_ASYNC_DATA_BIT: u32 = 22; pub const REQ_F_SKIP_LINK_CQES_BIT: u32 = 23;
pub const REQ_F_SINGLE_POLL_BIT: u32 = 24; pub const REQ_F_DOUBLE_POLL_BIT: u32 = 25; pub const REQ_F_MULTISHOT_BIT: u32 = 26;
pub const REQ_F_APOLL_MULTISHOT_BIT: u32 = 27; pub const REQ_F_CLEAR_POLLIN_BIT: u32 = 28; pub const REQ_F_SUPPORT_NOWAIT_BIT: u32 = 29;
pub const REQ_F_ISREG_BIT: u32 = 30; pub const REQ_F_POLL_NO_LAZY_BIT: u32 = 31; pub const REQ_F_CAN_POLL_BIT: u32 = 32;
pub const REQ_F_BL_EMPTY_BIT: u32 = 33; pub const REQ_F_BL_NO_RECYCLE_BIT: u32 = 34; pub const REQ_F_BUFFERS_COMMIT_BIT: u32 = 35;
pub const REQ_F_BUF_NODE_BIT: u32 = 36; pub const REQ_F_BUF_MORE_BIT: u32 = 37; pub const REQ_F_HAS_METADATA_BIT: u32 = 38;
pub const REQ_F_IMPORT_BUFFER_BIT: u32 = 39; pub const REQ_F_SQE_COPIED_BIT: u32 = 40; pub const REQ_F_IOPOLL_BIT: u32 = 41;

pub const REQ_F_FIXED_FILE: io_req_flags_t = io_req_flag(REQ_F_FIXED_FILE_BIT);
pub const REQ_F_IO_DRAIN: io_req_flags_t = io_req_flag(REQ_F_IO_DRAIN_BIT);
pub const REQ_F_LINK: io_req_flags_t = io_req_flag(REQ_F_LINK_BIT);
pub const REQ_F_HARDLINK: io_req_flags_t = io_req_flag(REQ_F_HARDLINK_BIT);
pub const REQ_F_FORCE_ASYNC: io_req_flags_t = io_req_flag(REQ_F_FORCE_ASYNC_BIT);
pub const REQ_F_BUFFER_SELECT: io_req_flags_t = io_req_flag(REQ_F_BUFFER_SELECT_BIT);
pub const REQ_F_CQE_SKIP: io_req_flags_t = io_req_flag(REQ_F_CQE_SKIP_BIT);
pub const REQ_F_FAIL: io_req_flags_t = io_req_flag(REQ_F_FAIL_BIT);
pub const REQ_F_INFLIGHT: io_req_flags_t = io_req_flag(REQ_F_INFLIGHT_BIT);
pub const REQ_F_CUR_POS: io_req_flags_t = io_req_flag(REQ_F_CUR_POS_BIT);
pub const REQ_F_NOWAIT: io_req_flags_t = io_req_flag(REQ_F_NOWAIT_BIT);
pub const REQ_F_LINK_TIMEOUT: io_req_flags_t = io_req_flag(REQ_F_LINK_TIMEOUT_BIT);
pub const REQ_F_NEED_CLEANUP: io_req_flags_t = io_req_flag(REQ_F_NEED_CLEANUP_BIT);
pub const REQ_F_POLLED: io_req_flags_t = io_req_flag(REQ_F_POLLED_BIT);
pub const REQ_F_IOPOLL_STATE: io_req_flags_t = io_req_flag(REQ_F_HYBRID_IOPOLL_STATE_BIT);
pub const REQ_F_BUFFER_SELECTED: io_req_flags_t = io_req_flag(REQ_F_BUFFER_SELECTED_BIT);
pub const REQ_F_BUFFER_RING: io_req_flags_t = io_req_flag(REQ_F_BUFFER_RING_BIT);
pub const REQ_F_REISSUE: io_req_flags_t = io_req_flag(REQ_F_REISSUE_BIT);
pub const REQ_F_SUPPORT_NOWAIT: io_req_flags_t = io_req_flag(REQ_F_SUPPORT_NOWAIT_BIT);
pub const REQ_F_ISREG: io_req_flags_t = io_req_flag(REQ_F_ISREG_BIT);
pub const REQ_F_CREDS: io_req_flags_t = io_req_flag(REQ_F_CREDS_BIT);
pub const REQ_F_REFCOUNT: io_req_flags_t = io_req_flag(REQ_F_REFCOUNT_BIT);
pub const REQ_F_ARM_LTIMEOUT: io_req_flags_t = io_req_flag(REQ_F_ARM_LTIMEOUT_BIT);
pub const REQ_F_ASYNC_DATA: io_req_flags_t = io_req_flag(REQ_F_ASYNC_DATA_BIT);
pub const REQ_F_SKIP_LINK_CQES: io_req_flags_t = io_req_flag(REQ_F_SKIP_LINK_CQES_BIT);
pub const REQ_F_SINGLE_POLL: io_req_flags_t = io_req_flag(REQ_F_SINGLE_POLL_BIT);
pub const REQ_F_DOUBLE_POLL: io_req_flags_t = io_req_flag(REQ_F_DOUBLE_POLL_BIT);
pub const REQ_F_MULTISHOT: io_req_flags_t = io_req_flag(REQ_F_MULTISHOT_BIT);
pub const REQ_F_APOLL_MULTISHOT: io_req_flags_t = io_req_flag(REQ_F_APOLL_MULTISHOT_BIT);
pub const REQ_F_CLEAR_POLLIN: io_req_flags_t = io_req_flag(REQ_F_CLEAR_POLLIN_BIT);
pub const REQ_F_POLL_NO_LAZY: io_req_flags_t = io_req_flag(REQ_F_POLL_NO_LAZY_BIT);
pub const REQ_F_CAN_POLL: io_req_flags_t = io_req_flag(REQ_F_CAN_POLL_BIT);
pub const REQ_F_BL_EMPTY: io_req_flags_t = io_req_flag(REQ_F_BL_EMPTY_BIT);
pub const REQ_F_BL_NO_RECYCLE: io_req_flags_t = io_req_flag(REQ_F_BL_NO_RECYCLE_BIT);
pub const REQ_F_BUFFERS_COMMIT: io_req_flags_t = io_req_flag(REQ_F_BUFFERS_COMMIT_BIT);
pub const REQ_F_BUF_NODE: io_req_flags_t = io_req_flag(REQ_F_BUF_NODE_BIT);
pub const REQ_F_BUF_MORE: io_req_flags_t = io_req_flag(REQ_F_BUF_MORE_BIT);
pub const REQ_F_HAS_METADATA: io_req_flags_t = io_req_flag(REQ_F_HAS_METADATA_BIT);
pub const REQ_F_IMPORT_BUFFER: io_req_flags_t = io_req_flag(REQ_F_IMPORT_BUFFER_BIT);
pub const REQ_F_SQE_COPIED: io_req_flags_t = io_req_flag(REQ_F_SQE_COPIED_BIT);
pub const REQ_F_IOPOLL: io_req_flags_t = io_req_flag(REQ_F_IOPOLL_BIT);

#[repr(C)] pub struct io_tw_req { pub req: *mut io_kiocb }
pub type io_req_tw_func_t = Option<unsafe extern "C" fn(io_tw_req, io_tw_token_t)>;
#[repr(C)] pub struct io_task_work { pub node: llist_node, pub func: io_req_tw_func_t }
#[repr(C)] pub union io_cqe_flags_fd { pub flags: u32, pub fd: i32 }
#[repr(C)] pub struct io_cqe { pub user_data: u64, pub res: i32, pub flags_fd: io_cqe_flags_fd }
#[repr(C)] pub struct io_cmd_data { pub file: *mut file, pub data: [u8; 56] }
pub unsafe fn io_kiocb_cmd_sz_check(_cmd_sz: usize) {}
pub unsafe fn cmd_to_io_kiocb(ptr: *mut libc::c_void) -> *mut io_kiocb { ptr as *mut io_kiocb }

#[repr(C)] pub union io_kiocb_file_cmd { pub file: *mut file, pub cmd: io_cmd_data }
#[repr(C)] pub union io_kiocb_buf { pub kbuf: *mut io_buffer, pub buf_node: *mut io_rsrc_node }
#[repr(C)] pub union io_kiocb_comp_poll { pub comp_list: io_wq_work_node, pub apoll_events: __poll_t }
#[repr(C)] pub union io_kiocb_task_poll { pub io_task_work: io_task_work, pub iopoll_start: u64 }
#[repr(C)] pub union io_kiocb_hash_list_rcu { pub hash_node: hlist_node, pub iopoll_node: list_head, pub rcu_head: rcu_head }
#[repr(C)] pub struct io_big_cqe { pub extra1: u64, pub extra2: u64 }
#[repr(C)] pub struct io_kiocb {
    pub file_cmd: io_kiocb_file_cmd, pub opcode: u8, pub iopoll_completed: u8, pub buf_index: u16,
    pub flags: io_req_flags_t, pub cqe: io_cqe, pub ctx: *mut io_ring_ctx, pub tctx: *mut io_uring_task,
    pub buf: io_kiocb_buf, pub comp_poll: io_kiocb_comp_poll, pub file_node: *mut io_rsrc_node,
    pub refs: atomic_t, pub cancel_seq_set: bool, pub task_poll: io_kiocb_task_poll,
    pub hash_list_rcu: io_kiocb_hash_list_rcu, pub apoll: *mut async_poll, pub async_data: *mut libc::c_void,
    pub poll_refs: atomic_t, pub link: *mut io_kiocb, pub creds: *const cred, pub work: io_wq_work, pub big_cqe: io_big_cqe,
}
#[repr(C)] pub struct io_overflow_cqe { pub list: list_head, pub cqe: io_uring_cqe }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
