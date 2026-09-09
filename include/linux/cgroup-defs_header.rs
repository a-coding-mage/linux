/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of linux/cgroup-defs.h. External kernel types/functions remain external. */

#[cfg(feature = "CONFIG_CGROUPS")]
pub const MAX_CGROUP_TYPE_NAMELEN: usize = 32;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const MAX_CGROUP_ROOT_NAMELEN: usize = 64;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const MAX_CFTYPE_NAME: usize = 64;

#[cfg(feature = "CONFIG_CGROUPS")]
pub const CSS_NO_REF: u32 = 1 << 0;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CSS_ONLINE: u32 = 1 << 1;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CSS_RELEASED: u32 = 1 << 2;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CSS_VISIBLE: u32 = 1 << 3;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CSS_DYING: u32 = 1 << 4;

#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_NOTIFY_ON_RELEASE: u32 = 0;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_CPUSET_CLONE_CHILDREN: u32 = 1;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_FREEZE: u32 = 2;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_FROZEN: u32 = 3;

#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_ROOT_NOPREFIX: u32 = 1 << 1;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_ROOT_XATTR: u32 = 1 << 2;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_ROOT_NS_DELEGATE: u32 = 1 << 3;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_ROOT_FAVOR_DYNMODS: u32 = 1 << 4;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_ROOT_CPUSET_V2_MODE: u32 = 1 << 16;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_ROOT_MEMORY_LOCAL_EVENTS: u32 = 1 << 17;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_ROOT_MEMORY_RECURSIVE_PROT: u32 = 1 << 18;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_ROOT_MEMORY_HUGETLB_ACCOUNTING: u32 = 1 << 19;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_ROOT_PIDS_LOCAL_EVENTS: u32 = 1 << 20;

#[cfg(feature = "CONFIG_CGROUPS")]
pub const CFTYPE_ONLY_ON_ROOT: u32 = 1 << 0;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CFTYPE_NOT_ON_ROOT: u32 = 1 << 1;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CFTYPE_NS_DELEGATABLE: u32 = 1 << 2;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CFTYPE_NO_PREFIX: u32 = 1 << 3;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CFTYPE_WORLD_WRITABLE: u32 = 1 << 4;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CFTYPE_DEBUG: u32 = 1 << 5;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const __CFTYPE_ONLY_ON_DFL: u32 = 1 << 16;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const __CFTYPE_NOT_ON_DFL: u32 = 1 << 17;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const __CFTYPE_ADDED: u32 = 1 << 18;

#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_ATTACH_LOCK_GLOBAL: u32 = 0;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_ATTACH_LOCK_NONE: u32 = 1;
#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGRP_ATTACH_LOCK_PER_THREADGROUP: u32 = 2;

#[cfg(feature = "CONFIG_CGROUPS")]
#[repr(C)]
pub struct cgroup_file { pub kn: *mut kernfs_node, pub notified_at: ::core::ffi::c_ulong, pub notify_timer: timer_list, pub lock: spinlock_t }

#[cfg(feature = "CONFIG_CGROUPS")]
#[repr(C)]
pub struct cgroup_subsys_state {
    pub cgroup: *mut cgroup, pub ss: *mut cgroup_subsys, pub refcnt: percpu_ref,
    pub rstat_cpu: *mut css_rstat_cpu, pub sibling: list_head, pub children: list_head,
    pub id: ::core::ffi::c_int, pub flags: u32, pub serial_nr: u64, pub online_cnt: atomic_t,
    pub destroy_work: work_struct, pub destroy_rwork: rcu_work, pub parent: *mut cgroup_subsys_state,
    pub nr_descendants: ::core::ffi::c_int, pub nr_populated_csets: ::core::ffi::c_int,
    pub nr_populated_children: ::core::ffi::c_int, pub kill_finish_work: work_struct,
    pub rstat_flush_next: *mut cgroup_subsys_state,
}

#[cfg(feature = "CONFIG_CGROUPS")]
#[repr(C)]
pub struct css_set {
    pub subsys: [*mut cgroup_subsys_state; CGROUP_SUBSYS_COUNT], pub refcount: refcount_t,
    pub dom_cset: *mut css_set, pub dfl_cgrp: *mut cgroup, pub nr_tasks: ::core::ffi::c_int,
    pub tasks: list_head, pub mg_tasks: list_head, pub dying_tasks: list_head, pub task_iters: list_head,
    pub e_cset_node: [list_head; CGROUP_SUBSYS_COUNT], pub threaded_csets: list_head,
    pub threaded_csets_node: list_head, pub hlist: hlist_node, pub cgrp_links: list_head,
    pub mg_src_preload_node: list_head, pub mg_dst_preload_node: list_head, pub mg_node: list_head,
    pub mg_src_cgrp: *mut cgroup, pub mg_dst_cgrp: *mut cgroup, pub mg_dst_cset: *mut css_set,
    pub dead: bool, pub rcu_head: rcu_head,
}

#[cfg(feature = "CONFIG_CGROUPS")]
#[repr(C)]
pub struct cgroup_base_stat { pub cputime: task_cputime, #[cfg(feature = "CONFIG_SCHED_CORE")] pub forceidle_sum: u64, pub ntime: u64 }

#[cfg(feature = "CONFIG_CGROUPS")]
#[repr(C)]
pub struct css_rstat_cpu { pub updated_children: *mut cgroup_subsys_state, pub updated_next: *mut cgroup_subsys_state, pub lnode: llist_node, pub owner: *mut cgroup_subsys_state }

#[cfg(feature = "CONFIG_CGROUPS")]
#[repr(C)]
pub struct cgroup_rstat_base_cpu {
    pub bsync: u64_stats_sync, pub bstat: cgroup_base_stat, pub last_bstat: cgroup_base_stat,
    pub subtree_bstat: cgroup_base_stat, pub last_subtree_bstat: cgroup_base_stat,
}

#[cfg(feature = "CONFIG_CGROUPS")]
#[repr(C)]
pub struct cgroup_freezer_state {
    pub freeze: bool, pub e_freeze: bool, pub nr_frozen_descendants: ::core::ffi::c_int,
    pub nr_frozen_tasks: ::core::ffi::c_int, pub freeze_seq: seqcount_spinlock_t,
    pub freeze_start_nsec: u64, pub frozen_nsec: u64,
}

#[cfg(feature = "CONFIG_CGROUPS")]
#[repr(C)]
pub struct cgroup {
    pub self_: cgroup_subsys_state, pub flags: ::core::ffi::c_ulong, pub level: ::core::ffi::c_int,
    pub max_depth: ::core::ffi::c_int, pub nr_descendants: ::core::ffi::c_int,
    pub nr_dying_descendants: ::core::ffi::c_int, pub max_descendants: ::core::ffi::c_int,
    pub nr_populated_domain_children: ::core::ffi::c_int, pub nr_populated_threaded_children: ::core::ffi::c_int,
    pub nr_threaded_children: ::core::ffi::c_int, pub kill_seq: u32, pub kn: *mut kernfs_node,
    pub procs_file: cgroup_file, pub events_file: cgroup_file, pub psi_files: [cgroup_file; NR_PSI_RESOURCES],
    pub subtree_control: u32, pub subtree_ss_mask: u32, pub old_subtree_control: u32, pub old_subtree_ss_mask: u32,
    pub subsys: [*mut cgroup_subsys_state; CGROUP_SUBSYS_COUNT], pub nr_dying_subsys: [::core::ffi::c_int; CGROUP_SUBSYS_COUNT],
    pub root: *mut cgroup_root, pub cset_links: list_head, pub e_csets: [list_head; CGROUP_SUBSYS_COUNT],
    pub dom_cgrp: *mut cgroup, pub old_dom_cgrp: *mut cgroup, pub rstat_base_cpu: *mut cgroup_rstat_base_cpu,
    pub last_bstat: cgroup_base_stat, pub bstat: cgroup_base_stat, pub prev_cputime: prev_cputime,
    pub pidlists: list_head, pub pidlist_mutex: mutex, pub offline_waitq: wait_queue_head_t,
    pub release_agent_work: work_struct, pub psi: *mut psi_group, pub bpf: cgroup_bpf, pub freezer: cgroup_freezer_state,
    #[cfg(feature = "CONFIG_BPF_SYSCALL")] pub bpf_cgrp_storage: *mut bpf_local_storage,
    #[cfg(feature = "CONFIG_EXT_SUB_SCHED")] pub scx_sched: *mut scx_sched,
    pub ancestors: *mut *mut cgroup,
}

#[cfg(feature = "CONFIG_CGROUPS")]
#[repr(C)]
pub struct cgroup_root { pub kf_root: *mut kernfs_root, pub subsys_mask: u32, pub hierarchy_id: ::core::ffi::c_int, pub root_list: list_head, pub rcu: rcu_head, pub nr_cgrps: atomic_t, pub flags: u32, pub release_agent_path: [::core::ffi::c_char; PATH_MAX], pub name: [::core::ffi::c_char; MAX_CGROUP_ROOT_NAMELEN], pub cgrp: cgroup }

#[cfg(feature = "CONFIG_CGROUPS")]
#[repr(C)]
pub struct cftype {
    pub name: [::core::ffi::c_char; MAX_CFTYPE_NAME], pub private: ::core::ffi::c_ulong, pub max_write_len: usize, pub flags: u32, pub file_offset: u32,
    pub ss: *mut cgroup_subsys, pub node: list_head, pub kf_ops: *mut kernfs_ops,
    pub open: Option<unsafe extern "C" fn(*mut kernfs_open_file) -> ::core::ffi::c_int>, pub release: Option<unsafe extern "C" fn(*mut kernfs_open_file)>,
    pub read_u64: Option<unsafe extern "C" fn(*mut cgroup_subsys_state, *mut cftype) -> u64>, pub read_s64: Option<unsafe extern "C" fn(*mut cgroup_subsys_state, *mut cftype) -> i64>,
    pub seq_show: Option<unsafe extern "C" fn(*mut seq_file, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub seq_start: Option<unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut ::core::ffi::c_void>, pub seq_next: Option<unsafe extern "C" fn(*mut seq_file, *mut ::core::ffi::c_void, *mut loff_t) -> *mut ::core::ffi::c_void>, pub seq_stop: Option<unsafe extern "C" fn(*mut seq_file, *mut ::core::ffi::c_void)>,
    pub write_u64: Option<unsafe extern "C" fn(*mut cgroup_subsys_state, *mut cftype, u64) -> ::core::ffi::c_int>, pub write_s64: Option<unsafe extern "C" fn(*mut cgroup_subsys_state, *mut cftype, i64) -> ::core::ffi::c_int>,
    pub write: Option<unsafe extern "C" fn(*mut kernfs_open_file, *mut ::core::ffi::c_char, usize, loff_t) -> isize>, pub poll: Option<unsafe extern "C" fn(*mut kernfs_open_file, *mut poll_table_struct) -> __poll_t>, pub lockdep_key: lock_class_key,
}

#[cfg(feature = "CONFIG_CGROUPS")]
#[repr(C)]
pub struct cgroup_subsys {
    pub css_alloc: Option<unsafe extern "C" fn(*mut cgroup_subsys_state) -> *mut cgroup_subsys_state>, pub css_online: Option<unsafe extern "C" fn(*mut cgroup_subsys_state) -> ::core::ffi::c_int>, pub css_offline: Option<unsafe extern "C" fn(*mut cgroup_subsys_state)>, pub css_released: Option<unsafe extern "C" fn(*mut cgroup_subsys_state)>, pub css_free: Option<unsafe extern "C" fn(*mut cgroup_subsys_state)>, pub css_reset: Option<unsafe extern "C" fn(*mut cgroup_subsys_state)>, pub css_killed: Option<unsafe extern "C" fn(*mut cgroup_subsys_state)>, pub css_rstat_flush: Option<unsafe extern "C" fn(*mut cgroup_subsys_state, ::core::ffi::c_int)>,
    pub css_extra_stat_show: Option<unsafe extern "C" fn(*mut seq_file, *mut cgroup_subsys_state) -> ::core::ffi::c_int>, pub css_local_stat_show: Option<unsafe extern "C" fn(*mut seq_file, *mut cgroup_subsys_state) -> ::core::ffi::c_int>,
    pub can_attach: Option<unsafe extern "C" fn(*mut cgroup_taskset) -> ::core::ffi::c_int>, pub cancel_attach: Option<unsafe extern "C" fn(*mut cgroup_taskset)>, pub attach: Option<unsafe extern "C" fn(*mut cgroup_taskset)>, pub can_fork: Option<unsafe extern "C" fn(*mut task_struct, *mut css_set) -> ::core::ffi::c_int>, pub cancel_fork: Option<unsafe extern "C" fn(*mut task_struct, *mut css_set)>, pub fork: Option<unsafe extern "C" fn(*mut task_struct)>, pub exit: Option<unsafe extern "C" fn(*mut task_struct)>, pub release: Option<unsafe extern "C" fn(*mut task_struct)>, pub bind: Option<unsafe extern "C" fn(*mut cgroup_subsys_state)>,
    pub early_init: bool, pub implicit_on_dfl: bool, pub threaded: bool, pub id: ::core::ffi::c_int, pub name: *const ::core::ffi::c_char, pub legacy_name: *const ::core::ffi::c_char, pub root: *mut cgroup_root, pub css_idr: idr, pub cfts: list_head, pub dfl_cftypes: *mut cftype, pub legacy_cftypes: *mut cftype, pub depends_on: u32, pub rstat_ss_lock: spinlock_t, pub lhead: *mut llist_head,
}

#[cfg(feature = "CONFIG_CGROUPS")]
extern "C" { pub static mut cgroup_threadgroup_rwsem: percpu_rw_semaphore; pub static mut cgroup_enable_per_threadgroup_rwsem: bool; }

#[cfg(feature = "CONFIG_CGROUPS")]
#[repr(C)]
pub struct cgroup_of_peak { pub value: ::core::ffi::c_ulong, pub list: list_head }

#[cfg(feature = "CONFIG_CGROUPS")]
pub unsafe fn cgroup_threadgroup_change_begin(tsk: *mut task_struct) {
    percpu_down_read(&raw mut cgroup_threadgroup_rwsem);
    if cgroup_enable_per_threadgroup_rwsem { down_read((*(*tsk).signal).cgroup_threadgroup_rwsem); }
}
#[cfg(feature = "CONFIG_CGROUPS")]
pub unsafe fn cgroup_threadgroup_change_end(tsk: *mut task_struct) {
    if cgroup_enable_per_threadgroup_rwsem { up_read((*(*tsk).signal).cgroup_threadgroup_rwsem); }
    percpu_up_read(&raw mut cgroup_threadgroup_rwsem);
}

#[cfg(not(feature = "CONFIG_CGROUPS"))]
pub const CGROUP_SUBSYS_COUNT: usize = 0;

#[cfg(feature = "CONFIG_SOCK_CGROUP_DATA")]
#[repr(C)]
pub struct sock_cgroup_data {
    pub cgroup: *mut cgroup,
    #[cfg(feature = "CONFIG_CGROUP_NET_CLASSID")] pub classid: u32,
    #[cfg(feature = "CONFIG_CGROUP_NET_PRIO")] pub prioidx: u16,
}
#[cfg(not(feature = "CONFIG_SOCK_CGROUP_DATA"))]
#[repr(C)]
pub struct sock_cgroup_data {}

#[cfg(feature = "CONFIG_SOCK_CGROUP_DATA")]
pub unsafe fn sock_cgroup_prioidx(skcd: *const sock_cgroup_data) -> u16 {
    #[cfg(feature = "CONFIG_CGROUP_NET_PRIO")]
    { return core::ptr::read_volatile(&(*skcd).prioidx); }
    #[cfg(not(feature = "CONFIG_CGROUP_NET_PRIO"))]
    { let _ = skcd; 1 }
}
#[cfg(all(feature = "CONFIG_SOCK_CGROUP_DATA", feature = "CONFIG_CGROUP_NET_CLASSID"))]
pub unsafe fn sock_cgroup_classid(skcd: *const sock_cgroup_data) -> u32 {
    core::ptr::read_volatile(&(*skcd).classid)
}
#[cfg(all(feature = "CONFIG_SOCK_CGROUP_DATA", feature = "CONFIG_CGROUP_NET_PRIO"))]
pub unsafe fn sock_cgroup_set_prioidx(skcd: *mut sock_cgroup_data, prioidx: u16) {
    core::ptr::write_volatile(&mut (*skcd).prioidx, prioidx);
}
#[cfg(all(feature = "CONFIG_SOCK_CGROUP_DATA", feature = "CONFIG_CGROUP_NET_CLASSID"))]
pub unsafe fn sock_cgroup_set_classid(skcd: *mut sock_cgroup_data, classid: u32) {
    core::ptr::write_volatile(&mut (*skcd).classid, classid);
}

// Forward declarations for types supplied by the kernel headers included by the original file.
// The generated translation intentionally does not implement those dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
