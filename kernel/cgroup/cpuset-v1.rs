// SPDX-License-Identifier: GPL-2.0-or-later

// Dependencies supplied by cgroup-internal.h and cpuset-internal.h remain external.

/* Legacy hierarchy call to cgroup_transfer_tasks() is handled asynchronously. */
#[repr(C)]
struct cpuset_remove_tasks_struct {
    work: work_struct,
    cs: *mut cpuset,
}

/* Frequency meter constants. */
const FM_COEF: u32 = 933;
const FM_MAXTICKS: u32 = 99;
const FM_MAXCNT: u32 = 1_000_000;
const FM_SCALE: u32 = 1000;

unsafe fn fmeter_init(fmp: *mut fmeter) {
    (*fmp).cnt = 0;
    (*fmp).val = 0;
    (*fmp).time = 0;
    spin_lock_init(&mut (*fmp).lock);
}

unsafe fn fmeter_update(fmp: *mut fmeter) {
    let now: time64_t = ktime_get_seconds();
    let mut ticks: u32 = (now - (*fmp).time) as u32;
    if ticks == 0 { return; }
    ticks = core::cmp::min(FM_MAXTICKS, ticks);
    while ticks > 0 {
        ticks -= 1;
        (*fmp).val = (FM_COEF * (*fmp).val) / FM_SCALE;
    }
    (*fmp).time = now;
    (*fmp).val += ((FM_SCALE - FM_COEF) * (*fmp).cnt) / FM_SCALE;
    (*fmp).cnt = 0;
}

unsafe fn fmeter_markevent(fmp: *mut fmeter) {
    spin_lock(&mut (*fmp).lock);
    fmeter_update(fmp);
    (*fmp).cnt = core::cmp::min(FM_MAXCNT, (*fmp).cnt + FM_SCALE);
    spin_unlock(&mut (*fmp).lock);
}

unsafe fn fmeter_getrate(fmp: *mut fmeter) -> i32 {
    spin_lock(&mut (*fmp).lock);
    fmeter_update(fmp);
    let val = (*fmp).val as i32;
    spin_unlock(&mut (*fmp).lock);
    val
}

pub static mut cpuset_memory_pressure_enabled: i32 = 0;

pub unsafe fn __cpuset_memory_pressure_bump() {
    rcu_read_lock();
    fmeter_markevent(&mut (*task_cs(current)).fmeter);
    rcu_read_unlock();
}

unsafe fn update_relax_domain_level(cs: *mut cpuset, val: s64) -> i32 {
    // CONFIG_SMP: reject values below -1 or above sched_domain_level_max + 1.
    #[cfg(CONFIG_SMP)]
    if val < -1 || val > sched_domain_level_max + 1 { return -EINVAL; }
    if val != (*cs).relax_domain_level {
        (*cs).relax_domain_level = val;
        if !cpumask_empty((*cs).cpus_allowed) && is_sched_load_balance(cs) {
            rebuild_sched_domains_locked();
        }
    }
    0
}

unsafe fn cpuset_write_s64(css: *mut cgroup_subsys_state, cft: *mut cftype, val: s64) -> i32 {
    let cs = css_cs(css);
    let kind = (*cft).private;
    let mut retval = -ENODEV;
    cpuset_full_lock();
    if !is_cpuset_online(cs) { cpuset_full_unlock(); return retval; }
    match kind {
        FILE_SCHED_RELAX_DOMAIN_LEVEL => {
            pr_info_once!("cpuset.%s is deprecated\n", (*cft).name);
            retval = update_relax_domain_level(cs, val);
        }
        _ => retval = -EINVAL,
    }
    cpuset_full_unlock();
    retval
}

unsafe fn cpuset_read_s64(css: *mut cgroup_subsys_state, cft: *mut cftype) -> s64 {
    let cs = css_cs(css);
    match (*cft).private {
        FILE_SCHED_RELAX_DOMAIN_LEVEL => (*cs).relax_domain_level,
        _ => { BUG!(); 0 }
    }
}

pub unsafe fn cpuset1_update_task_spread_flags(cs: *mut cpuset, tsk: *mut task_struct) {
    if cgroup_subsys_on_dfl(cpuset_cgrp_subsys) { return; }
    if is_spread_page(cs) { task_set_spread_page(tsk); }
    else { task_clear_spread_page(tsk); }
}

pub unsafe fn cpuset1_update_tasks_flags(cs: *mut cpuset) {
    let mut it: css_task_iter = core::mem::zeroed();
    let mut task: *mut task_struct;
    css_task_iter_start(&(*cs).css, 0, &mut it);
    loop {
        task = css_task_iter_next(&mut it);
        if task.is_null() { break; }
        cpuset1_update_task_spread_flags(cs, task);
    }
    css_task_iter_end(&mut it);
}

unsafe fn remove_tasks_in_empty_cpuset(cs: *mut cpuset) {
    let mut parent = parent_cs(cs);
    while cpumask_empty((*parent).cpus_allowed) || nodes_empty((*parent).mems_allowed) {
        parent = parent_cs(parent);
    }
    if cgroup_transfer_tasks((*parent).css.cgroup, (*cs).css.cgroup) != 0 {
        pr_err!("cpuset: failed to transfer tasks out of empty cpuset ");
        pr_cont_cgroup_name((*cs).css.cgroup);
        pr_cont!("\n");
    }
}

unsafe fn cpuset_migrate_tasks_workfn(work: *mut work_struct) {
    let s = container_of!(work, cpuset_remove_tasks_struct, work);
    remove_tasks_in_empty_cpuset((*s).cs);
    css_put(&mut (*s).cs.as_ref().unwrap().css);
    kfree(s as *mut core::ffi::c_void);
}

pub unsafe fn cpuset1_hotplug_update_tasks(cs: *mut cpuset, new_cpus: *mut cpumask,
    new_mems: *mut nodemask_t, cpus_updated: bool, mems_updated: bool) {
    cpuset_callback_lock_irq();
    cpumask_copy((*cs).cpus_allowed, new_cpus);
    cpumask_copy((*cs).effective_cpus, new_cpus);
    (*cs).mems_allowed = *new_mems;
    (*cs).effective_mems = *new_mems;
    cpuset_callback_unlock_irq();
    if cpus_updated && !cpumask_empty((*cs).cpus_allowed) { cpuset_update_tasks_cpumask(cs, new_cpus); }
    if mems_updated && !nodes_empty((*cs).mems_allowed) { cpuset_update_tasks_nodemask(cs); }
    let is_empty = cpumask_empty((*cs).cpus_allowed) || nodes_empty((*cs).mems_allowed);
    if is_empty && cgroup_has_tasks((*cs).css.cgroup) && css_tryget_online(&mut (*cs).css) {
        let s = kzalloc_obj::<cpuset_remove_tasks_struct>();
        if WARN_ON_ONCE(s.is_null()) { css_put(&mut (*cs).css); return; }
        (*s).cs = cs;
        INIT_WORK(&mut (*s).work, cpuset_migrate_tasks_workfn);
        schedule_work(&mut (*s).work);
    }
}

unsafe fn is_cpuset_subset(p: *const cpuset, q: *const cpuset) -> bool {
    cpumask_subset((*p).cpus_allowed, (*q).cpus_allowed) &&
    nodes_subset((*p).mems_allowed, (*q).mems_allowed) &&
    (is_cpu_exclusive(p) as i32) <= (is_cpu_exclusive(q) as i32) &&
    (is_mem_exclusive(p) as i32) <= (is_mem_exclusive(q) as i32)
}

pub unsafe fn cpuset1_validate_change(cur: *mut cpuset, trial: *mut cpuset) -> i32 {
    let mut css: *mut cgroup_subsys_state = core::ptr::null_mut();
    let mut c: *mut cpuset;
    let mut ret = -EBUSY;
    cpuset_for_each_child!(c, css, cur) {
        if !is_cpuset_subset(c, trial) { return ret; }
    }
    ret = -EACCES;
    let par = parent_cs(cur);
    if !par.is_null() && !is_cpuset_subset(trial, par) { return ret; }
    ret = -ENOSPC;
    if cpuset_is_populated(cur) {
        if !cpumask_empty((*cur).cpus_allowed) && cpumask_empty((*trial).cpus_allowed) { return ret; }
        if !nodes_empty((*cur).mems_allowed) && nodes_empty((*trial).mems_allowed) { return ret; }
    }
    0
}

pub unsafe fn cpuset1_cpus_excl_conflict(cs1: *mut cpuset, cs2: *mut cpuset) -> bool {
    if is_cpu_exclusive(cs1) || is_cpu_exclusive(cs2) {
        return cpumask_intersects((*cs1).cpus_allowed, (*cs2).cpus_allowed);
    }
    false
}

// CONFIG_PROC_PID_CPUSET conditional section is preserved below.
#[cfg(CONFIG_PROC_PID_CPUSET)]
pub unsafe fn proc_cpuset_show(m: *mut seq_file, _ns: *mut pid_namespace,
    _pid: *mut pid, tsk: *mut task_struct) -> i32 {
    let buf = kmalloc(PATH_MAX, GFP_KERNEL);
    if buf.is_null() { return -ENOMEM; }
    rcu_read_lock();
    spin_lock_irq(&mut css_set_lock);
    let css = task_css(tsk, cpuset_cgrp_id);
    let mut retval = cgroup_path_ns_locked((*css).cgroup, buf, PATH_MAX, (*current).nsproxy.cgroup_ns);
    spin_unlock_irq(&mut css_set_lock);
    rcu_read_unlock();
    if retval == -E2BIG { retval = -ENAMETOOLONG; }
    if retval >= 0 { seq_puts(m, buf); seq_putc(m, b'\n' as i32); retval = 0; }
    kfree(buf as *mut core::ffi::c_void);
    retval
}

unsafe fn cpuset_read_u64(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    let cs = css_cs(css);
    match (*cft).private {
        FILE_CPU_EXCLUSIVE => is_cpu_exclusive(cs) as u64,
        FILE_MEM_EXCLUSIVE => is_mem_exclusive(cs) as u64,
        FILE_MEM_HARDWALL => is_mem_hardwall(cs) as u64,
        FILE_SCHED_LOAD_BALANCE => is_sched_load_balance(cs) as u64,
        FILE_MEMORY_MIGRATE => is_memory_migrate(cs) as u64,
        FILE_MEMORY_PRESSURE_ENABLED => cpuset_memory_pressure_enabled as u64,
        FILE_MEMORY_PRESSURE => fmeter_getrate(&mut (*cs).fmeter) as u64,
        FILE_SPREAD_PAGE => is_spread_page(cs) as u64,
        FILE_SPREAD_SLAB => is_spread_slab(cs) as u64,
        _ => { BUG!(); 0 }
    }
}

unsafe fn cpuset_write_u64(css: *mut cgroup_subsys_state, cft: *mut cftype, val: u64) -> i32 {
    let cs = css_cs(css);
    let kind = (*cft).private;
    let mut retval = 0;
    cpuset_full_lock();
    if !is_cpuset_online(cs) { cpuset_full_unlock(); return -ENODEV; }
    match kind {
        FILE_CPU_EXCLUSIVE => retval = cpuset_update_flag(CS_CPU_EXCLUSIVE, cs, val),
        FILE_MEM_EXCLUSIVE => { pr_info_once!("cpuset.%s is deprecated\n", (*cft).name); retval = cpuset_update_flag(CS_MEM_EXCLUSIVE, cs, val); }
        FILE_MEM_HARDWALL => { pr_info_once!("cpuset.%s is deprecated\n", (*cft).name); retval = cpuset_update_flag(CS_MEM_HARDWALL, cs, val); }
        FILE_SCHED_LOAD_BALANCE => { pr_info_once!("cpuset.%s is deprecated, use cpuset.cpus.partition instead\n", (*cft).name); retval = cpuset_update_flag(CS_SCHED_LOAD_BALANCE, cs, val); }
        FILE_MEMORY_MIGRATE => { pr_info_once!("cpuset.%s is deprecated\n", (*cft).name); retval = cpuset_update_flag(CS_MEMORY_MIGRATE, cs, val); }
        FILE_MEMORY_PRESSURE_ENABLED => { pr_info_once!("cpuset.%s is deprecated, use memory.pressure with CONFIG_PSI instead\n", (*cft).name); cpuset_memory_pressure_enabled = (val != 0) as i32; }
        FILE_SPREAD_PAGE => { pr_info_once!("cpuset.%s is deprecated\n", (*cft).name); retval = cpuset_update_flag(CS_SPREAD_PAGE, cs, val); }
        FILE_SPREAD_SLAB => { pr_warn_once!("cpuset.%s is deprecated\n", (*cft).name); retval = cpuset_update_flag(CS_SPREAD_SLAB, cs, val); }
        _ => retval = -EINVAL,
    }
    cpuset_full_unlock();
    retval
}

pub unsafe fn cpuset1_init(cs: *mut cpuset) { fmeter_init(&mut (*cs).fmeter); (*cs).relax_domain_level = -1; }

pub unsafe fn cpuset1_online_css(css: *mut cgroup_subsys_state) {
    let cs = css_cs(css);
    let parent = parent_cs(cs);
    lockdep_assert_cpus_held(); lockdep_assert_cpuset_lock_held();
    if is_spread_page(parent) { set_bit(CS_SPREAD_PAGE, &mut (*cs).flags); }
    if is_spread_slab(parent) { set_bit(CS_SPREAD_SLAB, &mut (*cs).flags); }
    if !test_bit(CGRP_CPUSET_CLONE_CHILDREN, (*css).cgroup.flags) { return; }
    rcu_read_lock();
    let mut tmp_cs: *mut cpuset; let mut pos_css: *mut cgroup_subsys_state = core::ptr::null_mut();
    cpuset_for_each_child!(tmp_cs, pos_css, parent) {
        if is_mem_exclusive(tmp_cs) || is_cpu_exclusive(tmp_cs) { rcu_read_unlock(); return; }
    }
    rcu_read_unlock();
    cpuset_callback_lock_irq();
    (*cs).mems_allowed = (*parent).mems_allowed; (*cs).effective_mems = (*parent).mems_allowed;
    cpumask_copy((*cs).cpus_allowed, (*parent).cpus_allowed); cpumask_copy((*cs).effective_cpus, (*parent).cpus_allowed);
    cpuset_callback_unlock_irq();
}

unsafe fn update_domain_attr(dattr: *mut sched_domain_attr, c: *mut cpuset) {
    if (*dattr).relax_domain_level < (*c).relax_domain_level { (*dattr).relax_domain_level = (*c).relax_domain_level; }
}

unsafe fn update_domain_attr_tree(dattr: *mut sched_domain_attr, root_cs: *mut cpuset) {
    rcu_read_lock();
    let mut cp: *mut cpuset; let mut pos_css: *mut cgroup_subsys_state = core::ptr::null_mut();
    cpuset_for_each_descendant_pre!(cp, pos_css, root_cs) {
        if cpumask_empty((*cp).cpus_allowed) { pos_css = css_rightmost_descendant(pos_css); continue; }
        if is_sched_load_balance(cp) { update_domain_attr(dattr, cp); }
    }
    rcu_read_unlock();
}

// The scheduler-domain generation algorithm and cpuset1_files table retain the
// source ABI and require the corresponding kernel types/macros from dependencies.
// Their declarations are intentionally left as external dependency references.

pub unsafe fn cpuset1_generate_sched_domains(domains: *mut cpumask_var_t, attributes: *mut *mut sched_domain_attr) -> i32 {
    let mut doms: *mut cpumask_var_t = core::ptr::null_mut();
    let mut dattr: *mut sched_domain_attr = core::ptr::null_mut();
    if is_sched_load_balance(&top_cpuset) {
        doms = alloc_sched_domains(1);
        if !doms.is_null() {
            dattr = kmalloc_obj::<sched_domain_attr>();
            if !dattr.is_null() { *dattr = SD_ATTR_INIT; update_domain_attr_tree(dattr, &mut top_cpuset); }
            cpumask_and((*doms), top_cpuset.effective_cpus, housekeeping_cpumask(HK_TYPE_DOMAIN));
        }
        *domains = doms; *attributes = dattr; return 1;
    }
    let csa = kmalloc_objs::<*mut cpuset>(nr_cpusets());
    if csa.is_null() { *domains = core::ptr::null_mut(); *attributes = core::ptr::null_mut(); return 1; }
    let mut csn = 0usize;
    rcu_read_lock();
    let mut cp: *mut cpuset; let mut pos_css: *mut cgroup_subsys_state = core::ptr::null_mut();
    cpuset_for_each_descendant_pre!(cp, pos_css, &mut top_cpuset) {
        if cp == &mut top_cpuset { continue; }
        if !cpumask_empty((*cp).cpus_allowed) && !(is_sched_load_balance(cp) && cpumask_intersects((*cp).cpus_allowed, housekeeping_cpumask(HK_TYPE_DOMAIN))) { continue; }
        if is_sched_load_balance(cp) && !cpumask_empty((*cp).effective_cpus) { *csa.add(csn) = cp; csn += 1; }
        pos_css = css_rightmost_descendant(pos_css);
    }
    rcu_read_unlock();
    for i in 0..csn { uf_node_init(&mut (*csa.add(i)).node); }
    for i in 0..csn { for j in (i + 1)..csn { if cpusets_overlap(*csa.add(i), *csa.add(j)) { uf_union(&mut (*csa.add(i)).node, &mut (*csa.add(j)).node); } } }
    let mut ndoms = 0i32;
    for i in 0..csn { if uf_find(&mut (*csa.add(i)).node) == &mut (*csa.add(i)).node { ndoms += 1; } }
    doms = alloc_sched_domains(ndoms);
    if !doms.is_null() {
        dattr = kmalloc_objs::<sched_domain_attr>(ndoms as usize);
        let mut nslot = 0usize;
        for i in 0..csn {
            let mut update = false;
            for j in i..csn { if uf_find(&mut (*csa.add(j)).node) == &mut (*csa.add(i)).node {
                let dp = *doms.add(nslot);
                if i == j { update = true; cpumask_clear(dp); if !dattr.is_null() { *dattr.add(nslot) = SD_ATTR_INIT; } }
                cpumask_or(dp, dp, (*(*csa.add(j))).effective_cpus); cpumask_and(dp, dp, housekeeping_cpumask(HK_TYPE_DOMAIN));
                if !dattr.is_null() { update_domain_attr_tree(dattr.add(nslot), *csa.add(j)); }
            }}
            if update { nslot += 1; }
        }
        BUG_ON(nslot as i32 != ndoms);
    }
    kfree(csa as *mut core::ffi::c_void);
    if doms.is_null() { ndoms = 1; }
    *domains = doms; *attributes = dattr; ndoms
}

// The following file descriptors preserve the v1 ABI; callback and enum types
// are supplied by the cgroup/cpuset dependency headers.
#[no_mangle]
pub static mut cpuset1_files: [cftype; 15] = [
    cftype { name: "cpus", seq_show: Some(cpuset_common_seq_show), write: Some(cpuset_write_resmask), max_write_len: 100 + 6 * NR_CPUS, private: FILE_CPULIST },
    cftype { name: "mems", seq_show: Some(cpuset_common_seq_show), write: Some(cpuset_write_resmask), max_write_len: 100 + 6 * MAX_NUMNODES, private: FILE_MEMLIST },
    cftype { name: "effective_cpus", seq_show: Some(cpuset_common_seq_show), private: FILE_EFFECTIVE_CPULIST },
    cftype { name: "effective_mems", seq_show: Some(cpuset_common_seq_show), private: FILE_EFFECTIVE_MEMLIST },
    cftype { name: "cpu_exclusive", read_u64: Some(cpuset_read_u64), write_u64: Some(cpuset_write_u64), private: FILE_CPU_EXCLUSIVE },
    cftype { name: "mem_exclusive", read_u64: Some(cpuset_read_u64), write_u64: Some(cpuset_write_u64), private: FILE_MEM_EXCLUSIVE },
    cftype { name: "mem_hardwall", read_u64: Some(cpuset_read_u64), write_u64: Some(cpuset_write_u64), private: FILE_MEM_HARDWALL },
    cftype { name: "sched_load_balance", read_u64: Some(cpuset_read_u64), write_u64: Some(cpuset_write_u64), private: FILE_SCHED_LOAD_BALANCE },
    cftype { name: "sched_relax_domain_level", read_s64: Some(cpuset_read_s64), write_s64: Some(cpuset_write_s64), private: FILE_SCHED_RELAX_DOMAIN_LEVEL },
    cftype { name: "memory_migrate", read_u64: Some(cpuset_read_u64), write_u64: Some(cpuset_write_u64), private: FILE_MEMORY_MIGRATE },
    cftype { name: "memory_pressure", read_u64: Some(cpuset_read_u64), private: FILE_MEMORY_PRESSURE },
    cftype { name: "memory_spread_page", read_u64: Some(cpuset_read_u64), write_u64: Some(cpuset_write_u64), private: FILE_SPREAD_PAGE },
    cftype { name: "memory_spread_slab", read_u64: Some(cpuset_read_u64), write_u64: Some(cpuset_write_u64), private: FILE_SPREAD_SLAB },
    cftype { name: "memory_pressure_enabled", flags: CFTYPE_ONLY_ON_ROOT, read_u64: Some(cpuset_read_u64), write_u64: Some(cpuset_write_u64), private: FILE_MEMORY_PRESSURE_ENABLED },
    cftype::default(),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
