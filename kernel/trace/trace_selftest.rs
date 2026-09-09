// SPDX-License-Identifier: GPL-2.0
// Include in trace.c

// External kernel types, constants, macros, and functions are supplied by the
// surrounding kernel translation unit.

#[inline]
unsafe fn trace_valid_entry(entry: *mut trace_entry) -> i32 {
    match (*entry).type_ {
        TRACE_FN | TRACE_CTX | TRACE_WAKE | TRACE_STACK | TRACE_PRINT |
        TRACE_BRANCH | TRACE_GRAPH_ENT | TRACE_GRAPH_RETADDR_ENT |
        TRACE_GRAPH_RET => 1,
        _ => 0,
    }
}

unsafe fn trace_test_buffer_cpu(buf: *mut array_buffer, cpu: i32) -> i32 {
    let mut event: *mut ring_buffer_event;
    let mut loops: u32 = 0;
    while { event = ring_buffer_consume((*buf).buffer, cpu, core::ptr::null_mut(), core::ptr::null_mut()); !event.is_null() } {
        let entry = ring_buffer_event_data(event);
        if { loops = loops.wrapping_add(1); loops } > trace_buf_size {
            printk(KERN_CONT.as_ptr(), ".. bad ring buffer ");
            break;
        }
        if trace_valid_entry(entry) == 0 {
            printk(KERN_CONT.as_ptr(), ".. invalid entry %d ", (*entry).type_);
            break;
        }
    }
    if event.is_null() { return 0; }
    tracing_disabled = 1;
    printk(KERN_CONT.as_ptr(), ".. corrupted trace buffer .. ");
    -1
}

unsafe fn trace_test_buffer(buf: *mut array_buffer, count: *mut u64) -> i32 {
    let mut flags: u64 = 0;
    let mut cnt: u64 = 0;
    let mut ret = 0;
    local_irq_save(&mut flags);
    arch_spin_lock(&mut (*(*buf).tr).max_lock);
    cnt = ring_buffer_entries((*buf).buffer);
    tracing_off();
    for_each_possible_cpu!(cpu, {
        ret = trace_test_buffer_cpu(buf, cpu);
        if ret != 0 { break; }
    });
    tracing_on();
    arch_spin_unlock(&mut (*(*buf).tr).max_lock);
    local_irq_restore(flags);
    if !count.is_null() { *count = cnt; }
    ret
}

unsafe fn warn_failed_init_tracer(trace: *mut tracer, init_ret: i32) {
    printk(KERN_WARNING.as_ptr(), "Failed to init %s tracer, init returned %d\n", (*trace).name, init_ret);
}

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
mod function_tracer {
    use super::*;
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    static mut trace_selftest_test_probe1_cnt: i32 = 0;
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    static mut trace_selftest_test_probe2_cnt: i32 = 0;
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    static mut trace_selftest_test_probe3_cnt: i32 = 0;
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    static mut trace_selftest_test_global_cnt: i32 = 0;
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    static mut trace_selftest_test_dyn_cnt: i32 = 0;

    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    unsafe fn trace_selftest_test_probe1_func(_: u64, _: u64, _: *mut ftrace_ops, _: *mut ftrace_regs) { trace_selftest_test_probe1_cnt += 1; }
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    unsafe fn trace_selftest_test_probe2_func(_: u64, _: u64, _: *mut ftrace_ops, _: *mut ftrace_regs) { trace_selftest_test_probe2_cnt += 1; }
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    unsafe fn trace_selftest_test_probe3_func(_: u64, _: u64, _: *mut ftrace_ops, _: *mut ftrace_regs) { trace_selftest_test_probe3_cnt += 1; }
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    unsafe fn trace_selftest_test_global_func(_: u64, _: u64, _: *mut ftrace_ops, _: *mut ftrace_regs) { trace_selftest_test_global_cnt += 1; }
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    unsafe fn trace_selftest_test_dyn_func(_: u64, _: u64, _: *mut ftrace_ops, _: *mut ftrace_regs) { trace_selftest_test_dyn_cnt += 1; }

    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    static mut test_probe1: ftrace_ops = ftrace_ops { func: Some(trace_selftest_test_probe1_func), ..ftrace_ops::ZERO };
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    static mut test_probe2: ftrace_ops = ftrace_ops { func: Some(trace_selftest_test_probe2_func), ..ftrace_ops::ZERO };
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    static mut test_probe3: ftrace_ops = ftrace_ops { func: Some(trace_selftest_test_probe3_func), ..ftrace_ops::ZERO };

    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    unsafe fn print_counts() { printk(core::ptr::null(), "(%d %d %d %d %d) ", trace_selftest_test_probe1_cnt, trace_selftest_test_probe2_cnt, trace_selftest_test_probe3_cnt, trace_selftest_test_global_cnt, trace_selftest_test_dyn_cnt); }
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    unsafe fn reset_counts() { trace_selftest_test_probe1_cnt=0; trace_selftest_test_probe2_cnt=0; trace_selftest_test_probe3_cnt=0; trace_selftest_test_global_cnt=0; trace_selftest_test_dyn_cnt=0; }

    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    unsafe fn trace_selftest_ops(tr: *mut trace_array, cnt: i32) -> i32 {
        let save = ftrace_enabled; ftrace_enabled = 1; reset_counts();
        let mut n1 = concat!("*", stringify!(DYN_FTRACE_TEST_NAME));
        let n2 = concat!("*", stringify!(DYN_FTRACE_TEST_NAME2));
        ftrace_set_filter(&mut test_probe1, n1.as_ptr(), n1.len(), 1); ftrace_set_filter(&mut test_probe2, n2.as_ptr(), n2.len(), 1);
        ftrace_set_filter(&mut test_probe3, n1.as_ptr(), n1.len(), 1); ftrace_set_filter(&mut test_probe3, n2.as_ptr(), n2.len(), 0);
        register_ftrace_function(&mut test_probe1); register_ftrace_function(&mut test_probe2); register_ftrace_function(&mut test_probe3);
        if cnt > 1 { ftrace_init_array_ops(tr, Some(trace_selftest_test_global_func)); register_ftrace_function((*tr).ops); }
        DYN_FTRACE_TEST_NAME!(); print_counts();
        if trace_selftest_test_probe1_cnt != 1 || trace_selftest_test_probe2_cnt != 0 || trace_selftest_test_probe3_cnt != 1 || (cnt > 1 && trace_selftest_test_global_cnt == 0) { ftrace_enabled=save; return -1; }
        DYN_FTRACE_TEST_NAME2!(); print_counts();
        if trace_selftest_test_probe1_cnt != 1 || trace_selftest_test_probe2_cnt != 1 || trace_selftest_test_probe3_cnt != 2 { ftrace_enabled=save; return -1; }
        let dyn_ops = kzalloc_obj::<ftrace_ops>(); if dyn_ops.is_null() { printk(core::ptr::null(), "MEMORY ERROR "); ftrace_enabled=save; return -1; }
        (*dyn_ops).func = Some(trace_selftest_test_dyn_func); register_ftrace_function(dyn_ops); trace_selftest_test_global_cnt=0;
        DYN_FTRACE_TEST_NAME!(); print_counts(); DYN_FTRACE_TEST_NAME2!(); print_counts();
        n1 = concat!("!", stringify!(DYN_FTRACE_TEST_NAME)); ftrace_set_filter(&mut test_probe3, n1.as_ptr(), n1.len(), 0);
        DYN_FTRACE_TEST_NAME!(); print_counts(); DYN_FTRACE_TEST_NAME2!(); print_counts();
        unregister_ftrace_function(dyn_ops); kfree(dyn_ops); unregister_ftrace_function(&mut test_probe1); unregister_ftrace_function(&mut test_probe2); unregister_ftrace_function(&mut test_probe3);
        if cnt > 1 { unregister_ftrace_function((*tr).ops); } ftrace_reset_array_ops(tr); reset_counts(); DYN_FTRACE_TEST_NAME!(); DYN_FTRACE_TEST_NAME!();
        let ret = if trace_selftest_test_probe1_cnt!=0 || trace_selftest_test_probe2_cnt!=0 || trace_selftest_test_probe3_cnt!=0 || trace_selftest_test_global_cnt!=0 || trace_selftest_test_dyn_cnt!=0 { -1 } else { 0 }; ftrace_enabled=save; ret
    }

    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    unsafe fn trace_selftest_startup_dynamic_tracing(trace: *mut tracer, tr: *mut trace_array, func: unsafe extern "C" fn()) -> i32 { let save=ftrace_enabled; ftrace_enabled=1; func(); let name=concat!("*",stringify!(DYN_FTRACE_TEST_NAME)); ftrace_set_global_filter(name.as_ptr(),name.len(),1); let mut ret=tracer_init(trace,tr); if ret!=0 { warn_failed_init_tracer(trace,ret); } else { msleep(100); let mut count=0; ret=trace_test_buffer(&mut (*tr).array_buffer,&mut count); if ret==0 && count!=0 { ret=-1; } } if ret==0 { ret=trace_selftest_ops(tr,1); } if ret==0 { ret=trace_selftest_ops(tr,2); } ftrace_enabled=save; ftrace_set_global_filter(core::ptr::null(),0,1); ret }

    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    unsafe fn trace_selftest_function_recursion() -> i32 { 0 }
    #[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE"))]
    unsafe fn trace_selftest_startup_dynamic_tracing(_: *mut tracer, _: *mut trace_array, _: unsafe extern "C" fn()) -> i32 { 0 }
    #[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE"))]
    unsafe fn trace_selftest_function_recursion() -> i32 { 0 }

    pub unsafe fn trace_selftest_startup_function(trace: *mut tracer, tr: *mut trace_array) -> i32 {
        let save=ftrace_enabled; let mut count=0; msleep(1); ftrace_enabled=1; let mut ret=tracer_init(trace,tr);
        if ret==0 { msleep(100); tracing_stop(); ftrace_enabled=0; ret=trace_test_buffer(&mut (*tr).array_buffer,&mut count); ftrace_enabled=1; ((*trace).reset)(tr); tracing_start(); if ret==0 && count==0 { ret=-1; } }
        if ret==0 { ret=trace_selftest_startup_dynamic_tracing(trace,tr,DYN_FTRACE_TEST_NAME); } if ret==0 { ret=trace_selftest_function_recursion(); } ftrace_enabled=save; if ret!=0 { ftrace_kill(); } ret
    }
}

// The remaining tracer-specific entry points retain the C kernel interfaces;
// their implementations are provided in the same manner by the configured
// translation unit.
#[cfg(feature = "CONFIG_NOP_TRACER")]
pub unsafe fn trace_selftest_startup_nop(_: *mut tracer, _: *mut trace_array) -> i32 { 0 }

#[cfg(feature = "CONFIG_IRQSOFF_TRACER")]
pub unsafe fn trace_selftest_startup_irqsoff(trace: *mut tracer, tr: *mut trace_array) -> i32 {
    let save = (*tr).max_latency; let mut count=0; let mut ret=tracer_init(trace,tr); if ret!=0 { warn_failed_init_tracer(trace,ret); return ret; }
    (*tr).max_latency=0; local_irq_disable(); udelay(100); local_irq_enable(); ((*trace).stop)(tr); tracing_stop(); ret=trace_test_buffer(&mut (*tr).array_buffer,core::ptr::null_mut()); if ret==0 { ret=trace_test_buffer(&mut (*tr).snapshot_buffer,&mut count); } ((*trace).reset)(tr); tracing_start(); if ret==0 && count==0 { ret=-1; } (*tr).max_latency=save; ret
}

#[cfg(feature = "CONFIG_PREEMPT_TRACER")]
pub unsafe fn trace_selftest_startup_preemptoff(trace: *mut tracer, tr: *mut trace_array) -> i32 {
    if preempt_count()!=0 { printk(KERN_CONT.as_ptr(),"can not test ... force "); return 0; }
    let save=(*tr).max_latency; let mut count=0; let mut ret=tracer_init(trace,tr); if ret!=0 { warn_failed_init_tracer(trace,ret); return ret; }
    (*tr).max_latency=0; preempt_disable(); udelay(100); preempt_enable(); ((*trace).stop)(tr); tracing_stop(); ret=trace_test_buffer(&mut (*tr).array_buffer,core::ptr::null_mut()); if ret==0 { ret=trace_test_buffer(&mut (*tr).snapshot_buffer,&mut count); } ((*trace).reset)(tr); tracing_start(); if ret==0 && count==0 { ret=-1; } (*tr).max_latency=save; ret
}

#[cfg(all(feature = "CONFIG_IRQSOFF_TRACER", feature = "CONFIG_PREEMPT_TRACER"))]
pub unsafe fn trace_selftest_startup_preemptirqsoff(trace: *mut tracer, tr: *mut trace_array) -> i32 {
    let save=(*tr).max_latency; let mut count=0; let mut ret=tracer_init(trace,tr); if ret!=0 { return ret; } (*tr).max_latency=0; preempt_disable(); local_irq_disable(); udelay(100); preempt_enable(); local_irq_enable(); ((*trace).stop)(tr); tracing_stop(); ret=trace_test_buffer(&mut (*tr).array_buffer,core::ptr::null_mut()); if ret==0 { ret=trace_test_buffer(&mut (*tr).snapshot_buffer,&mut count); } tracing_start(); ((*trace).reset)(tr); (*tr).max_latency=save; if ret==0 && count==0 { ret=-1; } ret
}

#[cfg(feature = "CONFIG_BRANCH_TRACER")]
pub unsafe fn trace_selftest_startup_branch(trace: *mut tracer, tr: *mut trace_array) -> i32 {
    let mut count=0; let mut ret=tracer_init(trace,tr); if ret!=0 { warn_failed_init_tracer(trace,ret); return ret; } msleep(100); tracing_stop(); ret=trace_test_buffer(&mut (*tr).array_buffer,&mut count); ((*trace).reset)(tr); tracing_start(); if ret==0 && count==0 { ret=-1; } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
