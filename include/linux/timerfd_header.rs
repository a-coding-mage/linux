/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  include/linux/timerfd.h
 *
 *  Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
 *
 */

// Dependency supplied by the corresponding UAPI timerfd declarations:
// #include <uapi/linux/timerfd.h>

pub const TFD_SHARED_FCNTL_FLAGS: u32 = TFD_CLOEXEC | TFD_NONBLOCK;
/* Flags for timerfd_create.  */
pub const TFD_CREATE_FLAGS: u32 = TFD_SHARED_FCNTL_FLAGS;
/* Flags for timerfd_settime.  */
pub const TFD_SETTIME_FLAGS: u32 = TFD_TIMER_ABSTIME | TFD_TIMER_CANCEL_ON_SET;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
