// SPDX-License-Identifier: GPL-2.0-only
/*
 * Counter Watch Events - Test various counter watch events in a userspace application
 *
 * Copyright (C) STMicroelectronics 2023 - All Rights Reserved
 * Author: Fabrice Gasnier <fabrice.gasnier@foss.st.com>.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type c_ulonglong = u64;
type c_void = core::ffi::c_void;
type size_t = usize;
type ssize_t = isize;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const O_RDWR: c_int = 2;

const no_argument: c_int = 0;
const required_argument: c_int = 1;

const COUNTER_COMPONENT_NONE: c_int = 0;
const COUNTER_COMPONENT_SIGNAL: c_int = 1;
const COUNTER_COMPONENT_COUNT: c_int = 2;
const COUNTER_COMPONENT_FUNCTION: c_int = 3;
const COUNTER_COMPONENT_SYNAPSE_ACTION: c_int = 4;
const COUNTER_COMPONENT_EXTENSION: c_int = 5;

const COUNTER_SCOPE_DEVICE: c_int = 0;
const COUNTER_SCOPE_SIGNAL: c_int = 1;
const COUNTER_SCOPE_COUNT: c_int = 2;

const COUNTER_EVENT_OVERFLOW: c_int = 0;
const COUNTER_EVENT_UNDERFLOW: c_int = 1;
const COUNTER_EVENT_OVERFLOW_UNDERFLOW: c_int = 2;
const COUNTER_EVENT_THRESHOLD: c_int = 3;
const COUNTER_EVENT_INDEX: c_int = 4;
const COUNTER_EVENT_CHANGE_OF_STATE: c_int = 5;
const COUNTER_EVENT_CAPTURE: c_int = 6;
const COUNTER_EVENT_DIRECTION_CHANGE: c_int = 7;

// ioctl numbers are provided by linux/counter.h in the original C source.
unsafe extern "C" {
    static COUNTER_ADD_WATCH_IOCTL: c_ulong;
    static COUNTER_ENABLE_EVENTS_IOCTL: c_ulong;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_component {
    pub type_: c_uint,
    pub scope: c_uint,
    pub parent: c_uint,
    pub id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_watch {
    pub component: counter_component,
    pub event: c_uint,
    pub channel: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_event {
    pub timestamp: c_ulonglong,
    pub value: c_ulonglong,
    pub watch: counter_watch,
    pub status: c_int,
}

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    static mut optind: c_int;

    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        shortopts: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn getsubopt(
        optionp: *mut *mut c_char,
        tokens: *const *mut c_char,
        valuep: *mut *mut c_char,
    ) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn strerror(errnum: c_int) -> *mut c_char;

    static mut stderr: *mut FILE;
}

type c_long = i64;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

static mut simple_watch: [counter_watch; 1] = [counter_watch {
    component: counter_component {
        /* Component data: Count 0 count */
        type_: COUNTER_COMPONENT_COUNT as c_uint,
        scope: COUNTER_SCOPE_COUNT as c_uint,
        parent: 0,
        id: 0,
    },
    /* Event type: overflow or underflow */
    event: COUNTER_EVENT_OVERFLOW_UNDERFLOW as c_uint,
    /* Device event channel 0 */
    channel: 0,
}];

static mut counter_event_type_name: [*const c_char; 8] = [
    c"COUNTER_EVENT_OVERFLOW".as_ptr(),
    c"COUNTER_EVENT_UNDERFLOW".as_ptr(),
    c"COUNTER_EVENT_OVERFLOW_UNDERFLOW".as_ptr(),
    c"COUNTER_EVENT_THRESHOLD".as_ptr(),
    c"COUNTER_EVENT_INDEX".as_ptr(),
    c"COUNTER_EVENT_CHANGE_OF_STATE".as_ptr(),
    c"COUNTER_EVENT_CAPTURE".as_ptr(),
    c"COUNTER_EVENT_DIRECTION_CHANGE".as_ptr(),
];

static mut counter_component_type_name: [*const c_char; 6] = [
    c"COUNTER_COMPONENT_NONE".as_ptr(),
    c"COUNTER_COMPONENT_SIGNAL".as_ptr(),
    c"COUNTER_COMPONENT_COUNT".as_ptr(),
    c"COUNTER_COMPONENT_FUNCTION".as_ptr(),
    c"COUNTER_COMPONENT_SYNAPSE_ACTION".as_ptr(),
    c"COUNTER_COMPONENT_EXTENSION".as_ptr(),
];

static mut counter_scope_name: [*const c_char; 3] = [
    c"COUNTER_SCOPE_DEVICE".as_ptr(),
    c"COUNTER_SCOPE_SIGNAL".as_ptr(),
    c"COUNTER_SCOPE_COUNT".as_ptr(),
];

unsafe fn print_watch(watch: *mut counter_watch, nwatch: c_int) {
    let mut i: c_int;

    /* prints the watch array in C-like structure */
    printf(c"watch[%d] = {\n".as_ptr(), nwatch);
    i = 0;
    while i < nwatch {
        printf(
            c" [%d] =\t{\n\t\t.component.type = %s\n\t\t.component.scope = %s\n\t\t.component.parent = %d\n\t\t.component.id = %d\n\t\t.event = %s\n\t\t.channel = %d\n\t},\n".as_ptr(),
            i,
            counter_component_type_name[(*watch.add(i as usize)).component.type_ as usize],
            counter_scope_name[(*watch.add(i as usize)).component.scope as usize],
            (*watch.add(i as usize)).component.parent,
            (*watch.add(i as usize)).component.id,
            counter_event_type_name[(*watch.add(i as usize)).event as usize],
            (*watch.add(i as usize)).channel,
        );
        i += 1;
    }
    printf(c"};\n".as_ptr());
}

unsafe fn print_usage() {
    fprintf(
        stderr,
        c"Usage:\n\ncounter_watch_events [options] [-w <watchoptions>]\ncounter_watch_events [options] [-w <watch1 options>] [-w <watch2 options>]...\n\nWhen no --watch option has been provided, simple watch example is used:\ncounter_watch_events [options] -w comp_count,scope_count,evt_ovf_udf\n\nTest various watch events for given counter device.\n\nOptions:\n  -d, --debug                Prints debug information\n  -h, --help                 Prints usage\n  -n, --device-num <n>       Use /dev/counter<n> [default: /dev/counter0]\n  -l, --loop <n>             Loop for <n> events [default: 0 (forever)]\n  -w, --watch <watchoptions> comma-separated list of watch options\n\nWatch options:\n  scope_device               (COUNTER_SCOPE_DEVICE) [default: scope_device]\n  scope_signal               (COUNTER_SCOPE_SIGNAL)\n  scope_count                (COUNTER_SCOPE_COUNT)\n\n  comp_none                  (COUNTER_COMPONENT_NONE) [default: comp_none]\n  comp_signal                (COUNTER_COMPONENT_SIGNAL)\n  comp_count                 (COUNTER_COMPONENT_COUNT)\n  comp_function              (COUNTER_COMPONENT_FUNCTION)\n  comp_synapse_action        (COUNTER_COMPONENT_SYNAPSE_ACTION)\n  comp_extension             (COUNTER_COMPONENT_EXTENSION)\n\n  evt_ovf                    (COUNTER_EVENT_OVERFLOW) [default: evt_ovf]\n  evt_udf                    (COUNTER_EVENT_UNDERFLOW)\n  evt_ovf_udf                (COUNTER_EVENT_OVERFLOW_UNDERFLOW)\n  evt_threshold              (COUNTER_EVENT_THRESHOLD)\n  evt_index                  (COUNTER_EVENT_INDEX)\n  evt_change_of_state        (COUNTER_EVENT_CHANGE_OF_STATE)\n  evt_capture                (COUNTER_EVENT_CAPTURE)\n  evt_direction_change       (COUNTER_EVENT_DIRECTION_CHANGE)\n\n  chan=<n>                   channel <n> for this watch [default: 0]\n  id=<n>                     component id <n> for this watch [default: 0]\n  parent=<n>                 component parent <n> for this watch [default: 0]\n\nExample with two watched events:\n\ncounter_watch_events -d \\\n\t-w comp_count,scope_count,evt_ovf_udf \\\n\t-w comp_extension,scope_count,evt_capture,id=7,chan=3\n".as_ptr(),
    );
}

static longopts: [option; 6] = [
    option { name: c"debug".as_ptr(), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'd' as c_int },
    option { name: c"help".as_ptr(), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'h' as c_int },
    option { name: c"device-num".as_ptr(), has_arg: required_argument, flag: core::ptr::null_mut(), val: 'n' as c_int },
    option { name: c"loop".as_ptr(), has_arg: required_argument, flag: core::ptr::null_mut(), val: 'l' as c_int },
    option { name: c"watch".as_ptr(), has_arg: required_argument, flag: core::ptr::null_mut(), val: 'w' as c_int },
    option { name: core::ptr::null(), has_arg: 0, flag: core::ptr::null_mut(), val: 0 },
];

/* counter watch subopts */
const WATCH_SCOPE_DEVICE: c_int = 0;
const WATCH_SCOPE_SIGNAL: c_int = 1;
const WATCH_SCOPE_COUNT: c_int = 2;
const WATCH_COMPONENT_NONE: c_int = 3;
const WATCH_COMPONENT_SIGNAL: c_int = 4;
const WATCH_COMPONENT_COUNT: c_int = 5;
const WATCH_COMPONENT_FUNCTION: c_int = 6;
const WATCH_COMPONENT_SYNAPSE_ACTION: c_int = 7;
const WATCH_COMPONENT_EXTENSION: c_int = 8;
const WATCH_EVENT_OVERFLOW: c_int = 9;
const WATCH_EVENT_UNDERFLOW: c_int = 10;
const WATCH_EVENT_OVERFLOW_UNDERFLOW: c_int = 11;
const WATCH_EVENT_THRESHOLD: c_int = 12;
const WATCH_EVENT_INDEX: c_int = 13;
const WATCH_EVENT_CHANGE_OF_STATE: c_int = 14;
const WATCH_EVENT_CAPTURE: c_int = 15;
const WATCH_EVENT_DIRECTION_CHANGE: c_int = 16;
const WATCH_CHANNEL: c_int = 17;
const WATCH_ID: c_int = 18;
const WATCH_PARENT: c_int = 19;
const WATCH_SUBOPTS_MAX: usize = 20;

static mut counter_watch_subopts: [*mut c_char; WATCH_SUBOPTS_MAX + 1] = [
    /* component.scope */
    c"scope_device".as_ptr() as *mut c_char,
    c"scope_signal".as_ptr() as *mut c_char,
    c"scope_count".as_ptr() as *mut c_char,
    /* component.type */
    c"comp_none".as_ptr() as *mut c_char,
    c"comp_signal".as_ptr() as *mut c_char,
    c"comp_count".as_ptr() as *mut c_char,
    c"comp_function".as_ptr() as *mut c_char,
    c"comp_synapse_action".as_ptr() as *mut c_char,
    c"comp_extension".as_ptr() as *mut c_char,
    /* event */
    c"evt_ovf".as_ptr() as *mut c_char,
    c"evt_udf".as_ptr() as *mut c_char,
    c"evt_ovf_udf".as_ptr() as *mut c_char,
    c"evt_threshold".as_ptr() as *mut c_char,
    c"evt_index".as_ptr() as *mut c_char,
    c"evt_change_of_state".as_ptr() as *mut c_char,
    c"evt_capture".as_ptr() as *mut c_char,
    c"evt_direction_change".as_ptr() as *mut c_char,
    /* channel, id, parent */
    c"chan".as_ptr() as *mut c_char,
    c"id".as_ptr() as *mut c_char,
    c"parent".as_ptr() as *mut c_char,
    /* Empty entry ends the opts array */
    core::ptr::null_mut(),
];

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut c: c_int;
    let mut fd: c_int = 0;
    let mut i: c_int;
    let mut ret: c_int;
    let mut rc: c_int = 0;
    let mut debug: c_int = 0;
    let mut loop_: c_int = 0;
    let mut dev_num: c_int = 0;
    let mut nwatch: c_int = 0;
    let mut event_data: counter_event = core::mem::zeroed();
    let mut device_name: *mut c_char = core::ptr::null_mut();
    let mut subopts: *mut c_char;
    let mut value: *mut c_char = core::ptr::null_mut();
    let mut watches: *mut counter_watch;

    /*
     * 1st pass:
     * - list watch events number to allocate the watch array.
     * - parse normal options (other than watch options)
     */
    loop {
        c = getopt_long(argc, argv, c"dhn:l:w:".as_ptr(), longopts.as_ptr(), core::ptr::null_mut());
        if c == -1 {
            break;
        }
        match c {
            x if x == 'd' as c_int => {
                debug = 1;
            }
            x if x == 'h' as c_int => {
                print_usage();
                return EXIT_SUCCESS;
            }
            x if x == 'n' as c_int => {
                dev_num = strtoul(optarg, core::ptr::null_mut(), 10) as c_int;
                if errno != 0 {
                    perror(c"strtol failed: --device-num <n>\n".as_ptr());
                    return EXIT_FAILURE;
                }
            }
            x if x == 'l' as c_int => {
                loop_ = strtol(optarg, core::ptr::null_mut(), 10) as c_int;
                if errno != 0 {
                    perror(c"strtol failed: --loop <n>\n".as_ptr());
                    return EXIT_FAILURE;
                }
            }
            x if x == 'w' as c_int => {
                nwatch += 1;
            }
            _ => {
                return EXIT_FAILURE;
            }
        }
    }

    if nwatch != 0 {
        watches = calloc(nwatch as size_t, core::mem::size_of::<counter_watch>()) as *mut counter_watch;
        if watches.is_null() {
            perror(c"Error allocating watches\n".as_ptr());
            return EXIT_FAILURE;
        }
    } else {
        /* default to simple watch example */
        watches = simple_watch.as_mut_ptr();
        nwatch = simple_watch.len() as c_int;
    }

    /* 2nd pass: parse watch sub-options to fill in watch array */
    optind = 1;
    i = 0;
    loop {
        c = getopt_long(argc, argv, c"dhn:l:w:".as_ptr(), longopts.as_ptr(), core::ptr::null_mut());
        if c == -1 {
            break;
        }
        match c {
            x if x == 'w' as c_int => {
                subopts = optarg;
                while *subopts != 0 {
                    ret = getsubopt(&mut subopts, counter_watch_subopts.as_ptr(), &mut value);
                    match ret {
                        WATCH_SCOPE_DEVICE | WATCH_SCOPE_SIGNAL | WATCH_SCOPE_COUNT => {
                            /* match with counter_scope */
                            (*watches.add(i as usize)).component.scope = ret as c_uint;
                        }
                        WATCH_COMPONENT_NONE
                        | WATCH_COMPONENT_SIGNAL
                        | WATCH_COMPONENT_COUNT
                        | WATCH_COMPONENT_FUNCTION
                        | WATCH_COMPONENT_SYNAPSE_ACTION
                        | WATCH_COMPONENT_EXTENSION => {
                            /* match counter_component_type: subtract enum value */
                            ret -= WATCH_COMPONENT_NONE;
                            (*watches.add(i as usize)).component.type_ = ret as c_uint;
                        }
                        WATCH_EVENT_OVERFLOW
                        | WATCH_EVENT_UNDERFLOW
                        | WATCH_EVENT_OVERFLOW_UNDERFLOW
                        | WATCH_EVENT_THRESHOLD
                        | WATCH_EVENT_INDEX
                        | WATCH_EVENT_CHANGE_OF_STATE
                        | WATCH_EVENT_CAPTURE
                        | WATCH_EVENT_DIRECTION_CHANGE => {
                            /* match counter_event_type: subtract enum value */
                            ret -= WATCH_EVENT_OVERFLOW;
                            (*watches.add(i as usize)).event = ret as c_uint;
                        }
                        WATCH_CHANNEL => {
                            if value.is_null() {
                                fprintf(stderr, c"Invalid chan=<number>\n".as_ptr());
                                rc = EXIT_FAILURE;
                                goto_err_free_watches(watches);
                                return rc;
                            }
                            (*watches.add(i as usize)).channel =
                                strtoul(value, core::ptr::null_mut(), 10) as c_uint;
                            if errno != 0 {
                                perror(c"strtoul failed: chan=<number>\n".as_ptr());
                                rc = EXIT_FAILURE;
                                goto_err_free_watches(watches);
                                return rc;
                            }
                        }
                        WATCH_ID => {
                            if value.is_null() {
                                fprintf(stderr, c"Invalid id=<number>\n".as_ptr());
                                rc = EXIT_FAILURE;
                                goto_err_free_watches(watches);
                                return rc;
                            }
                            (*watches.add(i as usize)).component.id =
                                strtoul(value, core::ptr::null_mut(), 10) as c_uint;
                            if errno != 0 {
                                perror(c"strtoul failed: id=<number>\n".as_ptr());
                                rc = EXIT_FAILURE;
                                goto_err_free_watches(watches);
                                return rc;
                            }
                        }
                        WATCH_PARENT => {
                            if value.is_null() {
                                fprintf(stderr, c"Invalid parent=<number>\n".as_ptr());
                                rc = EXIT_FAILURE;
                                goto_err_free_watches(watches);
                                return rc;
                            }
                            (*watches.add(i as usize)).component.parent =
                                strtoul(value, core::ptr::null_mut(), 10) as c_uint;
                            if errno != 0 {
                                perror(c"strtoul failed: parent=<number>\n".as_ptr());
                                rc = EXIT_FAILURE;
                                goto_err_free_watches(watches);
                                return rc;
                            }
                        }
                        _ => {
                            fprintf(stderr, c"Unknown suboption '%s'\n".as_ptr(), value);
                            rc = EXIT_FAILURE;
                            goto_err_free_watches(watches);
                            return rc;
                        }
                    }
                }
                i += 1;
            }
            _ => {}
        }
    }

    if debug != 0 {
        print_watch(watches, nwatch);
    }

    ret = asprintf(&mut device_name, c"/dev/counter%d".as_ptr(), dev_num);
    if ret < 0 {
        fprintf(stderr, c"asprintf failed\n".as_ptr());
        rc = EXIT_FAILURE;
        goto_err_free_watches(watches);
        return rc;
    }

    if debug != 0 {
        printf(c"Opening %s\n".as_ptr(), device_name);
    }

    fd = open(device_name, O_RDWR);
    if fd == -1 {
        fprintf(stderr, c"Unable to open %s: %s\n".as_ptr(), device_name, strerror(errno));
        free(device_name as *mut c_void);
        rc = EXIT_FAILURE;
        goto_err_free_watches(watches);
        return rc;
    }
    free(device_name as *mut c_void);

    i = 0;
    while i < nwatch {
        ret = ioctl(fd, COUNTER_ADD_WATCH_IOCTL, watches.add(i as usize));
        if ret == -1 {
            fprintf(
                stderr,
                c"Error adding watches[%d]: %s\n".as_ptr(),
                i,
                strerror(errno),
            );
            rc = EXIT_FAILURE;
            close(fd);
            goto_err_free_watches(watches);
            return rc;
        }
        i += 1;
    }

    ret = ioctl(fd, COUNTER_ENABLE_EVENTS_IOCTL);
    if ret == -1 {
        perror(c"Error enabling events".as_ptr());
        rc = EXIT_FAILURE;
        close(fd);
        goto_err_free_watches(watches);
        return rc;
    }

    i = 0;
    while loop_ <= 0 || i < loop_ {
        ret = read(
            fd,
            &mut event_data as *mut counter_event as *mut c_void,
            core::mem::size_of::<counter_event>(),
        ) as c_int;
        if ret == -1 {
            perror(c"Failed to read event data".as_ptr());
            rc = EXIT_FAILURE;
            close(fd);
            goto_err_free_watches(watches);
            return rc;
        }

        if ret as usize != core::mem::size_of::<counter_event>() {
            fprintf(stderr, c"Failed to read event data (got: %d)\n".as_ptr(), ret);
            rc = EXIT_FAILURE;
            close(fd);
            goto_err_free_watches(watches);
            return rc;
        }

        printf(
            c"Timestamp: %llu\tData: %llu\t event: %s\tch: %d\n".as_ptr(),
            event_data.timestamp,
            event_data.value,
            counter_event_type_name[event_data.watch.event as usize],
            event_data.watch.channel,
        );

        if event_data.status != 0 {
            fprintf(
                stderr,
                c"Error %d: %s\n".as_ptr(),
                event_data.status,
                strerror(event_data.status),
            );
        }
        i += 1;
    }

    close(fd);
    goto_err_free_watches(watches);

    rc
}

unsafe fn goto_err_free_watches(watches: *mut counter_watch) {
    if watches != simple_watch.as_mut_ptr() {
        free(watches as *mut c_void);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
