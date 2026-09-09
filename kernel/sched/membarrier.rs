// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2010-2017 Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 *
 * membarrier system call
 */

// Dependency intent from <uapi/linux/membarrier.h> and "sched.h" is supplied
// by the surrounding kernel translation unit.

#[cfg(CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE)]
const MEMBARRIER_PRIVATE_EXPEDITED_SYNC_CORE_BITMASK: i32 =
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE |
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE;
#[cfg(not(CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE))]
const MEMBARRIER_PRIVATE_EXPEDITED_SYNC_CORE_BITMASK: i32 = 0;

#[cfg(CONFIG_RSEQ)]
const MEMBARRIER_PRIVATE_EXPEDITED_RSEQ_BITMASK: i32 =
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ |
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ;
#[cfg(not(CONFIG_RSEQ))]
const MEMBARRIER_PRIVATE_EXPEDITED_RSEQ_BITMASK: i32 = 0;

const MEMBARRIER_CMD_BITMASK: i32 = MEMBARRIER_CMD_GLOBAL |
    MEMBARRIER_CMD_GLOBAL_EXPEDITED |
    MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED |
    MEMBARRIER_CMD_PRIVATE_EXPEDITED |
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED |
    MEMBARRIER_PRIVATE_EXPEDITED_SYNC_CORE_BITMASK |
    MEMBARRIER_PRIVATE_EXPEDITED_RSEQ_BITMASK |
    MEMBARRIER_CMD_GET_REGISTRATIONS;

// DEFINE_LOCK_GUARD_0(mb, smp_mb(), smp_mb())
static mut membarrier_ipi_mutex: mutex = mutex::new();
static mut membarrier_cpu_mutexes: PerCpu<mutex> = PerCpu::new();

unsafe fn membarrier_init() -> i32 {
    let mut i: i32;
    for_each_possible_cpu!(i) {
        mutex_init(&mut per_cpu!(membarrier_cpu_mutexes, i));
    }
    0
}
core_initcall!(membarrier_init);

unsafe fn ipi_mb(_info: *mut core::ffi::c_void) { smp_mb(); }

unsafe fn ipi_sync_core(_info: *mut core::ffi::c_void) {
    smp_mb();
    sync_core_before_usermode();
}

unsafe fn ipi_rseq(_info: *mut core::ffi::c_void) {
    smp_mb();
    if rseq_v2(current) { rseq_sched_switch_event(current); }
    else { rseq_force_update(); }
}

unsafe fn ipi_sync_rq_state(info: *mut core::ffi::c_void) {
    let mm = info as *mut mm_struct;
    if (*current).mm != mm { return; }
    this_cpu_write!(runqueues.membarrier_state, atomic_read(&(*mm).membarrier_state));
    smp_mb();
}

pub unsafe fn membarrier_exec_mmap(mm: *mut mm_struct) {
    smp_mb();
    atomic_set(&mut (*mm).membarrier_state, 0);
    this_cpu_write!(runqueues.membarrier_state, 0);
}

pub unsafe fn membarrier_update_current_mm(next_mm: *mut mm_struct) {
    let rq = this_rq();
    let mut membarrier_state: i32 = 0;
    if !next_mm.is_null() { membarrier_state = atomic_read(&(*next_mm).membarrier_state); }
    if READ_ONCE!((*rq).membarrier_state) == membarrier_state { return; }
    WRITE_ONCE!((*rq).membarrier_state, membarrier_state);
}

unsafe fn membarrier_global_expedited() -> i32 {
    let mut tmpmask = CPUMASK_VAR_NULL;
    if num_online_cpus() == 1 { return 0; }
    if !zalloc_cpumask_var(&mut tmpmask, GFP_KERNEL) { return -ENOMEM; }
    let _mb = guard_mb();
    let _ipi = mutex_guard(&mut membarrier_ipi_mutex);
    let _cpus = cpus_read_lock_guard();
    rcu_read_lock();
    let mut cpu: i32;
    for_each_online_cpu!(cpu) {
        if cpu == raw_smp_processor_id() { continue; }
        if (READ_ONCE!((*cpu_rq(cpu)).membarrier_state) & MEMBARRIER_STATE_GLOBAL_EXPEDITED) == 0 { continue; }
        let p = rcu_dereference((*cpu_rq(cpu)).curr);
        if (*p).mm.is_null() { continue; }
        __cpumask_set_cpu(cpu, &mut tmpmask);
    }
    rcu_read_unlock();
    preempt_disable();
    smp_call_function_many(&tmpmask, ipi_mb, core::ptr::null_mut(), 1);
    preempt_enable();
    0
}

unsafe fn membarrier_private_expedited(flags: i32, cpu_id: i32) -> i32 {
    let mm = (*current).mm;
    let mut ipi_func: smp_call_func_t = ipi_mb;
    if flags == MEMBARRIER_FLAG_SYNC_CORE {
        if !IS_ENABLED!(CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE) { return -EINVAL; }
        if atomic_read(&(*mm).membarrier_state) & MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE_READY == 0 { return -EPERM; }
        ipi_func = ipi_sync_core; prepare_sync_core_cmd(mm);
    } else if flags == MEMBARRIER_FLAG_RSEQ {
        if !IS_ENABLED!(CONFIG_RSEQ) { return -EINVAL; }
        if atomic_read(&(*mm).membarrier_state) & MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ_READY == 0 { return -EPERM; }
        ipi_func = ipi_rseq;
    } else {
        WARN_ON_ONCE!(flags);
        if atomic_read(&(*mm).membarrier_state) & MEMBARRIER_STATE_PRIVATE_EXPEDITED_READY == 0 { return -EPERM; }
    }
    if flags != MEMBARRIER_FLAG_SYNC_CORE && (atomic_read(&(*mm).mm_users) == 1 || num_online_cpus() == 1) { return 0; }
    let _mb = guard_mb();
    if cpu_id >= 0 {
        if cpu_id >= nr_cpu_ids || !cpu_possible(cpu_id) || !cpu_online(cpu_id) { return 0; }
        let _ipi = mutex_guard(&mut per_cpu!(membarrier_cpu_mutexes, cpu_id));
        let _cpus = cpus_read_lock_guard();
        rcu_read_lock();
        let p = rcu_dereference((*cpu_rq(cpu_id)).curr);
        if p.is_null() || (*p).mm != mm { rcu_read_unlock(); return 0; }
        rcu_read_unlock();
        smp_call_function_single(cpu_id, ipi_func, core::ptr::null_mut(), 1);
    } else {
        let mut tmpmask = CPUMASK_VAR_NULL;
        if !zalloc_cpumask_var(&mut tmpmask, GFP_KERNEL) { return -ENOMEM; }
        let _ipi = mutex_guard(&mut membarrier_ipi_mutex);
        let _cpus = cpus_read_lock_guard();
        rcu_read_lock();
        let mut cpu: i32;
        for_each_online_cpu!(cpu) { let p = rcu_dereference((*cpu_rq(cpu)).curr); if !p.is_null() && (*p).mm == mm { __cpumask_set_cpu(cpu, &mut tmpmask); } }
        rcu_read_unlock();
        if flags != MEMBARRIER_FLAG_SYNC_CORE { preempt_disable(); smp_call_function_many(&tmpmask, ipi_func, core::ptr::null_mut(), true); preempt_enable(); }
        else { on_each_cpu_mask(&tmpmask, ipi_func, core::ptr::null_mut(), true); }
    }
    0
}

unsafe fn sync_runqueues_membarrier_state(mm: *mut mm_struct) -> i32 {
    let state = atomic_read(&(*mm).membarrier_state);
    if atomic_read(&(*mm).mm_users) == 1 || num_online_cpus() == 1 { this_cpu_write!(runqueues.membarrier_state, state); smp_mb(); return 0; }
    let mut tmpmask = CPUMASK_VAR_NULL;
    if !zalloc_cpumask_var(&mut tmpmask, GFP_KERNEL) { return -ENOMEM; }
    synchronize_rcu();
    let _ipi = mutex_guard(&mut membarrier_ipi_mutex);
    cpus_read_lock(); rcu_read_lock();
    let mut cpu: i32;
    for_each_online_cpu!(cpu) { let rq = cpu_rq(cpu); let p = rcu_dereference((*rq).curr); if !p.is_null() && (*p).mm == mm { __cpumask_set_cpu(cpu, &mut tmpmask); } }
    rcu_read_unlock(); on_each_cpu_mask(&tmpmask, ipi_sync_rq_state, mm as *mut _, true); free_cpumask_var(tmpmask); cpus_read_unlock(); 0
}

unsafe fn membarrier_register_global_expedited() -> i32 {
    let mm = (*current).mm;
    if atomic_read(&(*mm).membarrier_state) & MEMBARRIER_STATE_GLOBAL_EXPEDITED_READY != 0 { return 0; }
    atomic_or(MEMBARRIER_STATE_GLOBAL_EXPEDITED, &mut (*mm).membarrier_state);
    let ret = sync_runqueues_membarrier_state(mm); if ret != 0 { return ret; }
    atomic_or(MEMBARRIER_STATE_GLOBAL_EXPEDITED_READY, &mut (*mm).membarrier_state); 0
}

unsafe fn membarrier_register_private_expedited(flags: i32) -> i32 {
    let mm = (*current).mm;
    let mut ready_state = MEMBARRIER_STATE_PRIVATE_EXPEDITED_READY;
    let mut set_state = MEMBARRIER_STATE_PRIVATE_EXPEDITED;
    if flags == MEMBARRIER_FLAG_SYNC_CORE { if !IS_ENABLED!(CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE) { return -EINVAL; } ready_state = MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE_READY; }
    else if flags == MEMBARRIER_FLAG_RSEQ { if !IS_ENABLED!(CONFIG_RSEQ) { return -EINVAL; } ready_state = MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ_READY; }
    else { WARN_ON_ONCE!(flags); }
    if atomic_read(&(*mm).membarrier_state) & ready_state == ready_state { return 0; }
    if flags & MEMBARRIER_FLAG_SYNC_CORE != 0 { set_state |= MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE; }
    if flags & MEMBARRIER_FLAG_RSEQ != 0 { set_state |= MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ; }
    atomic_or(set_state, &mut (*mm).membarrier_state);
    let ret = sync_runqueues_membarrier_state(mm); if ret != 0 { return ret; }
    atomic_or(ready_state, &mut (*mm).membarrier_state); 0
}

unsafe fn membarrier_get_registrations() -> i32 {
    let mm = (*current).mm; let mut registrations_mask = 0; let mut state = atomic_read(&(*mm).membarrier_state);
    let states = [MEMBARRIER_STATE_GLOBAL_EXPEDITED | MEMBARRIER_STATE_GLOBAL_EXPEDITED_READY, MEMBARRIER_STATE_PRIVATE_EXPEDITED | MEMBARRIER_STATE_PRIVATE_EXPEDITED_READY, MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE | MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE_READY, MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ | MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ_READY];
    let cmds = [MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED, MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED, MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE, MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ];
    for i in 0..states.len() { if state & states[i] != 0 { registrations_mask |= cmds[i]; state &= !states[i]; } }
    WARN_ON_ONCE!(state != 0); registrations_mask
}

pub unsafe fn sys_membarrier(cmd: i32, flags: u32, mut cpu_id: i32) -> i32 {
    if cmd == MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ { if flags != 0 && flags != MEMBARRIER_CMD_FLAG_CPU { return -EINVAL; } } else if flags != 0 { return -EINVAL; }
    if flags & MEMBARRIER_CMD_FLAG_CPU == 0 { cpu_id = -1; }
    match cmd {
        MEMBARRIER_CMD_QUERY => { let mut mask = MEMBARRIER_CMD_BITMASK; if tick_nohz_full_enabled() { mask &= !MEMBARRIER_CMD_GLOBAL; } mask }
        MEMBARRIER_CMD_GLOBAL => { if tick_nohz_full_enabled() { return -EINVAL; } if num_online_cpus() > 1 { synchronize_rcu(); } 0 }
        MEMBARRIER_CMD_GLOBAL_EXPEDITED => membarrier_global_expedited(),
        MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED => membarrier_register_global_expedited(),
        MEMBARRIER_CMD_PRIVATE_EXPEDITED => membarrier_private_expedited(0, cpu_id),
        MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED => membarrier_register_private_expedited(0),
        MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE => membarrier_private_expedited(MEMBARRIER_FLAG_SYNC_CORE, cpu_id),
        MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE => membarrier_register_private_expedited(MEMBARRIER_FLAG_SYNC_CORE),
        MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ => membarrier_private_expedited(MEMBARRIER_FLAG_RSEQ, cpu_id),
        MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ => membarrier_register_private_expedited(MEMBARRIER_FLAG_RSEQ),
        MEMBARRIER_CMD_GET_REGISTRATIONS => membarrier_get_registrations(),
        _ => -EINVAL,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
