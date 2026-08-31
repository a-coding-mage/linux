// SPDX-License-Identifier: GPL-2.0
/*
 * in kernel monitor support: allows rv to control in-kernel monitors.
 *
 * Copyright (C) 2022 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

use core::ffi::{c_char, c_int, c_longlong, c_uint, c_ulonglong, c_void};

// Dependencies from <trace.h>, <utils.h>, <rv.h>, libc, and tracefs/tep headers.
use crate::{
    collect_registered_events, config_debug, debug_msg, err_msg, should_stop, MAX_DA_NAME_LEN,
    MAX_PATH, TEP_PRINT_COMM, TEP_PRINT_CPU, TEP_PRINT_NAME, TEP_PRINT_PID,
};

#[repr(C)]
pub struct monitor {
    pub name: [c_char; MAX_DA_NAME_LEN],
    pub desc: [c_char; MAX_PATH],
    pub enabled: c_int,
    pub nested: c_int,
}

#[repr(C)]
pub struct trace_instance {
    pub inst: *mut c_void,
    pub tep: *mut tep_handle,
    pub seq: *mut trace_seq,
}

#[repr(C)]
pub struct trace_seq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_record {
    pub cpu: c_int,
}

#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_event {
    pub tep: *mut tep_handle,
}

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: c_ulonglong,
    pub d_off: c_longlong,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

const no_argument: c_int = 0;
const required_argument: c_int = 1;
const DT_DIR: u8 = 4;

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn vsnprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ap: VaList) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn getpid() -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    static mut stderr: *mut c_void;

    fn tracefs_instance_file_read_number(
        instance: *mut c_void,
        file: *const c_char,
        val: *mut c_longlong,
    ) -> c_int;
    fn tracefs_instance_file_read(
        instance: *mut c_void,
        file: *const c_char,
        size: *mut usize,
    ) -> *mut c_char;
    fn tracefs_instance_file_write(
        instance: *mut c_void,
        file: *const c_char,
        str_: *const c_char,
    ) -> c_int;
    fn tracefs_instance_get_file(instance: *mut c_void, file: *const c_char) -> *mut c_char;
    fn tracefs_event_enable(instance: *mut c_void, system: *const c_char, event: *const c_char)
        -> c_int;
    fn tracefs_trace_on(instance: *mut c_void) -> c_int;
    fn tracefs_iterate_raw_events(
        tep: *mut tep_handle,
        instance: *mut c_void,
        cpus: *mut c_void,
        cpu_size: c_int,
        callback: unsafe extern "C" fn(
            *mut tep_event,
            *mut tep_record,
            c_int,
            *mut c_void,
        ) -> c_int,
        context: *mut c_void,
    ) -> c_int;

    fn trace_instance_init(inst: *mut trace_instance, name: *mut c_char) -> c_int;
    fn trace_instance_destroy(inst: *mut trace_instance);
    fn trace_seq_printf(s: *mut trace_seq, fmt: *const c_char, ...) -> c_int;
    fn trace_seq_do_printf(s: *mut trace_seq);
    fn trace_seq_reset(s: *mut trace_seq);

    fn tep_register_event_handler(
        tep: *mut tep_handle,
        id: c_int,
        sys_name: *const c_char,
        event_name: *const c_char,
        func: unsafe extern "C" fn(
            *mut trace_seq,
            *mut tep_record,
            *mut tep_event,
            *mut c_void,
        ) -> c_int,
        context: *mut c_void,
    ) -> c_int;
    fn tep_get_field_val(
        s: *mut trace_seq,
        event: *mut tep_event,
        name: *const c_char,
        record: *mut tep_record,
        val: *mut c_ulonglong,
        err: c_int,
    ) -> bool;
    fn tep_get_common_field_val(
        s: *mut trace_seq,
        event: *mut tep_event,
        name: *const c_char,
        record: *mut tep_record,
        val: *mut c_ulonglong,
        err: c_int,
    ) -> c_int;
    fn tep_print_event(
        tep: *mut tep_handle,
        s: *mut trace_seq,
        record: *mut tep_record,
        fmt: *const c_char,
        ...
    ) -> c_int;
    fn tep_get_field_raw(
        s: *mut trace_seq,
        event: *mut tep_event,
        name: *const c_char,
        record: *mut tep_record,
        len: *mut c_int,
        err: c_int,
    ) -> *mut c_char;
}

// Rust has no stable way to define this C variadic helper exactly; keep the
// source-level intent for the va_list used by ikm_usage.
type VaList = *mut c_void;

static mut config_has_id: c_int = 0;
static mut config_is_container: c_int = 0;
static mut config_my_pid: c_int = 0;
static mut config_trace: c_int = 0;

static mut config_initial_reactor: *mut c_char = core::ptr::null_mut();
static mut config_reactor: *mut c_char = core::ptr::null_mut();

/*
 * __ikm_read_enable - reads monitor's enable status
 *
 * __does not log errors.
 *
 * Returns the current status, or -1 if the monitor does not exist,
 * __hence not logging errors.
 */
unsafe fn __ikm_read_enable(monitor_name: *mut c_char) -> c_int {
    let mut path = [0 as c_char; MAX_PATH];
    let mut enabled: c_longlong = 0;
    let retval: c_int;

    snprintf(
        path.as_mut_ptr(),
        MAX_PATH,
        c"rv/monitors/%s/enable".as_ptr(),
        monitor_name,
    );

    retval = tracefs_instance_file_read_number(core::ptr::null_mut(), path.as_ptr(), &mut enabled);
    if retval < 0 {
        return -1;
    }

    enabled as c_int
}

/*
 * __ikm_find_monitor - find the full name of a possibly nested module
 *
 * __does not log errors.
 *
 * Returns 1 if we found the monitor, -1 on error and 0 if it does not exist.
 * The string out_name is populated with the full name, which can be
 * equal to monitor_name or container/monitor_name if nested
 */
unsafe fn __ikm_find_monitor_name(monitor_name: *mut c_char, out_name: *mut c_char) -> c_int {
    let mut available_monitors: *mut c_char;
    let mut cursor: *mut c_char;
    let mut line: *mut c_char;
    let len = strlen(monitor_name);
    let mut found: c_int = 0;

    available_monitors =
        tracefs_instance_file_read(core::ptr::null_mut(), c"rv/available_monitors".as_ptr(), core::ptr::null_mut());
    if available_monitors.is_null() {
        return -1;
    }

    config_is_container = 0;
    cursor = available_monitors;
    loop {
        line = strsep(&mut cursor, c"\n".as_ptr());
        if line.is_null() {
            break;
        }
        let colon = strchr(line, ':' as c_int);

        if strcmp(line, monitor_name) != 0
            && (colon.is_null() || strcmp(colon.add(1), monitor_name) != 0)
        {
            continue;
        }

        strncpy(out_name, line, 2 * MAX_DA_NAME_LEN);
        *out_name.add(2 * MAX_DA_NAME_LEN - 1) = '\0' as c_char;

        if !colon.is_null() {
            *out_name.add(colon.offset_from(line) as usize) = '/' as c_char;
        } else {
            /* If there are children, they are on the next line. */
            line = strsep(&mut cursor, c"\n".as_ptr());
            if !line.is_null() && strncmp(line, monitor_name, len) == 0 && *line.add(len) == ':' as c_char {
                config_is_container = 1;
            }
        }

        found = 1;
        break;
    }

    free(available_monitors as *mut c_void);
    found
}

/*
 * ikm_read_enable - reads monitor's enable status
 *
 * Returns the current status, or -1 on error.
 */
unsafe fn ikm_read_enable(monitor_name: *mut c_char) -> c_int {
    let enabled = __ikm_read_enable(monitor_name);
    if enabled < 0 {
        err_msg(c"ikm: fail read enabled: %d\n".as_ptr(), enabled);
        return -1;
    }

    debug_msg(c"ikm: read enabled: %d\n".as_ptr(), enabled);

    enabled
}

/*
 * ikm_write_enable - write to the monitor's enable file
 *
 * Return the number of bytes written, -1 on error.
 */
unsafe fn ikm_write_enable(monitor_name: *mut c_char, enable_disable: *mut c_char) -> c_int {
    let mut path = [0 as c_char; MAX_PATH];
    let retval: c_int;

    debug_msg(c"ikm: writing enabled: %s\n".as_ptr(), enable_disable);

    snprintf(
        path.as_mut_ptr(),
        MAX_PATH,
        c"rv/monitors/%s/enable".as_ptr(),
        monitor_name,
    );
    retval = tracefs_instance_file_write(core::ptr::null_mut(), path.as_ptr(), enable_disable);
    if retval < strlen(enable_disable) as c_int {
        err_msg(c"ikm: writing enabled: %s\n".as_ptr(), enable_disable);
        return -1;
    }

    retval
}

/*
 * ikm_enable - enable a monitor
 *
 * Returns -1 on failure. Success otherwise.
 */
unsafe fn ikm_enable(monitor_name: *mut c_char) -> c_int {
    ikm_write_enable(monitor_name, c"1".as_ptr() as *mut c_char)
}

/*
 * ikm_disable - disable a monitor
 *
 * Returns -1 on failure. Success otherwise.
 */
unsafe fn ikm_disable(monitor_name: *mut c_char) -> c_int {
    ikm_write_enable(monitor_name, c"0".as_ptr() as *mut c_char)
}

/*
 * ikm_read_desc - read monitors' description
 *
 * Return a dynamically allocated string with the monitor's
 * description, NULL otherwise.
 */
unsafe fn ikm_read_desc(monitor_name: *mut c_char) -> *mut c_char {
    let mut path = [0 as c_char; MAX_PATH];
    let desc: *mut c_char;

    snprintf(
        path.as_mut_ptr(),
        MAX_PATH,
        c"rv/monitors/%s/desc".as_ptr(),
        monitor_name,
    );
    desc = tracefs_instance_file_read(core::ptr::null_mut(), path.as_ptr(), core::ptr::null_mut());
    if desc.is_null() {
        err_msg(c"ikm: error reading monitor %s desc\n".as_ptr(), monitor_name);
        return core::ptr::null_mut();
    }

    *strstr(desc, c"\n".as_ptr()) = '\0' as c_char;

    desc
}

/*
 * ikm_fill_monitor_definition - fill monitor's definition
 *
 * Returns -1 on error, 1 if the monitor does not belong in the container, 0 otherwise.
 * container can be NULL
 */
unsafe fn ikm_fill_monitor_definition(
    name: *mut c_char,
    ikm: *mut monitor,
    container: *mut c_char,
) -> c_int {
    let enabled: c_int;
    let desc: *mut c_char;
    let mut nested_name: *mut c_char;

    nested_name = strstr(name, c":".as_ptr());
    if !nested_name.is_null() {
        /* it belongs in container if it starts with "container:" */
        if !container.is_null() {
            let len = strlen(container);

            if strncmp(name, container, len) != 0 || *name.add(len) != ':' as c_char {
                return 1;
            }
        }
        *nested_name = '/' as c_char;
        nested_name = nested_name.add(1);
        (*ikm).nested = 1;
    } else {
        if !container.is_null() {
            return 1;
        }
        nested_name = name;
        (*ikm).nested = 0;
    }

    enabled = ikm_read_enable(name);
    if enabled < 0 {
        err_msg(c"ikm: monitor %s fail to read enable file, bug?\n".as_ptr(), name);
        return -1;
    }

    desc = ikm_read_desc(name);
    if desc.is_null() {
        err_msg(c"ikm: monitor %s does not have desc file, bug?\n".as_ptr(), name);
        return -1;
    }

    strncpy((*ikm).name.as_mut_ptr(), nested_name, (*ikm).name.len() - 1);
    (*ikm).name[(*ikm).name.len() - 1] = '\0' as c_char;
    (*ikm).enabled = enabled;
    strncpy((*ikm).desc.as_mut_ptr(), desc, (*ikm).desc.len() - 1);
    (*ikm).desc[(*ikm).desc.len() - 1] = '\0' as c_char;
    free(desc as *mut c_void);

    0
}

/*
 * ikm_write_reactor - switch the reactor to *reactor
 *
 * Return the number or characters written, -1 on error.
 */
unsafe fn ikm_write_reactor(monitor_name: *mut c_char, reactor: *mut c_char) -> c_int {
    let mut path = [0 as c_char; MAX_PATH];
    let retval: c_int;

    snprintf(
        path.as_mut_ptr(),
        MAX_PATH,
        c"rv/monitors/%s/reactors".as_ptr(),
        monitor_name,
    );
    retval = tracefs_instance_file_write(core::ptr::null_mut(), path.as_ptr(), reactor);
    debug_msg(c"ikm: write \"%s\" reactors: %d\n".as_ptr(), reactor, retval);

    retval
}

/*
 * ikm_read_reactor - read the reactors file
 *
 * Returns a dynamically allocated string with monitor's
 * available reactors, or NULL on error.
 */
unsafe fn ikm_read_reactor(monitor_name: *mut c_char) -> *mut c_char {
    let mut path = [0 as c_char; MAX_PATH];
    let reactors: *mut c_char;

    snprintf(
        path.as_mut_ptr(),
        MAX_PATH,
        c"rv/monitors/%s/reactors".as_ptr(),
        monitor_name,
    );
    reactors = tracefs_instance_file_read(core::ptr::null_mut(), path.as_ptr(), core::ptr::null_mut());
    if reactors.is_null() {
        err_msg(c"ikm: fail reading monitor's %s reactors file\n".as_ptr(), monitor_name);
        return core::ptr::null_mut();
    }

    reactors
}

/*
 * ikm_get_current_reactor - get the current enabled reactor
 *
 * Reads the reactors file and find the currently enabled
 * [reactor].
 *
 * Returns a dynamically allocated memory with the current
 * reactor. NULL otherwise.
 */
unsafe fn ikm_get_current_reactor(monitor_name: *mut c_char) -> *mut c_char {
    let reactors = ikm_read_reactor(monitor_name);
    let mut curr_reactor: *mut c_char = core::ptr::null_mut();
    let mut start: *mut c_char;
    let end: *mut c_char;

    if reactors.is_null() {
        return core::ptr::null_mut();
    }

    start = strstr(reactors, c"[".as_ptr());
    if start.is_null() {
        free(reactors as *mut c_void);
        return curr_reactor;
    }

    start = start.add(1);

    end = strstr(start, c"]".as_ptr());
    if end.is_null() {
        free(reactors as *mut c_void);
        return curr_reactor;
    }

    *end = '\0' as c_char;

    curr_reactor = calloc(strlen(start) + 1, core::mem::size_of::<c_char>()) as *mut c_char;
    if curr_reactor.is_null() {
        free(reactors as *mut c_void);
        return curr_reactor;
    }

    strncpy(curr_reactor, start, strlen(start));
    debug_msg(c"ikm: read current reactor %s\n".as_ptr(), curr_reactor);

    free(reactors as *mut c_void);

    curr_reactor
}

unsafe fn ikm_has_id(monitor_name: *mut c_char) -> c_int {
    let mut path = [0 as c_char; MAX_PATH];
    let format: *mut c_char;
    let has_id: c_int;

    snprintf(
        path.as_mut_ptr(),
        MAX_PATH,
        c"events/rv/event_%s/format".as_ptr(),
        monitor_name,
    );
    format = tracefs_instance_file_read(core::ptr::null_mut(), path.as_ptr(), core::ptr::null_mut());
    if format.is_null() {
        err_msg(c"ikm: fail reading monitor's %s format event file\n".as_ptr(), monitor_name);
        return -1;
    }

    /* print fmt: "%d: %s x %s -> %s %s", REC->id, ... */
    has_id = (!strstr(format, c"REC->id".as_ptr()).is_null()) as c_int;

    debug_msg(
        c"ikm: monitor %s has id: %s\n".as_ptr(),
        monitor_name,
        if has_id != 0 { c"yes".as_ptr() } else { c"no".as_ptr() },
    );

    free(format as *mut c_void);

    has_id
}

/**
 * ikm_list_monitors - list all available monitors
 *
 * Returns 0 on success, -1 otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn ikm_list_monitors(container: *mut c_char) -> c_int {
    let available_monitors: *mut c_char;
    let mut ikm: monitor = core::mem::zeroed();
    let mut curr: *mut c_char;
    let mut next: *mut c_char;
    let mut retval: c_int;
    let mut list_monitor: c_int = 0;

    available_monitors =
        tracefs_instance_file_read(core::ptr::null_mut(), c"rv/available_monitors".as_ptr(), core::ptr::null_mut());

    if available_monitors.is_null() {
        err_msg(c"ikm: available monitors is not available, is CONFIG_RV enabled?\n".as_ptr());
        return -1;
    }

    curr = available_monitors;
    loop {
        next = strstr(curr, c"\n".as_ptr());
        *next = '\0' as c_char;

        retval = ikm_fill_monitor_definition(curr, &mut ikm, container);
        if retval < 0 {
            err_msg(c"ikm: error reading %d in kernel monitor, skipping\n".as_ptr(), curr);
        }

        if retval == 0 {
            let indent = (ikm.nested != 0 && container.is_null()) as c_int;

            list_monitor = 1;
            printf(
                c"%s%-*s %s %s\n".as_ptr(),
                if indent != 0 { c" - ".as_ptr() } else { c"".as_ptr() },
                if indent != 0 { MAX_DA_NAME_LEN as c_int - 3 } else { MAX_DA_NAME_LEN as c_int },
                ikm.name.as_ptr(),
                ikm.desc.as_ptr(),
                if ikm.enabled != 0 { c"[ON]".as_ptr() } else { c"[OFF]".as_ptr() },
            );
        }
        next = next.add(1);
        curr = next;

        if strlen(curr) == 0 {
            break;
        }
    }

    if list_monitor == 0 {
        if !container.is_null() {
            printf(c"-- No monitor found in container %s --\n".as_ptr(), container);
        } else {
            printf(c"-- No monitor found --\n".as_ptr());
        }
    }

    free(available_monitors as *mut c_void);

    0
}

unsafe fn ikm_print_header(s: *mut trace_seq) {
    trace_seq_printf(s, c"%16s-%-8s %5s %5s ".as_ptr(), c"<TASK>".as_ptr(), c"PID".as_ptr(), c"[CPU]".as_ptr(), c"TYPE".as_ptr());
    if config_has_id != 0 {
        trace_seq_printf(s, c"%8s ".as_ptr(), c"ID".as_ptr());
    }

    trace_seq_printf(
        s,
        c"%24s x %-24s -> %-24s %s\n".as_ptr(),
        c"STATE".as_ptr(),
        c"EVENT".as_ptr(),
        c"NEXT_STATE".as_ptr(),
        c"FINAL".as_ptr(),
    );

    trace_seq_printf(s, c"%16s %-8s %5s %5s ".as_ptr(), c" | ".as_ptr(), c" | ".as_ptr(), c" | ".as_ptr(), c" | ".as_ptr());

    if config_has_id != 0 {
        trace_seq_printf(s, c"%8s ".as_ptr(), c" | ".as_ptr());
    }

    trace_seq_printf(
        s,
        c"%24s   %-24s    %-24s %s\n".as_ptr(),
        c" | ".as_ptr(),
        c" | ".as_ptr(),
        c" | ".as_ptr(),
        c"|".as_ptr(),
    );
}

/*
 * ikm_event_handler - callback to handle event events
 *
 * Called any time a rv:"monitor"_event events is generated.
 * It parses and prints event.
 */
unsafe extern "C" fn ikm_event_handler(
    s: *mut trace_seq,
    record: *mut tep_record,
    trace_event: *mut tep_event,
    _context: *mut c_void,
) -> c_int {
    /* if needed: struct trace_instance *inst = context; */
    let state: *mut c_char;
    let event: *mut c_char;
    let next_state: *mut c_char;
    let mut final_state: c_ulonglong = 0;
    let mut pid: c_ulonglong = 0;
    let mut id: c_ulonglong = 0;
    let mut val: c_int = 0;
    let mut missing_id: bool = false;

    if config_has_id != 0 {
        missing_id = tep_get_field_val(s, trace_event, c"id".as_ptr(), record, &mut id, 1);
    }

    tep_get_common_field_val(s, trace_event, c"common_pid".as_ptr(), record, &mut pid, 1);

    if config_has_id != 0 && config_my_pid as c_ulonglong == id {
        return 0;
    } else if config_my_pid as c_ulonglong == pid {
        return 0;
    }

    tep_print_event(
        (*trace_event).tep,
        s,
        record,
        c"%16s-%-8d [%.3d] ".as_ptr(),
        TEP_PRINT_COMM,
        TEP_PRINT_PID,
        TEP_PRINT_CPU,
    );

    if config_is_container != 0 {
        tep_print_event((*trace_event).tep, s, record, c"%s ".as_ptr(), TEP_PRINT_NAME);
    } else {
        trace_seq_printf(s, c"event ".as_ptr());
    }

    if config_has_id != 0 {
        if missing_id {
            /* placeholder if we are dealing with a mixed-type container*/
            trace_seq_printf(s, c"        ".as_ptr());
        } else {
            trace_seq_printf(s, c"%8llu ".as_ptr(), id);
        }
    }

    state = tep_get_field_raw(s, trace_event, c"state".as_ptr(), record, &mut val, 0);
    event = tep_get_field_raw(s, trace_event, c"event".as_ptr(), record, &mut val, 0);
    next_state = tep_get_field_raw(s, trace_event, c"next_state".as_ptr(), record, &mut val, 0);
    tep_get_field_val(
        s,
        trace_event,
        c"final_state".as_ptr(),
        record,
        &mut final_state,
        1,
    );

    trace_seq_printf(
        s,
        c"%24s x %-24s -> %-24s %s\n".as_ptr(),
        state,
        event,
        next_state,
        if final_state != 0 { c"Y".as_ptr() } else { c"N".as_ptr() },
    );

    trace_seq_do_printf(s);
    trace_seq_reset(s);

    0
}

/*
 * ikm_error_handler - callback to handle error events
 *
 * Called any time a rv:"monitor"_errors events is generated.
 * It parses and prints event.
 */
unsafe extern "C" fn ikm_error_handler(
    s: *mut trace_seq,
    record: *mut tep_record,
    trace_event: *mut tep_event,
    _context: *mut c_void,
) -> c_int {
    let mut pid: c_ulonglong = 0;
    let mut id: c_ulonglong = 0;
    let cpu = (*record).cpu;
    let state: *mut c_char;
    let event: *mut c_char;
    let mut val: c_int = 0;
    let mut missing_id: bool = false;

    if config_has_id != 0 {
        missing_id = tep_get_field_val(s, trace_event, c"id".as_ptr(), record, &mut id, 1);
    }

    tep_get_common_field_val(s, trace_event, c"common_pid".as_ptr(), record, &mut pid, 1);

    if config_has_id != 0 && config_my_pid as c_ulonglong == id {
        return 0;
    } else if config_my_pid as c_ulonglong == pid {
        return 0;
    }

    trace_seq_printf(s, c"%8lld [%03d] ".as_ptr(), pid, cpu);

    if config_is_container != 0 {
        tep_print_event((*trace_event).tep, s, record, c"%s ".as_ptr(), TEP_PRINT_NAME);
    } else {
        trace_seq_printf(s, c"error ".as_ptr());
    }

    if config_has_id != 0 {
        if missing_id {
            /* placeholder if we are dealing with a mixed-type container*/
            trace_seq_printf(s, c"        ".as_ptr());
        } else {
            trace_seq_printf(s, c"%8llu ".as_ptr(), id);
        }
    }

    state = tep_get_field_raw(s, trace_event, c"state".as_ptr(), record, &mut val, 0);
    event = tep_get_field_raw(s, trace_event, c"event".as_ptr(), record, &mut val, 0);

    trace_seq_printf(s, c"%24s x %s\n".as_ptr(), state, event);

    trace_seq_do_printf(s);
    trace_seq_reset(s);

    0
}

unsafe fn ikm_enable_trace_events(monitor_name: *mut c_char, inst: *mut trace_instance) -> c_int {
    let mut event = [0 as c_char; MAX_DA_NAME_LEN + 7]; /* max(error_,event_) + '0' = 7 */
    let mut retval: c_int;

    snprintf(event.as_mut_ptr(), event.len(), c"event_%s".as_ptr(), monitor_name);
    retval = tracefs_event_enable((*inst).inst, c"rv".as_ptr(), event.as_ptr());
    if retval != 0 {
        return -1;
    }

    tep_register_event_handler(
        (*inst).tep,
        -1,
        c"rv".as_ptr(),
        event.as_ptr(),
        ikm_event_handler,
        core::ptr::null_mut(),
    );

    snprintf(event.as_mut_ptr(), event.len(), c"error_%s".as_ptr(), monitor_name);
    retval = tracefs_event_enable((*inst).inst, c"rv".as_ptr(), event.as_ptr());
    if retval != 0 {
        return -1;
    }

    tep_register_event_handler(
        (*inst).tep,
        -1,
        c"rv".as_ptr(),
        event.as_ptr(),
        ikm_error_handler,
        core::ptr::null_mut(),
    );

    /* set if at least 1 monitor has id in case of a container */
    config_has_id = ikm_has_id(monitor_name);
    if config_has_id < 0 {
        return -1;
    }

    0
}

unsafe fn ikm_enable_trace_container(
    monitor_name: *mut c_char,
    inst: *mut trace_instance,
) -> c_int {
    let mut dp: *mut DIR;
    let abs_path: *mut c_char;
    let mut rv_path = [0 as c_char; MAX_PATH];
    let mut ep: *mut dirent;
    let mut retval: c_int = 0;

    snprintf(
        rv_path.as_mut_ptr(),
        MAX_PATH,
        c"rv/monitors/%s".as_ptr(),
        monitor_name,
    );
    abs_path = tracefs_instance_get_file(core::ptr::null_mut(), rv_path.as_ptr());
    if abs_path.is_null() {
        return -1;
    }
    dp = opendir(abs_path);
    if dp.is_null() {
        free(abs_path as *mut c_void);
        return retval;
    }

    loop {
        if retval != 0 {
            break;
        }
        ep = readdir(dp);
        if ep.is_null() {
            break;
        }
        if (*ep).d_type != DT_DIR || (*ep).d_name[0] == '.' as c_char {
            continue;
        }
        retval = ikm_enable_trace_events((*ep).d_name.as_mut_ptr(), inst);
    }

    closedir(dp);
    free(abs_path as *mut c_void);
    retval
}

/*
 * ikm_setup_trace_instance - set up a tracing instance to collect data
 *
 * Create a trace instance, enable rv: events and enable the trace.
 *
 * Returns the trace_instance * with all set, NULL otherwise.
 */
unsafe fn ikm_setup_trace_instance(monitor_name: *mut c_char) -> *mut trace_instance {
    let inst: *mut trace_instance;
    let mut retval: c_int;

    if config_trace == 0 {
        return core::ptr::null_mut();
    }

    /* alloc data */
    inst = calloc(1, core::mem::size_of::<trace_instance>()) as *mut trace_instance;
    if inst.is_null() {
        err_msg(c"ikm: failed to allocate trace instance".as_ptr());
        return core::ptr::null_mut();
    }

    retval = trace_instance_init(inst, monitor_name);
    if retval != 0 {
        free(inst as *mut c_void);
        return core::ptr::null_mut();
    }

    if config_is_container != 0 {
        retval = ikm_enable_trace_container(monitor_name, inst);
    } else {
        retval = ikm_enable_trace_events(monitor_name, inst);
    }
    if retval != 0 {
        trace_instance_destroy(inst);
        free(inst as *mut c_void);
        return core::ptr::null_mut();
    }

    /* ready to enable */
    tracefs_trace_on((*inst).inst);

    inst
}

/**
 * ikm_destroy_trace_instance - destroy a previously created instance
 */
unsafe fn ikm_destroy_trace_instance(inst: *mut trace_instance) {
    if inst.is_null() {
        return;
    }

    trace_instance_destroy(inst);
    free(inst as *mut c_void);
}

/*
 * ikm_usage_print_reactors - print all available reactors, one per line.
 */
unsafe fn ikm_usage_print_reactors() {
    let reactors =
        tracefs_instance_file_read(core::ptr::null_mut(), c"rv/available_reactors".as_ptr(), core::ptr::null_mut());
    let mut start: *mut c_char;
    let mut end: *mut c_char;

    if reactors.is_null() {
        return;
    }

    fprintf(stderr, c"  available reactors:".as_ptr());

    start = reactors;
    end = strstr(start, c"\n".as_ptr());

    while !end.is_null() {
        *end = '\0' as c_char;

        fprintf(stderr, c" %s".as_ptr(), start);

        end = end.add(1);
        start = end;
        end = strstr(start, c"\n".as_ptr());
    }

    fprintf(stderr, c"\n".as_ptr());
}

/*
 * ikm_usage - print usage
 */
// Original C function is variadic:
// static void ikm_usage(int exit_val, char *monitor_name, const char *fmt, ...)
unsafe fn ikm_usage(exit_val: c_int, monitor_name: *mut c_char, message: *const c_char) -> ! {
    static USAGE: [*const c_char; 8] = [
        c"".as_ptr(),
        c"\t-h/--help: print this menu and the reactor list".as_ptr(),
        c"\t-r/--reactor 'reactor': enables the 'reactor'".as_ptr(),
        c"\t-s/--self: when tracing (-t), also trace rv command".as_ptr(),
        c"\t-t/--trace: trace monitor's event".as_ptr(),
        c"\t-v/--verbose: print debug messages".as_ptr(),
        c"".as_ptr(),
        core::ptr::null(),
    ];
    let mut i: c_int;

    fprintf(stderr, c"  %s\n".as_ptr(), message);

    fprintf(
        stderr,
        c"\n  usage: rv mon %s [-h] [-q] [-r reactor] [-s] [-v]".as_ptr(),
        monitor_name,
    );

    i = 0;
    while !USAGE[i as usize].is_null() {
        fprintf(stderr, c"%s\n".as_ptr(), USAGE[i as usize]);
        i += 1;
    }

    ikm_usage_print_reactors();
    exit(exit_val);
}

/*
 * parse_arguments - parse arguments and set config
 */
unsafe fn parse_arguments(monitor_name: *mut c_char, argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut c: c_int;
    let retval: c_int;

    config_my_pid = getpid();

    loop {
        static mut LONG_OPTIONS: [option; 6] = [
            option { name: c"help".as_ptr(), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'h' as c_int },
            option { name: c"reactor".as_ptr(), has_arg: required_argument, flag: core::ptr::null_mut(), val: 'r' as c_int },
            option { name: c"self".as_ptr(), has_arg: no_argument, flag: core::ptr::null_mut(), val: 's' as c_int },
            option { name: c"trace".as_ptr(), has_arg: no_argument, flag: core::ptr::null_mut(), val: 't' as c_int },
            option { name: c"verbose".as_ptr(), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'v' as c_int },
            option { name: core::ptr::null(), has_arg: 0, flag: core::ptr::null_mut(), val: 0 },
        ];

        /* getopt_long stores the option index here. */
        let mut option_index: c_int = 0;

        c = getopt_long(argc, argv, c"hr:stv".as_ptr(), LONG_OPTIONS.as_ptr(), &mut option_index);

        /* detect the end of the options. */
        if c == -1 {
            break;
        }

        match c {
            x if x == 'h' as c_int => {
                ikm_usage(0, monitor_name, c"help:".as_ptr());
            }
            x if x == 'r' as c_int => {
                config_reactor = optarg;
            }
            x if x == 's' as c_int => {
                config_my_pid = -1;
            }
            x if x == 't' as c_int => {
                config_trace = 1;
            }
            x if x == 'v' as c_int => {
                config_debug = 1;
            }
            _ => {}
        }
    }

    if !config_reactor.is_null() {
        config_initial_reactor = ikm_get_current_reactor(monitor_name);
        if config_initial_reactor.is_null() {
            ikm_usage(
                1,
                monitor_name,
                c"ikm: failed to read current reactor, are reactors enabled?".as_ptr(),
            );
        }

        retval = ikm_write_reactor(monitor_name, config_reactor);
        if retval <= 0 {
            ikm_usage(
                1,
                monitor_name,
                c"ikm: failed to set %s reactor, is it available?".as_ptr(),
            );
        }
    }

    debug_msg(c"ikm: my pid is %d\n".as_ptr(), config_my_pid);

    0
}

/**
 * ikm_run_monitor - apply configs and run the monitor
 *
 * Returns 1 if a monitor was found an executed, 0 if no
 * monitors were found, or -1 on error.
 */
#[no_mangle]
pub unsafe extern "C" fn ikm_run_monitor(
    monitor_name: *mut c_char,
    argc: c_int,
    argv: *mut *mut c_char,
) -> c_int {
    let mut inst: *mut trace_instance = core::ptr::null_mut();
    let mut nested_name: *mut c_char;
    let mut full_name = [0 as c_char; 2 * MAX_DA_NAME_LEN];
    let mut retval: c_int;

    nested_name = strstr(monitor_name, c":".as_ptr());
    if !nested_name.is_null() {
        nested_name = nested_name.add(1);
    } else {
        nested_name = monitor_name;
    }

    retval = __ikm_find_monitor_name(monitor_name, full_name.as_mut_ptr());
    if retval == 0 {
        return 0;
    }
    if retval < 0 {
        err_msg(c"ikm: error finding monitor %s\n".as_ptr(), nested_name);
        return -1;
    }

    retval = __ikm_read_enable(full_name.as_mut_ptr());
    if retval != 0 {
        err_msg(
            c"ikm: monitor %s (in-kernel) is already enabled\n".as_ptr(),
            nested_name,
        );
        return -1;
    }

    /* we should be good to go */
    retval = parse_arguments(full_name.as_mut_ptr(), argc, argv);
    if retval != 0 {
        ikm_usage(1, nested_name, c"ikm: failed parsing arguments".as_ptr());
    }

    if config_trace != 0 {
        inst = ikm_setup_trace_instance(nested_name);
        if inst.is_null() {
            ikm_destroy_trace_instance(inst);
            if !config_reactor.is_null() && !config_initial_reactor.is_null() {
                ikm_write_reactor(full_name.as_mut_ptr(), config_initial_reactor);
            }
            return -1;
        }
    }

    retval = ikm_enable(full_name.as_mut_ptr());
    if retval < 0 {
        ikm_destroy_trace_instance(inst);
        if !config_reactor.is_null() && !config_initial_reactor.is_null() {
            ikm_write_reactor(full_name.as_mut_ptr(), config_initial_reactor);
        }
        return -1;
    }

    if config_trace != 0 {
        ikm_print_header((*inst).seq);
    }

    while !should_stop() {
        if config_trace != 0 {
            retval = tracefs_iterate_raw_events(
                (*inst).tep,
                (*inst).inst,
                core::ptr::null_mut(),
                0,
                collect_registered_events,
                inst as *mut c_void,
            );
            if retval != 0 {
                err_msg(c"ikm: error reading trace buffer\n".as_ptr());
                break;
            }
        }

        sleep(1);
    }

    ikm_disable(full_name.as_mut_ptr());
    ikm_destroy_trace_instance(inst);

    if !config_reactor.is_null() && !config_initial_reactor.is_null() {
        ikm_write_reactor(full_name.as_mut_ptr(), config_initial_reactor);
    }

    1
}
