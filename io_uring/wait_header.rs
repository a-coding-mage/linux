// SPDX-License-Identifier: GPL-2.0
//
// Dependency: declarations from <linux/io_uring_types.h> are supplied by
// other translated files.

/*
 * ->cq_wait_nr is armed with the number of lazy task_work adds the waiter
 * still needs, and counted down by the add side, with the add reaching zero
 * issuing the (single) wake up for this wait cycle. Zero and below means no
 * wake up is to be issued: IO_CQ_WAKE_INIT when no task is waiting (also
 * what a forced wake up resets it to when claiming one), zero once the
 * countdown has fired.
 */
pub const IO_CQ_WAKE_INIT: i32 = -1;

#[repr(C)]
pub struct ext_arg {
    pub argsz: usize,
    pub ts: timespec64,
    pub sig: *const sigset_t,
    pub min_time: ktime_t,
    pub ts_set: bool,
    pub iowait: bool,
}

extern "C" {
    pub fn io_cqring_wait(
        ctx: *mut io_ring_ctx,
        min_events: ::std::os::raw::c_int,
        flags: u32,
        ext_arg: *mut ext_arg,
    ) -> ::std::os::raw::c_int;
    pub fn io_run_task_work_sig(ctx: *mut io_ring_ctx) -> ::std::os::raw::c_int;
    pub fn io_cqring_do_overflow_flush(ctx: *mut io_ring_ctx);
    pub fn io_cqring_overflow_flush_locked(ctx: *mut io_ring_ctx);
}

#[inline]
pub unsafe fn __io_cqring_events(ctx: *mut io_ring_ctx) -> u32 {
    let rings: *mut io_rings = io_get_rings(ctx);
    (*ctx).cached_cq_tail - READ_ONCE((*rings).cq.head)
}

#[inline]
pub unsafe fn __io_cqring_events_user(ctx: *mut io_ring_ctx) -> u32 {
    let rings: *mut io_rings = io_get_rings(ctx);

    READ_ONCE((*rings).cq.tail) - READ_ONCE((*rings).cq.head)
}

/*
 * Reads the tail/head of the CQ ring while providing an acquire ordering,
 * see comment at top of io_uring.c.
 */
#[inline]
pub unsafe fn io_cqring_events(ctx: *mut io_ring_ctx) -> u32 {
    smp_rmb();
    __io_cqring_events(ctx)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
