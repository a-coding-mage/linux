/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the Linux kernel smpboot header.
// Dependency supplied externally: linux/types.h

use core::ffi::c_char;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

/* Cookie handed to the thread_fn */
#[repr(C)]
pub struct smpboot_thread_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

/**
 * struct smp_hotplug_thread - CPU hotplug related thread descriptor
 * @store:              Pointer to per cpu storage for the task pointers
 * @list:               List head for core management
 * @thread_should_run:  Check whether the thread should run or not. Called with
 *                      preemption disabled.
 * @thread_fn:          The associated thread function
 * @create:             Optional setup function, called when the thread gets
 *                      created (Not called from the thread context)
 * @setup:              Optional setup function, called when the thread gets
 *                      operational the first time
 * @cleanup:            Optional cleanup function, called when the thread
 *                      should stop (module exit)
 * @park:               Optional park function, called when the thread is
 *                      parked (cpu offline)
 * @unpark:             Optional unpark function, called when the thread is
 *                      unparked (cpu online)
 * @selfparking:        Thread is not parked by the park function.
 * @thread_comm:        The base name of the thread
 */
#[repr(C)]
pub struct smp_hotplug_thread {
    pub store: *mut *mut task_struct,
    pub list: list_head,
    pub thread_should_run: Option<unsafe extern "C" fn(cpu: u32) -> i32>,
    pub thread_fn: Option<unsafe extern "C" fn(cpu: u32)>,
    pub create: Option<unsafe extern "C" fn(cpu: u32)>,
    pub setup: Option<unsafe extern "C" fn(cpu: u32)>,
    pub cleanup: Option<unsafe extern "C" fn(cpu: u32, online: bool)>,
    pub park: Option<unsafe extern "C" fn(cpu: u32)>,
    pub unpark: Option<unsafe extern "C" fn(cpu: u32)>,
    pub selfparking: bool,
    pub thread_comm: *const c_char,
}

unsafe extern "C" {
    pub fn smpboot_register_percpu_thread(
        plug_thread: *mut smp_hotplug_thread,
    ) -> i32;

    pub fn smpboot_unregister_percpu_thread(
        plug_thread: *mut smp_hotplug_thread,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
