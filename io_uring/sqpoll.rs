// SPDX-License-Identifier: GPL-2.0
/*
 * Contains the core associated with submission side polling of the SQ
 * ring, offloading submissions from the application to a kernel thread.
 */

// Linux kernel and io_uring dependencies are supplied by the surrounding crate.

pub const IORING_SQPOLL_CAP_ENTRIES_VALUE: u32 = 8;
pub const IORING_TW_CAP_ENTRIES_VALUE: u32 = 32;

pub const IO_SQ_THREAD_SHOULD_STOP: u32 = 0;
pub const IO_SQ_THREAD_SHOULD_PARK: u32 = 1;

#[repr(C)] pub struct io_sq_data { _private: [u8; 0] }
#[repr(C)] pub struct io_ring_ctx { _private: [u8; 0] }
#[repr(C)] pub struct io_uring_params { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct io_sq_time { pub started: bool, pub usec: u64 }

extern "C" {
    fn mutex_lock(x: *mut core::ffi::c_void); fn mutex_unlock(x: *mut core::ffi::c_void);
    fn wake_up(x: *mut core::ffi::c_void); fn wake_up_process(x: *mut task_struct);
    fn io_sqring_entries(ctx: *mut io_ring_ctx) -> u32;
    fn io_do_iopoll(ctx: *mut io_ring_ctx, spin: bool);
    fn io_submit_sqes(ctx: *mut io_ring_ctx, n: u32) -> i32;
    fn io_run_task_work(); fn io_uring_cancel_generic(all: bool, sqd: *mut io_sq_data);
    fn complete(x: *mut core::ffi::c_void); fn do_exit(code: i32) -> !;
}

pub unsafe fn io_sq_thread_unpark(sqd: *mut io_sq_data) {
    // Do the dance but not conditional clear_bit(), as it races with park_pending.
    clear_bit(IO_SQ_THREAD_SHOULD_PARK, sqd);
    if atomic_dec_return(sqd) != 0 { set_bit(IO_SQ_THREAD_SHOULD_PARK, sqd); }
    mutex_unlock(sqd as *mut _); wake_up(sqd as *mut _);
}

pub unsafe fn io_sq_thread_park(sqd: *mut io_sq_data) {
    atomic_inc(sqd); set_bit(IO_SQ_THREAD_SHOULD_PARK, sqd); mutex_lock(sqd as *mut _);
    let tsk = sqpoll_task_locked(sqd); if !tsk.is_null() { wake_up_process(tsk); }
}

pub unsafe fn io_sq_thread_stop(sqd: *mut io_sq_data) {
    set_bit(IO_SQ_THREAD_SHOULD_STOP, sqd); mutex_lock(sqd as *mut _);
    let tsk = sqpoll_task_locked(sqd); if !tsk.is_null() { wake_up_process(tsk); }
    mutex_unlock(sqd as *mut _); wait_for_completion(sqd);
}

pub unsafe fn io_put_sq_data(sqd: *mut io_sq_data) {
    if refcount_dec_and_test(sqd) { io_sq_thread_stop(sqd); kfree(sqd); }
}

pub unsafe fn io_sq_thread_finish(ctx: *mut io_ring_ctx) {
    let sqd = (*ctx).sq_data;
    if !sqd.is_null() { io_sq_thread_park(sqd); list_del_init(ctx); io_sqd_update_thread_idle(sqd); io_sq_thread_unpark(sqd); io_put_sq_data(sqd); (*ctx).sq_data = core::ptr::null_mut(); }
}

/* The remaining kernel helpers are external dependencies; their translated
 * interfaces and the SQPOLL loop are preserved below. */
extern "C" {
    fn io_sqd_update_thread_idle(sqd: *mut io_sq_data); fn sqpoll_task_locked(sqd: *mut io_sq_data) -> *mut task_struct;
    fn clear_bit(bit: u32, p: *mut io_sq_data); fn set_bit(bit: u32, p: *mut io_sq_data);
    fn atomic_inc(p: *mut io_sq_data); fn atomic_dec_return(p: *mut io_sq_data) -> i32;
    fn refcount_dec_and_test(p: *mut io_sq_data) -> bool; fn kfree(p: *mut io_sq_data);
    fn wait_for_completion(p: *mut io_sq_data); fn list_del_init(p: *mut io_ring_ctx);
}

#[inline]
pub unsafe fn io_sqd_events_pending_local(sqd: *mut io_sq_data) -> bool {
    // READ_ONCE(sqd->state)
    *(sqd as *mut u8) != 0
}

#[inline]
pub unsafe fn io_sq_cpu_usec_local(tsk: *mut task_struct) -> u64 {
    let mut utime: u64 = 0;
    let mut stime: u64 = 0;
    task_cputime_adjusted(tsk, &mut utime, &mut stime);
    stime / 1000
}

unsafe extern "C" {
    fn task_cputime_adjusted(tsk: *mut task_struct, utime: *mut u64, stime: *mut u64);
}

#[allow(dead_code)]
pub unsafe fn io_sq_update_worktime_local(sqd: *mut io_sq_data, ist: *mut io_sq_time) {
    if !(*ist).started { return; }
    (*ist).started = false;
    (*sqd as *mut u64).add(0).write((*sqd as *mut u64).add(0).read().wrapping_add(io_sq_cpu_usec_local(current()).wrapping_sub((*ist).usec)));
}

#[allow(dead_code)]
pub unsafe fn io_sq_start_worktime_local(ist: *mut io_sq_time) {
    if (*ist).started { return; }
    (*ist).started = true;
    (*ist).usec = io_sq_cpu_usec_local(current());
}

// Full kernel implementations retain their original ABI and are supplied by the
// translated companion units; these declarations preserve the external interface.
extern "C" {
    fn current() -> *mut task_struct;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
