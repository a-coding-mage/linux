// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2018 Netronome Systems, Inc. */
/* This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

use std::ffi::c_void;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong, c_ulonglong};
use std::ptr;

const MMAP_PAGE_CNT: usize = 16;

type __u32 = u32;
type __u64 = u64;

const PERF_SAMPLE_RAW: __u64 = 1 << 10;
const PERF_SAMPLE_TIME: __u64 = 1 << 2;
const PERF_TYPE_SOFTWARE: __u32 = 1;
const PERF_COUNT_SW_BPF_OUTPUT: __u64 = 10;
const PERF_RECORD_SAMPLE: __u32 = 9;
const PERF_RECORD_LOST: __u32 = 2;
const BPF_MAP_TYPE_PERF_EVENT_ARRAY: __u32 = 4;
const LIBBPF_PERF_EVENT_CONT: bpf_perf_event_ret = 0;
const EINTR: c_int = 4;
const SIGINT: c_int = 2;
const SIGHUP: c_int = 1;
const SIGTERM: c_int = 15;

static mut stop: bool = false;

#[repr(C)]
pub struct perf_event_header {
    type_: __u32,
    misc: u16,
    size: u16,
}

#[repr(C)]
pub struct perf_event_attr {
    type_: __u32,
    size: __u32,
    config: __u64,
    sample_period: __u64,
    sample_type: __u64,
    read_format: __u64,
    flags: __u64,
    wakeup_events: __u32,
}

#[repr(C)]
pub struct perf_event_sample {
    header: perf_event_header,
    time: __u64,
    size: __u32,
    data: [c_uchar; 0],
}

#[repr(C)]
pub struct perf_event_lost {
    header: perf_event_header,
    id: __u64,
    lost: __u64,
}

#[repr(C)]
pub struct event_pipe_ctx {
    all_cpus: bool,
    cpu: c_int,
    idx: c_int,
}

#[repr(C)]
pub struct bpf_map_info {
    type_: __u32,
}

#[repr(C)]
pub struct perf_buffer;

#[repr(C)]
pub struct perf_buffer_raw_opts {
    sz: usize,
    cpu_cnt: usize,
    cpus: *mut c_int,
    map_keys: *mut c_int,
}

type bpf_perf_event_ret = c_int;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

extern "C" {
    static mut json_output: bool;
    static mut json_wtr: *mut c_void;
    static mut stdout: *mut c_void;
    static mut stderr: *mut c_void;
    static mut errno: c_int;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    fn close(fd: c_int) -> c_int;

    fn jsonw_start_object(wtr: *mut c_void);
    fn jsonw_name(wtr: *mut c_void, name: *const c_char);
    fn jsonw_uint(wtr: *mut c_void, num: c_ulonglong);
    fn jsonw_end_object(wtr: *mut c_void);
    fn jsonw_start_array(wtr: *mut c_void);
    fn jsonw_end_array(wtr: *mut c_void);

    fn print_data_json(data: *const c_uchar, len: __u32);
    fn fprint_hex(stream: *mut c_void, data: *const c_uchar, len: __u32, sep: *const c_char);
    fn p_err(format: *const c_char, ...);
    fn is_prefix(pfx: *const c_char, str_: *const c_char) -> bool;
    fn map_parse_fd_and_info(
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
        info: *mut bpf_map_info,
        info_len: *mut __u32,
        flags: c_int,
    ) -> c_int;
    fn perf_buffer__new_raw(
        map_fd: c_int,
        page_cnt: usize,
        attr: *mut perf_event_attr,
        event_cb: Option<
            unsafe extern "C" fn(*mut c_void, c_int, *mut perf_event_header) -> bpf_perf_event_ret,
        >,
        ctx: *mut c_void,
        opts: *mut perf_buffer_raw_opts,
    ) -> *mut perf_buffer;
    fn perf_buffer__poll(pb: *mut perf_buffer, timeout_ms: c_int) -> c_int;
    fn perf_buffer__free(pb: *mut perf_buffer);

    /* BAD_ARG() is a bpftool macro supplied by main.h. */
    fn BAD_ARG();
}

unsafe fn NEXT_ARG(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    *argc -= 1;
    *argv = (*argv).add(1);
}

unsafe extern "C" fn int_exit(_signo: c_int) {
    fprintf(stderr, b"Stopping...\n\0".as_ptr() as *const c_char);
    stop = true;
}

unsafe extern "C" fn print_bpf_output(
    private_data: *mut c_void,
    cpu: c_int,
    event: *mut perf_event_header,
) -> bpf_perf_event_ret {
    let e = event as *mut perf_event_sample;
    let lost = event as *mut perf_event_lost;
    let ctx = private_data as *mut event_pipe_ctx;
    let idx = if (*ctx).all_cpus { cpu } else { (*ctx).idx };

    if json_output {
        jsonw_start_object(json_wtr);
        jsonw_name(json_wtr, b"type\0".as_ptr() as *const c_char);
        jsonw_uint(json_wtr, (*e).header.type_ as c_ulonglong);
        jsonw_name(json_wtr, b"cpu\0".as_ptr() as *const c_char);
        jsonw_uint(json_wtr, cpu as c_ulonglong);
        jsonw_name(json_wtr, b"index\0".as_ptr() as *const c_char);
        jsonw_uint(json_wtr, idx as c_ulonglong);
        if (*e).header.type_ == PERF_RECORD_SAMPLE {
            jsonw_name(json_wtr, b"timestamp\0".as_ptr() as *const c_char);
            jsonw_uint(json_wtr, (*e).time as c_ulonglong);
            jsonw_name(json_wtr, b"data\0".as_ptr() as *const c_char);
            print_data_json((*e).data.as_ptr(), (*e).size);
        } else if (*e).header.type_ == PERF_RECORD_LOST {
            jsonw_name(json_wtr, b"lost\0".as_ptr() as *const c_char);
            jsonw_start_object(json_wtr);
            jsonw_name(json_wtr, b"id\0".as_ptr() as *const c_char);
            jsonw_uint(json_wtr, (*lost).id as c_ulonglong);
            jsonw_name(json_wtr, b"count\0".as_ptr() as *const c_char);
            jsonw_uint(json_wtr, (*lost).lost as c_ulonglong);
            jsonw_end_object(json_wtr);
        }
        jsonw_end_object(json_wtr);
    } else {
        if (*e).header.type_ == PERF_RECORD_SAMPLE {
            printf(
                b"== @%llu.%09llu CPU: %d index: %d =====\n\0".as_ptr() as *const c_char,
                ((*e).time / 1000000000u64) as c_ulonglong,
                ((*e).time % 1000000000u64) as c_ulonglong,
                cpu,
                idx,
            );
            fprint_hex(stdout, (*e).data.as_ptr(), (*e).size, b" \0".as_ptr() as *const c_char);
            printf(b"\n\0".as_ptr() as *const c_char);
        } else if (*e).header.type_ == PERF_RECORD_LOST {
            printf(
                b"lost %llu events\n\0".as_ptr() as *const c_char,
                (*lost).lost as c_ulonglong,
            );
        } else {
            printf(
                b"unknown event type=%u size=%u\n\0".as_ptr() as *const c_char,
                (*e).header.type_,
                (*e).header.size as c_uint,
            );
        }
    }

    LIBBPF_PERF_EVENT_CONT
}

#[no_mangle]
pub unsafe extern "C" fn do_event_pipe(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut perf_attr = perf_event_attr {
        sample_type: PERF_SAMPLE_RAW | PERF_SAMPLE_TIME,
        type_: PERF_TYPE_SOFTWARE,
        config: PERF_COUNT_SW_BPF_OUTPUT,
        sample_period: 1,
        wakeup_events: 1,
        size: 0,
        read_format: 0,
        flags: 0,
    };
    let mut map_info = bpf_map_info { type_: 0 };
    let mut opts = perf_buffer_raw_opts {
        sz: size_of::<perf_buffer_raw_opts>(),
        cpu_cnt: 0,
        cpus: ptr::null_mut(),
        map_keys: ptr::null_mut(),
    };
    let mut ctx = event_pipe_ctx {
        all_cpus: true,
        cpu: -1,
        idx: -1,
    };
    let mut pb: *mut perf_buffer;
    let mut map_info_len: __u32;
    let mut err: c_int;
    let map_fd: c_int;

    map_info_len = size_of::<bpf_map_info>() as __u32;
    map_fd = map_parse_fd_and_info(&mut argc, &mut argv, &mut map_info, &mut map_info_len, 0);
    if map_fd < 0 {
        return -1;
    }

    if map_info.type_ != BPF_MAP_TYPE_PERF_EVENT_ARRAY {
        p_err(b"map is not a perf event array\0".as_ptr() as *const c_char);
        goto_err_close_map(map_fd);
        return -1;
    }

    while argc != 0 {
        if argc < 2 {
            BAD_ARG();
            goto_err_close_map(map_fd);
            return -1;
        }

        if is_prefix(*argv, b"cpu\0".as_ptr() as *const c_char) {
            let mut endptr: *mut c_char = ptr::null_mut();

            NEXT_ARG(&mut argc, &mut argv);
            ctx.cpu = strtoul(*argv, &mut endptr, 0) as c_int;
            if *endptr != 0 {
                p_err(b"can't parse %s as CPU ID\0".as_ptr() as *const c_char, *argv);
                goto_err_close_map(map_fd);
                return -1;
            }

            NEXT_ARG(&mut argc, &mut argv);
        } else if is_prefix(*argv, b"index\0".as_ptr() as *const c_char) {
            let mut endptr: *mut c_char = ptr::null_mut();

            NEXT_ARG(&mut argc, &mut argv);
            ctx.idx = strtoul(*argv, &mut endptr, 0) as c_int;
            if *endptr != 0 {
                p_err(b"can't parse %s as index\0".as_ptr() as *const c_char, *argv);
                goto_err_close_map(map_fd);
                return -1;
            }

            NEXT_ARG(&mut argc, &mut argv);
        } else {
            BAD_ARG();
            goto_err_close_map(map_fd);
            return -1;
        }

        ctx.all_cpus = false;
    }

    if !ctx.all_cpus {
        if ctx.idx == -1 || ctx.cpu == -1 {
            p_err(b"cpu and index must be specified together\0".as_ptr() as *const c_char);
            goto_err_close_map(map_fd);
            return -1;
        }
    } else {
        ctx.cpu = 0;
        ctx.idx = 0;
    }

    opts.cpu_cnt = if ctx.all_cpus { 0 } else { 1 };
    opts.cpus = &mut ctx.cpu;
    opts.map_keys = &mut ctx.idx;
    pb = perf_buffer__new_raw(
        map_fd,
        MMAP_PAGE_CNT,
        &mut perf_attr,
        Some(print_bpf_output),
        &mut ctx as *mut event_pipe_ctx as *mut c_void,
        &mut opts,
    );
    if pb.is_null() {
        p_err(
            b"failed to create perf buffer: %s (%d)\0".as_ptr() as *const c_char,
            strerror(errno),
            errno,
        );
        goto_err_close_map(map_fd);
        return -1;
    }

    signal(SIGINT, Some(int_exit));
    signal(SIGHUP, Some(int_exit));
    signal(SIGTERM, Some(int_exit));

    if json_output {
        jsonw_start_array(json_wtr);
    }

    while !stop {
        err = perf_buffer__poll(pb, 200);
        if err < 0 && err != -EINTR {
            p_err(
                b"perf buffer polling failed: %s (%d)\0".as_ptr() as *const c_char,
                strerror(errno),
                errno,
            );
            perf_buffer__free(pb);
            close(map_fd);
            return -1;
        }
    }

    if json_output {
        jsonw_end_array(json_wtr);
    }

    perf_buffer__free(pb);
    close(map_fd);

    0
}

unsafe fn goto_err_close_map(map_fd: c_int) {
    close(map_fd);
}
