// SPDX-License-Identifier: GPL-2.0
// Clock event device management.

// External kernel types, constants, macros, globals, and functions are supplied by other files.

static mut clockevent_devices: list_head = LIST_HEAD_INIT(clockevent_devices);
static mut clockevents_released: list_head = LIST_HEAD_INIT(clockevents_released);
static mut clockevents_lock: raw_spinlock_t = __RAW_SPIN_LOCK_UNLOCKED(clockevents_lock);
static mut clockevents_mutex: mutex = __MUTEX_INITIALIZER(clockevents_mutex);

#[repr(C)]
struct ce_unbind {
    ce: *mut clock_event_device,
    res: c_int,
}

unsafe fn cev_delta2ns(latch: c_ulong, evt: *mut clock_event_device, ismax: bool) -> u64 {
    let mut clc = (latch as u64) << (*evt).shift;
    let mut rnd: u64;
    if WARN_ON((*evt).mult == 0) { (*evt).mult = 1; }
    rnd = (*evt).mult as u64 - 1;
    if (clc >> (*evt).shift) != latch as u64 { clc = !0u64; }
    if (!0u64 - clc > rnd) && (!ismax || (*evt).mult <= (1u64 << (*evt).shift)) { clc += rnd; }
    clc = do_div_u64(clc, (*evt).mult as u64);
    if clc > 1000 { clc } else { 1000 }
}

pub unsafe fn clockevent_delta2ns(latch: c_ulong, evt: *mut clock_event_device) -> u64 {
    cev_delta2ns(latch, evt, false)
}

unsafe fn __clockevents_switch_state(dev: *mut clock_event_device, state: clock_event_state) -> c_int {
    if (*dev).features & CLOCK_EVT_FEAT_DUMMY != 0 { return 0; }
    (*dev).next_event_forced = 0;
    match state {
        CLOCK_EVT_STATE_DETACHED | CLOCK_EVT_STATE_SHUTDOWN => {
            if let Some(f) = (*dev).set_state_shutdown { f(dev) } else { 0 }
        }
        CLOCK_EVT_STATE_PERIODIC => {
            if (*dev).features & CLOCK_EVT_FEAT_PERIODIC == 0 { return -ENOSYS; }
            if let Some(f) = (*dev).set_state_periodic { f(dev) } else { 0 }
        }
        CLOCK_EVT_STATE_ONESHOT => {
            if (*dev).features & CLOCK_EVT_FEAT_ONESHOT == 0 { return -ENOSYS; }
            if let Some(f) = (*dev).set_state_oneshot { f(dev) } else { 0 }
        }
        CLOCK_EVT_STATE_ONESHOT_STOPPED => {
            if WARN_ONCE(!clockevent_state_oneshot(dev), "Current state: %d\n", clockevent_get_state(dev)) { return -EINVAL; }
            if let Some(f) = (*dev).set_state_oneshot_stopped { f(dev) } else { -ENOSYS }
        }
        _ => -ENOSYS,
    }
}

pub unsafe fn clockevents_switch_state(dev: *mut clock_event_device, state: clock_event_state) {
    if clockevent_get_state(dev) != state && __clockevents_switch_state(dev, state) == 0 {
        clockevent_set_state(dev, state);
        if clockevent_state_oneshot(dev) && WARN_ON((*dev).mult == 0) { (*dev).mult = 1; }
    }
}

pub unsafe fn clockevents_shutdown(dev: *mut clock_event_device) {
    clockevents_switch_state(dev, CLOCK_EVT_STATE_SHUTDOWN);
    (*dev).next_event = KTIME_MAX;
    (*dev).next_event_forced = 0;
}

pub unsafe fn clockevents_tick_resume(dev: *mut clock_event_device) -> c_int {
    if let Some(f) = (*dev).tick_resume { f(dev) } else { 0 }
}

#[cfg(CONFIG_GENERIC_CLOCKEVENTS_MIN_ADJUST)]
const MIN_DELTA_LIMIT: u64 = NSEC_PER_SEC / HZ;

#[cfg(CONFIG_GENERIC_CLOCKEVENTS_MIN_ADJUST)]
unsafe fn clockevents_increase_min_delta(dev: *mut clock_event_device) -> c_int {
    if (*dev).min_delta_ns >= MIN_DELTA_LIMIT { printk_deferred!(KERN_WARNING, "CE: Reprogramming failure. Giving up\n"); (*dev).next_event = KTIME_MAX; return -ETIME; }
    if (*dev).min_delta_ns < 5000 { (*dev).min_delta_ns = 5000; } else { (*dev).min_delta_ns += (*dev).min_delta_ns >> 1; }
    if (*dev).min_delta_ns > MIN_DELTA_LIMIT { (*dev).min_delta_ns = MIN_DELTA_LIMIT; }
    printk_deferred!(KERN_WARNING, "CE: %s increased min_delta_ns to %llu nsec\n", if !(*dev).name.is_null() { (*dev).name } else { b"?\0".as_ptr() as *const c_char }, (*dev).min_delta_ns);
    0
}

unsafe fn clockevents_program_min_delta(dev: *mut clock_event_device) -> c_int {
    #[cfg(CONFIG_GENERIC_CLOCKEVENTS_MIN_ADJUST)]
    { let mut i = 0; loop { let delta = (*dev).min_delta_ns as c_longlong; (*dev).next_event = ktime_add_ns(ktime_get(), delta); if clockevent_state_shutdown(dev) { return 0; } (*dev).retries += 1; let clc = ((delta as u64 * (*dev).mult as u64) >> (*dev).shift) as c_ulong; if ((*dev).set_next_event.unwrap())(clc, dev) == 0 { return 0; } i += 1; if i > 2 { if clockevents_increase_min_delta(dev) != 0 { return -ETIME; } i = 0; } } }
    #[cfg(not(CONFIG_GENERIC_CLOCKEVENTS_MIN_ADJUST))]
    { let mut delta: c_longlong = 0; for _ in 0..10 { delta += (*dev).min_delta_ns as c_longlong; (*dev).next_event = ktime_add_ns(ktime_get(), delta); if clockevent_state_shutdown(dev) { return 0; } (*dev).retries += 1; let clc = ((delta as u64 * (*dev).mult as u64) >> (*dev).shift) as c_ulong; if ((*dev).set_next_event.unwrap())(clc, dev) == 0 { return 0; } } -ETIME }
}

#[cfg(CONFIG_GENERIC_CLOCKEVENTS_COUPLED)]
unsafe fn clockevent_set_next_coupled(dev: *mut clock_event_device, expires: ktime_t) -> bool { let mut cycles = 0; if (*dev).features & CLOCK_EVT_FEAT_CLOCKSOURCE_COUPLED == 0 || !ktime_expiry_to_cycles((*dev).cs_id, expires, &mut cycles) { return false; } (*dev).set_next_coupled.unwrap()(cycles, dev); true }
#[cfg(not(CONFIG_GENERIC_CLOCKEVENTS_COUPLED))]
unsafe fn clockevent_set_next_coupled(_: *mut clock_event_device, _: ktime_t) -> bool { false }

pub unsafe fn clockevents_program_event(dev: *mut clock_event_device, expires: ktime_t, force: bool) -> c_int {
    if WARN_ON_ONCE(expires < 0) { return -ETIME; }
    (*dev).next_event = expires;
    if clockevent_state_shutdown(dev) { return 0; }
    WARN_ONCE(!clockevent_state_oneshot(dev), "Current state: %d\n", clockevent_get_state(dev));
    if (*dev).features & CLOCK_EVT_FEAT_HRTIMER != 0 { return (*dev).set_next_ktime.unwrap()(expires, dev); }
    if clockevent_set_next_coupled(dev, expires) { return 0; }
    let mut delta = ktime_to_ns(ktime_sub(expires, ktime_get()));
    if delta <= 0 && !force { return -ETIME; }
    if delta > (*dev).min_delta_ns as c_longlong { delta = core::cmp::min(delta, (*dev).max_delta_ns as c_longlong); let cycles = ((delta as u64 * (*dev).mult as u64) >> (*dev).shift) as c_ulong; if (*dev).set_next_event.unwrap()(cycles, dev) == 0 { (*dev).next_event_forced = 0; return 0; } }
    if (*dev).next_event_forced != 0 { return 0; }
    if (*dev).set_next_event.unwrap()((*dev).min_delta_ticks, dev) != 0 { if !force || clockevents_program_min_delta(dev) != 0 { return -ETIME; } }
    (*dev).next_event_forced = 1; 0
}

unsafe fn clockevents_notify_released() { while !list_empty(&mut clockevents_released) { let dev = list_entry(clockevents_released.next, clock_event_device, list); list_move(&mut (*dev).list, &mut clockevent_devices); tick_check_new_device(dev); } }

unsafe fn clockevents_replace(ced: *mut clock_event_device) -> c_int { let mut newdev = core::ptr::null_mut(); list_for_each_entry!(dev, &clockevent_devices, list, { if dev != ced && clockevent_state_detached(dev) && tick_check_replacement(newdev, dev) && try_module_get((*dev).owner) { if !newdev.is_null() { module_put((*newdev).owner); } newdev = dev; }); if !newdev.is_null() { tick_install_replacement(newdev); list_del_init(&mut (*ced).list); 0 } else { -EBUSY } }

unsafe fn __clockevents_try_unbind(ced: *mut clock_event_device, cpu: c_int) -> c_int { if clockevent_state_detached(ced) { list_del_init(&mut (*ced).list); 0 } else if ced == per_cpu!(tick_cpu_device, cpu).evtdev { -EAGAIN } else { -EBUSY } }
unsafe extern "C" fn __clockevents_unbind(arg: *mut c_void) { let cu = arg as *mut ce_unbind; raw_spin_lock(&mut clockevents_lock); (*cu).res = __clockevents_try_unbind((*cu).ce, smp_processor_id()); if (*cu).res == -EAGAIN { (*cu).res = clockevents_replace((*cu).ce); } raw_spin_unlock(&mut clockevents_lock); }
unsafe fn clockevents_unbind(ced: *mut clock_event_device, cpu: c_int) -> c_int { let mut cu = ce_unbind { ce: ced, res: -ENODEV }; smp_call_function_single(cpu, Some(__clockevents_unbind), &mut cu as *mut _ as *mut c_void, 1); cu.res }
pub unsafe fn clockevents_unbind_device(ced: *mut clock_event_device, cpu: c_int) -> c_int { mutex_lock(&mut clockevents_mutex); let ret = clockevents_unbind(ced, cpu); mutex_unlock(&mut clockevents_mutex); ret }

pub unsafe fn clockevents_register_device(dev: *mut clock_event_device) { let mut flags = 0; clockevent_set_state(dev, CLOCK_EVT_STATE_DETACHED); if (*dev).cpumask.is_null() { WARN_ON(num_possible_cpus() > 1); (*dev).cpumask = cpumask_of(smp_processor_id()); } if (*dev).cpumask == cpu_all_mask { WARN!(true, "%s cpumask == cpu_all_mask, using cpu_possible_mask instead\n", (*dev).name); (*dev).cpumask = cpu_possible_mask; } raw_spin_lock_irqsave(&mut clockevents_lock, &mut flags); list_add(&mut (*dev).list, &mut clockevent_devices); tick_check_new_device(dev); clockevents_notify_released(); raw_spin_unlock_irqrestore(&mut clockevents_lock, flags); }
unsafe fn clockevents_config(dev: *mut clock_event_device, freq: u32) { if (*dev).features & CLOCK_EVT_FEAT_ONESHOT == 0 { return; } let mut sec = (*dev).max_delta_ticks as u64 / freq as u64; if sec == 0 { sec = 1; } else if sec > 600 && (*dev).max_delta_ticks > UINT_MAX as _ { sec = 600; } clockevents_calc_mult_shift(dev, freq, sec); (*dev).min_delta_ns = cev_delta2ns((*dev).min_delta_ticks, dev, false); (*dev).max_delta_ns = cev_delta2ns((*dev).max_delta_ticks, dev, true); }
pub unsafe fn clockevents_config_and_register(dev: *mut clock_event_device, freq: u32, min_delta: c_ulong, max_delta: c_ulong) { (*dev).min_delta_ticks = min_delta; (*dev).max_delta_ticks = max_delta; clockevents_config(dev, freq); clockevents_register_device(dev); }
pub unsafe fn __clockevents_update_freq(dev: *mut clock_event_device, freq: u32) -> c_int { clockevents_config(dev, freq); if clockevent_state_oneshot(dev) { return clockevents_program_event(dev, (*dev).next_event, false); } if clockevent_state_periodic(dev) { return __clockevents_switch_state(dev, CLOCK_EVT_STATE_PERIODIC); } 0 }
pub unsafe fn clockevents_update_freq(dev: *mut clock_event_device, freq: u32) -> c_int { let mut flags = 0; local_irq_save(&mut flags); let mut ret = tick_broadcast_update_freq(dev, freq); if ret == -ENODEV { ret = __clockevents_update_freq(dev, freq); } local_irq_restore(flags); ret }
pub unsafe fn clockevents_handle_noop(_: *mut clock_event_device) {}
pub unsafe fn clockevents_exchange_device(old: *mut clock_event_device, new: *mut clock_event_device) { if !old.is_null() { module_put((*old).owner); clockevents_switch_state(old, CLOCK_EVT_STATE_DETACHED); list_move(&mut (*old).list, &mut clockevents_released); } if !new.is_null() { BUG_ON(!clockevent_state_detached(new)); clockevents_shutdown(new); } }
pub unsafe fn clockevents_suspend() { list_for_each_entry_reverse!(dev, &clockevent_devices, list, { if let Some(f) = (*dev).suspend { if !clockevent_state_detached(dev) { f(dev); } } }); }
pub unsafe fn clockevents_resume() { list_for_each_entry!(dev, &clockevent_devices, list, { if let Some(f) = (*dev).resume { if !clockevent_state_detached(dev) { f(dev); } } }); }

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe fn tick_offline_cpu(cpu: c_uint) { raw_spin_lock(&mut clockevents_lock); tick_broadcast_offline(cpu); tick_shutdown(); list_for_each_entry_safe!(dev, tmp, &clockevents_released, list, { list_del(&mut (*dev).list); }); list_for_each_entry_safe!(dev, tmp, &clockevent_devices, list, { if cpumask_test_cpu(cpu, (*dev).cpumask) && cpumask_weight((*dev).cpumask) == 1 && !tick_is_broadcast_device(dev) { BUG_ON(!clockevent_state_detached(dev)); list_del(&mut (*dev).list); } }); raw_spin_unlock(&mut clockevents_lock); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
