// SPDX-License-Identifier: GPL-2.0-only
/*
 * Thermal monitoring tool based on the thermal netlink events.
 *
 * Copyright (C) 2022 Linaro Ltd.
 *
 * Author: Daniel Lezcano <daniel.lezcano@kernel.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

type size_t = usize;

const no_argument: c_int = 0;
const required_argument: c_int = 1;

const LOG_INFO: c_int = 6;
const TO_STDOUT: c_int = 0;
const TO_SYSLOG: c_int = 1;

const THERMAL_THRESHOLD_WAY_UP: c_int = 1;
const THERMAL_THRESHOLD_WAY_DOWN: c_int = 2;

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
struct options {
    loglevel: c_int,
    logopt: c_int,
    interactive: c_int,
    daemonize: c_int,
}

/*
 * Layouts and declarations normally supplied by <thermal.h> and
 * "thermal-tools.h". Fields below are the fields this implementation source
 * directly dereferences.
 */
#[repr(C)]
struct thermal_threshold {
    temperature: c_int,
    direction: c_int,
}

#[repr(C)]
struct thermal_trip {
    id: c_int,
    type_: c_int,
    temp: c_int,
    hyst: c_int,
}

#[repr(C)]
struct thermal_zone {
    id: c_int,
    name: *mut c_char,
    trip: *mut thermal_trip,
    thresholds: *mut thermal_threshold,
    temp: c_int,
    governor: *mut c_char,
}

#[repr(C)]
struct thermal_handler {
    _private: [u8; 0],
}

#[repr(C)]
struct thermal_data {
    tz: *mut thermal_zone,
    th: *mut thermal_handler,
}

#[repr(C)]
struct thermal_events_ops {
    tz_create: Option<unsafe extern "C" fn(*const c_char, c_int, *mut c_void) -> c_int>,
    tz_delete: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
    tz_disable: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
    tz_enable: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
    trip_high: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_void) -> c_int>,
    trip_low: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_void) -> c_int>,
    trip_add: Option<unsafe extern "C" fn(c_int, c_int, c_int, c_int, c_int, *mut c_void) -> c_int>,
    trip_delete: Option<unsafe extern "C" fn(c_int, c_int, *mut c_void) -> c_int>,
    trip_change: Option<unsafe extern "C" fn(c_int, c_int, c_int, c_int, c_int, *mut c_void) -> c_int>,
    cdev_add: Option<unsafe extern "C" fn(*const c_char, c_int, c_int, *mut c_void) -> c_int>,
    cdev_delete: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
    cdev_update: Option<unsafe extern "C" fn(c_int, c_int, *mut c_void) -> c_int>,
    gov_change: Option<unsafe extern "C" fn(c_int, *const c_char, *mut c_void) -> c_int>,
    threshold_add: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_void) -> c_int>,
    threshold_delete: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_void) -> c_int>,
    threshold_flush: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
    threshold_up: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_void) -> c_int>,
    threshold_down: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_void) -> c_int>,
}

#[repr(C)]
struct thermal_ops {
    events: thermal_events_ops,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn daemon(nochdir: c_int, noclose: c_int) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;

    fn INFO(fmt: *const c_char, ...);
    fn ERROR(fmt: *const c_char, ...);

    fn thermal_cmd_get_temp(th: *mut c_void, tz: *mut thermal_zone) -> c_int;
    fn thermal_cmd_get_governor(th: *mut c_void, tz: *mut thermal_zone) -> c_int;
    fn thermal_cmd_threshold_flush(th: *mut thermal_handler, tz: *mut thermal_zone) -> c_int;
    fn thermal_cmd_threshold_add(
        th: *mut thermal_handler,
        tz: *mut thermal_zone,
        temperature: c_int,
        direction: c_int,
    ) -> c_int;
    fn thermal_zone_find_by_id(tz: *mut thermal_zone, id: c_int) -> *mut thermal_zone;
    fn thermal_events_handle(th: *mut thermal_handler, arg: *mut c_void) -> c_int;
    fn thermal_init(ops: *mut thermal_ops) -> *mut thermal_handler;
    fn thermal_zone_discover(th: *mut thermal_handler) -> *mut thermal_zone;
    fn thermal_events_fd(th: *mut thermal_handler) -> c_int;

    fn for_each_thermal_trip(
        trip: *mut thermal_trip,
        cb: unsafe extern "C" fn(*mut thermal_trip, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn for_each_thermal_threshold(
        thresholds: *mut thermal_threshold,
        cb: unsafe extern "C" fn(*mut thermal_threshold, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn for_each_thermal_zone(
        tz: *mut thermal_zone,
        cb: unsafe extern "C" fn(*mut thermal_zone, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;

    fn log_str2level(str_: *mut c_char) -> c_int;
    fn log_init(level: c_int, name: *mut c_char, logopt: c_int) -> c_int;
    fn mainloop_init() -> c_int;
    fn mainloop_add(
        fd: c_int,
        cb: unsafe extern "C" fn(c_int, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn mainloop(timeout: c_int) -> c_int;
}

unsafe extern "C" fn show_threshold(th: *mut thermal_threshold, _arg: *mut c_void) -> c_int {
    unsafe {
        INFO(
            c"threshold temp=%d, direction=%d\n".as_ptr(),
            (*th).temperature,
            (*th).direction,
        );
    }

    0
}

unsafe extern "C" fn show_trip(tt: *mut thermal_trip, _arg: *mut c_void) -> c_int {
    unsafe {
        INFO(
            c"trip id=%d, type=%d, temp=%d, hyst=%d\n".as_ptr(),
            (*tt).id,
            (*tt).type_,
            (*tt).temp,
            (*tt).hyst,
        );
    }

    0
}

unsafe extern "C" fn show_temp(tz: *mut thermal_zone, arg: *mut c_void) -> c_int {
    unsafe {
        thermal_cmd_get_temp(arg, tz);

        INFO(c"temperature: %d\n".as_ptr(), (*tz).temp);
    }

    0
}

unsafe extern "C" fn show_governor(tz: *mut thermal_zone, arg: *mut c_void) -> c_int {
    unsafe {
        thermal_cmd_get_governor(arg, tz);

        INFO(c"governor: '%s'\n".as_ptr(), (*tz).governor);
    }

    0
}

unsafe extern "C" fn show_tz(tz: *mut thermal_zone, arg: *mut c_void) -> c_int {
    unsafe {
        INFO(c"thermal zone '%s', id=%d\n".as_ptr(), (*tz).name, (*tz).id);

        for_each_thermal_trip((*tz).trip, show_trip, ptr::null_mut());

        for_each_thermal_threshold((*tz).thresholds, show_threshold, ptr::null_mut());

        show_temp(tz, arg);

        show_governor(tz, arg);
    }

    0
}

unsafe extern "C" fn set_threshold(tz: *mut thermal_zone, arg: *mut c_void) -> c_int {
    unsafe {
        let th = arg as *mut thermal_handler;
        let thresholds: [c_int; 5] = [43000, 65000, 49000, 55000, 57000];
        let mut i: size_t;

        INFO(
            c"Setting threshold for thermal zone '%s', id=%d\n".as_ptr(),
            (*tz).name,
            (*tz).id,
        );

        if thermal_cmd_threshold_flush(th, tz) != 0 {
            ERROR(c"Failed to flush all previous thresholds\n".as_ptr());
            return -1;
        }

        i = 0;
        while i < thresholds.len() {
            if thermal_cmd_threshold_add(
                th,
                tz,
                thresholds[i],
                THERMAL_THRESHOLD_WAY_UP | THERMAL_THRESHOLD_WAY_DOWN,
            ) != 0
            {
                ERROR(c"Failed to set threshold\n".as_ptr());
                return -1;
            }
            i += 1;
        }
    }

    0
}

unsafe extern "C" fn tz_create(name: *const c_char, tz_id: c_int, _arg: *mut c_void) -> c_int {
    unsafe {
        INFO(c"Thermal zone '%s'/%d created\n".as_ptr(), name, tz_id);
    }

    0
}

unsafe extern "C" fn tz_delete(tz_id: c_int, _arg: *mut c_void) -> c_int {
    unsafe {
        INFO(c"Thermal zone %d deleted\n".as_ptr(), tz_id);
    }

    0
}

unsafe extern "C" fn tz_disable(tz_id: c_int, arg: *mut c_void) -> c_int {
    unsafe {
        let td = arg as *mut thermal_data;
        let tz = thermal_zone_find_by_id((*td).tz, tz_id);

        INFO(c"Thermal zone %d ('%s') disabled\n".as_ptr(), tz_id, (*tz).name);
    }

    0
}

unsafe extern "C" fn tz_enable(tz_id: c_int, arg: *mut c_void) -> c_int {
    unsafe {
        let td = arg as *mut thermal_data;
        let tz = thermal_zone_find_by_id((*td).tz, tz_id);

        INFO(c"Thermal zone %d ('%s') enabled\n".as_ptr(), tz_id, (*tz).name);
    }

    0
}

unsafe extern "C" fn trip_high(
    tz_id: c_int,
    trip_id: c_int,
    temp: c_int,
    arg: *mut c_void,
) -> c_int {
    unsafe {
        let td = arg as *mut thermal_data;
        let tz = thermal_zone_find_by_id((*td).tz, tz_id);

        INFO(
            c"Thermal zone %d ('%s'): trip point %d crossed way up with %d °C\n".as_ptr(),
            tz_id,
            (*tz).name,
            trip_id,
            temp,
        );
    }

    0
}

unsafe extern "C" fn trip_low(
    tz_id: c_int,
    trip_id: c_int,
    temp: c_int,
    arg: *mut c_void,
) -> c_int {
    unsafe {
        let td = arg as *mut thermal_data;
        let tz = thermal_zone_find_by_id((*td).tz, tz_id);

        INFO(
            c"Thermal zone %d ('%s'): trip point %d crossed way down with %d °C\n".as_ptr(),
            tz_id,
            (*tz).name,
            trip_id,
            temp,
        );
    }

    0
}

unsafe extern "C" fn trip_add(
    tz_id: c_int,
    trip_id: c_int,
    type_: c_int,
    temp: c_int,
    hyst: c_int,
    _arg: *mut c_void,
) -> c_int {
    unsafe {
        INFO(
            c"Trip point added %d: id=%d, type=%d, temp=%d, hyst=%d\n".as_ptr(),
            tz_id,
            trip_id,
            type_,
            temp,
            hyst,
        );
    }

    0
}

unsafe extern "C" fn trip_delete(tz_id: c_int, trip_id: c_int, _arg: *mut c_void) -> c_int {
    unsafe {
        INFO(c"Trip point deleted %d: id=%d\n".as_ptr(), tz_id, trip_id);
    }

    0
}

unsafe extern "C" fn trip_change(
    tz_id: c_int,
    trip_id: c_int,
    type_: c_int,
    temp: c_int,
    hyst: c_int,
    arg: *mut c_void,
) -> c_int {
    unsafe {
        let td = arg as *mut thermal_data;
        let tz = thermal_zone_find_by_id((*td).tz, tz_id);

        INFO(
            c"Trip point changed %d: id=%d, type=%d, temp=%d, hyst=%d\n".as_ptr(),
            tz_id,
            trip_id,
            type_,
            temp,
            hyst,
        );

        (*(*tz).trip.add(trip_id as usize)).type_ = type_;
        (*(*tz).trip.add(trip_id as usize)).temp = temp;
        (*(*tz).trip.add(trip_id as usize)).hyst = hyst;
    }

    0
}

unsafe extern "C" fn cdev_add(
    name: *const c_char,
    cdev_id: c_int,
    max_state: c_int,
    _arg: *mut c_void,
) -> c_int {
    unsafe {
        INFO(
            c"Cooling device '%s'/%d (max state=%d) added\n".as_ptr(),
            name,
            cdev_id,
            max_state,
        );
    }

    0
}

unsafe extern "C" fn cdev_delete(cdev_id: c_int, _arg: *mut c_void) -> c_int {
    unsafe {
        INFO(c"Cooling device %d deleted".as_ptr(), cdev_id);
    }

    0
}

unsafe extern "C" fn cdev_update(cdev_id: c_int, cur_state: c_int, _arg: *mut c_void) -> c_int {
    unsafe {
        INFO(c"cdev:%d state:%d\n".as_ptr(), cdev_id, cur_state);
    }

    0
}

unsafe extern "C" fn gov_change(tz_id: c_int, name: *const c_char, arg: *mut c_void) -> c_int {
    unsafe {
        let td = arg as *mut thermal_data;
        let tz = thermal_zone_find_by_id((*td).tz, tz_id);

        INFO(
            c"%s: governor changed %s -> %s\n".as_ptr(),
            (*tz).name,
            (*tz).governor,
            name,
        );

        strcpy((*tz).governor, name);
    }

    0
}

unsafe extern "C" fn threshold_add(
    tz_id: c_int,
    temp: c_int,
    direction: c_int,
    _arg: *mut c_void,
) -> c_int {
    unsafe {
        INFO(
            c"Threshold added tz_id=%d: temp=%d, direction=%d\n".as_ptr(),
            tz_id,
            temp,
            direction,
        );
    }

    0
}

unsafe extern "C" fn threshold_delete(
    tz_id: c_int,
    temp: c_int,
    direction: c_int,
    _arg: *mut c_void,
) -> c_int {
    unsafe {
        INFO(
            c"Threshold deleted tz_id=%d: temp=%d, direction=%d\n".as_ptr(),
            tz_id,
            temp,
            direction,
        );
    }

    0
}

unsafe extern "C" fn threshold_flush(tz_id: c_int, _arg: *mut c_void) -> c_int {
    unsafe {
        INFO(c"Thresholds flushed tz_id=%d\n".as_ptr(), tz_id);
    }

    0
}

unsafe extern "C" fn threshold_up(
    tz_id: c_int,
    temp: c_int,
    prev_temp: c_int,
    _arg: *mut c_void,
) -> c_int {
    unsafe {
        INFO(
            c"Threshold crossed way up tz_id=%d: temp=%d, prev_temp=%d\n".as_ptr(),
            tz_id,
            temp,
            prev_temp,
        );
    }

    0
}

unsafe extern "C" fn threshold_down(
    tz_id: c_int,
    temp: c_int,
    prev_temp: c_int,
    _arg: *mut c_void,
) -> c_int {
    unsafe {
        INFO(
            c"Threshold crossed way down tz_id=%d: temp=%d, prev_temp=%d\n".as_ptr(),
            tz_id,
            temp,
            prev_temp,
        );
    }

    0
}

static mut ops: thermal_ops = thermal_ops {
    events: thermal_events_ops {
        tz_create: Some(tz_create),
        tz_delete: Some(tz_delete),
        tz_disable: Some(tz_disable),
        tz_enable: Some(tz_enable),
        trip_high: Some(trip_high),
        trip_low: Some(trip_low),
        trip_add: Some(trip_add),
        trip_delete: Some(trip_delete),
        trip_change: Some(trip_change),
        cdev_add: Some(cdev_add),
        cdev_delete: Some(cdev_delete),
        cdev_update: Some(cdev_update),
        gov_change: Some(gov_change),
        threshold_add: Some(threshold_add),
        threshold_delete: Some(threshold_delete),
        threshold_flush: Some(threshold_flush),
        threshold_up: Some(threshold_up),
        threshold_down: Some(threshold_down),
    },
};

unsafe extern "C" fn thermal_event(_fd: c_int, arg: *mut c_void) -> c_int {
    unsafe {
        let td = arg as *mut thermal_data;

        thermal_events_handle((*td).th, td as *mut c_void)
    }
}

unsafe extern "C" fn usage(cmd: *const c_char) {
    unsafe {
        printf(
            c"%s : A thermal monitoring engine based on notifications\n".as_ptr(),
            cmd,
        );
        printf(c"Usage: %s [options]\n".as_ptr(), cmd);
        printf(c"\t-h, --help\t\tthis help\n".as_ptr());
        printf(c"\t-d, --daemonize\n".as_ptr());
        printf(c"\t-l <level>, --loglevel <level>\tlog level: ".as_ptr());
        printf(c"DEBUG, INFO, NOTICE, WARN, ERROR\n".as_ptr());
        printf(c"\t-s, --syslog\t\toutput to syslog\n".as_ptr());
        printf(c"\n".as_ptr());
        exit(0);
    }
}

unsafe extern "C" fn options_init(
    argc: c_int,
    argv: *mut *mut c_char,
    options: *mut options,
) -> c_int {
    unsafe {
        let mut opt: c_int;

        let long_options: [option; 5] = [
            option {
                name: c"help".as_ptr(),
                has_arg: no_argument,
                flag: ptr::null_mut(),
                val: 'h' as c_int,
            },
            option {
                name: c"daemonize".as_ptr(),
                has_arg: no_argument,
                flag: ptr::null_mut(),
                val: 'd' as c_int,
            },
            option {
                name: c"syslog".as_ptr(),
                has_arg: no_argument,
                flag: ptr::null_mut(),
                val: 's' as c_int,
            },
            option {
                name: c"loglevel".as_ptr(),
                has_arg: required_argument,
                flag: ptr::null_mut(),
                val: 'l' as c_int,
            },
            option {
                name: ptr::null(),
                has_arg: 0,
                flag: ptr::null_mut(),
                val: 0,
            },
        ];

        loop {
            let mut optindex: c_int = 0;

            opt = getopt_long(
                argc,
                argv,
                c"l:dhs".as_ptr(),
                long_options.as_ptr(),
                &mut optindex,
            );
            if opt == -1 {
                break;
            }

            match opt {
                x if x == 'l' as c_int => {
                    (*options).loglevel = log_str2level(optarg);
                }
                x if x == 'd' as c_int => {
                    (*options).daemonize = 1;
                }
                x if x == 's' as c_int => {
                    (*options).logopt = TO_SYSLOG;
                }
                x if x == 'h' as c_int => {
                    usage(basename(*argv.add(0)));
                }
                _ => {
                    /* '?' */
                    return -1;
                }
            }
        }
    }

    0
}

const THERMAL_ENGINE_SUCCESS: c_int = 0;
const THERMAL_ENGINE_OPTION_ERROR: c_int = 1;
const THERMAL_ENGINE_DAEMON_ERROR: c_int = 2;
const THERMAL_ENGINE_LOG_ERROR: c_int = 3;
const THERMAL_ENGINE_THERMAL_ERROR: c_int = 4;
const THERMAL_ENGINE_THRESHOLD_ERROR: c_int = 5;
const THERMAL_ENGINE_MAINLOOP_ERROR: c_int = 6;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        let mut td = thermal_data {
            tz: ptr::null_mut(),
            th: ptr::null_mut(),
        };
        let mut options = options {
            loglevel: LOG_INFO,
            logopt: TO_STDOUT,
            interactive: 0,
            daemonize: 0,
        };

        if options_init(argc, argv, &mut options) != 0 {
            ERROR(c"Usage: %s --help\n".as_ptr(), *argv.add(0));
            return THERMAL_ENGINE_OPTION_ERROR;
        }

        if options.daemonize != 0 && daemon(0, 0) != 0 {
            ERROR(c"Failed to daemonize: %m\n".as_ptr());
            return THERMAL_ENGINE_DAEMON_ERROR;
        }

        if log_init(options.loglevel, basename(*argv.add(0)), options.logopt) != 0 {
            ERROR(c"Failed to initialize logging facility\n".as_ptr());
            return THERMAL_ENGINE_LOG_ERROR;
        }

        td.th = thermal_init(&raw mut ops);
        if td.th.is_null() {
            ERROR(c"Failed to initialize the thermal library\n".as_ptr());
            return THERMAL_ENGINE_THERMAL_ERROR;
        }

        td.tz = thermal_zone_discover(td.th);
        if td.tz.is_null() {
            ERROR(c"No thermal zone available\n".as_ptr());
            return THERMAL_ENGINE_THERMAL_ERROR;
        }

        for_each_thermal_zone(td.tz, set_threshold, td.th as *mut c_void);

        for_each_thermal_zone(td.tz, show_tz, td.th as *mut c_void);

        if mainloop_init() != 0 {
            ERROR(c"Failed to initialize the mainloop\n".as_ptr());
            return THERMAL_ENGINE_MAINLOOP_ERROR;
        }

        if mainloop_add(thermal_events_fd(td.th), thermal_event, &mut td as *mut _ as *mut c_void)
            != 0
        {
            ERROR(c"Failed to setup the mainloop\n".as_ptr());
            return THERMAL_ENGINE_MAINLOOP_ERROR;
        }

        INFO(c"Waiting for thermal events ...\n".as_ptr());

        if mainloop(-1) != 0 {
            ERROR(c"Mainloop failed\n".as_ptr());
            return THERMAL_ENGINE_MAINLOOP_ERROR;
        }

        THERMAL_ENGINE_SUCCESS
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
