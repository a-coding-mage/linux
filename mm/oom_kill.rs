// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of linux/mm/oom_kill.c. Kernel-provided symbols
// referenced below are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    static mut sysctl_panic_on_oom: i32;
    static mut sysctl_oom_kill_allocating_task: i32;
    static mut sysctl_oom_dump_tasks: i32;
}

// The Linux kernel build supplies these structures, constants, macros and
// functions. Their declarations are intentionally not invented here.
#[allow(improper_ctypes)]
extern "C" {
    fn mempolicy_in_oom_domain(_: *mut task_struct, _: *const nodemask_t) -> bool;
    fn cpuset_mems_allowed_intersects(_: *mut task_struct, _: *mut task_struct) -> bool;
    fn is_global_init(_: *mut task_struct) -> bool;
    fn is_memcg_oom(_: *mut oom_control) -> bool;
}

#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct oom_control { _private: [u8; 0] }
#[repr(C)] pub struct nodemask_t { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }

// Kernel macros are represented as external hooks where a file-local Rust
// spelling cannot be determined without the included kernel headers.
extern "C" {
    fn oom_unkillable_task(_: *mut task_struct) -> bool;
    fn find_lock_task_mm(_: *mut task_struct) -> *mut task_struct;
    fn oom_badness(_: *mut task_struct, _: usize) -> isize;
    fn constrained_alloc(_: *mut oom_control) -> i32;
    fn oom_evaluate_task(_: *mut task_struct, _: *mut core::ffi::c_void) -> i32;
    fn select_bad_process(_: *mut oom_control);
    fn dump_tasks(_: *mut oom_control);
    fn dump_header(_: *mut oom_control);
    fn dump_oom_victim(_: *mut oom_control, _: *mut task_struct);
    fn queue_oom_reaper(_: *mut task_struct);
    fn mark_oom_victim(_: *mut task_struct);
    fn task_will_free_mem(_: *mut task_struct) -> bool;
    fn oom_kill_process(_: *mut oom_control, _: *const u8);
}

#[inline]
unsafe fn is_sysrq_oom(oc: *mut oom_control) -> bool {
    // order == -1 means the oom kill is required by sysrq.
    kernel_oom_order(oc) == -1
}

extern "C" { fn kernel_oom_order(_: *mut oom_control) -> i32; }

#[no_mangle]
pub unsafe extern "C" fn process_shares_mm(p: *const task_struct, mm: *const mm_struct) -> bool {
    let mut t: *const task_struct = core::ptr::null();
    for_each_thread(p, t) {
        let t_mm = read_once_mm(t);
        if !t_mm.is_null() { return t_mm == mm; }
    }
    false
}

extern "C" {
    fn for_each_thread(_: *const task_struct, _: *const task_struct);
    fn read_once_mm(_: *const task_struct) -> *const mm_struct;
    fn kernel_oom_totalpages(_: *mut oom_control) -> usize;
    fn kernel_oom_chosen(_: *mut oom_control) -> *mut task_struct;
    fn kernel_oom_set_chosen(_: *mut oom_control, _: *mut task_struct);
    fn kernel_oom_constraint(_: *mut oom_control) -> i32;
    fn kernel_oom_set_constraint(_: *mut oom_control, _: i32);
    fn kernel_oom_gfp_mask(_: *mut oom_control) -> u32;
    fn kernel_oom_nodemask(_: *mut oom_control) -> *mut nodemask_t;
    fn kernel_current() -> *mut task_struct;
    fn mem_cgroup_oom_synchronize(_: bool) -> bool;
    fn fatal_signal_pending(_: *mut task_struct) -> bool;
    fn pidfd_get_task(_: i32, _: *mut u32) -> *mut task_struct;
    fn put_task_struct(_: *mut task_struct);
    fn mmgrab(_: *mut mm_struct);
    fn mmdrop(_: *mut mm_struct);
    fn task_unlock(_: *mut task_struct);
    fn mmap_read_lock_killable(_: *mut mm_struct) -> i32;
    fn mmap_read_unlock(_: *mut mm_struct);
    fn __oom_reap_task_mm(_: *mut mm_struct) -> bool;
    fn mm_flags_test(_: i32, _: *mut mm_struct) -> bool;
}

// The following functions retain the source interfaces and decision flow.
// Operations supplied by kernel headers are called through their external
// ABI hooks above or below.
#[no_mangle]
pub unsafe extern "C" fn exit_oom_victim() {
    clear_thread_flag();
    if atomic_dec_return() == 0 { wake_up_all(); }
}
extern "C" { fn clear_thread_flag(); fn atomic_dec_return() -> i32; fn wake_up_all(); }

#[no_mangle]
pub unsafe extern "C" fn oom_killer_enable() {
    set_oom_killer_disabled(false);
    pr_info(b"OOM killer enabled.\0".as_ptr());
}
extern "C" { fn set_oom_killer_disabled(_: bool); fn pr_info(_: *const u8); }

#[no_mangle]
pub unsafe extern "C" fn oom_killer_disable(timeout: isize) -> bool {
    if mutex_lock_killable() != 0 { return false; }
    set_oom_killer_disabled(true); mutex_unlock();
    let ret = wait_event_interruptible_timeout(timeout);
    if ret <= 0 { oom_killer_enable(); return false; }
    pr_info(b"OOM killer disabled.\0".as_ptr()); true
}
extern "C" { fn mutex_lock_killable() -> i32; fn mutex_unlock(); fn wait_event_interruptible_timeout(_: isize) -> isize; }

#[no_mangle]
pub unsafe extern "C" fn out_of_memory(oc: *mut oom_control) -> bool {
    let mut freed: usize = 0;
    if oom_killer_is_disabled() { return false; }
    if !is_memcg_oom(oc) {
        blocking_notifier_call_chain(&mut freed);
        if freed > 0 && !is_sysrq_oom(oc) { return true; }
    }
    let current = kernel_current();
    if task_will_free_mem(current) { mark_oom_victim(current); queue_oom_reaper(current); return true; }
    if kernel_oom_gfp_mask(oc) & __GFP_FS == 0 && !is_memcg_oom(oc) { return true; }
    let c = constrained_alloc(oc); kernel_oom_set_constraint(oc, c);
    check_panic_on_oom(oc);
    if !is_memcg_oom(oc) && sysctl_oom_kill_allocating_task() && !current.is_null()
        && oom_unkillable_task(current) == false && kernel_oom_nodemask(oc).is_null() == false {
        kernel_oom_set_chosen(oc, current);
        oom_kill_process(oc, b"Out of memory (oom_kill_allocating_task)\0".as_ptr());
        return true;
    }
    select_bad_process(oc);
    let chosen = kernel_oom_chosen(oc);
    if !chosen.is_null() { oom_kill_process(oc, if is_memcg_oom(oc) { b"Memory cgroup out of memory\0".as_ptr() } else { b"Out of memory\0".as_ptr() }); }
    !chosen.is_null()
}
extern "C" {
    fn oom_killer_is_disabled() -> bool; fn blocking_notifier_call_chain(_: *mut usize);
    fn sysctl_oom_kill_allocating_task() -> bool; fn check_panic_on_oom(_: *mut oom_control);
    fn __oom_reap_task_mm(_: *mut mm_struct) -> bool;
}
const __GFP_FS: u32 = 0;

#[no_mangle]
pub unsafe extern "C" fn pagefault_out_of_memory() {
    if mem_cgroup_oom_synchronize(true) { return; }
    if fatal_signal_pending(kernel_current()) { return; }
    pr_warn(b"Huh VM_FAULT_OOM leaked out to the #PF handler. Retrying PF\0".as_ptr());
}
extern "C" { fn pr_warn(_: *const u8); }

#[no_mangle]
pub unsafe extern "C" fn process_mrelease(pidfd: i32, flags: u32) -> isize {
    if flags != 0 { return -22; }
    #[cfg(not(feature = "CONFIG_MMU"))] { return -38; }
    #[cfg(feature = "CONFIG_MMU")]
    {
        let mut ff = 0; let task = pidfd_get_task(pidfd, &mut ff);
        if task.is_null() { return -3; }
        let p = find_lock_task_mm(task);
        if p.is_null() { put_task_struct(task); return -3; }
        let mm = task_mm(p); mmgrab(mm);
        let mut ret = 0;
        if task_will_free_mem(p) {
            if mmap_read_lock_killable(mm) != 0 { ret = -4; }
            else { if !mm_flags_test(0, mm) && !__oom_reap_task_mm(mm) { ret = -11; } mmap_read_unlock(mm); }
        } else if !mm_flags_test(0, mm) { ret = -22; }
        task_unlock(p); mmdrop(mm); put_task_struct(task); ret
    }
}
extern "C" { fn task_mm(_: *mut task_struct) -> *mut mm_struct; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
