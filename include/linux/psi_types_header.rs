/* SPDX-License-Identifier: GPL-2.0 */

/* The declarations below correspond to CONFIG_PSI. */
#[cfg(feature = "CONFIG_PSI")]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psi_task_count {
    NR_IOWAIT,
    NR_MEMSTALL,
    NR_RUNNING,
    /*
     * For IO and CPU stalls the presence of running/oncpu tasks
     * in the domain means a partial rather than a full stall.
     * For memory it's not so simple because of page reclaimers:
     * they are running/oncpu while representing a stall. To tell
     * whether a domain has productivity left or not, we need to
     * distinguish between regular running (i.e. productive)
     * threads and memstall ones.
     */
    NR_MEMSTALL_RUNNING,
    NR_PSI_TASK_COUNTS = 4,
}

#[cfg(feature = "CONFIG_PSI")]
pub const TSK_IOWAIT: u32 = 1 << (NR_IOWAIT as u32);
#[cfg(feature = "CONFIG_PSI")]
pub const TSK_MEMSTALL: u32 = 1 << (NR_MEMSTALL as u32);
#[cfg(feature = "CONFIG_PSI")]
pub const TSK_RUNNING: u32 = 1 << (NR_RUNNING as u32);
#[cfg(feature = "CONFIG_PSI")]
pub const TSK_MEMSTALL_RUNNING: u32 = 1 << (NR_MEMSTALL_RUNNING as u32);
#[cfg(feature = "CONFIG_PSI")]
pub const TSK_ONCPU: u32 = 1 << (NR_PSI_TASK_COUNTS as u32);

#[cfg(feature = "CONFIG_PSI")]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psi_res {
    PSI_IO,
    PSI_MEM,
    PSI_CPU,
    #[cfg(feature = "CONFIG_IRQ_TIME_ACCOUNTING")]
    PSI_IRQ,
    NR_PSI_RESOURCES,
}

#[cfg(feature = "CONFIG_PSI")]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psi_states {
    PSI_IO_SOME,
    PSI_IO_FULL,
    PSI_MEM_SOME,
    PSI_MEM_FULL,
    PSI_CPU_SOME,
    PSI_CPU_FULL,
    #[cfg(feature = "CONFIG_IRQ_TIME_ACCOUNTING")]
    PSI_IRQ_FULL,
    /* Only per-CPU, to weigh the CPU in the global average: */
    PSI_NONIDLE,
    NR_PSI_STATES,
}

#[cfg(feature = "CONFIG_PSI")]
pub const PSI_ONCPU: u32 = 1 << (NR_PSI_STATES as u32);
#[cfg(feature = "CONFIG_PSI")]
pub const PSI_STATE_RESCHEDULE: u32 = 1 << ((NR_PSI_STATES as u32) + 1);

#[cfg(feature = "CONFIG_PSI")]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psi_aggregators {
    PSI_AVGS = 0,
    PSI_POLL,
    NR_PSI_AGGREGATORS,
}

#[cfg(feature = "CONFIG_PSI")]
#[repr(C)]
pub struct psi_group_cpu {
    /* 1st cacheline updated by the scheduler */
    /* States of the tasks belonging to this group */
    pub tasks: [core::ffi::c_uint; 4],
    /* Aggregate pressure state derived from the tasks */
    pub state_mask: u32,
    /* Period time sampling buckets for each state of interest (ns) */
    pub times: [u32; NR_PSI_STATES as usize],
    /* Time of last task change in this group (rq_clock) */
    pub state_start: u64,
    /* 2nd cacheline updated by the aggregator */
    /* Delta detection against the sampling buckets */
    pub times_prev: [[u32; NR_PSI_STATES as usize]; NR_PSI_AGGREGATORS as usize],
}

#[cfg(feature = "CONFIG_PSI")]
#[repr(C)]
pub struct psi_window {
    /* Window size in ns */
    pub size: u64,
    /* Start time of the current window in ns */
    pub start_time: u64,
    /* Value at the start of the window */
    pub start_value: u64,
    /* Value growth in the previous window */
    pub prev_growth: u64,
}

/* External kernel types are supplied by other translated headers. */
#[cfg(feature = "CONFIG_PSI")]
#[repr(C)]
pub struct psi_trigger {
    pub state: psi_states,
    pub threshold: u64,
    pub node: list_head,
    pub group: *mut psi_group,
    pub event_wait: wait_queue_head_t,
    pub of: *mut kernfs_open_file,
    pub event: core::ffi::c_int,
    pub win: psi_window,
    pub last_event_time: u64,
    pub pending_event: bool,
    pub aggregator: psi_aggregators,
}

#[cfg(feature = "CONFIG_PSI")]
#[repr(C)]
pub struct psi_group {
    pub parent: *mut psi_group,
    pub enabled: bool,
    pub avgs_lock: mutex,
    pub pcpu: *mut psi_group_cpu,
    pub avg_total: [u64; (NR_PSI_STATES - 1) as usize],
    pub avg_last_update: u64,
    pub avg_next_update: u64,
    pub avgs_work: delayed_work,
    pub avg_triggers: list_head,
    pub avg_nr_triggers: [u32; (NR_PSI_STATES - 1) as usize],
    pub total: [[u64; (NR_PSI_STATES - 1) as usize]; NR_PSI_AGGREGATORS as usize],
    pub avg: [[c_ulong; 3]; (NR_PSI_STATES - 1) as usize],
    pub rtpoll_task: *mut task_struct,
    pub rtpoll_timer: timer_list,
    pub rtpoll_wait: wait_queue_head_t,
    pub rtpoll_wakeup: atomic_t,
    pub rtpoll_scheduled: atomic_t,
    pub rtpoll_trigger_lock: mutex,
    pub rtpoll_triggers: list_head,
    pub rtpoll_nr_triggers: [u32; (NR_PSI_STATES - 1) as usize],
    pub rtpoll_states: u32,
    pub rtpoll_min_period: u64,
    pub rtpoll_total: [u64; (NR_PSI_STATES - 1) as usize],
    pub rtpoll_next_update: u64,
    pub rtpoll_until: u64,
}

#[cfg(not(feature = "CONFIG_PSI"))]
pub const NR_PSI_RESOURCES: usize = 0;

#[cfg(not(feature = "CONFIG_PSI"))]
#[repr(C)]
pub struct psi_group {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
