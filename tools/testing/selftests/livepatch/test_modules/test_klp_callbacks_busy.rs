// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Joe Lawrence <joe.lawrence@redhat.com>

// C dependency intent:
// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// #include <linux/module.h>
// #include <linux/kernel.h>
// #include <linux/sched.h>
// #include <linux/workqueue.h>
// #include <linux/delay.h>

use core::ffi::{c_char, c_uint, c_void};

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn complete(x: *mut completion);
    fn msleep(msecs: c_uint);
    fn schedule_work(work: *mut work_struct) -> bool;
    fn wait_for_completion(x: *mut completion);
    fn flush_work(work: *mut work_struct) -> bool;
    fn pr_info(fmt: *const c_char, ...);
}

/* load/run-time control from sysfs writer  */
static mut block_transition: bool = false;
// module_param(block_transition, bool, 0644);
// MODULE_PARM_DESC(block_transition, "block_transition (default=false)");

unsafe extern "C" {
    // static DECLARE_WORK(work, busymod_work_func);
    static mut work: work_struct;

    // static DECLARE_COMPLETION(busymod_work_started);
    static mut busymod_work_started: completion;
}

unsafe extern "C" fn busymod_work_func(_work: *mut work_struct) {
    unsafe {
        pr_info(c"%s enter\n".as_ptr(), c"busymod_work_func".as_ptr());
        complete(&raw mut busymod_work_started);

        while core::ptr::read_volatile(&raw const block_transition) {
            /*
             * Busy-wait until the sysfs writer has acknowledged a
             * blocked transition and clears the flag.
             */
            msleep(20);
        }

        pr_info(c"%s exit\n".as_ptr(), c"busymod_work_func".as_ptr());
    }
}

unsafe extern "C" fn test_klp_callbacks_busy_init() -> i32 {
    unsafe {
        pr_info(c"%s\n".as_ptr(), c"test_klp_callbacks_busy_init".as_ptr());
        schedule_work(&raw mut work);

        /*
         * To synchronize kernel messages, hold the init function from
         * exiting until the work function's entry message has printed.
         */
        wait_for_completion(&raw mut busymod_work_started);

        if !block_transition {
            /*
             * Serialize output: print all messages from the work
             * function before returning from init().
             */
            flush_work(&raw mut work);
        }

        0
    }
}

unsafe extern "C" fn test_klp_callbacks_busy_exit() {
    unsafe {
        core::ptr::write_volatile(&raw mut block_transition, false);
        flush_work(&raw mut work);
        pr_info(c"%s\n".as_ptr(), c"test_klp_callbacks_busy_exit".as_ptr());
    }
}

// module_init(test_klp_callbacks_busy_init);
// module_exit(test_klp_callbacks_busy_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Joe Lawrence <joe.lawrence@redhat.com>");
// MODULE_DESCRIPTION("Livepatch test: busy target module");


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
