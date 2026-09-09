// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * coupled.c - helper functions to enter the same idle state on multiple cpus
 *
 * Copyright (c) 2011 Google, Inc.
 *
 * Author: Colin Cross <ccross@android.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

/** DOC: Coupled cpuidle states
 *
 * This file implements coordinated entry into coupled idle states.
 */

#[repr(C)]
pub struct cpuidle_coupled {
    pub coupled_cpus: cpumask_t,
    pub requested_state: [c_int; NR_CPUS],
    pub ready_waiting_counts: atomic_t,
    pub abort_barrier: atomic_t,
    pub online_count: c_int,
    pub refcnt: c_int,
    pub prevent: c_int,
}

pub const WAITING_BITS: c_int = 16;
pub const MAX_WAITING_CPUS: c_int = 1 << WAITING_BITS;
pub const WAITING_MASK: c_int = MAX_WAITING_CPUS - 1;
pub const READY_MASK: c_int = !WAITING_MASK;
pub const CPUIDLE_COUPLED_NOT_IDLE: c_int = -1;

static mut cpuidle_coupled_poke_cb: [call_single_data_t; NR_CPUS] = [/* supplied by kernel */ unsafe { core::mem::zeroed() }; NR_CPUS];
static mut cpuidle_coupled_poke_pending: cpumask_t = unsafe { core::mem::zeroed() };
static mut cpuidle_coupled_poked: cpumask_t = unsafe { core::mem::zeroed() };

pub unsafe fn cpuidle_coupled_parallel_barrier(dev: *mut cpuidle_device, a: *mut atomic_t) {
    let n = (*(*dev).coupled).online_count;
    smp_mb__before_atomic();
    atomic_inc(a);
    while atomic_read(a) < n { cpu_relax(); }
    if atomic_inc_return(a) == n * 2 {
        atomic_set(a, 0);
        return;
    }
    while atomic_read(a) > n { cpu_relax(); }
}

pub unsafe fn cpuidle_state_is_coupled(drv: *mut cpuidle_driver, state: c_int) -> bool {
    ((*drv).states.add(state as usize).flags & CPUIDLE_FLAG_COUPLED) != 0
}

pub unsafe fn cpuidle_coupled_state_verify(drv: *mut cpuidle_driver) -> c_int {
    let mut i = (*drv).state_count - 1;
    while i >= 0 {
        if cpuidle_state_is_coupled(drv, i) &&
            ((*drv).safe_state_index == i || (*drv).safe_state_index < 0 ||
             (*drv).safe_state_index >= (*drv).state_count) { return -EINVAL; }
        i -= 1;
    }
    0
}

unsafe fn cpuidle_coupled_set_ready(c: *mut cpuidle_coupled) { atomic_add(MAX_WAITING_CPUS, &mut (*c).ready_waiting_counts); }
unsafe fn cpuidle_coupled_set_not_ready(c: *mut cpuidle_coupled) -> c_int {
    let all = (*c).online_count | ((*c).online_count << WAITING_BITS);
    if atomic_add_unless(&mut (*c).ready_waiting_counts, -MAX_WAITING_CPUS, all) { 0 } else { -EINVAL }
}
unsafe fn cpuidle_coupled_no_cpus_ready(c: *mut cpuidle_coupled) -> c_int { (atomic_read(&mut (*c).ready_waiting_counts) >> WAITING_BITS) == 0 as c_int }
unsafe fn cpuidle_coupled_cpus_ready(c: *mut cpuidle_coupled) -> bool { (atomic_read(&mut (*c).ready_waiting_counts) >> WAITING_BITS) == (*c).online_count }
unsafe fn cpuidle_coupled_cpus_waiting(c: *mut cpuidle_coupled) -> bool { (atomic_read(&mut (*c).ready_waiting_counts) & WAITING_MASK) == (*c).online_count }
unsafe fn cpuidle_coupled_no_cpus_waiting(c: *mut cpuidle_coupled) -> c_int { (atomic_read(&mut (*c).ready_waiting_counts) & WAITING_MASK) == 0 as c_int }

unsafe fn cpuidle_coupled_get_state(dev: *mut cpuidle_device, c: *mut cpuidle_coupled) -> c_int {
    let mut state = INT_MAX;
    smp_rmb();
    for_each_cpu!(i, &(*c).coupled_cpus, {
        if cpu_online(i) && (*c).requested_state[i as usize] < state { state = (*c).requested_state[i as usize]; }
    });
    state
}

unsafe extern "C" fn cpuidle_coupled_handle_poke(info: *mut c_void) {
    let cpu = info as usize as c_int;
    cpumask_set_cpu(cpu, &mut cpuidle_coupled_poked);
    cpumask_clear_cpu(cpu, &mut cpuidle_coupled_poke_pending);
}
unsafe fn cpuidle_coupled_poke(cpu: c_int) {
    let csd = &mut cpuidle_coupled_poke_cb[cpu as usize];
    if !cpumask_test_and_set_cpu(cpu, &mut cpuidle_coupled_poke_pending) { smp_call_function_single_async(cpu, csd); }
}
unsafe fn cpuidle_coupled_poke_others(this_cpu: c_int, c: *mut cpuidle_coupled) {
    for_each_cpu!(cpu, &(*c).coupled_cpus, { if cpu != this_cpu && cpu_online(cpu) { cpuidle_coupled_poke(cpu); } });
}
unsafe fn cpuidle_coupled_set_waiting(cpu: c_int, c: *mut cpuidle_coupled, next: c_int) -> c_int {
    (*c).requested_state[cpu as usize] = next;
    atomic_inc_return(&mut (*c).ready_waiting_counts) & WAITING_MASK
}
unsafe fn cpuidle_coupled_set_not_waiting(cpu: c_int, c: *mut cpuidle_coupled) {
    atomic_dec(&mut (*c).ready_waiting_counts); (*c).requested_state[cpu as usize] = CPUIDLE_COUPLED_NOT_IDLE;
}
unsafe fn cpuidle_coupled_set_done(cpu: c_int, c: *mut cpuidle_coupled) {
    cpuidle_coupled_set_not_waiting(cpu, c); atomic_sub(MAX_WAITING_CPUS, &mut (*c).ready_waiting_counts);
}
unsafe fn cpuidle_coupled_clear_pokes(cpu: c_int) -> c_int {
    if !cpumask_test_cpu(cpu, &cpuidle_coupled_poke_pending) { return 0; }
    local_irq_enable(); while cpumask_test_cpu(cpu, &cpuidle_coupled_poke_pending) { cpu_relax(); } local_irq_disable(); 1
}
unsafe fn cpuidle_coupled_any_pokes_pending(c: *mut cpuidle_coupled) -> bool {
    cpumask_first_and_and(&cpu_online_mask, &(*c).coupled_cpus, &cpuidle_coupled_poke_pending) < nr_cpu_ids
}

pub unsafe fn cpuidle_enter_state_coupled(dev: *mut cpuidle_device, drv: *mut cpuidle_driver, mut next_state: c_int) -> c_int {
    let mut entered_state = -1;
    let c = (*dev).coupled;
    if c.is_null() { return -EINVAL; }
    while (*c).prevent != 0 {
        cpuidle_coupled_clear_pokes((*dev).cpu);
        if need_resched() { local_irq_enable(); return entered_state; }
        entered_state = cpuidle_enter_state(dev, drv, (*drv).safe_state_index); local_irq_disable();
    }
    smp_rmb();
    'reset: loop {
        cpumask_clear_cpu((*dev).cpu, &mut cpuidle_coupled_poked);
        let w = cpuidle_coupled_set_waiting((*dev).cpu, c, next_state);
        if w == (*c).online_count { cpumask_set_cpu((*dev).cpu, &mut cpuidle_coupled_poked); cpuidle_coupled_poke_others((*dev).cpu, c); }
        loop {
            while !cpuidle_coupled_cpus_waiting(c) || !cpumask_test_cpu((*dev).cpu, &cpuidle_coupled_poked) {
                if cpuidle_coupled_clear_pokes((*dev).cpu) != 0 { continue; }
                if need_resched() { cpuidle_coupled_set_not_waiting((*dev).cpu, c); break 'reset; }
                if (*c).prevent != 0 { cpuidle_coupled_set_not_waiting((*dev).cpu, c); break 'reset; }
                entered_state = cpuidle_enter_state(dev, drv, (*drv).safe_state_index); local_irq_disable();
            }
            cpuidle_coupled_clear_pokes((*dev).cpu);
            if need_resched() { cpuidle_coupled_set_not_waiting((*dev).cpu, c); break 'reset; }
            smp_wmb(); cpuidle_coupled_set_ready(c);
            while !cpuidle_coupled_cpus_ready(c) {
                if !cpuidle_coupled_cpus_waiting(c) && cpuidle_coupled_set_not_ready(c) == 0 { continue; }
                cpu_relax();
            }
            smp_rmb();
            if cpuidle_coupled_any_pokes_pending(c) { cpuidle_coupled_set_done((*dev).cpu, c); cpuidle_coupled_parallel_barrier(dev, &mut (*c).abort_barrier); continue 'reset; }
            next_state = cpuidle_coupled_get_state(dev, c); entered_state = cpuidle_enter_state(dev, drv, next_state); cpuidle_coupled_set_done((*dev).cpu, c); break 'reset;
        }
    }
    local_irq_enable(); while cpuidle_coupled_no_cpus_ready(c) == 0 { cpu_relax(); } entered_state
}

unsafe fn cpuidle_coupled_update_online_cpus(c: *mut cpuidle_coupled) { (*c).online_count = cpumask_weight_and(&cpu_online_mask, &(*c).coupled_cpus); }

pub unsafe fn cpuidle_coupled_register_device(dev: *mut cpuidle_device) -> c_int {
    if cpumask_empty(&(*dev).coupled_cpus) { return 0; }
    let mut coupled: *mut cpuidle_coupled = core::ptr::null_mut();
    for_each_cpu!(cpu, &(*dev).coupled_cpus, { let d = per_cpu!(cpuidle_devices, cpu); if !d.is_null() && !(*d).coupled.is_null() { coupled = (*d).coupled; } });
    if coupled.is_null() { coupled = kzalloc_obj::<cpuidle_coupled>(); if coupled.is_null() { return -ENOMEM; } (*coupled).coupled_cpus = (*dev).coupled_cpus; }
    (*dev).coupled = coupled;
    if WARN_ON(!cpumask_equal(&(*dev).coupled_cpus, &(*coupled).coupled_cpus)) { (*coupled).prevent += 1; }
    cpuidle_coupled_update_online_cpus(coupled); (*coupled).refcnt += 1;
    let csd = &mut cpuidle_coupled_poke_cb[(*dev).cpu as usize]; INIT_CSD!(csd, cpuidle_coupled_handle_poke, (*dev).cpu as usize as *mut c_void); 0
}

pub unsafe fn cpuidle_coupled_unregister_device(dev: *mut cpuidle_device) {
    let c = (*dev).coupled; if cpumask_empty(&(*dev).coupled_cpus) { return; }
    (*c).refcnt -= 1; if (*c).refcnt == 0 { kfree(c); } (*dev).coupled = core::ptr::null_mut();
}

unsafe fn cpuidle_coupled_prevent_idle(c: *mut cpuidle_coupled) { let cpu = get_cpu(); (*c).prevent += 1; cpuidle_coupled_poke_others(cpu, c); put_cpu(); while cpuidle_coupled_no_cpus_waiting(c) == 0 { cpu_relax(); } }
unsafe fn cpuidle_coupled_allow_idle(c: *mut cpuidle_coupled) { let cpu = get_cpu(); smp_wmb(); (*c).prevent -= 1; cpuidle_coupled_poke_others(cpu, c); put_cpu(); }
unsafe extern "C" fn coupled_cpu_online(cpu: c_uint) -> c_int { mutex_lock(&mut cpuidle_lock); let d = per_cpu!(cpuidle_devices, cpu); if !d.is_null() && !(*d).coupled.is_null() { cpuidle_coupled_update_online_cpus((*d).coupled); cpuidle_coupled_allow_idle((*d).coupled); } mutex_unlock(&mut cpuidle_lock); 0 }
unsafe extern "C" fn coupled_cpu_up_prepare(cpu: c_uint) -> c_int { mutex_lock(&mut cpuidle_lock); let d = per_cpu!(cpuidle_devices, cpu); if !d.is_null() && !(*d).coupled.is_null() { cpuidle_coupled_prevent_idle((*d).coupled); } mutex_unlock(&mut cpuidle_lock); 0 }
unsafe extern "C" fn cpuidle_coupled_init() -> c_int {
    let mut ret = cpuhp_setup_state_nocalls(CPUHP_CPUIDLE_COUPLED_PREPARE, c"cpuidle/coupled:prepare", coupled_cpu_up_prepare, coupled_cpu_online);
    if ret != 0 { return ret; }
    ret = cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, c"cpuidle/coupled:online", coupled_cpu_online, coupled_cpu_up_prepare);
    if ret < 0 { cpuhp_remove_state_nocalls(CPUHP_CPUIDLE_COUPLED_PREPARE); } ret
}
core_initcall!(cpuidle_coupled_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
