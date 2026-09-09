/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/backing-dev-defs.h. Included Linux types are external dependencies. */

pub enum page {}
pub enum device {}
pub enum dentry {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum wb_state {
    WB_registered,
    WB_writeback_running,
    WB_has_dirty_io,
    WB_start_all,
    WB_start_dontcache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum wb_stat_item {
    WB_RECLAIMABLE,
    WB_WRITEBACK,
    WB_DIRTIED,
    WB_WRITTEN,
    WB_DONTCACHE_DIRTY,
    NR_WB_STAT_ITEMS,
}

/* #define WB_STAT_BATCH (8*(1+ilog2(nr_cpu_ids))) */
pub const WB_STAT_BATCH: usize = 8 * (1 + ilog2(nr_cpu_ids));

#[repr(C)]
#[derive(Copy, Clone)]
pub enum wb_reason {
    WB_REASON_BACKGROUND,
    WB_REASON_VMSCAN,
    WB_REASON_SYNC,
    WB_REASON_PERIODIC,
    WB_REASON_FS_FREE_SPACE,
    WB_REASON_FORKER_THREAD,
    WB_REASON_FOREIGN_FLUSH,
    WB_REASON_DONTCACHE,
    WB_REASON_MAX,
}

#[repr(C)]
pub struct wb_completion {
    pub cnt: atomic_t,
    pub waitq: *mut wait_queue_head_t,
    pub progress_stamp: c_ulong,
    pub wait_start: c_ulong,
}

#[inline]
pub unsafe fn __WB_COMPLETION_INIT(waitq: *mut wait_queue_head_t) -> wb_completion {
    wb_completion { cnt: ATOMIC_INIT(1), waitq, progress_stamp: 0, wait_start: 0 }
}

#[repr(C)]
pub struct bdi_writeback {
    pub bdi: *mut backing_dev_info,
    pub state: c_ulong,
    pub last_old_flush: c_ulong,
    pub b_dirty: list_head,
    pub b_io: list_head,
    pub b_more_io: list_head,
    pub b_dirty_time: list_head,
    pub list_lock: spinlock_t,
    pub writeback_inodes: atomic_t,
    pub stat: [percpu_counter; NR_WB_STAT_ITEMS as usize],
    pub bw_time_stamp: c_ulong,
    pub dirtied_stamp: c_ulong,
    pub written_stamp: c_ulong,
    pub write_bandwidth: c_ulong,
    pub avg_write_bandwidth: c_ulong,
    pub dirty_ratelimit: c_ulong,
    pub balanced_dirty_ratelimit: c_ulong,
    pub completions: fprop_local_percpu,
    pub dirty_exceeded: c_int,
    pub start_all_reason: wb_reason,
    pub work_lock: spinlock_t,
    pub work_list: list_head,
    pub dwork: delayed_work,
    pub bw_dwork: delayed_work,
    pub bdi_node: list_head,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub refcnt: percpu_ref,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub memcg_completions: fprop_local_percpu,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub memcg_css: *mut cgroup_subsys_state,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub blkcg_css: *mut cgroup_subsys_state,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub memcg_node: list_head,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub blkcg_node: list_head,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub b_attached: list_head,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub offline_node: list_head,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub switch_work: work_struct,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub switch_wbs_ctxs: llist_head,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub release_work_or_rcu: wb_writeback_union,
}

#[repr(C)]
pub union wb_writeback_union {
    pub release_work: work_struct,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct backing_dev_info {
    pub id: u64,
    pub rb_node: rb_node,
    pub bdi_list: list_head,
    pub ra_pages: c_ulong,
    pub io_pages: c_ulong,
    pub refcnt: kref,
    pub capabilities: c_uint,
    pub min_ratio: c_uint,
    pub max_ratio: c_uint,
    pub max_prop_frac: c_uint,
    pub tot_write_bandwidth: atomic_long_t,
    pub last_bdp_sleep: c_ulong,
    pub wb: bdi_writeback,
    pub wb_list: list_head,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub cgwb_tree: radix_tree_root,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub cgwb_release_mutex: mutex,
    #[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
    pub wb_switch_rwsem: rw_semaphore,
    pub wb_waitq: wait_queue_head_t,
    pub dev: *mut device,
    pub dev_name: [c_char; 64],
    pub owner: *mut device,
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    pub debug_dir: *mut dentry,
}

#[repr(C)]
pub struct wb_lock_cookie {
    pub locked: bool,
    pub flags: c_ulong,
}

#[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
#[inline]
pub unsafe fn wb_tryget(wb: *mut bdi_writeback) -> bool {
    if wb != (*wb).bdi.wrapping_add(0).cast::<bdi_writeback>() { percpu_ref_tryget(&mut (*wb).refcnt) } else { true }
}

#[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
#[inline]
pub unsafe fn wb_get(wb: *mut bdi_writeback) { if wb != &mut (*(*wb).bdi).wb { percpu_ref_get(&mut (*wb).refcnt); } }

#[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
#[inline]
pub unsafe fn wb_put_many(wb: *mut bdi_writeback, nr: c_ulong) { if !(*wb).bdi.is_null() && wb != &mut (*(*wb).bdi).wb { percpu_ref_put_many(&mut (*wb).refcnt, nr); } }

#[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
#[inline]
pub unsafe fn wb_put(wb: *mut bdi_writeback) { wb_put_many(wb, 1); }

#[cfg(feature = "CONFIG_CGROUP_WRITEBACK")]
#[inline]
pub unsafe fn wb_dying(wb: *mut bdi_writeback) -> bool { percpu_ref_is_dying(&(*wb).refcnt) }

#[cfg(not(feature = "CONFIG_CGROUP_WRITEBACK"))]
#[inline] pub unsafe fn wb_tryget(_: *mut bdi_writeback) -> bool { true }
#[cfg(not(feature = "CONFIG_CGROUP_WRITEBACK"))]
#[inline] pub unsafe fn wb_get(_: *mut bdi_writeback) {}
#[cfg(not(feature = "CONFIG_CGROUP_WRITEBACK"))]
#[inline] pub unsafe fn wb_put(_: *mut bdi_writeback) {}
#[cfg(not(feature = "CONFIG_CGROUP_WRITEBACK"))]
#[inline] pub unsafe fn wb_put_many(_: *mut bdi_writeback, _: c_ulong) {}
#[cfg(not(feature = "CONFIG_CGROUP_WRITEBACK"))]
#[inline] pub unsafe fn wb_dying(_: *mut bdi_writeback) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
