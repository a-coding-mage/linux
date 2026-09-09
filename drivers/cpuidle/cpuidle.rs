/*
 * cpuidle.c - core cpuidle infrastructure
 *
 * (C) 2006-2007 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>
 *               Shaohua Li <shaohua.li@intel.com>
 *               Adam Belay <abelay@novell.com>
 *
 * This code is licenced under the GPL.
 */

// Kernel headers and "cpuidle.h" provide the declarations used below.

static mut ENABLED_DEVICES: i32 = 0;
static mut OFF: i32 = 0;
static mut INITIALIZED: i32 = 0;

pub unsafe fn cpuidle_disabled() -> i32 { OFF }

pub unsafe fn disable_cpuidle() { OFF = 1; }

pub unsafe fn cpuidle_not_available(
    drv: *mut cpuidle_driver, dev: *mut cpuidle_device,
) -> bool {
    OFF != 0 || INITIALIZED == 0 || drv.is_null() || dev.is_null() || (*dev).enabled == 0
}

pub unsafe fn cpuidle_play_dead() -> i32 {
    let dev = __this_cpu_read_cpuidle_devices();
    let drv = cpuidle_get_cpu_driver(dev);
    if drv.is_null() { return -ENODEV; }
    let mut i = (*drv).state_count - 1;
    while i >= 0 {
        let state = &mut (*drv).states[i as usize];
        if let Some(enter_dead) = state.enter_dead { enter_dead(dev, i); }
        i -= 1;
    }
    -ENODEV
}

unsafe fn find_deepest_state(
    drv: *mut cpuidle_driver, dev: *mut cpuidle_device, max_latency_ns: u64,
    forbidden_flags: u32, s2idle: bool,
) -> i32 {
    let mut latency_req = 0u64;
    let mut ret = 0i32;
    let mut i = 1;
    while i < (*drv).state_count {
        let s = &(*drv).states[i as usize];
        if (*dev).states_usage[i as usize].disable != 0
            || s.exit_latency_ns <= latency_req
            || s.exit_latency_ns > max_latency_ns
            || (s.flags & forbidden_flags) != 0
            || (s2idle && s.enter_s2idle.is_none()) { i += 1; continue; }
        latency_req = s.exit_latency_ns;
        ret = i;
        i += 1;
    }
    ret
}

pub unsafe fn cpuidle_use_deepest_state(latency_limit_ns: u64) {
    preempt_disable();
    let dev = cpuidle_get_device();
    if !dev.is_null() { (*dev).forced_idle_latency_limit_ns = latency_limit_ns; }
    preempt_enable();
}

pub unsafe fn cpuidle_find_deepest_state(
    drv: *mut cpuidle_driver, dev: *mut cpuidle_device, latency_limit_ns: u64,
) -> i32 { find_deepest_state(drv, dev, latency_limit_ns, 0, false) }

#[cfg(CONFIG_SUSPEND)]
unsafe fn enter_s2idle_proper(drv: *mut cpuidle_driver, dev: *mut cpuidle_device, index: i32) {
    let target_state = &mut (*drv).states[index as usize];
    instrumentation_begin();
    let time_start = ns_to_ktime(local_clock_noinstr());
    tick_freeze();
    stop_critical_timings();
    if (target_state.flags & CPUIDLE_FLAG_RCU_IDLE) == 0 {
        ct_cpuidle_enter();
        instrumentation_begin();
    }
    if let Some(enter) = target_state.enter_s2idle { enter(dev, drv, index); }
    if WARN_ON_ONCE(!irqs_disabled()) { raw_local_irq_disable(); }
    if (target_state.flags & CPUIDLE_FLAG_RCU_IDLE) == 0 {
        instrumentation_end(); ct_cpuidle_exit();
    }
    tick_unfreeze(); start_critical_timings();
    let time_end = ns_to_ktime(local_clock_noinstr());
    (*dev).states_usage[index as usize].s2idle_time += ktime_us_delta(time_end, time_start);
    (*dev).states_usage[index as usize].s2idle_usage += 1;
    instrumentation_end();
}

#[cfg(CONFIG_SUSPEND)]
pub unsafe fn cpuidle_enter_s2idle(
    drv: *mut cpuidle_driver, dev: *mut cpuidle_device, latency_limit_ns: u64,
) -> i32 {
    let index = find_deepest_state(drv, dev, latency_limit_ns, 0, true);
    if index > 0 { enter_s2idle_proper(drv, dev, index); local_irq_enable(); }
    index
}

pub unsafe fn cpuidle_enter_state(
    dev: *mut cpuidle_device, drv: *mut cpuidle_driver, mut index: i32,
) -> i32 {
    let target_state = &mut (*drv).states[index as usize];
    let mut broadcast = (target_state.flags & CPUIDLE_FLAG_TIMER_STOP) != 0;
    instrumentation_begin();
    if broadcast && tick_broadcast_enter() {
        index = find_deepest_state(drv, dev, target_state.exit_latency_ns, CPUIDLE_FLAG_TIMER_STOP, false);
        broadcast = false;
    }
    let target_state = &mut (*drv).states[index as usize];
    if (target_state.flags & CPUIDLE_FLAG_TLB_FLUSHED) != 0 { leave_mm(); }
    sched_idle_set_state(target_state); trace_cpu_idle(index, (*dev).cpu);
    let time_start = ns_to_ktime(local_clock_noinstr());
    stop_critical_timings();
    if (target_state.flags & CPUIDLE_FLAG_RCU_IDLE) == 0 { ct_cpuidle_enter(); instrumentation_begin(); }
    let entered_state = target_state.enter(dev, drv, index);
    if WARN_ONCE(!irqs_disabled(), "%ps leaked IRQ state", target_state.enter) { raw_local_irq_disable(); }
    if (target_state.flags & CPUIDLE_FLAG_RCU_IDLE) == 0 { instrumentation_end(); ct_cpuidle_exit(); }
    start_critical_timings(); sched_clock_idle_wakeup_event();
    let time_end = ns_to_ktime(local_clock_noinstr()); trace_cpu_idle(PWR_EVENT_EXIT, (*dev).cpu);
    sched_idle_set_state(core::ptr::null_mut());
    if broadcast { tick_broadcast_exit(); }
    if !cpuidle_state_is_coupled(drv, index) { local_irq_enable(); }
    if entered_state >= 0 {
        let diff = ktime_sub(time_end, time_start);
        (*dev).last_residency_ns = diff;
        (*dev).states_usage[entered_state as usize].time_ns += diff;
        (*dev).states_usage[entered_state as usize].usage += 1;
        if diff < (*drv).states[entered_state as usize].target_residency_ns {
            let mut i = entered_state - 1;
            while i >= 0 { if (*dev).states_usage[i as usize].disable == 0 { (*dev).states_usage[entered_state as usize].above += 1; trace_cpu_idle_miss((*dev).cpu, entered_state, false); break; } i -= 1; }
        } else if diff > (*drv).states[entered_state as usize].exit_latency_ns {
            let mut i = entered_state + 1;
            while i < (*drv).state_count { if (*dev).states_usage[i as usize].disable == 0 { if diff - (*drv).states[entered_state as usize].exit_latency_ns >= (*drv).states[i as usize].target_residency_ns { (*dev).states_usage[entered_state as usize].below += 1; trace_cpu_idle_miss((*dev).cpu, entered_state, true); } break; } i += 1; }
        }
    } else { (*dev).last_residency_ns = 0; (*dev).states_usage[index as usize].rejected += 1; }
    instrumentation_end(); entered_state
}

pub unsafe fn cpuidle_select(drv: *mut cpuidle_driver, dev: *mut cpuidle_device, stop_tick: *mut bool) -> i32 { cpuidle_curr_governor.select(drv, dev, stop_tick) }

pub unsafe fn cpuidle_enter(drv: *mut cpuidle_driver, dev: *mut cpuidle_device, index: i32) -> i32 {
    WRITE_ONCE(&mut (*dev).next_hrtimer, tick_nohz_get_next_hrtimer());
    let ret = if cpuidle_state_is_coupled(drv, index) { cpuidle_enter_state_coupled(dev, drv, index) } else { cpuidle_enter_state(dev, drv, index) };
    WRITE_ONCE(&mut (*dev).next_hrtimer, 0); ret
}

pub unsafe fn cpuidle_reflect(dev: *mut cpuidle_device, index: i32) {
    if cpuidle_curr_governor.reflect.is_some() && index >= 0 { (cpuidle_curr_governor.reflect.unwrap())(dev, index); }
}

pub const CPUIDLE_POLL_MIN: u64 = 10000;
pub const CPUIDLE_POLL_MAX: u64 = TICK_NSEC / 16;

pub unsafe fn cpuidle_poll_time(drv: *mut cpuidle_driver, dev: *mut cpuidle_device) -> u64 {
    if (*dev).poll_limit_ns != 0 { return (*dev).poll_limit_ns; }
    let mut limit_ns = CPUIDLE_POLL_MAX;
    let mut i = 1;
    while i < (*drv).state_count { if (*dev).states_usage[i as usize].disable == 0 { let state_limit = (*drv).states[i as usize].target_residency_ns; if state_limit >= CPUIDLE_POLL_MIN { limit_ns = core::cmp::min(state_limit, CPUIDLE_POLL_MAX); break; } } i += 1; }
    (*dev).poll_limit_ns = limit_ns; limit_ns
}

pub unsafe fn cpuidle_install_idle_handler() { if ENABLED_DEVICES != 0 { smp_wmb(); INITIALIZED = 1; } }
pub unsafe fn cpuidle_uninstall_idle_handler() { if ENABLED_DEVICES != 0 { INITIALIZED = 0; wake_up_all_idle_cpus(); } synchronize_rcu(); }
pub unsafe fn cpuidle_pause_and_lock() { mutex_lock(&mut cpuidle_lock); cpuidle_uninstall_idle_handler(); }
pub unsafe fn cpuidle_resume_and_unlock() { cpuidle_install_idle_handler(); mutex_unlock(&mut cpuidle_lock); }
pub unsafe fn cpuidle_pause() { mutex_lock(&mut cpuidle_lock); cpuidle_uninstall_idle_handler(); mutex_unlock(&mut cpuidle_lock); }
pub unsafe fn cpuidle_resume() { mutex_lock(&mut cpuidle_lock); cpuidle_install_idle_handler(); mutex_unlock(&mut cpuidle_lock); }

pub unsafe fn cpuidle_enable_device(dev: *mut cpuidle_device) -> i32 {
    if dev.is_null() { return -EINVAL; } if (*dev).enabled != 0 { return 0; }
    if cpuidle_curr_governor.is_null() { return -EIO; }
    let drv = cpuidle_get_cpu_driver(dev); if drv.is_null() { return -EIO; }
    if (*dev).registered == 0 { return -EINVAL; }
    let mut ret = cpuidle_add_device_sysfs(dev); if ret != 0 { return ret; }
    if let Some(enable) = (*cpuidle_curr_governor).enable { ret = enable(drv, dev); if ret != 0 { cpuidle_remove_device_sysfs(dev); return ret; } }
    smp_wmb(); (*dev).enabled = 1; ENABLED_DEVICES += 1; 0
}

pub unsafe fn cpuidle_disable_device(dev: *mut cpuidle_device) {
    if dev.is_null() || (*dev).enabled == 0 { return; }
    let drv = cpuidle_get_cpu_driver(dev); if drv.is_null() || cpuidle_curr_governor.is_null() { return; }
    (*dev).enabled = 0; if let Some(disable) = (*cpuidle_curr_governor).disable { disable(drv, dev); }
    cpuidle_remove_device_sysfs(dev); ENABLED_DEVICES -= 1;
}

unsafe fn __cpuidle_unregister_device(dev: *mut cpuidle_device) {
    let drv = cpuidle_get_cpu_driver(dev);
    list_del(&mut (*dev).device_list);
    per_cpu_cpuidle_devices((*dev).cpu, core::ptr::null_mut());
    module_put((*drv).owner);
    (*dev).registered = 0;
}

unsafe fn __cpuidle_device_init(dev: *mut cpuidle_device) {
    memset((*dev).states_usage.as_mut_ptr(), 0, core::mem::size_of_val(&(*dev).states_usage));
    (*dev).last_residency_ns = 0;
    (*dev).next_hrtimer = 0;
}

unsafe fn __cpuidle_register_device(dev: *mut cpuidle_device) -> i32 {
    let drv = cpuidle_get_cpu_driver(dev);
    let cpu = (*dev).cpu;
    if !per_cpu_cpuidle_devices(cpu).is_null() { pr_info("CPU%d: cpuidle device already registered\n", cpu); return -EEXIST; }
    if !try_module_get((*drv).owner) { return -EINVAL; }
    let mut i = 0;
    while i < (*drv).state_count {
        if ((*drv).states[i as usize].flags & CPUIDLE_FLAG_UNUSABLE) != 0 { (*dev).states_usage[i as usize].disable |= CPUIDLE_STATE_DISABLED_BY_DRIVER; }
        if ((*drv).states[i as usize].flags & CPUIDLE_FLAG_OFF) != 0 { (*dev).states_usage[i as usize].disable |= CPUIDLE_STATE_DISABLED_BY_USER; }
        i += 1;
    }
    per_cpu_cpuidle_devices(cpu, dev);
    list_add(&mut (*dev).device_list, &mut cpuidle_detected_devices);
    let ret = cpuidle_coupled_register_device(dev);
    if ret != 0 { __cpuidle_unregister_device(dev); } else { (*dev).registered = 1; }
    ret
}

pub unsafe fn cpuidle_register_device(dev: *mut cpuidle_device) -> i32 {
    if dev.is_null() { return -EINVAL; }
    let mut ret = -EBUSY;
    mutex_lock(&mut cpuidle_lock);
    if (*dev).registered != 0 { mutex_unlock(&mut cpuidle_lock); return ret; }
    __cpuidle_device_init(dev);
    ret = __cpuidle_register_device(dev);
    if ret != 0 { mutex_unlock(&mut cpuidle_lock); return ret; }
    ret = cpuidle_add_sysfs(dev);
    if ret != 0 { __cpuidle_unregister_device(dev); mutex_unlock(&mut cpuidle_lock); return ret; }
    ret = cpuidle_enable_device(dev);
    if ret != 0 { cpuidle_remove_sysfs(dev); __cpuidle_unregister_device(dev); mutex_unlock(&mut cpuidle_lock); return ret; }
    cpuidle_install_idle_handler(); mutex_unlock(&mut cpuidle_lock); ret
}

pub unsafe fn cpuidle_unregister_device_no_lock(dev: *mut cpuidle_device) {
    if dev.is_null() || (*dev).registered == 0 { return; }
    cpuidle_disable_device(dev); cpuidle_remove_sysfs(dev); __cpuidle_unregister_device(dev); cpuidle_coupled_unregister_device(dev);
}

pub unsafe fn cpuidle_unregister_device(dev: *mut cpuidle_device) {
    if dev.is_null() || (*dev).registered == 0 { return; }
    cpuidle_pause_and_lock(); cpuidle_unregister_device_no_lock(dev); cpuidle_resume_and_unlock();
}

pub unsafe fn cpuidle_unregister(drv: *mut cpuidle_driver) {
    for_each_cpu!(_cpu, (*drv).cpumask, {
        let device = per_cpu_cpuidle_dev(_cpu);
        cpuidle_unregister_device(device);
    });
    cpuidle_unregister_driver(drv);
}

pub unsafe fn cpuidle_register(drv: *mut cpuidle_driver, coupled_cpus: *const cpumask) -> i32 {
    let mut ret = cpuidle_register_driver(drv);
    if ret != 0 { pr_err("failed to register cpuidle driver\n"); return ret; }
    for_each_cpu!(_cpu, (*drv).cpumask, {
        let device = per_cpu_cpuidle_dev(_cpu);
        (*device).cpu = _cpu;
        #[cfg(CONFIG_ARCH_NEEDS_CPU_IDLE_COUPLED)]
        if !coupled_cpus.is_null() { (*device).coupled_cpus = *coupled_cpus; }
        ret = cpuidle_register_device(device);
        if ret != 0 { pr_err("Failed to register cpuidle device for cpu%d\n", _cpu); cpuidle_unregister(drv); break; }
    });
    ret
}

pub unsafe fn cpuidle_init() -> i32 { if cpuidle_disabled() != 0 { -ENODEV } else { cpuidle_add_interface() } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
