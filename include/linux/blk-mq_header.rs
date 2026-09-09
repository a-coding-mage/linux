/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/blk-mq.h; included kernel dependencies remain external. */

pub const BLKDEV_MIN_RQ: u32 = 4;
pub const BLKDEV_DEFAULT_RQ: u32 = 128;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rq_end_io_ret { RQ_END_IO_NONE, RQ_END_IO_FREE }
pub type rq_end_io_fn = unsafe extern "C" fn(*mut request, blk_status_t, *const io_comp_batch) -> rq_end_io_ret;
pub type req_flags_t = __u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rqf_flags {
    __RQF_STARTED, __RQF_FLUSH_SEQ, __RQF_MIXED_MERGE, __RQF_DONTPREP,
    __RQF_SCHED_TAGS, __RQF_USE_SCHED, __RQF_FAILED, __RQF_QUIET,
    __RQF_IO_STAT, __RQF_PM, __RQF_HASHED, __RQF_STATS, __RQF_SPECIAL_PAYLOAD,
    __RQF_ZONE_WRITE_PLUGGING, __RQF_TIMED_OUT, __RQF_RESV, __RQF_BITS,
}
pub const RQF_STARTED: req_flags_t = 1 << 0;
pub const RQF_FLUSH_SEQ: req_flags_t = 1 << 1;
pub const RQF_MIXED_MERGE: req_flags_t = 1 << 2;
pub const RQF_DONTPREP: req_flags_t = 1 << 3;
pub const RQF_SCHED_TAGS: req_flags_t = 1 << 4;
pub const RQF_USE_SCHED: req_flags_t = 1 << 5;
pub const RQF_FAILED: req_flags_t = 1 << 6;
pub const RQF_QUIET: req_flags_t = 1 << 7;
pub const RQF_IO_STAT: req_flags_t = 1 << 8;
pub const RQF_PM: req_flags_t = 1 << 9;
pub const RQF_HASHED: req_flags_t = 1 << 10;
pub const RQF_STATS: req_flags_t = 1 << 11;
pub const RQF_SPECIAL_PAYLOAD: req_flags_t = 1 << 12;
pub const RQF_ZONE_WRITE_PLUGGING: req_flags_t = 1 << 13;
pub const RQF_TIMED_OUT: req_flags_t = 1 << 14;
pub const RQF_RESV: req_flags_t = 1 << 15;
pub const RQF_NOMERGE_FLAGS: req_flags_t = RQF_STARTED | RQF_FLUSH_SEQ | RQF_SPECIAL_PAYLOAD;

#[repr(C)] pub enum mq_rq_state { MQ_RQ_IDLE = 0, MQ_RQ_IN_FLIGHT = 1, MQ_RQ_COMPLETE = 2 }

#[repr(C)]
pub union request_queue_link { pub queuelist: list_head, pub rq_next: *mut request }
#[repr(C)]
pub union request_hash_link { pub hash: hlist_node, pub ipi_list: llist_node }
#[repr(C)]
pub union request_rb_link { pub rb_node: rb_node, pub special_vec: bio_vec }
#[repr(C)] pub struct request_elv { pub icq: *mut io_cq, pub priv_: [*mut core::ffi::c_void; 2] }
#[repr(C)] pub struct request_flush { pub seq: c_uint, pub saved_end_io: *mut rq_end_io_fn }

#[repr(C)]
pub struct request {
    pub q: *mut request_queue, pub mq_ctx: *mut blk_mq_ctx, pub mq_hctx: *mut blk_mq_hw_ctx,
    pub cmd_flags: blk_opf_t, pub rq_flags: req_flags_t, pub tag: c_int, pub internal_tag: c_int,
    pub timeout: c_uint, pub __data_len: c_uint, pub __sector: sector_t,
    pub bio: *mut bio, pub biotail: *mut bio, pub link: request_queue_link, pub part: *mut block_device,
    #[cfg(CONFIG_BLK_RQ_ALLOC_TIME)] pub alloc_time_ns: u64,
    pub start_time_ns: u64, pub io_start_time_ns: u64,
    #[cfg(CONFIG_BLK_WBT)] pub wbt_flags: u16,
    pub stats_sectors: u16, pub nr_phys_segments: u16, pub nr_integrity_segments: u16,
    pub phys_gap_bit: u8,
    #[cfg(CONFIG_BLK_INLINE_ENCRYPTION)] pub crypt_ctx: *mut bio_crypt_ctx,
    #[cfg(CONFIG_BLK_INLINE_ENCRYPTION)] pub crypt_keyslot: *mut blk_crypto_keyslot,
    pub state: mq_rq_state, pub ref_: atomic_t, pub deadline: c_ulong,
    pub hash_link: request_hash_link, pub rb_link: request_rb_link, pub elv: request_elv,
    pub flush: request_flush, pub fifo_time: u64, pub end_io: *mut rq_end_io_fn,
    pub end_io_data: *mut core::ffi::c_void,
}

pub unsafe fn req_phys_gap_mask(req: *const request) -> c_ulong { !(((1 << (*req).phys_gap_bit) >> 1) - 1) }
pub unsafe fn req_op(req: *const request) -> req_op { (*req).cmd_flags & REQ_OP_MASK }
pub unsafe fn blk_rq_is_passthrough(rq: *const request) -> bool { blk_op_is_passthrough((*rq).cmd_flags) }
pub unsafe fn req_get_ioprio(req: *mut request) -> u16 { if !(*req).bio.is_null() { (*(*req).bio).bi_ioprio } else { 0 } }
pub unsafe fn rq_list_empty(rl: *const rq_list) -> bool { (*rl).head.is_null() }
pub unsafe fn rq_list_init(rl: *mut rq_list) { (*rl).head = core::ptr::null_mut(); (*rl).tail = core::ptr::null_mut(); }
pub unsafe fn rq_list_add_tail(rl: *mut rq_list, rq: *mut request) { (*rq).link.rq_next = core::ptr::null_mut(); if !(*rl).tail.is_null() { (*(*rl).tail).link.rq_next = rq } else { (*rl).head = rq }; (*rl).tail = rq; }
pub unsafe fn rq_list_add_head(rl: *mut rq_list, rq: *mut request) { (*rq).link.rq_next = (*rl).head; (*rl).head = rq; if (*rl).tail.is_null() { (*rl).tail = rq } }
pub unsafe fn rq_list_pop(rl: *mut rq_list) -> *mut request { let rq=(*rl).head; if !rq.is_null() { (*rl).head=(*rq).link.rq_next; if (*rl).head.is_null(){(*rl).tail=core::ptr::null_mut()} (*rq).link.rq_next=core::ptr::null_mut() } rq }
pub unsafe fn rq_list_peek(rl: *const rq_list) -> *mut request { (*rl).head }

#[repr(C)] pub enum blk_eh_timer_return { BLK_EH_DONE, BLK_EH_RESET_TIMER }
#[repr(C)] pub struct blk_mq_hw_ctx { pub dispatch_lock: spinlock_t, pub dispatch: list_head, pub state: c_ulong, pub run_work: delayed_work, pub cpumask: cpumask_var_t, pub next_cpu: c_int, pub next_cpu_batch: c_int, pub flags: c_ulong, pub sched_data: *mut core::ffi::c_void, pub queue: *mut request_queue, pub fq: *mut blk_flush_queue, pub driver_data: *mut core::ffi::c_void, pub ctx_map: sbitmap, pub dispatch_from: *mut blk_mq_ctx, pub dispatch_busy: c_uint, pub type_: u16, pub nr_ctx: u16, pub ctxs: *mut *mut blk_mq_ctx, pub dispatch_wait_lock: spinlock_t, pub dispatch_wait: wait_queue_entry_t, pub wait_index: atomic_t, pub tags: *mut blk_mq_tags, pub sched_tags: *mut blk_mq_tags, pub numa_node: c_int, pub queue_num: c_uint, pub nr_active: atomic_t, pub cpuhp_online: hlist_node, pub cpuhp_dead: hlist_node, pub kobj: kobject, pub hctx_list: list_head }
#[repr(C)] pub struct blk_mq_queue_map { pub mq_map: *mut c_uint, pub nr_queues: c_uint, pub queue_offset: c_uint }
#[repr(C)] pub enum hctx_type { HCTX_TYPE_DEFAULT, HCTX_TYPE_READ, HCTX_TYPE_POLL, HCTX_MAX_TYPES }
#[repr(C)] pub struct blk_mq_tag_set { pub ops: *const blk_mq_ops, pub map: [blk_mq_queue_map; 3], pub nr_maps: c_uint, pub nr_hw_queues: c_uint, pub queue_depth: c_uint, pub reserved_tags: c_uint, pub cmd_size: c_uint, pub numa_node: c_int, pub timeout: c_uint, pub flags: c_uint, pub driver_data: *mut core::ffi::c_void, pub tags: *mut *mut blk_mq_tags, pub shared_tags: *mut blk_mq_tags, pub tag_list_lock: mutex, pub tag_list: list_head, pub srcu: *mut srcu_struct, pub tags_srcu: srcu_struct, pub update_nr_hwq_lock: rw_semaphore }
#[repr(C)] pub struct blk_mq_queue_data { pub rq: *mut request, pub last: bool }
pub type busy_tag_iter_fn = unsafe extern "C" fn(*mut request, *mut core::ffi::c_void) -> bool;

#[repr(C)] pub struct blk_mq_ops { pub queue_rq: Option<unsafe extern "C" fn(*mut blk_mq_hw_ctx,*const blk_mq_queue_data)->blk_status_t>, pub commit_rqs: Option<unsafe extern "C" fn(*mut blk_mq_hw_ctx)>, pub queue_rqs: Option<unsafe extern "C" fn(*mut rq_list)>, pub get_budget: Option<unsafe extern "C" fn(*mut request_queue)->c_int>, pub put_budget: Option<unsafe extern "C" fn(*mut request_queue,c_int)>, pub set_rq_budget_token: Option<unsafe extern "C" fn(*mut request,c_int)>, pub get_rq_budget_token: Option<unsafe extern "C" fn(*mut request)->c_int>, pub timeout: Option<unsafe extern "C" fn(*mut request)->blk_eh_timer_return>, pub poll: Option<unsafe extern "C" fn(*mut blk_mq_hw_ctx,*mut io_comp_batch)->c_int>, pub complete: Option<unsafe extern "C" fn(*mut request)>, pub init_hctx: Option<unsafe extern "C" fn(*mut blk_mq_hw_ctx,*mut core::ffi::c_void,c_uint)->c_int>, pub exit_hctx: Option<unsafe extern "C" fn(*mut blk_mq_hw_ctx,c_uint)>, pub init_request: Option<unsafe extern "C" fn(*mut blk_mq_tag_set,*mut request,c_uint,c_int)->c_int>, pub exit_request: Option<unsafe extern "C" fn(*mut blk_mq_tag_set,*mut request,c_uint)>, pub cleanup_rq: Option<unsafe extern "C" fn(*mut request)>, pub busy: Option<unsafe extern "C" fn(*mut request_queue)->bool>, pub map_queues: Option<unsafe extern "C" fn(*mut blk_mq_tag_set)> }

pub const BLK_MQ_F_TAG_QUEUE_SHARED:u32=1<<1; pub const BLK_MQ_F_STACKING:u32=1<<2; pub const BLK_MQ_F_TAG_HCTX_SHARED:u32=1<<3; pub const BLK_MQ_F_BLOCKING:u32=1<<4; pub const BLK_MQ_F_TAG_RR:u32=1<<5; pub const BLK_MQ_F_NO_SCHED_BY_DEFAULT:u32=1<<6; pub const BLK_MQ_F_MAX:u32=1<<7; pub const BLK_MQ_MAX_DEPTH:u32=10240; pub const BLK_MQ_NO_HCTX_IDX:u32=!0;
#[repr(C)] pub enum blk_mq_state { BLK_MQ_S_STOPPED, BLK_MQ_S_TAG_ACTIVE, BLK_MQ_S_SCHED_RESTART, BLK_MQ_S_INACTIVE, BLK_MQ_S_MAX }
pub const BLK_MQ_UNIQUE_TAG_BITS:u32=16; pub const BLK_MQ_UNIQUE_TAG_MASK:u32=(1<<16)-1;
#[repr(C)] pub struct blk_mq_tags { pub nr_tags:c_uint,pub nr_reserved_tags:c_uint,pub active_queues:c_uint,pub bitmap_tags:sbitmap_queue,pub breserved_tags:sbitmap_queue,pub rqs:*mut *mut request,pub static_rqs:*mut *mut request,pub page_list:list_head,pub lock:spinlock_t,pub rcu_head:rcu_head }

pub unsafe fn blk_mq_tag_to_rq(tags:*mut blk_mq_tags, tag:c_uint)->*mut request { if tag<(*tags).nr_tags { prefetch((*tags).rqs.add(tag as usize)); *(*tags).rqs.add(tag as usize) } else { core::ptr::null_mut() } }
pub unsafe fn blk_mq_unique_tag_to_hwq(t:u32)->u16 {(t>>16) as u16} pub unsafe fn blk_mq_unique_tag_to_tag(t:u32)->u16 {(t&BLK_MQ_UNIQUE_TAG_MASK) as u16}
pub unsafe fn blk_mq_rq_state(rq:*mut request)->mq_rq_state { READ_ONCE((*rq).state) }
pub unsafe fn blk_mq_request_started(rq:*mut request)->c_int {(blk_mq_rq_state(rq)!=MQ_RQ_IDLE) as c_int}
pub unsafe fn blk_mq_request_completed(rq:*mut request)->c_int {(blk_mq_rq_state(rq)==MQ_RQ_COMPLETE) as c_int}
pub unsafe fn blk_mq_set_request_complete(rq:*mut request){WRITE_ONCE((*rq).state,MQ_RQ_COMPLETE)}
pub unsafe fn blk_mq_complete_request_direct(rq:*mut request, complete:unsafe extern "C" fn(*mut request)){WRITE_ONCE((*rq).state,MQ_RQ_COMPLETE);complete(rq)}
pub unsafe fn blk_mq_need_time_stamp(rq:*mut request)->bool {(*rq).rq_flags&(RQF_IO_STAT|RQF_STATS|RQF_USE_SCHED)!=0}
pub unsafe fn blk_mq_is_reserved_rq(rq:*mut request)->bool {(*rq).rq_flags&RQF_RESV!=0}
pub unsafe fn blk_rq_pos(rq:*const request)->sector_t{(*rq).__sector} pub unsafe fn blk_rq_bytes(rq:*const request)->c_uint{(*rq).__data_len}
pub unsafe fn blk_rq_sectors(rq:*const request)->c_uint{blk_rq_bytes(rq)>>SECTOR_SHIFT} pub unsafe fn blk_rq_cur_sectors(rq:*const request)->c_uint{(blk_rq_cur_bytes(rq) as c_uint)>>SECTOR_SHIFT}
pub unsafe fn blk_rq_stats_sectors(rq:*const request)->u16{(*rq).stats_sectors}
pub unsafe fn blk_rq_nr_phys_segments(rq:*mut request)->u16{if (*rq).rq_flags&RQF_SPECIAL_PAYLOAD!=0{1}else{(*rq).nr_phys_segments}}

/* The remaining kernel declarations are intentionally represented as external symbols. */
extern "C" { pub fn blk_mq_unique_tag(rq:*mut request)->u32; pub fn blk_mq_start_request(rq:*mut request); pub fn blk_mq_end_request(rq:*mut request,error:blk_status_t); pub fn blk_mq_free_request(rq:*mut request); pub fn blk_mq_complete_request(rq:*mut request); pub fn blk_mq_stop_hw_queue(hctx:*mut blk_mq_hw_ctx); pub fn blk_mq_start_hw_queue(hctx:*mut blk_mq_hw_ctx); pub fn blk_rq_init(q:*mut request_queue,rq:*mut request); pub fn blk_rq_map_kern(rq:*mut request,kbuf:*mut core::ffi::c_void,len:c_uint,gfp:gfp_t)->c_int; pub fn blk_rq_is_poll(rq:*mut request)->bool; pub fn blk_update_request(rq:*mut request,error:blk_status_t,nr_bytes:c_uint)->bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
