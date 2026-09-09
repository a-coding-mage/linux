/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Translated from cpuset-internal.h. */

/* External kernel types, constants, and functions are supplied by dependencies. */

#[repr(C)]
pub struct fmeter {
    pub cnt: ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
    pub time: time64_t,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prs_errcode {
    PERR_NONE = 0,
    PERR_INVCPUS,
    PERR_INVPARENT,
    PERR_NOTPART,
    PERR_NOTEXCL,
    PERR_NOCPUS,
    PERR_HOTPLUG,
    PERR_CPUSEMPTY,
    PERR_HKEEPING,
    PERR_ACCESS,
    PERR_REMOTE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpuset_flagbits_t {
    CS_CPU_EXCLUSIVE,
    CS_MEM_EXCLUSIVE,
    CS_MEM_HARDWALL,
    CS_MEMORY_MIGRATE,
    CS_SCHED_LOAD_BALANCE,
    CS_SPREAD_PAGE,
    CS_SPREAD_SLAB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpuset_filetype_t {
    FILE_MEMORY_MIGRATE,
    FILE_CPULIST,
    FILE_MEMLIST,
    FILE_EFFECTIVE_CPULIST,
    FILE_EFFECTIVE_MEMLIST,
    FILE_SUBPARTS_CPULIST,
    FILE_EXCLUSIVE_CPULIST,
    FILE_EFFECTIVE_XCPULIST,
    FILE_ISOLATED_CPULIST,
    FILE_CPU_EXCLUSIVE,
    FILE_MEM_EXCLUSIVE,
    FILE_MEM_HARDWALL,
    FILE_SCHED_LOAD_BALANCE,
    FILE_PARTITION_ROOT,
    FILE_SCHED_RELAX_DOMAIN_LEVEL,
    FILE_MEMORY_PRESSURE_ENABLED,
    FILE_MEMORY_PRESSURE,
    FILE_SPREAD_PAGE,
    FILE_SPREAD_SLAB,
}

#[repr(C)]
pub struct cpuset {
    pub css: cgroup_subsys_state,
    pub flags: ::core::ffi::c_ulong,
    pub cpus_allowed: cpumask_var_t,
    pub mems_allowed: nodemask_t,
    pub effective_cpus: cpumask_var_t,
    pub effective_mems: nodemask_t,
    pub effective_xcpus: cpumask_var_t,
    pub exclusive_cpus: cpumask_var_t,
    pub old_mems_allowed: nodemask_t,
    pub attach_node: llist_node,
    pub partition_root_state: ::core::ffi::c_int,
    pub remote_partition: bool,
    pub nr_deadline_tasks: atomic_t,
    pub nr_migrate_dl_tasks: ::core::ffi::c_int,
    pub sum_migrate_dl_bw: u64,
    pub dl_bw_cpu: ::core::ffi::c_int,
    pub prs_err: prs_errcode,
    pub partition_file: cgroup_file,
    #[cfg(CONFIG_CPUSETS_V1)]
    pub fmeter: fmeter,
    #[cfg(CONFIG_CPUSETS_V1)]
    pub relax_domain_level: ::core::ffi::c_int,
    #[cfg(CONFIG_CPUSETS_V1)]
    pub node: uf_node,
}

extern "C" {
    pub static mut top_cpuset: cpuset;

    pub fn rebuild_sched_domains_locked();
    pub fn cpuset_callback_lock_irq();
    pub fn cpuset_callback_unlock_irq();
    pub fn cpuset_update_tasks_cpumask(cs: *mut cpuset, new_cpus: *mut cpumask);
    pub fn cpuset_update_tasks_nodemask(cs: *mut cpuset);
    pub fn cpuset_update_flag(bit: cpuset_flagbits_t, cs: *mut cpuset, turning_on: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn cpuset_write_resmask(of: *mut kernfs_open_file, buf: *mut ::core::ffi::c_char, nbytes: usize, off: loff_t) -> ssize_t;
    pub fn cpuset_common_seq_show(sf: *mut seq_file, v: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn cpuset_full_lock();
    pub fn cpuset_full_unlock();
}

#[inline]
pub unsafe fn css_cs(css: *mut cgroup_subsys_state) -> *mut cpuset {
    if !css.is_null() { container_of!(css, cpuset, css) } else { ::core::ptr::null_mut() }
}

#[inline]
pub unsafe fn task_cs(task: *mut task_struct) -> *mut cpuset {
    css_cs(task_css(task, cpuset_cgrp_id))
}

#[inline]
pub unsafe fn parent_cs(cs: *mut cpuset) -> *mut cpuset {
    css_cs((*cs).css.parent)
}

#[inline]
pub unsafe fn is_cpuset_online(cs: *mut cpuset) -> bool {
    css_is_online(&mut (*cs).css) && !css_is_dying(&mut (*cs).css)
}

#[inline]
pub unsafe fn is_cpu_exclusive(cs: *const cpuset) -> ::core::ffi::c_int { test_bit(CS_CPU_EXCLUSIVE, &(*cs).flags) }
#[inline]
pub unsafe fn is_mem_exclusive(cs: *const cpuset) -> ::core::ffi::c_int { test_bit(CS_MEM_EXCLUSIVE, &(*cs).flags) }
#[inline]
pub unsafe fn is_mem_hardwall(cs: *const cpuset) -> ::core::ffi::c_int { test_bit(CS_MEM_HARDWALL, &(*cs).flags) }
#[inline]
pub unsafe fn is_sched_load_balance(cs: *const cpuset) -> ::core::ffi::c_int { test_bit(CS_SCHED_LOAD_BALANCE, &(*cs).flags) }
#[inline]
pub unsafe fn is_memory_migrate(cs: *const cpuset) -> ::core::ffi::c_int { test_bit(CS_MEMORY_MIGRATE, &(*cs).flags) }
#[inline]
pub unsafe fn is_spread_page(cs: *const cpuset) -> ::core::ffi::c_int { test_bit(CS_SPREAD_PAGE, &(*cs).flags) }
#[inline]
pub unsafe fn is_spread_slab(cs: *const cpuset) -> ::core::ffi::c_int { test_bit(CS_SPREAD_SLAB, &(*cs).flags) }

#[inline]
pub unsafe fn cpusets_overlap(a: *mut cpuset, b: *mut cpuset) -> ::core::ffi::c_int {
    cpumask_intersects((*a).effective_cpus, (*b).effective_cpus)
}

#[inline]
pub unsafe fn nr_cpusets() -> ::core::ffi::c_int {
    static_key_count(&cpusets_enabled_key.key) + 1
}

#[inline]
pub unsafe fn cpuset_is_populated(cs: *mut cpuset) -> bool {
    lockdep_assert_cpuset_lock_held();
    cgroup_is_populated((*cs).css.cgroup)
}

/* cpuset_for_each_child and cpuset_for_each_descendant_pre retain their C iteration semantics. */

#[cfg(CONFIG_CPUSETS_V1)]
extern "C" {
    pub static mut cpuset1_files: cftype;
    pub fn cpuset1_update_task_spread_flags(cs: *mut cpuset, tsk: *mut task_struct);
    pub fn cpuset1_update_tasks_flags(cs: *mut cpuset);
    pub fn cpuset1_hotplug_update_tasks(cs: *mut cpuset, new_cpus: *mut cpumask, new_mems: *mut nodemask_t, cpus_updated: bool, mems_updated: bool);
    pub fn cpuset1_validate_change(cur: *mut cpuset, trial: *mut cpuset) -> ::core::ffi::c_int;
    pub fn cpuset1_cpus_excl_conflict(cs1: *mut cpuset, cs2: *mut cpuset) -> bool;
    pub fn cpuset1_init(cs: *mut cpuset);
    pub fn cpuset1_online_css(css: *mut cgroup_subsys_state);
    pub fn cpuset1_generate_sched_domains(domains: *mut *mut cpumask_var_t, attributes: *mut *mut sched_domain_attr) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_CPUSETS_V1))]
#[inline] pub unsafe fn cpuset1_update_task_spread_flags(_: *mut cpuset, _: *mut task_struct) {}
#[cfg(not(CONFIG_CPUSETS_V1))]
#[inline] pub unsafe fn cpuset1_update_tasks_flags(_: *mut cpuset) {}
#[cfg(not(CONFIG_CPUSETS_V1))]
#[inline] pub unsafe fn cpuset1_hotplug_update_tasks(_: *mut cpuset, _: *mut cpumask, _: *mut nodemask_t, _: bool, _: bool) {}
#[cfg(not(CONFIG_CPUSETS_V1))]
#[inline] pub unsafe fn cpuset1_validate_change(_: *mut cpuset, _: *mut cpuset) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_CPUSETS_V1))]
#[inline] pub unsafe fn cpuset1_cpus_excl_conflict(_: *mut cpuset, _: *mut cpuset) -> bool { false }
#[cfg(not(CONFIG_CPUSETS_V1))]
#[inline] pub unsafe fn cpuset1_init(_: *mut cpuset) {}
#[cfg(not(CONFIG_CPUSETS_V1))]
#[inline] pub unsafe fn cpuset1_online_css(_: *mut cgroup_subsys_state) {}
#[cfg(not(CONFIG_CPUSETS_V1))]
#[inline] pub unsafe fn cpuset1_generate_sched_domains(_: *mut *mut cpumask_var_t, _: *mut *mut sched_domain_attr) -> ::core::ffi::c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
