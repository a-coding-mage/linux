/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/cpuset.h. C header dependencies are supplied externally. */

extern "C" {
    pub fn lockdep_is_cpuset_held() -> bool;
}

#[cfg(CONFIG_CPUSETS)]
extern "C" {
    pub static mut cpusets_pre_enable_key: static_key_false;
    pub static mut cpusets_enabled_key: static_key_false;
    pub static mut cpusets_insane_config_key: static_key_false;
}

#[cfg(CONFIG_CPUSETS)]
#[inline]
pub unsafe fn cpusets_enabled() -> bool {
    static_branch_unlikely(&raw const cpusets_enabled_key)
}

#[cfg(CONFIG_CPUSETS)]
#[inline]
pub unsafe fn cpuset_inc() {
    static_branch_inc_cpuslocked(&raw const cpusets_pre_enable_key);
    static_branch_inc_cpuslocked(&raw const cpusets_enabled_key);
}

#[cfg(CONFIG_CPUSETS)]
#[inline]
pub unsafe fn cpuset_dec() {
    static_branch_dec_cpuslocked(&raw const cpusets_enabled_key);
    static_branch_dec_cpuslocked(&raw const cpusets_pre_enable_key);
}

#[cfg(CONFIG_CPUSETS)]
#[inline]
pub unsafe fn cpusets_insane_config() -> bool {
    static_branch_unlikely(&raw const cpusets_insane_config_key)
}

#[cfg(CONFIG_CPUSETS)]
extern "C" {
    pub fn cpuset_init() -> i32;
    pub fn cpuset_init_smp();
    pub fn cpuset_force_rebuild();
    pub fn cpuset_update_active_cpus();
    pub fn inc_dl_tasks_cs(task: *mut task_struct);
    pub fn dec_dl_tasks_cs(task: *mut task_struct);
    pub fn cpuset_lock();
    pub fn cpuset_unlock();
    pub fn lockdep_assert_cpuset_lock_held();
    pub fn cpuset_cpus_allowed_locked(p: *mut task_struct, mask: *mut cpumask);
    pub fn cpuset_cpus_allowed(p: *mut task_struct, mask: *mut cpumask);
    pub fn cpuset_cpus_allowed_fallback(p: *mut task_struct) -> bool;
    pub fn cpuset_num_cpus(cgroup: *mut cgroup) -> i32;
    pub fn cpuset_mems_allowed(p: *mut task_struct) -> nodemask_t;
    pub fn cpuset_init_current_mems_allowed();
    pub fn cpuset_nodemask_valid_mems_allowed(nodemask: *const nodemask_t) -> i32;
    pub fn cpuset_current_node_allowed(node: i32, gfp_mask: gfp_t) -> bool;
    pub fn cpuset_mems_allowed_intersects(tsk1: *const task_struct, tsk2: *const task_struct) -> i32;
    pub fn cpuset_task_status_allowed(m: *mut seq_file, task: *mut task_struct);
    pub fn proc_cpuset_show(m: *mut seq_file, ns: *mut pid_namespace, pid: *mut pid, tsk: *mut task_struct) -> i32;
    pub fn cpuset_mem_spread_node() -> i32;
    pub fn current_cpuset_is_being_rebound() -> bool;
    pub fn dl_rebuild_rd_accounting();
    pub fn rebuild_sched_domains();
    pub fn cpuset_print_current_mems_allowed();
    pub fn cpuset_reset_sched_domains();
    pub fn cpuset_nodes_allowed(cgroup: *mut cgroup, mask: *mut nodemask_t);
}

#[cfg(CONFIG_CPUSETS)]
#[inline]
pub unsafe fn __cpuset_zone_allowed(z: *mut zone, gfp_mask: gfp_t) -> bool {
    cpuset_current_node_allowed(zone_to_nid(z), gfp_mask)
}

#[cfg(CONFIG_CPUSETS)]
#[inline]
pub unsafe fn cpuset_zone_allowed(z: *mut zone, gfp_mask: gfp_t) -> bool {
    if cpusets_enabled() { __cpuset_zone_allowed(z, gfp_mask) } else { true }
}

#[cfg(CONFIG_CPUSETS)]
#[inline]
pub unsafe fn cpuset_do_page_mem_spread() -> i32 { task_spread_page(current) }

#[cfg(CONFIG_CPUSETS)]
#[inline]
pub unsafe fn read_mems_allowed_begin() -> u32 {
    if !static_branch_unlikely(&raw const cpusets_pre_enable_key) { return 0; }
    read_seqcount_begin(&(*current).mems_allowed_seq)
}

#[cfg(CONFIG_CPUSETS)]
#[inline]
pub unsafe fn read_mems_allowed_retry(seq: u32) -> bool {
    if !static_branch_unlikely(&raw const cpusets_enabled_key) { return false; }
    read_seqcount_retry(&(*current).mems_allowed_seq, seq)
}

#[cfg(CONFIG_CPUSETS)]
#[inline]
pub unsafe fn set_mems_allowed(nodemask: nodemask_t) {
    let mut flags: c_ulong = 0;
    task_lock(current);
    local_irq_save(&mut flags);
    write_seqcount_begin(&mut (*current).mems_allowed_seq);
    (*current).mems_allowed = nodemask;
    write_seqcount_end(&mut (*current).mems_allowed_seq);
    local_irq_restore(flags);
    task_unlock(current);
}

#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpusets_enabled() -> bool { false }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpusets_insane_config() -> bool { false }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_init() -> i32 { 0 }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_init_smp() {}
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_force_rebuild() {}
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_update_active_cpus() { partition_sched_domains(1, core::ptr::null_mut(), core::ptr::null_mut()); }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn inc_dl_tasks_cs(_: *mut task_struct) {}
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn dec_dl_tasks_cs(_: *mut task_struct) {}
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_lock() {}
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_unlock() {}
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn lockdep_assert_cpuset_lock_held() {}
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_cpus_allowed_locked(p: *mut task_struct, mask: *mut cpumask) { cpumask_copy(mask, task_cpu_possible_mask(p)); }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_cpus_allowed(p: *mut task_struct, mask: *mut cpumask) { cpuset_cpus_allowed_locked(p, mask); }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_cpus_allowed_fallback(_: *mut task_struct) -> bool { false }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_num_cpus(_: *mut cgroup) -> i32 { num_online_cpus() }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_mems_allowed(_: *mut task_struct) -> nodemask_t { node_possible_map }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_init_current_mems_allowed() {}
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_nodemask_valid_mems_allowed(_: *const nodemask_t) -> i32 { 1 }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn __cpuset_zone_allowed(_: *mut zone, _: gfp_t) -> bool { true }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_zone_allowed(_: *mut zone, _: gfp_t) -> bool { true }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_mems_allowed_intersects(_: *const task_struct, _: *const task_struct) -> i32 { 1 }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_memory_pressure_bump() {}
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_task_status_allowed(_: *mut seq_file, _: *mut task_struct) {}
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_mem_spread_node() -> i32 { 0 }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_do_page_mem_spread() -> i32 { 0 }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn current_cpuset_is_being_rebound() -> bool { false }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn dl_rebuild_rd_accounting() {}
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn rebuild_sched_domains() { partition_sched_domains(1, core::ptr::null_mut(), core::ptr::null_mut()); }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_reset_sched_domains() { partition_sched_domains(1, core::ptr::null_mut(), core::ptr::null_mut()); }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_print_current_mems_allowed() {}
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn set_mems_allowed(_: nodemask_t) {}
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn read_mems_allowed_begin() -> u32 { 0 }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn read_mems_allowed_retry(_: u32) -> bool { false }
#[cfg(not(CONFIG_CPUSETS))]
#[inline] pub unsafe fn cpuset_nodes_allowed(_: *mut cgroup, mask: *mut nodemask_t) { nodes_copy(*mask, node_states[N_MEMORY]); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
