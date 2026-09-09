// SPDX-License-Identifier: GPL-2.0
/*
 * Generic sched_clock() support, to extend low level hardware time
 * counters to full 64-bit ns values.
 */

// Linux header dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct clock_data {
    pub seq: seqcount_latch_t,
    pub read_data: [clock_read_data; 2],
    pub wrap_kt: ktime_t,
    pub rate: c_ulong,
    pub actual_read_sched_clock: Option<unsafe extern "C" fn() -> u64>,
}

static mut sched_clock_timer: hrtimer = unsafe { core::mem::zeroed() };
static mut irqtime: c_int = -1;

unsafe extern "C" fn jiffy_sched_clock_read() -> u64 {
    // We don't need to use get_jiffies_64 on 32-bit arches here because we
    // register with BITS_PER_LONG.
    (jiffies.wrapping_sub(INITIAL_JIFFIES)) as u64
}

static mut cd: clock_data = clock_data {
    seq: unsafe { core::mem::zeroed() },
    read_data: unsafe { core::mem::zeroed() },
    wrap_kt: 0,
    rate: 0,
    actual_read_sched_clock: Some(jiffy_sched_clock_read),
};

#[inline(always)]
unsafe fn cyc_to_ns(cyc: u64, mult: u32, shift: u32) -> u64 {
    cyc.wrapping_mul(mult as u64) >> shift
}

pub unsafe extern "C" fn sched_clock_read_begin(seq: *mut c_uint) -> *mut clock_read_data {
    *seq = read_seqcount_latch(&raw mut cd.seq);
    (*(&raw mut cd).cast::<clock_data>()).read_data.as_mut_ptr().add((*seq & 1) as usize)
}

pub unsafe extern "C" fn sched_clock_read_retry(seq: c_uint) -> c_int {
    read_seqcount_latch_retry(&raw mut cd.seq, seq)
}

#[inline(always)]
unsafe fn __sched_clock() -> u64 {
    let mut seq: c_uint;
    let mut cyc: u64;
    let mut res: u64;
    loop {
        seq = raw_read_seqcount_latch(&raw mut cd.seq);
        let rd = &*cd.read_data.as_ptr().add((seq & 1) as usize);
        cyc = (rd.read_sched_clock.unwrap()() - rd.epoch_cyc) & rd.sched_clock_mask;
        res = rd.epoch_ns + cyc_to_ns(cyc, rd.mult, rd.shift);
        if raw_read_seqcount_latch_retry(&raw mut cd.seq, seq) == 0 { break; }
    }
    res
}

pub unsafe extern "C" fn sched_clock_noinstr() -> u64 { __sched_clock() }

pub unsafe extern "C" fn sched_clock() -> u64 {
    preempt_disable_notrace();
    kcsan_nestable_atomic_begin();
    let ns = __sched_clock();
    kcsan_nestable_atomic_end();
    preempt_enable_notrace();
    ns
}

unsafe fn update_clock_read_data(rd: *const clock_read_data) {
    write_seqcount_latch_begin(&raw mut cd.seq);
    cd.read_data[0] = *rd;
    write_seqcount_latch(&raw mut cd.seq);
    cd.read_data[1] = *rd;
    write_seqcount_latch_end(&raw mut cd.seq);
}

unsafe fn update_sched_clock() {
    let mut rd = cd.read_data[0];
    let cyc = cd.actual_read_sched_clock.unwrap()();
    let ns = rd.epoch_ns + cyc_to_ns((cyc - rd.epoch_cyc) & rd.sched_clock_mask, rd.mult, rd.shift);
    rd.epoch_ns = ns;
    rd.epoch_cyc = cyc;
    update_clock_read_data(&rd);
}

unsafe extern "C" fn sched_clock_poll(hrt: *mut hrtimer) -> hrtimer_restart {
    update_sched_clock();
    hrtimer_forward_now(hrt, cd.wrap_kt);
    HRTIMER_RESTART
}

pub unsafe extern "C" fn sched_clock_register(read: Option<unsafe extern "C" fn() -> u64>, bits: c_int, rate: c_ulong) {
    if cd.rate > rate { return; }
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    let mut new_mult = 0u32;
    let mut new_shift = 0u32;
    clocks_calc_mult_shift(&mut new_mult, &mut new_shift, rate, NSEC_PER_SEC, 3600);
    let new_mask = CLOCKSOURCE_MASK(bits);
    cd.rate = rate;
    let wrap = clocks_calc_max_nsecs(new_mult, new_shift, 0, new_mask, core::ptr::null_mut());
    cd.wrap_kt = ns_to_ktime(wrap);
    let mut rd = cd.read_data[0];
    let new_epoch = read.unwrap()();
    let cyc = cd.actual_read_sched_clock.unwrap()();
    let ns = rd.epoch_ns + cyc_to_ns((cyc - rd.epoch_cyc) & rd.sched_clock_mask, rd.mult, rd.shift);
    cd.actual_read_sched_clock = read;
    rd.read_sched_clock = read;
    rd.sched_clock_mask = new_mask;
    rd.mult = new_mult;
    rd.shift = new_shift;
    rd.epoch_cyc = new_epoch;
    rd.epoch_ns = ns;
    update_clock_read_data(&rd);
    if ACCESS_PRIVATE!(&sched_clock_timer, function) != core::ptr::null_mut() {
        hrtimer_start(&raw mut sched_clock_timer, cd.wrap_kt, HRTIMER_MODE_REL_HARD);
    }
    local_irq_restore(flags);
}

pub unsafe extern "C" fn generic_sched_clock_init() {
    if cd.actual_read_sched_clock == Some(jiffy_sched_clock_read) {
        sched_clock_register(Some(jiffy_sched_clock_read), BITS_PER_LONG, HZ);
    }
    update_sched_clock();
    hrtimer_setup(&raw mut sched_clock_timer, Some(sched_clock_poll), CLOCK_MONOTONIC, HRTIMER_MODE_REL_HARD);
    hrtimer_start(&raw mut sched_clock_timer, cd.wrap_kt, HRTIMER_MODE_REL_HARD);
}

unsafe extern "C" fn suspended_sched_clock_read() -> u64 {
    let seq = read_seqcount_latch(&raw mut cd.seq);
    cd.read_data[(seq & 1) as usize].epoch_cyc
}

pub unsafe extern "C" fn sched_clock_suspend() -> c_int {
    update_sched_clock();
    hrtimer_cancel(&raw mut sched_clock_timer);
    cd.read_data[0].read_sched_clock = Some(suspended_sched_clock_read);
    0
}

unsafe extern "C" fn sched_clock_syscore_suspend(_: *mut c_void) -> c_int { sched_clock_suspend() }

pub unsafe extern "C" fn sched_clock_resume() {
    cd.read_data[0].epoch_cyc = cd.actual_read_sched_clock.unwrap()();
    hrtimer_start(&raw mut sched_clock_timer, cd.wrap_kt, HRTIMER_MODE_REL_HARD);
    cd.read_data[0].read_sched_clock = cd.actual_read_sched_clock;
}

unsafe extern "C" fn sched_clock_syscore_resume(_: *mut c_void) { sched_clock_resume(); }

static sched_clock_syscore_ops: syscore_ops = syscore_ops {
    suspend: Some(sched_clock_syscore_suspend),
    resume: Some(sched_clock_syscore_resume),
};

static mut sched_clock_syscore: syscore = syscore { ops: &sched_clock_syscore_ops };

unsafe extern "C" fn sched_clock_syscore_init() -> c_int {
    register_syscore(&raw mut sched_clock_syscore);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
