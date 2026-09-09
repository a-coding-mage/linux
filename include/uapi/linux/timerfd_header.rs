/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  include/linux/timerfd.h
 *
 *  Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
 *
 */

// #include <linux/types.h>
// For O_CLOEXEC and O_NONBLOCK: #include <linux/fcntl.h>
// For _IO helpers: #include <linux/ioctl.h>

/*
 * CAREFUL: Check include/asm-generic/fcntl.h when defining
 * new flags, since they might collide with O_* ones. We want
 * to re-use O_* flags that couldn't possibly have a meaning
 * from eventfd, in order to leave a free define-space for
 * shared O_* flags.
 *
 * Also make sure to update the masks in include/linux/timerfd.h
 * when adding new flags.
 */
pub const TFD_TIMER_ABSTIME: u32 = 1u32 << 0;
pub const TFD_TIMER_CANCEL_ON_SET: u32 = 1u32 << 1;
pub const TFD_CLOEXEC: _ = O_CLOEXEC;
pub const TFD_NONBLOCK: _ = O_NONBLOCK;

pub const TFD_IOC_SET_TICKS: u64 = _IOW!(b'T', 0, u64);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
