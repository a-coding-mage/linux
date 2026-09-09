/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of sched.h; included kernel symbols remain external dependencies. */

#[repr(C)] pub struct rq { _private: [u8; 0] }
#[repr(C)] pub struct cfs_rq { _private: [u8; 0] }
#[repr(C)] pub struct rt_rq { _private: [u8; 0] }
#[repr(C)] pub struct sched_group { _private: [u8; 0] }
#[repr(C)] pub struct cpuidle_state { _private: [u8; 0] }

pub const TASK_ON_RQ_QUEUED: i32 = 1;
pub const TASK_ON_RQ_MIGRATING: i32 = 2;
pub const DL_SCALE: i32 = 10;
pub const SCHED_FLAG_SUGOV: u32 = 0x10000000;
pub const MIN_SHARES: usize = 1usize << 1;
pub const MAX_SHARES: usize = 1usize << 18;
pub const MDF_PUSH: u32 = 0x01;

#[cfg(target_pointer_width = "64")]
pub const NICE_0_LOAD_SHIFT: u32 = SCHED_FIXEDPOINT_SHIFT + SCHED_FIXEDPOINT_SHIFT;
#[cfg(not(target_pointer_width = "64"))]
pub const NICE_0_LOAD_SHIFT: u32 = SCHED_FIXEDPOINT_SHIFT;
pub const NICE_0_LOAD: i64 = 1i64 << NICE_0_LOAD_SHIFT;
pub const RUNTIME_INF: u64 = u64::MAX;

#[inline] pub fn idle_policy(policy: i32) -> i32 { (policy == SCHED_IDLE) as i32 }
#[inline] pub fn normal_policy(policy: i32) -> i32 {
    #[cfg(feature = "CONFIG_SCHED_CLASS_EXT")] if policy == SCHED_EXT { return 1; }
    (policy == SCHED_NORMAL) as i32
}
#[inline] pub fn fair_policy(policy: i32) -> i32 { (normal_policy(policy) != 0 || policy == SCHED_BATCH) as i32 }
#[inline] pub fn rt_policy(policy: i32) -> i32 { (policy == SCHED_FIFO || policy == SCHED_RR) as i32 }
#[inline] pub fn dl_policy(policy: i32) -> i32 { (policy == SCHED_DEADLINE) as i32 }
#[inline] pub fn valid_policy(policy: i32) -> bool { idle_policy(policy) != 0 || fair_policy(policy) != 0 || rt_policy(policy) != 0 || dl_policy(policy) != 0 }

#[repr(C)] pub struct asym_cap_data { pub link: list_head, pub rcu: rcu_head, pub capacity: c_ulong, pub cpus: [c_ulong; 0] }
#[repr(C)] pub struct rt_prio_array { pub bitmap: [c_ulong; 0], pub queue: [list_head; 0] }
#[repr(C)] pub struct rt_bandwidth { pub rt_runtime_lock: raw_spinlock_t, pub rt_period: ktime_t, pub rt_runtime: u64, pub rt_period_timer: hrtimer, pub rt_period_active: c_uint }
#[repr(C)] pub struct dl_bw { pub lock: raw_spinlock_t, pub bw: u64, pub total_bw: u64 }
#[repr(C)] pub struct cfs_bandwidth { pub _private: [u8; 0] }
#[repr(C)] pub struct balance_callback { pub next: *mut balance_callback, pub func: Option<unsafe extern "C" fn(*mut rq)> }
#[repr(C)] pub struct perf_domain { pub em_pd: *mut em_perf_domain, pub next: *mut perf_domain, pub rcu: rcu_head }
#[repr(C)] pub struct root_domain { pub refcount: atomic_t, pub rto_count: atomic_t, pub rcu: rcu_head, pub span: cpumask_var_t, pub online: cpumask_var_t, pub overloaded: bool, pub overutilized: bool, pub dlo_mask: cpumask_var_t, pub dlo_count: atomic_t, pub dl_bw: dl_bw, pub cpudl: cpudl, pub visit_cookie: u64, pub rto_mask: cpumask_var_t, pub cpupri: cpupri, pub pd: *mut perf_domain }
#[repr(C)] pub struct uclamp_bucket { pub value: c_ulong, pub tasks: c_ulong }
#[repr(C)] pub struct uclamp_rq { pub value: c_uint, pub bucket: [uclamp_bucket; UCLAMP_BUCKETS as usize] }

#[cfg(feature = "CONFIG_SCHED_CLASS_EXT")]
#[repr(C)] pub struct scx_rq { pub local_dsq: scx_dispatch_q, pub runnable_list: list_head, pub ddsp_deferred_locals: list_head, pub ops_qseq: c_ulong, pub remote_activate_enq_flags: u64, pub remote_activate_sch: *mut scx_sched, pub nr_running: u32, pub cpuperf_target: u32, pub in_select_cpu: bool, pub cpu_released: bool, pub flags: u32, pub nr_immed: u32, pub clock: u64, pub cpus_to_sync: cpumask_var_t, pub kick_sync_pending: bool, pub kick_sync: c_ulong, pub sched_pcpus_to_kick: list_head, pub deferred_reenq_lock: raw_spinlock_t, pub deferred_reenq_locals: list_head, pub deferred_reenq_users: list_head, pub deferred_bal_cb: balance_callback, pub kick_sync_bal_cb: balance_callback, pub deferred_irq_work: irq_work, pub kick_cpus_irq_work: irq_work }

#[repr(C)] pub struct dl_rq { pub root: rb_root_cached, pub dl_nr_running: c_uint, pub earliest_dl: sched_dl_times, pub overloaded: bool, pub curr: *mut sched_dl_entity, pub pushable_dl_tasks_root: rb_root_cached, pub running_bw: u64, pub this_bw: u64, pub extra_bw: u64, pub max_bw: u64, pub bw_ratio: u64 }
#[repr(C)] pub struct sched_dl_times { pub curr: u64, pub next: u64 }

#[inline] pub unsafe fn dl_bandwidth_enabled() -> i32 { (sysctl_sched_rt_runtime >= 0) as i32 }
#[inline] pub unsafe fn rt_bandwidth_enabled() -> i32 { (sysctl_sched_rt_runtime >= 0) as i32 }
#[inline] pub unsafe fn update_avg(avg: *mut u64, sample: u64) { let diff = sample.wrapping_sub(*avg); *avg = (*avg).wrapping_add((diff as i64 / 8) as u64); }
#[inline] pub unsafe fn rt_rq_is_runnable(rt: *mut rt_rq) -> bool { (*rt).rt_queued != 0 && (*rt).rt_nr_running != 0 }

extern "C" {
    pub static mut scheduler_running: c_int;
    pub static mut calc_load_update: c_ulong;
    pub static mut calc_load_tasks: atomic_long_t;
    pub static mut sysctl_sched_rt_period: c_int;
    pub static mut sysctl_sched_rt_runtime: c_int;
    pub static mut sched_rr_timeslice: c_int;
    pub static mut asym_cap_list: list_head;
    pub static mut root_task_group: task_group;
    pub fn calc_global_load_tick(this_rq: *mut rq);
    pub fn calc_load_fold_active(this_rq: *mut rq, adjust: c_long) -> c_long;
    pub fn call_trace_sched_update_nr_running(rq: *mut rq, count: c_int);
    pub fn init_dl_bw(dl_b: *mut dl_bw);
    pub fn sched_dl_global_validate() -> c_int;
    pub fn sched_dl_do_global();
    pub fn sched_dl_overflow(p: *mut task_struct, policy: c_int, attr: *const sched_attr) -> c_int;
    pub fn dl_server_update(dl_se: *mut sched_dl_entity, delta_exec: i64);
    pub fn dl_server_start(dl_se: *mut sched_dl_entity);
    pub fn dl_server_stop(dl_se: *mut sched_dl_entity);
    pub fn dl_server_init(dl_se: *mut sched_dl_entity, rq: *mut rq, pick_task: dl_server_pick_f);
    pub fn sched_init_dl_servers();
    pub fn sched_init_domains(cpu_map: *const cpumask) -> c_int;
    pub fn rq_attach_root(rq: *mut rq, rd: *mut root_domain);
}

// External kernel types and constants referenced above are supplied by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
