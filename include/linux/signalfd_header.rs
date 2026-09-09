/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  include/linux/signalfd.h
 *
 *  Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
 *
 */

// Dependency intent from <uapi/linux/signalfd.h> and <linux/sched/signal.h>.

#[cfg(feature = "CONFIG_SIGNALFD")]
pub unsafe fn signalfd_notify(tsk: *mut task_struct, sig: ::core::ffi::c_int) {
    // Deliver the signal to listening signalfd.
    if unlikely(waitqueue_active((*(*tsk).sighand).signalfd_wqh)) {
        wake_up((*(*tsk).sighand).signalfd_wqh);
    }
}

#[cfg(feature = "CONFIG_SIGNALFD")]
extern "C" {
    pub fn signalfd_cleanup(sighand: *mut sighand_struct);
}

#[cfg(not(feature = "CONFIG_SIGNALFD"))]
pub unsafe fn signalfd_notify(_tsk: *mut task_struct, _sig: ::core::ffi::c_int) {}

#[cfg(not(feature = "CONFIG_SIGNALFD"))]
pub unsafe fn signalfd_cleanup(_sighand: *mut sighand_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
