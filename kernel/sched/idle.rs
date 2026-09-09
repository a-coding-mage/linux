// SPDX-License-Identifier: GPL-2.0-only
/*
 * Generic entry points for the idle threads and
 * implementation of the idle task scheduling class.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Symbols supplied by the kernel headers and other translation units.
extern "C" {
    static __cpuidle_text_start: c_char;
    static __cpuidle_text_end: c_char;
}

static mut CPU_IDLE_FORCE_POLL: c_int = 0;

pub unsafe fn sched_idle_set_state(idle_state: *mut cpuidle_state) {
    idle_set_state(this_rq(), idle_state);
}

pub unsafe fn cpu_idle_poll_ctrl(enable: bool) {
    if enable {
        CPU_IDLE_FORCE_POLL += 1;
    } else {
        CPU_IDLE_FORCE_POLL -= 1;
        WARN_ON_ONCE(CPU_IDLE_FORCE_POLL < 0);
    }
}

// CONFIG_GENERIC_IDLE_POLL_SETUP conditionally supplies the __setup handlers.
unsafe fn cpu_idle_poll_setup(_unused: *mut c_char) -> c_int {
    CPU_IDLE_FORCE_POLL = 1;
    1
}

unsafe fn cpu_idle_nopoll_setup(_unused: *mut c_char) -> c_int {
    CPU_IDLE_FORCE_POLL = 0;
    1
}

unsafe fn cpu_idle_poll() -> c_int {
    instrumentation_begin();
    trace_cpu_idle(0, smp_processor_id());
    stop_critical_timings();
    ct_cpuidle_enter();

    raw_local_irq_enable();
    while !tif_need_resched() &&
        (CPU_IDLE_FORCE_POLL != 0 || tick_check_broadcast_expired())
    {
        cpu_relax();
    }
    raw_local_irq_disable();

    ct_cpuidle_exit();
    start_critical_timings();
    trace_cpu_idle(PWR_EVENT_EXIT, smp_processor_id());
    local_irq_enable();
    instrumentation_end();
    1
}

pub unsafe fn arch_cpu_idle_prepare() {}
pub unsafe fn arch_cpu_idle_enter() {}
pub unsafe fn arch_cpu_idle_exit() {}
pub unsafe fn arch_cpu_idle_dead() -> ! { loop {} }
pub unsafe fn arch_cpu_idle() { CPU_IDLE_FORCE_POLL = 1; }

unsafe fn cond_tick_broadcast_enter() {
    // CONFIG_GENERIC_CLOCKEVENTS_BROADCAST_IDLE guarded static-key check.
    if static_branch_unlikely(&arch_needs_tick_broadcast) { tick_broadcast_enter(); }
}

unsafe fn cond_tick_broadcast_exit() {
    if static_branch_unlikely(&arch_needs_tick_broadcast) { tick_broadcast_exit(); }
}

unsafe fn default_idle_call() {
    instrumentation_begin();
    if !current_clr_polling_and_test() {
        cond_tick_broadcast_enter();
        trace_cpu_idle(1, smp_processor_id());
        stop_critical_timings();
        ct_cpuidle_enter();
        arch_cpu_idle();
        ct_cpuidle_exit();
        start_critical_timings();
        trace_cpu_idle(PWR_EVENT_EXIT, smp_processor_id());
        cond_tick_broadcast_exit();
    }
    local_irq_enable();
    instrumentation_end();
}

unsafe fn call_cpuidle_s2idle(drv: *mut cpuidle_driver, dev: *mut cpuidle_device,
                              max_latency_ns: u64) -> c_int {
    if current_clr_polling_and_test() { return -EBUSY; }
    cpuidle_enter_s2idle(drv, dev, max_latency_ns)
}

unsafe fn call_cpuidle(drv: *mut cpuidle_driver, dev: *mut cpuidle_device,
                       next_state: c_int) -> c_int {
    if current_clr_polling_and_test() {
        (*dev).last_residency_ns = 0;
        local_irq_enable();
        return -EBUSY;
    }
    cpuidle_enter(drv, dev, next_state)
}

unsafe fn idle_call_stop_or_retain_tick(stop_tick: bool) {
    if stop_tick || tick_nohz_tick_stopped() { tick_nohz_idle_stop_tick(); }
    else { tick_nohz_idle_retain_tick(); }
}

unsafe fn cpuidle_idle_call(mut stop_tick: bool) {
    let dev = cpuidle_get_device();
    let drv = cpuidle_get_cpu_driver(dev);
    let mut next_state: c_int;
    let mut entered_state: c_int;

    if need_resched() { local_irq_enable(); return; }
    if cpuidle_not_available(drv, dev) {
        idle_call_stop_or_retain_tick(stop_tick);
        default_idle_call();
        __current_set_polling();
        if WARN_ON_ONCE(irqs_disabled()) { local_irq_enable(); }
        return;
    }

    if idle_should_enter_s2idle() || (*dev).forced_idle_latency_limit_ns != 0 {
        let max_latency_ns: u64;
        if idle_should_enter_s2idle() {
            max_latency_ns = cpu_wakeup_latency_qos_limit() * NSEC_PER_USEC;
            entered_state = call_cpuidle_s2idle(drv, dev, max_latency_ns);
            if entered_state > 0 { __current_set_polling(); return; }
        } else { max_latency_ns = (*dev).forced_idle_latency_limit_ns; }
        tick_nohz_idle_stop_tick();
        next_state = cpuidle_find_deepest_state(drv, dev, max_latency_ns);
        call_cpuidle(drv, dev, next_state);
    } else if (*drv).state_count > 1 {
        stop_tick = true;
        next_state = cpuidle_select(drv, dev, &mut stop_tick);
        idle_call_stop_or_retain_tick(stop_tick);
        entered_state = call_cpuidle(drv, dev, next_state);
        cpuidle_reflect(dev, entered_state);
    } else {
        idle_call_stop_or_retain_tick(stop_tick);
        call_cpuidle(drv, dev, 0);
    }
    __current_set_polling();
    if WARN_ON_ONCE(irqs_disabled()) { local_irq_enable(); }
}

unsafe fn do_idle() {
    let cpu = smp_processor_id();
    let mut got_tick = false;
    if cpu_is_offline(cpu) {
        local_irq_disable();
        WARN_ON_ONCE(need_resched());
        cpuhp_report_idle_dead();
        arch_cpu_idle_dead();
    }
    nohz_run_idle_balance(cpu);
    __current_set_polling();
    tick_nohz_idle_enter();
    while !need_resched() {
        local_irq_disable();
        arch_cpu_idle_enter();
        rcu_nocb_flush_deferred_wakeup();
        if CPU_IDLE_FORCE_POLL != 0 || tick_check_broadcast_expired() {
            tick_nohz_idle_restart_tick();
            cpu_idle_poll();
        } else { cpuidle_idle_call(got_tick); }
        got_tick = tick_nohz_idle_got_tick();
        arch_cpu_idle_exit();
    }
    preempt_set_need_resched();
    tick_nohz_idle_exit();
    __current_clr_polling();
    smp_mb__after_atomic();
    flush_smp_call_function_queue();
    schedule_idle();
    if unlikely(klp_patch_pending(current)) { klp_update_patch_state(current); }
}

pub unsafe fn cpu_in_idle(pc: c_ulong) -> bool {
    pc >= (&__cpuidle_text_start as *const c_char as c_ulong) &&
        pc < (&__cpuidle_text_end as *const c_char as c_ulong)
}

#[repr(C)]
struct idle_timer { timer: hrtimer, done: c_int }

unsafe fn idle_inject_timer_fn(timer: *mut hrtimer) -> hrtimer_restart {
    let it = container_of(timer, idle_timer, timer);
    WRITE_ONCE((*it).done, 1);
    set_tsk_need_resched(current);
    HRTIMER_NORESTART
}

pub unsafe fn play_idle_precise(duration_ns: u64, latency_ns: u64) {
    let mut it: idle_timer = core::mem::zeroed();
    WARN_ON_ONCE(current_policy() != SCHED_FIFO);
    WARN_ON_ONCE((*current).nr_cpus_allowed != 1);
    WARN_ON_ONCE((*current).flags & PF_KTHREAD == 0);
    WARN_ON_ONCE((*current).flags & PF_NO_SETAFFINITY == 0);
    WARN_ON_ONCE(duration_ns == 0);
    WARN_ON_ONCE((*current).mm != core::ptr::null_mut());
    rcu_sleep_check(); preempt_disable(); (*current).flags |= PF_IDLE;
    cpuidle_use_deepest_state(latency_ns);
    it.done = 0;
    hrtimer_setup_on_stack(&mut it.timer, idle_inject_timer_fn, CLOCK_MONOTONIC, HRTIMER_MODE_REL_HARD);
    hrtimer_start(&mut it.timer, ns_to_ktime(duration_ns), HRTIMER_MODE_REL_PINNED_HARD);
    while READ_ONCE(it.done) == 0 { do_idle(); }
    cpuidle_use_deepest_state(0); (*current).flags &= !PF_IDLE;
    preempt_fold_need_resched(); preempt_enable();
}

pub unsafe fn cpu_startup_entry(state: cpuhp_state) {
    (*current).flags |= PF_IDLE;
    arch_cpu_idle_prepare(); cpuhp_online_idle(state);
    loop { do_idle(); }
}

unsafe fn select_task_rq_idle(p: *mut task_struct, _cpu: c_int, _flags: c_int) -> c_int { task_cpu(p) }
unsafe fn balance_idle(_rq: *mut rq, _rf: *mut rq_flags) -> c_int { WARN_ON_ONCE(true); 0 }
unsafe fn wakeup_preempt_idle(rq: *mut rq, _p: *mut task_struct, _flags: c_int) { resched_curr(rq); }
unsafe fn update_curr_idle(rq: *mut rq);

unsafe fn put_prev_task_idle(rq: *mut rq, _prev: *mut task_struct, _next: *mut task_struct) {
    update_curr_idle(rq); scx_update_idle(rq, false, true); update_rq_avg_idle(rq);
}
unsafe fn set_next_task_idle(rq: *mut rq, next: *mut task_struct, _first: bool) {
    update_idle_core(rq); scx_update_idle(rq, true, true); schedstat_inc((*rq).sched_goidle);
    (*next).se.exec_start = rq_clock_task(rq); update_idle_rq_clock_pelt(rq);
}
pub unsafe fn pick_task_idle(rq: *mut rq, _rf: *mut rq_flags) -> *mut task_struct {
    if scx_enabled() && is_idle_task((*rq).curr) { scx_update_idle(rq, true, false); }
    (*rq).idle
}
unsafe fn dequeue_task_idle(rq: *mut rq, _p: *mut task_struct, _flags: c_int) -> bool {
    raw_spin_rq_unlock_irq(rq); printk(KERN_ERR, b"bad: scheduling from the idle thread!\0".as_ptr()); dump_stack(); raw_spin_rq_lock_irq(rq); true
}
unsafe fn task_tick_idle(rq: *mut rq, _curr: *mut task_struct, _queued: c_int) { update_curr_idle(rq); }
unsafe fn switching_to_idle(_rq: *mut rq, _p: *mut task_struct) { BUG(); }
unsafe fn prio_changed_idle(_rq: *mut rq, p: *mut task_struct, oldprio: u64) { if (*p).prio != oldprio { BUG(); } }
unsafe fn update_curr_idle(rq: *mut rq) {
    let se = &mut (*(*rq).idle).se; let now = rq_clock_task(rq); let delta_exec = now - se.exec_start;
    if unlikely(delta_exec <= 0) { return; } se.exec_start = now; dl_server_update_idle(&mut (*rq).fair_server, delta_exec);
}

// DEFINE_SCHED_CLASS(idle): the idle scheduling-class vtable consists of the
// functions above; enqueue/yield are intentionally absent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
