// SPDX-License-Identifier: GPL-2.0
/*
 * dlfilter-show-cycles.c: Print the number of cycles at the start of each line
 * Copyright (c) 2021, Intel Corporation.
 */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int};

type __u32 = u32;
type __s32 = i32;
type __u64 = u64;

const MAX_CPU: usize = 4096;

const INSTR_CYC: c_int = 0;
const BRNCH_CYC: c_int = 1;
const OTHER_CYC: c_int = 2;
const MAX_ENTRY: usize = 3;

static mut cycles: [[__u64; MAX_ENTRY]; MAX_CPU] = [[0; MAX_ENTRY]; MAX_CPU];
static mut cycles_rpt: [[__u64; MAX_ENTRY]; MAX_CPU] = [[0; MAX_ENTRY]; MAX_CPU];

const BITS: usize = 16;
const TABLESZ: usize = 1 << BITS;
const TABLEMAX: c_int = (TABLESZ / 2) as c_int;
const MASK: __u32 = (TABLESZ - 1) as __u32;

#[repr(C)]
struct entry {
    used: __u32,
    tid: __s32,
    cycles: [__u64; MAX_ENTRY],
    cycles_rpt: [__u64; MAX_ENTRY],
}

static mut table: [entry; TABLESZ] = [entry {
    used: 0,
    tid: 0,
    cycles: [0; MAX_ENTRY],
    cycles_rpt: [0; MAX_ENTRY],
}; TABLESZ];

static mut tid_cnt: c_int = 0;

#[repr(C)]
pub struct perf_dlfilter_sample {
    pub event: *const c_char,
    pub cpu: __s32,
    pub tid: __s32,
    pub cyc_cnt: __u64,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

unsafe fn event_entry(event: *const c_char) -> c_int {
    if event.is_null() {
        return OTHER_CYC;
    }
    if strncmp(event, c"instructions".as_ptr(), 12) == 0 {
        return INSTR_CYC;
    }
    if strncmp(event, c"branches".as_ptr(), 8) == 0 {
        return BRNCH_CYC;
    }
    OTHER_CYC
}

unsafe fn find_entry(tid: __s32) -> *mut entry {
    let mut pos: __u32 = (tid as __u32) & MASK;
    let mut e: *mut entry;

    e = &raw mut table[pos as usize];
    while (*e).used != 0 {
        if (*e).tid == tid {
            return e;
        }
        pos = pos.wrapping_add(1);
        if pos as usize == TABLESZ {
            pos = 0;
        }
        e = &raw mut table[pos as usize];
    }

    if tid_cnt >= TABLEMAX {
        fprintf(stderr, c"Too many threads\n".as_ptr());
        return std::ptr::null_mut();
    }

    tid_cnt += 1;
    (*e).used = 1;
    (*e).tid = tid;
    e
}

unsafe fn add_entry(tid: __s32, pos: c_int, cnt: __u64) {
    let e: *mut entry = find_entry(tid);

    if !e.is_null() {
        (*e).cycles[pos as usize] = (*e).cycles[pos as usize].wrapping_add(cnt);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn filter_event_early(
    data: *mut c_void,
    sample: *const perf_dlfilter_sample,
    ctx: *mut c_void,
) -> c_int {
    let cpu: __s32 = (*sample).cpu;
    let tid: __s32 = (*sample).tid;
    let pos: c_int;

    let _ = data;
    let _ = ctx;

    if (*sample).cyc_cnt == 0 {
        return 0;
    }

    pos = event_entry((*sample).event);

    if cpu >= 0 && cpu < MAX_CPU as __s32 {
        cycles[cpu as usize][pos as usize] =
            cycles[cpu as usize][pos as usize].wrapping_add((*sample).cyc_cnt);
    } else if tid != -1 {
        add_entry(tid, pos, (*sample).cyc_cnt);
    }
    0
}

unsafe fn print_vals(cycles: __u64, delta: __u64) {
    if delta != 0 {
        printf(
            c"%10llu %10llu ".as_ptr(),
            cycles as libc_ulonglong,
            delta as libc_ulonglong,
        );
    } else {
        printf(c"%10llu %10s ".as_ptr(), cycles as libc_ulonglong, c"".as_ptr());
    }
}

type libc_ulonglong = u64;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn filter_event(
    data: *mut c_void,
    sample: *const perf_dlfilter_sample,
    ctx: *mut c_void,
) -> c_int {
    let cpu: __s32 = (*sample).cpu;
    let tid: __s32 = (*sample).tid;
    let pos: c_int;

    let _ = data;
    let _ = ctx;

    pos = event_entry((*sample).event);

    if cpu >= 0 && cpu < MAX_CPU as __s32 {
        print_vals(
            cycles[cpu as usize][pos as usize],
            cycles[cpu as usize][pos as usize].wrapping_sub(cycles_rpt[cpu as usize][pos as usize]),
        );
        cycles_rpt[cpu as usize][pos as usize] = cycles[cpu as usize][pos as usize];
        return 0;
    }

    if tid != -1 {
        let e: *mut entry = find_entry(tid);

        if !e.is_null() {
            print_vals(
                (*e).cycles[pos as usize],
                (*e).cycles[pos as usize].wrapping_sub((*e).cycles_rpt[pos as usize]),
            );
            (*e).cycles_rpt[pos as usize] = (*e).cycles[pos as usize];
            return 0;
        }
    }

    printf(c"%22s".as_ptr(), c"".as_ptr());
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn filter_description(
    long_description: *mut *const c_char,
) -> *const c_char {
    static long_desc: &[u8] = b"Cycle counts are accumulated per CPU (or \
per thread if CPU is not recorded) from IPC information, and \
printed together with the change since the last print, at the \
start of each line. Separate counts are kept for branches, \
instructions or other events.\0";

    *long_description = long_desc.as_ptr() as *const c_char;
    c"Print the number of cycles at the start of each line".as_ptr()
}
