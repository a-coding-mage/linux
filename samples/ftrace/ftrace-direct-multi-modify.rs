// SPDX-License-Identifier: GPL-2.0-only
// Kernel dependencies supplied by the surrounding build.

unsafe extern "C" {
    fn trace_printk(fmt: *const core::ffi::c_char, ...);
    fn wake_up_process(task: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn schedule();
    fn kthread_should_stop() -> core::ffi::c_bool;
    fn set_current_state(state: core::ffi::c_int);
    fn schedule_timeout(timeout: core::ffi::c_long) -> core::ffi::c_long;
    fn modify_ftrace_direct(ops: *mut ftrace_ops, addr: usize) -> core::ffi::c_int;
    fn ftrace_set_filter_ip(ops: *mut ftrace_ops, ip: usize, remove: core::ffi::c_int, reset: core::ffi::c_int) -> core::ffi::c_int;
    fn register_ftrace_direct(ops: *mut ftrace_ops, addr: usize) -> core::ffi::c_int;
    fn kthread_run(thread: unsafe extern "C" fn(*mut core::ffi::c_void) -> core::ffi::c_int, arg: *mut core::ffi::c_void, name: *const core::ffi::c_char) -> *mut task_struct;
    fn kthread_stop(task: *mut task_struct) -> core::ffi::c_int;
    fn unregister_ftrace_direct(ops: *mut ftrace_ops, addr: usize, reset: bool);
}

#[repr(C)]
pub struct ftrace_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

const TASK_INTERRUPTIBLE: core::ffi::c_int = 1;
const HZ: core::ffi::c_long = 1;

#[no_mangle]
pub unsafe extern "C" fn my_direct_func1(ip: usize) {
    trace_printk(c"my direct func1 ip %lx\n".as_ptr(), ip);
}

#[no_mangle]
pub unsafe extern "C" fn my_direct_func2(ip: usize) {
    trace_printk(c"my direct func2 ip %lx\n".as_ptr(), ip);
}

unsafe extern "C" {
    fn my_tramp1(arg: *mut core::ffi::c_void);
    fn my_tramp2(arg: *mut core::ffi::c_void);
}

// The following architecture-specific trampoline bodies are supplied as
// inline assembly in the C source.  They must remain architecture-specific
// assembly when integrated with the kernel build; the declarations above
// preserve their externally visible Rust interfaces.

static mut my_tramp: usize = my_tramp1 as usize;
static mut tramps: [usize; 2] = [my_tramp1 as usize, my_tramp2 as usize];
static mut direct: ftrace_ops = ftrace_ops { _private: [] };

unsafe extern "C" fn simple_thread(_arg: *mut core::ffi::c_void) -> core::ffi::c_int {
    static mut t: core::ffi::c_int = 0;
    let mut ret: core::ffi::c_int = 0;

    while !kthread_should_stop() {
        set_current_state(TASK_INTERRUPTIBLE);
        schedule_timeout(2 * HZ);

        if ret != 0 {
            continue;
        }
        t ^= 1;
        ret = modify_ftrace_direct(&raw mut direct, tramps[t as usize]);
        if ret == 0 {
            my_tramp = tramps[t as usize];
        }
        // WARN_ON_ONCE(ret);
    }

    0
}

static mut simple_tsk: *mut task_struct = core::ptr::null_mut();

unsafe extern "C" fn ftrace_direct_multi_init() -> core::ffi::c_int {
    let ret: core::ffi::c_int;

    ftrace_set_filter_ip(&raw mut direct, wake_up_process as usize, 0, 0);
    ftrace_set_filter_ip(&raw mut direct, schedule as usize, 0, 0);

    ret = register_ftrace_direct(&raw mut direct, my_tramp);

    if ret == 0 {
        simple_tsk = kthread_run(simple_thread, core::ptr::null_mut(), c"event-sample-fn".as_ptr());
    }
    ret
}

unsafe extern "C" fn ftrace_direct_multi_exit() {
    kthread_stop(simple_tsk);
    unregister_ftrace_direct(&raw mut direct, my_tramp, true);
}

// module_init(ftrace_direct_multi_init);
// module_exit(ftrace_direct_multi_exit);
// MODULE_AUTHOR("Jiri Olsa");
// MODULE_DESCRIPTION("Example use case of using modify_ftrace_direct()");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
