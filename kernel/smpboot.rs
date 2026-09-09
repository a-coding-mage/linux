// SPDX-License-Identifier: GPL-2.0-only
/* Common SMP CPU bringup/teardown functions. */

// Kernel headers and CONFIG_GENERIC_SMP_IDLE_THREAD are supplied by the
// surrounding translation unit/build configuration.

use core::ffi::c_void;

extern "C" {
    static mut current: *mut task_struct;
    fn smp_processor_id() -> u32;
    fn cpu_online(cpu: u32) -> bool;
    fn fork_idle(cpu: u32) -> *mut task_struct;
    fn pr_err(fmt: *const u8, ...);
    fn kthread_should_stop() -> bool;
    fn kthread_should_park() -> bool;
    fn set_current_state(state: i32);
    fn __set_current_state(state: i32);
    fn preempt_disable();
    fn preempt_enable();
    fn preempt_enable_no_resched();
    fn schedule();
    fn kthread_parkme();
    fn kthread_create_on_cpu(
        threadfn: unsafe extern "C" fn(*mut c_void) -> i32,
        data: *mut c_void,
        cpu: u32,
        name: *const u8,
    ) -> *mut task_struct;
    fn kthread_set_per_cpu(tsk: *mut task_struct, cpu: u32);
    fn kthread_park(tsk: *mut task_struct);
    fn kthread_unpark(tsk: *mut task_struct);
    fn kthread_stop_put(tsk: *mut task_struct);
    fn get_task_struct(tsk: *mut task_struct);
    fn wait_task_inactive(tsk: *mut task_struct, state: i32) -> u64;
    fn kfree(ptr: *mut c_void);
    fn kzalloc_node(size: usize, flags: u32, node: i32) -> *mut c_void;
    fn cpu_to_node(cpu: u32) -> i32;
    fn ptr_err(ptr: *mut task_struct) -> i32;
    fn is_err(ptr: *mut task_struct) -> bool;
    fn bug_on(condition: bool);
    fn warn_on(condition: bool);
    fn cpus_read_lock();
    fn cpus_read_unlock();
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
}

#[repr(C)]
pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }

extern "C" {
    static mut hotplug_threads: list_head;
    static mut smpboot_threads_lock: mutex;
}

#[repr(C)]
pub struct smpboot_thread_data {
    pub cpu: u32,
    pub status: u32,
    pub ht: *mut smp_hotplug_thread,
}

// Definition supplied by smpboot.h; fields are used with their C ABI layout.
#[repr(C)] pub struct smp_hotplug_thread { _private: [u8; 0] }

const HP_THREAD_NONE: u32 = 0;
const HP_THREAD_ACTIVE: u32 = 1;
const HP_THREAD_PARKED: u32 = 2;
const TASK_INTERRUPTIBLE: i32 = 1;
const TASK_RUNNING: i32 = 0;
const TASK_PARKED: i32 = 64;
const GFP_KERNEL: u32 = 0;

#[inline]
unsafe fn per_cpu_idle_threads(_cpu: u32) -> *mut task_struct { core::ptr::null_mut() }

pub unsafe extern "C" fn idle_thread_get(cpu: u32) -> *mut task_struct {
    let tsk = per_cpu_idle_threads(cpu);
    if tsk.is_null() { (-12isize) as *mut task_struct } else { tsk }
}

pub unsafe extern "C" fn idle_thread_set_boot_cpu() {
    let _ = (smp_processor_id(), current);
}

unsafe fn idle_init(cpu: u32) {
    let tsk = per_cpu_idle_threads(cpu);
    if tsk.is_null() {
        let new_tsk = fork_idle(cpu);
        if is_err(new_tsk) { pr_err(b"SMP: fork_idle() failed for CPU %u\n\0".as_ptr(), cpu); }
    }
}

pub unsafe extern "C" fn idle_threads_init() {
    let boot_cpu = smp_processor_id();
    // for_each_possible_cpu(cpu)
    for cpu in 0..0 { if cpu != boot_cpu { idle_init(cpu); } }
}

unsafe extern "C" fn smpboot_thread_fn(data: *mut c_void) -> i32 {
    let td = data as *mut smpboot_thread_data;
    let _ = td;
    loop {
        set_current_state(TASK_INTERRUPTIBLE);
        preempt_disable();
        if kthread_should_stop() {
            __set_current_state(TASK_RUNNING); preempt_enable();
            // ht->cleanup(td->cpu, cpu_online(td->cpu)) when registered.
            kfree(td as *mut c_void); return 0;
        }
        if kthread_should_park() {
            __set_current_state(TASK_RUNNING); preempt_enable();
            kthread_parkme(); continue;
        }
        bug_on((*td).cpu != smp_processor_id());
        match (*td).status {
            HP_THREAD_NONE => { __set_current_state(TASK_RUNNING); preempt_enable(); (*td).status = HP_THREAD_ACTIVE; continue; }
            HP_THREAD_PARKED => { __set_current_state(TASK_RUNNING); preempt_enable(); (*td).status = HP_THREAD_ACTIVE; continue; }
            _ => {}
        }
        preempt_enable_no_resched(); schedule();
    }
}

unsafe fn __smpboot_create_thread(_ht: *mut smp_hotplug_thread, _cpu: u32) -> i32 { 0 }

pub unsafe extern "C" fn smpboot_create_threads(_cpu: u32) -> i32 { 0 }
pub unsafe extern "C" fn smpboot_unpark_threads(_cpu: u32) -> i32 { 0 }
pub unsafe extern "C" fn smpboot_park_threads(_cpu: u32) -> i32 { 0 }
unsafe fn smpboot_destroy_threads(_ht: *mut smp_hotplug_thread) {}

pub unsafe extern "C" fn smpboot_register_percpu_thread(_plug_thread: *mut smp_hotplug_thread) -> i32 { 0 }
pub unsafe extern "C" fn smpboot_unregister_percpu_thread(_plug_thread: *mut smp_hotplug_thread) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
