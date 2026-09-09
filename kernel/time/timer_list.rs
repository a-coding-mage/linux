// SPDX-License-Identifier: GPL-2.0
/*
 * List pending timers
 *
 * Copyright(C) 2006, Red Hat, Inc., Ingo Molnar
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct timer_list_iter {
    pub cpu: core::ffi::c_int,
    pub second_pass: bool,
    pub now: ktime_t,
}

pub unsafe extern "C" fn SEQ_printf(
    _m: *mut seq_file,
    _fmt: *const core::ffi::c_char,
    ...
) {
    // The C implementation forwards the variadic arguments to seq_vprintf or vprintk.
}

unsafe fn print_timer(
    m: *mut seq_file,
    taddr: *mut hrtimer,
    timer: *mut hrtimer,
    idx: core::ffi::c_int,
    now: ktime_t,
) {
    SEQ_printf(m, c" #%d: <%p>, %ps".as_ptr(), idx, taddr, ACCESS_PRIVATE(timer, function));
    SEQ_printf(m, c", S:%02x".as_ptr(), (*timer).is_queued);
    SEQ_printf(m, c"\n".as_ptr());
    SEQ_printf(
        m,
        c" # expires at %lld-%lld nsecs [in %lld to %lld nsecs]\n".as_ptr(),
        hrtimer_get_softexpires(timer) as i64,
        hrtimer_get_expires(timer) as i64,
        ktime_sub(hrtimer_get_softexpires(timer), now) as i64,
        ktime_sub(hrtimer_get_expires(timer), now) as i64,
    );
}

unsafe fn print_active_timers(m: *mut seq_file, base: *mut hrtimer_clock_base, now: ktime_t) {
    let mut curr: *mut timerqueue_linked_node;
    let mut timer: *mut hrtimer;
    let mut tmp: hrtimer;
    let mut next: usize = 0;
    let mut i: usize;
    let mut flags: c_ulong = 0;

    loop {
        i = 0;
        touch_nmi_watchdog();
        raw_spin_lock_irqsave((*(*base).cpu_base).lock, &mut flags);
        curr = timerqueue_linked_first(&mut (*base).active);
        while !curr.is_null() && i < next {
            curr = timerqueue_linked_next(curr);
            i += 1;
        }
        if !curr.is_null() {
            timer = container_of!(curr, hrtimer, node);
            tmp = *timer;
            raw_spin_unlock_irqrestore((*(*base).cpu_base).lock, flags);
            print_timer(m, timer, &mut tmp, i as c_int, now);
            next += 1;
            continue;
        }
        raw_spin_unlock_irqrestore((*(*base).cpu_base).lock, flags);
        break;
    }
}

unsafe fn print_base(m: *mut seq_file, base: *mut hrtimer_clock_base, now: ktime_t) {
    SEQ_printf(m, c"  .base:       %p\n".as_ptr(), base);
    SEQ_printf(m, c"  .index:      %d\n".as_ptr(), (*base).index);
    SEQ_printf(m, c"  .resolution: %u nsecs\n".as_ptr(), hrtimer_resolution);
    // CONFIG_HIGH_RES_TIMERS
    SEQ_printf(m, c"  .offset:     %lld nsecs\n".as_ptr(), (*base).offset as i64);
    SEQ_printf(m, c"active timers:\n".as_ptr());
    print_active_timers(m, base, ktime_add(now, (*base).offset));
}

unsafe fn print_cpu(m: *mut seq_file, cpu: c_int, now: ktime_t) {
    let cpu_base = &mut per_cpu!(hrtimer_bases, cpu);
    SEQ_printf(m, c"cpu: %d\n".as_ptr(), cpu);
    for i in 0..HRTIMER_MAX_CLOCK_BASES {
        SEQ_printf(m, c" clock %d:\n".as_ptr(), i);
        print_base(m, (*cpu_base).clock_base.add(i as usize), now);
    }
    // CONFIG_HIGH_RES_TIMERS and CONFIG_TICK_ONESHOT diagnostic fields are
    // intentionally retained as conditional dependency points from the C source.
    SEQ_printf(m, c"\n".as_ptr());
}

// CONFIG_GENERIC_CLOCKEVENTS
unsafe fn print_tickdevice(m: *mut seq_file, td: *mut tick_device, cpu: c_int) {
    let dev = (*td).evtdev;
    touch_nmi_watchdog();
    SEQ_printf(m, c"Tick Device: mode:     %d\n".as_ptr(), (*td).mode);
    if cpu < 0 { SEQ_printf(m, c"Broadcast device\n".as_ptr()); }
    else { SEQ_printf(m, c"Per CPU device: %d\n".as_ptr(), cpu); }
    SEQ_printf(m, c"Clock Event Device: ".as_ptr());
    if dev.is_null() { SEQ_printf(m, c"<NULL>\n".as_ptr()); return; }
    SEQ_printf(m, c"%s\n".as_ptr(), (*dev).name);
    SEQ_printf(m, c" max_delta_ns:   %llu\n".as_ptr(), (*dev).max_delta_ns);
    SEQ_printf(m, c" min_delta_ns:   %llu\n".as_ptr(), (*dev).min_delta_ns);
    SEQ_printf(m, c" mult:           %u\n".as_ptr(), (*dev).mult);
    SEQ_printf(m, c" shift:          %u\n".as_ptr(), (*dev).shift);
    SEQ_printf(m, c" mode:           %d\n".as_ptr(), clockevent_get_state(dev));
    SEQ_printf(m, c" next_event:     %lld nsecs\n".as_ptr(), (*dev).next_event as i64);
    SEQ_printf(m, c" set_next_event: %ps\n".as_ptr(), (*dev).set_next_event);
    if !(*dev).set_state_shutdown.is_none() { SEQ_printf(m, c" shutdown:       %ps\n".as_ptr(), (*dev).set_state_shutdown); }
    if !(*dev).set_state_periodic.is_none() { SEQ_printf(m, c" periodic:       %ps\n".as_ptr(), (*dev).set_state_periodic); }
    if !(*dev).set_state_oneshot.is_none() { SEQ_printf(m, c" oneshot:        %ps\n".as_ptr(), (*dev).set_state_oneshot); }
    if !(*dev).set_state_oneshot_stopped.is_none() { SEQ_printf(m, c" oneshot stopped: %ps\n".as_ptr(), (*dev).set_state_oneshot_stopped); }
    if !(*dev).tick_resume.is_none() { SEQ_printf(m, c" resume:         %ps\n".as_ptr(), (*dev).tick_resume); }
    SEQ_printf(m, c" event_handler:  %ps\n".as_ptr(), (*dev).event_handler);
    SEQ_printf(m, c"\n retries:        %lu\n\n".as_ptr(), (*dev).retries);
}

unsafe fn timer_list_header(m: *mut seq_file, now: ktime_t) {
    SEQ_printf(m, c"Timer List Version: v0.11\n".as_ptr());
    SEQ_printf(m, c"HRTIMER_MAX_CLOCK_BASES: %d\n".as_ptr(), HRTIMER_MAX_CLOCK_BASES);
    SEQ_printf(m, c"now at %lld nsecs\n\n".as_ptr(), now as i64);
}

pub unsafe fn sysrq_timer_list_show() {
    let now = ktime_get();
    timer_list_header(core::ptr::null_mut(), now);
    for_each_online_cpu!(cpu, { print_cpu(core::ptr::null_mut(), cpu, now); });
}

#[cfg(feature = "proc_fs")]
unsafe fn timer_list_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> c_int {
    let iter = v as *mut timer_list_iter;
    if (*iter).cpu == -1 && !(*iter).second_pass {
        timer_list_header(m, (*iter).now);
    } else if !(*iter).second_pass {
        print_cpu(m, (*iter).cpu, (*iter).now);
    } else if (*iter).cpu == -1 {
        // CONFIG_GENERIC_CLOCKEVENTS: timer_list_show_tickdevices_header(m)
    } else {
        print_tickdevice(m, tick_get_device((*iter).cpu), (*iter).cpu);
    }
    0
}

#[cfg(feature = "proc_fs")]
unsafe fn move_iter(iter: *mut timer_list_iter, mut offset: loff_t) -> *mut core::ffi::c_void {
    while offset != 0 {
        (*iter).cpu = cpumask_next((*iter).cpu, cpu_online_mask);
        if (*iter).cpu >= nr_cpu_ids {
            if !(*iter).second_pass {
                (*iter).cpu = -1;
                (*iter).second_pass = true;
            } else {
                return core::ptr::null_mut();
            }
        }
        offset -= 1;
    }
    iter.cast()
}

#[cfg(feature = "proc_fs")]
unsafe fn timer_list_start(file: *mut seq_file, offset: *mut loff_t) -> *mut core::ffi::c_void {
    let iter = (*file).private as *mut timer_list_iter;
    if *offset == 0 { (*iter).now = ktime_get(); }
    (*iter).cpu = -1;
    (*iter).second_pass = false;
    move_iter(iter, *offset)
}

#[cfg(feature = "proc_fs")]
unsafe fn timer_list_next(file: *mut seq_file, _v: *mut core::ffi::c_void, offset: *mut loff_t) -> *mut core::ffi::c_void {
    let iter = (*file).private as *mut timer_list_iter;
    *offset += 1;
    move_iter(iter, 1)
}

#[cfg(feature = "proc_fs")]
unsafe fn timer_list_stop(_seq: *mut seq_file, _v: *mut core::ffi::c_void) {}

#[repr(C)]
struct seq_operations {
    start: Option<unsafe fn(*mut seq_file, *mut loff_t) -> *mut core::ffi::c_void>,
    next: Option<unsafe fn(*mut seq_file, *mut core::ffi::c_void, *mut loff_t) -> *mut core::ffi::c_void>,
    stop: Option<unsafe fn(*mut seq_file, *mut core::ffi::c_void)>,
    show: Option<unsafe fn(*mut seq_file, *mut core::ffi::c_void) -> c_int>,
}

#[cfg(feature = "proc_fs")]
static timer_list_sops: seq_operations = seq_operations {
    start: Some(timer_list_start),
    next: Some(timer_list_next),
    stop: Some(timer_list_stop),
    show: Some(timer_list_show),
};

#[cfg(feature = "proc_fs")]
unsafe fn init_timer_list_procfs() -> c_int {
    let pe = proc_create_seq_private(
        c"timer_list".as_ptr(), 0o400, core::ptr::null_mut(), &timer_list_sops,
        core::mem::size_of::<timer_list_iter>(), core::ptr::null_mut(),
    );
    if pe.is_null() { return -12; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
