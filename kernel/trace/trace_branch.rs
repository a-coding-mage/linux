// SPDX-License-Identifier: GPL-2.0
/*
 * unlikely profiler
 *
 * Copyright (C) 2008 Steven Rostedt <srostedt@redhat.com>
 */

#[cfg(CONFIG_BRANCH_TRACER)]
static mut branch_trace: tracer = tracer { name: "branch", ..unsafe { core::mem::zeroed() } };
#[cfg(CONFIG_BRANCH_TRACER)]
static mut branch_tracing_enabled: i32 = 0;
#[cfg(CONFIG_BRANCH_TRACER)]
static mut branch_tracing_mutex: mutex = mutex { };
#[cfg(CONFIG_BRANCH_TRACER)]
static mut branch_tracer: *mut trace_array = core::ptr::null_mut();

#[cfg(CONFIG_BRANCH_TRACER)]
unsafe fn probe_likely_condition(f: *mut ftrace_likely_data, val: i32, expect: i32) {
    let tr = branch_tracer;
    let mut flags: ulong = 0;
    let mut trace_ctx: uint;
    let mut p: *const c_char;

    if (*current).trace_recursion & TRACE_BRANCH_BIT != 0 { return; }
    if tr.is_null() { return; }

    raw_local_irq_save(&mut flags);
    (*current).trace_recursion |= TRACE_BRANCH_BIT;
    if !tracer_tracing_is_on_cpu(tr, raw_smp_processor_id()) { goto_out!(); }

    trace_ctx = tracing_gen_ctx_flags(flags);
    let buffer = (*tr).array_buffer.buffer;
    let event = trace_buffer_lock_reserve(buffer, TRACE_BRANCH, core::mem::size_of::<trace_branch>(), trace_ctx);
    if event.is_null() { goto_out!(); }

    let entry = ring_buffer_event_data(event) as *mut trace_branch;
    p = (*f).data.file.add(strlen((*f).data.file));
    while p >= (*f).data.file && *p != b'/' as c_char { p = p.sub(1); }
    p = p.add(1);

    strscpy((*entry).func.as_mut_ptr(), (*f).data.func);
    strscpy((*entry).file.as_mut_ptr(), p);
    (*entry).constant = (*f).constant;
    (*entry).line = (*f).data.line;
    (*entry).correct = (val == expect) as _;
    trace_buffer_unlock_commit_nostack(buffer, event);

    (*current).trace_recursion &= !TRACE_BRANCH_BIT;
    raw_local_irq_restore(flags);
}

#[cfg(CONFIG_BRANCH_TRACER)]
#[inline]
unsafe fn trace_likely_condition(f: *mut ftrace_likely_data, val: i32, expect: i32) {
    if branch_tracing_enabled == 0 { return; }
    probe_likely_condition(f, val, expect);
}

#[cfg(CONFIG_BRANCH_TRACER)]
unsafe fn enable_branch_tracing(tr: *mut trace_array) -> i32 {
    mutex_lock(&mut branch_tracing_mutex);
    branch_tracer = tr;
    smp_wmb();
    branch_tracing_enabled += 1;
    mutex_unlock(&mut branch_tracing_mutex);
    0
}

#[cfg(CONFIG_BRANCH_TRACER)]
unsafe fn disable_branch_tracing() {
    mutex_lock(&mut branch_tracing_mutex);
    if branch_tracing_enabled != 0 { branch_tracing_enabled -= 1; }
    mutex_unlock(&mut branch_tracing_mutex);
}

#[cfg(CONFIG_BRANCH_TRACER)]
unsafe fn branch_trace_init(tr: *mut trace_array) -> i32 { enable_branch_tracing(tr) }
#[cfg(CONFIG_BRANCH_TRACER)]
unsafe fn branch_trace_reset(_tr: *mut trace_array) { disable_branch_tracing(); }

#[cfg(CONFIG_BRANCH_TRACER)]
unsafe fn trace_branch_print(iter: *mut trace_iterator, _flags: i32, _event: *mut trace_event) -> print_line_t {
    let field = trace_assign_type((*iter).ent) as *mut trace_branch;
    trace_seq_printf(&mut (*iter).seq, "[%s] %s:%s:%d\n", if (*field).correct { "  ok  " } else { " MISS " }, (*field).func.as_ptr(), (*field).file.as_ptr(), (*field).line);
    trace_handle_return(&mut (*iter).seq)
}

#[cfg(CONFIG_BRANCH_TRACER)]
unsafe fn branch_print_header(s: *mut seq_file) {
    seq_puts(s, "#           TASK-PID    CPU#    TIMESTAMP  CORRECT  FUNC:FILE:LINE\n#              | |       |          |             |\n");
}

#[cfg(not(CONFIG_BRANCH_TRACER))]
#[inline]
unsafe fn trace_likely_condition(_f: *mut ftrace_likely_data, _val: i32, _expect: i32) {}

unsafe fn ftrace_likely_update(f: *mut ftrace_likely_data, mut val: i32, expect: i32, is_constant: i32) {
    let flags = user_access_save();
    if is_constant != 0 { (*f).constant += 1; val = expect; }
    trace_likely_condition(f, val, expect);
    if val == expect { (*f).data.correct += 1; } else { (*f).data.incorrect += 1; }
    user_access_restore(flags);
}

extern "C" {
    static __start_annotated_branch_profile: ulong;
    static __stop_annotated_branch_profile: ulong;
    #[cfg(CONFIG_PROFILE_ALL_BRANCHES)] static __start_branch_profile: ulong;
    #[cfg(CONFIG_PROFILE_ALL_BRANCHES)] static __stop_branch_profile: ulong;
}

unsafe fn get_incorrect_percent(p: *const ftrace_branch_data) -> long {
    if (*p).correct != 0 { (*p).incorrect * 100 / ((*p).correct + (*p).incorrect) }
    else if (*p).incorrect != 0 { 100 } else { -1 }
}

unsafe fn branch_stat_process_file(p: *mut ftrace_branch_data) -> *const c_char {
    let mut f = (*p).file.as_ptr().add(strlen((*p).file.as_ptr()));
    while f >= (*p).file.as_ptr() && *f != b'/' as c_char { f = f.sub(1); }
    f.add(1)
}

unsafe fn branch_stat_show(m: *mut seq_file, p: *mut ftrace_branch_data, f: *const c_char) {
    let percent = get_incorrect_percent(p);
    if percent < 0 { seq_puts(m, "  X "); } else { seq_printf(m, "%3ld ", percent); }
    seq_printf(m, "%-30.30s %-20.20s %d\n", (*p).func.as_ptr(), f, (*p).line);
}

unsafe fn branch_stat_show_normal(m: *mut seq_file, p: *mut ftrace_branch_data, f: *const c_char) -> i32 {
    seq_printf(m, "%8lu %8lu ", (*p).correct, (*p).incorrect);
    branch_stat_show(m, p, f); 0
}

unsafe fn annotate_branch_stat_show(m: *mut seq_file, v: *mut c_void) -> i32 {
    let p = v as *mut ftrace_likely_data;
    let f = branch_stat_process_file(&mut (*p).data);
    if (*p).constant == 0 { return branch_stat_show_normal(m, &mut (*p).data, f); }
    let mut l = snprintf_len("/%lu", (*p).constant);
    l = if l > 8 { 0 } else { 8 - l };
    seq_printf(m, "%8lu/%lu %*lu ", (*p).data.correct, (*p).constant, l, (*p).data.incorrect);
    branch_stat_show(m, &mut (*p).data, f); 0
}

unsafe fn annotated_branch_stat_cmp(p1: *const c_void, p2: *const c_void) -> i32 {
    let a = p1 as *const ftrace_branch_data; let b = p2 as *const ftrace_branch_data;
    let pa = get_incorrect_percent(a); let pb = get_incorrect_percent(b);
    if pa < pb { return -1; } if pa > pb { return 1; }
    if (*a).incorrect < (*b).incorrect { return -1; } if (*a).incorrect > (*b).incorrect { return 1; }
    if (*a).correct > (*b).correct { -1 } else if (*a).correct < (*b).correct { 1 } else { 0 }
}

unsafe fn annotated_branch_stat_headers(m: *mut seq_file) -> i32 {
    seq_puts(m, " correct incorrect  %        Function                  File              Line\n ------- ---------  -        --------                  ----              ----\n"); 0
}

unsafe fn annotated_branch_stat_start(_trace: *mut tracer_stat) -> *mut c_void {
    &__start_annotated_branch_profile as *const _ as *mut c_void
}

unsafe fn annotated_branch_stat_next(v: *mut c_void, _idx: i32) -> *mut c_void {
    let p = (v as *mut ftrace_likely_data).add(1);
    if (p as *mut c_void) >= (&__stop_annotated_branch_profile as *const _ as *mut c_void) { core::ptr::null_mut() } else { p as *mut c_void }
}

#[cfg(CONFIG_PROFILE_ALL_BRANCHES)]
unsafe fn all_branch_stat_headers(m: *mut seq_file) -> i32 {
    seq_puts(m, "   miss      hit    %        Function                  File              Line\n ------- ---------  -        --------                  ----              ----\n"); 0
}

#[cfg(CONFIG_PROFILE_ALL_BRANCHES)]
unsafe fn all_branch_stat_start(_trace: *mut tracer_stat) -> *mut c_void {
    &__start_branch_profile as *const _ as *mut c_void
}

#[cfg(CONFIG_PROFILE_ALL_BRANCHES)]
unsafe fn all_branch_stat_next(v: *mut c_void, _idx: i32) -> *mut c_void {
    let p = (v as *mut ftrace_branch_data).add(1);
    if (p as *mut c_void) >= (&__stop_branch_profile as *const _ as *mut c_void) { core::ptr::null_mut() } else { p as *mut c_void }
}

#[cfg(CONFIG_PROFILE_ALL_BRANCHES)]
unsafe fn all_branch_stat_show(m: *mut seq_file, v: *mut c_void) -> i32 {
    let p = v as *mut ftrace_branch_data;
    branch_stat_show_normal(m, p, branch_stat_process_file(p))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
