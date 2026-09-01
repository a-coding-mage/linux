// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source: tracing/rtla/src/trace.c

use libc::{
    c_char, c_int, c_void, close, creat, free, mode_t, printf, read, snprintf, ssize_t, strerror,
    strlen, write, EINTR, ENODEV, O_RDONLY,
};

const MAX_PATH: usize = 1024;
const UINT64_MAX: u64 = u64::MAX;

#[repr(C)]
pub struct tracefs_instance {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_seq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_record {
    pub missed_events: u64,
}

pub type tep_event_handler_func = Option<
    unsafe extern "C" fn(
        s: *mut trace_seq,
        record: *mut tep_record,
        event: *mut tep_event,
        context: *mut c_void,
    ),
>;

#[repr(C)]
pub struct tep_event {
    pub handler: tep_event_handler_func,
}

#[repr(C)]
pub struct trace_instance {
    pub inst: *mut tracefs_instance,
    pub seq: *mut trace_seq,
    pub tep: *mut tep_handle,
    pub missed_events: u64,
    pub processed_events: u64,
}

#[repr(C)]
pub struct trace_events {
    pub next: *mut trace_events,
    pub system: *mut c_char,
    pub event: *mut c_char,
    pub filter: *mut c_char,
    pub trigger: *mut c_char,
    pub enabled: c_int,
    pub filter_enabled: c_int,
    pub trigger_enabled: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tracefs_tracers {
    TRACEFS_TRACER_CUSTOM,
    TRACEFS_TRACER_NOP,
}

unsafe extern "C" {
    fn tracefs_tracer_set(
        inst: *mut tracefs_instance,
        tracer: tracefs_tracers,
        ...
    ) -> c_int;
    fn tracefs_instance_create(instance_name: *mut c_char) -> *mut tracefs_instance;
    fn tracefs_instance_destroy(inst: *mut tracefs_instance);
    fn tracefs_instance_free(inst: *mut tracefs_instance);
    fn tracefs_instance_file_open(
        inst: *mut tracefs_instance,
        file: *const c_char,
        flags: c_int,
    ) -> c_int;
    fn tracefs_local_events(tracing_dir: *const c_char) -> *mut tep_handle;
    fn tracefs_trace_off(inst: *mut tracefs_instance) -> c_int;
    fn tracefs_trace_on(inst: *mut tracefs_instance) -> c_int;
    fn tracefs_follow_missed_events(
        inst: *mut tracefs_instance,
        callback: Option<
            unsafe extern "C" fn(
                event: *mut tep_event,
                record: *mut tep_record,
                cpu: c_int,
                context: *mut c_void,
            ) -> c_int,
        >,
        context: *mut c_void,
    ) -> c_int;
    fn tracefs_event_file_write(
        inst: *mut tracefs_instance,
        system: *const c_char,
        event: *const c_char,
        file: *const c_char,
        str_: *const c_char,
    ) -> c_int;
    fn tracefs_event_file_read(
        inst: *mut tracefs_instance,
        system: *const c_char,
        event: *const c_char,
        file: *const c_char,
        flags: c_int,
    ) -> *mut c_char;
    fn tracefs_event_disable(
        inst: *mut tracefs_instance,
        system: *const c_char,
        event: *const c_char,
    ) -> c_int;
    fn tracefs_event_enable(
        inst: *mut tracefs_instance,
        system: *const c_char,
        event: *const c_char,
    ) -> c_int;
    fn tracefs_instance_set_buffer_size(
        inst: *mut tracefs_instance,
        size: c_int,
        cpu: c_int,
    ) -> c_int;

    fn tep_free(tep: *mut tep_handle);
    fn trace_seq_init(s: *mut trace_seq);

    fn calloc_fatal(nmemb: usize, size: usize) -> *mut c_void;
    fn strdup_fatal(str_: *const c_char) -> *mut c_char;
    fn str_has_prefix(str_: *const c_char, prefix: *const c_char) -> c_int;

    fn debug_msg(fmt: *const c_char, ...);
    fn err_msg(fmt: *const c_char, ...);
}

unsafe fn errno_value() -> c_int {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        *libc::__errno_location()
    }
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    {
        0
    }
}

unsafe fn trace_event_name(tevent: *mut trace_events) -> *mut c_char {
    if (*tevent).event.is_null() {
        c"*".as_ptr() as *mut c_char
    } else {
        (*tevent).event
    }
}

/*
 * enable_tracer_by_name - enable a tracer on the given instance
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn enable_tracer_by_name(
    inst: *mut tracefs_instance,
    tracer_name: *const c_char,
) -> c_int {
    let tracer: tracefs_tracers;
    let retval: c_int;

    tracer = tracefs_tracers::TRACEFS_TRACER_CUSTOM;

    debug_msg(c"Enabling %s tracer\n".as_ptr(), tracer_name);

    retval = tracefs_tracer_set(inst, tracer, tracer_name);
    if retval < 0 {
        if errno_value() == ENODEV {
            err_msg(c"Tracer %s not found!\n".as_ptr(), tracer_name);
        }

        err_msg(c"Failed to enable the %s tracer\n".as_ptr(), tracer_name);
        return -1;
    }

    0
}

/*
 * disable_tracer - set nop tracer to the insta
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn disable_tracer(inst: *mut tracefs_instance) {
    let t: tracefs_tracers = tracefs_tracers::TRACEFS_TRACER_NOP;
    let retval: c_int;

    retval = tracefs_tracer_set(inst, t);
    if retval < 0 {
        err_msg(c"Oops, error disabling tracer\n".as_ptr());
    }
}

/*
 * create_instance - create a trace instance with *instance_name
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_instance(instance_name: *mut c_char) -> *mut tracefs_instance {
    tracefs_instance_create(instance_name)
}

/*
 * destroy_instance - remove a trace instance and free the data
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_instance(inst: *mut tracefs_instance) {
    tracefs_instance_destroy(inst);
    tracefs_instance_free(inst);
}

/*
 * save_trace_to_file - save the trace output of the instance to the file
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn save_trace_to_file(
    inst: *mut tracefs_instance,
    filename: *const c_char,
) -> c_int {
    let file: *const c_char = c"trace".as_ptr();
    let mode: mode_t = 0o644;
    let mut buffer = [0 as c_char; 4096];
    let out_fd: c_int;
    let in_fd: c_int;
    let mut retval: c_int = -1;
    let mut n_read: ssize_t;
    let mut n_written: ssize_t;

    if inst.is_null() || filename.is_null() {
        return 0;
    }

    in_fd = tracefs_instance_file_open(inst, file, O_RDONLY);
    if in_fd < 0 {
        err_msg(c"Failed to open trace file\n".as_ptr());
        return -1;
    }

    printf(c"  Saving trace to %s\n".as_ptr(), filename);
    out_fd = creat(filename, mode);
    if out_fd < 0 {
        err_msg(c"Failed to create output file %s\n".as_ptr(), filename);
        close(in_fd);
        return retval;
    }

    loop {
        n_read = read(
            in_fd,
            buffer.as_mut_ptr() as *mut c_void,
            core::mem::size_of_val(&buffer),
        );
        if n_read < 0 {
            if errno_value() == EINTR {
                continue;
            }
            err_msg(
                c"Error reading trace file: %s\n".as_ptr(),
                strerror(errno_value()),
            );
            break;
        }
        if n_read == 0 {
            retval = 0;
            break;
        }

        n_written = 0;
        while n_written < n_read {
            let w: ssize_t = write(
                out_fd,
                buffer.as_ptr().offset(n_written as isize) as *const c_void,
                (n_read - n_written) as usize,
            );

            if w < 0 {
                if errno_value() == EINTR {
                    continue;
                }
                err_msg(
                    c"Error writing trace file: %s\n".as_ptr(),
                    strerror(errno_value()),
                );
                close(out_fd);
                close(in_fd);
                return retval;
            }
            n_written += w;
        }
    }

    close(out_fd);
    close(in_fd);
    retval
}

/*
 * collect_registered_events - call the existing callback function for the event
 *
 * If an event has a registered callback function, call it.
 * Otherwise, ignore the event.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn collect_registered_events(
    event: *mut tep_event,
    record: *mut tep_record,
    cpu: c_int,
    context: *mut c_void,
) -> c_int {
    let trace: *mut trace_instance = context as *mut trace_instance;
    let s: *mut trace_seq = (*trace).seq;

    let _ = cpu;
    (*trace).processed_events = (*trace).processed_events.wrapping_add(1);

    if (*event).handler.is_none() {
        return 0;
    }

    ((*event).handler.unwrap())(s, record, event, context);

    0
}

/*
 * collect_missed_events - record number of missed events
 *
 * If rtla cannot keep up with events generated by tracer, events are going
 * to fall out of the ring buffer.
 * Collect how many events were missed so it can be reported to the user.
 */
unsafe extern "C" fn collect_missed_events(
    event: *mut tep_event,
    record: *mut tep_record,
    cpu: c_int,
    context: *mut c_void,
) -> c_int {
    let trace: *mut trace_instance = context as *mut trace_instance;

    let _ = event;
    let _ = cpu;
    if (*trace).missed_events == UINT64_MAX {
        return 0;
    }

    if (*record).missed_events > 0 {
        (*trace).missed_events = (*trace).missed_events.wrapping_add((*record).missed_events);
    } else {
        /* Events missed but no data on how many */
        (*trace).missed_events = UINT64_MAX;
    }

    0
}

/*
 * trace_instance_destroy - destroy and free a rtla trace instance
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_instance_destroy(trace: *mut trace_instance) {
    if !(*trace).inst.is_null() {
        disable_tracer((*trace).inst);
        destroy_instance((*trace).inst);
        (*trace).inst = core::ptr::null_mut();
    }

    if !(*trace).seq.is_null() {
        free((*trace).seq as *mut c_void);
        (*trace).seq = core::ptr::null_mut();
    }

    if !(*trace).tep.is_null() {
        tep_free((*trace).tep);
        (*trace).tep = core::ptr::null_mut();
    }
}

/*
 * trace_instance_init - create an rtla trace instance
 *
 * It is more than the tracefs instance, as it contains other
 * things required for the tracing, such as the local events and
 * a seq file.
 *
 * Note that the trace instance is returned disabled. This allows
 * the tool to apply some other configs, like setting priority
 * to the kernel threads, before starting generating trace entries.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_instance_init(
    trace: *mut trace_instance,
    tool_name: *mut c_char,
) -> c_int {
    (*trace).seq = calloc_fatal(1, core::mem::size_of::<trace_seq>()) as *mut trace_seq;

    trace_seq_init((*trace).seq);

    (*trace).inst = create_instance(tool_name);
    if (*trace).inst.is_null() {
        trace_instance_destroy(trace);
        return 1;
    }

    (*trace).tep = tracefs_local_events(core::ptr::null());
    if (*trace).tep.is_null() {
        trace_instance_destroy(trace);
        return 1;
    }

    /*
     * Let the main enable the record after setting some other
     * things such as the priority of the tracer's threads.
     */
    tracefs_trace_off((*trace).inst);

    /*
     * Collect the number of events missed due to tracefs buffer
     * overflow.
     */
    (*trace).missed_events = 0;
    tracefs_follow_missed_events((*trace).inst, Some(collect_missed_events), trace as *mut c_void);

    (*trace).processed_events = 0;

    0
}

/*
 * trace_instance_start - start tracing a given rtla instance
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_instance_start(trace: *mut trace_instance) -> c_int {
    tracefs_trace_on((*trace).inst)
}

/*
 * trace_instance_stop - stop tracing a given rtla instance
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_instance_stop(trace: *mut trace_instance) -> c_int {
    tracefs_trace_off((*trace).inst)
}

/*
 * trace_events_free - free a list of trace events
 */
unsafe fn trace_events_free(events: *mut trace_events) {
    let mut tevent: *mut trace_events = events;
    let mut free_event: *mut trace_events;

    while !tevent.is_null() {
        free_event = tevent;

        tevent = (*tevent).next;

        if !(*free_event).filter.is_null() {
            free((*free_event).filter as *mut c_void);
        }
        if !(*free_event).trigger.is_null() {
            free((*free_event).trigger as *mut c_void);
        }
        free((*free_event).system as *mut c_void);
        free(free_event as *mut c_void);
    }
}

/*
 * trace_event_alloc - alloc and parse a single trace event
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_event_alloc(event_string: *const c_char) -> *mut trace_events {
    let tevent: *mut trace_events;

    tevent = calloc_fatal(1, core::mem::size_of::<trace_events>()) as *mut trace_events;

    (*tevent).system = strdup_fatal(event_string);

    (*tevent).event = libc::strstr((*tevent).system, c":".as_ptr());
    if !(*tevent).event.is_null() {
        *(*tevent).event = b'\0' as c_char;
        (*tevent).event = (*tevent).event.offset(1);
    }

    tevent
}

/*
 * trace_event_add_filter - record an event filter
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_event_add_filter(event: *mut trace_events, filter: *mut c_char) {
    if !(*event).filter.is_null() {
        free((*event).filter as *mut c_void);
    }

    (*event).filter = strdup_fatal(filter);
}

/*
 * trace_event_add_trigger - record an event trigger action
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_event_add_trigger(event: *mut trace_events, trigger: *mut c_char) {
    if !(*event).trigger.is_null() {
        free((*event).trigger as *mut c_void);
    }

    (*event).trigger = strdup_fatal(trigger);
}

/*
 * trace_event_disable_filter - disable an event filter
 */
unsafe fn trace_event_disable_filter(instance: *mut trace_instance, tevent: *mut trace_events) {
    let mut filter = [0 as c_char; MAX_PATH];
    let retval: c_int;

    if (*tevent).filter.is_null() {
        return;
    }

    if (*tevent).filter_enabled == 0 {
        return;
    }

    debug_msg(
        c"Disabling %s:%s filter %s\n".as_ptr(),
        (*tevent).system,
        trace_event_name(tevent),
        (*tevent).filter,
    );

    snprintf(
        filter.as_mut_ptr(),
        filter.len(),
        c"!%s\n".as_ptr(),
        (*tevent).filter,
    );

    retval = tracefs_event_file_write(
        (*instance).inst,
        (*tevent).system,
        (*tevent).event,
        c"filter".as_ptr(),
        filter.as_ptr(),
    );
    if retval < 0 {
        err_msg(
            c"Error disabling %s:%s filter %s\n".as_ptr(),
            (*tevent).system,
            trace_event_name(tevent),
            (*tevent).filter,
        );
    }
}

/*
 * trace_event_save_hist - save the content of an event hist
 *
 * If the trigger is a hist: one, save the content of the hist file.
 */
unsafe fn trace_event_save_hist(instance: *mut trace_instance, tevent: *mut trace_events) {
    let mut index: usize;
    let hist_len: usize;
    let mode: mode_t = 0o644;
    let mut path = [0 as c_char; MAX_PATH];
    let hist: *mut c_char;
    let out_fd: c_int;

    if tevent.is_null() {
        return;
    }

    /* trigger enables hist */
    if (*tevent).trigger.is_null() {
        return;
    }

    /* is this a hist: trigger? */
    if str_has_prefix((*tevent).trigger, c"hist:".as_ptr()) == 0 {
        return;
    }

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        c"%s_%s_hist.txt".as_ptr(),
        (*tevent).system,
        (*tevent).event,
    );

    printf(
        c"  Saving event %s:%s hist to %s\n".as_ptr(),
        (*tevent).system,
        (*tevent).event,
        path.as_ptr(),
    );

    out_fd = creat(path.as_ptr(), mode);
    if out_fd < 0 {
        err_msg(c"  Failed to create %s output file\n".as_ptr(), path.as_ptr());
        return;
    }

    hist = tracefs_event_file_read(
        (*instance).inst,
        (*tevent).system,
        (*tevent).event,
        c"hist".as_ptr(),
        0,
    );
    if hist.is_null() {
        err_msg(
            c"  Failed to read %s:%s hist file\n".as_ptr(),
            (*tevent).system,
            (*tevent).event,
        );
        close(out_fd);
        return;
    }

    index = 0;
    hist_len = strlen(hist);
    loop {
        let written: ssize_t = write(
            out_fd,
            hist.add(index) as *const c_void,
            hist_len.wrapping_sub(index),
        );

        if written < 0 {
            if errno_value() == EINTR {
                continue;
            }
            err_msg(
                c"  Error writing hist file: %s\n".as_ptr(),
                strerror(errno_value()),
            );
            break;
        }
        index = index.wrapping_add(written as usize);
        if index >= hist_len {
            break;
        }
    }

    free(hist as *mut c_void);
    close(out_fd);
}

/*
 * trace_event_disable_trigger - disable an event trigger
 */
unsafe fn trace_event_disable_trigger(instance: *mut trace_instance, tevent: *mut trace_events) {
    let mut trigger = [0 as c_char; MAX_PATH];
    let retval: c_int;

    if (*tevent).trigger.is_null() {
        return;
    }

    if (*tevent).trigger_enabled == 0 {
        return;
    }

    debug_msg(
        c"Disabling %s:%s trigger %s\n".as_ptr(),
        (*tevent).system,
        trace_event_name(tevent),
        (*tevent).trigger,
    );

    trace_event_save_hist(instance, tevent);

    snprintf(
        trigger.as_mut_ptr(),
        trigger.len(),
        c"!%s\n".as_ptr(),
        (*tevent).trigger,
    );

    retval = tracefs_event_file_write(
        (*instance).inst,
        (*tevent).system,
        (*tevent).event,
        c"trigger".as_ptr(),
        trigger.as_ptr(),
    );
    if retval < 0 {
        err_msg(
            c"Error disabling %s:%s trigger %s\n".as_ptr(),
            (*tevent).system,
            trace_event_name(tevent),
            (*tevent).trigger,
        );
    }
}

/*
 * trace_events_disable - disable all trace events
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_events_disable(
    instance: *mut trace_instance,
    events: *mut trace_events,
) {
    let mut tevent: *mut trace_events = events;

    if events.is_null() {
        return;
    }

    while !tevent.is_null() {
        debug_msg(
            c"Disabling event %s:%s\n".as_ptr(),
            (*tevent).system,
            trace_event_name(tevent),
        );
        if (*tevent).enabled != 0 {
            trace_event_disable_filter(instance, tevent);
            trace_event_disable_trigger(instance, tevent);
            tracefs_event_disable((*instance).inst, (*tevent).system, (*tevent).event);
        }

        (*tevent).enabled = 0;
        tevent = (*tevent).next;
    }
}

/*
 * trace_event_enable_filter - enable an event filter associated with an event
 */
unsafe fn trace_event_enable_filter(
    instance: *mut trace_instance,
    tevent: *mut trace_events,
) -> c_int {
    let mut filter = [0 as c_char; MAX_PATH];
    let retval: c_int;

    if (*tevent).filter.is_null() {
        return 0;
    }

    if (*tevent).event.is_null() {
        err_msg(
            c"Filter %s applies only for single events, not for all %s:* events\n".as_ptr(),
            (*tevent).filter,
            (*tevent).system,
        );
        return 1;
    }

    snprintf(
        filter.as_mut_ptr(),
        filter.len(),
        c"%s\n".as_ptr(),
        (*tevent).filter,
    );

    debug_msg(
        c"Enabling %s:%s filter %s\n".as_ptr(),
        (*tevent).system,
        trace_event_name(tevent),
        (*tevent).filter,
    );

    retval = tracefs_event_file_write(
        (*instance).inst,
        (*tevent).system,
        (*tevent).event,
        c"filter".as_ptr(),
        filter.as_ptr(),
    );
    if retval < 0 {
        err_msg(
            c"Error enabling %s:%s filter %s\n".as_ptr(),
            (*tevent).system,
            trace_event_name(tevent),
            (*tevent).filter,
        );
        return 1;
    }

    (*tevent).filter_enabled = 1;
    0
}

/*
 * trace_event_enable_trigger - enable an event trigger associated with an event
 */
unsafe fn trace_event_enable_trigger(
    instance: *mut trace_instance,
    tevent: *mut trace_events,
) -> c_int {
    let mut trigger = [0 as c_char; MAX_PATH];
    let retval: c_int;

    if (*tevent).trigger.is_null() {
        return 0;
    }

    if (*tevent).event.is_null() {
        err_msg(
            c"Trigger %s applies only for single events, not for all %s:* events\n".as_ptr(),
            (*tevent).trigger,
            (*tevent).system,
        );
        return 1;
    }

    snprintf(
        trigger.as_mut_ptr(),
        trigger.len(),
        c"%s\n".as_ptr(),
        (*tevent).trigger,
    );

    debug_msg(
        c"Enabling %s:%s trigger %s\n".as_ptr(),
        (*tevent).system,
        trace_event_name(tevent),
        (*tevent).trigger,
    );

    retval = tracefs_event_file_write(
        (*instance).inst,
        (*tevent).system,
        (*tevent).event,
        c"trigger".as_ptr(),
        trigger.as_ptr(),
    );
    if retval < 0 {
        err_msg(
            c"Error enabling %s:%s trigger %s\n".as_ptr(),
            (*tevent).system,
            trace_event_name(tevent),
            (*tevent).trigger,
        );
        return 1;
    }

    (*tevent).trigger_enabled = 1;

    0
}

/*
 * trace_events_enable - enable all events
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_events_enable(
    instance: *mut trace_instance,
    events: *mut trace_events,
) -> c_int {
    let mut tevent: *mut trace_events = events;
    let mut retval: c_int;

    while !tevent.is_null() {
        debug_msg(
            c"Enabling event %s:%s\n".as_ptr(),
            (*tevent).system,
            trace_event_name(tevent),
        );
        retval = tracefs_event_enable((*instance).inst, (*tevent).system, (*tevent).event);
        if retval < 0 {
            err_msg(
                c"Error enabling event %s:%s\n".as_ptr(),
                (*tevent).system,
                trace_event_name(tevent),
            );
            return 1;
        }

        retval = trace_event_enable_filter(instance, tevent);
        if retval != 0 {
            return 1;
        }

        retval = trace_event_enable_trigger(instance, tevent);
        if retval != 0 {
            return 1;
        }

        (*tevent).enabled = 1;
        tevent = (*tevent).next;
    }

    0
}

/*
 * trace_events_destroy - disable and free all trace events
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_events_destroy(
    instance: *mut trace_instance,
    events: *mut trace_events,
) {
    if events.is_null() {
        return;
    }

    trace_events_disable(instance, events);
    trace_events_free(events);
}

/*
 * trace_set_buffer_size - set the per-cpu tracing buffer size.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_set_buffer_size(
    trace: *mut trace_instance,
    size: c_int,
) -> c_int {
    let retval: c_int;

    debug_msg(c"Setting trace buffer size to %d Kb\n".as_ptr(), size);
    retval = tracefs_instance_set_buffer_size((*trace).inst, size, -1);
    if retval != 0 {
        err_msg(c"Error setting trace buffer size\n".as_ptr());
    }

    retval
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
