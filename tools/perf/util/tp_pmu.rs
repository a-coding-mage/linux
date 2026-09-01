// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Translated from perf/util/tp_pmu.c.
// Dependencies originally provided by:
// "tp_pmu.h", "pmus.h", <api/fs/fs.h>, <api/fs/tracing_path.h>,
// <api/io_dir.h>, <linux/kernel.h>, <errno.h>, and <string.h>.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut errno: c_int;

    fn get_events_file(sys: *const c_char) -> *mut c_char;
    fn put_events_file(path: *mut c_char);
    fn get_tracing_file(name: *const c_char) -> *mut c_char;
    fn filename__read_int(filename: *const c_char, value: *mut c_int) -> c_int;
    fn filename__read_str(
        filename: *const c_char,
        buf: *mut *mut c_char,
        size: *mut usize,
    ) -> c_int;
    fn io_dir__init(dir: *mut io_dir, dirfd: c_int);
    fn io_dir__readdir(dir: *mut io_dir) -> *mut io_dirent64;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
}

// Constants are supplied by external C headers in the original translation unit.
const PATH_MAX: usize = 4096;
const FILENAME_MAX: usize = 4096;
const O_CLOEXEC: c_int = 0o2000000;
const O_DIRECTORY: c_int = 0o200000;
const O_RDONLY: c_int = 0;
const PERF_TYPE_TRACEPOINT: u32 = 2;

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
    pub type_: u32,
}

#[repr(C)]
pub struct pmu_event_info {
    pub pmu: *const perf_pmu,
    pub pmu_name: *const c_char,
    pub event_type_desc: *const c_char,
    pub encoding_desc: *const c_char,
    pub long_desc: *const c_char,
    pub name: *const c_char,
}

#[repr(C)]
pub struct io_dir {
    pub dirfd: c_int,
}

#[repr(C)]
pub struct io_dirent64 {
    pub d_name: [c_char; 256],
}

pub type tp_event_callback =
    unsafe extern "C" fn(state: *mut c_void, sys: *const c_char, name: *const c_char) -> c_int;
pub type tp_sys_callback =
    unsafe extern "C" fn(state: *mut c_void, sys: *const c_char) -> c_int;
pub type pmu_event_callback =
    unsafe extern "C" fn(state: *mut c_void, info: *const pmu_event_info) -> c_int;

#[no_mangle]
pub unsafe extern "C" fn tp_pmu__id(sys: *const c_char, name: *const c_char) -> c_int {
    let tp_dir = get_events_file(sys);
    let mut path = [0 as c_char; PATH_MAX];
    let mut id: c_int = 0;
    let mut err: c_int;

    if tp_dir.is_null() {
        return -1;
    }

    scnprintf(
        path.as_mut_ptr(),
        PATH_MAX,
        b"%s/%s/id\0".as_ptr() as *const c_char,
        tp_dir,
        name,
    );
    put_events_file(tp_dir);
    err = filename__read_int(path.as_ptr(), &mut id);
    if err != 0 {
        return err;
    }

    id
}

#[no_mangle]
pub unsafe extern "C" fn tp_pmu__for_each_tp_event(
    sys: *const c_char,
    state: *mut c_void,
    cb: tp_event_callback,
) -> c_int {
    let mut evt_path: *mut c_char;
    let mut evt_ent: *mut io_dirent64;
    let mut evt_dir: io_dir = core::mem::zeroed();
    let mut ret: c_int = 0;

    evt_path = get_events_file(sys);
    if evt_path.is_null() {
        return -errno;
    }

    io_dir__init(
        &mut evt_dir,
        open(evt_path, O_CLOEXEC | O_DIRECTORY | O_RDONLY),
    );
    if evt_dir.dirfd < 0 {
        ret = -errno;
        put_events_file(evt_path);
        return ret;
    }
    put_events_file(evt_path);

    while ret == 0 {
        evt_ent = io_dir__readdir(&mut evt_dir);
        if evt_ent.is_null() {
            break;
        }

        if strcmp((*evt_ent).d_name.as_ptr(), b".\0".as_ptr() as *const c_char) == 0
            || strcmp((*evt_ent).d_name.as_ptr(), b"..\0".as_ptr() as *const c_char) == 0
            || strcmp((*evt_ent).d_name.as_ptr(), b"enable\0".as_ptr() as *const c_char) == 0
            || strcmp((*evt_ent).d_name.as_ptr(), b"filter\0".as_ptr() as *const c_char) == 0
        {
            continue;
        }

        ret = cb(state, sys, (*evt_ent).d_name.as_ptr());
        if ret != 0 {
            break;
        }
    }
    close(evt_dir.dirfd);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn tp_pmu__for_each_tp_sys(
    state: *mut c_void,
    cb: tp_sys_callback,
) -> c_int {
    let mut events_ent: *mut io_dirent64;
    let mut events_dir: io_dir = core::mem::zeroed();
    let mut ret: c_int = 0;
    let events_dir_path = get_tracing_file(b"events\0".as_ptr() as *const c_char);

    if events_dir_path.is_null() {
        return -errno;
    }

    io_dir__init(
        &mut events_dir,
        open(events_dir_path, O_CLOEXEC | O_DIRECTORY | O_RDONLY),
    );
    if events_dir.dirfd < 0 {
        ret = -errno;
        put_events_file(events_dir_path);
        return ret;
    }
    put_events_file(events_dir_path);

    while ret == 0 {
        events_ent = io_dir__readdir(&mut events_dir);
        if events_ent.is_null() {
            break;
        }

        if strcmp((*events_ent).d_name.as_ptr(), b".\0".as_ptr() as *const c_char) == 0
            || strcmp((*events_ent).d_name.as_ptr(), b"..\0".as_ptr() as *const c_char) == 0
            || strcmp((*events_ent).d_name.as_ptr(), b"enable\0".as_ptr() as *const c_char) == 0
            || strcmp((*events_ent).d_name.as_ptr(), b"header_event\0".as_ptr() as *const c_char)
                == 0
            || strcmp((*events_ent).d_name.as_ptr(), b"header_page\0".as_ptr() as *const c_char)
                == 0
        {
            continue;
        }

        ret = cb(state, (*events_ent).d_name.as_ptr());
    }
    close(events_dir.dirfd);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__is_tracepoint(pmu: *const perf_pmu) -> bool {
    (*pmu).type_ == PERF_TYPE_TRACEPOINT
}

#[repr(C)]
struct for_each_event_args {
    state: *mut c_void,
    cb: pmu_event_callback,
    pmu: *const perf_pmu,
}

unsafe extern "C" fn for_each_event_cb(
    state: *mut c_void,
    sys_name: *const c_char,
    evt_name: *const c_char,
) -> c_int {
    let args = state as *mut for_each_event_args;
    let mut name = [0 as c_char; 2 * FILENAME_MAX + 2];
    /* 16 possible hex digits and 22 other characters and \0. */
    let mut encoding = [0 as c_char; 16 + 22];
    let mut format: *mut c_char = core::ptr::null_mut();
    let mut format_size: usize = 0;
    let mut info: pmu_event_info = core::mem::zeroed();
    info.pmu = (*args).pmu;
    info.pmu_name = (*(*args).pmu).name;
    info.event_type_desc = b"Tracepoint event\0".as_ptr() as *const c_char;
    let tp_dir = get_events_file(sys_name);
    let mut path = [0 as c_char; PATH_MAX];
    let mut id: c_int = 0;
    let mut err: c_int;

    if tp_dir.is_null() {
        return -1;
    }

    scnprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%s/%s/id\0".as_ptr() as *const c_char,
        tp_dir,
        evt_name,
    );
    err = filename__read_int(path.as_ptr(), &mut id);
    if err == 0 {
        snprintf(
            encoding.as_mut_ptr(),
            encoding.len(),
            b"tracepoint/config=0x%x/\0".as_ptr() as *const c_char,
            id,
        );
        info.encoding_desc = encoding.as_ptr();
    }

    scnprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%s/%s/format\0".as_ptr() as *const c_char,
        tp_dir,
        evt_name,
    );
    put_events_file(tp_dir);
    err = filename__read_str(path.as_ptr(), &mut format, &mut format_size);
    if err == 0 {
        info.long_desc = format;
        let mut i: usize = 0;
        while i < format_size {
            /* Swap tabs to spaces due to some rendering issues. */
            if *format.add(i) == b'\t' as c_char {
                *format.add(i) = b' ' as c_char;
            }
            i += 1;
        }
    }
    snprintf(
        name.as_mut_ptr(),
        name.len(),
        b"%s:%s\0".as_ptr() as *const c_char,
        sys_name,
        evt_name,
    );
    info.name = name.as_ptr();
    err = ((*args).cb)((*args).state, &info);
    free(format as *mut c_void);
    err
}

unsafe extern "C" fn for_each_event_sys_cb(
    state: *mut c_void,
    sys_name: *const c_char,
) -> c_int {
    tp_pmu__for_each_tp_event(sys_name, state, for_each_event_cb)
}

#[no_mangle]
pub unsafe extern "C" fn tp_pmu__for_each_event(
    pmu: *mut perf_pmu,
    state: *mut c_void,
    cb: pmu_event_callback,
) -> c_int {
    let mut args = for_each_event_args {
        state,
        cb,
        pmu,
    };

    tp_pmu__for_each_tp_sys(&mut args as *mut for_each_event_args as *mut c_void, for_each_event_sys_cb)
}

unsafe extern "C" fn num_events_cb(
    state: *mut c_void,
    _sys_name: *const c_char,
    _evt_name: *const c_char,
) -> c_int {
    let count = state as *mut usize;

    *count += 1;
    0
}

unsafe extern "C" fn num_events_sys_cb(
    state: *mut c_void,
    sys_name: *const c_char,
) -> c_int {
    tp_pmu__for_each_tp_event(sys_name, state, num_events_cb)
}

#[no_mangle]
pub unsafe extern "C" fn tp_pmu__num_events(_pmu: *mut perf_pmu) -> usize {
    let mut count: usize = 0;

    tp_pmu__for_each_tp_sys(&mut count as *mut usize as *mut c_void, num_events_sys_cb);
    count
}

#[no_mangle]
pub unsafe extern "C" fn tp_pmu__have_event(
    _pmu: *mut perf_pmu,
    name: *const c_char,
) -> bool {
    let mut dup_name: *mut c_char;
    let mut colon: *mut c_char;
    let id: c_int;

    colon = strchr(name as *mut c_char, ':' as c_int);
    if colon.is_null() {
        return false;
    }

    dup_name = strdup(name);
    if dup_name.is_null() {
        return false;
    }

    colon = dup_name.add(colon.offset_from(name) as usize);
    *colon = '\0' as c_char;
    id = tp_pmu__id(dup_name, colon.add(1));
    free(dup_name as *mut c_void);
    id >= 0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
