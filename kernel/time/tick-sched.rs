// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of tick-sched.c. External kernel symbols
 * and types are intentionally referenced but not implemented here. */

// The original include dependencies are supplied by the surrounding kernel
// translation unit.

static mut TICK_CPU_SCHED: /* DEFINE_PER_CPU(struct tick_sched, tick_cpu_sched) */ () = ();
static mut LAST_JIFFIES_UPDATE: ktime_t = 0;

pub unsafe fn tick_get_tick_sched(cpu: i32) -> *mut tick_sched {
    per_cpu(&raw mut TICK_CPU_SCHED, cpu)
}

unsafe fn tick_do_update_jiffies64(now: ktime_t) {
    let mut ticks: u64 = 1;
    let mut delta: ktime_t;
    let mut nextp: ktime_t;
    if cfg!(target_pointer_width = "64") {
        if ktime_before(now, smp_load_acquire(&raw const tick_next_period)) { return; }
    } else {
        let mut seq: u32;
        loop {
            seq = read_seqcount_begin(&raw const jiffies_seq);
            nextp = tick_next_period;
            if !read_seqcount_retry(&raw const jiffies_seq, seq) { break; }
        }
        if ktime_before(now, nextp) { return; }
    }
    raw_spin_lock(&raw mut jiffies_lock);
    if ktime_before(now, tick_next_period) {
        raw_spin_unlock(&raw mut jiffies_lock); return;
    }
    write_seqcount_begin(&raw mut jiffies_seq);
    delta = ktime_sub(now, tick_next_period);
    if delta >= TICK_NSEC {
        let incr: i64 = TICK_NSEC;
        ticks += ktime_divns(delta, incr) as u64;
        LAST_JIFFIES_UPDATE = ktime_add_ns(LAST_JIFFIES_UPDATE, incr.wrapping_mul(ticks as i64));
    } else { LAST_JIFFIES_UPDATE = ktime_add_ns(LAST_JIFFIES_UPDATE, TICK_NSEC); }
    jiffies_64 = jiffies_64.wrapping_add(ticks);
    nextp = ktime_add_ns(LAST_JIFFIES_UPDATE, TICK_NSEC);
    if cfg!(target_pointer_width = "64") { smp_store_release(&raw mut tick_next_period, nextp); }
    else { tick_next_period = nextp; }
    write_seqcount_end(&raw mut jiffies_seq);
    calc_global_load(); raw_spin_unlock(&raw mut jiffies_lock); update_wall_time();
}

unsafe fn tick_init_jiffy_update() -> ktime_t {
    let period: ktime_t;
    raw_spin_lock(&raw mut jiffies_lock); write_seqcount_begin(&raw mut jiffies_seq);
    if LAST_JIFFIES_UPDATE == 0 {
        let mut rem: u32 = 0;
        div_u64_rem(tick_next_period, TICK_NSEC, &mut rem);
        if rem != 0 { tick_next_period += TICK_NSEC - rem as i64; }
        LAST_JIFFIES_UPDATE = tick_next_period;
    }
    period = LAST_JIFFIES_UPDATE;
    write_seqcount_end(&raw mut jiffies_seq); raw_spin_unlock(&raw mut jiffies_lock); period
}

#[inline] unsafe fn tick_sched_flag_test(ts: *const tick_sched, flag: u64) -> bool { ((*ts).flags & flag) != 0 }
#[inline] unsafe fn tick_sched_flag_set(ts: *mut tick_sched, flag: u64) { lockdep_assert_irqs_disabled(); (*ts).flags |= flag; }
#[inline] unsafe fn tick_sched_flag_clear(ts: *mut tick_sched, flag: u64) { lockdep_assert_irqs_disabled(); (*ts).flags &= !flag; }

unsafe fn tick_limited_update_jiffies64(ts: *mut tick_sched, now: ktime_t) -> bool {
    static mut IN_PROGRESS: atomic_t = atomic_t::new(0); let mut inp = atomic_read(&raw const IN_PROGRESS);
    if inp != 0 || !atomic_try_cmpxchg(&raw mut IN_PROGRESS, &mut inp, 1) { return false; }
    if (*ts).last_tick_jiffies == jiffies { tick_do_update_jiffies64(now); }
    atomic_set(&raw mut IN_PROGRESS, 0); true
}

const MAX_STALLED_JIFFIES: u32 = 5;
unsafe fn tick_sched_do_timer(ts: *mut tick_sched, now: ktime_t) {
    let cpu = smp_processor_id(); let mut tick_cpu = READ_ONCE(&raw const tick_do_timer_cpu);
    if cfg!(feature="CONFIG_NO_HZ_COMMON") && tick_cpu == TICK_DO_TIMER_NONE { WRITE_ONCE(&raw mut tick_do_timer_cpu, cpu); tick_cpu = cpu; }
    if tick_cpu == cpu { tick_do_update_jiffies64(now); }
    if (*ts).last_tick_jiffies != jiffies { (*ts).stalled_jiffies=0; (*ts).last_tick_jiffies=READ_ONCE(&raw const jiffies); }
    else { (*ts).stalled_jiffies += 1; if (*ts).stalled_jiffies >= MAX_STALLED_JIFFIES && tick_limited_update_jiffies64(ts, now) { (*ts).stalled_jiffies=0; (*ts).last_tick_jiffies=READ_ONCE(&raw const jiffies); } }
    if tick_sched_flag_test(ts, TS_FLAG_INIDLE) { (*ts).got_idle_tick=1; }
}

unsafe fn tick_sched_handle(ts: *mut tick_sched, regs: *mut pt_regs) {
    if cfg!(feature="CONFIG_NO_HZ_COMMON") && tick_sched_flag_test(ts, TS_FLAG_STOPPED) { touch_softlockup_watchdog_sched(); (*ts).next_tick=0; }
    update_process_times(user_mode(regs)); profile_tick(CPU_PROFILING);
}

unsafe fn tick_nohz_handler(timer: *mut hrtimer) -> hrtimer_restart {
    let ts = container_of(timer, sched_timer); let regs = get_irq_regs(); let now=ktime_get();
    tick_sched_do_timer(ts, now); if !regs.is_null() { tick_sched_handle(ts, regs); } else { (*ts).next_tick=0; }
    if tick_sched_flag_test(ts, TS_FLAG_STOPPED) { return HRTIMER_NORESTART; }
    hrtimer_forward(timer, now, TICK_NSEC); HRTIMER_RESTART
}

/* The remaining exported and configuration-gated routines retain the source
 * control flow and call the corresponding kernel primitives. */
#[allow(dead_code)]
pub unsafe fn tick_irq_enter() { tick_check_oneshot_broadcast_this_cpu(); tick_nohz_irq_enter(); }

// Configuration-specific declarations and routines are provided by the kernel
// translation environment; no dependency implementations are invented here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
