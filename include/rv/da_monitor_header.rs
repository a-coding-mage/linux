/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of rv/da_monitor.h.  Kernel and generated-model symbols are external. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* Includes and preprocessor hooks from the C header are supplied by the kernel/model. */

static mut rv_this: rv_monitor = unsafe { core::mem::zeroed() };

#[inline]
unsafe fn react(curr_state: states, event: events) {
    rv_react(&mut rv_this, concat!("rv: monitor ", stringify!(MONITOR_NAME),
        " does not allow event ", "%s", " on state ", "%s\n"),
        model_get_event_name(event), model_get_state_name(curr_state));
}

#[inline]
unsafe fn da_monitor_reset_state(da_mon: *mut da_monitor) {
    core::ptr::write_volatile(&mut (*da_mon).monitoring, 0);
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
    (*da_mon).curr_state = model_get_initial_state();
}

#[inline]
unsafe fn da_monitor_reset(da_mon: *mut da_monitor) {
    da_monitor_reset_hook!(da_mon);
    da_monitor_reset_state(da_mon);
}

#[inline]
unsafe fn da_monitor_start(da_mon: *mut da_monitor) {
    (*da_mon).curr_state = model_get_initial_state();
    da_monitor_init_hook!(da_mon);
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
    core::ptr::write_volatile(&mut (*da_mon).monitoring, 1);
}

#[inline]
unsafe fn da_monitoring(da_mon: *mut da_monitor) -> bool {
    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
    core::ptr::read_volatile(&(*da_mon).monitoring) != 0
}

#[inline]
unsafe fn da_monitor_enabled() -> bool {
    if !rv_monitoring_on() || !rv_this.enabled { return false; }
    true
}

#[inline]
unsafe fn da_monitor_handling_event(da_mon: *mut da_monitor) -> bool {
    da_monitor_enabled() && da_monitoring(da_mon)
}

/* RV_MON_TYPE == RV_MON_GLOBAL */
#[cfg(RV_MON_GLOBAL)]
static mut DA_MON_NAME: da_monitor = unsafe { core::mem::zeroed() };

#[cfg(RV_MON_GLOBAL)]
unsafe fn da_get_monitor() -> *mut da_monitor { &raw mut DA_MON_NAME }

#[cfg(RV_MON_GLOBAL)]
unsafe fn __da_monitor_reset_all(reset: unsafe fn(*mut da_monitor)) { reset(da_get_monitor()); }

#[cfg(RV_MON_GLOBAL)]
unsafe fn da_monitor_reset_all() { __da_monitor_reset_all(da_monitor_reset); }
#[cfg(RV_MON_GLOBAL)]
unsafe fn da_monitor_reset_state_all() { __da_monitor_reset_all(da_monitor_reset_state); }
#[cfg(RV_MON_GLOBAL)]
unsafe fn da_monitor_init() -> i32 { da_monitor_reset_state_all(); 0 }
#[cfg(RV_MON_GLOBAL)]
unsafe fn da_monitor_destroy() { da_monitor_reset_all(); da_monitor_sync_hook!(); }

/* RV_MON_TYPE == RV_MON_PER_CPU. Per-CPU access is provided by the kernel. */
#[cfg(RV_MON_PER_CPU)]
unsafe fn da_get_monitor() -> *mut da_monitor { this_cpu_ptr(DA_MON_NAME) }
#[cfg(RV_MON_PER_CPU)]
unsafe fn __da_monitor_reset_all(reset: unsafe fn(*mut da_monitor)) {
    for cpu in cpu_online_mask() { reset(per_cpu_ptr(DA_MON_NAME, cpu)); }
}
#[cfg(RV_MON_PER_CPU)]
unsafe fn da_monitor_reset_all() { __da_monitor_reset_all(da_monitor_reset); }
#[cfg(RV_MON_PER_CPU)]
unsafe fn da_monitor_reset_state_all() { __da_monitor_reset_all(da_monitor_reset_state); }
#[cfg(RV_MON_PER_CPU)]
unsafe fn da_monitor_init() -> i32 { da_monitor_reset_state_all(); 0 }
#[cfg(RV_MON_PER_CPU)]
unsafe fn da_monitor_destroy() { da_monitor_reset_all(); da_monitor_sync_hook!(); }

/* RV_MON_TYPE == RV_MON_PER_TASK */
#[cfg(RV_MON_PER_TASK)]
static mut task_mon_slot: i32 = RV_PER_TASK_MONITOR_INIT;
#[cfg(RV_MON_PER_TASK)]
unsafe fn da_get_monitor(tsk: *mut task_struct) -> *mut da_monitor { &mut (*tsk).rv[task_mon_slot as usize].da_mon }
#[cfg(RV_MON_PER_TASK)]
unsafe fn da_reset(tsk: *mut task_struct) { da_monitor_reset(da_get_monitor(tsk)); }
#[cfg(RV_MON_PER_TASK)]
unsafe fn da_get_target(da_mon: *mut da_monitor) -> *mut task_struct { container_of_task(da_mon, task_mon_slot) }
#[cfg(RV_MON_PER_TASK)]
unsafe fn da_get_id(da_mon: *mut da_monitor) -> da_id_type { (*da_get_target(da_mon)).pid as da_id_type }
#[cfg(RV_MON_PER_TASK)]
unsafe fn da_monitor_init() -> i32 { let slot = rv_get_task_monitor_slot(); if slot < 0 || slot >= RV_PER_TASK_MONITOR_INIT { return slot; } task_mon_slot = slot; da_monitor_reset_state_all(); 0 }

/* RV_MON_TYPE == RV_MON_PER_OBJ */
#[repr(C)]
struct da_monitor_storage { id: da_id_type, target: monitor_target, rv: rv_task_monitor, node: hlist_node, rcu: rcu_head }
#[cfg(RV_MON_PER_OBJ)]
static mut da_monitor_ht: hashtable = unsafe { core::mem::zeroed() };
#[cfg(RV_MON_PER_OBJ)]
unsafe fn da_get_monitor(id: da_id_type, _target: monitor_target) -> *mut da_monitor { let s = __da_get_mon_storage(id); if s.is_null() { core::ptr::null_mut() } else { &mut (*s).rv.da_mon } }
#[cfg(RV_MON_PER_OBJ)]
unsafe fn __da_get_mon_storage(_id: da_id_type) -> *mut da_monitor_storage { core::ptr::null_mut() /* hash_for_each_possible_rcu */ }
#[cfg(RV_MON_PER_OBJ)]
unsafe fn da_get_target(da_mon: *mut da_monitor) -> monitor_target { container_of_storage(da_mon).target }
#[cfg(RV_MON_PER_OBJ)]
unsafe fn da_get_id(da_mon: *mut da_monitor) -> da_id_type { container_of_storage(da_mon).id }
#[cfg(RV_MON_PER_OBJ)]
unsafe fn da_monitor_init() -> i32 { hash_init!(&raw mut da_monitor_ht); 0 }

#[inline]
unsafe fn da_event(da_mon: *mut da_monitor, event: events, id: da_id_type) -> bool {
    let mut curr_state = core::ptr::read_volatile(&(*da_mon).curr_state);
    for _ in 0..MAX_DA_RETRY_RACING_EVENTS {
        let next_state = model_get_next_state(curr_state, event);
        if next_state == INVALID_STATE { react(curr_state, event); da_trace_error(da_mon, model_get_state_name(curr_state), model_get_event_name(event), id); return false; }
        if try_cmpxchg(&mut (*da_mon).curr_state, &mut curr_state, next_state) {
            if !da_monitor_event_hook!(da_mon, curr_state, event, next_state, id) { return false; }
            da_trace_event(da_mon, model_get_state_name(curr_state), model_get_event_name(event), model_get_state_name(next_state), model_is_final_state(next_state), id);
            return true;
        }
    }
    trace_rv_retries_error!(stringify!(MONITOR_NAME), model_get_event_name(event));
    pr_warn!("rv: {} retries reached for event %s, resetting monitor {}", MAX_DA_RETRY_RACING_EVENTS, model_get_event_name(event), stringify!(MONITOR_NAME));
    false
}

#[inline]
unsafe fn __da_handle_event_common(da_mon: *mut da_monitor, event: events, id: da_id_type) { if !da_event(da_mon, event, id) { da_monitor_reset(da_mon); } }
#[inline]
unsafe fn __da_handle_event(da_mon: *mut da_monitor, event: events, id: da_id_type) { if da_monitor_handling_event(da_mon) { __da_handle_event_common(da_mon, event, id); } }
#[inline]
unsafe fn __da_handle_start_event(da_mon: *mut da_monitor, event: events, id: da_id_type) -> bool { if !da_monitor_enabled() { return false; } if !da_monitoring(da_mon) { da_monitor_start(da_mon); return false; } __da_handle_event_common(da_mon, event, id); true }
#[inline]
unsafe fn __da_handle_start_run_event(da_mon: *mut da_monitor, event: events, id: da_id_type) -> bool { if !da_monitor_enabled() { return false; } if !da_monitoring(da_mon) { da_monitor_start(da_mon); } __da_handle_event_common(da_mon, event, id); true }

/* Generated trace-event functions and kernel types are intentionally external. */
unsafe fn da_trace_event(_m: *mut da_monitor, _c: *mut i8, _e: *mut i8, _n: *mut i8, _f: bool, _id: da_id_type) { trace_event!(MONITOR_NAME, _c, _e, _n, _f); }
unsafe fn da_trace_error(_m: *mut da_monitor, _c: *mut i8, _e: *mut i8, _id: da_id_type) { trace_error!(MONITOR_NAME, _c, _e); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
