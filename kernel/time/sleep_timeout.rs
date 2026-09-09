// SPDX-License-Identifier: GPL-2.0
/*
 *  Kernel internal schedule timeout and sleeping functions
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct process_timer {
    timer: timer_list,
    task: *mut task_struct,
}

unsafe fn process_timeout(t: *mut timer_list) {
    let timeout = timer_container_of::<process_timer>(t, 0);
    wake_up_process((*timeout).task);
}

pub unsafe fn schedule_timeout(mut timeout: i64) -> i64 {
    let mut timer: process_timer = core::mem::zeroed();
    let expire: u64;

    match timeout {
        MAX_SCHEDULE_TIMEOUT => {
            schedule();
            return if timeout < 0 { 0 } else { timeout };
        }
        _ => {
            if timeout < 0 {
                pr_err("%s: wrong timeout value %lx\n", "schedule_timeout", timeout);
                dump_stack();
                __set_current_state(TASK_RUNNING);
                return 0;
            }
        }
    }

    expire = (timeout as u64).wrapping_add(jiffies);
    timer.task = current;
    timer_setup_on_stack(&mut timer.timer, process_timeout, 0);
    timer.timer.expires = expire;
    add_timer(&mut timer.timer);
    schedule();
    timer_delete_sync(&mut timer.timer);
    timer_destroy_on_stack(&mut timer.timer);

    timeout = (expire.wrapping_sub(jiffies)) as i64;
    if timeout < 0 { 0 } else { timeout }
}

pub unsafe fn schedule_timeout_interruptible(timeout: i64) -> i64 {
    __set_current_state(TASK_INTERRUPTIBLE);
    schedule_timeout(timeout)
}

pub unsafe fn schedule_timeout_killable(timeout: i64) -> i64 {
    __set_current_state(TASK_KILLABLE);
    schedule_timeout(timeout)
}

pub unsafe fn schedule_timeout_uninterruptible(timeout: i64) -> i64 {
    __set_current_state(TASK_UNINTERRUPTIBLE);
    schedule_timeout(timeout)
}

pub unsafe fn schedule_timeout_idle(timeout: i64) -> i64 {
    __set_current_state(TASK_IDLE);
    schedule_timeout(timeout)
}

pub unsafe fn schedule_hrtimeout_range_clock(
    expires: *mut ktime_t, delta: u64, mode: hrtimer_mode, clock_id: clockid_t,
) -> i32 {
    let mut t: hrtimer_sleeper = core::mem::zeroed();
    if !expires.is_null() && *expires == 0 {
        __set_current_state(TASK_RUNNING);
        return 0;
    }
    if expires.is_null() {
        schedule();
        return -EINTR;
    }
    hrtimer_setup_sleeper_on_stack(&mut t, clock_id, mode);
    hrtimer_set_expires_range_ns(&mut t.timer, *expires, delta);
    hrtimer_sleeper_start_expires(&mut t, mode);
    if !t.task.is_null() { schedule(); }
    hrtimer_cancel(&mut t.timer);
    destroy_hrtimer_on_stack(&mut t.timer);
    __set_current_state(TASK_RUNNING);
    if t.task.is_null() { 0 } else { -EINTR }
}

pub unsafe fn schedule_hrtimeout_range(expires: *mut ktime_t, delta: u64, mode: hrtimer_mode) -> i32 {
    schedule_hrtimeout_range_clock(expires, delta, mode, CLOCK_MONOTONIC)
}

pub unsafe fn schedule_hrtimeout(expires: *mut ktime_t, mode: hrtimer_mode) -> i32 {
    schedule_hrtimeout_range(expires, 0, mode)
}

pub unsafe fn msleep(msecs: u32) {
    let mut timeout = msecs_to_jiffies(msecs);
    while timeout != 0 { timeout = schedule_timeout_uninterruptible(timeout); }
}

pub unsafe fn msleep_interruptible(msecs: u32) -> u64 {
    let mut timeout = msecs_to_jiffies(msecs);
    while timeout != 0 && !signal_pending(current) {
        timeout = schedule_timeout_interruptible(timeout);
    }
    jiffies_to_msecs(timeout)
}

pub unsafe fn usleep_range_state(min: u64, max: u64, state: u32) {
    let mut exp = ktime_add_us(ktime_get(), min);
    let mut delta = (max.wrapping_sub(min)).wrapping_mul(NSEC_PER_USEC as u64);
    if WARN_ON_ONCE(max < min) { delta = 0; }
    loop {
        __set_current_state(state);
        if schedule_hrtimeout_range(&mut exp, delta, HRTIMER_MODE_ABS) == 0 { break; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
