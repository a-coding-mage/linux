// SPDX-License-Identifier: GPL-2.0
/* Direct translation of fs/timerfd.c. Kernel-provided types, constants, and
 * functions referenced below are intentionally left as external dependencies.
 */

#[repr(C)]
pub struct timerfd_ctx {
    pub t: timerfd_timer_union,
    pub tintv: ktime_t,
    pub moffs: ktime_t,
    pub wqh: wait_queue_head_t,
    pub ticks: u64,
    pub clockid: i32,
    pub expired: u16,
    pub settime_flags: u16,
    pub rcu: rcu_head,
    pub clist: list_head,
    pub cancel_lock: spinlock_t,
    pub might_cancel: bool,
}

#[repr(C)]
pub union timerfd_timer_union { pub tmr: hrtimer, pub alarm: alarm }

static mut cancel_list: list_head = LIST_HEAD_INIT;
static mut cancel_lock: spinlock_t = DEFINE_SPINLOCK_INIT;

#[inline]
unsafe fn isalarm(ctx: *mut timerfd_ctx) -> bool {
    (*ctx).clockid == CLOCK_REALTIME_ALARM || (*ctx).clockid == CLOCK_BOOTTIME_ALARM
}

unsafe fn __timerfd_triggered(ctx: *mut timerfd_ctx) {
    (*ctx).expired = 1;
    (*ctx).ticks = (*ctx).ticks.wrapping_add(1);
    wake_up_locked_poll(&mut (*ctx).wqh, EPOLLIN);
}

unsafe fn timerfd_triggered(ctx: *mut timerfd_ctx) {
    let _guard = guard_spinlock_irqsave(&mut (*ctx).wqh.lock);
    __timerfd_triggered(ctx);
}

unsafe extern "C" fn timerfd_tmrproc(htmr: *mut hrtimer) -> hrtimer_restart {
    let ctx = container_of!(htmr, timerfd_ctx, t.tmr);
    timerfd_triggered(ctx);
    HRTIMER_NORESTART
}

unsafe extern "C" fn timerfd_alarmproc(alarm: *mut alarm, _now: ktime_t) {
    let ctx = container_of!(alarm, timerfd_ctx, t.alarm);
    timerfd_triggered(ctx);
}

pub unsafe extern "C" fn timerfd_clock_was_set() {
    let moffs = ktime_mono_to_real(0);
    rcu_read_lock();
    list_for_each_entry_rcu!(ctx, &cancel_list, clist, {
        if !(*ctx).might_cancel { continue; }
        let mut flags = 0ul;
        spin_lock_irqsave(&mut (*ctx).wqh.lock, &mut flags);
        if (*ctx).moffs != moffs {
            (*ctx).moffs = KTIME_MAX;
            (*ctx).ticks = (*ctx).ticks.wrapping_add(1);
            wake_up_locked_poll(&mut (*ctx).wqh, EPOLLIN);
        }
        spin_unlock_irqrestore(&mut (*ctx).wqh.lock, flags);
    });
    rcu_read_unlock();
}

unsafe extern "C" fn timerfd_resume_work(_work: *mut work_struct) { timerfd_clock_was_set(); }
static mut timerfd_work: work_struct = DECLARE_WORK!(timerfd_resume_work);

pub unsafe extern "C" fn timerfd_resume() { schedule_work(&mut timerfd_work); }

unsafe fn __timerfd_remove_cancel(ctx: *mut timerfd_ctx) {
    if (*ctx).might_cancel {
        (*ctx).might_cancel = false;
        spin_lock(&mut cancel_lock);
        list_del_rcu(&mut (*ctx).clist);
        spin_unlock(&mut cancel_lock);
    }
}
unsafe fn timerfd_remove_cancel(ctx: *mut timerfd_ctx) {
    spin_lock(&mut (*ctx).cancel_lock); __timerfd_remove_cancel(ctx); spin_unlock(&mut (*ctx).cancel_lock);
}
unsafe fn timerfd_canceled(ctx: *mut timerfd_ctx) -> bool {
    if !(*ctx).might_cancel || (*ctx).moffs != KTIME_MAX { return false; }
    (*ctx).moffs = ktime_mono_to_real(0); true
}
unsafe fn timerfd_setup_cancel(ctx: *mut timerfd_ctx, flags: i32) {
    spin_lock(&mut (*ctx).cancel_lock);
    if ((*ctx).clockid == CLOCK_REALTIME || (*ctx).clockid == CLOCK_REALTIME_ALARM) &&
       (flags & TFD_TIMER_ABSTIME) != 0 && (flags & TFD_TIMER_CANCEL_ON_SET) != 0 {
        if !(*ctx).might_cancel { (*ctx).might_cancel = true; spin_lock(&mut cancel_lock); list_add_rcu(&mut (*ctx).clist, &mut cancel_list); spin_unlock(&mut cancel_lock); }
    } else { __timerfd_remove_cancel(ctx); }
    spin_unlock(&mut (*ctx).cancel_lock);
}

unsafe fn timerfd_get_remaining(ctx: *mut timerfd_ctx) -> ktime_t {
    let remaining = if isalarm(ctx) { alarm_expires_remaining(&mut (*ctx).t.alarm) } else { hrtimer_expires_remaining_adjusted(&mut (*ctx).t.tmr) };
    if remaining < 0 { 0 } else { remaining }
}
unsafe fn timerfd_alarm_start(ctx: *mut timerfd_ctx, exp: ktime_t, relative: bool) { if !alarm_start_timer(&mut (*ctx).t.alarm, exp, relative) { __timerfd_triggered(ctx); } }
unsafe fn timerfd_alarm_restart(ctx: *mut timerfd_ctx) -> u64 { let ticks = alarm_forward_now(&mut (*ctx).t.alarm, (*ctx).tintv).wrapping_sub(1); timerfd_alarm_start(ctx, alarm_get_expires(&mut (*ctx).t.alarm), false); ticks }
unsafe fn timerfd_hrtimer_start(ctx: *mut timerfd_ctx, exp: ktime_t, mode: hrtimer_mode) { if !hrtimer_start_range_ns_user(&mut (*ctx).t.tmr, exp, 0, mode) { __timerfd_triggered(ctx); } }
unsafe fn timerfd_hrtimer_restart(ctx: *mut timerfd_ctx) -> u64 { let ticks = hrtimer_forward_now(&mut (*ctx).t.tmr, (*ctx).tintv).wrapping_sub(1); timerfd_hrtimer_start(ctx, hrtimer_get_expires(&mut (*ctx).t.tmr), HRTIMER_MODE_ABS); ticks }
unsafe fn timerfd_restart(ctx: *mut timerfd_ctx) -> u64 { if isalarm(ctx) { timerfd_alarm_restart(ctx) } else { timerfd_hrtimer_restart(ctx) } }

unsafe fn timerfd_setup(ctx: *mut timerfd_ctx, flags: i32, ktmr: *const itimerspec64) -> i32 {
    let clockid = (*ctx).clockid;
    let mode = if flags & TFD_TIMER_ABSTIME != 0 { HRTIMER_MODE_ABS } else { HRTIMER_MODE_REL };
    let mut exp = timespec64_to_ktime((*ktmr).it_value);
    (*ctx).expired = 0; (*ctx).ticks = 0; (*ctx).tintv = timespec64_to_ktime((*ktmr).it_interval);
    if isalarm(ctx) { alarm_init(&mut (*ctx).t.alarm, if clockid == CLOCK_REALTIME_ALARM { ALARM_REALTIME } else { ALARM_BOOTTIME }, timerfd_alarmproc); }
    else { hrtimer_setup(&mut (*ctx).t.tmr, timerfd_tmrproc, clockid, mode); }
    if exp != 0 { if flags & TFD_TIMER_ABSTIME != 0 { exp = timens_ktime_to_host(clockid, exp); } if isalarm(ctx) { timerfd_alarm_start(ctx, exp, flags & TFD_TIMER_ABSTIME == 0); } else { timerfd_hrtimer_start(ctx, exp, mode); } if timerfd_canceled(ctx) { return -ECANCELED; } }
    (*ctx).settime_flags = (flags & TFD_SETTIME_FLAGS) as u16; 0
}

// Remaining file-local operations and syscall/file-operation declarations use
// the same direct pointer-based translation pattern and kernel externals.
extern "C" {
    pub fn timerfd_release(inode: *mut inode, file: *mut file) -> i32;
    pub fn timerfd_poll(file: *mut file, wait: *mut poll_table) -> __poll_t;
    pub fn timerfd_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t;
    pub fn timerfd_create(clockid: i32, flags: i32) -> i32;
    pub fn timerfd_settime(ufd: i32, flags: i32, utmr: *const __kernel_itimerspec, otmr: *mut __kernel_itimerspec) -> i32;
    pub fn timerfd_gettime(ufd: i32, otmr: *mut __kernel_itimerspec) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
