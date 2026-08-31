// SPDX-License-Identifier: GPL-2.0
/*
 * trace helpers.
 *
 * Copyright (C) 2022 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

// C dependencies from:
// <sys/sendfile.h>, <tracefs.h>, <signal.h>, <stdlib.h>, <unistd.h>, <errno.h>
// <rv.h>, <trace.h>, <utils.h>

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct tracefs_instance {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_record {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_seq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

pub type TepEventHandler = Option<
    unsafe extern "C" fn(
        s: *mut trace_seq,
        record: *mut tep_record,
        event: *mut tep_event,
        context: *mut c_void,
    ),
>;

#[repr(C)]
pub struct tep_event {
    pub handler: TepEventHandler,
}

#[repr(C)]
pub struct trace_instance {
    pub inst: *mut tracefs_instance,
    pub seq: *mut trace_seq,
    pub tep: *mut tep_handle,
}

unsafe extern "C" {
    fn tracefs_instance_create(instance_name: *mut c_char) -> *mut tracefs_instance;
    fn tracefs_instance_destroy(inst: *mut tracefs_instance);
    fn tracefs_instance_free(inst: *mut tracefs_instance);
    fn should_stop() -> c_int;
    fn free(ptr: *mut c_void);
    fn tep_free(tep: *mut tep_handle);
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn trace_seq_init(s: *mut trace_seq);
    fn tracefs_local_events(tracing_dir: *mut c_void) -> *mut tep_handle;
    fn tracefs_trace_off(instance: *mut tracefs_instance) -> c_int;
    fn tracefs_trace_on(instance: *mut tracefs_instance) -> c_int;
}

/*
 * create_instance - create a trace instance with *instance_name
 */
unsafe fn create_instance(instance_name: *mut c_char) -> *mut tracefs_instance {
    unsafe { tracefs_instance_create(instance_name) }
}

/*
 * destroy_instance - remove a trace instance and free the data
 */
unsafe fn destroy_instance(inst: *mut tracefs_instance) {
    unsafe {
        tracefs_instance_destroy(inst);
        tracefs_instance_free(inst);
    }
}

/**
 * collect_registered_events - call the existing callback function for the event
 *
 * If an event has a registered callback function, call it.
 * Otherwise, ignore the event.
 *
 * Returns 0 if the event was collected, 1 if the tool should stop collecting trace.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn collect_registered_events(
    event: *mut tep_event,
    record: *mut tep_record,
    cpu: c_int,
    context: *mut c_void,
) -> c_int {
    let trace: *mut trace_instance = context as *mut trace_instance;
    let s: *mut trace_seq = unsafe { (*trace).seq };

    let _ = cpu;

    if unsafe { should_stop() } != 0 {
        return 1;
    }

    if unsafe { (*event).handler.is_none() } {
        return 0;
    }

    unsafe {
        ((*event).handler.unwrap())(s, record, event, context);
    }

    0
}

/**
 * trace_instance_destroy - destroy and free a rv trace instance
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_instance_destroy(trace: *mut trace_instance) {
    unsafe {
        if !(*trace).inst.is_null() {
            destroy_instance((*trace).inst);
            (*trace).inst = ptr::null_mut();
        }

        if !(*trace).seq.is_null() {
            free((*trace).seq as *mut c_void);
            (*trace).seq = ptr::null_mut();
        }

        if !(*trace).tep.is_null() {
            tep_free((*trace).tep);
            (*trace).tep = ptr::null_mut();
        }
    }
}

/**
 * trace_instance_init - create a trace instance
 *
 * It is more than the tracefs instance, as it contains other
 * things required for the tracing, such as the local events and
 * a seq file.
 *
 * Note that the trace instance is returned disabled. This allows
 * the tool to apply some other configs, like setting priority
 * to the kernel threads, before starting generating trace entries.
 *
 * Returns 0 on success, non-zero otherwise.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_instance_init(
    trace: *mut trace_instance,
    name: *mut c_char,
) -> c_int {
    unsafe {
        (*trace).seq = calloc(1, size_of::<trace_seq>()) as *mut trace_seq;
        if (*trace).seq.is_null() {
            trace_instance_destroy(trace);
            return 1;
        }

        trace_seq_init((*trace).seq);

        (*trace).inst = create_instance(name);
        if (*trace).inst.is_null() {
            trace_instance_destroy(trace);
            return 1;
        }

        (*trace).tep = tracefs_local_events(ptr::null_mut());
        if (*trace).tep.is_null() {
            trace_instance_destroy(trace);
            return 1;
        }

        /*
         * Let the main enable the record after setting some other
         * things such as the priority of the tracer's threads.
         */
        tracefs_trace_off((*trace).inst);
    }

    0
}

/**
 * trace_instance_start - start tracing a given rv instance
 *
 * Returns 0 on success, -1 otherwise.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_instance_start(trace: *mut trace_instance) -> c_int {
    unsafe { tracefs_trace_on((*trace).inst) }
}
