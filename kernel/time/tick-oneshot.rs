// SPDX-License-Identifier: GPL-2.0
/*
 * This file contains functions which manage high resolution tick
 * related events.
 *
 * Copyright(C) 2005-2006, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
 * Copyright(C) 2005-2007, Red Hat, Inc., Ingo Molnar
 * Copyright(C) 2006-2007, Timesys Corp., Thomas Gleixner
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/cpu.h, linux/err.h, linux/hrtimer.h, linux/interrupt.h,
// linux/percpu.h, linux/profile.h, linux/sched.h, and tick-internal.h.

/**
 * tick_program_event - program the CPU local timer device for the next event
 * @expires: the time at which the next timer event should occur
 * @force: flag to force reprograming even if the event time hasn't changed
 *
 * Return: 0 on success, negative error code on failure
 */
pub unsafe fn tick_program_event(expires: ktime_t, force: i32) -> i32 {
    let dev: *mut clock_event_device = __this_cpu_read(tick_cpu_device.evtdev);

    if unlikely(expires == KTIME_MAX) {
        /*
         * We don't need the clock event device any more, stop it.
         */
        clockevents_switch_state(dev, CLOCK_EVT_STATE_ONESHOT_STOPPED);
        (*dev).next_event = KTIME_MAX;
        return 0;
    }

    if unlikely(clockevent_state_oneshot_stopped(dev)) {
        /*
         * We need the clock event again, configure it in ONESHOT mode
         * before using it.
         */
        clockevents_switch_state(dev, CLOCK_EVT_STATE_ONESHOT);
    }

    clockevents_program_event(dev, expires, force)
}

/**
 * tick_resume_oneshot - resume oneshot mode
 */
pub unsafe fn tick_resume_oneshot() {
    let dev: *mut clock_event_device = __this_cpu_read(tick_cpu_device.evtdev);

    clockevents_switch_state(dev, CLOCK_EVT_STATE_ONESHOT);
    clockevents_program_event(dev, ktime_get(), true);
}

/**
 * tick_setup_oneshot - setup the event device for oneshot mode (hres or nohz)
 * @newdev: Pointer to the clock event device to configure
 * @handler: Function to be called when the event device triggers an interrupt
 * @next_event: Initial expiry time for the next event (in ktime)
 *
 * Configures the specified clock event device for onshot mode,
 * assigns the given handler as its event callback, and programs
 * the device to trigger at the specified next event time.
 */
pub unsafe fn tick_setup_oneshot(
    newdev: *mut clock_event_device,
    handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
    next_event: ktime_t,
) {
    (*newdev).event_handler = handler;
    clockevents_switch_state(newdev, CLOCK_EVT_STATE_ONESHOT);
    clockevents_program_event(newdev, next_event, true);
}

/**
 * tick_switch_to_oneshot - switch to oneshot mode
 * @handler: function to call when an event occurs on the tick device
 *
 * Return: 0 on success, -EINVAL if the tick device is not present,
 *         not functional, or does not support oneshot mode.
 */
pub unsafe fn tick_switch_to_oneshot(
    handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
) -> i32 {
    let td: *mut tick_device = this_cpu_ptr(&mut tick_cpu_device);
    let dev: *mut clock_event_device = (*td).evtdev;

    if dev.is_null()
        || ((*dev).features & CLOCK_EVT_FEAT_ONESHOT) == 0
        || !tick_device_is_functional(dev)
    {
        pr_info("Clockevents: could not switch to one-shot mode:");
        if dev.is_null() {
            pr_cont(" no tick device\n");
        } else if !tick_device_is_functional(dev) {
            pr_cont(" %s is not functional.\n", (*dev).name);
        } else {
            pr_cont(" %s does not support one-shot mode.\n", (*dev).name);
        }
        return -EINVAL;
    }

    (*td).mode = TICKDEV_MODE_ONESHOT;
    (*dev).event_handler = handler;
    clockevents_switch_state(dev, CLOCK_EVT_STATE_ONESHOT);
    tick_broadcast_switch_to_oneshot();
    0
}

/**
 * tick_oneshot_mode_active - check whether the system is in oneshot mode
 *
 * Return: 1 when either nohz or highres are enabled, otherwise 0.
 */
pub unsafe fn tick_oneshot_mode_active() -> i32 {
    let mut flags: ulong;
    let ret: i32;

    local_irq_save(&mut flags);
    ret = (__this_cpu_read(tick_cpu_device.mode) == TICKDEV_MODE_ONESHOT) as i32;
    local_irq_restore(flags);

    ret
}

#[cfg(CONFIG_HIGH_RES_TIMERS)]
/**
 * tick_init_highres - switch to high resolution mode
 *
 * Called with interrupts disabled.
 *
 * Return: 0 on success, -EINVAL if the tick device cannot switch
 *         to oneshot/high-resolution mode.
 */
pub unsafe fn tick_init_highres() -> i32 {
    tick_switch_to_oneshot(Some(hrtimer_interrupt))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
