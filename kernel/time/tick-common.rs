// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of tick-common.c. Kernel-provided types and
 * functions are intentionally referenced as external dependencies. */

// C includes and configuration-dependent declarations are supplied by the
// surrounding kernel translation unit.

pub static mut tick_cpu_device: [tick_device; 0] = [];
pub static mut tick_next_period: ktime_t = 0;
pub static mut tick_do_timer_cpu: i32 = TICK_DO_TIMER_BOOT;

#[cfg(CONFIG_NO_HZ_FULL)]
static mut tick_do_timer_boot_cpu: i32 = -1;

pub unsafe fn tick_get_device(cpu: i32) -> *mut tick_device {
    &mut tick_cpu_device[cpu as usize] as *mut tick_device
}

pub unsafe fn tick_is_oneshot_available() -> i32 {
    let dev = __this_cpu_read_tick_device_evtdev();
    if dev.is_null() || ((*dev).features & CLOCK_EVT_FEAT_ONESHOT) == 0 { return 0; }
    if ((*dev).features & CLOCK_EVT_FEAT_C3STOP) == 0 { return 1; }
    tick_broadcast_oneshot_available()
}

unsafe fn tick_periodic(cpu: i32) {
    if READ_ONCE(&tick_do_timer_cpu) == cpu {
        raw_spin_lock(&mut jiffies_lock);
        write_seqcount_begin(&mut jiffies_seq);
        tick_next_period = ktime_add_ns(tick_next_period, TICK_NSEC);
        do_timer(1);
        write_seqcount_end(&mut jiffies_seq);
        raw_spin_unlock(&mut jiffies_lock);
        update_wall_time();
    }
    update_process_times(user_mode(get_irq_regs()));
    profile_tick(CPU_PROFILING);
}

pub unsafe fn tick_handle_periodic(dev: *mut clock_event_device) {
    let cpu = smp_processor_id();
    let mut next = (*dev).next_event;
    (*dev).next_event_forced = 0;
    tick_periodic(cpu);
    if IS_ENABLED_CONFIG_TICK_ONESHOT && (*dev).event_handler != Some(tick_handle_periodic) { return; }
    if !clockevent_state_oneshot(dev) { return; }
    loop {
        next = ktime_add_ns(next, TICK_NSEC);
        if !clockevents_program_event(dev, next, false) { return; }
        if timekeeping_valid_for_hres() { tick_periodic(cpu); }
    }
}

pub unsafe fn tick_setup_periodic(dev: *mut clock_event_device, broadcast: i32) {
    tick_set_periodic_handler(dev, broadcast);
    if !tick_device_is_functional(dev) { return; }
    if ((*dev).features & CLOCK_EVT_FEAT_PERIODIC) != 0 && !tick_broadcast_oneshot_active() {
        clockevents_switch_state(dev, CLOCK_EVT_STATE_PERIODIC);
    } else {
        let mut seq: u32;
        let next: ktime_t;
        loop {
            seq = read_seqcount_begin(&jiffies_seq);
            next = tick_next_period;
            if !read_seqcount_retry(&jiffies_seq, seq) { break; }
        }
        clockevents_switch_state(dev, CLOCK_EVT_STATE_ONESHOT);
        loop { if !clockevents_program_event(dev, next, false) { return; } }
    }
}

unsafe fn tick_setup_device(td: *mut tick_device, newdev: *mut clock_event_device, cpu: i32, cpumask: *const cpumask) {
    let mut handler: Option<unsafe extern "C" fn(*mut clock_event_device)> = None;
    let mut next_event: ktime_t = 0;
    if (*td).evtdev.is_null() {
        if READ_ONCE(&tick_do_timer_cpu) == TICK_DO_TIMER_BOOT {
            WRITE_ONCE(&mut tick_do_timer_cpu, cpu);
            tick_next_period = ktime_get();
            #[cfg(CONFIG_NO_HZ_FULL)]
            { if tick_nohz_full_cpu(cpu) { tick_do_timer_boot_cpu = cpu; } }
        }
        (*td).mode = TICKDEV_MODE_PERIODIC;
    } else {
        handler = (*(*td).evtdev).event_handler;
        next_event = (*(*td).evtdev).next_event;
        (*(*td).evtdev).event_handler = Some(clockevents_handle_noop);
    }
    (*td).evtdev = newdev;
    if !cpumask_equal((*newdev).cpumask, cpumask) { irq_set_affinity((*newdev).irq, cpumask); }
    if tick_device_uses_broadcast(newdev, cpu) { return; }
    if (*td).mode == TICKDEV_MODE_PERIODIC { tick_setup_periodic(newdev, 0); }
    else { tick_setup_oneshot(newdev, handler, next_event); }
}

pub unsafe fn tick_install_replacement(newdev: *mut clock_event_device) {
    let td = this_cpu_ptr(&mut tick_cpu_device);
    let cpu = smp_processor_id();
    clockevents_exchange_device((*td).evtdev, newdev);
    tick_setup_device(td, newdev, cpu, cpumask_of(cpu));
    if ((*newdev).features & CLOCK_EVT_FEAT_ONESHOT) != 0 { tick_oneshot_notify(); }
}

unsafe fn tick_check_percpu(curdev: *mut clock_event_device, newdev: *mut clock_event_device, cpu: i32) -> bool {
    if !cpumask_test_cpu(cpu, (*newdev).cpumask) { return false; }
    if cpumask_equal((*newdev).cpumask, cpumask_of(cpu)) { return true; }
    if (*newdev).irq >= 0 && !irq_can_set_affinity((*newdev).irq) { return false; }
    if !curdev.is_null() && cpumask_equal((*curdev).cpumask, cpumask_of(cpu)) { return false; }
    true
}

unsafe fn tick_check_preferred(curdev: *mut clock_event_device, newdev: *mut clock_event_device) -> bool {
    if ((*newdev).features & CLOCK_EVT_FEAT_ONESHOT) == 0 {
        if !curdev.is_null() && ((*curdev).features & CLOCK_EVT_FEAT_ONESHOT) != 0 { return false; }
        if tick_oneshot_mode_active() { return false; }
    }
    curdev.is_null() || (*newdev).rating > (*curdev).rating || !cpumask_equal((*curdev).cpumask, (*newdev).cpumask)
}

pub unsafe fn tick_check_replacement(curdev: *mut clock_event_device, newdev: *mut clock_event_device) -> bool {
    tick_check_percpu(curdev, newdev, smp_processor_id()) && tick_check_preferred(curdev, newdev)
}

pub unsafe fn tick_check_new_device(newdev: *mut clock_event_device) {
    let cpu = smp_processor_id();
    let td = &mut tick_cpu_device[cpu as usize] as *mut tick_device;
    let mut curdev = (*td).evtdev;
    if !tick_check_replacement(curdev, newdev) { tick_install_broadcast_device(newdev, cpu); return; }
    if !try_module_get((*newdev).owner) { return; }
    if tick_is_broadcast_device(curdev) { clockevents_shutdown(curdev); curdev = core::ptr::null_mut(); }
    clockevents_exchange_device(curdev, newdev);
    tick_setup_device(td, newdev, cpu, cpumask_of(cpu));
    if ((*newdev).features & CLOCK_EVT_FEAT_ONESHOT) != 0 { tick_oneshot_notify(); }
}

pub unsafe fn tick_broadcast_oneshot_control(state: tick_broadcast_state) -> i32 {
    let td = this_cpu_ptr(&mut tick_cpu_device);
    if ((*td).evtdev).is_null() || ((*(*td).evtdev).features & CLOCK_EVT_FEAT_C3STOP) == 0 { return 0; }
    __tick_broadcast_oneshot_control(state)
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe fn tick_assert_timekeeping_handover() { WARN_ON_ONCE(tick_do_timer_cpu == smp_processor_id()); }
#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe fn tick_cpu_dying(dying_cpu: u32) -> i32 {
    if tick_do_timer_cpu == dying_cpu as i32 { tick_do_timer_cpu = cpumask_first(cpu_online_mask); }
    tick_sched_timer_dying(dying_cpu); tick_offline_cpu(dying_cpu); 0
}
#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe fn tick_shutdown() {
    let td = this_cpu_ptr(&mut tick_cpu_device); let dev = (*td).evtdev;
    (*td).mode = TICKDEV_MODE_PERIODIC;
    if !dev.is_null() { clockevents_exchange_device(dev, core::ptr::null_mut()); (*dev).event_handler = Some(clockevents_handle_noop); (*td).evtdev = core::ptr::null_mut(); }
}

pub unsafe fn tick_suspend_local() { let td = this_cpu_ptr(&mut tick_cpu_device); clockevents_shutdown((*td).evtdev); }
pub unsafe fn tick_resume_local() {
    let td = this_cpu_ptr(&mut tick_cpu_device); let broadcast = tick_resume_check_broadcast();
    clockevents_tick_resume((*td).evtdev);
    if !broadcast { if (*td).mode == TICKDEV_MODE_PERIODIC { tick_setup_periodic((*td).evtdev, 0); } else { tick_resume_oneshot(); } }
    hrtimers_resume_local();
}
pub unsafe fn tick_suspend() { tick_suspend_local(); tick_suspend_broadcast(); }
pub unsafe fn tick_resume() { tick_resume_broadcast(); tick_resume_local(); }

#[cfg(CONFIG_SUSPEND)]
static mut tick_freeze_depth: u32 = 0;
#[cfg(CONFIG_SUSPEND)]
pub unsafe fn tick_freeze() {
    raw_spin_lock(&mut tick_freeze_lock); tick_freeze_depth += 1;
    if tick_freeze_depth == num_online_cpus() { trace_suspend_resume(TPS("timekeeping_freeze"), smp_processor_id(), true); lock_map_acquire_try(&tick_freeze_map); system_state = SYSTEM_SUSPEND; sched_clock_suspend(); timekeeping_suspend(); lock_map_release(&tick_freeze_map); }
    else { tick_suspend_local(); }
    raw_spin_unlock(&mut tick_freeze_lock);
}
#[cfg(CONFIG_SUSPEND)]
pub unsafe fn tick_unfreeze() {
    raw_spin_lock(&mut tick_freeze_lock);
    if tick_freeze_depth == num_online_cpus() { lock_map_acquire_try(&tick_freeze_map); timekeeping_resume(); sched_clock_resume(); lock_map_release(&tick_freeze_map); system_state = SYSTEM_RUNNING; trace_suspend_resume(TPS("timekeeping_freeze"), smp_processor_id(), false); }
    else { touch_softlockup_watchdog(); tick_resume_local(); }
    tick_freeze_depth -= 1; raw_spin_unlock(&mut tick_freeze_lock);
}

pub unsafe fn tick_init() { tick_broadcast_init(); tick_nohz_init(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
