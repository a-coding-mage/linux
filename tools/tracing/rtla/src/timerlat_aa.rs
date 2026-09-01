// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2023 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

// C dependencies removed from executable Rust:
// stdlib.h, errno.h, timerlat.h, unistd.h

use core::ffi::{c_char, c_double, c_int, c_ulong, c_void};
use core::ptr;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum timelat_state {
    TIMERLAT_INIT = 0,
    TIMERLAT_WAITING_IRQ,
    TIMERLAT_WAITING_THREAD,
}

/* Used to fill spaces in the output */
static spaces: *const c_char =
    b"                                                         \0".as_ptr() as *const c_char;

const MAX_COMM: usize = 24;

#[repr(C)]
pub struct trace_seq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_record {
    pub ts: u64,
    pub cpu: c_int,
}

#[repr(C)]
pub struct tep_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tracefs_instance {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_instance {
    pub tep: *mut tep_handle,
    pub inst: *mut tracefs_instance,
}

#[repr(C)]
pub struct osnoise_tool {
    pub trace: trace_instance,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stack_format {
    STACK_FORMAT_TRUNCATE = 0,
    STACK_FORMAT_SKIP,
    STACK_FORMAT_FULL,
}

/*
 * Per-cpu data statistics and data.
 */
#[repr(C)]
struct timerlat_aa_data {
    /* Current CPU state */
    curr_state: c_int,

    /* timerlat IRQ latency */
    tlat_irq_seqnum: u64,
    tlat_irq_latency: u64,
    tlat_irq_timstamp: u64,

    /* timerlat Thread latency */
    tlat_thread_seqnum: u64,
    tlat_thread_latency: u64,
    tlat_thread_timstamp: u64,

    /*
     * Information about the thread running when the IRQ
     * arrived.
     *
     * This can be blocking or interference, depending on the
     * priority of the thread. Assuming timerlat is the highest
     * prio, it is blocking. If timerlat has a lower prio, it is
     * interference.
     * note: "unsigned long long" because they are fetch using tep_get_field_val();
     */
    run_thread_pid: u64,
    run_thread_comm: [c_char; MAX_COMM],
    thread_blocking_duration: u64,
    max_exit_idle_latency: u64,

    /* Information about the timerlat timer irq */
    timer_irq_start_time: u64,
    timer_irq_start_delay: u64,
    timer_irq_duration: u64,
    timer_exit_from_idle: u64,

    /*
     * Information about the last IRQ before the timerlat irq
     * arrived.
     *
     * If now - timestamp is <= latency, it might have influenced
     * in the timerlat irq latency. Otherwise, ignore it.
     */
    prev_irq_duration: u64,
    prev_irq_timstamp: u64,

    /*
     * Interference sum.
     */
    thread_nmi_sum: u64,
    thread_irq_sum: u64,
    thread_softirq_sum: u64,
    thread_thread_sum: u64,

    /*
     * Interference task information.
     */
    prev_irqs_seq: *mut trace_seq,
    nmi_seq: *mut trace_seq,
    irqs_seq: *mut trace_seq,
    softirqs_seq: *mut trace_seq,
    threads_seq: *mut trace_seq,
    stack_seq: *mut trace_seq,

    /*
     * Current thread.
     */
    current_comm: [c_char; MAX_COMM],
    current_pid: u64,

    /*
     * Is the system running a kworker?
     */
    kworker: u64,
    kworker_func: u64,
}

/*
 * The analysis context and system wide view
 */
#[repr(C)]
struct timerlat_aa_context {
    dump_tasks: c_int,
    stack_format: stack_format,

    /* per CPU data */
    taa_data: *mut timerlat_aa_data,

    /*
     * required to translate function names and register
     * events.
     */
    tool: *mut osnoise_tool,
}

unsafe extern "C" {
    static mut nr_cpus: c_int;
    static mut errno: c_int;

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;

    fn trace_seq_reset(s: *mut trace_seq);
    fn trace_seq_printf(s: *mut trace_seq, format: *const c_char, ...) -> c_int;
    fn trace_seq_do_printf(s: *mut trace_seq);
    fn trace_seq_destroy(s: *mut trace_seq);
    fn trace_seq_init(s: *mut trace_seq);

    fn tep_get_field_val(
        s: *mut trace_seq,
        event: *mut tep_event,
        name: *const c_char,
        record: *mut tep_record,
        val: *mut u64,
        err: c_int,
    ) -> c_int;
    fn tep_get_common_field_val(
        s: *mut trace_seq,
        event: *mut tep_event,
        name: *const c_char,
        record: *mut tep_record,
        val: *mut u64,
        err: c_int,
    ) -> c_int;
    fn tep_get_field_raw(
        s: *mut trace_seq,
        event: *mut tep_event,
        name: *const c_char,
        record: *mut tep_record,
        len: *mut c_int,
        err: c_int,
    ) -> *mut c_char;
    fn tep_find_function(tep: *mut tep_handle, addr: c_ulong) -> *const c_char;
    fn tep_register_event_handler(
        tep: *mut tep_handle,
        id: c_int,
        sys_name: *const c_char,
        event_name: *const c_char,
        func: unsafe extern "C" fn(*mut trace_seq, *mut tep_record, *mut tep_event, *mut c_void) -> c_int,
        context: *mut c_void,
    ) -> c_int;
    fn tep_unregister_event_handler(
        tep: *mut tep_handle,
        id: c_int,
        sys_name: *const c_char,
        event_name: *const c_char,
        func: unsafe extern "C" fn(*mut trace_seq, *mut tep_record, *mut tep_event, *mut c_void) -> c_int,
        context: *mut c_void,
    ) -> c_int;

    fn tracefs_iterate_raw_events(
        tep: *mut tep_handle,
        inst: *mut tracefs_instance,
        systems: *mut c_void,
        flags: c_int,
        callback: unsafe extern "C" fn(*mut tep_event, *mut tep_record, *mut c_void) -> c_int,
        context: *mut c_void,
    ) -> c_int;
    fn collect_registered_events(
        event: *mut tep_event,
        record: *mut tep_record,
        context: *mut c_void,
    ) -> c_int;
    fn tracefs_event_enable(
        inst: *mut tracefs_instance,
        system: *const c_char,
        event: *const c_char,
    ) -> c_int;
    fn tracefs_event_disable(
        inst: *mut tracefs_instance,
        system: *const c_char,
        event: *const c_char,
    ) -> c_int;
    fn err_msg(format: *const c_char, ...);

    fn ns_to_usf(ns: u64) -> c_double;
    fn ns_to_per(total: c_int, val: u64) -> c_double;
}

/*
 * The data is stored as a local variable, but accessed via a helper function.
 *
 * It could be stored inside the trace context. But every access would
 * require container_of() + a series of pointers. Do we need it? Not sure.
 *
 * For now keep it simple. If needed, store it in the tool, add the *context
 * as a parameter in timerlat_aa_get_ctx() and do the magic there.
 */
static mut __timerlat_aa_ctx: *mut timerlat_aa_context = ptr::null_mut();

unsafe fn timerlat_aa_get_ctx() -> *mut timerlat_aa_context {
    __timerlat_aa_ctx
}

/*
 * timerlat_aa_get_data - Get the per-cpu data from the timerlat context
 */
unsafe fn timerlat_aa_get_data(taa_ctx: *mut timerlat_aa_context, cpu: c_int) -> *mut timerlat_aa_data {
    (*taa_ctx).taa_data.add(cpu as usize)
}

/*
 * timerlat_aa_irq_latency - Handles timerlat IRQ event
 */
unsafe fn timerlat_aa_irq_latency(
    taa_data: *mut timerlat_aa_data,
    s: *mut trace_seq,
    record: *mut tep_record,
    event: *mut tep_event,
) -> c_int {
    /*
     * For interference, we start now looking for things that can delay
     * the thread.
     */
    (*taa_data).curr_state = timelat_state::TIMERLAT_WAITING_THREAD as c_int;
    (*taa_data).tlat_irq_timstamp = (*record).ts;

    /*
     * Zero values.
     */
    (*taa_data).thread_nmi_sum = 0;
    (*taa_data).thread_irq_sum = 0;
    (*taa_data).thread_softirq_sum = 0;
    (*taa_data).thread_thread_sum = 0;
    (*taa_data).thread_blocking_duration = 0;
    (*taa_data).timer_irq_start_time = 0;
    (*taa_data).timer_irq_duration = 0;
    (*taa_data).timer_exit_from_idle = 0;

    /*
     * Zero interference tasks.
     */
    trace_seq_reset((*taa_data).nmi_seq);
    trace_seq_reset((*taa_data).irqs_seq);
    trace_seq_reset((*taa_data).softirqs_seq);
    trace_seq_reset((*taa_data).threads_seq);

    /* IRQ latency values */
    tep_get_field_val(s, event, b"timer_latency\0".as_ptr() as *const c_char, record, &mut (*taa_data).tlat_irq_latency, 1);
    tep_get_field_val(s, event, b"seqnum\0".as_ptr() as *const c_char, record, &mut (*taa_data).tlat_irq_seqnum, 1);

    /* The thread that can cause blocking */
    tep_get_common_field_val(s, event, b"common_pid\0".as_ptr() as *const c_char, record, &mut (*taa_data).run_thread_pid, 1);

    /*
     * Get exit from idle case.
     *
     * If it is not idle thread:
     */
    if (*taa_data).run_thread_pid != 0 {
        return 0;
    }

    /*
     * if the latency is shorter than the known exit from idle:
     */
    if (*taa_data).tlat_irq_latency < (*taa_data).max_exit_idle_latency {
        return 0;
    }

    /*
     * To be safe, ignore the cases in which an IRQ/NMI could have
     * interfered with the timerlat IRQ.
     */
    if (*taa_data).tlat_irq_timstamp.wrapping_sub((*taa_data).tlat_irq_latency)
        < (*taa_data).prev_irq_timstamp.wrapping_add((*taa_data).prev_irq_duration)
    {
        return 0;
    }

    (*taa_data).max_exit_idle_latency = (*taa_data).tlat_irq_latency;

    0
}

/*
 * timerlat_aa_thread_latency - Handles timerlat thread event
 */
unsafe fn timerlat_aa_thread_latency(
    taa_data: *mut timerlat_aa_data,
    s: *mut trace_seq,
    record: *mut tep_record,
    event: *mut tep_event,
) -> c_int {
    /*
     * For interference, we start now looking for things that can delay
     * the IRQ of the next cycle.
     */
    (*taa_data).curr_state = timelat_state::TIMERLAT_WAITING_IRQ as c_int;
    (*taa_data).tlat_thread_timstamp = (*record).ts;

    /* Thread latency values */
    tep_get_field_val(s, event, b"timer_latency\0".as_ptr() as *const c_char, record, &mut (*taa_data).tlat_thread_latency, 1);
    tep_get_field_val(s, event, b"seqnum\0".as_ptr() as *const c_char, record, &mut (*taa_data).tlat_thread_seqnum, 1);

    0
}

/*
 * timerlat_aa_handler - Handle timerlat events
 *
 * This function is called to handle timerlat events recording statistics.
 *
 * Returns 0 on success, -1 otherwise.
 */
unsafe extern "C" fn timerlat_aa_handler(
    s: *mut trace_seq,
    record: *mut tep_record,
    event: *mut tep_event,
    _context: *mut c_void,
) -> c_int {
    let taa_ctx = timerlat_aa_get_ctx();
    let taa_data = timerlat_aa_get_data(taa_ctx, (*record).cpu);
    let mut thread: u64 = 0;

    if taa_data.is_null() {
        return -1;
    }

    tep_get_field_val(s, event, b"context\0".as_ptr() as *const c_char, record, &mut thread, 1);
    if thread == 0 {
        timerlat_aa_irq_latency(taa_data, s, record, event)
    } else {
        timerlat_aa_thread_latency(taa_data, s, record, event)
    }
}

/*
 * timerlat_aa_nmi_handler - Handles NMI noise
 *
 * It is used to collect information about interferences from NMI. It is
 * hooked to the osnoise:nmi_noise event.
 */
unsafe extern "C" fn timerlat_aa_nmi_handler(
    s: *mut trace_seq,
    record: *mut tep_record,
    event: *mut tep_event,
    _context: *mut c_void,
) -> c_int {
    let taa_ctx = timerlat_aa_get_ctx();
    let taa_data = timerlat_aa_get_data(taa_ctx, (*record).cpu);
    let mut duration: u64 = 0;
    let mut start: u64 = 0;

    tep_get_field_val(s, event, b"duration\0".as_ptr() as *const c_char, record, &mut duration, 1);
    tep_get_field_val(s, event, b"start\0".as_ptr() as *const c_char, record, &mut start, 1);

    if (*taa_data).curr_state == timelat_state::TIMERLAT_WAITING_IRQ as c_int {
        (*taa_data).prev_irq_duration = duration;
        (*taa_data).prev_irq_timstamp = start;

        trace_seq_reset((*taa_data).prev_irqs_seq);
        trace_seq_printf((*taa_data).prev_irqs_seq, b"  %24s %.*s %9.2f us\n\0".as_ptr() as *const c_char,
                         b"nmi\0".as_ptr() as *const c_char,
                         24, spaces,
                         ns_to_usf(duration));
        return 0;
    }

    (*taa_data).thread_nmi_sum = (*taa_data).thread_nmi_sum.wrapping_add(duration);
    trace_seq_printf((*taa_data).nmi_seq, b"  %24s %.*s %9.2f us\n\0".as_ptr() as *const c_char,
                     b"nmi\0".as_ptr() as *const c_char,
                     24, spaces, ns_to_usf(duration));

    0
}

/*
 * timerlat_aa_irq_handler - Handles IRQ noise
 *
 * It is used to collect information about interferences from IRQ. It is
 * hooked to the osnoise:irq_noise event.
 *
 * It is a little bit more complex than the other because it measures:
 *	- The IRQs that can delay the timer IRQ before it happened.
 *	- The Timerlat IRQ handler
 *	- The IRQs that happened between the timerlat IRQ and the timerlat thread
 *	  (IRQ interference).
 */
unsafe extern "C" fn timerlat_aa_irq_handler(
    s: *mut trace_seq,
    record: *mut tep_record,
    event: *mut tep_event,
    _context: *mut c_void,
) -> c_int {
    let taa_ctx = timerlat_aa_get_ctx();
    let taa_data = timerlat_aa_get_data(taa_ctx, (*record).cpu);
    let expected_start: u64;
    let mut duration: u64 = 0;
    let mut vector: u64 = 0;
    let mut start: u64 = 0;
    let mut val: c_int = 0;

    tep_get_field_val(s, event, b"duration\0".as_ptr() as *const c_char, record, &mut duration, 1);
    tep_get_field_val(s, event, b"start\0".as_ptr() as *const c_char, record, &mut start, 1);
    tep_get_field_val(s, event, b"vector\0".as_ptr() as *const c_char, record, &mut vector, 1);
    let desc = tep_get_field_raw(s, event, b"desc\0".as_ptr() as *const c_char, record, &mut val, 1);

    /*
     * Before the timerlat IRQ.
     */
    if (*taa_data).curr_state == timelat_state::TIMERLAT_WAITING_IRQ as c_int {
        (*taa_data).prev_irq_duration = duration;
        (*taa_data).prev_irq_timstamp = start;

        trace_seq_reset((*taa_data).prev_irqs_seq);
        trace_seq_printf((*taa_data).prev_irqs_seq, b"  %24s:%-3llu %.*s %9.2f us\n\0".as_ptr() as *const c_char,
                         desc, vector,
                         15, spaces,
                         ns_to_usf(duration));
        return 0;
    }

    /*
     * The timerlat IRQ: taa_data->timer_irq_start_time is zeroed at
     * the timerlat irq handler.
     */
    if (*taa_data).timer_irq_start_time == 0 {
        expected_start = (*taa_data).tlat_irq_timstamp.wrapping_sub((*taa_data).tlat_irq_latency);

        (*taa_data).timer_irq_start_time = start;
        (*taa_data).timer_irq_duration = duration;

        /*
         * We are dealing with two different clock sources: the
         * external clock source that timerlat uses as a reference
         * and the clock used by the tracer. There are also two
         * moments: the time reading the clock and the timer in
         * which the event is placed in the buffer (the trace
         * event timestamp). If the processor is slow or there
         * is some hardware noise, the difference between the
         * timestamp and the external clock read can be longer
         * than the IRQ handler delay, resulting in a negative
         * time. If so, set IRQ start delay as 0. In the end,
         * it is less relevant than the noise.
         */
        if expected_start < (*taa_data).timer_irq_start_time {
            (*taa_data).timer_irq_start_delay = (*taa_data).timer_irq_start_time.wrapping_sub(expected_start);
        } else {
            (*taa_data).timer_irq_start_delay = 0;
        }

        /*
         * not exit from idle.
         */
        if (*taa_data).run_thread_pid != 0 {
            return 0;
        }

        if expected_start > (*taa_data).prev_irq_timstamp.wrapping_add((*taa_data).prev_irq_duration) {
            (*taa_data).timer_exit_from_idle = (*taa_data).timer_irq_start_delay;
        }

        return 0;
    }

    /*
     * IRQ interference.
     */
    (*taa_data).thread_irq_sum = (*taa_data).thread_irq_sum.wrapping_add(duration);
    trace_seq_printf((*taa_data).irqs_seq, b"  %24s:%-3llu %.*s %9.2f us\n\0".as_ptr() as *const c_char,
                     desc, vector,
                     24, spaces,
                     ns_to_usf(duration));

    0
}

static mut softirq_name: [*mut c_char; 10] = [
    b"HI\0".as_ptr() as *mut c_char,
    b"TIMER\0".as_ptr() as *mut c_char,
    b"NET_TX\0".as_ptr() as *mut c_char,
    b"NET_RX\0".as_ptr() as *mut c_char,
    b"BLOCK\0".as_ptr() as *mut c_char,
    b"IRQ_POLL\0".as_ptr() as *mut c_char,
    b"TASKLET\0".as_ptr() as *mut c_char,
    b"SCHED\0".as_ptr() as *mut c_char,
    b"HRTIMER\0".as_ptr() as *mut c_char,
    b"RCU\0".as_ptr() as *mut c_char,
];

/*
 * timerlat_aa_softirq_handler - Handles Softirq noise
 *
 * It is used to collect information about interferences from Softirq. It is
 * hooked to the osnoise:softirq_noise event.
 *
 * It is only printed in the non-rt kernel, as softirqs become thread on RT.
 */
unsafe extern "C" fn timerlat_aa_softirq_handler(
    s: *mut trace_seq,
    record: *mut tep_record,
    event: *mut tep_event,
    _context: *mut c_void,
) -> c_int {
    let taa_ctx = timerlat_aa_get_ctx();
    let taa_data = timerlat_aa_get_data(taa_ctx, (*record).cpu);
    let mut duration: u64 = 0;
    let mut vector: u64 = 0;
    let mut start: u64 = 0;

    if (*taa_data).curr_state == timelat_state::TIMERLAT_WAITING_IRQ as c_int {
        return 0;
    }

    tep_get_field_val(s, event, b"duration\0".as_ptr() as *const c_char, record, &mut duration, 1);
    tep_get_field_val(s, event, b"start\0".as_ptr() as *const c_char, record, &mut start, 1);
    tep_get_field_val(s, event, b"vector\0".as_ptr() as *const c_char, record, &mut vector, 1);

    (*taa_data).thread_softirq_sum = (*taa_data).thread_softirq_sum.wrapping_add(duration);

    trace_seq_printf((*taa_data).softirqs_seq, b"  %24s:%-3llu %.*s %9.2f us\n\0".as_ptr() as *const c_char,
                     if vector < softirq_name.len() as u64 { softirq_name[vector as usize] } else { b"UNKNOWN\0".as_ptr() as *mut c_char },
                     vector, 24, spaces,
                     ns_to_usf(duration));
    0
}

/*
 * timerlat_aa_softirq_handler - Handles thread noise
 *
 * It is used to collect information about interferences from threads. It is
 * hooked to the osnoise:thread_noise event.
 *
 * Note: if you see thread noise, your timerlat thread was not the highest prio one.
 */
unsafe extern "C" fn timerlat_aa_thread_handler(
    s: *mut trace_seq,
    record: *mut tep_record,
    event: *mut tep_event,
    _context: *mut c_void,
) -> c_int {
    let taa_ctx = timerlat_aa_get_ctx();
    let taa_data = timerlat_aa_get_data(taa_ctx, (*record).cpu);
    let mut duration: u64 = 0;
    let mut start: u64 = 0;
    let mut pid: u64 = 0;
    let mut val: c_int = 0;

    if (*taa_data).curr_state == timelat_state::TIMERLAT_WAITING_IRQ as c_int {
        return 0;
    }

    tep_get_field_val(s, event, b"duration\0".as_ptr() as *const c_char, record, &mut duration, 1);
    tep_get_field_val(s, event, b"start\0".as_ptr() as *const c_char, record, &mut start, 1);

    tep_get_common_field_val(s, event, b"common_pid\0".as_ptr() as *const c_char, record, &mut pid, 1);
    let comm = tep_get_field_raw(s, event, b"comm\0".as_ptr() as *const c_char, record, &mut val, 1);

    if pid == (*taa_data).run_thread_pid && (*taa_data).thread_blocking_duration == 0 {
        (*taa_data).thread_blocking_duration = duration;

        if !comm.is_null() {
            strncpy((*taa_data).run_thread_comm.as_mut_ptr(), comm, MAX_COMM);
        } else {
            sprintf((*taa_data).run_thread_comm.as_mut_ptr(), b"<...>\0".as_ptr() as *const c_char);
        }
    } else {
        (*taa_data).thread_thread_sum = (*taa_data).thread_thread_sum.wrapping_add(duration);

        trace_seq_printf((*taa_data).threads_seq, b"  %24s:%-12llu %.*s %9.2f us\n\0".as_ptr() as *const c_char,
                         comm, pid,
                         15, spaces,
                         ns_to_usf(duration));
    }

    0
}

/*
 * timerlat_aa_stack_handler - Handles timerlat IRQ stack trace
 *
 * Saves and parse the stack trace generated by the timerlat IRQ.
 */
unsafe extern "C" fn timerlat_aa_stack_handler(
    s: *mut trace_seq,
    record: *mut tep_record,
    event: *mut tep_event,
    _context: *mut c_void,
) -> c_int {
    let taa_ctx = timerlat_aa_get_ctx();
    let taa_data = timerlat_aa_get_data(taa_ctx, (*record).cpu);
    let stack_format = (*taa_ctx).stack_format;
    let mut val: c_int = 0;
    let mut i: u64;

    trace_seq_reset((*taa_data).stack_seq);

    trace_seq_printf((*taa_data).stack_seq, b"    Blocking thread stack trace\n\0".as_ptr() as *const c_char);
    let caller = tep_get_field_raw(s, event, b"caller\0".as_ptr() as *const c_char, record, &mut val, 1) as *mut c_ulong;

    if !caller.is_null() {
        let mut size: u64 = 0;
        let max_entries: u64;

        if tep_get_field_val(s, event, b"size\0".as_ptr() as *const c_char, record, &mut size, 1) == 0 {
            max_entries = if size < 64 { size } else { 64 };
        } else {
            max_entries = 64;
        }

        i = 0;
        while i < max_entries {
            let function = tep_find_function((*(*taa_ctx).tool).trace.tep, *caller.add(i as usize));
            if function.is_null() {
                if stack_format == stack_format::STACK_FORMAT_TRUNCATE {
                    break;
                } else if stack_format == stack_format::STACK_FORMAT_SKIP {
                    i += 1;
                    continue;
                } else if stack_format == stack_format::STACK_FORMAT_FULL {
                    trace_seq_printf((*taa_data).stack_seq, b" %.*s -> 0x%lx\n\0".as_ptr() as *const c_char,
                                     14, spaces, *caller.add(i as usize));
                }
            } else {
                trace_seq_printf((*taa_data).stack_seq, b" %.*s -> %s\n\0".as_ptr() as *const c_char,
                                 14, spaces, function);
            }
            i += 1;
        }
    }

    0
}

/*
 * timerlat_aa_sched_switch_handler - Tracks the current thread running on the CPU
 *
 * Handles the sched:sched_switch event to trace the current thread running on the
 * CPU. It is used to display the threads running on the other CPUs when the trace
 * stops.
 */
unsafe extern "C" fn timerlat_aa_sched_switch_handler(
    s: *mut trace_seq,
    record: *mut tep_record,
    event: *mut tep_event,
    _context: *mut c_void,
) -> c_int {
    let taa_ctx = timerlat_aa_get_ctx();
    let taa_data = timerlat_aa_get_data(taa_ctx, (*record).cpu);
    let mut val: c_int = 0;

    tep_get_field_val(s, event, b"next_pid\0".as_ptr() as *const c_char, record, &mut (*taa_data).current_pid, 1);
    let comm = tep_get_field_raw(s, event, b"next_comm\0".as_ptr() as *const c_char, record, &mut val, 1);

    strncpy((*taa_data).current_comm.as_mut_ptr(), comm, MAX_COMM);

    /*
     * If this was a kworker, clean the last kworkers that ran.
     */
    (*taa_data).kworker = 0;
    (*taa_data).kworker_func = 0;

    0
}

/*
 * timerlat_aa_kworker_start_handler - Tracks a kworker running on the CPU
 *
 * Handles workqueue:workqueue_execute_start event, keeping track of
 * the job that a kworker could be doing in the CPU.
 *
 * We already catch problems of hardware related latencies caused by work queues
 * running driver code that causes hardware stall. For example, with DRM drivers.
 */
unsafe extern "C" fn timerlat_aa_kworker_start_handler(
    s: *mut trace_seq,
    record: *mut tep_record,
    event: *mut tep_event,
    _context: *mut c_void,
) -> c_int {
    let taa_ctx = timerlat_aa_get_ctx();
    let taa_data = timerlat_aa_get_data(taa_ctx, (*record).cpu);

    tep_get_field_val(s, event, b"work\0".as_ptr() as *const c_char, record, &mut (*taa_data).kworker, 1);
    tep_get_field_val(s, event, b"function\0".as_ptr() as *const c_char, record, &mut (*taa_data).kworker_func, 1);
    0
}

/*
 * timerlat_thread_analysis - Prints the analysis of a CPU that hit a stop tracing
 *
 * This is the core of the analysis.
 */
unsafe fn timerlat_thread_analysis(
    taa_data: *mut timerlat_aa_data,
    _cpu: c_int,
    _irq_thresh: c_int,
    _thread_thresh: c_int,
) {
    let exp_irq_ts: i64;
    let total: c_int;
    let irq: c_int;

    /*
     * IRQ latency or Thread latency?
     */
    if (*taa_data).tlat_irq_seqnum > (*taa_data).tlat_thread_seqnum {
        irq = 1;
        total = (*taa_data).tlat_irq_latency as c_int;
    } else {
        irq = 0;
        total = (*taa_data).tlat_thread_latency as c_int;
    }

    /*
     * Expected IRQ arrival time using the trace clock as the base.
     *
     * TODO: Add a list of previous IRQ, and then run the list backwards.
     */
    exp_irq_ts = (*taa_data).timer_irq_start_time.wrapping_sub((*taa_data).timer_irq_start_delay) as i64;
    if exp_irq_ts < (*taa_data).prev_irq_timstamp.wrapping_add((*taa_data).prev_irq_duration) as i64 {
        if (*taa_data).prev_irq_timstamp < (*taa_data).timer_irq_start_time {
            printf(b"  Previous IRQ interference: %.*s up to  %9.2f us\n\0".as_ptr() as *const c_char,
                   16, spaces,
                   ns_to_usf((*taa_data).prev_irq_duration));
        }
    }

    /*
     * The delay that the IRQ suffered before starting.
     */
    printf(b"  IRQ handler delay: %.*s %16s  %9.2f us (%.2f %%)\n\0".as_ptr() as *const c_char, 16, spaces,
           if ns_to_usf((*taa_data).timer_exit_from_idle) > 10.0 { b"(exit from idle)\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char },
           ns_to_usf((*taa_data).timer_irq_start_delay),
           ns_to_per(total, (*taa_data).timer_irq_start_delay));

    /*
     * Timerlat IRQ.
     */
    printf(b"  IRQ latency: %.*s %9.2f us\n\0".as_ptr() as *const c_char, 40, spaces,
           ns_to_usf((*taa_data).tlat_irq_latency));

    if irq != 0 {
        /*
         * If the trace stopped due to IRQ, the other events will not happen
         * because... the trace stopped :-).
         *
         * That is all folks, the stack trace was printed before the stop,
         * so it will be displayed, it is the key.
         */
        printf(b"  Blocking thread:\n\0".as_ptr() as *const c_char);
        printf(b" %.*s %24s:%-9llu\n\0".as_ptr() as *const c_char, 6, spaces, (*taa_data).run_thread_comm.as_ptr(),
               (*taa_data).run_thread_pid);
    } else {
        /*
         * The duration of the IRQ handler that handled the timerlat IRQ.
         */
        printf(b"  Timerlat IRQ duration: %.*s %9.2f us (%.2f %%)\n\0".as_ptr() as *const c_char,
               30, spaces,
               ns_to_usf((*taa_data).timer_irq_duration),
               ns_to_per(total, (*taa_data).timer_irq_duration));

        /*
         * The amount of time that the current thread postponed the scheduler.
         *
         * Recalling that it is net from NMI/IRQ/Softirq interference, so there
         * is no need to compute values here.
         */
        printf(b"  Blocking thread: %.*s %9.2f us (%.2f %%)\n\0".as_ptr() as *const c_char, 36, spaces,
               ns_to_usf((*taa_data).thread_blocking_duration),
               ns_to_per(total, (*taa_data).thread_blocking_duration));

        printf(b" %.*s %24s:%-9llu %.*s %9.2f us\n\0".as_ptr() as *const c_char, 6, spaces,
               (*taa_data).run_thread_comm.as_ptr(), (*taa_data).run_thread_pid,
               12, spaces, ns_to_usf((*taa_data).thread_blocking_duration));
    }

    /*
     * Print the stack trace!
     */
    trace_seq_do_printf((*taa_data).stack_seq);

    /*
     * NMIs can happen during the IRQ, so they are always possible.
     */
    if (*taa_data).thread_nmi_sum != 0 {
        printf(b"  NMI interference %.*s %9.2f us (%.2f %%)\n\0".as_ptr() as *const c_char, 36, spaces,
               ns_to_usf((*taa_data).thread_nmi_sum),
               ns_to_per(total, (*taa_data).thread_nmi_sum));
    }

    /*
     * If it is an IRQ latency, the other factors can be skipped.
     */
    if irq == 0 {
        /*
         * Prints the interference caused by IRQs to the thread latency.
         */
        if (*taa_data).thread_irq_sum != 0 {
            printf(b"  IRQ interference %.*s %9.2f us (%.2f %%)\n\0".as_ptr() as *const c_char, 36, spaces,
                   ns_to_usf((*taa_data).thread_irq_sum),
                   ns_to_per(total, (*taa_data).thread_irq_sum));

            trace_seq_do_printf((*taa_data).irqs_seq);
        }

        /*
         * Prints the interference caused by Softirqs to the thread latency.
         */
        if (*taa_data).thread_softirq_sum != 0 {
            printf(b"  Softirq interference %.*s %9.2f us (%.2f %%)\n\0".as_ptr() as *const c_char, 32, spaces,
                   ns_to_usf((*taa_data).thread_softirq_sum),
                   ns_to_per(total, (*taa_data).thread_softirq_sum));

            trace_seq_do_printf((*taa_data).softirqs_seq);
        }

        /*
         * Prints the interference caused by other threads to the thread latency.
         *
         * If this happens, your timerlat is not the highest prio. OK, migration
         * thread can happen. But otherwise, you are not measuring the "scheduling
         * latency" only, and here is the difference from scheduling latency and
         * timer handling latency.
         */
        if (*taa_data).thread_thread_sum != 0 {
            printf(b"  Thread interference %.*s %9.2f us (%.2f %%)\n\0".as_ptr() as *const c_char, 33, spaces,
                   ns_to_usf((*taa_data).thread_thread_sum),
                   ns_to_per(total, (*taa_data).thread_thread_sum));

            trace_seq_do_printf((*taa_data).threads_seq);
        }
    }

    /*
     * Done.
     */
    printf(b"------------------------------------------------------------------------\n\0".as_ptr() as *const c_char);
    printf(b"  %s latency: %.*s %9.2f us (100%%)\n\0".as_ptr() as *const c_char,
           if irq != 0 { b"   IRQ\0".as_ptr() as *const c_char } else { b"Thread\0".as_ptr() as *const c_char },
           37, spaces, ns_to_usf(total as u64));
}

unsafe fn timerlat_auto_analysis_collect_trace(taa_ctx: *mut timerlat_aa_context) -> c_int {
    let trace = &mut (*(*taa_ctx).tool).trace as *mut trace_instance;
    let retval: c_int;

    retval = tracefs_iterate_raw_events((*trace).tep,
                                        (*trace).inst,
                                        ptr::null_mut(),
                                        0,
                                        collect_registered_events,
                                        trace as *mut c_void);
    if retval < 0 {
        err_msg(b"Error iterating on events\n\0".as_ptr() as *const c_char);
        return 0;
    }

    1
}

/**
 * timerlat_auto_analysis - Analyze the collected data
 */
#[no_mangle]
pub unsafe extern "C" fn timerlat_auto_analysis(mut irq_thresh: c_int, mut thread_thresh: c_int) {
    let taa_ctx = timerlat_aa_get_ctx();
    let mut max_exit_from_idle: u64 = 0;
    let mut taa_data: *mut timerlat_aa_data;
    let mut max_exit_from_idle_cpu: c_int = 0;
    let mut tep: *mut tep_handle;
    let mut cpu: c_int;

    timerlat_auto_analysis_collect_trace(taa_ctx);

    /* bring stop tracing to the ns scale */
    irq_thresh = irq_thresh.wrapping_mul(1000);
    thread_thresh = thread_thresh.wrapping_mul(1000);

    cpu = 0;
    while cpu < nr_cpus {
        taa_data = timerlat_aa_get_data(taa_ctx, cpu);

        if irq_thresh != 0 && (*taa_data).tlat_irq_latency >= irq_thresh as u64 {
            printf(b"## CPU %d hit stop tracing, analyzing it ##\n\0".as_ptr() as *const c_char, cpu);
            timerlat_thread_analysis(taa_data, cpu, irq_thresh, thread_thresh);
        } else if thread_thresh != 0 && (*taa_data).tlat_thread_latency >= thread_thresh as u64 {
            printf(b"## CPU %d hit stop tracing, analyzing it ##\n\0".as_ptr() as *const c_char, cpu);
            timerlat_thread_analysis(taa_data, cpu, irq_thresh, thread_thresh);
        }

        if (*taa_data).max_exit_idle_latency > max_exit_from_idle {
            max_exit_from_idle = (*taa_data).max_exit_idle_latency;
            max_exit_from_idle_cpu = cpu;
        }

        cpu += 1;
    }

    if max_exit_from_idle != 0 {
        printf(b"\n\0".as_ptr() as *const c_char);
        printf(b"Max timerlat IRQ latency from idle: %.2f us in cpu %d\n\0".as_ptr() as *const c_char,
               ns_to_usf(max_exit_from_idle), max_exit_from_idle_cpu);
    }
    if (*taa_ctx).dump_tasks == 0 {
        return;
    }

    printf(b"\n\0".as_ptr() as *const c_char);
    printf(b"Printing CPU tasks:\n\0".as_ptr() as *const c_char);
    cpu = 0;
    while cpu < nr_cpus {
        taa_data = timerlat_aa_get_data(taa_ctx, cpu);
        tep = (*(*taa_ctx).tool).trace.tep;

        printf(b"    [%.3d] %24s:%llu\0".as_ptr() as *const c_char, cpu, (*taa_data).current_comm.as_ptr(), (*taa_data).current_pid);

        if (*taa_data).kworker_func != 0 {
            let kworker = tep_find_function(tep, (*taa_data).kworker as c_ulong);
            printf(b" kworker:%s:%s\0".as_ptr() as *const c_char,
                   if !kworker.is_null() { kworker } else { b"<...>\0".as_ptr() as *const c_char },
                   tep_find_function(tep, (*taa_data).kworker_func as c_ulong));
        }
        printf(b"\n\0".as_ptr() as *const c_char);
        cpu += 1;
    }
}

/*
 * timerlat_aa_destroy_seqs - Destroy seq files used to store parsed data
 */
unsafe fn timerlat_aa_destroy_seqs(taa_ctx: *mut timerlat_aa_context) {
    let mut taa_data: *mut timerlat_aa_data;
    let mut i: c_int;

    if (*taa_ctx).taa_data.is_null() {
        return;
    }

    i = 0;
    while i < nr_cpus {
        taa_data = timerlat_aa_get_data(taa_ctx, i);

        if !(*taa_data).prev_irqs_seq.is_null() {
            trace_seq_destroy((*taa_data).prev_irqs_seq);
            free((*taa_data).prev_irqs_seq as *mut c_void);
        }

        if !(*taa_data).nmi_seq.is_null() {
            trace_seq_destroy((*taa_data).nmi_seq);
            free((*taa_data).nmi_seq as *mut c_void);
        }

        if !(*taa_data).irqs_seq.is_null() {
            trace_seq_destroy((*taa_data).irqs_seq);
            free((*taa_data).irqs_seq as *mut c_void);
        }

        if !(*taa_data).softirqs_seq.is_null() {
            trace_seq_destroy((*taa_data).softirqs_seq);
            free((*taa_data).softirqs_seq as *mut c_void);
        }

        if !(*taa_data).threads_seq.is_null() {
            trace_seq_destroy((*taa_data).threads_seq);
            free((*taa_data).threads_seq as *mut c_void);
        }

        if !(*taa_data).stack_seq.is_null() {
            trace_seq_destroy((*taa_data).stack_seq);
            free((*taa_data).stack_seq as *mut c_void);
        }
        i += 1;
    }
}

/*
 * timerlat_aa_init_seqs - Init seq files used to store parsed information
 *
 * Instead of keeping data structures to store raw data, use seq files to
 * store parsed data.
 *
 * Allocates and initialize seq files.
 *
 * Returns 0 on success, -1 otherwise.
 */
unsafe fn timerlat_aa_init_seqs(taa_ctx: *mut timerlat_aa_context) -> c_int {
    let mut taa_data: *mut timerlat_aa_data;
    let mut i: c_int;

    i = 0;
    while i < nr_cpus {
        taa_data = timerlat_aa_get_data(taa_ctx, i);

        (*taa_data).prev_irqs_seq = calloc(1, core::mem::size_of::<trace_seq>()) as *mut trace_seq;
        if (*taa_data).prev_irqs_seq.is_null() {
            timerlat_aa_destroy_seqs(taa_ctx);
            return -1;
        }

        trace_seq_init((*taa_data).prev_irqs_seq);

        (*taa_data).nmi_seq = calloc(1, core::mem::size_of::<trace_seq>()) as *mut trace_seq;
        if (*taa_data).nmi_seq.is_null() {
            timerlat_aa_destroy_seqs(taa_ctx);
            return -1;
        }

        trace_seq_init((*taa_data).nmi_seq);

        (*taa_data).irqs_seq = calloc(1, core::mem::size_of::<trace_seq>()) as *mut trace_seq;
        if (*taa_data).irqs_seq.is_null() {
            timerlat_aa_destroy_seqs(taa_ctx);
            return -1;
        }

        trace_seq_init((*taa_data).irqs_seq);

        (*taa_data).softirqs_seq = calloc(1, core::mem::size_of::<trace_seq>()) as *mut trace_seq;
        if (*taa_data).softirqs_seq.is_null() {
            timerlat_aa_destroy_seqs(taa_ctx);
            return -1;
        }

        trace_seq_init((*taa_data).softirqs_seq);

        (*taa_data).threads_seq = calloc(1, core::mem::size_of::<trace_seq>()) as *mut trace_seq;
        if (*taa_data).threads_seq.is_null() {
            timerlat_aa_destroy_seqs(taa_ctx);
            return -1;
        }

        trace_seq_init((*taa_data).threads_seq);

        (*taa_data).stack_seq = calloc(1, core::mem::size_of::<trace_seq>()) as *mut trace_seq;
        if (*taa_data).stack_seq.is_null() {
            timerlat_aa_destroy_seqs(taa_ctx);
            return -1;
        }

        trace_seq_init((*taa_data).stack_seq);
        i += 1;
    }

    0
}

/*
 * timerlat_aa_unregister_events - Unregister events used in the auto-analysis
 */
unsafe fn timerlat_aa_unregister_events(tool: *mut osnoise_tool, dump_tasks: c_int) {
    tep_unregister_event_handler((*tool).trace.tep, -1, b"ftrace\0".as_ptr() as *const c_char, b"timerlat\0".as_ptr() as *const c_char,
                                 timerlat_aa_handler, tool as *mut c_void);

    tracefs_event_disable((*tool).trace.inst, b"osnoise\0".as_ptr() as *const c_char, ptr::null());

    tep_unregister_event_handler((*tool).trace.tep, -1, b"osnoise\0".as_ptr() as *const c_char, b"nmi_noise\0".as_ptr() as *const c_char,
                                 timerlat_aa_nmi_handler, tool as *mut c_void);

    tep_unregister_event_handler((*tool).trace.tep, -1, b"osnoise\0".as_ptr() as *const c_char, b"irq_noise\0".as_ptr() as *const c_char,
                                 timerlat_aa_irq_handler, tool as *mut c_void);

    tep_unregister_event_handler((*tool).trace.tep, -1, b"osnoise\0".as_ptr() as *const c_char, b"softirq_noise\0".as_ptr() as *const c_char,
                                 timerlat_aa_softirq_handler, tool as *mut c_void);

    tep_unregister_event_handler((*tool).trace.tep, -1, b"osnoise\0".as_ptr() as *const c_char, b"thread_noise\0".as_ptr() as *const c_char,
                                 timerlat_aa_thread_handler, tool as *mut c_void);

    tep_unregister_event_handler((*tool).trace.tep, -1, b"ftrace\0".as_ptr() as *const c_char, b"kernel_stack\0".as_ptr() as *const c_char,
                                 timerlat_aa_stack_handler, tool as *mut c_void);
    if dump_tasks == 0 {
        return;
    }

    tracefs_event_disable((*tool).trace.inst, b"sched\0".as_ptr() as *const c_char, b"sched_switch\0".as_ptr() as *const c_char);
    tep_unregister_event_handler((*tool).trace.tep, -1, b"sched\0".as_ptr() as *const c_char, b"sched_switch\0".as_ptr() as *const c_char,
                                 timerlat_aa_sched_switch_handler, tool as *mut c_void);

    tracefs_event_disable((*tool).trace.inst, b"workqueue\0".as_ptr() as *const c_char, b"workqueue_execute_start\0".as_ptr() as *const c_char);
    tep_unregister_event_handler((*tool).trace.tep, -1, b"workqueue\0".as_ptr() as *const c_char, b"workqueue_execute_start\0".as_ptr() as *const c_char,
                                 timerlat_aa_kworker_start_handler, tool as *mut c_void);
}

/*
 * timerlat_aa_register_events - Register events used in the auto-analysis
 *
 * Returns 0 on success, -1 otherwise.
 */
unsafe fn timerlat_aa_register_events(tool: *mut osnoise_tool, dump_tasks: c_int) -> c_int {
    let mut retval: c_int;

    tep_register_event_handler((*tool).trace.tep, -1, b"ftrace\0".as_ptr() as *const c_char, b"timerlat\0".as_ptr() as *const c_char,
                               timerlat_aa_handler, tool as *mut c_void);

    /*
     * register auto-analysis handlers.
     */
    retval = tracefs_event_enable((*tool).trace.inst, b"osnoise\0".as_ptr() as *const c_char, ptr::null());
    if retval < 0 && errno == 0 {
        err_msg(b"Could not find osnoise events\n\0".as_ptr() as *const c_char);
        timerlat_aa_unregister_events(tool, dump_tasks);
        return -1;
    }

    tep_register_event_handler((*tool).trace.tep, -1, b"osnoise\0".as_ptr() as *const c_char, b"nmi_noise\0".as_ptr() as *const c_char,
                               timerlat_aa_nmi_handler, tool as *mut c_void);

    tep_register_event_handler((*tool).trace.tep, -1, b"osnoise\0".as_ptr() as *const c_char, b"irq_noise\0".as_ptr() as *const c_char,
                               timerlat_aa_irq_handler, tool as *mut c_void);

    tep_register_event_handler((*tool).trace.tep, -1, b"osnoise\0".as_ptr() as *const c_char, b"softirq_noise\0".as_ptr() as *const c_char,
                               timerlat_aa_softirq_handler, tool as *mut c_void);

    tep_register_event_handler((*tool).trace.tep, -1, b"osnoise\0".as_ptr() as *const c_char, b"thread_noise\0".as_ptr() as *const c_char,
                               timerlat_aa_thread_handler, tool as *mut c_void);

    tep_register_event_handler((*tool).trace.tep, -1, b"ftrace\0".as_ptr() as *const c_char, b"kernel_stack\0".as_ptr() as *const c_char,
                               timerlat_aa_stack_handler, tool as *mut c_void);

    if dump_tasks == 0 {
        return 0;
    }

    /*
     * Dump task events.
     */
    retval = tracefs_event_enable((*tool).trace.inst, b"sched\0".as_ptr() as *const c_char, b"sched_switch\0".as_ptr() as *const c_char);
    if retval < 0 && errno == 0 {
        err_msg(b"Could not find sched_switch\n\0".as_ptr() as *const c_char);
        timerlat_aa_unregister_events(tool, dump_tasks);
        return -1;
    }

    tep_register_event_handler((*tool).trace.tep, -1, b"sched\0".as_ptr() as *const c_char, b"sched_switch\0".as_ptr() as *const c_char,
                               timerlat_aa_sched_switch_handler, tool as *mut c_void);

    retval = tracefs_event_enable((*tool).trace.inst, b"workqueue\0".as_ptr() as *const c_char, b"workqueue_execute_start\0".as_ptr() as *const c_char);
    if retval < 0 && errno == 0 {
        err_msg(b"Could not find workqueue_execute_start\n\0".as_ptr() as *const c_char);
        timerlat_aa_unregister_events(tool, dump_tasks);
        return -1;
    }

    tep_register_event_handler((*tool).trace.tep, -1, b"workqueue\0".as_ptr() as *const c_char, b"workqueue_execute_start\0".as_ptr() as *const c_char,
                               timerlat_aa_kworker_start_handler, tool as *mut c_void);

    0
}

/**
 * timerlat_aa_destroy - Destroy timerlat auto-analysis
 */
#[no_mangle]
pub unsafe extern "C" fn timerlat_aa_destroy() {
    let taa_ctx = timerlat_aa_get_ctx();

    if taa_ctx.is_null() {
        return;
    }

    if !(*taa_ctx).taa_data.is_null() {
        timerlat_aa_unregister_events((*taa_ctx).tool, (*taa_ctx).dump_tasks);
        timerlat_aa_destroy_seqs(taa_ctx);
        free((*taa_ctx).taa_data as *mut c_void);
    }
    free(taa_ctx as *mut c_void);
}

/**
 * timerlat_aa_init - Initialize timerlat auto-analysis
 *
 * Returns 0 on success, -1 otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn timerlat_aa_init(
    tool: *mut osnoise_tool,
    dump_tasks: c_int,
    stack_format: stack_format,
) -> c_int {
    let taa_ctx: *mut timerlat_aa_context;
    let mut retval: c_int;

    taa_ctx = calloc(1, core::mem::size_of::<timerlat_aa_context>()) as *mut timerlat_aa_context;
    if taa_ctx.is_null() {
        return -1;
    }

    __timerlat_aa_ctx = taa_ctx;

    (*taa_ctx).tool = tool;
    (*taa_ctx).dump_tasks = dump_tasks;
    (*taa_ctx).stack_format = stack_format;

    (*taa_ctx).taa_data = calloc(nr_cpus as usize, core::mem::size_of::<timerlat_aa_data>()) as *mut timerlat_aa_data;
    if (*taa_ctx).taa_data.is_null() {
        timerlat_aa_destroy();
        return -1;
    }

    retval = timerlat_aa_init_seqs(taa_ctx);
    if retval != 0 {
        timerlat_aa_destroy();
        return -1;
    }

    retval = timerlat_aa_register_events(tool, dump_tasks);
    if retval != 0 {
        timerlat_aa_destroy();
        return -1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
