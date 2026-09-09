// SPDX-License-Identifier: GPL-2.0-or-later
// Kernel Live Patching transition functions.

const MAX_STACK_ENTRIES: usize = 100;
const STACK_ERR_BUF_SIZE: usize = 128;
const SIGNALS_TIMEOUT: u32 = 15;

// These kernel types, constants, macros, and functions are supplied by the
// surrounding kernel translation.
extern "C" {
    static mut klp_transition_patch: *mut klp_patch;
    static mut klp_target_state: i32;
    static mut klp_signals_cnt: u32;
}

#[repr(C)] pub struct klp_patch { pub mod_: *mut klp_module, pub replace: bool, pub enabled: bool, pub forced: bool }
#[repr(C)] pub struct klp_module { pub name: *const u8 }
#[repr(C)] pub struct klp_object { pub patched: bool }
#[repr(C)] pub struct klp_func { pub transition: bool, pub new_func: *const core::ffi::c_void, pub new_size: usize, pub old_func: *const core::ffi::c_void, pub old_size: usize, pub old_name: *const u8 }
#[repr(C)] pub struct klp_ops { pub func_stack: list_head }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { pub patch_state: i32, pub flags: usize, pub comm: [u8; 16], pub pid: i32 }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }

const KLP_TRANSITION_IDLE: i32 = 0;
const KLP_TRANSITION_PATCHED: i32 = 1;
const KLP_TRANSITION_UNPATCHED: i32 = 0;
const EAGAIN: i32 = 11;
const EINVAL: i32 = 22;
const EBUSY: i32 = 16;
const EADDRINUSE: i32 = 98;

static mut klp_stack_entries: [[usize; MAX_STACK_ENTRIES]; 1] = [[0; MAX_STACK_ENTRIES]; 1];

unsafe fn klp_transition_work_fn(_work: *mut work_struct) {
    mutex_lock();
    if !klp_transition_patch.is_null() { klp_try_complete_transition(); }
    mutex_unlock();
}
unsafe fn klp_sync(_work: *mut work_struct) {}

unsafe fn klp_synchronize_transition() { schedule_on_each_cpu(klp_sync); }

unsafe fn klp_complete_transition() {
    let mut obj: *mut klp_object;
    let mut func: *mut klp_func;
    let mut g: *mut task_struct;
    let mut task: *mut task_struct;
    let mut cpu: u32;
    if (*klp_transition_patch).replace && klp_target_state == KLP_TRANSITION_PATCHED {
        klp_unpatch_replaced_patches(klp_transition_patch); klp_discard_nops(klp_transition_patch);
    }
    if klp_target_state == KLP_TRANSITION_UNPATCHED { klp_unpatch_objects(klp_transition_patch); klp_synchronize_transition(); }
    klp_for_each_object!(klp_transition_patch, obj, { klp_for_each_func!(obj, func, { (*func).transition = false; }); });
    if klp_target_state == KLP_TRANSITION_PATCHED { klp_synchronize_transition(); }
    read_lock();
    for_each_process_thread!(g, task, { WARN_ON_ONCE(test_tsk_thread_flag(task)); (*task).patch_state = KLP_TRANSITION_IDLE; });
    read_unlock();
    for_each_possible_cpu!(cpu, { task = idle_task(cpu); WARN_ON_ONCE(test_tsk_thread_flag(task)); (*task).patch_state = KLP_TRANSITION_IDLE; });
    klp_for_each_object!(klp_transition_patch, obj, { if klp_is_object_loaded(obj) { if klp_target_state == KLP_TRANSITION_PATCHED { klp_post_patch_callback(obj); } else { klp_post_unpatch_callback(obj); } } });
    klp_target_state = KLP_TRANSITION_IDLE; klp_transition_patch = core::ptr::null_mut();
}

pub unsafe fn klp_cancel_transition() { if klp_target_state != KLP_TRANSITION_PATCHED { return; } klp_target_state = KLP_TRANSITION_UNPATCHED; klp_complete_transition(); }

pub unsafe fn klp_update_patch_state(task: *mut task_struct) {
    preempt_disable_notrace();
    if test_and_clear_tsk_thread_flag(task) { (*task).patch_state = klp_target_state; }
    preempt_enable_notrace();
}

unsafe fn klp_check_stack_func(func: *mut klp_func, entries: *mut usize, nr_entries: i32) -> i32 {
    let (addr, size) = if klp_target_state == KLP_TRANSITION_UNPATCHED { ((*func).new_func as usize, (*func).new_size) } else {
        let ops = klp_find_ops((*func).old_func); if list_is_singular(&(*ops).func_stack) { ((*func).old_func as usize, (*func).old_size) } else { let prev = list_next_entry(func); ((*prev).new_func as usize, (*prev).new_size) }
    };
    for i in 0..nr_entries { let a = *entries.offset(i as isize); if a >= addr && a < addr.wrapping_add(size) { return -EAGAIN; } }
    0
}

unsafe fn klp_check_stack(task: *mut task_struct, oldname: *mut *const u8) -> i32 {
    let entries = klp_stack_entries[0].as_mut_ptr(); let mut obj: *mut klp_object; let mut func: *mut klp_func;
    let ret = stack_trace_save_tsk_reliable(task, entries, MAX_STACK_ENTRIES as u32); if ret < 0 { return -EINVAL; }
    klp_for_each_object!(klp_transition_patch, obj, { if (*obj).patched { klp_for_each_func!(obj, func, { if klp_check_stack_func(func, entries, ret) != 0 { *oldname = (*func).old_name; return -EADDRINUSE; } }); } }); 0
}

unsafe fn klp_check_and_switch_task(task: *mut task_struct, arg: *mut core::ffi::c_void) -> i32 { if task_curr(task) && task != current() { return -EBUSY; } let r = klp_check_stack(task, arg as *mut _); if r != 0 { return r; } clear_tsk_thread_flag(task); (*task).patch_state = klp_target_state; 0 }
unsafe fn klp_try_switch_task(task: *mut task_struct) -> bool { if (*task).patch_state == klp_target_state { return true; } if !klp_have_reliable_stack() { return false; } let mut n: *const u8 = core::ptr::null(); let r = if task == current() { klp_check_and_switch_task(task, &mut n as *mut _ as *mut _) } else { task_call_func(task, klp_check_and_switch_task, &mut n as *mut _ as *mut _) }; r == 0 }

pub unsafe fn __klp_sched_try_switch() { lockdep_assert_preemption_disabled(); let c = current(); if klp_patch_pending(c) { smp_rmb(); klp_try_switch_task(c); } }

unsafe fn klp_send_signals() { if klp_signals_cnt == SIGNALS_TIMEOUT { } let mut g: *mut task_struct; let mut task: *mut task_struct; read_lock(); for_each_process_thread!(g, task, { if klp_patch_pending(task) { if (*task).flags & PF_KTHREAD != 0 { wake_up_state(task, TASK_INTERRUPTIBLE); } else { set_notify_signal(task); } } }); read_unlock(); }

pub unsafe fn klp_try_complete_transition() {
    let mut cpu: u32; let mut g: *mut task_struct; let mut task: *mut task_struct; let patch: *mut klp_patch; let mut complete = true;
    read_lock(); for_each_process_thread!(g, task, { if !klp_try_switch_task(task) { complete = false; } }); read_unlock();
    cpus_read_lock(); for_each_possible_cpu!(cpu, { task = idle_task(cpu); if cpu_online(cpu) { if !klp_try_switch_task(task) { complete = false; wake_up_if_idle(cpu); } } else if (*task).patch_state != klp_target_state { clear_tsk_thread_flag(task); (*task).patch_state = klp_target_state; } }); cpus_read_unlock();
    if !complete { if klp_signals_cnt != 0 && klp_signals_cnt % SIGNALS_TIMEOUT == 0 { klp_send_signals(); } klp_signals_cnt += 1; schedule_delayed_work(); return; }
    klp_resched_disable(); patch = klp_transition_patch; klp_complete_transition(); if !(*patch).enabled { klp_free_patch_async(patch); } else if (*patch).replace { klp_free_replaced_patches_async(patch); }
}

pub unsafe fn klp_start_transition() { let mut g: *mut task_struct; let mut task: *mut task_struct; let mut cpu: u32; read_lock(); for_each_process_thread!(g, task, { if (*task).patch_state != klp_target_state { set_tsk_thread_flag(task); } }); read_unlock(); for_each_possible_cpu!(cpu, { task = idle_task(cpu); if (*task).patch_state != klp_target_state { set_tsk_thread_flag(task); } }); klp_resched_enable(); klp_signals_cnt = 0; }

pub unsafe fn klp_init_transition(patch: *mut klp_patch, state: i32) { let mut g: *mut task_struct; let mut task: *mut task_struct; let mut cpu: u32; let mut obj: *mut klp_object; let mut func: *mut klp_func; klp_transition_patch = patch; klp_target_state = state; let initial_state = if state == 0 { 1 } else { 0 }; read_lock(); for_each_process_thread!(g, task, { (*task).patch_state = initial_state; }); read_unlock(); for_each_possible_cpu!(cpu, { task = idle_task(cpu); (*task).patch_state = initial_state; }); smp_wmb(); klp_for_each_object!(patch, obj, { klp_for_each_func!(obj, func, { (*func).transition = true; }); }); }

pub unsafe fn klp_reverse_transition() { let mut cpu: u32; let mut g: *mut task_struct; let mut task: *mut task_struct; read_lock(); for_each_process_thread!(g, task, { clear_tsk_thread_flag(task); }); read_unlock(); for_each_possible_cpu!(cpu, { clear_tsk_thread_flag(idle_task(cpu)); }); klp_synchronize_transition(); (*klp_transition_patch).enabled = !(*klp_transition_patch).enabled; klp_target_state = !klp_target_state; smp_wmb(); klp_start_transition(); }

pub unsafe fn klp_copy_process(child: *mut task_struct) { if test_tsk_thread_flag(current()) { set_tsk_thread_flag(child); } else { clear_tsk_thread_flag(child); } (*child).patch_state = (*current()).patch_state; }

pub unsafe fn klp_force_transition() { let mut cpu: u32; let mut g: *mut task_struct; let mut task: *mut task_struct; read_lock(); for_each_process_thread!(g, task, { klp_update_patch_state(task); }); read_unlock(); for_each_possible_cpu!(cpu, { klp_update_patch_state(idle_task(cpu)); }); if klp_target_state == KLP_TRANSITION_UNPATCHED { (*klp_transition_patch).forced = true; } else if (*klp_transition_patch).replace { let mut patch: *mut klp_patch; klp_for_each_patch!(patch, { if patch != klp_transition_patch { (*patch).forced = true; } }); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
