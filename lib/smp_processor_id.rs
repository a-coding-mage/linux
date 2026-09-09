// SPDX-License-Identifier: GPL-2.0
/*
 * lib/smp_processor_id.c
 *
 * DEBUG_PREEMPT variant of smp_processor_id().
 */

// C dependencies: <linux/export.h>, <linux/kprobes.h>, and <linux/sched.h>.

use core::ffi::{c_char, c_int, c_uint, c_void};

// External kernel types and symbols supplied by the surrounding translation.
#[repr(C)]
pub struct TaskStruct {
    pub migration_disabled: c_int,
    pub comm: [c_char; 16],
    pub pid: c_int,
}

unsafe extern "C" {
    static mut current: *mut TaskStruct;
    static mut system_state: c_int;

    fn raw_smp_processor_id() -> c_int;
    fn preempt_count() -> c_uint;
    fn irqs_disabled() -> c_int;
    fn is_percpu_thread() -> c_int;
    fn preempt_disable_notrace();
    fn instrumentation_begin();
    fn printk_ratelimit() -> c_int;
    fn printk(format: *const c_char, ...);
    fn dump_stack();
    fn instrumentation_end();
    fn preempt_enable_no_resched_notrace();
    fn __builtin_return_address(level: c_uint) -> *mut c_void;
}

// SYSTEM_SCHEDULING and KERN_ERR are supplied by the kernel headers.
const SYSTEM_SCHEDULING: c_int = 1;

#[inline]
unsafe fn check_preemption_disabled(what1: *const c_char, what2: *const c_char) -> c_uint {
    let this_cpu = raw_smp_processor_id() as c_uint;

    if preempt_count() != 0 {
        return this_cpu;
    }

    if irqs_disabled() != 0 {
        return this_cpu;
    }

    if is_percpu_thread() != 0 {
        return this_cpu;
    }

    if (*current).migration_disabled != 0 {
        return this_cpu;
    }

    /*
     * It is valid to assume CPU-locality during early bootup:
     */
    if system_state < SYSTEM_SCHEDULING {
        return this_cpu;
    }

    /*
     * Avoid recursion:
     */
    preempt_disable_notrace();

    instrumentation_begin();
    if printk_ratelimit() == 0 {
        instrumentation_end();
        preempt_enable_no_resched_notrace();
        return this_cpu;
    }

    // printk(KERN_ERR "BUG: using %s%s() in preemptible [%08x] code: %s/%d\n", ...)
    printk(
        b"BUG: using %s%s() in preemptible [%08x] code: %s/%d\n\0".as_ptr() as *const c_char,
        what1,
        what2,
        preempt_count().wrapping_sub(1),
        (*current).comm.as_ptr(),
        (*current).pid,
    );

    printk(
        b"caller is %pS\n\0".as_ptr() as *const c_char,
        __builtin_return_address(0),
    );
    dump_stack();

    instrumentation_end();
    preempt_enable_no_resched_notrace();
    this_cpu
}

#[inline]
pub unsafe fn debug_smp_processor_id() -> c_uint {
    check_preemption_disabled(
        b"smp_processor_id\0".as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
    )
}

// EXPORT_SYMBOL(debug_smp_processor_id);

#[inline]
pub unsafe fn __this_cpu_preempt_check(op: *const c_char) {
    check_preemption_disabled(
        b"__this_cpu_\0".as_ptr() as *const c_char,
        op,
    );
}

// EXPORT_SYMBOL(__this_cpu_preempt_check);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
