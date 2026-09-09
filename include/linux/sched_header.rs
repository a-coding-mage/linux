/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of linux/sched.h.  Types and symbols supplied by the
// included kernel headers remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

pub const TASK_RUNNING: u32 = 0x00000000;
pub const TASK_INTERRUPTIBLE: u32 = 0x00000001;
pub const TASK_UNINTERRUPTIBLE: u32 = 0x00000002;
pub const __TASK_STOPPED: u32 = 0x00000004;
pub const __TASK_TRACED: u32 = 0x00000008;
pub const EXIT_DEAD: u32 = 0x00000010;
pub const EXIT_ZOMBIE: u32 = 0x00000020;
pub const EXIT_TRACE: u32 = EXIT_ZOMBIE | EXIT_DEAD;
pub const TASK_PARKED: u32 = 0x00000040;
pub const TASK_DEAD: u32 = 0x00000080;
pub const TASK_WAKEKILL: u32 = 0x00000100;
pub const TASK_WAKING: u32 = 0x00000200;
pub const TASK_NOLOAD: u32 = 0x00000400;
pub const TASK_NEW: u32 = 0x00000800;
pub const TASK_RTLOCK_WAIT: u32 = 0x00001000;
pub const TASK_FREEZABLE: u32 = 0x00002000;
pub const TASK_FROZEN: u32 = 0x00008000;
pub const TASK_STATE_MAX: u32 = 0x00010000;
pub const TASK_ANY: u32 = TASK_STATE_MAX - 1;
pub const TASK_KILLABLE: u32 = TASK_WAKEKILL | TASK_UNINTERRUPTIBLE;
pub const TASK_STOPPED: u32 = TASK_WAKEKILL | __TASK_STOPPED;
pub const TASK_TRACED: u32 = __TASK_TRACED;
pub const TASK_IDLE: u32 = TASK_UNINTERRUPTIBLE | TASK_NOLOAD;
pub const TASK_NORMAL: u32 = TASK_INTERRUPTIBLE | TASK_UNINTERRUPTIBLE;
pub const TASK_REPORT: u32 = TASK_RUNNING | TASK_INTERRUPTIBLE | TASK_UNINTERRUPTIBLE |
    __TASK_STOPPED | __TASK_TRACED | EXIT_DEAD | EXIT_ZOMBIE | TASK_PARKED;

pub const TASK_COMM_LEN: usize = 16;
pub const SCHED_FIXEDPOINT_SHIFT: u32 = 10;
pub const SCHED_FIXEDPOINT_SCALE: i64 = 1i64 << SCHED_FIXEDPOINT_SHIFT;
pub const SCHED_CAPACITY_SHIFT: u32 = SCHED_FIXEDPOINT_SHIFT;
pub const SCHED_CAPACITY_SCALE: i64 = 1i64 << SCHED_CAPACITY_SHIFT;
pub const UTIL_EST_WEIGHT_SHIFT: u32 = 2;
pub const UTIL_AVG_UNCHANGED: u32 = 0x80000000;
pub const PERF_NR_CONTEXTS: u32 = 4;
pub const MAX_SCHEDULE_TIMEOUT: isize = isize::MAX;

#[repr(C)]
pub struct prev_cputime {
    #[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
    pub utime: u64,
    #[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
    pub stime: u64,
    #[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
    pub lock: raw_spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum vtime_state { VTIME_INACTIVE = 0, VTIME_IDLE, VTIME_SYS, VTIME_USER, VTIME_GUEST }

#[repr(C)]
pub struct vtime { pub seqcount: seqcount_t, pub starttime: u64, pub state: vtime_state,
    pub cpu: u32, pub utime: u64, pub stime: u64, pub gtime: u64 }

#[repr(C)]
pub enum uclamp_id { UCLAMP_MIN = 0, UCLAMP_MAX, UCLAMP_CNT }

#[repr(C)]
pub struct sched_param { pub sched_priority: i32 }

#[repr(C)]
pub struct load_weight { pub weight: usize, pub inv_weight: u32 }

#[repr(C, align(64))]
pub struct sched_avg { pub last_update_time: u64, pub load_sum: u64, pub runnable_sum: u64,
    pub util_sum: u32, pub period_contrib: u32, pub load_avg: usize,
    pub runnable_avg: usize, pub util_avg: usize, pub util_est: u32 }

#[repr(C, align(64))]
pub struct sched_statistics {
    #[cfg(CONFIG_SCHEDSTATS)]
    pub wait_start: u64, #[cfg(CONFIG_SCHEDSTATS)] pub wait_max: u64,
    #[cfg(CONFIG_SCHEDSTATS)] pub wait_count: u64, #[cfg(CONFIG_SCHEDSTATS)] pub wait_sum: u64,
    #[cfg(CONFIG_SCHEDSTATS)] pub iowait_count: u64, #[cfg(CONFIG_SCHEDSTATS)] pub iowait_sum: u64,
    #[cfg(CONFIG_SCHEDSTATS)] pub sleep_start: u64, #[cfg(CONFIG_SCHEDSTATS)] pub sleep_max: u64,
    #[cfg(CONFIG_SCHEDSTATS)] pub sum_sleep_runtime: i64, #[cfg(CONFIG_SCHEDSTATS)] pub block_start: u64,
    #[cfg(CONFIG_SCHEDSTATS)] pub block_max: u64, #[cfg(CONFIG_SCHEDSTATS)] pub sum_block_runtime: i64,
    #[cfg(CONFIG_SCHEDSTATS)] pub exec_max: i64, #[cfg(CONFIG_SCHEDSTATS)] pub slice_max: u64,
}

#[repr(C)]
pub struct sched_entity { pub load: load_weight, pub h_load: load_weight, pub run_node: rb_node,
    pub deadline: u64, pub min_vruntime: u64, pub min_slice: u64, pub max_slice: u64,
    pub group_node: list_head, pub on_rq: u8, pub sched_delayed: u8, pub rel_deadline: u8,
    pub custom_slice: u8, pub exec_start: u64, pub sum_exec_runtime: u64, pub prev_sum_exec_runtime: u64,
    pub vruntime: u64, pub vlag: i64, pub vprot: u64, pub slice: u64, pub nr_migrations: u64,
    pub avg: sched_avg }

#[repr(C)]
pub struct sched_rt_entity { pub run_list: list_head, pub timeout: usize, pub watchdog_stamp: usize,
    pub time_slice: u32, pub on_rq: u16, pub on_list: u16, pub back: *mut sched_rt_entity }

#[repr(C)]
pub struct sched_dl_entity { pub rb_node: rb_node, pub dl_runtime: u64, pub dl_deadline: u64,
    pub dl_period: u64, pub dl_bw: u64, pub dl_density: u64, pub runtime: i64, pub deadline: u64,
    pub flags: u32, pub dl_throttled: u32, pub dl_yielded: u32, pub dl_non_contending: u32,
    pub dl_overrun: u32, pub dl_server: u32, pub dl_server_active: u32, pub dl_defer: u32,
    pub dl_defer_armed: u32, pub dl_defer_running: u32, pub dl_defer_idle: u32,
    pub dl_bw_attached: u32, pub dl_timer: hrtimer, pub inactive_timer: hrtimer,
    pub rq: *mut rq, pub server_pick_task: Option<unsafe extern "C" fn(*mut sched_dl_entity, *mut rq_flags) -> *mut task_struct> }

#[repr(C)]
pub union rcu_special { pub b: rcu_special_bits, pub s: u32 }
#[repr(C)] pub struct rcu_special_bits { pub blocked: u8, pub need_qs: u8, pub exp_hint: u8, pub need_mb: u8 }
#[repr(C)] pub struct wake_q_node { pub next: *mut wake_q_node }
#[repr(C)] pub struct task_ipi_mask { pub ipi_mask_ptr: *mut cpumask_t }

#[repr(C)]
pub struct task_struct {
    pub __state: u32, pub saved_state: u32, pub stack: *mut core::ffi::c_void,
    pub usage: refcount_t, pub flags: u32, pub ptrace: u32, pub on_cpu: u8, pub on_rq: u8,
    pub is_blocked: u8, pub wakee_flips: u32, pub wakee_flip_decay_ts: usize,
    pub last_wakee: *mut task_struct, pub recent_used_cpu: i32, pub wake_cpu: i32,
    pub prio: i32, pub static_prio: i32, pub normal_prio: i32, pub rt_priority: u32,
    pub se: sched_entity, pub rt: sched_rt_entity, pub dl: sched_dl_entity,
    pub dl_server: *mut sched_dl_entity, pub sched_class: *const sched_class,
    pub sched_info: sched_info, pub tasks: list_head, pub mm: *mut mm_struct,
    pub active_mm: *mut mm_struct, pub exit_state: i32, pub exit_code: i32, pub exit_signal: i32,
    pub pdeath_signal: i32, pub jobctl: usize, pub personality: u32,
    pub pid: pid_t, pub tgid: pid_t, pub real_parent: *mut task_struct, pub parent: *mut task_struct,
    pub children: list_head, pub sibling: list_head, pub group_leader: *mut task_struct,
    pub thread_pid: *mut pid, pub utime: u64, pub stime: u64, pub gtime: u64,
    pub prev_cputime: prev_cputime, pub nvcsw: usize, pub nivcsw: usize,
    pub start_time: u64, pub start_boottime: u64, pub min_flt: usize, pub maj_flt: usize,
    pub comm: [u8; TASK_COMM_LEN], pub signal: *mut signal_struct, pub blocked: sigset_t,
    pub real_blocked: sigset_t, pub saved_sigmask: sigset_t, pub pending: sigpending,
    pub alloc_lock: spinlock_t, pub pi_lock: raw_spinlock_t, pub wake_q: wake_q_node,
    pub atomic_flags: usize, pub rcu: rcu_head, pub pagefault_disabled: i32,
    pub thread: thread_struct,
}

#[repr(C)] pub struct sched_info { #[cfg(CONFIG_SCHED_INFO)] pub pcount: usize,
    #[cfg(CONFIG_SCHED_INFO)] pub run_delay: u64, #[cfg(CONFIG_SCHED_INFO)] pub max_run_delay: u64,
    #[cfg(CONFIG_SCHED_INFO)] pub min_run_delay: u64, #[cfg(CONFIG_SCHED_INFO)] pub last_arrival: u64,
    #[cfg(CONFIG_SCHED_INFO)] pub last_queued: u64 }

extern "C" { pub fn sched_tick(); pub fn schedule_timeout(timeout: isize) -> isize;
    pub fn schedule_timeout_interruptible(timeout: isize) -> isize;
    pub fn schedule_timeout_killable(timeout: isize) -> isize;
    pub fn schedule_timeout_uninterruptible(timeout: isize) -> isize;
    pub fn schedule_timeout_idle(timeout: isize) -> isize; pub fn schedule();
    pub fn schedule_preempt_disabled(); pub fn preempt_schedule_irq();
    pub static mut def_root_domain: root_domain; pub static mut sched_domains_mutex: mutex;
    pub fn sched_domains_mutex_lock(); pub fn sched_domains_mutex_unlock(); pub static mut cad_pid: *mut pid; }

#[inline] pub unsafe fn task_is_running(task: *const task_struct) -> bool { (*task).__state == TASK_RUNNING }
#[inline] pub unsafe fn is_user_task(task: *const task_struct) -> bool { !(*task).mm.is_null() && ((*task).flags & (0x00200000 | 0x00004000)) == 0 }
pub const TASK_REPORT_IDLE: u32 = TASK_REPORT + 1;
pub const TASK_REPORT_MAX: u32 = TASK_REPORT_IDLE << 1;

// The remaining kernel configuration-specific declarations are intentionally
// represented by their original external types and conditional intent.
extern "C" { pub static mut current: *mut task_struct; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
