// SPDX-License-Identifier: GPL-2.0-only
//
// C dependencies supplied by the Linux kernel and sample-trace-array.h are
// intentionally referenced here rather than implemented in this translation.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct trace_array {
    _private: [u8; 0],
}
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

extern "C" {
    static mut jiffies: c_ulong;
    fn trace_array_set_clr_event(
        tr: *mut trace_array,
        subsystem: *const c_char,
        event: *const c_char,
        set: bool,
    );
    fn schedule_work(work: *mut work_struct);
    fn set_current_state(state: c_int);
    fn schedule_timeout(timeout: c_ulong);
    fn trace_array_printk(
        tr: *mut trace_array,
        ip: *const c_void,
        format: *const c_char,
        count: c_int,
    );
    fn trace_sample_event(count: c_int, jiffies: c_ulong);
    fn kthread_should_stop() -> bool;
    fn msecs_to_jiffies(milliseconds: c_ulong) -> c_ulong;
    fn add_timer(timer: *mut timer_list);
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn timer_delete(timer: *mut timer_list);
    fn cancel_work_sync(work: *mut work_struct);
    fn trace_array_put(tr: *mut trace_array);
    fn trace_array_get_by_name(name: *const c_char, tracing: *const c_char) -> *mut trace_array;
    fn trace_array_init_printk(tr: *mut trace_array);
    fn kthread_run(threadfn: unsafe extern "C" fn(*mut c_void) -> c_int, data: *mut c_void, name: *const c_char) -> *mut task_struct;
    fn is_err(ptr: *mut task_struct) -> bool;
    fn kthread_stop(task: *mut task_struct) -> c_int;
    fn trace_array_destroy(tr: *mut trace_array);
}

type c_ulong = core::ffi::c_ulong;

static mut tr: *mut trace_array = core::ptr::null_mut();
static mut simple_tsk: *mut task_struct = core::ptr::null_mut();

/*
 * Any file that uses trace points, must include the header.
 * But only one file, must include the header by defining
 * CREATE_TRACE_POINTS first.  This will make the C code that
 * creates the handles for the trace points.
 */
// #define CREATE_TRACE_POINTS
// #include "sample-trace-array.h"

unsafe extern "C" fn trace_work_fn(_work: *mut work_struct) {
    /*
     * Disable tracing for event "sample_event".
     */
    trace_array_set_clr_event(
        tr,
        c"sample-subsystem".as_ptr(),
        c"sample_event".as_ptr(),
        false,
    );
}

static mut trace_work: work_struct = work_struct { _private: [] };

/*
 * mytimer: Timer setup to disable tracing for event "sample_event". This
 * timer is only for the purposes of the sample module to demonstrate access of
 * Ftrace instances from within kernel.
 */
static mut mytimer: timer_list = timer_list { _private: [] };

unsafe extern "C" fn mytimer_handler(_unused: *mut timer_list) {
    schedule_work(&raw mut trace_work);
}

unsafe fn simple_thread_func(count: c_int) {
    set_current_state(1 /* TASK_INTERRUPTIBLE */);
    schedule_timeout(1 /* HZ */);

    /*
     * Printing count value using trace_array_printk() - trace_printk()
     * equivalent for the instance buffers.
     */
    trace_array_printk(
        tr,
        core::ptr::null(),
        c"trace_array_printk: count=%d\n".as_ptr(),
        count,
    );
    /*
     * Tracepoint for event "sample_event". This will print the
     * current value of count and current jiffies.
     */
    trace_sample_event(count, jiffies);
}

unsafe extern "C" fn simple_thread(_arg: *mut c_void) -> c_int {
    let mut count: c_int = 0;
    let delay: c_ulong = msecs_to_jiffies(5000);

    /*
     * Enable tracing for "sample_event".
     */
    trace_array_set_clr_event(tr, c"sample-subsystem".as_ptr(), c"sample_event".as_ptr(), true);

    /*
     * Adding timer - mytimer. This timer will disable tracing after
     * delay seconds.
     *
     */
    add_timer(&raw mut mytimer);
    mod_timer(&raw mut mytimer, jiffies.wrapping_add(delay));

    while !kthread_should_stop() {
        simple_thread_func(count);
        count = count.wrapping_add(1);
    }

    timer_delete(&raw mut mytimer);
    cancel_work_sync(&raw mut trace_work);

    /*
     * trace_array_put() decrements the reference counter associated with
     * the trace array - "tr". We are done using the trace array, hence
     * decrement the reference counter so that it can be destroyed using
     * trace_array_destroy().
     */
    trace_array_put(tr);

    0
}

unsafe extern "C" fn sample_trace_array_init() -> c_int {
    /*
     * Return a pointer to the trace array with name "sample-instance" if it
     * exists, else create a new trace array.
     *
     * NOTE: This function increments the reference counter
     * associated with the trace array - "tr".
     */
    tr = trace_array_get_by_name(c"sample-instance".as_ptr(), c"sched,timer,kprobes".as_ptr());

    if tr.is_null() {
        return -1;
    }
    /*
     * If context specific per-cpu buffers havent already been allocated.
     */
    trace_array_init_printk(tr);

    simple_tsk = kthread_run(simple_thread, core::ptr::null_mut(), c"sample-instance".as_ptr());
    if is_err(simple_tsk) {
        trace_array_put(tr);
        trace_array_destroy(tr);
        return -1;
    }

    0
}

unsafe extern "C" fn sample_trace_array_exit() {
    kthread_stop(simple_tsk);

    /*
     * We are unloading our module and no longer require the trace array.
     * Remove/destroy "tr" using trace_array_destroy()
     */
    trace_array_destroy(tr);
}

// module_init(sample_trace_array_init);
// module_exit(sample_trace_array_exit);

// MODULE_AUTHOR("Divya Indi");
// MODULE_DESCRIPTION("Sample module for kernel access to Ftrace instances");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
