// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2009, Steven Rostedt <srostedt@redhat.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

const BUFSIZ: usize = 8192;
const STDOUT_FILENO: c_int = 1;
const SEEK_CUR: c_int = 1;
const TEP_NSEC_OUTPUT: c_int = 1;

#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_event {
    pub pevent: *mut tep_handle,
}

unsafe extern "C" {
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn lseek(fd: c_int, offset: isize, whence: c_int) -> isize;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn atof(nptr: *const c_char) -> f64;

    fn pr_debug(format: *const c_char, ...);
    fn tep_read_number(pevent: *mut tep_handle, ptr: *const c_void, size: c_int) -> c_ulonglong;
    fn tep_parse_header_page(
        pevent: *mut tep_handle,
        buf: *mut c_char,
        size: c_ulonglong,
        long_size: c_int,
    ) -> c_int;
    fn tep_get_long_size(pevent: *mut tep_handle) -> c_int;
    fn tep_set_long_size(pevent: *mut tep_handle, long_size: c_int);
    fn tep_get_header_page_size(pevent: *mut tep_handle) -> c_int;
    fn tep_set_flag(pevent: *mut tep_handle, flag: c_int);
    fn tep_set_file_bigendian(pevent: *mut tep_handle, endian: c_int);
    fn tep_set_local_bigendian(pevent: *mut tep_handle, endian: c_int);
    fn tep_set_page_size(pevent: *mut tep_handle, page_size: c_int);
    fn tep_print_funcs(pevent: *mut tep_handle);
    fn tep_print_printk(pevent: *mut tep_handle);

    fn parse_ftrace_printk(pevent: *mut tep_handle, buf: *mut c_char, size: c_uint);
    fn parse_ftrace_file(pevent: *mut tep_handle, buf: *mut c_char, size: c_ulonglong) -> isize;
    fn parse_event_file(
        pevent: *mut tep_handle,
        buf: *mut c_char,
        size: c_ulonglong,
        sys: *mut c_char,
    ) -> isize;
    fn parse_saved_cmdline(pevent: *mut tep_handle, buf: *mut c_char, size: c_ulonglong);
    fn trace_event__init(tevent: *mut trace_event) -> c_int;
    fn trace_event__cleanup(tevent: *mut trace_event);
    fn host_is_bigendian() -> bool;
}

static mut INPUT_FD: c_int = 0;

static mut TRACE_DATA_SIZE: isize = 0;
static mut REPIPE: bool = false;

unsafe fn __do_read(fd: c_int, mut buf: *mut c_void, mut size: usize) -> isize {
    let rsize = size;

    while size != 0 {
        let ret = unsafe { read(fd, buf, size) };

        if ret <= 0 {
            return -1;
        }

        if unsafe { REPIPE } {
            let retw = unsafe { write(STDOUT_FILENO, buf as *const c_void, ret as usize) };

            if retw <= 0 || retw != ret {
                unsafe { pr_debug(c"repiping input file".as_ptr()) };
                return -1;
            }
        }

        size -= ret as usize;
        buf = unsafe { (buf as *mut u8).add(ret as usize) as *mut c_void };
    }

    rsize as isize
}

unsafe fn do_read(data: *mut c_void, size: usize) -> isize {
    let r;

    r = unsafe { __do_read(INPUT_FD, data, size) };
    if r <= 0 {
        unsafe {
            pr_debug(
                c"reading input file (size expected=%zu received=%zd)".as_ptr(),
                size,
                r,
            )
        };
        return -1;
    }

    unsafe {
        TRACE_DATA_SIZE += r;
    }

    r
}

/* If it fails, the next read will report it */
unsafe fn skip(mut size: usize) {
    let mut buf = [0 as c_char; BUFSIZ];
    let mut ret: isize;

    while size != 0 {
        let len = if size > BUFSIZ { BUFSIZ } else { size };

        ret = unsafe { do_read(buf.as_mut_ptr() as *mut c_void, len) };
        if ret <= 0 {
            break;
        }

        size -= ret as usize;
    }
}

unsafe fn read4(pevent: *mut tep_handle) -> c_uint {
    let mut data: c_uint = 0;

    if unsafe { do_read(&mut data as *mut c_uint as *mut c_void, 4) } < 0 {
        return 0;
    }
    unsafe { tep_read_number(pevent, &data as *const c_uint as *const c_void, 4) as c_uint }
}

unsafe fn read8(pevent: *mut tep_handle) -> c_ulonglong {
    let mut data: c_ulonglong = 0;

    if unsafe { do_read(&mut data as *mut c_ulonglong as *mut c_void, 8) } < 0 {
        return 0;
    }
    unsafe { tep_read_number(pevent, &data as *const c_ulonglong as *const c_void, 8) }
}

unsafe fn read_string() -> *mut c_char {
    let mut buf = [0 as c_char; BUFSIZ];
    let mut str_: *mut c_char = core::ptr::null_mut();
    let mut size: c_int = 0;
    let mut r: isize;
    let mut c: c_char = 0;

    loop {
        r = unsafe { read(INPUT_FD, &mut c as *mut c_char as *mut c_void, 1) };
        if r < 0 {
            unsafe { pr_debug(c"reading input file".as_ptr()) };
            break;
        }

        if r == 0 {
            unsafe { pr_debug(c"no data".as_ptr()) };
            break;
        }

        if unsafe { REPIPE } {
            let retw = unsafe { write(STDOUT_FILENO, &c as *const c_char as *const c_void, 1) };

            if retw <= 0 || retw != r {
                unsafe { pr_debug(c"repiping input file string".as_ptr()) };
                break;
            }
        }

        if size >= BUFSIZ as c_int - 1 {
            unsafe { pr_debug(c"string too long (max %zu bytes)".as_ptr(), BUFSIZ - 1) };
            break;
        }

        buf[size as usize] = c;
        size += 1;

        if c == 0 {
            unsafe {
                TRACE_DATA_SIZE += size as isize;
            }

            str_ = unsafe { malloc(size as usize) as *mut c_char };
            if !str_.is_null() {
                unsafe {
                    memcpy(
                        str_ as *mut c_void,
                        buf.as_ptr() as *const c_void,
                        size as usize,
                    );
                }
            }
            break;
        }
    }

    str_
}

unsafe fn read_proc_kallsyms(pevent: *mut tep_handle) -> c_int {
    let mut size: c_uint;

    size = unsafe { read4(pevent) };
    if size == 0 {
        return 0;
    }
    /*
     * Just skip it, now that we configure libtraceevent to use the
     * tools/perf/ symbol resolver.
     *
     * We need to skip it so that we can continue parsing old perf.data
     * files, that contains this /proc/kallsyms payload.
     *
     * Newer perf.data files will have just the 4-bytes zeros "kallsyms
     * payload", so that older tools can continue reading it and interpret
     * it as "no kallsyms payload is present".
     */
    unsafe {
        lseek(INPUT_FD, size as isize, SEEK_CUR);
        TRACE_DATA_SIZE += size as isize;
    }
    0
}

unsafe fn read_ftrace_printk(pevent: *mut tep_handle) -> c_int {
    let mut size: c_uint;
    let buf: *mut c_char;

    /* it can have 0 size */
    size = unsafe { read4(pevent) };
    if size == 0 {
        return 0;
    }

    if size == c_uint::MAX {
        unsafe { pr_debug(c"invalid ftrace printk size\n".as_ptr()) };
        return -1;
    }

    buf = unsafe { malloc(size as usize + 1) as *mut c_char };
    if buf.is_null() {
        return -1;
    }

    if unsafe { do_read(buf as *mut c_void, size as usize) } < 0 {
        unsafe { free(buf as *mut c_void) };
        return -1;
    }

    unsafe {
        *buf.add(size as usize) = 0;

        parse_ftrace_printk(pevent, buf, size);

        free(buf as *mut c_void);
    }
    0
}

unsafe fn read_header_files(pevent: *mut tep_handle) -> c_int {
    let mut size: c_ulonglong;
    let header_page: *mut c_char;
    let mut buf = [0 as c_char; BUFSIZ];
    let ret: isize = 0;

    if unsafe { do_read(buf.as_mut_ptr() as *mut c_void, 12) } < 0 {
        return -1;
    }

    if unsafe {
        memcmp(
            buf.as_ptr() as *const c_void,
            c"header_page".as_ptr() as *const c_void,
            12,
        )
    } != 0
    {
        unsafe { pr_debug(c"did not read header page".as_ptr()) };
        return -1;
    }

    size = unsafe { read8(pevent) };

    header_page = unsafe { malloc(size as usize) as *mut c_char };
    if header_page.is_null() {
        return -1;
    }

    if unsafe { do_read(header_page as *mut c_void, size as usize) } < 0 {
        unsafe {
            pr_debug(c"did not read header page".as_ptr());
            free(header_page as *mut c_void);
        }
        return -1;
    }

    if unsafe { tep_parse_header_page(pevent, header_page, size, tep_get_long_size(pevent)) } == 0 {
        /*
         * The commit field in the page is of type long,
         * use that instead, since it represents the kernel.
         */
        unsafe { tep_set_long_size(pevent, tep_get_header_page_size(pevent)) };
    }
    unsafe { free(header_page as *mut c_void) };

    if unsafe { do_read(buf.as_mut_ptr() as *mut c_void, 13) } < 0 {
        return -1;
    }

    if unsafe {
        memcmp(
            buf.as_ptr() as *const c_void,
            c"header_event".as_ptr() as *const c_void,
            13,
        )
    } != 0
    {
        unsafe { pr_debug(c"did not read header event".as_ptr()) };
        return -1;
    }

    size = unsafe { read8(pevent) };
    unsafe { skip(size as usize) };

    ret as c_int
}

unsafe fn read_ftrace_file(pevent: *mut tep_handle, size: c_ulonglong) -> c_int {
    let mut ret: isize;
    let buf: *mut c_char;

    buf = unsafe { malloc(size as usize) as *mut c_char };
    if buf.is_null() {
        unsafe { pr_debug(c"memory allocation failure\n".as_ptr()) };
        return -1;
    }

    ret = unsafe { do_read(buf as *mut c_void, size as usize) };
    if ret < 0 {
        unsafe { pr_debug(c"error reading ftrace file.\n".as_ptr()) };
    } else {
        ret = unsafe { parse_ftrace_file(pevent, buf, size) };
        if ret < 0 {
            unsafe { pr_debug(c"error parsing ftrace file.\n".as_ptr()) };
        }
    }
    unsafe { free(buf as *mut c_void) };
    ret as c_int
}

unsafe fn read_event_file(
    pevent: *mut tep_handle,
    sys: *mut c_char,
    size: c_ulonglong,
) -> c_int {
    let mut ret: isize;
    let buf: *mut c_char;

    buf = unsafe { malloc(size as usize) as *mut c_char };
    if buf.is_null() {
        unsafe { pr_debug(c"memory allocation failure\n".as_ptr()) };
        return -1;
    }

    ret = unsafe { do_read(buf as *mut c_void, size as usize) };
    if ret >= 0 {
        ret = unsafe { parse_event_file(pevent, buf, size, sys) };
        if ret < 0 {
            unsafe { pr_debug(c"error parsing event file.\n".as_ptr()) };
        }
    }
    unsafe { free(buf as *mut c_void) };
    ret as c_int
}

unsafe fn read_ftrace_files(pevent: *mut tep_handle) -> c_int {
    let mut size: c_ulonglong;
    let count: c_int;
    let mut i: c_int;
    let mut ret: c_int;

    count = unsafe { read4(pevent) as c_int };

    i = 0;
    while i < count {
        size = unsafe { read8(pevent) };
        ret = unsafe { read_ftrace_file(pevent, size) };
        if ret != 0 {
            return ret;
        }
        i += 1;
    }
    0
}

unsafe fn read_event_files(pevent: *mut tep_handle) -> c_int {
    let mut size: c_ulonglong;
    let mut sys: *mut c_char;
    let systems: c_int;
    let count: c_int;
    let mut i: c_int;
    let mut x: c_int;
    let mut ret: isize;

    systems = unsafe { read4(pevent) as c_int };

    i = 0;
    while i < systems {
        sys = unsafe { read_string() };
        if sys.is_null() {
            return -1;
        }

        count = unsafe { read4(pevent) as c_int };

        x = 0;
        while x < count {
            size = unsafe { read8(pevent) };
            ret = unsafe { read_event_file(pevent, sys, size) as isize };
            if ret != 0 {
                unsafe { free(sys as *mut c_void) };
                return ret as c_int;
            }
            x += 1;
        }
        unsafe { free(sys as *mut c_void) };
        i += 1;
    }
    0
}

unsafe fn read_saved_cmdline(pevent: *mut tep_handle) -> c_int {
    let mut size: c_ulonglong;
    let buf: *mut c_char;
    let mut ret: isize;

    /* it can have 0 size */
    size = unsafe { read8(pevent) };
    if size == 0 {
        return 0;
    }

    if size == c_ulonglong::MAX {
        unsafe { pr_debug(c"invalid saved cmdline size".as_ptr()) };
        return -1;
    }

    buf = unsafe { malloc(size as usize + 1) as *mut c_char };
    if buf.is_null() {
        unsafe { pr_debug(c"memory allocation failure\n".as_ptr()) };
        return -1;
    }

    ret = unsafe { do_read(buf as *mut c_void, size as usize) };
    if ret < 0 {
        unsafe { pr_debug(c"error reading saved cmdlines\n".as_ptr()) };
    } else {
        unsafe {
            *buf.add(ret as usize) = 0;

            parse_saved_cmdline(pevent, buf, size);
        }
        ret = 0;
    }
    unsafe { free(buf as *mut c_void) };
    ret as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_report(
    fd: c_int,
    tevent: *mut trace_event,
    __repipe: bool,
) -> isize {
    let mut buf = [0 as c_char; BUFSIZ];
    let test = [23 as c_char, 8 as c_char, 68 as c_char];
    let version: *mut c_char;
    let show_version: c_int = 0;
    let show_funcs: c_int = 0;
    let show_printk: c_int = 0;
    let mut size: isize = -1;
    let file_bigendian: c_int;
    let host_bigendian: c_int;
    let file_long_size: c_int;
    let file_page_size: c_int;
    let mut pevent: *mut tep_handle = core::ptr::null_mut();
    let mut err: c_int;

    unsafe {
        REPIPE = __repipe;
        INPUT_FD = fd;
    }

    if unsafe { do_read(buf.as_mut_ptr() as *mut c_void, 3) } < 0 {
        return -1;
    }
    if unsafe {
        memcmp(
            buf.as_ptr() as *const c_void,
            test.as_ptr() as *const c_void,
            3,
        )
    } != 0
    {
        unsafe { pr_debug(c"no trace data in the file".as_ptr()) };
        return -1;
    }

    if unsafe { do_read(buf.as_mut_ptr() as *mut c_void, 7) } < 0 {
        return -1;
    }
    if unsafe {
        memcmp(
            buf.as_ptr() as *const c_void,
            c"tracing".as_ptr() as *const c_void,
            7,
        )
    } != 0
    {
        unsafe { pr_debug(c"not a trace file (missing 'tracing' tag)".as_ptr()) };
        return -1;
    }

    version = unsafe { read_string() };
    if version.is_null() {
        return -1;
    }
    if show_version != 0 {
        unsafe { printf(c"version = %s\n".as_ptr(), version) };
    }

    if unsafe { do_read(buf.as_mut_ptr() as *mut c_void, 1) } < 0 {
        unsafe { free(version as *mut c_void) };
        return -1;
    }
    file_bigendian = buf[0] as c_int;
    host_bigendian = if unsafe { host_is_bigendian() } { 1 } else { 0 };

    if unsafe { trace_event__init(tevent) } != 0 {
        unsafe { pr_debug(c"trace_event__init failed".as_ptr()) };
        unsafe { free(version as *mut c_void) };
        return size;
    }

    pevent = unsafe { (*tevent).pevent };

    unsafe {
        tep_set_flag(pevent, TEP_NSEC_OUTPUT);
        tep_set_file_bigendian(pevent, file_bigendian);
        tep_set_local_bigendian(pevent, host_bigendian);
    }

    if unsafe { do_read(buf.as_mut_ptr() as *mut c_void, 1) } < 0 {
        unsafe {
            if !pevent.is_null() {
                trace_event__cleanup(tevent);
            }
            free(version as *mut c_void);
        }
        return size;
    }
    file_long_size = buf[0] as c_int;

    file_page_size = unsafe { read4(pevent) as c_int };
    if file_page_size == 0 {
        unsafe {
            if !pevent.is_null() {
                trace_event__cleanup(tevent);
            }
            free(version as *mut c_void);
        }
        return size;
    }

    unsafe {
        tep_set_long_size(pevent, file_long_size);
        tep_set_page_size(pevent, file_page_size);
    }

    err = unsafe { read_header_files(pevent) };
    if err == 0 {
        err = unsafe { read_ftrace_files(pevent) };
    }
    if err == 0 {
        err = unsafe { read_event_files(pevent) };
    }
    if err == 0 {
        err = unsafe { read_proc_kallsyms(pevent) };
    }
    if err == 0 {
        err = unsafe { read_ftrace_printk(pevent) };
    }
    if err == 0 && unsafe { atof(version) } >= 0.6 {
        err = unsafe { read_saved_cmdline(pevent) };
    }
    if err != 0 {
        unsafe {
            if !pevent.is_null() {
                trace_event__cleanup(tevent);
            }
            free(version as *mut c_void);
        }
        return size;
    }

    size = unsafe { TRACE_DATA_SIZE };
    unsafe {
        REPIPE = false;
    }

    if show_funcs != 0 {
        unsafe { tep_print_funcs(pevent) };
    } else if show_printk != 0 {
        unsafe { tep_print_printk(pevent) };
    }

    pevent = core::ptr::null_mut();

    unsafe {
        if !pevent.is_null() {
            trace_event__cleanup(tevent);
        }
        free(version as *mut c_void);
    }
    size
}
