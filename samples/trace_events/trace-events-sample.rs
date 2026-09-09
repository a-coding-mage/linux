// SPDX-License-Identifier: GPL-2.0-only
// The Linux kernel module and trace-event declarations are supplied by the
// surrounding build environment.

/*
 * Any file that uses trace points, must include the header.
 * But only one file, must include the header by defining
 * CREATE_TRACE_POINTS first.  This will make the C code that
 * creates the handles for the trace points.
 */
// CREATE_TRACE_POINTS

static random_strings: [&'static [u8]; 5] = [
    b"Mother Goose\0",
    b"Snoopy\0",
    b"Gandalf\0",
    b"Frodo\0",
    b"One ring to rule them all\0",
];

unsafe fn do_simple_thread_func(cnt: i32, fmt: *const i8, mut args: ...) {
    let bitmask: [usize; 1] = [0xdeadbeefusize];
    let mut array: [i32; 6] = [0; 6];
    let len = cnt % 5;
    let mut i: i32;

    set_current_state(TASK_INTERRUPTIBLE);
    schedule_timeout(HZ);

    i = 0;
    while i < len {
        array[i as usize] = i + 1;
        i += 1;
    }
    array[i as usize] = 0;

    // Silly tracepoints
    trace_foo_bar(b"hello\0".as_ptr(), cnt, array.as_mut_ptr(),
                  random_strings[len as usize].as_ptr(), (*current).cpus_ptr,
                  fmt, &mut args);

    trace_foo_with_template_simple(b"HELLO\0".as_ptr(), cnt);
    trace_foo_bar_with_cond(b"Some times print\0".as_ptr(), cnt);
    trace_foo_with_template_cond(b"prints other times\0".as_ptr(), cnt);
    trace_foo_with_template_print(b"I have to be different\0".as_ptr(), cnt);
    trace_foo_rel_loc(b"Hello __rel_loc\0".as_ptr(), cnt,
                      bitmask.as_ptr(), (*current).cpus_ptr);
}

unsafe fn simple_thread_func(cnt: i32) {
    do_simple_thread_func(cnt, b"iter=%d\0".as_ptr() as *const i8, cnt);
}

unsafe extern "C" fn simple_thread(_arg: *mut core::ffi::c_void) -> i32 {
    let mut cnt = 0;
    while !kthread_should_stop() {
        simple_thread_func(cnt);
        cnt += 1;
    }
    0
}

static mut simple_tsk: *mut task_struct = core::ptr::null_mut();
static mut simple_tsk_fn: *mut task_struct = core::ptr::null_mut();

unsafe fn simple_thread_func_fn(cnt: i32) {
    set_current_state(TASK_INTERRUPTIBLE);
    schedule_timeout(HZ);

    // More silly tracepoints
    trace_foo_bar_with_fn(b"Look at me\0".as_ptr(), cnt);
    trace_foo_with_template_fn(b"Look at me too\0".as_ptr(), cnt);
}

unsafe extern "C" fn simple_thread_fn(_arg: *mut core::ffi::c_void) -> i32 {
    let mut cnt = 0;
    while !kthread_should_stop() {
        simple_thread_func_fn(cnt);
        cnt += 1;
    }
    0
}

static mut thread_mutex: mutex = DEFINE_MUTEX!();
static mut simple_thread_cnt: i32 = 0;
static mut foo_timer_data: *mut foo_timer_data = core::ptr::null_mut();

unsafe extern "C" fn sample_timer_cb(t: *mut timer_list) {
    let data = container_of!(t, foo_timer_data, timer);

    get_cpu();
    trace_foo_timer_fn(data);
    (*this_cpu_ptr((*data).counter)) += 1;
    put_cpu();

    mod_timer(t, jiffies + HZ);
}

#[no_mangle]
pub unsafe extern "C" fn foo_bar_reg() -> i32 {
    mutex_lock(&mut thread_mutex);
    if simple_thread_cnt != 0 {
        simple_thread_cnt += 1;
        mutex_unlock(&mut thread_mutex);
        return 0;
    }
    simple_thread_cnt += 1;

    pr_info!(b"Starting thread for foo_bar_fn\n\0");
    /*
     * We shouldn't be able to start a trace when the module is
     * unloading (there's other locks to prevent that). But
     * for consistency sake, we still take the thread_mutex.
     */
    simple_tsk_fn = kthread_run(simple_thread_fn, core::ptr::null_mut(),
                                b"event-sample-fn\0".as_ptr());
    if IS_ERR_OR_NULL!(simple_tsk_fn) {
        pr_err!(b"Failed to create simple_thread_fn\n\0");
        simple_tsk_fn = core::ptr::null_mut();
    }
    mutex_unlock(&mut thread_mutex);
    0
}

#[no_mangle]
pub unsafe extern "C" fn foo_bar_unreg() {
    mutex_lock(&mut thread_mutex);
    simple_thread_cnt -= 1;
    if simple_thread_cnt != 0 {
        mutex_unlock(&mut thread_mutex);
        return;
    }

    pr_info!(b"Killing thread for foo_bar_fn\n\0");
    if !simple_tsk_fn.is_null() {
        kthread_stop(simple_tsk_fn);
    }
    simple_tsk_fn = core::ptr::null_mut();
    mutex_unlock(&mut thread_mutex);
}

unsafe extern "C" fn trace_event_init() -> i32 {
    foo_timer_data = kzalloc_obj!(*foo_timer_data, GFP_KERNEL);
    if foo_timer_data.is_null() {
        return -ENOMEM;
    }

    (*foo_timer_data).name = b"sample_timer_counter\0".as_ptr();
    (*foo_timer_data).counter = alloc_percpu!(i32);
    if (*foo_timer_data).counter.is_null() {
        kfree(foo_timer_data);
        return -ENOMEM;
    }

    timer_setup!(&mut (*foo_timer_data).timer, sample_timer_cb, 0);
    mod_timer(&mut (*foo_timer_data).timer, jiffies + HZ);

    simple_tsk = kthread_run(simple_thread, core::ptr::null_mut(),
                             b"event-sample\0".as_ptr());
    if IS_ERR!(simple_tsk) {
        timer_shutdown_sync(&mut (*foo_timer_data).timer);
        free_percpu((*foo_timer_data).counter);
        kfree(foo_timer_data);
        return PTR_ERR!(simple_tsk);
    }
    0
}

unsafe extern "C" fn trace_event_exit() {
    kthread_stop(simple_tsk);
    mutex_lock(&mut thread_mutex);
    if !simple_tsk_fn.is_null() {
        kthread_stop(simple_tsk_fn);
    }
    simple_tsk_fn = core::ptr::null_mut();
    mutex_unlock(&mut thread_mutex);

    timer_shutdown_sync(&mut (*foo_timer_data).timer);
    free_percpu((*foo_timer_data).counter);
    kfree(foo_timer_data);
}

module_init!(trace_event_init);
module_exit!(trace_event_exit);

MODULE_AUTHOR!(b"Steven Rostedt\0");
MODULE_DESCRIPTION!(b"trace-events-sample\0");
MODULE_LICENSE!(b"GPL\0");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
