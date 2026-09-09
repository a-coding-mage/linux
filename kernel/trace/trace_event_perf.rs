// SPDX-License-Identifier: GPL-2.0
/*
 * trace event based perf event profiling/tracing
 *
 * Copyright (C) 2009 Red Hat Inc, Peter Zijlstra
 * Copyright (C) 2009-2010 Frederic Weisbecker <fweisbec@gmail.com>
 */

// Kernel headers and symbols referenced by this translation are supplied by
// the surrounding kernel bindings.

static mut PERF_TRACE_BUF: [*mut core::ffi::c_char; PERF_NR_CONTEXTS] =
    [core::ptr::null_mut(); PERF_NR_CONTEXTS];

/* Force it to be aligned to unsigned long to avoid misaligned accesses. */
#[repr(C)]
struct PerfTrace([core::ffi::c_ulong; PERF_MAX_TRACE_SIZE / core::mem::size_of::<core::ffi::c_ulong>()]);

/* Count the events in use (per event id, not per instance). */
static mut TOTAL_REF_COUNT: core::ffi::c_int = 0;

unsafe fn perf_trace_event_perm(tp_event: *mut trace_event_call, p_event: *mut perf_event) -> i32 {
    let mut ret: i32;
    if (*tp_event).perf_perm.is_some() {
        ret = ((*tp_event).perf_perm.unwrap())(tp_event, p_event);
        if ret != 0 { return ret; }
    }
    if !(*p_event).parent.is_null() { return 0; }
    if ftrace_event_is_function(tp_event) {
        ret = perf_allow_tracepoint();
        if ret != 0 { return ret; }
        if !is_sampling_event(p_event) { return 0; }
        if !(*p_event).attr.exclude_callchain_user { return -EINVAL; }
        if (*p_event).attr.sample_type & PERF_SAMPLE_STACK_USER != 0 { return -EINVAL; }
    }
    if (*p_event).attr.sample_type & PERF_SAMPLE_RAW == 0 { return 0; }
    if (*p_event).attach_state == PERF_ATTACH_TASK && (*tp_event).flags & TRACE_EVENT_FL_CAP_ANY != 0 { return 0; }
    ret = perf_allow_tracepoint();
    if ret != 0 { return ret; }
    0
}

unsafe fn perf_trace_event_reg(tp_event: *mut trace_event_call, p_event: *mut perf_event) -> i32 {
    let mut list: *mut hlist_head = core::ptr::null_mut();
    let mut ret: i32 = -ENOMEM;
    let mut cpu: i32;
    (*p_event).tp_event = tp_event;
    (*tp_event).perf_refcount += 1;
    if (*tp_event).perf_refcount > 1 { return 0; }
    list = alloc_percpu::<hlist_head>();
    if list.is_null() { goto_fail(tp_event, ret); return ret; }
    for_each_possible_cpu!(cpu) { INIT_HLIST_HEAD(per_cpu_ptr(list, cpu)); }
    (*tp_event).perf_events = list;
    if TOTAL_REF_COUNT == 0 {
        for i in 0..PERF_NR_CONTEXTS {
            let buf = alloc_percpu::<PerfTrace>() as *mut core::ffi::c_char;
            if buf.is_null() { goto_fail(tp_event, ret); return ret; }
            PERF_TRACE_BUF[i] = buf;
        }
    }
    ret = ((*(*tp_event).class).reg)(tp_event, TRACE_REG_PERF_REGISTER, core::ptr::null_mut());
    if ret != 0 { goto_fail(tp_event, ret); return ret; }
    TOTAL_REF_COUNT += 1;
    0
}

unsafe fn goto_fail(tp_event: *mut trace_event_call, _ret: i32) {
    if TOTAL_REF_COUNT == 0 {
        for i in 0..PERF_NR_CONTEXTS { free_percpu(PERF_TRACE_BUF[i]); PERF_TRACE_BUF[i] = core::ptr::null_mut(); }
    }
    (*tp_event).perf_refcount -= 1;
    if (*tp_event).perf_refcount == 0 { free_percpu((*tp_event).perf_events); (*tp_event).perf_events = core::ptr::null_mut(); }
}

unsafe fn perf_trace_event_unreg(p_event: *mut perf_event) {
    let tp_event = (*p_event).tp_event;
    (*tp_event).perf_refcount -= 1;
    if (*tp_event).perf_refcount > 0 { return; }
    ((*(*tp_event).class).reg)(tp_event, TRACE_REG_PERF_UNREGISTER, core::ptr::null_mut());
    tracepoint_synchronize_unregister();
    free_percpu((*tp_event).perf_events); (*tp_event).perf_events = core::ptr::null_mut();
    TOTAL_REF_COUNT -= 1;
    if TOTAL_REF_COUNT == 0 { for i in 0..PERF_NR_CONTEXTS { free_percpu(PERF_TRACE_BUF[i]); PERF_TRACE_BUF[i] = core::ptr::null_mut(); } }
}

unsafe fn perf_trace_event_open(p_event: *mut perf_event) -> i32 { let t = (*p_event).tp_event; ((*(*t).class).reg)(t, TRACE_REG_PERF_OPEN, p_event) }
unsafe fn perf_trace_event_close(p_event: *mut perf_event) { let t = (*p_event).tp_event; ((*(*t).class).reg)(t, TRACE_REG_PERF_CLOSE, p_event); }

unsafe fn perf_trace_event_init(t: *mut trace_event_call, p: *mut perf_event) -> i32 {
    let mut ret = perf_trace_event_perm(t, p); if ret != 0 { return ret; }
    ret = perf_trace_event_reg(t, p); if ret != 0 { return ret; }
    ret = perf_trace_event_open(p); if ret != 0 { perf_trace_event_unreg(p); return ret; } 0
}

pub unsafe fn perf_trace_init(p_event: *mut perf_event) -> i32 {
    let event_id = (*p_event).attr.config; let mut ret = -EINVAL;
    mutex_lock(&event_mutex);
    list_for_each_entry!(tp_event, &ftrace_events, list) {
        if (*tp_event).event.r#type == event_id && !(*tp_event).class.is_null() && (*(*tp_event).class).reg.is_some() && trace_event_try_get_ref(tp_event) {
            ret = perf_trace_event_init(tp_event, p_event); if ret != 0 { trace_event_put_ref(tp_event); } break;
        }
    }
    mutex_unlock(&event_mutex); ret
}

pub unsafe fn perf_trace_destroy(p: *mut perf_event) { mutex_lock(&event_mutex); perf_trace_event_close(p); perf_trace_event_unreg(p); trace_event_put_ref((*p).tp_event); mutex_unlock(&event_mutex); }

#[cfg(feature = "CONFIG_KPROBE_EVENTS")]
pub unsafe fn perf_kprobe_init(p: *mut perf_event, is_retprobe: bool) -> i32 {
    let mut ret; let mut func: *mut i8 = core::ptr::null_mut();
    if (*p).attr.kprobe_func != 0 { func = strndup_user(u64_to_user_ptr((*p).attr.kprobe_func), KSYM_NAME_LEN); if IS_ERR(func) { ret = PTR_ERR(func); return if ret == -EINVAL { -E2BIG } else { ret }; } if *func == 0 { kfree(func); func = core::ptr::null_mut(); } }
    let t = create_local_trace_kprobe(func, (*p).attr.kprobe_addr as *mut _, (*p).attr.probe_offset, is_retprobe);
    if IS_ERR(t) { ret = PTR_ERR(t); } else { mutex_lock(&event_mutex); ret = perf_trace_event_init(t, p); if ret != 0 { destroy_local_trace_kprobe(t); } mutex_unlock(&event_mutex); }
    kfree(func); ret
}
#[cfg(feature = "CONFIG_KPROBE_EVENTS")]
pub unsafe fn perf_kprobe_destroy(p: *mut perf_event) { mutex_lock(&event_mutex); perf_trace_event_close(p); perf_trace_event_unreg(p); trace_event_put_ref((*p).tp_event); mutex_unlock(&event_mutex); destroy_local_trace_kprobe((*p).tp_event); }

#[cfg(feature = "CONFIG_UPROBE_EVENTS")]
pub unsafe fn perf_uprobe_init(p: *mut perf_event, ref_ctr_offset: usize, is_retprobe: bool) -> i32 {
    if (*p).attr.uprobe_path == 0 { return -EINVAL; }
    let path = strndup_user(u64_to_user_ptr((*p).attr.uprobe_path), PATH_MAX); if IS_ERR(path) { let r = PTR_ERR(path); return if r == -EINVAL { -E2BIG } else { r }; }
    if *path == 0 { kfree(path); return -EINVAL; }
    let t = create_local_trace_uprobe(path, (*p).attr.probe_offset, ref_ctr_offset, is_retprobe); let mut ret;
    if IS_ERR(t) { ret = PTR_ERR(t); } else { mutex_lock(&event_mutex); ret = perf_trace_event_init(t, p); if ret != 0 { destroy_local_trace_uprobe(t); } mutex_unlock(&event_mutex); }
    kfree(path); ret
}
#[cfg(feature = "CONFIG_UPROBE_EVENTS")]
pub unsafe fn perf_uprobe_destroy(p: *mut perf_event) { mutex_lock(&event_mutex); perf_trace_event_close(p); perf_trace_event_unreg(p); trace_event_put_ref((*p).tp_event); mutex_unlock(&event_mutex); destroy_local_trace_uprobe((*p).tp_event); }

pub unsafe fn perf_trace_add(p: *mut perf_event, flags: i32) -> i32 {
    let t = (*p).tp_event; let hwc = &mut (*p).hw;
    if flags & PERF_EF_START == 0 { (*p).hw.state = PERF_HES_STOPPED; }
    if is_sampling_event(p) { hwc.last_period = hwc.sample_period; perf_swevent_set_period(p); }
    if !(((*(*t).class).reg)(t, TRACE_REG_PERF_ADD, p) != 0) { let pcpu = (*t).perf_events; if WARN_ON_ONCE(pcpu.is_null()) { return -EINVAL; } hlist_add_head_rcu(&mut (*p).hlist_entry, this_cpu_ptr(pcpu)); } 0
}
pub unsafe fn perf_trace_del(p: *mut perf_event, _flags: i32) { let t = (*p).tp_event; if ((*(*t).class).reg)(t, TRACE_REG_PERF_DEL, p) == 0 { hlist_del_rcu(&mut (*p).hlist_entry); } }

pub unsafe fn perf_trace_buf_alloc(size: i32, regs: *mut *mut pt_regs, rctxp: *mut i32) -> *mut core::ffi::c_void {
    BUILD_BUG_ON!(PERF_MAX_TRACE_SIZE % core::mem::size_of::<usize>() != 0);
    if WARN_ONCE(size > PERF_MAX_TRACE_SIZE, "perf buffer not large enough, wanted %d, have %d", size, PERF_MAX_TRACE_SIZE) { return core::ptr::null_mut(); }
    let rctx = perf_swevent_get_recursion_context(); *rctxp = rctx; if rctx < 0 { return core::ptr::null_mut(); }
    if !regs.is_null() { *regs = this_cpu_ptr(&mut __perf_regs[rctx as usize]); }
    let raw = this_cpu_ptr(PERF_TRACE_BUF[rctx as usize]);
    core::ptr::write_bytes(raw.add(size as usize - core::mem::size_of::<u64>()), 0, core::mem::size_of::<u64>()); raw as *mut _
}
pub unsafe fn perf_trace_buf_update(record: *mut core::ffi::c_void, typ: u16) { tracing_generic_entry_update(record as *mut trace_entry, typ, tracing_gen_ctx()); }

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
unsafe fn perf_ftrace_function_call(ip: usize, parent_ip: usize, ops: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    if !rcu_is_watching() { return; } let bit = ftrace_test_recursion_trylock(ip, parent_ip); if bit < 0 { return; }
    if (*ops).private as usize != smp_processor_id() as usize { ftrace_test_recursion_unlock(bit); return; }
    let event = container_of!(ops, perf_event, ftrace_ops); let mut head = hlist_head { first: &mut (*event).hlist_entry };
    const ENTRY_SIZE: usize = (ALIGN!(core::mem::size_of::<ftrace_entry>() + core::mem::size_of::<u32>(), core::mem::size_of::<u64>()) - core::mem::size_of::<u32>());
    BUILD_BUG_ON!(ENTRY_SIZE > PERF_MAX_TRACE_SIZE); let mut regs = core::mem::zeroed::<pt_regs>(); perf_fetch_caller_regs(&mut regs);
    let mut rctx = 0; let entry = perf_trace_buf_alloc(ENTRY_SIZE as i32, core::ptr::null_mut(), &mut rctx) as *mut ftrace_entry;
    if !entry.is_null() { (*entry).ip = ip; (*entry).parent_ip = parent_ip; perf_trace_buf_submit(entry as *mut _, ENTRY_SIZE as i32, rctx, TRACE_FN, 1, &mut regs, &mut head, core::ptr::null_mut()); }
    ftrace_test_recursion_unlock(bit);
}

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
unsafe fn perf_ftrace_function_register(event: *mut perf_event) -> i32 { (*event).ftrace_ops.func = Some(perf_ftrace_function_call); (*event).ftrace_ops.private = nr_cpu_ids as *mut _; register_ftrace_function(&mut (*event).ftrace_ops) }
#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
unsafe fn perf_ftrace_function_unregister(event: *mut perf_event) -> i32 { let ops = &mut (*event).ftrace_ops; let mut ret = 0; if ops.flags & FTRACE_OPS_FL_ENABLED != 0 { ret = unregister_ftrace_function(ops); } ftrace_free_filter(ops); ret }

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
pub unsafe fn perf_ftrace_event_register(_call: *mut trace_event_call, typ: trace_reg, data: *mut core::ffi::c_void) -> i32 {
    match typ { TRACE_REG_REGISTER | TRACE_REG_UNREGISTER => {}, TRACE_REG_PERF_REGISTER | TRACE_REG_PERF_UNREGISTER => return 0, TRACE_REG_PERF_OPEN => return perf_ftrace_function_register(data as *mut _), TRACE_REG_PERF_CLOSE => return perf_ftrace_function_unregister(data as *mut _), TRACE_REG_PERF_ADD => { (*(data as *mut perf_event)).ftrace_ops.private = smp_processor_id() as *mut _; return 1; }, TRACE_REG_PERF_DEL => { (*(data as *mut perf_event)).ftrace_ops.private = nr_cpu_ids as *mut _; return 1; } }
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
