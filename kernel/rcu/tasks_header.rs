/* SPDX-License-Identifier: GPL-2.0+ */
/* Task-based RCU implementations. Rust translation of tasks.h. */

/* Names below are supplied by the kernel translation unit. */
#[cfg(CONFIG_TASKS_RCU_GENERIC)]
use core::ffi::c_void;

#[cfg(CONFIG_TASKS_RCU_GENERIC)]
#[repr(C)]
pub struct rcu_tasks_percpu {
    pub cblist: rcu_segcblist,
    pub lock: raw_spinlock_t,
    pub rtp_jiffies: c_ulong,
    pub rtp_n_lock_retries: c_ulong,
    pub lazy_timer: timer_list,
    pub urgent_gp: c_uint,
    pub rtp_work: work_struct,
    pub rtp_irq_work: irq_work,
    pub barrier_q_head: rcu_head,
    pub rtp_blkd_tasks: list_head,
    pub rtp_exit_list: list_head,
    pub cpu: c_int,
    pub index: c_int,
    pub rtpp: *mut rcu_tasks,
}

#[cfg(CONFIG_TASKS_RCU_GENERIC)]
#[repr(C)]
pub struct rcu_tasks {
    pub cbs_wait: rcuwait,
    pub cbs_gbl_lock: raw_spinlock_t,
    pub tasks_gp_mutex: mutex,
    pub gp_state: c_int,
    pub gp_sleep: c_int,
    pub init_fract: c_int,
    pub gp_jiffies: c_ulong,
    pub gp_start: c_ulong,
    pub tasks_gp_seq: c_ulong,
    pub n_ipis: c_ulong,
    pub kthread_ptr: *mut task_struct,
    pub lazy_jiffies: c_ulong,
    pub gp_func: Option<unsafe extern "C" fn(*mut rcu_tasks)>,
    pub pregp_func: Option<unsafe extern "C" fn(*mut list_head)>,
    pub pertask_func: Option<unsafe extern "C" fn(*mut task_struct, *mut list_head)>,
    pub postscan_func: Option<unsafe extern "C" fn(*mut list_head)>,
    pub holdouts_func: Option<unsafe extern "C" fn(*mut list_head, bool, *mut bool)>,
    pub postgp_func: Option<unsafe extern "C" fn(*mut rcu_tasks)>,
    pub call_func: Option<unsafe extern "C" fn(*mut rcu_head, Option<unsafe extern "C" fn(*mut rcu_head)>)>,
    pub wait_state: c_uint,
    pub rtpcpu: *mut rcu_tasks_percpu,
    pub rtpcp_array: *mut *mut rcu_tasks_percpu,
    pub percpu_enqueue_shift: c_int,
    pub percpu_enqueue_lim: c_int,
    pub percpu_dequeue_lim: c_int,
    pub percpu_dequeue_gpseq: c_ulong,
    pub barrier_q_mutex: mutex,
    pub barrier_q_count: atomic_t,
    pub barrier_q_completion: completion,
    pub barrier_q_seq: c_ulong,
    pub barrier_q_start: c_ulong,
    pub name: *mut c_char,
    pub kname: *mut c_char,
}

#[cfg(CONFIG_TASKS_RCU_GENERIC)]
pub const RTGS_INIT: c_int = 0;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] pub const RTGS_WAIT_WAIT_CBS: c_int = 1;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] pub const RTGS_WAIT_GP: c_int = 2;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] pub const RTGS_PRE_WAIT_GP: c_int = 3;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] pub const RTGS_SCAN_TASKLIST: c_int = 4;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] pub const RTGS_POST_SCAN_TASKLIST: c_int = 5;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] pub const RTGS_WAIT_SCAN_HOLDOUTS: c_int = 6;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] pub const RTGS_SCAN_HOLDOUTS: c_int = 7;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] pub const RTGS_POST_GP: c_int = 8;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] pub const RTGS_WAIT_READERS: c_int = 9;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] pub const RTGS_INVOKE_CBS: c_int = 10;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] pub const RTGS_WAIT_CBS: c_int = 11;

#[cfg(CONFIG_TASKS_RCU_GENERIC)]
static mut rcu_task_stall_timeout: c_int = RCU_TASK_STALL_TIMEOUT;
#[cfg(CONFIG_TASKS_RCU_GENERIC)]
pub const RCU_TASK_BOOT_STALL_TIMEOUT: c_int = HZ * 30;
#[cfg(CONFIG_TASKS_RCU_GENERIC)]
pub const RCU_TASK_STALL_TIMEOUT: c_int = HZ * 60 * 10;
#[cfg(CONFIG_TASKS_RCU_GENERIC)]
pub const RCU_TASK_STALL_INFO: c_int = HZ * 10;

#[cfg(CONFIG_TASKS_RCU_GENERIC)]
static mut rcu_task_stall_info: c_int = RCU_TASK_STALL_INFO;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] static mut rcu_task_stall_info_mult: c_int = 3;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] static mut rcu_task_enqueue_lim: c_int = -1;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] static mut rcu_task_cb_adjust: bool = false;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] static mut rcu_task_contend_lim: c_int = 100;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] static mut rcu_task_collapse_lim: c_int = 10;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] static mut rcu_task_lazy_lim: c_int = 32;
#[cfg(CONFIG_TASKS_RCU_GENERIC)] static mut rcu_task_cpu_ids: c_int = 0;

/* Initialize per-CPU callback lists for the specified Tasks-RCU flavor. */
#[cfg(CONFIG_TASKS_RCU_GENERIC)]
unsafe fn cblist_init_generic(rtp: *mut rcu_tasks) {
    if rcu_task_enqueue_lim < 0 { rcu_task_enqueue_lim = 1; rcu_task_cb_adjust = true; }
    else if rcu_task_enqueue_lim == 0 { rcu_task_enqueue_lim = 1; }
    (*rtp).percpu_enqueue_lim = rcu_task_enqueue_lim;
    (*rtp).percpu_dequeue_lim = rcu_task_enqueue_lim;
    (*rtp).percpu_enqueue_shift = ilog2(rcu_task_cpu_ids / rcu_task_enqueue_lim);
}

#[cfg(CONFIG_TASKS_RCU_GENERIC)]
unsafe fn set_tasks_gp_state(rtp: *mut rcu_tasks, newstate: c_int) {
    (*rtp).gp_state = newstate;
    (*rtp).gp_jiffies = jiffies;
}

#[cfg(CONFIG_TASKS_RCU_GENERIC)]
unsafe fn rcu_tasks_lazy_time(rtp: *mut rcu_tasks) -> c_ulong { jiffies + (*rtp).lazy_jiffies }

#[cfg(CONFIG_TASKS_RCU_GENERIC)]
unsafe fn rcu_tasks_invoke_cbs_wq(wp: *mut work_struct) {
    let rtpcp = container_of!(wp, rcu_tasks_percpu, rtp_work);
    rcu_tasks_invoke_cbs((*rtpcp).rtpp, rtpcp);
}

#[cfg(CONFIG_TASKS_RCU_GENERIC)]
unsafe fn rcu_tasks_invoke_cbs(rtp: *mut rcu_tasks, rtpcp: *mut rcu_tasks_percpu) {
    if rcu_segcblist_empty(&mut (*rtpcp).cblist) { return; }
    let mut rcl = RCU_CBLIST_INITIALIZER!();
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave_rcu_node(rtpcp, &mut flags);
    srcu_segcblist_advance(&mut (*rtpcp).cblist, rcu_seq_current(&(*rtp).tasks_gp_seq));
    rcu_segcblist_extract_done_cbs(&mut (*rtpcp).cblist, &mut rcl);
    raw_spin_unlock_irqrestore_rcu_node(rtpcp, flags);
    while let Some(rhp) = rcu_cblist_dequeue(&mut rcl) {
        local_bh_disable(); ((*rhp).func.unwrap())(rhp); local_bh_enable(); cond_resched();
    }
}

#[cfg(CONFIG_TASKS_RCU)]
pub unsafe fn call_rcu_tasks(rhp: *mut rcu_head, func: rcu_callback_t) {
    call_rcu_tasks_generic(rhp, func, &mut rcu_tasks);
}

#[cfg(CONFIG_TASKS_RCU)]
pub unsafe fn synchronize_rcu_tasks() { synchronize_rcu_tasks_generic(&mut rcu_tasks); }

#[cfg(CONFIG_TASKS_RCU)]
pub unsafe fn rcu_barrier_tasks() { rcu_barrier_tasks_generic(&mut rcu_tasks); }

#[cfg(CONFIG_TASKS_RUDE_RCU)]
pub unsafe fn synchronize_rcu_tasks_rude() {
    if !IS_ENABLED(CONFIG_ARCH_WANTS_NO_INSTR) || IS_ENABLED(CONFIG_FORCE_TASKS_RUDE_RCU) {
        synchronize_rcu_tasks_generic(&mut rcu_tasks_rude);
    }
}

#[cfg(CONFIG_TASKS_TRACE_RCU)]
pub unsafe fn rcu_tasks_trace_batches_completed() -> c_ulong {
    srcu_batches_completed(&mut rcu_tasks_trace_srcu_struct)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
