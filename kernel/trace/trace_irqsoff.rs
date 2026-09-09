// SPDX-License-Identifier: GPL-2.0
// trace irqs off critical timings
// Translated from trace_irqsoff.c; external kernel declarations are supplied by dependencies.

#[cfg(any(CONFIG_IRQSOFF_TRACER, CONFIG_PREEMPT_TRACER))]
mod irqsoff {
    use super::*;

    static mut irqsoff_trace: *mut trace_array = core::ptr::null_mut();
    static mut tracer_enabled: i32 = 0;
    static mut tracing_cpu: i32 = 0; // DEFINE_PER_CPU(int, tracing_cpu)
    static mut max_trace_lock: raw_spinlock_t = unsafe { core::mem::zeroed() };
    const TRACER_IRQS_OFF: i32 = 1 << 1;
    const TRACER_PREEMPT_OFF: i32 = 1 << 2;
    static mut trace_type: i32 = 0;
    static mut save_flags: i32 = 0;
    static mut max_sequence: c_ulong = 0;

    unsafe fn preempt_trace(pc: i32) -> i32 {
        if cfg!(CONFIG_PREEMPT_TRACER) { ((trace_type & TRACER_PREEMPT_OFF) != 0 && pc != 0) as i32 } else { 0 }
    }
    unsafe fn irq_trace() -> i32 {
        if cfg!(CONFIG_IRQSOFF_TRACER) { ((trace_type & TRACER_IRQS_OFF) != 0 && irqs_disabled()) as i32 } else { 0 }
    }
    unsafe fn is_graph(tr: *mut trace_array) -> bool {
        if cfg!(CONFIG_FUNCTION_GRAPH_TRACER) { ((*tr).trace_flags & TRACE_ITER(DISPLAY_GRAPH)) != 0 } else { false }
    }

    unsafe fn func_prolog_dec(tr: *mut trace_array, data: *mut *mut trace_array_cpu, flags: *mut c_ulong) -> i32 {
        let cpu = raw_smp_processor_id();
        if per_cpu!(tracing_cpu, cpu) == 0 { return 0; }
        local_save_flags(flags);
        if !irqs_disabled_flags(*flags) && preempt_count() == 0 { return 0; }
        *data = per_cpu_ptr((*tr).array_buffer.data, cpu);
        let disabled = local_inc_return(&mut (**data).disabled);
        if disabled == 1 { 1 } else { local_dec(&mut (**data).disabled); 0 }
    }

    #[cfg(CONFIG_FUNCTION_TRACER)]
    unsafe extern "C" fn irqsoff_tracer_call(ip: c_ulong, parent_ip: c_ulong, _op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
        let tr = irqsoff_trace; let mut data = core::ptr::null_mut(); let mut flags = 0; 
        if func_prolog_dec(tr, &mut data, &mut flags) == 0 { return; }
        let ctx = tracing_gen_ctx_flags(flags); trace_function(tr, ip, parent_ip, ctx, fregs); local_dec(&mut (*data).disabled);
    }

    unsafe fn __trace_function(tr: *mut trace_array, ip: c_ulong, parent_ip: c_ulong, ctx: c_uint) {
        if is_graph(tr) { trace_graph_function(tr, ip, parent_ip, ctx); } else { trace_function(tr, ip, parent_ip, ctx, core::ptr::null_mut()); }
    }

    unsafe fn report_latency(tr: *mut trace_array, delta: u64) -> bool {
        if tracing_thresh != 0 { delta >= tracing_thresh } else { delta > (*tr).max_latency }
    }

    unsafe fn check_critical_timing(tr: *mut trace_array, data: *mut trace_array_cpu, parent_ip: c_ulong, cpu: i32) {
        let t0 = (*data).preempt_timestamp; let t1 = ftrace_now(cpu); let delta = t1 - t0; let ctx = tracing_gen_ctx();
        if !report_latency(tr, delta) { (*data).critical_sequence = max_sequence; (*data).preempt_timestamp = ftrace_now(cpu); __trace_function(tr, CALLER_ADDR0, parent_ip, ctx); return; }
        raw_spin_lock_irqsave(&mut max_trace_lock, &mut 0);
        if report_latency(tr, delta) { __trace_function(tr, CALLER_ADDR0, parent_ip, ctx); __trace_stack(tr, ctx, 5); if (*data).critical_sequence == max_sequence { (*data).critical_end = parent_ip; if !is_tracing_stopped() { (*tr).max_latency = delta; update_max_tr_single(tr, current, cpu); } max_sequence += 1; } }
        raw_spin_unlock_irqrestore(&mut max_trace_lock, 0);
        (*data).critical_sequence = max_sequence; (*data).preempt_timestamp = ftrace_now(cpu); __trace_function(tr, CALLER_ADDR0, parent_ip, ctx);
    }

    unsafe fn start_critical_timing(ip: c_ulong, parent_ip: c_ulong) {
        let tr = irqsoff_trace; if tracer_enabled == 0 || !tracing_is_enabled() { return; } let cpu = raw_smp_processor_id();
        if per_cpu!(tracing_cpu, cpu) != 0 { return; } let data = per_cpu_ptr((*tr).array_buffer.data, cpu); if data.is_null() || local_read(&(*data).disabled) != 0 { return; }
        if local_inc_return(&mut (*data).disabled) == 1 { (*data).critical_sequence = max_sequence; (*data).preempt_timestamp = ftrace_now(cpu); (*data).critical_start = if parent_ip != 0 { parent_ip } else { ip }; __trace_function(tr, ip, parent_ip, tracing_gen_ctx()); per_cpu_set!(tracing_cpu, cpu, 1); } local_dec(&mut (*data).disabled);
    }

    unsafe fn stop_critical_timing(ip: c_ulong, parent_ip: c_ulong) {
        let cpu = raw_smp_processor_id(); if per_cpu!(tracing_cpu, cpu) != 0 { per_cpu_set!(tracing_cpu, cpu, 0); } else { return; }
        let tr = irqsoff_trace; if tracer_enabled == 0 || !tracing_is_enabled() { return; } let data = per_cpu_ptr((*tr).array_buffer.data, cpu);
        if data.is_null() || (*data).critical_start == 0 || local_read(&(*data).disabled) != 0 { return; }
        if local_inc_return(&mut (*data).disabled) == 1 { let ctx = tracing_gen_ctx(); __trace_function(tr, ip, parent_ip, ctx); check_critical_timing(tr, data, if parent_ip != 0 { parent_ip } else { ip }, cpu); (*data).critical_start = 0; } local_dec(&mut (*data).disabled);
    }

    pub unsafe extern "C" fn start_critical_timings() { if preempt_trace(preempt_count()) != 0 || irq_trace() != 0 { start_critical_timing(CALLER_ADDR0, CALLER_ADDR1); } }
    pub unsafe extern "C" fn stop_critical_timings() { if preempt_trace(preempt_count()) != 0 || irq_trace() != 0 { stop_critical_timing(CALLER_ADDR0, CALLER_ADDR1); } }
    pub unsafe extern "C" fn tracer_hardirqs_on(a0: c_ulong, a1: c_ulong) { if preempt_trace(preempt_count()) == 0 && irq_trace() != 0 { stop_critical_timing(a0, a1); } }
    pub unsafe extern "C" fn tracer_hardirqs_off(a0: c_ulong, a1: c_ulong) { if preempt_trace(preempt_count()) == 0 && irq_trace() != 0 { start_critical_timing(a0, a1); } }
    pub unsafe extern "C" fn tracer_preempt_on(a0: c_ulong, a1: c_ulong) { if preempt_trace(preempt_count()) != 0 && irq_trace() == 0 { stop_critical_timing(a0, a1); } }
    pub unsafe extern "C" fn tracer_preempt_off(a0: c_ulong, a1: c_ulong) { if preempt_trace(preempt_count()) != 0 && irq_trace() == 0 { start_critical_timing(a0, a1); } }

    // Function-graph support and tracer registration retain the C interfaces and ordering.
    #[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
    unsafe fn irqsoff_display_graph(tr: *mut trace_array, set: i32) -> i32 {
        if is_graph(tr) == (set != 0) { return 0; }
        stop_irqsoff_tracer(irqsoff_trace, if set == 0 { 1 } else { 0 });
        for_each_possible_cpu!(cpu, { per_cpu_set!(tracing_cpu, cpu, 0); });
        (*tr).max_latency = 0; tracing_reset_online_cpus(&mut (*irqsoff_trace).array_buffer);
        start_irqsoff_tracer(irqsoff_trace, set)
    }

    unsafe fn start_irqsoff_tracer(tr: *mut trace_array, graph: i32) -> i32 {
        let ret = register_irqsoff_function(tr, graph, 0);
        tracer_enabled = if ret == 0 && tracing_is_enabled() { 1 } else { 0 }; ret
    }
    unsafe fn stop_irqsoff_tracer(_tr: *mut trace_array, graph: i32) { tracer_enabled = 0; unregister_irqsoff_function(irqsoff_trace, graph); }

    #[cfg(CONFIG_FUNCTION_TRACER)]
    static mut function_enabled: bool = false;
    unsafe fn register_irqsoff_function(tr: *mut trace_array, graph: i32, set: i32) -> i32 {
        #[cfg(CONFIG_FUNCTION_TRACER)] {
            if function_enabled || (set == 0 && ((*tr).trace_flags & TRACE_ITER(FUNCTION)) == 0) { return 0; }
            let ret = if graph != 0 { register_ftrace_graph(&mut fgraph_ops) } else { register_ftrace_function((*tr).ops) };
            if ret == 0 { function_enabled = true; } return ret;
        }
        0
    }
    unsafe fn unregister_irqsoff_function(tr: *mut trace_array, graph: i32) {
        #[cfg(CONFIG_FUNCTION_TRACER)] if function_enabled { if graph != 0 { unregister_ftrace_graph(&mut fgraph_ops); } else { unregister_ftrace_function((*tr).ops); } function_enabled = false; }
    }
    unsafe fn irqsoff_function_set(tr: *mut trace_array, mask: u32, set: i32) -> i32 { if mask & TRACE_ITER(FUNCTION) == 0 { return 0; } if set != 0 { register_irqsoff_function(tr, is_graph(tr) as i32, 1); } else { unregister_irqsoff_function(tr, is_graph(tr) as i32); } 1 }
    unsafe fn irqsoff_flag_changed(tr: *mut trace_array, mask: u64, set: i32) -> i32 { if irqsoff_function_set(tr, mask as u32, set) != 0 { return 0; } trace_keep_overwrite((*tr).current_trace, mask, set) }
    static mut irqsoff_busy: bool = false;
    unsafe fn __irqsoff_tracer_init(tr: *mut trace_array) -> i32 { if irqsoff_busy { return -EBUSY; } save_flags = (*tr).trace_flags; set_tracer_flag(tr, TRACE_ITER(OVERWRITE), 1); set_tracer_flag(tr, TRACE_ITER(LATENCY_FMT), 1); set_tracer_flag(tr, TRACE_ITER(PAUSE_ON_TRACE), 1); (*tr).max_latency = 0; irqsoff_trace = tr; smp_wmb(); ftrace_init_array_ops(tr, irqsoff_tracer_call); start_irqsoff_tracer(tr, 0); irqsoff_busy = true; 0 }
    unsafe fn __irqsoff_tracer_reset(tr: *mut trace_array) { stop_irqsoff_tracer(tr, is_graph(tr) as i32); set_tracer_flag(tr, TRACE_ITER(LATENCY_FMT), save_flags & TRACE_ITER(LATENCY_FMT)); set_tracer_flag(tr, TRACE_ITER(OVERWRITE), save_flags & TRACE_ITER(OVERWRITE)); set_tracer_flag(tr, TRACE_ITER(PAUSE_ON_TRACE), save_flags & TRACE_ITER(PAUSE_ON_TRACE)); ftrace_reset_array_ops(tr); irqsoff_busy = false; }
    unsafe fn irqsoff_tracer_start(_tr: *mut trace_array) { tracer_enabled = 1; }
    unsafe fn irqsoff_tracer_stop(_tr: *mut trace_array) { tracer_enabled = 0; }
    pub unsafe fn init_irqsoff_tracer() -> i32 { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
