// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008,2009, Steven Rostedt <srostedt@redhat.com>
 */

use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulonglong, c_void};
use std::ptr;

const VERSION: &[u8] = b"0.6\0";
const MAX_EVENT_LENGTH: usize = 512;
const BUFSIZ: usize = 8192;
const MAXPATHLEN: usize = 4096;

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const SEEK_CUR: c_int = 1;
const DT_DIR: u8 = 4;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const PERF_TYPE_TRACEPOINT: u32 = 2;

type ssize_t = isize;
type off_t = i64;
type u64 = u64;

static mut output_fd: c_int = 0;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub node: list_head,
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub name: *mut c_char,
}

#[repr(C)]
pub struct tracing_data {
    pub temp: bool,
    pub size: off_t,
    pub temp_file: [c_char; MAXPATHLEN],
}

#[repr(C)]
struct tracepoint_path {
    system: *mut c_char,
    name: *mut c_char,
    next: *mut tracepoint_path,
}

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

unsafe extern "C" {
    static mut page_size: c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn pwrite(fd: c_int, buf: *const c_void, count: usize, offset: off_t) -> ssize_t;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn rewinddir(dirp: *mut DIR);
    fn free(ptr: *mut c_void);
    fn malloc(size: usize) -> *mut c_void;
    fn zalloc(size: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strndup(s: *const c_char, n: usize) -> *mut c_char;
    fn atoll(nptr: *const c_char) -> c_long;
    fn snprintf(str: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn scnprintf(str: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn __errno_location() -> *mut c_int;

    fn host_is_bigendian() -> bool;
    fn get_events_file(name: *const c_char) -> *mut c_char;
    fn put_events_file(file: *mut c_char);
    fn get_tracing_file(name: *const c_char) -> *mut c_char;
    fn put_tracing_file(file: *mut c_char);
    fn tracing_events__opendir() -> *mut DIR;
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn zfree<T>(ptrp: *mut *mut T) {
    if !(*ptrp).is_null() {
        free(*ptrp as *mut c_void);
        *ptrp = ptr::null_mut();
    }
}

unsafe fn c_lit(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn dirent_name(dent: *mut dirent) -> *mut c_char {
    (*dent).d_name.as_mut_ptr()
}

unsafe fn is_event_dir(dent: *mut dirent) -> bool {
    if (*dent).d_type != DT_DIR {
        return false;
    }
    strcmp(dirent_name(dent), c_lit(b".\0")) != 0 && strcmp(dirent_name(dent), c_lit(b"..\0")) != 0
}

/* unfortunately, you can not stat debugfs or proc files for size */
unsafe fn record_file(file: *const c_char, hdr_sz: ssize_t) -> c_int {
    let mut size: c_ulonglong = 0;
    let mut buf = [0u8; BUFSIZ];
    let hdr_pos = lseek(output_fd, 0, SEEK_CUR);
    let mut err = -EIO;

    let fd = open(file, O_RDONLY);
    if fd < 0 {
        pr_debug(c_lit(b"Can't read '%s'\0"), file);
        return -errno();
    }

    /* put in zeros for file size, then fill true size later */
    if hdr_sz != 0 {
        if write(output_fd, &size as *const _ as *const c_void, hdr_sz as usize) != hdr_sz {
            close(fd);
            return err;
        }
    }

    loop {
        let r = read(fd, buf.as_mut_ptr() as *mut c_void, BUFSIZ);
        if r > 0 {
            size = size.wrapping_add(r as c_ulonglong);
            if write(output_fd, buf.as_ptr() as *const c_void, r as usize) != r {
                close(fd);
                return err;
            }
        }
        if r <= 0 {
            break;
        }
    }

    /* ugh, handle big-endian hdr_size == 4 */
    let mut sizep = &size as *const _ as *const c_char;
    if host_is_bigendian() {
        sizep = sizep.add(mem::size_of::<u64>() - hdr_sz as usize);
    }

    if hdr_sz != 0 && pwrite(output_fd, sizep as *const c_void, hdr_sz as usize, hdr_pos) < 0 {
        pr_debug(c_lit(b"writing file size failed\n\0"));
        close(fd);
        return err;
    }

    err = 0;
    close(fd);
    err
}

unsafe fn record_header_files() -> c_int {
    let mut path = get_events_file(c_lit(b"header_page\0"));
    let mut st: stat = mem::zeroed();
    let mut err = -EIO;

    if path.is_null() {
        pr_debug(c_lit(b"can't get tracing/events/header_page\0"));
        return -ENOMEM;
    }

    if stat(path, &mut st) < 0 {
        pr_debug(c_lit(b"can't read '%s'\0"), path);
        put_events_file(path);
        return err;
    }

    if write(output_fd, c_lit(b"header_page\0") as *const c_void, 12) != 12 {
        pr_debug(c_lit(b"can't write header_page\n\0"));
        put_events_file(path);
        return err;
    }

    if record_file(path, 8) < 0 {
        pr_debug(c_lit(b"can't record header_page file\n\0"));
        put_events_file(path);
        return err;
    }

    put_events_file(path);

    path = get_events_file(c_lit(b"header_event\0"));
    if path.is_null() {
        pr_debug(c_lit(b"can't get tracing/events/header_event\0"));
        return -ENOMEM;
    }

    if stat(path, &mut st) < 0 {
        pr_debug(c_lit(b"can't read '%s'\0"), path);
        put_events_file(path);
        return err;
    }

    if write(output_fd, c_lit(b"header_event\0") as *const c_void, 13) != 13 {
        pr_debug(c_lit(b"can't write header_event\n\0"));
        put_events_file(path);
        return err;
    }

    if record_file(path, 8) < 0 {
        pr_debug(c_lit(b"can't record header_event file\n\0"));
        put_events_file(path);
        return err;
    }

    err = 0;
    put_events_file(path);
    err
}

unsafe fn name_in_tp_list(sys: *mut c_char, mut tps: *mut tracepoint_path) -> bool {
    while !tps.is_null() {
        if strcmp(sys, (*tps).name) == 0 {
            return true;
        }
        tps = (*tps).next;
    }

    false
}

unsafe fn copy_event_system(sys: *const c_char, tps: *mut tracepoint_path) -> c_int {
    let mut st: stat = mem::zeroed();
    let mut format: *mut c_char = ptr::null_mut();
    let dir = opendir(sys);
    let mut count: c_int = 0;
    let mut err: c_int;

    if dir.is_null() {
        pr_debug(c_lit(b"can't read directory '%s'\0"), sys);
        return -errno();
    }

    loop {
        let dent = readdir(dir);
        if dent.is_null() {
            break;
        }
        if is_event_dir(dent) {
            if !name_in_tp_list(dirent_name(dent), tps) {
                continue;
            }

            if asprintf(&mut format, c_lit(b"%s/%s/format\0"), sys, dirent_name(dent)) < 0 {
                err = -ENOMEM;
                closedir(dir);
                return err;
            }
            let ret = stat(format, &mut st);
            free(format as *mut c_void);
            if ret < 0 {
                continue;
            }
            count += 1;
        }
    }

    if write(output_fd, &count as *const _ as *const c_void, 4) != 4 {
        pr_debug(c_lit(b"can't write count\n\0"));
        closedir(dir);
        return -EIO;
    }

    rewinddir(dir);
    loop {
        let dent = readdir(dir);
        if dent.is_null() {
            break;
        }
        if is_event_dir(dent) {
            if !name_in_tp_list(dirent_name(dent), tps) {
                continue;
            }

            if asprintf(&mut format, c_lit(b"%s/%s/format\0"), sys, dirent_name(dent)) < 0 {
                err = -ENOMEM;
                closedir(dir);
                return err;
            }
            let ret = stat(format, &mut st);

            if ret >= 0 {
                err = record_file(format, 8);
                if err != 0 {
                    free(format as *mut c_void);
                    closedir(dir);
                    return err;
                }
            }
            free(format as *mut c_void);
        }
    }
    closedir(dir);
    0
}

unsafe fn record_ftrace_files(tps: *mut tracepoint_path) -> c_int {
    let path = get_events_file(c_lit(b"ftrace\0"));
    if path.is_null() {
        pr_debug(c_lit(b"can't get tracing/events/ftrace\0"));
        return -ENOMEM;
    }

    let ret = copy_event_system(path, tps);

    put_tracing_file(path);

    ret
}

unsafe fn system_in_tp_list(sys: *mut c_char, mut tps: *mut tracepoint_path) -> bool {
    while !tps.is_null() {
        if strcmp(sys, (*tps).system) == 0 {
            return true;
        }
        tps = (*tps).next;
    }

    false
}

unsafe fn record_event_files(tps: *mut tracepoint_path) -> c_int {
    let mut st: stat = mem::zeroed();
    let mut sys: *mut c_char = ptr::null_mut();
    let path = get_tracing_file(c_lit(b"events\0"));
    let mut count: c_int = 0;

    if path.is_null() {
        pr_debug(c_lit(b"can't get tracing/events\0"));
        return -ENOMEM;
    }

    let dir = opendir(path);
    if dir.is_null() {
        let err = -errno();
        pr_debug(c_lit(b"can't read directory '%s'\0"), path);
        put_tracing_file(path);
        return err;
    }

    loop {
        let dent = readdir(dir);
        if dent.is_null() {
            break;
        }
        if is_event_dir(dent) {
            if strcmp(dirent_name(dent), c_lit(b"ftrace\0")) == 0
                || !system_in_tp_list(dirent_name(dent), tps)
            {
                continue;
            }

            count += 1;
        }
    }

    if write(output_fd, &count as *const _ as *const c_void, 4) != 4 {
        pr_debug(c_lit(b"can't write count\n\0"));
        closedir(dir);
        put_tracing_file(path);
        return -EIO;
    }

    rewinddir(dir);
    loop {
        let dent = readdir(dir);
        if dent.is_null() {
            break;
        }
        if is_event_dir(dent) {
            if strcmp(dirent_name(dent), c_lit(b"ftrace\0")) == 0
                || !system_in_tp_list(dirent_name(dent), tps)
            {
                continue;
            }

            if asprintf(&mut sys, c_lit(b"%s/%s\0"), path, dirent_name(dent)) < 0 {
                closedir(dir);
                put_tracing_file(path);
                return -ENOMEM;
            }
            let ret = stat(sys, &mut st);
            if ret >= 0 {
                let size = strlen(dirent_name(dent)) + 1;

                if write(output_fd, dirent_name(dent) as *const c_void, size) != size as ssize_t
                    || copy_event_system(sys, tps) < 0
                {
                    free(sys as *mut c_void);
                    closedir(dir);
                    put_tracing_file(path);
                    return -EIO;
                }
            }
            free(sys as *mut c_void);
        }
    }
    closedir(dir);
    put_tracing_file(path);

    0
}

unsafe fn record_proc_kallsyms() -> c_int {
    let size: c_ulonglong = 0;
    /*
     * Just to keep older perf.data file parsers happy, record a zero
     * sized kallsyms file, i.e. do the same thing that was done when
     * /proc/kallsyms (or something specified via --kallsyms, in a
     * different path) couldn't be read.
     */
    if write(output_fd, &size as *const _ as *const c_void, 4) != 4 {
        -EIO
    } else {
        0
    }
}

unsafe fn record_ftrace_printk() -> c_int {
    let mut size: c_uint;
    let mut st: stat = mem::zeroed();
    let mut err = 0;

    let path = get_tracing_file(c_lit(b"printk_formats\0"));
    if path.is_null() {
        pr_debug(c_lit(b"can't get tracing/printk_formats\0"));
        return -ENOMEM;
    }

    let ret = stat(path, &mut st);
    if ret < 0 {
        /* not found */
        size = 0;
        if write(output_fd, &size as *const _ as *const c_void, 4) != 4 {
            err = -EIO;
        }
        put_tracing_file(path);
        return err;
    }
    err = record_file(path, 4);

    put_tracing_file(path);
    err
}

unsafe fn record_saved_cmdline() -> c_int {
    let mut size: c_ulonglong;
    let mut st: stat = mem::zeroed();
    let mut err = 0;

    let path = get_tracing_file(c_lit(b"saved_cmdlines\0"));
    if path.is_null() {
        pr_debug(c_lit(b"can't get tracing/saved_cmdline\0"));
        return -ENOMEM;
    }

    let ret = stat(path, &mut st);
    if ret < 0 {
        /* not found */
        size = 0;
        if write(output_fd, &size as *const _ as *const c_void, 8) != 8 {
            err = -EIO;
        }
        put_tracing_file(path);
        return err;
    }
    err = record_file(path, 8);

    put_tracing_file(path);
    err
}

unsafe fn put_tracepoints_path(mut tps: *mut tracepoint_path) {
    while !tps.is_null() {
        let t = tps;

        tps = (*tps).next;
        zfree(&mut (*t).name);
        zfree(&mut (*t).system);
        free(t as *mut c_void);
    }
}

unsafe fn tracepoint_id_to_path(config: u64) -> *mut tracepoint_path {
    let mut path: *mut tracepoint_path = ptr::null_mut();
    let mut id_buf = [0 as c_char; 24];
    let mut evt_path = [0 as c_char; MAXPATHLEN];

    let sys_dir = tracing_events__opendir();
    if sys_dir.is_null() {
        return ptr::null_mut();
    }

    loop {
        let sys_dirent = readdir(sys_dir);
        if sys_dirent.is_null() {
            break;
        }
        if !is_event_dir(sys_dirent) {
            continue;
        }
        let dir_path = get_events_file(dirent_name(sys_dirent));
        if dir_path.is_null() {
            continue;
        }
        let evt_dir = opendir(dir_path);
        if evt_dir.is_null() {
            put_events_file(dir_path);
            continue;
        }

        loop {
            let evt_dirent = readdir(evt_dir);
            if evt_dirent.is_null() {
                break;
            }
            if !is_event_dir(evt_dirent) {
                continue;
            }

            scnprintf(
                evt_path.as_mut_ptr(),
                MAXPATHLEN,
                c_lit(b"%s/%s/id\0"),
                dir_path,
                dirent_name(evt_dirent),
            );
            let fd = open(evt_path.as_ptr(), O_RDONLY);
            if fd < 0 {
                continue;
            }
            if read(fd, id_buf.as_mut_ptr() as *mut c_void, id_buf.len()) < 0 {
                close(fd);
                continue;
            }
            close(fd);
            let id = atoll(id_buf.as_ptr()) as u64;
            if id == config {
                put_events_file(dir_path);
                closedir(evt_dir);
                closedir(sys_dir);
                path = zalloc(mem::size_of::<tracepoint_path>()) as *mut tracepoint_path;
                if path.is_null() {
                    return ptr::null_mut();
                }
                if asprintf(
                    &mut (*path).system,
                    c_lit(b"%.*s\0"),
                    MAX_EVENT_LENGTH as c_int,
                    dirent_name(sys_dirent),
                ) < 0
                {
                    free(path as *mut c_void);
                    return ptr::null_mut();
                }
                if asprintf(
                    &mut (*path).name,
                    c_lit(b"%.*s\0"),
                    MAX_EVENT_LENGTH as c_int,
                    dirent_name(evt_dirent),
                ) < 0
                {
                    zfree(&mut (*path).system);
                    free(path as *mut c_void);
                    return ptr::null_mut();
                }
                return path;
            }
        }
        closedir(evt_dir);
        put_events_file(dir_path);
    }

    closedir(sys_dir);
    path
}

#[no_mangle]
pub unsafe extern "C" fn tracepoint_id_to_name(config: u64) -> *mut c_char {
    let path = tracepoint_id_to_path(config);
    let mut buf: *mut c_char = ptr::null_mut();

    if !path.is_null() && asprintf(&mut buf, c_lit(b"%s:%s\0"), (*path).system, (*path).name) < 0 {
        buf = ptr::null_mut();
    }

    put_tracepoints_path(path);
    buf
}

unsafe fn tracepoint_name_to_path(name: *const c_char) -> *mut tracepoint_path {
    let path = zalloc(mem::size_of::<tracepoint_path>()) as *mut tracepoint_path;
    let strp = strchr(name, ':' as c_int);

    if path.is_null() || strp.is_null() {
        free(path as *mut c_void);
        return ptr::null_mut();
    }

    (*path).system = strndup(name, strp.offset_from(name) as usize);
    (*path).name = strdup(strp.add(1));

    if (*path).system.is_null() || (*path).name.is_null() {
        zfree(&mut (*path).system);
        zfree(&mut (*path).name);
        zfree(&mut (path as *mut tracepoint_path));
    }

    path
}

unsafe fn evsel_from_node(node: *mut list_head) -> *mut evsel {
    node as *mut evsel
}

unsafe fn get_tracepoints_path(pattrs: *mut list_head) -> *mut tracepoint_path {
    let mut path: tracepoint_path = mem::zeroed();
    let mut ppath: *mut tracepoint_path = &mut path;
    let mut nr_tracepoints = 0;

    let mut node = (*pattrs).next;
    while node != pattrs {
        let pos = evsel_from_node(node);
        if (*pos).core.attr.type_ != PERF_TYPE_TRACEPOINT {
            node = (*node).next;
            continue;
        }
        nr_tracepoints += 1;

        if !(*pos).name.is_null() {
            (*ppath).next = tracepoint_name_to_path((*pos).name);
            if !(*ppath).next.is_null() {
                node = (*node).next;
                ppath = (*ppath).next;
                continue;
            }

            if strchr((*pos).name, ':' as c_int).is_null() {
                (*ppath).next = tracepoint_id_to_path((*pos).core.attr.config);
                if !(*ppath).next.is_null() {
                    node = (*node).next;
                    ppath = (*ppath).next;
                    continue;
                }
            }

            pr_debug(c_lit(b"No memory to alloc tracepoints list\n\0"));
            put_tracepoints_path(path.next);
            return ptr::null_mut();
        }

        (*ppath).next = tracepoint_id_to_path((*pos).core.attr.config);
        if (*ppath).next.is_null() {
            pr_debug(c_lit(b"No memory to alloc tracepoints list\n\0"));
            put_tracepoints_path(path.next);
            return ptr::null_mut();
        }

        node = (*node).next;
        ppath = (*ppath).next;
    }

    if nr_tracepoints > 0 {
        path.next
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn have_tracepoints(pattrs: *mut list_head) -> bool {
    let mut node = (*pattrs).next;
    while node != pattrs {
        let pos = evsel_from_node(node);
        if (*pos).core.attr.type_ == PERF_TYPE_TRACEPOINT {
            return true;
        }
        node = (*node).next;
    }

    false
}

unsafe fn tracing_data_header() -> c_int {
    let mut buf = [0 as c_char; 20];

    /* just guessing this is someone's birthday.. ;) */
    buf[0] = 23;
    buf[1] = 8;
    buf[2] = 68;
    ptr::copy_nonoverlapping(c_lit(b"tracing\0"), buf.as_mut_ptr().add(3), 7);

    if write(output_fd, buf.as_ptr() as *const c_void, 10) != 10 {
        return -1;
    }

    let size = strlen(VERSION.as_ptr() as *const c_char) + 1;
    if write(output_fd, VERSION.as_ptr() as *const c_void, size) != size as ssize_t {
        return -1;
    }

    /* save endian */
    if host_is_bigendian() {
        buf[0] = 1;
    } else {
        buf[0] = 0;
    }

    if write(output_fd, buf.as_ptr() as *const c_void, 1) != 1 {
        return -1;
    }

    /* save size of long */
    buf[0] = mem::size_of::<c_long>() as c_char;
    if write(output_fd, buf.as_ptr() as *const c_void, 1) != 1 {
        return -1;
    }

    /* save page_size */
    if write(output_fd, &page_size as *const _ as *const c_void, 4) != 4 {
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn tracing_data_get(
    pattrs: *mut list_head,
    fd: c_int,
    temp: bool,
) -> *mut tracing_data {
    let mut err: c_int;

    output_fd = fd;

    let tps = get_tracepoints_path(pattrs);
    if tps.is_null() {
        return ptr::null_mut();
    }

    let tdata = malloc(mem::size_of::<tracing_data>()) as *mut tracing_data;
    if tdata.is_null() {
        return ptr::null_mut();
    }

    (*tdata).temp = temp;
    (*tdata).size = 0;

    if temp {
        snprintf(
            (*tdata).temp_file.as_mut_ptr(),
            (*tdata).temp_file.len(),
            c_lit(b"/tmp/perf-XXXXXX\0"),
        );
        if mkstemp((*tdata).temp_file.as_mut_ptr()) == 0 {
            pr_debug(c_lit(b"Can't make temp file\0"));
            free(tdata as *mut c_void);
            return ptr::null_mut();
        }

        let temp_fd = open((*tdata).temp_file.as_ptr(), O_RDWR);
        if temp_fd < 0 {
            pr_debug(c_lit(b"Can't read '%s'\0"), (*tdata).temp_file.as_ptr());
            free(tdata as *mut c_void);
            return ptr::null_mut();
        }

        /*
         * Set the temp file the default output, so all the
         * tracing data are stored into it.
         */
        output_fd = temp_fd;
    }

    err = tracing_data_header();
    if err == 0 {
        err = record_header_files();
    }
    if err == 0 {
        err = record_ftrace_files(tps);
    }
    if err == 0 {
        err = record_event_files(tps);
    }
    if err == 0 {
        err = record_proc_kallsyms();
    }
    if err == 0 {
        err = record_ftrace_printk();
    }
    if err == 0 {
        err = record_saved_cmdline();
    }

    /*
     * All tracing data are stored by now, we can restore
     * the default output file in case we used temp file.
     */
    if temp {
        (*tdata).size = lseek(output_fd, 0, SEEK_CUR);
        close(output_fd);
        output_fd = fd;
    }

    if err != 0 {
        let mut tmp = tdata;
        zfree(&mut tmp);
    }

    put_tracepoints_path(tps);
    tdata
}

#[no_mangle]
pub unsafe extern "C" fn tracing_data_put(tdata: *mut tracing_data) -> c_int {
    let mut err = 0;

    if (*tdata).temp {
        err = record_file((*tdata).temp_file.as_ptr(), 0);
        unlink((*tdata).temp_file.as_ptr());
    }

    free(tdata as *mut c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn read_tracing_data(fd: c_int, pattrs: *mut list_head) -> c_int {
    /*
     * We work over the real file, so we can write data
     * directly, no temp file is needed.
     */
    let tdata = tracing_data_get(pattrs, fd, false);
    if tdata.is_null() {
        return -ENOMEM;
    }

    tracing_data_put(tdata)
}
