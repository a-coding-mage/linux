// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/values.c.
// C includes required inttypes.h, stdio.h, stdlib.h, string.h, errno.h,
// linux/zalloc.h, values.h, debug.h, and evsel.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type u32 = u32;
pub type u64 = u64;

const ENOMEM: c_int = 12;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel_core {
    pub idx: c_uint,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct perf_read_values {
    pub threads_max: c_int,
    pub pid: *mut u32,
    pub tid: *mut u32,
    pub value: *mut *mut u64,
    pub threads: c_int,
    pub counters_max: c_int,
    pub counters: *mut *mut evsel,
    pub num_counters: c_int,
}

unsafe extern "C" {
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;

    fn pr_debug(format: *const c_char, ...);
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
}

unsafe fn zfree<T>(ptr: *mut *mut T) {
    if !(*ptr).is_null() {
        free(*ptr as *mut c_void);
        *ptr = core::ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_read_values_init(values: *mut perf_read_values) -> c_int {
    (*values).threads_max = 16;
    (*values).pid = calloc(
        (*values).threads_max as usize,
        core::mem::size_of::<u32>(),
    ) as *mut u32;
    (*values).tid = calloc(
        (*values).threads_max as usize,
        core::mem::size_of::<u32>(),
    ) as *mut u32;
    (*values).value = calloc(
        (*values).threads_max as usize,
        core::mem::size_of::<*mut u64>(),
    ) as *mut *mut u64;
    if (*values).pid.is_null() || (*values).tid.is_null() || (*values).value.is_null() {
        pr_debug(c"failed to allocate read_values threads arrays".as_ptr());
        zfree(&mut (*values).pid);
        zfree(&mut (*values).tid);
        zfree(&mut (*values).value);
        return -ENOMEM;
    }
    (*values).threads = 0;

    (*values).counters_max = 16;
    (*values).counters = malloc(
        (*values).counters_max as usize * core::mem::size_of::<*mut evsel>(),
    ) as *mut *mut evsel;
    if (*values).counters.is_null() {
        pr_debug(c"failed to allocate read_values counters array".as_ptr());
        zfree(&mut (*values).counters);
        zfree(&mut (*values).pid);
        zfree(&mut (*values).tid);
        zfree(&mut (*values).value);
        return -ENOMEM;
    }
    (*values).num_counters = 0;

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_read_values_destroy(values: *mut perf_read_values) {
    let mut i: c_int;

    if (*values).threads_max == 0 || (*values).counters_max == 0 {
        return;
    }

    i = 0;
    while i < (*values).threads {
        zfree((*values).value.offset(i as isize));
        i += 1;
    }
    zfree(&mut (*values).value);
    zfree(&mut (*values).pid);
    zfree(&mut (*values).tid);
    zfree(&mut (*values).counters);
}

unsafe fn perf_read_values__enlarge_threads(values: *mut perf_read_values) -> c_int {
    let nthreads_max: c_int = (*values).threads_max * 2;
    let npid: *mut c_void = realloc(
        (*values).pid as *mut c_void,
        nthreads_max as usize * core::mem::size_of::<u32>(),
    );
    let ntid: *mut c_void = realloc(
        (*values).tid as *mut c_void,
        nthreads_max as usize * core::mem::size_of::<u32>(),
    );
    let nvalue: *mut c_void = realloc(
        (*values).value as *mut c_void,
        nthreads_max as usize * core::mem::size_of::<*mut u64>(),
    );

    if npid.is_null() || ntid.is_null() || nvalue.is_null() {
        free(npid);
        free(ntid);
        free(nvalue);
        pr_debug(c"failed to enlarge read_values threads arrays".as_ptr());
        return -ENOMEM;
    }

    (*values).threads_max = nthreads_max;
    (*values).pid = npid as *mut u32;
    (*values).tid = ntid as *mut u32;
    (*values).value = nvalue as *mut *mut u64;
    0
}

unsafe fn perf_read_values__findnew_thread(
    values: *mut perf_read_values,
    pid: u32,
    tid: u32,
) -> c_int {
    let mut i: c_int;

    i = 0;
    while i < (*values).threads {
        if *(*values).pid.offset(i as isize) == pid && *(*values).tid.offset(i as isize) == tid {
            return i;
        }
        i += 1;
    }

    if (*values).threads == (*values).threads_max {
        i = perf_read_values__enlarge_threads(values);
        if i < 0 {
            return i;
        }
    }

    i = (*values).threads;

    *(*values).value.offset(i as isize) = calloc(
        (*values).counters_max as usize,
        core::mem::size_of::<u64>(),
    ) as *mut u64;
    if (*(*values).value.offset(i as isize)).is_null() {
        pr_debug(c"failed to allocate read_values counters array".as_ptr());
        return -ENOMEM;
    }
    *(*values).pid.offset(i as isize) = pid;
    *(*values).tid.offset(i as isize) = tid;
    (*values).threads = i + 1;

    i
}

unsafe fn perf_read_values__enlarge_counters(values: *mut perf_read_values) -> c_int {
    let counters_max: c_int = (*values).counters_max * 2;
    let new_counters: *mut *mut evsel = realloc(
        (*values).counters as *mut c_void,
        counters_max as usize * core::mem::size_of::<*mut evsel>(),
    ) as *mut *mut evsel;

    if new_counters.is_null() {
        pr_debug(c"failed to enlarge read_values counters array".as_ptr());
        return -ENOMEM;
    }

    let mut i: c_int = 0;
    while i < (*values).threads {
        let value: *mut u64 = realloc(
            *(*values).value.offset(i as isize) as *mut c_void,
            counters_max as usize * core::mem::size_of::<u64>(),
        ) as *mut u64;

        if value.is_null() {
            pr_debug(c"failed to enlarge read_values ->values array".as_ptr());
            free(new_counters as *mut c_void);
            return -ENOMEM;
        }

        let mut j: c_int = (*values).counters_max;
        while j < counters_max {
            *value.offset(j as isize) = 0;
            j += 1;
        }

        *(*values).value.offset(i as isize) = value;
        i += 1;
    }

    (*values).counters_max = counters_max;
    (*values).counters = new_counters;

    0
}

unsafe fn perf_read_values__findnew_counter(
    values: *mut perf_read_values,
    evsel: *mut evsel,
) -> c_int {
    let mut i: c_int;

    i = 0;
    while i < (*values).num_counters {
        if *(*values).counters.offset(i as isize) == evsel {
            return i;
        }
        i += 1;
    }

    if (*values).num_counters == (*values).counters_max {
        let err: c_int = perf_read_values__enlarge_counters(values);

        if err != 0 {
            return err;
        }
    }

    i = (*values).num_counters;
    (*values).num_counters += 1;
    *(*values).counters.offset(i as isize) = evsel;

    i
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_read_values_add_value(
    values: *mut perf_read_values,
    pid: u32,
    tid: u32,
    evsel: *mut evsel,
    value: u64,
) -> c_int {
    let tindex: c_int;
    let cindex: c_int;

    tindex = perf_read_values__findnew_thread(values, pid, tid);
    if tindex < 0 {
        return tindex;
    }
    cindex = perf_read_values__findnew_counter(values, evsel);
    if cindex < 0 {
        return cindex;
    }

    let slot = (*(*values).value.offset(tindex as isize)).offset(cindex as isize);
    *slot = (*slot).wrapping_add(value);
    0
}

unsafe fn perf_read_values__display_pretty(fp: *mut FILE, values: *mut perf_read_values) {
    let mut i: c_int;
    let mut j: c_int;
    let mut pidwidth: c_int;
    let mut tidwidth: c_int;
    let counterwidth: *mut c_int;

    counterwidth = malloc((*values).num_counters as usize * core::mem::size_of::<c_int>())
        as *mut c_int;
    if counterwidth.is_null() {
        fprintf(
            fp,
            c"INTERNAL ERROR: Failed to allocate counterwidth array\n".as_ptr(),
        );
        return;
    }
    tidwidth = 3;
    pidwidth = 3;
    j = 0;
    while j < (*values).num_counters {
        *counterwidth.offset(j as isize) =
            strlen(evsel__name(*(*values).counters.offset(j as isize))) as c_int;
        j += 1;
    }
    i = 0;
    while i < (*values).threads {
        let mut width: c_int;

        width = snprintf(
            core::ptr::null_mut(),
            0,
            c"%d".as_ptr(),
            *(*values).pid.offset(i as isize),
        );
        if width > pidwidth {
            pidwidth = width;
        }
        width = snprintf(
            core::ptr::null_mut(),
            0,
            c"%d".as_ptr(),
            *(*values).tid.offset(i as isize),
        );
        if width > tidwidth {
            tidwidth = width;
        }
        j = 0;
        while j < (*values).num_counters {
            width = snprintf(
                core::ptr::null_mut(),
                0,
                c"%llu".as_ptr(),
                *(*(*values).value.offset(i as isize)).offset(j as isize),
            );
            if width > *counterwidth.offset(j as isize) {
                *counterwidth.offset(j as isize) = width;
            }
            j += 1;
        }
        i += 1;
    }

    fprintf(fp, c"# %*s  %*s".as_ptr(), pidwidth, c"PID".as_ptr(), tidwidth, c"TID".as_ptr());
    j = 0;
    while j < (*values).num_counters {
        fprintf(
            fp,
            c"  %*s".as_ptr(),
            *counterwidth.offset(j as isize),
            evsel__name(*(*values).counters.offset(j as isize)),
        );
        j += 1;
    }
    fprintf(fp, c"\n".as_ptr());

    i = 0;
    while i < (*values).threads {
        fprintf(
            fp,
            c"  %*d  %*d".as_ptr(),
            pidwidth,
            *(*values).pid.offset(i as isize),
            tidwidth,
            *(*values).tid.offset(i as isize),
        );
        j = 0;
        while j < (*values).num_counters {
            fprintf(
                fp,
                c"  %*llu".as_ptr(),
                *counterwidth.offset(j as isize),
                *(*(*values).value.offset(i as isize)).offset(j as isize),
            );
            j += 1;
        }
        fprintf(fp, c"\n".as_ptr());
        i += 1;
    }
    free(counterwidth as *mut c_void);
}

unsafe fn perf_read_values__display_raw(fp: *mut FILE, values: *mut perf_read_values) {
    let mut width: c_int;
    let mut pidwidth: c_int;
    let mut tidwidth: c_int;
    let mut namewidth: c_int;
    let mut rawwidth: c_int;
    let mut countwidth: c_int;
    let mut i: c_int;
    let mut j: c_int;

    tidwidth = 3; /* TID */
    pidwidth = 3; /* PID */
    namewidth = 4; /* "Name" */
    rawwidth = 3; /* "Raw" */
    countwidth = 5; /* "Count" */

    i = 0;
    while i < (*values).threads {
        width = snprintf(
            core::ptr::null_mut(),
            0,
            c"%d".as_ptr(),
            *(*values).pid.offset(i as isize),
        );
        if width > pidwidth {
            pidwidth = width;
        }
        width = snprintf(
            core::ptr::null_mut(),
            0,
            c"%d".as_ptr(),
            *(*values).tid.offset(i as isize),
        );
        if width > tidwidth {
            tidwidth = width;
        }
        i += 1;
    }
    j = 0;
    while j < (*values).num_counters {
        width = strlen(evsel__name(*(*values).counters.offset(j as isize))) as c_int;
        if width > namewidth {
            namewidth = width;
        }
        width = snprintf(
            core::ptr::null_mut(),
            0,
            c"%x".as_ptr(),
            (**(*values).counters.offset(j as isize)).core.idx,
        );
        if width > rawwidth {
            rawwidth = width;
        }
        j += 1;
    }
    i = 0;
    while i < (*values).threads {
        j = 0;
        while j < (*values).num_counters {
            width = snprintf(
                core::ptr::null_mut(),
                0,
                c"%llu".as_ptr(),
                *(*(*values).value.offset(i as isize)).offset(j as isize),
            );
            if width > countwidth {
                countwidth = width;
            }
            j += 1;
        }
        i += 1;
    }

    fprintf(
        fp,
        c"# %*s  %*s  %*s  %*s  %*s\n".as_ptr(),
        pidwidth,
        c"PID".as_ptr(),
        tidwidth,
        c"TID".as_ptr(),
        namewidth,
        c"Name".as_ptr(),
        rawwidth,
        c"Raw".as_ptr(),
        countwidth,
        c"Count".as_ptr(),
    );
    i = 0;
    while i < (*values).threads {
        j = 0;
        while j < (*values).num_counters {
            fprintf(
                fp,
                c"  %*d  %*d  %*s  %*x  %*llu".as_ptr(),
                pidwidth,
                *(*values).pid.offset(i as isize),
                tidwidth,
                *(*values).tid.offset(i as isize),
                namewidth,
                evsel__name(*(*values).counters.offset(j as isize)),
                rawwidth,
                (**(*values).counters.offset(j as isize)).core.idx,
                countwidth,
                *(*(*values).value.offset(i as isize)).offset(j as isize),
            );
            j += 1;
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_read_values_display(
    fp: *mut FILE,
    values: *mut perf_read_values,
    raw: c_int,
) {
    if raw != 0 {
        perf_read_values__display_raw(fp, values);
    } else {
        perf_read_values__display_pretty(fp, values);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
