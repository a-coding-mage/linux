// SPDX-License-Identifier: GPL-2.0
/*
 *    Virtual cpu timer based timer functions.
 *
 *    Copyright IBM Corp. 2004, 2012
 *    Author(s): Jan Glauber <jan.glauber@de.ibm.com>
 */

// Linux and s390 dependencies supplied by other translation units.

static fn virt_timer_expire();

static mut virt_timer_list: list_head = LIST_HEAD_INIT();
static mut virt_timer_lock: spinlock_t = __SPIN_LOCK_UNLOCKED();
static mut virt_timer_current: atomic64_t = ATOMIC64_INIT(0);
static mut virt_timer_elapsed: atomic64_t = ATOMIC64_INIT(0);

static mut mt_cycles: [u64; 8] = [0; 8];
static mut mt_scaling_mult: u64 = 1;
static mut mt_scaling_div: u64 = 1;
static mut mt_scaling_jiffies: u64 = 0;

static unsafe fn set_vtimer(expires: u64) {
    let lc = get_lowcore();
    let timer: u64;
    // Store current cpu timer value and set new value immediately afterwards.
    asm!("stpt {timer}\n spt {expires}", timer = out(reg) timer, expires = in(reg) expires);
    (*lc).system_timer = (*lc).system_timer.wrapping_add((*lc).last_update_timer.wrapping_sub(timer));
    (*lc).last_update_timer = expires;
}

static unsafe fn virt_timer_forward(mut elapsed: u64) -> i32 {
    lockdep_assert_irqs_disabled();
    if list_empty(&virt_timer_list) { return 0; }
    elapsed = atomic64_add_return(elapsed, &mut virt_timer_elapsed) as u64;
    (elapsed >= atomic64_read(&virt_timer_current) as u64) as i32
}

unsafe fn update_mt_scaling() {
    let mut cycles_new = [0u64; 8];
    let cycles_old = this_cpu_ptr(&mut mt_cycles);
    let mut fac = 1u64;
    let mut mult = 0u64;
    let mut div = 0u64;
    stcctm(MT_DIAG, smp_cpu_mtid + 1, cycles_new.as_mut_ptr());
    for i in 0..=(smp_cpu_mtid as usize) {
        let delta = cycles_new[i].wrapping_sub((*cycles_old)[i]);
        div = div.wrapping_add(delta);
        mult = mult.wrapping_mul((i + 1) as u64).wrapping_add(delta.wrapping_mul(fac));
        fac = fac.wrapping_mul((i + 1) as u64);
    }
    div = div.wrapping_mul(fac);
    if div > 0 {
        __this_cpu_write(&mut mt_scaling_mult, mult);
        __this_cpu_write(&mut mt_scaling_div, div);
        memcpy(cycles_old, cycles_new.as_ptr(), core::mem::size_of::<u64>() * (smp_cpu_mtid as usize + 1));
    }
    __this_cpu_write(&mut mt_scaling_jiffies, jiffies_64);
}

static unsafe fn update_tsk_timer(tsk_vtime: *mut u64, new: u64) -> u64 {
    let delta = new.wrapping_sub(*tsk_vtime);
    *tsk_vtime = new;
    delta
}

static unsafe fn scale_vtime(vtime: u64) -> u64 {
    let mult = __this_cpu_read(&mt_scaling_mult);
    let div = __this_cpu_read(&mt_scaling_div);
    if smp_cpu_mtid != 0 { vtime.wrapping_mul(mult) / div } else { vtime }
}

unsafe fn account_system_index_scaled(p: *mut task_struct, cputime: u64, index: cpu_usage_stat) {
    (*p).stimescaled = (*p).stimescaled.wrapping_add(cputime_to_nsecs(scale_vtime(cputime)));
    account_system_index_time(p, cputime_to_nsecs(cputime), index);
}

static unsafe fn vtime_reset_last_update(lc: *mut lowcore) {
    asm!("stpt {timer}\n stckf {clock}", timer = out(reg) (*lc).last_update_timer, clock = out(reg) (*lc).last_update_clock);
}

static unsafe fn do_account_vtime(tsk: *mut task_struct) -> i32 {
    let lc = get_lowcore();
    let clock = (*lc).last_update_clock;
    let timer = (*lc).last_update_timer;
    vtime_reset_last_update(lc);
    let clock = (*lc).last_update_clock.wrapping_sub(clock);
    let timer = timer.wrapping_sub((*lc).last_update_timer);
    if hardirq_count() != 0 { (*lc).hardirq_timer = (*lc).hardirq_timer.wrapping_add(timer); } else { (*lc).system_timer = (*lc).system_timer.wrapping_add(timer); }
    if smp_cpu_mtid != 0 && time_after64(jiffies_64, __this_cpu_read(&mt_scaling_jiffies)) { update_mt_scaling(); }
    let user = update_tsk_timer(&mut (*tsk).thread.user_timer, (*lc).user_timer);
    let guest = update_tsk_timer(&mut (*tsk).thread.guest_timer, (*lc).guest_timer);
    let system = update_tsk_timer(&mut (*tsk).thread.system_timer, (*lc).system_timer);
    let hardirq = update_tsk_timer(&mut (*tsk).thread.hardirq_timer, (*lc).hardirq_timer);
    let softirq = update_tsk_timer(&mut (*tsk).thread.softirq_timer, (*lc).softirq_timer);
    (*lc).steal_timer = (*lc).steal_timer.wrapping_add(clock.wrapping_sub(user).wrapping_sub(guest).wrapping_sub(system).wrapping_sub(hardirq).wrapping_sub(softirq));
    if user != 0 { account_user_time(tsk, cputime_to_nsecs(user)); (*tsk).utimescaled += cputime_to_nsecs(scale_vtime(user)); }
    if guest != 0 { account_guest_time(tsk, cputime_to_nsecs(guest)); (*tsk).utimescaled += cputime_to_nsecs(scale_vtime(guest)); }
    if system != 0 { account_system_index_scaled(tsk, system, CPUTIME_SYSTEM); }
    if hardirq != 0 { account_system_index_scaled(tsk, hardirq, CPUTIME_IRQ); }
    if softirq != 0 { account_system_index_scaled(tsk, softirq, CPUTIME_SOFTIRQ); }
    virt_timer_forward(user.wrapping_add(guest).wrapping_add(system).wrapping_add(hardirq).wrapping_add(softirq))
}

pub unsafe fn vtime_task_switch(prev: *mut task_struct) {
    let lc = get_lowcore();
    do_account_vtime(prev);
    (*prev).thread.user_timer = (*lc).user_timer;
    (*prev).thread.guest_timer = (*lc).guest_timer;
    (*prev).thread.system_timer = (*lc).system_timer;
    (*prev).thread.hardirq_timer = (*lc).hardirq_timer;
    (*prev).thread.softirq_timer = (*lc).softirq_timer;
    (*lc).user_timer = (*current).thread.user_timer;
    (*lc).guest_timer = (*current).thread.guest_timer;
    (*lc).system_timer = (*current).thread.system_timer;
    (*lc).hardirq_timer = (*current).thread.hardirq_timer;
    (*lc).softirq_timer = (*current).thread.softirq_timer;
}

pub unsafe fn vtime_flush(tsk: *mut task_struct) {
    let lc = get_lowcore();
    if do_account_vtime(tsk) != 0 { virt_timer_expire(); }
    let steal = (*lc).steal_timer;
    let mut avg_steal = (*lc).avg_steal_timer;
    if (steal as i64) > 0 { (*lc).steal_timer = 0; account_steal_time(cputime_to_nsecs(steal)); avg_steal = avg_steal.wrapping_add(steal); }
    (*lc).avg_steal_timer = avg_steal / 2;
}

static unsafe fn vtime_delta() -> u64 {
    let lc = get_lowcore();
    let timer = (*lc).last_update_timer;
    (*lc).last_update_timer = get_cpu_timer();
    timer.wrapping_sub((*lc).last_update_timer)
}

pub unsafe fn vtime_account_kernel(tsk: *mut task_struct) {
    let lc = get_lowcore();
    let delta = vtime_delta();
    if (*tsk).flags & PF_VCPU != 0 { (*lc).guest_timer += delta; } else { (*lc).system_timer += delta; }
}

pub unsafe fn vtime_account_softirq(_tsk: *mut task_struct) { (*get_lowcore()).softirq_timer += vtime_delta(); }
pub unsafe fn vtime_account_hardirq(_tsk: *mut task_struct) { (*get_lowcore()).hardirq_timer += vtime_delta(); }

static unsafe fn list_add_sorted(timer: *mut vtimer_list, head: *mut list_head) {
    list_for_each_entry!(tmp, head, entry, vtimer_list, {
        if (*tmp).expires > (*timer).expires { list_add_tail(&mut (*timer).entry, &mut (*tmp).entry); return; }
    });
    list_add_tail(&mut (*timer).entry, head);
}

static unsafe fn virt_timer_expire() {
    let mut timer: *mut vtimer_list;
    let mut tmp: *mut vtimer_list;
    let mut cb_list = LIST_HEAD_INIT();
    spin_lock(&mut virt_timer_lock);
    let elapsed = atomic64_read(&virt_timer_elapsed) as u64;
    list_for_each_entry_safe!(timer, tmp, &mut virt_timer_list, entry, vtimer_list, {
        if (*timer).expires < elapsed { list_move_tail(&mut (*timer).entry, &mut cb_list); } else { (*timer).expires -= elapsed; }
    });
    if !list_empty(&virt_timer_list) { timer = list_first_entry(&mut virt_timer_list, vtimer_list, entry); atomic64_set(&mut virt_timer_current, (*timer).expires as i64); }
    atomic64_sub(elapsed as i64, &mut virt_timer_elapsed);
    spin_unlock(&mut virt_timer_lock);
    list_for_each_entry_safe!(timer, tmp, &mut cb_list, entry, vtimer_list, {
        list_del_init(&mut (*timer).entry); ((*timer).function)(*timer).data;
        if (*timer).interval != 0 { (*timer).expires = (*timer).interval + atomic64_read(&virt_timer_elapsed) as u64; spin_lock(&mut virt_timer_lock); list_add_sorted(timer, &mut virt_timer_list); spin_unlock(&mut virt_timer_lock); }
    });
}

pub unsafe fn init_virt_timer(timer: *mut vtimer_list) { (*timer).function = None; INIT_LIST_HEAD(&mut (*timer).entry); }
static unsafe fn vtimer_pending(timer: *mut vtimer_list) -> bool { !list_empty(&(*timer).entry) }

static unsafe fn internal_add_vtimer(timer: *mut vtimer_list) {
    if list_empty(&virt_timer_list) { atomic64_set(&mut virt_timer_current, (*timer).expires as i64); atomic64_set(&mut virt_timer_elapsed, 0); list_add(&mut (*timer).entry, &mut virt_timer_list); }
    else { (*timer).expires += atomic64_read(&virt_timer_elapsed) as u64; if (*timer).expires as i64 < atomic64_read(&virt_timer_current) { atomic64_set(&mut virt_timer_current, (*timer).expires as i64); } list_add_sorted(timer, &mut virt_timer_list); }
}

static unsafe fn __add_vtimer(timer: *mut vtimer_list, periodic: i32) { (*timer).interval = if periodic != 0 { (*timer).expires } else { 0 }; let mut flags = 0; spin_lock_irqsave(&mut virt_timer_lock, &mut flags); internal_add_vtimer(timer); spin_unlock_irqrestore(&mut virt_timer_lock, flags); }
pub unsafe fn add_virt_timer(timer: *mut vtimer_list) { __add_vtimer(timer, 0); }
pub unsafe fn add_virt_timer_periodic(timer: *mut vtimer_list) { __add_vtimer(timer, 1); }

static unsafe fn __mod_vtimer(timer: *mut vtimer_list, expires: u64, periodic: i32) -> i32 { BUG_ON!((*timer).function.is_none()); if (*timer).expires == expires && vtimer_pending(timer) { return 1; } let mut flags = 0; spin_lock_irqsave(&mut virt_timer_lock, &mut flags); let rc = vtimer_pending(timer) as i32; if rc != 0 { list_del_init(&mut (*timer).entry); } (*timer).interval = if periodic != 0 { expires } else { 0 }; (*timer).expires = expires; internal_add_vtimer(timer); spin_unlock_irqrestore(&mut virt_timer_lock, flags); rc }
pub unsafe fn mod_virt_timer(timer: *mut vtimer_list, expires: u64) -> i32 { __mod_vtimer(timer, expires, 0) }
pub unsafe fn mod_virt_timer_periodic(timer: *mut vtimer_list, expires: u64) -> i32 { __mod_vtimer(timer, expires, 1) }
pub unsafe fn del_virt_timer(timer: *mut vtimer_list) -> i32 { if !vtimer_pending(timer) { return 0; } let mut flags = 0; spin_lock_irqsave(&mut virt_timer_lock, &mut flags); list_del_init(&mut (*timer).entry); spin_unlock_irqrestore(&mut virt_timer_lock, flags); 1 }

pub unsafe fn vtime_init() {
    set_vtimer(VTIMER_MAX_SLICE);
    if smp_cpu_mtid != 0 { __this_cpu_write(&mut mt_scaling_jiffies, jiffies); __this_cpu_write(&mut mt_scaling_mult, 1); __this_cpu_write(&mut mt_scaling_div, 1); stcctm(MT_DIAG, smp_cpu_mtid + 1, this_cpu_ptr(&mut mt_cycles)); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
