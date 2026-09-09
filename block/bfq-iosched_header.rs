/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of bfq-iosched.h. Kernel dependencies are supplied externally. */

pub const BFQ_IOPRIO_CLASSES: usize = 3;
pub const BFQ_CL_IDLE_TIMEOUT: u64 = HZ / 5;
pub const BFQ_MIN_WEIGHT: u32 = 1;
pub const BFQ_MAX_WEIGHT: u32 = 1000;
pub const BFQ_WEIGHT_CONVERSION_COEFF: u32 = 10;
pub const BFQ_DEFAULT_QUEUE_IOPRIO: u16 = 4;
pub const BFQ_DEFAULT_GRP_IOPRIO: u16 = 0;
pub const BFQ_DEFAULT_GRP_CLASS: u16 = IOPRIO_CLASS_BE;
pub const MAX_BFQQ_NAME_LENGTH: usize = 16;
pub const BFQ_SOFTRT_WEIGHT_FACTOR: u32 = 100;
pub const BFQ_MAX_ACTUATORS: usize = 8;

pub struct bfq_entity;
pub struct bfq_group;
pub struct bfq_data;
pub struct bfq_queue;
pub struct bfq_io_cq;
pub struct request;
pub struct bio;
pub struct request_queue;
pub struct blkcg_gq;
pub struct cftype;
pub struct blkcg_policy;
pub struct rb_root;
pub struct rb_root_cached;
pub struct rb_node;
pub struct list_head;
pub struct hlist_node;
pub struct hlist_head;
pub struct io_cq;
pub struct hrtimer;
pub struct spinlock_t;
pub struct blk_independent_access_range;
pub struct blkcg_policy_data;
pub struct blkg_rwstat;
pub struct percpu_counter;
pub struct atomic64_t;
pub struct ktime_t;
pub type sector_t = u64;
pub type pid_t = i32;
pub type blk_opf_t = u32;
extern "C" { static HZ: u64; static IOPRIO_CLASS_BE: u16; }

#[repr(C)]
pub struct bfq_service_tree { pub active: rb_root, pub idle: rb_root, pub first_idle: *mut bfq_entity, pub last_idle: *mut bfq_entity, pub vtime: u64, pub wsum: usize }
#[repr(C)]
pub struct bfq_sched_data { pub in_service_entity: *mut bfq_entity, pub next_in_service: *mut bfq_entity, pub service_tree: [bfq_service_tree; BFQ_IOPRIO_CLASSES], pub bfq_class_idle_last_service: usize }
#[repr(C)]
pub struct bfq_weight_counter { pub weight: u32, pub num_active: u32, pub weights_node: rb_node }

#[repr(C)]
pub struct bfq_entity {
    pub rb_node: rb_node, pub on_st_or_in_serv: bool, pub start: u64, pub finish: u64,
    pub tree: *mut rb_root, pub min_start: u64, pub service: i32, pub budget: i32, pub allocated: i32,
    pub dev_weight: i32, pub weight: i32, pub new_weight: i32, pub orig_weight: i32,
    pub parent: *mut bfq_entity, pub my_sched_data: *mut bfq_sched_data, pub sched_data: *mut bfq_sched_data,
    pub prio_changed: i32, pub last_bfqq_created: *mut bfq_queue,
}
#[repr(C)] pub struct bfq_ttime { pub last_end_request: u64, pub ttime_total: u64, pub ttime_samples: usize, pub ttime_mean: u64 }

#[repr(C)]
pub struct bfq_queue {
    pub ref_: i32, pub stable_ref: i32, pub bfqd: *mut bfq_data, pub ioprio: u16, pub ioprio_class: u16,
    pub new_ioprio: u16, pub new_ioprio_class: u16, pub last_serv_time_ns: u64, pub inject_limit: u32,
    pub decrease_time_jif: usize, pub new_bfqq: *mut bfq_queue, pub pos_node: rb_node, pub pos_root: *mut rb_root,
    pub sort_list: rb_root, pub next_rq: *mut request, pub queued: [i32;2], pub meta_pending: i32, pub fifo: list_head,
    pub entity: bfq_entity, pub weight_counter: *mut bfq_weight_counter, pub max_budget: i32, pub budget_timeout: usize,
    pub dispatched: i32, pub flags: usize, pub bfqq_list: list_head, pub ttime: bfq_ttime, pub io_start_time: u64,
    pub tot_idle_time: u64, pub seek_history: u32, pub burst_list_node: hlist_node, pub last_request_pos: sector_t,
    pub requests_within_timer: u32, pub pid: pid_t, pub bic: *mut bfq_io_cq, pub wr_cur_max_time: usize,
    pub soft_rt_next_start: usize, pub last_wr_start_finish: usize, pub wr_coeff: u32, pub last_idle_bklogged: usize,
    pub service_from_backlogged: usize, pub service_from_wr: usize, pub wr_start_at_switch_to_srt: usize,
    pub split_time: usize, pub first_IO_time: usize, pub creation_time: usize, pub waker_bfqq: *mut bfq_queue,
    pub tentative_waker_bfqq: *mut bfq_queue, pub num_waker_detections: u32, pub waker_detection_started: u64,
    pub woken_list_node: hlist_node, pub woken_list: hlist_head, pub actuator_idx: u32,
}
#[repr(C)] pub struct bfq_iocq_bfqq_data { pub saved_has_short_ttime: bool, pub saved_IO_bound: bool, pub saved_in_large_burst: bool, pub was_in_burst_list: bool, pub saved_weight: u32, pub saved_io_start_time: u64, pub saved_tot_idle_time: u64, pub saved_wr_coeff: usize, pub saved_last_wr_start_finish: usize, pub saved_service_from_wr: usize, pub saved_wr_start_at_switch_to_srt: usize, pub saved_ttime: bfq_ttime, pub saved_wr_cur_max_time: u32, pub saved_inject_limit: u32, pub saved_decrease_time_jif: usize, pub saved_last_serv_time_ns: u64, pub stable_merge_bfqq: *mut bfq_queue, pub stably_merged: bool }
#[repr(C)] pub struct bfq_io_cq { pub icq: io_cq, pub bfqq: [[*mut bfq_queue; BFQ_MAX_ACTUATORS];2], pub ioprio: i32, pub bfqq_data: [bfq_iocq_bfqq_data; BFQ_MAX_ACTUATORS], pub requests: u32 }

#[repr(C)]
pub struct bfq_data {
    pub queue: *mut request_queue, pub dispatch: list_head, pub root_group: *mut bfq_group, pub queue_weights_tree: rb_root_cached,
    pub busy_queues: [u32;3], pub wr_busy_queues: i32, pub queued: i32, pub tot_rq_in_driver: i32, pub rq_in_driver: [i32;BFQ_MAX_ACTUATORS],
    pub nonrot_with_queueing: bool, pub max_rq_in_driver: i32, pub hw_tag_samples: i32, pub hw_tag: i32, pub budgets_assigned: i32,
    pub idle_slice_timer: hrtimer, pub in_service_queue: *mut bfq_queue, pub last_position: sector_t, pub in_serv_last_pos: sector_t,
    pub last_completion: u64, pub last_completed_rq_bfqq: *mut bfq_queue, pub last_bfqq_created: *mut bfq_queue, pub last_empty_occupied_ns: u64,
    pub wait_dispatch: bool, pub waited_rq: *mut request, pub rqs_injected: bool, pub first_dispatch: u64, pub last_dispatch: u64,
    pub last_budget_start: ktime_t, pub last_idling_start: ktime_t, pub last_idling_start_jiffies: usize, pub peak_rate_samples: i32,
    pub sequential_samples: u32, pub tot_sectors_dispatched: u64, pub last_rq_max_size: u32, pub delta_from_first: u64, pub peak_rate: u32,
    pub bfq_max_budget: i32, pub active_list: [list_head;BFQ_MAX_ACTUATORS], pub idle_list: list_head, pub bfq_fifo_expire: [u64;2],
    pub bfq_back_penalty: u32, pub bfq_back_max: u32, pub bfq_slice_idle: u32, pub bfq_user_max_budget: i32, pub bfq_timeout: u32,
    pub strict_guarantees: bool, pub last_ins_in_burst: usize, pub bfq_burst_interval: usize, pub burst_size: i32, pub burst_parent_entity: *mut bfq_entity,
    pub bfq_large_burst_thresh: usize, pub large_burst: bool, pub burst_list: hlist_head, pub low_latency: bool, pub bfq_wr_coeff: u32,
    pub bfq_wr_rt_max_time: u32, pub bfq_wr_min_idle_time: u32, pub bfq_wr_min_inter_arr_async: usize, pub bfq_wr_max_softrt_rate: u32,
    pub rate_dur_prod: u64, pub oom_bfqq: bfq_queue, pub lock: spinlock_t, pub bio_bic: *mut bfq_io_cq, pub bio_bfqq: *mut bfq_queue,
    pub async_depths: [[u32;2];2], pub num_actuators: u32, pub sector: [sector_t;BFQ_MAX_ACTUATORS], pub nr_sectors: [sector_t;BFQ_MAX_ACTUATORS],
    pub ia_ranges: [blk_independent_access_range;BFQ_MAX_ACTUATORS], pub actuator_load_threshold: u32,
}

#[repr(i32)] pub enum bfqq_state_flags { BFQQF_just_created=0, BFQQF_busy, BFQQF_wait_request, BFQQF_non_blocking_wait_rq, BFQQF_fifo_expire, BFQQF_has_short_ttime, BFQQF_sync, BFQQF_IO_bound, BFQQF_in_large_burst, BFQQF_softrt_update, BFQQF_coop, BFQQF_split_coop }
#[repr(i32)] pub enum bfqq_expiration { BFQQE_TOO_IDLE=0, BFQQE_BUDGET_TIMEOUT, BFQQE_BUDGET_EXHAUSTED, BFQQE_NO_MORE_REQUESTS, BFQQE_PREEMPTED }
#[repr(C)] pub struct bfq_stat { pub cpu_cnt: percpu_counter, pub aux_cnt: atomic64_t }
#[repr(C)] pub struct bfqg_stats { pub bytes: blkg_rwstat, pub ios: blkg_rwstat }
#[repr(C)] pub struct bfq_group_data { pub pd: blkcg_policy_data, pub weight: u32 }
#[repr(C)] pub struct bfq_group { pub entity: bfq_entity, pub sched_data: bfq_sched_data, pub bfqd: *mut bfq_data, pub async_bfqq: [[[ *mut bfq_queue; BFQ_MAX_ACTUATORS]; IOPRIO_NR_LEVELS];2], pub async_idle_bfqq: [*mut bfq_queue;BFQ_MAX_ACTUATORS], pub my_entity: *mut bfq_entity, pub active_entities: i32, pub num_queues_with_pending_reqs: i32, pub rq_pos_tree: rb_root, pub stats: bfqg_stats }

pub const BFQ_SERVICE_TREE_INIT: bfq_service_tree = bfq_service_tree { active: rb_root {}, idle: rb_root {}, first_idle: core::ptr::null_mut(), last_idle: core::ptr::null_mut(), vtime: 0, wsum: 0 };
extern "C" { pub static bfq_timeout: i32; }

extern "C" {
    pub fn bic_to_bfqq(bic:*mut bfq_io_cq,is_sync:bool,actuator_idx:u32)->*mut bfq_queue; pub fn bic_set_bfqq(bic:*mut bfq_io_cq,bfqq:*mut bfq_queue,is_sync:bool,actuator_idx:u32); pub fn bic_to_bfqd(bic:*mut bfq_io_cq)->*mut bfq_data;
    pub fn bfq_pos_tree_add_move(bfqd:*mut bfq_data,bfqq:*mut bfq_queue); pub fn bfq_weights_tree_add(bfqq:*mut bfq_queue); pub fn bfq_weights_tree_remove(bfqq:*mut bfq_queue);
    pub fn bfq_bfqq_expire(bfqd:*mut bfq_data,bfqq:*mut bfq_queue,compensate:bool,reason:bfqq_expiration); pub fn bfq_put_queue(bfqq:*mut bfq_queue); pub fn bfq_put_cooperator(bfqq:*mut bfq_queue); pub fn bfq_schedule_dispatch(bfqd:*mut bfq_data);
    pub fn bfq_entity_to_bfqq(entity:*mut bfq_entity)->*mut bfq_queue; pub fn bfq_tot_busy_queues(bfqd:*mut bfq_data)->u32; pub fn bfq_entity_service_tree(entity:*mut bfq_entity)->*mut bfq_service_tree; pub fn bfq_entity_of(node:*mut rb_node)->*mut bfq_entity; pub fn bfq_ioprio_to_weight(ioprio:i32)->u16;
}

pub unsafe fn bfq_bfqq_name(bfqq:*mut bfq_queue, str_:*mut i8, len:i32) { let ty = if bfq_bfqq_sync(bfqq)!=0 { b'S' } else { b'A' }; if (*bfqq).pid != -1 { snprintf(str_,len,b"bfq%d%c\0".as_ptr() as *const i8,(*bfqq).pid,ty as i32); } else { snprintf(str_,len,b"bfqSHARED-%c\0".as_ptr() as *const i8,ty as i32); } }
extern "C" { fn bfq_bfqq_sync(bfqq:*const bfq_queue)->i32; fn snprintf(s:*mut i8,n:i32,fmt:*const i8,...); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
