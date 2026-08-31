// SPDX-License-Identifier: GPL-2.0-only

/* kselftest for allocinfo ioctl
 * allocinfo ioctl retrieves allocinfo data through ioctl
 * Copyright (C) 2026 Google, Inc.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

type size_t = usize;
type __u64 = u64;

// Dependencies from C headers:
// errno.h, fcntl.h, stdio.h, stdlib.h, string.h, stdbool.h, unistd.h,
// sys/ioctl.h, linux/types.h, linux/alloc_tag.h, and ../kselftest.h.

const MAX_LINE_LEN: usize = 512;
const ALLOCINFO_PROC: &[u8] = b"/proc/allocinfo\0";

const O_RDONLY: c_int = 0;

const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;

const ALLOCINFO_STR_SIZE: usize = 64;
const ALLOCINFO_IOC_CONTENT_ID: c_ulong = 0;
const ALLOCINFO_IOC_GET_AT: c_ulong = 0;
const ALLOCINFO_IOC_GET_NEXT: c_ulong = 0;
const ALLOCINFO_FILTER_MASK_FILENAME: __u64 = 1 << 0;
const ALLOCINFO_FILTER_MASK_FUNCTION: __u64 = 1 << 1;
const ALLOCINFO_FILTER_MASK_LINENO: __u64 = 1 << 2;
const ALLOCINFO_FILTER_MASK_MIN_SIZE: __u64 = 1 << 3;
const ALLOCINFO_FILTER_MASK_MAX_SIZE: __u64 = 1 << 4;

#[repr(C)]
#[derive(Copy, Clone)]
struct allocinfo_counter {
    bytes: __u64,
    calls: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct allocinfo_tag {
    filename: [c_char; ALLOCINFO_STR_SIZE],
    function: [c_char; ALLOCINFO_STR_SIZE],
    lineno: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct allocinfo_tag_data {
    counter: allocinfo_counter,
    tag: allocinfo_tag,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct allocinfo_filter_fields {
    filename: [c_char; ALLOCINFO_STR_SIZE],
    function: [c_char; ALLOCINFO_STR_SIZE],
    lineno: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct allocinfo_filter {
    mask: __u64,
    fields: allocinfo_filter_fields,
    min_size: __u64,
    max_size: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct allocinfo_content_id {
    id: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct allocinfo_get_at {
    filter: allocinfo_filter,
    pos: __u64,
    data: allocinfo_tag_data,
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;

    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_set_plan(plan: c_int);
    fn ksft_test_result(condition: bool, fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_finished() -> !;
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone)]
enum ioctl_ret {
    IOCTL_SUCCESS = 0,
    IOCTL_FAILURE = 1,
    IOCTL_INVALID_DATA = 2,
}

const VEC_MAX_ENTRIES: usize = 32;

#[repr(C)]
struct allocinfo_tag_data_vec {
    tag: [allocinfo_tag_data; VEC_MAX_ENTRIES],
    count: __u64,
}

unsafe fn __allocinfo_get_content_id(
    dev_fd: c_int,
    params: *mut allocinfo_content_id,
) -> c_int {
    unsafe { ioctl(dev_fd, ALLOCINFO_IOC_CONTENT_ID, params) }
}

unsafe fn __allocinfo_get_at(dev_fd: c_int, params: *mut allocinfo_get_at) -> c_int {
    unsafe { ioctl(dev_fd, ALLOCINFO_IOC_GET_AT, params) }
}

unsafe fn __allocinfo_get_next(dev_fd: c_int, params: *mut allocinfo_tag_data) -> c_int {
    unsafe { ioctl(dev_fd, ALLOCINFO_IOC_GET_NEXT, params) }
}

unsafe fn match_entry(
    procfs_entry: *const allocinfo_tag_data,
    tag_data: *const allocinfo_tag_data,
    match_bytes: bool,
    match_calls: bool,
    match_lineno: bool,
    match_function: bool,
    match_filename: bool,
) -> bool {
    if match_bytes && unsafe { (*tag_data).counter.bytes != (*procfs_entry).counter.bytes } {
        unsafe { ksft_print_msg(c"size retrieved through ioctl does not match procfs\n".as_ptr()) };
        return false;
    }

    if match_calls && unsafe { (*tag_data).counter.calls != (*procfs_entry).counter.calls } {
        unsafe {
            ksft_print_msg(c"call count retrieved through ioctl does not match procfs\n".as_ptr())
        };
        return false;
    }

    if match_lineno && unsafe { (*tag_data).tag.lineno != (*procfs_entry).tag.lineno } {
        unsafe { ksft_print_msg(c"lineno retrieved through ioctl does not match procfs\n".as_ptr()) };
        return false;
    }

    if match_function
        && unsafe {
            strncmp(
                (*tag_data).tag.function.as_ptr(),
                (*procfs_entry).tag.function.as_ptr(),
                ALLOCINFO_STR_SIZE,
            ) != 0
        }
    {
        unsafe {
            ksft_print_msg(c"function retrieved through ioctl does not match procfs\n".as_ptr())
        };
        return false;
    }

    if match_filename
        && unsafe {
            strncmp(
                (*tag_data).tag.filename.as_ptr(),
                (*procfs_entry).tag.filename.as_ptr(),
                ALLOCINFO_STR_SIZE,
            ) != 0
        }
    {
        unsafe {
            ksft_print_msg(c"filename retrieved through ioctl does not match procfs\n".as_ptr())
        };
        return false;
    }
    true
}

unsafe fn match_entries(
    procfs_entries: *const allocinfo_tag_data_vec,
    tags: *const allocinfo_tag_data_vec,
    match_bytes: bool,
    match_calls: bool,
    match_lineno: bool,
    match_function: bool,
    match_filename: bool,
) -> bool {
    let mut i: __u64;

    if unsafe { (*procfs_entries).count != (*tags).count } {
        unsafe {
            ksft_print_msg(
                c"Entry count mismatch. ioctl entries: %llu, proc entries: %llu\n".as_ptr(),
                (*tags).count,
                (*procfs_entries).count,
            )
        };
        return false;
    }
    i = 0;
    while unsafe { i < (*procfs_entries).count } {
        if !unsafe {
            match_entry(
                (*procfs_entries).tag.as_ptr().add(i as usize),
                (*tags).tag.as_ptr().add(i as usize),
                match_bytes,
                match_calls,
                match_lineno,
                match_function,
                match_filename,
            )
        } {
            unsafe { ksft_print_msg(c"%lluth entry does not match.\n".as_ptr(), i) };
            return false;
        }
        i += 1;
    }
    true
}

unsafe fn allocinfo_str(mut str_: *const c_char) -> *const c_char {
    let len: size_t = unsafe { strlen(str_) };

    if len >= ALLOCINFO_STR_SIZE {
        str_ = unsafe { str_.add((len - ALLOCINFO_STR_SIZE) + 1) };
    }
    str_
}

unsafe fn allocinfo_copy_str(dest: *mut c_char, src: *const c_char) {
    unsafe { strncpy(dest, allocinfo_str(src), ALLOCINFO_STR_SIZE - 1) };
    unsafe { *dest.add(ALLOCINFO_STR_SIZE - 1) = 0 };
}

unsafe fn get_filtered_procfs_entries(
    procfs_entries: *mut allocinfo_tag_data_vec,
    filter: *const allocinfo_filter,
) -> c_int {
    let fp: *mut FILE = unsafe { fopen(ALLOCINFO_PROC.as_ptr() as *const c_char, c"r".as_ptr()) };
    let mut line: [c_char; MAX_LINE_LEN] = [0; MAX_LINE_LEN];
    let mut matches: c_int;
    let mut procfs_entry: allocinfo_tag_data =
        unsafe { core::mem::zeroed::<allocinfo_tag_data>() };

    if fp.is_null() {
        unsafe { ksft_print_msg(c"Failed to open /proc/allocinfo for reading\n".as_ptr()) };
        return 1;
    }
    unsafe {
        memset(
            procfs_entries as *mut c_void,
            0,
            core::mem::size_of::<allocinfo_tag_data_vec>(),
        )
    };
    while unsafe { !fgets(line.as_mut_ptr(), line.len() as c_int, fp).is_null() }
        && unsafe { (*procfs_entries).count < VEC_MAX_ENTRIES as __u64 }
    {
        let mut filename: [c_char; MAX_LINE_LEN] = [0; MAX_LINE_LEN];
        let mut function: [c_char; MAX_LINE_LEN] = [0; MAX_LINE_LEN];

        unsafe {
            memset(
                &mut procfs_entry as *mut _ as *mut c_void,
                0,
                core::mem::size_of_val(&procfs_entry),
            )
        };
        matches = unsafe {
            sscanf(
                line.as_ptr(),
                c"%llu %llu %[^:]:%llu func:%s".as_ptr(),
                &mut procfs_entry.counter.bytes,
                &mut procfs_entry.counter.calls,
                filename.as_mut_ptr(),
                &mut procfs_entry.tag.lineno,
                function.as_mut_ptr(),
            )
        };

        if matches != 5 {
            continue;
        }

        unsafe { allocinfo_copy_str(procfs_entry.tag.filename.as_mut_ptr(), filename.as_ptr()) };
        unsafe { allocinfo_copy_str(procfs_entry.tag.function.as_mut_ptr(), function.as_ptr()) };

        if unsafe { (*filter).mask & ALLOCINFO_FILTER_MASK_FILENAME } != 0 {
            if unsafe {
                strncmp(
                    procfs_entry.tag.filename.as_ptr(),
                    (*filter).fields.filename.as_ptr(),
                    ALLOCINFO_STR_SIZE,
                ) != 0
            } {
                continue;
            }
        }
        if unsafe { (*filter).mask & ALLOCINFO_FILTER_MASK_FUNCTION } != 0 {
            if unsafe {
                strncmp(
                    procfs_entry.tag.function.as_ptr(),
                    (*filter).fields.function.as_ptr(),
                    ALLOCINFO_STR_SIZE,
                ) != 0
            } {
                continue;
            }
        }
        if unsafe { (*filter).mask & ALLOCINFO_FILTER_MASK_LINENO } != 0 {
            if procfs_entry.tag.lineno != unsafe { (*filter).fields.lineno } {
                continue;
            }
        }
        if unsafe { (*filter).mask & ALLOCINFO_FILTER_MASK_MIN_SIZE } != 0 {
            if procfs_entry.counter.bytes < unsafe { (*filter).min_size } {
                continue;
            }
        }
        if unsafe { (*filter).mask & ALLOCINFO_FILTER_MASK_MAX_SIZE } != 0 {
            if procfs_entry.counter.bytes > unsafe { (*filter).max_size } {
                continue;
            }
        }

        unsafe {
            let idx = (*procfs_entries).count as usize;
            (*procfs_entries).count += 1;
            memcpy(
                (*procfs_entries).tag.as_mut_ptr().add(idx) as *mut c_void,
                &procfs_entry as *const _ as *const c_void,
                core::mem::size_of_val(&procfs_entry),
            )
        };
    }
    unsafe { fclose(fp) };
    0
}

unsafe fn get_filtered_ioctl_entries(
    tags: *mut allocinfo_tag_data_vec,
    filter: *const allocinfo_filter,
    start_pos: __u64,
) -> ioctl_ret {
    let fd: c_int = unsafe { open(ALLOCINFO_PROC.as_ptr() as *const c_char, O_RDONLY) };

    if fd < 0 {
        unsafe { ksft_print_msg(c"Failed to open /proc/allocinfo for IOCTL\n".as_ptr()) };
        return ioctl_ret::IOCTL_FAILURE;
    }

    let mut start_cont_id: allocinfo_content_id = unsafe { core::mem::zeroed() };
    let mut end_cont_id: allocinfo_content_id = unsafe { core::mem::zeroed() };
    let mut get_at_params: allocinfo_get_at = unsafe { core::mem::zeroed() };
    let max_retries: c_int = 10;
    let mut retry_count: c_int = 0;
    let mut status: ioctl_ret;

    /*
     * __allocinfo_get_content_id may return different values if a kernel module was loaded
     * between the two calls. If that happens, the data gathered cannot be considered consistent
     * and hence needs to be fetched again to avoid flakiness.
     */
    loop {
        if unsafe { __allocinfo_get_content_id(fd, &mut start_cont_id) } != 0 {
            unsafe { ksft_print_msg(c"allocinfo_get_content_id failed\n".as_ptr()) };
            status = ioctl_ret::IOCTL_FAILURE;
            break;
        }

        unsafe {
            memset(
                tags as *mut c_void,
                0,
                core::mem::size_of::<allocinfo_tag_data_vec>(),
            );
            memset(
                &mut get_at_params as *mut _ as *mut c_void,
                0,
                core::mem::size_of_val(&get_at_params),
            );
            memcpy(
                &mut get_at_params.filter as *mut _ as *mut c_void,
                filter as *const c_void,
                core::mem::size_of::<allocinfo_filter>(),
            );
        }
        get_at_params.pos = start_pos;
        if unsafe { __allocinfo_get_at(fd, &mut get_at_params) } != 0 {
            unsafe { ksft_print_msg(c"allocinfo_get_at failed\n".as_ptr()) };
            status = ioctl_ret::IOCTL_FAILURE;
            break;
        }
        unsafe {
            let idx = (*tags).count as usize;
            (*tags).count += 1;
            memcpy(
                (*tags).tag.as_mut_ptr().add(idx) as *mut c_void,
                &get_at_params.data as *const _ as *const c_void,
                core::mem::size_of_val(&get_at_params.data),
            );
        }

        while unsafe { (*tags).count < VEC_MAX_ENTRIES as __u64 }
            && unsafe {
                __allocinfo_get_next(fd, (*tags).tag.as_mut_ptr().add((*tags).count as usize))
                    == 0
            }
        {
            unsafe { (*tags).count += 1 };
        }

        if unsafe { __allocinfo_get_content_id(fd, &mut end_cont_id) } != 0 {
            unsafe { ksft_print_msg(c"allocinfo_get_content_id failed\n".as_ptr()) };
            status = ioctl_ret::IOCTL_FAILURE;
            break;
        }

        if start_cont_id.id == end_cont_id.id {
            status = ioctl_ret::IOCTL_SUCCESS;
        } else {
            unsafe { ksft_print_msg(c"allocinfo_get_content_id mismatch, retrying...\n".as_ptr()) };
            status = ioctl_ret::IOCTL_INVALID_DATA;
        }

        if !(status == ioctl_ret::IOCTL_INVALID_DATA && {
            let old = retry_count;
            retry_count += 1;
            old < max_retries
        }) {
            break;
        }
    }

    unsafe { close(fd) };
    status
}

unsafe fn run_filter_test(filter: *const allocinfo_filter) -> c_int {
    let tags: *mut allocinfo_tag_data_vec =
        unsafe { malloc(core::mem::size_of::<allocinfo_tag_data_vec>()) as *mut _ };
    let procfs_entries: *mut allocinfo_tag_data_vec =
        unsafe { malloc(core::mem::size_of::<allocinfo_tag_data_vec>()) as *mut _ };
    let ioctl_status: ioctl_ret;
    let mut ret: c_int = KSFT_PASS;

    if tags.is_null() || procfs_entries.is_null() {
        unsafe { ksft_print_msg(c"Memory allocation failed.\n".as_ptr()) };
        ret = KSFT_FAIL;
        unsafe { free(tags as *mut c_void) };
        unsafe { free(procfs_entries as *mut c_void) };
        return ret;
    }

    if unsafe { get_filtered_procfs_entries(procfs_entries, filter) } != 0 {
        unsafe { ksft_print_msg(c"Error retrieving entries from /proc/allocinfo\n".as_ptr()) };
        ret = KSFT_SKIP;
        unsafe { free(tags as *mut c_void) };
        unsafe { free(procfs_entries as *mut c_void) };
        return ret;
    }

    if unsafe { (*procfs_entries).count == 0 } {
        unsafe { ksft_print_msg(c"No entries found in /proc/allocinfo, skipping test\n".as_ptr()) };
        ret = KSFT_SKIP;
        unsafe { free(tags as *mut c_void) };
        unsafe { free(procfs_entries as *mut c_void) };
        return ret;
    }

    ioctl_status = unsafe { get_filtered_ioctl_entries(tags, filter, 0) };
    if ioctl_status == ioctl_ret::IOCTL_INVALID_DATA {
        unsafe { ksft_print_msg(c"Trouble retrieving valid IOCTL entries, skipping.\n".as_ptr()) };
        ret = KSFT_SKIP;
        unsafe { free(tags as *mut c_void) };
        unsafe { free(procfs_entries as *mut c_void) };
        return ret;
    }
    if ioctl_status == ioctl_ret::IOCTL_FAILURE {
        unsafe { ksft_print_msg(c"Error retrieving IOCTL entries.\n".as_ptr()) };
        ret = KSFT_FAIL;
        unsafe { free(tags as *mut c_void) };
        unsafe { free(procfs_entries as *mut c_void) };
        return ret;
    }

    if !unsafe { match_entries(procfs_entries, tags, false, false, true, true, true) } {
        ret = KSFT_FAIL;
    }

    unsafe { free(tags as *mut c_void) };
    unsafe { free(procfs_entries as *mut c_void) };
    ret
}

unsafe fn test_filename_filter() -> c_int {
    let mut filter: allocinfo_filter = unsafe { core::mem::zeroed() };
    let target_filename: *const c_char = c"mm/memory.c".as_ptr();

    unsafe {
        memset(
            &mut filter as *mut _ as *mut c_void,
            0,
            core::mem::size_of_val(&filter),
        )
    };
    filter.mask |= ALLOCINFO_FILTER_MASK_FILENAME;
    unsafe {
        strncpy(
            filter.fields.filename.as_mut_ptr(),
            target_filename,
            ALLOCINFO_STR_SIZE,
        )
    };

    unsafe { run_filter_test(&filter) }
}

unsafe fn test_function_filter() -> c_int {
    let mut filter: allocinfo_filter = unsafe { core::mem::zeroed() };
    let target_function: *const c_char = c"dup_mm".as_ptr();

    unsafe {
        memset(
            &mut filter as *mut _ as *mut c_void,
            0,
            core::mem::size_of_val(&filter),
        )
    };
    filter.mask |= ALLOCINFO_FILTER_MASK_FUNCTION;
    unsafe {
        strncpy(
            filter.fields.function.as_mut_ptr(),
            target_function,
            ALLOCINFO_STR_SIZE,
        )
    };

    unsafe { run_filter_test(&filter) }
}

unsafe fn test_size_filter() -> c_int {
    let fd: c_int;
    let tags: *mut allocinfo_tag_data_vec =
        unsafe { malloc(core::mem::size_of::<allocinfo_tag_data_vec>()) as *mut _ };
    let procfs_entries: *mut allocinfo_tag_data_vec =
        unsafe { malloc(core::mem::size_of::<allocinfo_tag_data_vec>()) as *mut _ };
    let mut filter: allocinfo_filter = unsafe { core::mem::zeroed() };
    let mut ret: c_int = KSFT_PASS;
    let mut target_size: __u64 = 0;
    let mut i: __u64;
    let mut pos: __u64;
    let mut found_tag: *mut allocinfo_tag_data = core::ptr::null_mut();
    let target_function: *const c_char = c"do_init_module".as_ptr();
    let mut start_cont_id: allocinfo_content_id = unsafe { core::mem::zeroed() };
    let mut end_cont_id: allocinfo_content_id = unsafe { core::mem::zeroed() };
    let mut retry: c_int = 0;
    let max_retries: c_int = 10;

    if tags.is_null() || procfs_entries.is_null() {
        unsafe { ksft_print_msg(c"Memory allocation failed.\n".as_ptr()) };
        ret = KSFT_FAIL;
        unsafe { free(tags as *mut c_void) };
        unsafe { free(procfs_entries as *mut c_void) };
        return ret;
    }

    fd = unsafe { open(ALLOCINFO_PROC.as_ptr() as *const c_char, O_RDONLY) };
    if fd < 0 {
        unsafe {
            ksft_print_msg(
                c"Failed to open /proc/allocinfo: %s\n".as_ptr(),
                strerror(errno),
            )
        };
        ret = KSFT_SKIP;
        unsafe { free(tags as *mut c_void) };
        unsafe { free(procfs_entries as *mut c_void) };
        return ret;
    }

    loop {
        found_tag = core::ptr::null_mut();
        pos = 0;

        if unsafe { __allocinfo_get_content_id(fd, &mut start_cont_id) } != 0 {
            unsafe { ksft_print_msg(c"allocinfo_get_content_id failed\n".as_ptr()) };
            ret = KSFT_FAIL;
            unsafe { close(fd) };
            unsafe { free(tags as *mut c_void) };
            unsafe { free(procfs_entries as *mut c_void) };
            return ret;
        }

        unsafe {
            memset(
                &mut filter as *mut _ as *mut c_void,
                0,
                core::mem::size_of_val(&filter),
            )
        };
        filter.mask |= ALLOCINFO_FILTER_MASK_FUNCTION;
        unsafe {
            strncpy(
                filter.fields.function.as_mut_ptr(),
                target_function,
                ALLOCINFO_STR_SIZE,
            )
        };

        if unsafe { get_filtered_procfs_entries(procfs_entries, &filter) } != 0 {
            unsafe { ksft_print_msg(c"Error retrieving entries from /proc/allocinfo\n".as_ptr()) };
            ret = KSFT_SKIP;
            unsafe { close(fd) };
            unsafe { free(tags as *mut c_void) };
            unsafe { free(procfs_entries as *mut c_void) };
            return ret;
        }

        if unsafe { (*procfs_entries).count == 0 } {
            unsafe { ksft_print_msg(c"Function %s not found in procfs\n".as_ptr(), target_function) };
            ret = KSFT_SKIP;
            unsafe { close(fd) };
            unsafe { free(tags as *mut c_void) };
            unsafe { free(procfs_entries as *mut c_void) };
            return ret;
        }

        target_size = unsafe { (*procfs_entries).tag[0].counter.bytes };

        unsafe {
            memset(
                &mut filter as *mut _ as *mut c_void,
                0,
                core::mem::size_of_val(&filter),
            )
        };
        filter.mask |= ALLOCINFO_FILTER_MASK_MIN_SIZE | ALLOCINFO_FILTER_MASK_MAX_SIZE;
        filter.min_size = target_size;
        filter.max_size = target_size;

        loop {
            let mut get_at_params: allocinfo_get_at = unsafe { core::mem::zeroed() };

            unsafe {
                memset(
                    &mut get_at_params as *mut _ as *mut c_void,
                    0,
                    core::mem::size_of_val(&get_at_params),
                );
                memcpy(
                    &mut get_at_params.filter as *mut _ as *mut c_void,
                    &filter as *const _ as *const c_void,
                    core::mem::size_of_val(&filter),
                );
            }
            get_at_params.pos = pos;

            if unsafe { __allocinfo_get_at(fd, &mut get_at_params) } != 0 {
                break;
            }

            unsafe { (*tags).count = 0 };
            unsafe {
                let idx = (*tags).count as usize;
                (*tags).count += 1;
                memcpy(
                    (*tags).tag.as_mut_ptr().add(idx) as *mut c_void,
                    &get_at_params.data as *const _ as *const c_void,
                    core::mem::size_of_val(&get_at_params.data),
                );
            }

            while unsafe { (*tags).count < VEC_MAX_ENTRIES as __u64 }
                && unsafe {
                    __allocinfo_get_next(fd, (*tags).tag.as_mut_ptr().add((*tags).count as usize))
                        == 0
                }
            {
                unsafe { (*tags).count += 1 };
            }

            i = 0;
            while unsafe { i < (*tags).count } {
                if unsafe {
                    strcmp(
                        (*tags).tag[i as usize].tag.function.as_ptr(),
                        target_function,
                    ) == 0
                } {
                    found_tag = unsafe { (*tags).tag.as_mut_ptr().add(i as usize) };
                    break;
                }
                i += 1;
            }

            if !found_tag.is_null() || unsafe { (*tags).count < VEC_MAX_ENTRIES as __u64 } {
                break;
            }

            pos += unsafe { (*tags).count };
        }

        if unsafe { __allocinfo_get_content_id(fd, &mut end_cont_id) } != 0 {
            unsafe { ksft_print_msg(c"allocinfo_get_content_id failed\n".as_ptr()) };
            ret = KSFT_FAIL;
            unsafe { close(fd) };
            unsafe { free(tags as *mut c_void) };
            unsafe { free(procfs_entries as *mut c_void) };
            return ret;
        }

        if start_cont_id.id == end_cont_id.id {
            break;
        }

        unsafe {
            ksft_print_msg(
                c"Module load detected during size verification, retrying...\n".as_ptr(),
            )
        };

        let old = retry;
        retry += 1;
        if !(old < max_retries) {
            break;
        }
    }

    if start_cont_id.id == end_cont_id.id && found_tag.is_null() {
        unsafe {
            ksft_print_msg(
                c"Entry with function %s not found in IOCTL results\n".as_ptr(),
                target_function,
            )
        };
        ret = KSFT_FAIL;
    } else if start_cont_id.id != end_cont_id.id {
        unsafe {
            ksft_print_msg(
                c"Failed to match content_ids for procfs and IOCTL, skipping...\n".as_ptr(),
            )
        };
        ret = KSFT_SKIP;
    } else if !found_tag.is_null() && unsafe { (*found_tag).counter.bytes != target_size } {
        unsafe {
            ksft_print_msg(
                c"IOCTL entry size %llu does not match target size %llu\n".as_ptr(),
                (*found_tag).counter.bytes,
                target_size,
            )
        };
        ret = KSFT_FAIL;
    }

    unsafe { close(fd) };
    unsafe { free(tags as *mut c_void) };
    unsafe { free(procfs_entries as *mut c_void) };
    ret
}

unsafe fn test_lineno_filter() -> c_int {
    let tags: *mut allocinfo_tag_data_vec =
        unsafe { malloc(core::mem::size_of::<allocinfo_tag_data_vec>()) as *mut _ };
    let procfs_entries: *mut allocinfo_tag_data_vec =
        unsafe { malloc(core::mem::size_of::<allocinfo_tag_data_vec>()) as *mut _ };
    let mut filter: allocinfo_filter = unsafe { core::mem::zeroed() };
    let ioctl_status: ioctl_ret;
    let mut ret: c_int = KSFT_PASS;
    let mut i: __u64;
    let target_lineno: __u64;
    let target_tag: *mut allocinfo_tag_data;
    let mut found: bool = false;

    if tags.is_null() || procfs_entries.is_null() {
        unsafe { ksft_print_msg(c"Memory allocation failed.\n".as_ptr()) };
        ret = KSFT_FAIL;
        unsafe { free(tags as *mut c_void) };
        unsafe { free(procfs_entries as *mut c_void) };
        return ret;
    }

    unsafe {
        memset(
            &mut filter as *mut _ as *mut c_void,
            0,
            core::mem::size_of_val(&filter),
        )
    };

    if unsafe { get_filtered_procfs_entries(procfs_entries, &filter) } != 0 {
        unsafe { ksft_print_msg(c"Error retrieving entries from /proc/allocinfo\n".as_ptr()) };
        ret = KSFT_SKIP;
        unsafe { free(tags as *mut c_void) };
        unsafe { free(procfs_entries as *mut c_void) };
        return ret;
    }
    if unsafe { (*procfs_entries).count == 0 } {
        unsafe { ksft_print_msg(c"Could not retrieve procfs entries\n".as_ptr()) };
        ret = KSFT_SKIP;
        unsafe { free(tags as *mut c_void) };
        unsafe { free(procfs_entries as *mut c_void) };
        return ret;
    }
    /*
     * We depend on the procfs results to determine the line number for the filter before
     * making the ioctl query. Hence, we cannot reuse run_filter_test here.
     */
    target_tag = unsafe { (*procfs_entries).tag.as_mut_ptr().add(0) };
    target_lineno = unsafe { (*target_tag).tag.lineno };

    filter.mask |= ALLOCINFO_FILTER_MASK_LINENO;
    filter.fields.lineno = target_lineno;

    ioctl_status = unsafe { get_filtered_ioctl_entries(tags, &filter, 0) };
    if ioctl_status == ioctl_ret::IOCTL_INVALID_DATA {
        unsafe { ksft_print_msg(c"Trouble retrieving valid IOCTL entries, skipping.\n".as_ptr()) };
        ret = KSFT_SKIP;
        unsafe { free(tags as *mut c_void) };
        unsafe { free(procfs_entries as *mut c_void) };
        return ret;
    }
    if ioctl_status == ioctl_ret::IOCTL_FAILURE {
        unsafe { ksft_print_msg(c"Error retrieving IOCTL entries.\n".as_ptr()) };
        ret = KSFT_FAIL;
        unsafe { free(tags as *mut c_void) };
        unsafe { free(procfs_entries as *mut c_void) };
        return ret;
    }

    i = 0;
    while unsafe { i < (*tags).count } {
        if unsafe { (*tags).tag[i as usize].tag.lineno != target_lineno } {
            unsafe {
                ksft_print_msg(
                    c"IOCTL entry %llu has incorrect lineno %llu.\n".as_ptr(),
                    i,
                    (*tags).tag[i as usize].tag.lineno,
                )
            };
            ret = KSFT_FAIL;
            unsafe { free(tags as *mut c_void) };
            unsafe { free(procfs_entries as *mut c_void) };
            return ret;
        }

        if unsafe {
            strncmp(
                (*tags).tag[i as usize].tag.function.as_ptr(),
                (*target_tag).tag.function.as_ptr(),
                ALLOCINFO_STR_SIZE,
            ) == 0
                && strncmp(
                    (*tags).tag[i as usize].tag.filename.as_ptr(),
                    (*target_tag).tag.filename.as_ptr(),
                    ALLOCINFO_STR_SIZE,
                ) == 0
        } {
            found = true;
        }
        i += 1;
    }

    if !found {
        unsafe {
            ksft_print_msg(
                c"Original procfs entry not found in IOCTL lineno filter results.\n".as_ptr(),
            )
        };
        ret = KSFT_FAIL;
    }

    unsafe { free(tags as *mut c_void) };
    unsafe { free(procfs_entries as *mut c_void) };
    ret
}

fn main() {
    let mut ret: c_int;

    unsafe { ksft_set_plan(4) };

    ret = unsafe { test_filename_filter() };
    if ret == KSFT_SKIP {
        unsafe { ksft_test_result_skip(c"Skipping test_filename_filter\n".as_ptr()) };
    } else {
        unsafe { ksft_test_result(ret == KSFT_PASS, c"test_filename_filter\n".as_ptr()) };
    }

    ret = unsafe { test_function_filter() };
    if ret == KSFT_SKIP {
        unsafe { ksft_test_result_skip(c"Skipping test_function_filter\n".as_ptr()) };
    } else {
        unsafe { ksft_test_result(ret == KSFT_PASS, c"test_function_filter\n".as_ptr()) };
    }

    ret = unsafe { test_size_filter() };
    if ret == KSFT_SKIP {
        unsafe { ksft_test_result_skip(c"Skipping test_size_filter\n".as_ptr()) };
    } else {
        unsafe { ksft_test_result(ret == KSFT_PASS, c"test_size_filter\n".as_ptr()) };
    }

    ret = unsafe { test_lineno_filter() };
    if ret == KSFT_SKIP {
        unsafe { ksft_test_result_skip(c"Skipping test_lineno_filter\n".as_ptr()) };
    } else {
        unsafe { ksft_test_result(ret == KSFT_PASS, c"test_lineno_filter\n".as_ptr()) };
    }

    unsafe { ksft_finished() };
}
