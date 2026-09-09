/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of tree.h. Included kernel types are external dependencies. */

/* C build configuration controls these declarations in the original header. */
extern "C" {
    pub static HZ: ::core::ffi::c_ulong;
}

pub const RCU_KTHREAD_STOPPED: i32 = 0;
pub const RCU_KTHREAD_RUNNING: i32 = 1;
pub const RCU_KTHREAD_WAITING: i32 = 2;
pub const RCU_KTHREAD_OFFCPU: i32 = 3;
pub const RCU_KTHREAD_YIELDING: i32 = 4;
pub const RCU_KTHREAD_MAX: i32 = 4;
pub const DEFER_QS_IDLE: i32 = 0;
pub const DEFER_QS_PENDING: i32 = 1;
pub const RCU_NOCB_WAKE_NOT: i32 = 0;
pub const RCU_NOCB_WAKE_BYPASS: i32 = 1;
pub const RCU_NOCB_WAKE_LAZY: i32 = 2;
pub const RCU_NOCB_WAKE: i32 = 3;
pub const SR_MAX_USERS_WAKE_FROM_GP: usize = 5;
pub const SR_NORMAL_GP_WAIT_HEAD_MAX: usize = 5;
pub const RCU_GP_FLAG_INIT: i32 = 0x1;
pub const RCU_GP_FLAG_FQS: i32 = 0x2;
pub const RCU_GP_FLAG_OVLD: i32 = 0x4;
pub const RCU_GP_IDLE: i16 = 0;
pub const RCU_GP_WAIT_GPS: i16 = 1;
pub const RCU_GP_DONE_GPS: i16 = 2;
pub const RCU_GP_ONOFF: i16 = 3;
pub const RCU_GP_INIT: i16 = 4;
pub const RCU_GP_WAIT_FQS: i16 = 5;
pub const RCU_GP_DOING_FQS: i16 = 6;
pub const RCU_GP_CLEANUP: i16 = 7;
pub const RCU_GP_CLEANED: i16 = 8;

/* External kernel types. */
pub type raw_spinlock_t = u8; pub type spinlock_t = u8; pub type arch_spinlock_t = u8;
pub type kthread_work = u8; pub type kthread_worker = u8; pub type list_head = u8;
pub type rt_mutex = u8; pub type mutex = u8; pub type task_struct = u8;
pub type swait_queue_head = u8; pub type wait_queue_head_t = u8; pub type work_struct = u8;
pub type irq_work = u8; pub type rcu_head = u8; pub type rcu_segcblist = u8;
pub type rcu_cblist = u8; pub type rcu_gp_seq = u8; pub type llist_head = u8;
pub type llist_node = u8; pub type atomic_t = u8; pub type completion = u8; pub type timer_list = u8;

#[repr(C)] pub struct rcu_exp_work { pub rew_s: usize, pub rew_work: kthread_work }
#[repr(C)] pub struct rcu_node {
    pub lock: raw_spinlock_t, pub gp_seq: usize, pub gp_seq_needed: usize, pub completedqs: usize,
    pub qsmask: usize, pub rcu_gp_init_mask: usize, pub qsmaskinit: usize, pub qsmaskinitnext: usize,
    pub expmask: usize, pub expmaskinit: usize, pub expmaskinitnext: usize,
    pub exp_kworker: *mut kthread_worker, pub cbovldmask: usize, pub ffmask: usize, pub grpmask: usize,
    pub grplo: i32, pub grphi: i32, pub grpnum: u8, pub level: u8, pub wait_blkd_tasks: bool,
    pub parent: *mut rcu_node, pub blkd_tasks: list_head, pub gp_tasks: *mut list_head,
    pub exp_tasks: *mut list_head, pub boost_tasks: *mut list_head, pub boost_mtx: rt_mutex,
    pub boost_time: usize, pub kthread_mutex: mutex, pub boost_kthread_task: *mut task_struct,
    pub boost_kthread_status: u32, pub n_boosts: usize, pub fqslock: raw_spinlock_t,
    pub exp_lock: spinlock_t, pub exp_seq_rq: usize, pub exp_wq: [wait_queue_head_t; 4],
    pub rew: rcu_exp_work, pub exp_need_flush: bool, pub exp_poll_lock: raw_spinlock_t,
    pub exp_seq_poll_rq: usize, pub exp_poll_wq: work_struct,
}
#[repr(C)] pub union rcu_noqs { pub b: rcu_noqs_bits, pub s: u16 }
#[repr(C)] pub struct rcu_noqs_bits { pub norm: u8, pub exp: u8 }
#[repr(C)] pub struct rcu_snap_record { pub gp_seq: usize, pub cputime_irq: u64, pub cputime_softirq: u64, pub cputime_system: u64, pub nr_hardirqs: u64, pub nr_softirqs: u32, pub nr_csw: u64, pub jiffies: usize }
#[repr(C)] pub struct rcu_data {
    pub gp_seq: usize, pub gp_seq_needed: usize, pub cpu_no_qs: rcu_noqs, pub core_needs_qs: bool,
    pub beenonline: bool, pub gpwrap: bool, pub gpwrap_count: u32, pub cpu_started: bool,
    pub mynode: *mut rcu_node, pub grpmask: usize, pub ticks_this_gp: usize, pub defer_qs_iw: irq_work,
    pub defer_qs_pending: i32, pub strict_work: work_struct, pub cblist: rcu_segcblist,
    pub qlen_last_fqs_check: isize, pub n_cbs_invoked: usize, pub n_force_qs_snap: usize, pub blimit: isize,
    pub watching_snap: i32, pub rcu_need_heavy_qs: bool, pub rcu_urgent_qs: bool, pub rcu_forced_tick: bool,
    pub rcu_forced_tick_exp: bool, pub barrier_seq_snap: usize, pub barrier_head: rcu_head,
    pub exp_watching_snap: i32, pub rcu_cpu_kthread_task: *mut task_struct, pub rcu_cpu_kthread_status: u32,
    pub rcu_cpu_has_work: i8, pub rcuc_activity: usize, pub softirq_snap: u32, pub rcu_iw: irq_work,
    pub rcu_iw_pending: bool, pub rcu_iw_gp_seq: usize, pub rcu_ofl_gp_seq: usize, pub rcu_ofl_gp_state: i16,
    pub rcu_onl_gp_seq: usize, pub rcu_onl_gp_state: i16, pub last_fqs_resched: usize,
    pub last_sched_clock: usize, pub snap_record: rcu_snap_record, pub lazy_len: isize, pub cpu: i32,
}
#[inline] pub unsafe fn rcu_defer_qs_clear(rdp: *mut rcu_data) { (*rdp).defer_qs_pending = DEFER_QS_IDLE; }
#[repr(C)] pub struct sr_wait_node { pub inuse: atomic_t, pub node: llist_node }
#[repr(C)] pub struct rcu_state {
    pub node: *mut rcu_node, pub level: *mut *mut rcu_node, pub ncpus: i32, pub n_online_cpus: i32,
    pub gp_seq: usize, pub gp_max: usize, pub gp_kthread: *mut task_struct, pub gp_wq: swait_queue_head,
    pub gp_flags: i16, pub gp_state: i16, pub gp_wake_time: usize, pub gp_wake_seq: usize,
    pub gp_seq_polled: usize, pub gp_seq_polled_snap: usize, pub gp_seq_polled_exp_snap: usize,
    pub barrier_mutex: mutex, pub barrier_cpu_count: atomic_t, pub barrier_completion: completion,
    pub barrier_sequence: usize, pub barrier_lock: raw_spinlock_t, pub exp_mutex: mutex, pub exp_wake_mutex: mutex,
    pub expedited_sequence: usize, pub expedited_wq: swait_queue_head, pub ncpus_snap: i32, pub cbovld: u8, pub cbovldnext: u8,
    pub jiffies_force_qs: usize, pub jiffies_kick_kthreads: usize, pub n_force_qs: usize, pub gp_start: usize,
    pub gp_end: usize, pub gp_activity: usize, pub gp_req_activity: usize, pub jiffies_stall: usize,
    pub nr_fqs_jiffies_stall: i32, pub jiffies_resched: usize, pub n_force_qs_gpstart: usize,
    pub name: *const i8, pub abbr: i8, pub ofl_lock: arch_spinlock_t, pub srs_next: llist_head,
    pub srs_wait_tail: *mut llist_node, pub srs_done_tail: *mut llist_node,
    pub srs_wait_nodes: [sr_wait_node; SR_NORMAL_GP_WAIT_HEAD_MAX], pub srs_cleanup_work: work_struct,
    pub srs_cleanups_pending: atomic_t,
}

/* The following are declaration-only interfaces from the original header. */
extern "C" {
    fn rcu_bootup_announce(); fn rcu_qs(); fn rcu_preempt_blocked_readers_cgp(rnp: *mut rcu_node) -> i32;
    fn rcu_print_task_exp_stall(rnp: *mut rcu_node) -> i32;
    fn rcu_preempt_check_blocked_tasks(rnp: *mut rcu_node); fn rcu_flavor_sched_clock_irq(user: i32);
    fn dump_blkd_tasks(rnp: *mut rcu_node, ncheck: i32); fn zero_cpu_stall_ticks(rdp: *mut rcu_data);
    fn rcu_bind_gp_kthread(); fn rcu_nohz_full_cpu() -> bool; fn record_gp_stall_check_time();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
