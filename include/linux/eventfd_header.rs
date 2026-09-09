/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  include/linux/eventfd.h
 *
 *  Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
 */

// Dependencies supplied by the corresponding Linux Rust translation.
// #include <linux/wait.h>
// #include <linux/err.h>
// #include <linux/percpu-defs.h>
// #include <linux/percpu.h>
// #include <linux/sched.h>
// #include <uapi/linux/eventfd.h>

/*
 * CAREFUL: Check include/uapi/asm-generic/fcntl.h when defining
 * new flags, since they might collide with O_* ones. We want
 * to re-use O_* flags that couldn't possibly have a meaning
 * from eventfd, in order to leave a free define-space for
 * shared O_* flags.
 */
pub const EFD_SHARED_FCNTL_FLAGS: u32 = O_CLOEXEC | O_NONBLOCK;
pub const EFD_FLAGS_SET: u32 = EFD_SHARED_FCNTL_FLAGS | EFD_SEMAPHORE;

pub struct eventfd_ctx;
pub struct file;

// CONFIG_EVENTFD selects the declaration branch below.
#[cfg(feature = "CONFIG_EVENTFD")]
extern "C" {
    pub fn eventfd_ctx_put(ctx: *mut eventfd_ctx);
    pub fn eventfd_fget(fd: i32) -> *mut file;
    pub fn eventfd_ctx_fdget(fd: i32) -> *mut eventfd_ctx;
    pub fn eventfd_ctx_fileget(file: *mut file) -> *mut eventfd_ctx;
    pub fn eventfd_signal_mask(ctx: *mut eventfd_ctx, mask: __poll_t);
    pub fn eventfd_ctx_remove_wait_queue(
        ctx: *mut eventfd_ctx,
        wait: *mut wait_queue_entry_t,
        cnt: *mut __u64,
    ) -> i32;
    pub fn eventfd_ctx_do_read(ctx: *mut eventfd_ctx, cnt: *mut __u64);
}

#[cfg(feature = "CONFIG_EVENTFD")]
#[inline]
pub unsafe fn eventfd_signal_allowed() -> bool {
    !(*current).in_eventfd
}

// !CONFIG_EVENTFD error-layer definitions.
#[cfg(not(feature = "CONFIG_EVENTFD"))]
#[inline]
pub unsafe fn eventfd_ctx_fdget(_fd: i32) -> *mut eventfd_ctx {
    ERR_PTR(-(ENOSYS as isize))
}

#[cfg(not(feature = "CONFIG_EVENTFD"))]
#[inline]
pub unsafe fn eventfd_signal_mask(_ctx: *mut eventfd_ctx, _mask: __poll_t) {}

#[cfg(not(feature = "CONFIG_EVENTFD"))]
#[inline]
pub unsafe fn eventfd_ctx_put(_ctx: *mut eventfd_ctx) {}

#[cfg(not(feature = "CONFIG_EVENTFD"))]
#[inline]
pub unsafe fn eventfd_ctx_remove_wait_queue(
    _ctx: *mut eventfd_ctx,
    _wait: *mut wait_queue_entry_t,
    _cnt: *mut __u64,
) -> i32 {
    -(ENOSYS as i32)
}

#[cfg(not(feature = "CONFIG_EVENTFD"))]
#[inline]
pub unsafe fn eventfd_signal_allowed() -> bool {
    true
}

#[cfg(not(feature = "CONFIG_EVENTFD"))]
#[inline]
pub unsafe fn eventfd_ctx_do_read(_ctx: *mut eventfd_ctx, _cnt: *mut __u64) {}

#[inline]
pub unsafe fn eventfd_signal(ctx: *mut eventfd_ctx) {
    eventfd_signal_mask(ctx, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
